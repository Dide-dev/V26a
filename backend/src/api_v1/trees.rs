/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::session::Claims;
use crate::db::cards::{get_tree_cards, remove_card};
use crate::db::trees::{
    add_tree, db_get_tree_all_data, get_languages, get_queried_count, get_queried_list,
    get_subjects, get_tree_owner, is_structure_valid, language_exists, subject_exists,
};
use crate::db_structs::pagination::OptionalPaginationOptions;
use crate::db_structs::trees::{
    Language, Subject, TreeAddData, TreeFilters, TreeListResponse, TreeRemoveCardData,
    TreeStructure, TreeUpdateData, TreeUpdateStructureData, TreeWithCards,
};
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::{ArcState, db, get_connection};

use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use sqlx::{PgConnection, PgExecutor, PgPool};

use std::collections::HashMap;

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<ArcState> {
    Router::new()
        .route("/add", post(add_new_tree))
        .route("/remove/{tree_id}", delete(remove_tree))
        .route("/update-structure", post(update_tree_structure))
        .route("/remove-card", post(remove_card_from_tree))
        .route("/get-with-cards/{tree_id}", get(get_tree_with_cards))
        .route("/get-list", get(get_list))
        .route("/update-data", post(update_tree_data))
        .route("/get-all-languages", get(get_all_languages))
        .route("/get-all-subjects", get(get_all_subjects))
}

#[utoipa::path(
    post,
    path = "/add",
    context_path = "/api/v1/trees",
    request_body = TreeAddData,
    responses(
        (status = 200, description = "Tree successfully added with returned id.", body = i32, content_type = "application/json"),
        (status = 400, description = "Tree structure contains cards, invalid ISBN provided or non-existent language or subject provided."),
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
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the [`TreeData`](crate::db_structs::trees::TreeData) in
/// [`TreeAddData`] contains an invalid ISBN.
///
/// Returns [`MissingLanguage`](ApiError::MissingLanguage) if the [`TreeData`](crate::db_structs::trees::TreeData) in
/// [`TreeAddData`] contains a non-existent language.
///
/// Returns [`MissingSubject`](ApiError::MissingSubject) if the [`TreeData`](crate::db_structs::trees::TreeData) in
/// [`TreeAddData`] contains a non-existent subject.
///
/// Returns [`NonEmptyStructure`](ApiError::NonEmptyStructure) if the [`TreeStructure`] in
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
    let mut transaction = db.begin().await?;
    if !language_exists(&mut *transaction, &tree_add_data.tree_data.language_code).await? {
        return Err(ApiError::MissingLanguage(
            tree_add_data.tree_data.language_code,
        ));
    }
    if let Some(subject_id) = &tree_add_data.tree_data.subject_id
        && !subject_exists(&mut *transaction, *subject_id).await?
    {
        return Err(ApiError::MissingSubject(*subject_id));
    }
    let id = add_tree(&mut transaction, &tree_add_data, claims.sub).await?;

    transaction.commit().await?;

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

    db::trees::remove_tree(&db, tree_id).await
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

    let mut transaction = db.begin().await?;

    update_structure_if_valid(
        &mut transaction,
        tree_update_data.id,
        tree_update_data.new_structure,
    )
    .await?;

    transaction.commit().await?;

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

    let mut transaction = db.begin().await?;

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

    transaction.commit().await?;

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
        (status = 400, description = "Tree with given id cannot be found, invalid ISBN provided, non-existent language provided or non-existent subject provided."),
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
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the [`TreeData`](crate::db_structs::trees::TreeData) in
/// [`TreeAddData`] contains an invalid ISBN.
///
/// Returns [`MissingLanguage`](ApiError::MissingLanguage) if the [`TreeData`](crate::db_structs::trees::TreeData) in
/// [`TreeAddData`] contains a non-existent language.
///
/// Returns [`MissingSubject`](ApiError::MissingSubject) if the [`TreeData`](crate::db_structs::trees::TreeData) in
/// [`TreeAddData`] contains a non-existent subject.
///
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
    let mut transaction = db.begin().await?;

    // Verify the tree's existence and the user's permissions.
    verify_tree_permissions(&mut *transaction, tree_update_data.id, claims).await?;

    if !language_exists(&mut *transaction, &tree_update_data.new_data.language_code).await? {
        return Err(ApiError::MissingLanguage(
            tree_update_data.new_data.language_code.clone(),
        ));
    }
    if let Some(subject_id) = &tree_update_data.new_data.subject_id
        && !subject_exists(&mut *transaction, *subject_id).await?
    {
        return Err(ApiError::MissingSubject(*subject_id));
    }

    db::trees::update_tree_data(
        &mut transaction,
        tree_update_data.id,
        tree_update_data.new_data,
    )
    .await?;

    transaction.commit().await?;

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
    let (metadata, structure) = db_get_tree_all_data(get_connection!(db), tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    if !metadata.data.published {
        // If the tree is not published, only the author is allowed access
        let Some(claims) = claims else {
            return Err(ApiError::MissingAuthentication);
        };

        if claims.sub != metadata.gen_data.owner_id {
            return Err(ApiError::NoPermission);
        }
    }

    let tree_cards = get_tree_cards(&db, tree_id).await?;

    Ok(Json(TreeWithCards {
        metadata,
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
/// `db` - A reference to the database connection to use for the query.
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

    let mut transaction = db.begin().await?;

    let trees_count = get_queried_count(&mut *transaction, &filters, optional_user_id).await?;
    if trees_count == 0 {
        return Ok(Json(TreeListResponse {
            total_trees: 0,
            trees: Vec::new(),
        }));
    }

    let trees = get_queried_list(&mut *transaction, &page, &filters, optional_user_id).await?;

    transaction.commit().await?;

    let list = TreeListResponse {
        total_trees: i32::try_from(trees_count).map_err(|_| ApiError::DatabaseMemoryCap)?,
        trees,
    };

    Ok(Json(list))
}

#[utoipa::path(
    get,
    path = "/get-all-languages",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Successfully retrieved the list of languages.", body = Vec<String>),
        (status = 500, description = "Internal server error."),
    )
)]
/// Retrieve the list of all tree languages.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// # Returns
/// A list of all tree languages, as [`Vec<String>`].
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_all_languages(
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Language>>, ApiError> {
    Ok(Json(get_languages(&db).await?))
}

#[utoipa::path(
    get,
    path = "/get-all-subjects",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Successfully retrieved the list of subjects.", body = Vec<String>),
        (status = 500, description = "Internal server error."),
    )
)]
/// Retrieve the list of all tree subjects.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// # Returns
/// A list of all tree subjects, as [`Vec<String>`].
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_all_subjects(
    Extension(db): Extension<PgPool>,
) -> Result<Json<HashMap<String, Vec<Subject>>>, ApiError> {
    Ok(Json(get_subjects(&db).await?))
}
