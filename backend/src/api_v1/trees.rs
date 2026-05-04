/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::session::Claims;
use crate::db::cards::{get_tree_cards, remove_card};
use crate::db::trees::{
    add_tree, db_get_tree_all_data, get_queried_count, get_queried_count_pagination,
    get_queried_list, get_queried_list_pagination, get_tree_owner, is_structure_valid,
};
use crate::db_structs::pagination::OptionalPaginationOptions;
use crate::db_structs::trees::{
    TreeAddData, TreeFilters, TreeListResponse, TreeRemoveCardData, TreeStructure,
    TreeUpdateData, TreeUpdateStructureData, TreeWithCards,
};
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::{Config, db};

use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use sqlx::{PgConnection, PgExecutor, PgPool};

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_tree))
        .route("/remove/{tree_id}", delete(remove_tree))
        .route("/update-structure", post(update_tree_structure))
        .route("/remove-card", post(remove_card_from_tree))
        .route("/get-with-cards/{tree_id}", get(get_tree_with_cards))
        .route("/get-list", get(get_list))
        .route("/update-data", post(update_tree_data))
        .route("/get-mine-all", get(get_all_owned_trees))
}

#[utoipa::path(
    post,
    path = "/add",
    context_path = "/api/v1/trees",
    request_body = TreeAddData,
    responses(
        (status = 200, description = "Tree successfully added with returned id.", body = i32, content_type = "application/json"),
        (status = 400, description = ApiError::NonEmptyStructure.to_string()),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie)
    )
)]
/// Add a tree to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_add_data` - The data used to initialize the tree, as [`TreeAddData`].
///
/// # Returns
/// The id assigned to the new tree.
///
/// # Errors
/// Return a [`NonEmptyStructure`](ApiError::NonEmptyStructure) if the [`TreeStructure`] in
/// [`TreeAddData`] contains cards.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_tree(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(tree_add_data): Json<TreeAddData>,
) -> Result<Json<i32>, ApiError> {
    if !is_tree_structure_empty(&tree_add_data.tree_structure) {
        return Err(ApiError::NonEmptyStructure);
    }
    let id = add_tree(&db, &tree_add_data, claims.sub).await?;

    let json_id = Json(id);
    Ok(json_id)
}

#[utoipa::path(
    delete,
    path = "/remove/{tree_id}",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Tree successfully removed."),
        (status = 400, description = ApiError::NonEmptyStructure.to_string()),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("tree_id" = i32, Path),
    )
)]
/// Remove a tree together with its cards from the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_id` - Tree to remove.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with the given `tree_id` exists.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to remove
/// the given tree.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn remove_tree(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
) -> Result<(), ApiError> {
    verify_tree_permissions(&db, tree_id, claims).await?;

    crate::db::trees::remove_tree(&db, tree_id).await
}

/// Check whether a tree structure contains no cards.
///
/// # Returns
/// `true` if the tree structure contains no cards, `false` if the tree structure contains cards.
fn is_tree_structure_empty(tree_structure: &TreeStructure) -> bool {
    tree_structure.flatten().is_empty()
}

#[utoipa::path(
    post,
    path = "/update_structure",
    context_path = "/api/v1/trees",
    request_body = TreeUpdateStructureData,
    responses(
        (status = 200, description = "Tree structure successfully updated."),
        (status = 400, description = "Tree with given id cannot be found or the new structure is not valid."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Update the structure of an existing tree in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_update_data` - The data used to update the data of the tree, as [`TreeUpdateStructureData`].
/// Overwrites the current structure.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with the `id` specified in `tree_update_data` exists.
///
/// Returns [`MismatchedStructure`](ApiError::MismatchedStructure) if either of the following are the case:
/// * Not all cards in the structure reference the `tree_id` in the `Cards` database table.
/// * A card is referenced more than once in the structure.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the tree for the given `tree_id`.
///
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if more than 9,999 cards in the
/// `Cards` table link to `tree_id`.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) or a [`SerdeException`](ApiError::SerdeException) instead.
async fn update_tree_structure(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(tree_update_data): Json<TreeUpdateStructureData>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    verify_tree_permissions(&db, tree_update_data.id, claims).await?;

    let mut transaction = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    update_structure_if_valid(
        &mut transaction,
        tree_update_data.id,
        tree_update_data.new_structure,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/remove-card",
    context_path = "/api/v1/trees",
    request_body = TreeRemoveCardData,
    responses(
        (status = 200, description = "Card successfully removed from tree."),
        (status = 400, description = "Tree or card with given id cannot be found or the new structure is not valid."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Remove a card from a tree.
///
/// Removes the card from the database table `cards`, and overwrites the current tree structure,
/// if the provided new structure in `tree_card_removal_data` is still valid when the card is removed.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_card_removal_data` - The data used to remove the card and update the tree structure,
/// as [`TreeRemoveCardData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with `tree_id` exists.
///
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// Returns [`MismatchedStructure`](ApiError::MismatchedStructure) if either of the following are the case:
/// * Not all cards in the structure reference the `tree_id` in the `cards` database table.
/// * A card is referenced more than once in the structure.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change the tree
/// for the given `tree_id`.
///
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if more than 9,999 cards are found
/// which link to `tree_id`.
///
/// May return a [`SerdeException`](ApiError::SerdeException)
/// or [`DatabaseException`](ApiError::DatabaseException) instead.
async fn remove_card_from_tree(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(tree_card_removal_data): Json<TreeRemoveCardData>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    verify_tree_permissions(&db, tree_card_removal_data.tree_id, claims).await?;

    let mut transaction = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    // First, we remove the card from the database, within the transaction.
    remove_card(&mut *transaction, tree_card_removal_data.card_id).await?;

    // Then, we check if the structure is still valid with the card being removed.
    // This ensures the card is actually removed from the structure.
    update_structure_if_valid(
        &mut transaction,
        tree_card_removal_data.tree_id,
        tree_card_removal_data.new_structure,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Check if `new_structure` is valid, and if it is, update the structure in the database.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query as a [`&mut PgConnection`](PgConnection)
///
/// `tree_id` - The id of the tree which should be updated.
///
/// `new_structure` - The new structure for the tree as a [`TreeStructure`], which should be checked,
/// and, if valid, assigned as the structure for `tree_id` in the database.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MismatchedStructure`](ApiError::MismatchedStructure) if either of the following are the case:
/// * Not all cards in the structure reference the `tree_id` in the `cards` database table.
/// * A card is referenced more than once in the structure.
///
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if more than 9,999 cards are found
/// which link to `tree_id`.
///
/// May return a [`SerdeException`](ApiError::SerdeException)
/// or a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_structure_if_valid(
    db: &mut PgConnection,
    tree_id: i32,
    new_structure: TreeStructure,
) -> Result<(), ApiError> {
    // Verify the new structure matches the cards linked to the tree in the database.
    let is_structure_valid = is_structure_valid(&mut *db, tree_id, &new_structure).await?;
    if !is_structure_valid {
        return Err(ApiError::MismatchedStructure);
    }

    db::trees::update_tree_structure(db, tree_id, &new_structure).await?;
    Ok(())
}

#[utoipa::path(
    post,
    path = "/update-data",
    context_path = "/api/v1/trees",
    request_body = TreeUpdateData,
    responses(
        (status = 200, description = "Tree data updated successfully."),
        (status = 400, description = "Tree with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Update the data of an existing tree in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_update_data` - The data used to update the tree, as [`TreeUpdateData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with the `id` specified in `tree_update_data` exists.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the tree for the given `tree_id`.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_tree_data(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(tree_update_data): Json<TreeUpdateData>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    verify_tree_permissions(&db, tree_update_data.id, claims).await?;

    db::trees::update_tree_data(&db, tree_update_data.id, tree_update_data.new_data).await?;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/get-with-cards/{tree_id}",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Successfully retrieved requested tree.", body = TreeWithCards),
        (status = 400, description = "Tree with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("tree_id" = i32, Path),
    )
)]
/// Request all data of a specific tree together with all of its cards.
///
/// # Arguments
/// `claims` - The session cookie of the user, as an [`Option`]. Necessary if retrieving the user
/// is retrieving their own, unpublished tree.
///
/// `tree_id` - The tree to retrieve data from.
///
/// # Returns
/// The tree data together with its cards as [`TreeWithCards`].
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if `tree_id` does not exist.
///
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the tree is not published
/// and the user is not logged in.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the tree is not published
/// and the user is not its owner.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_tree_with_cards(
    claims: Option<Claims>,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
) -> Result<Json<TreeWithCards>, ApiError> {
    let (data, gen_data, structure) = db_get_tree_all_data(&db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    if !data.published {
        // If the tree is not published, only the author is allowed access
        let Some(claims) = claims else {
            return Err(ApiError::MissingAuthentication);
        };

        if claims.sub != gen_data.owner_id {
            return Err(ApiError::NoPermission);
        }
    }

    let tree_cards = get_tree_cards(&db, tree_id).await?;

    Ok(Json(TreeWithCards {
        data,
        gen_data,
        structure,
        cards: tree_cards,
    }))
}

/// Checks whether the tree exists and if it is owned by the user,
/// or if the user is an admin.
/// This function could be expanded in the future if multiple users can be
/// added to a tree as collaborators, but that is currently not the case.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The tree of which the ownerships gets verified.
///
/// `claims` - The claims of the user who sent this request.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change the tree
/// for the given `tree_id`.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with `tree_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn verify_tree_permissions(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    claims: Claims,
) -> Result<(), ApiError> {
    // Check whether the tree exists.
    let owner_id = get_tree_owner(db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    if claims.account_type == AccountType::Admin || claims.sub == owner_id {
        Ok(())
    } else {
        Err(ApiError::NoPermission)
    }
}

#[utoipa::path(
    get,
    path = "/get-list",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Successfully retrieved the list of trees with given filters.", body = TreeListResponse),
        (status = 400, description = "Requested page or page size has exceeded a limit."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Option<Claims>, Cookie),
        ("page" = OptionalPaginationOptions, Query),
        ("filters" = TreeFilters, Query),
    )
)]
/// Retrieve the list of all trees satisfying the given filters at a given page of a given size.
///
/// # Arguments
/// `claims` - The session cookie of the user, as an [`Option`]. Only necessary if the query filter
/// `showOwnPrivate` is set to `true`.
///
/// `page` - The requested page, given as [`OptionalPaginationOptions`] through a query.
///
/// `filters` - The requested filters to apply to the trees, given as [`TreeFilters`] through a query.
///
/// # Returns
/// A list with the trees satisfying the filters at the requested page, together with an integer
/// with the next page num, or [`None`] if it doesn't exist, as [`TreeListResponse`].
///
/// # Errors
/// Returns [`MaxPageSizeExceeded`](ApiError::MaxPageSizeExceeded) if the user requests more than
/// 100 items per page.
///
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) or
/// [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) instead.
pub async fn get_list(
    claims: Option<Claims>,
    Extension(db): Extension<PgPool>,
    Query(page): Query<OptionalPaginationOptions>,
    Query(filters): Query<TreeFilters>,
) -> Result<Json<TreeListResponse>, ApiError> {
    let page = page.to_some_with_default(1, 50, 1000, 100)?;

    let optional_user_id = claims.map(|c| c.sub);

    let mut transaction = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    let trees_count = get_queried_count(&mut *transaction, &filters, optional_user_id).await?;
    if trees_count == 0 {
        return Ok(Json(TreeListResponse {
            total_trees: 0,
            trees: Vec::new(),
        }));
    }

    let trees = get_queried_list(&mut *transaction, &page, &filters, optional_user_id).await?;

    transaction
        .commit()
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    let list = TreeListResponse {
        total_trees: i32::try_from(trees_count).map_err(|_| ApiError::DatabaseMemoryCap)?,
        trees,
    };

    Ok(Json(list))
}

#[utoipa::path(
    get,
    path = "/get-mine-all",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Successfully retrieved all owned trees.", body = TreeListResponse),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("page" = OptionalPaginationOptions, Query),
    )
)]

/// Request metadata of all trees from the user.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `page` - The requested page, given as [`OptionalPaginationOptions`] through a query.
///
/// # Returns
/// A list containing the metadata of all trees owned by the user and the total amount of trees owned by the user, as [`TreeListResponse`].
///
/// # Errors
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if the limit on the database memory disallows the trees count.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_all_owned_trees(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Query(page): Query<OptionalPaginationOptions>,
) -> Result<Json<TreeListResponse>, ApiError> {
    let page = page.to_some_with_default(1, 50, 1000, 100)?;

    let mut transaction = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    let trees_count = get_queried_count_pagination(&mut *transaction, claims.sub).await?;
    if trees_count == 0 {
        return Ok(Json(TreeListResponse {
            total_trees: 0,
            trees: Vec::new(),
        }));
    }

    let trees = get_queried_list_pagination(&mut *transaction, &page, claims.sub).await?;

    transaction
        .commit()
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    let list = TreeListResponse {
        total_trees: i32::try_from(trees_count).map_err(|_| ApiError::DatabaseMemoryCap)?,
        trees,
    };

    Ok(Json(list))
}
