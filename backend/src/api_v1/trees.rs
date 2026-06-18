/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db::cards::{add_card, get_tree_cards, remove_card};
use crate::db::trees::{
    add_co_author, add_language, add_subject, add_tree, db_get_tree_all_data, get_languages,
    get_queried_count, get_queried_list, get_subjects, get_tree_owner, is_co_author_of_tree,
    is_structure_valid, language_exists, remove_co_author, subject_exists, subject_id_exists,
    verify_tree_accessible_by_user,
};
use crate::db::users::get_user;
use crate::db::users::get_user_by_name;
use crate::db_structs::pagination::OptionalPaginationOptions;
use crate::db_structs::session::Claims;
use crate::db_structs::trees::{
    Language, Node, RootNode, Subject, SubjectData, TreeAddData, TreeData, TreeFilters,
    TreeListResponse, TreeStructure, TreeWithCards,
};
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::limits::TREE_TITLE_MAX_LENGTH;
use crate::{ArcState, db, get_connection};

use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool};
use utoipa::ToSchema;

use crate::db_structs::Validated;
use crate::db_structs::cards::CardData;
use crate::messages::Message;
use std::collections::HashMap;

pub mod edit_lock;

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<ArcState> {
    Router::new()
        .route("/", get(get_list).post(add_new_tree))
        .route("/languages", get(get_all_languages).post(add_new_language))
        .route("/languages/{code}", put(update_language))
        .route("/subjects", get(get_all_subjects).post(add_new_subject))
        .route("/subjects/{id}", put(update_subject))
        .route(
            "/{tree_id}",
            get(get_tree_with_cards)
                .put(update_tree_data)
                .delete(remove_tree),
        )
        .route("/{tree_id}/duplicate", post(duplicate_tree))
        .route("/{tree_id}/structure", put(update_tree_structure))
        .route("/{tree_id}/cards", post(add_new_card))
        .route("/{tree_id}/cards/{card_id}", delete(remove_card_from_tree))
        .route(
            "/{tree_id}/co-authors/{username}",
            post(authorize_co_author).delete(deauthorize_co_author),
        )
        .route(
            "/{tree_id}/edit-lock",
            // This endpoint requires to be wrapped in an [`axum::routing::any`],
            // see the docs of [`WebSocketUpgrade`](axum::extract::WebSocketUpgrade) for why.
            axum::routing::any(edit_lock::acquire_edit_lock),
        )
}

#[utoipa::path(
    post,
    path = "",
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_tree(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(tree_add_data): Json<TreeAddData>,
) -> Result<Json<i32>, ApiError> {
    tree_add_data.validate_inputs()?;

    let mut transaction = db.begin().await?;
    if !language_exists(&mut *transaction, &tree_add_data.tree_data.language_code).await? {
        return Err(ApiError::MissingLanguage(
            tree_add_data.tree_data.language_code,
        ));
    }
    if let Some(subject_id) = &tree_add_data.tree_data.subject_id
        && !subject_id_exists(&mut *transaction, *subject_id).await?
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
    path = "/{tree_id}",
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn remove_tree(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
) -> Result<(), ApiError> {
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;

    db::trees::remove_tree(&db, tree_id).await
}

#[utoipa::path(
    put,
    path = "/{tree_id}/structure",
    context_path = "/api/v1/trees",
    request_body = TreeStructure,
    responses(
        (status = 200, description = "Tree structure successfully updated."),
        (status = 400, description = "Tree with given id cannot be found or the new structure is not valid."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("tree_id" = i32, Path),
    )
)]
/// Update the structure of an existing tree in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `new_structure` - The new structure, as [`TreeStructure`].
/// Overwrites the current structure.
///
/// `tree_id` - The tree to update.
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) or a [`SerdeException`](ApiError::SerdeException) instead.
async fn update_tree_structure(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
    Json(new_structure): Json<TreeStructure>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;

    let mut transaction = db.begin().await?;

    update_structure_if_valid(&mut transaction, tree_id, new_structure).await?;

    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/{tree_id}/cards",
    context_path = "/api/v1/trees",
    request_body = CardData,
    responses(
        (status = 200, description = "Card successfully added with returned id.", body = i32, content_type = "application/json"),
        (status = 400, description = "Tree with given id does not exist."),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("tree_id" = i32, Path),
    )
)]
/// Add a card to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_id` - The tree to add the card to.
///
/// `card_data` - The data used of the card, as [`CardData`].
///
/// # Returns
/// The id assigned to the new card.
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with the `tree_id` referenced in `card_data` exists.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the tree for the given `tree_id` referenced in `card_data`.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_card(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
    Json(card_data): Json<CardData>,
) -> Result<Json<i32>, ApiError> {
    card_data.validate_inputs()?;
    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;

    let id = add_card(&db, tree_id, &card_data).await?;

    Ok(Json(id))
}

#[utoipa::path(
    delete,
    path = "/{tree_id}/cards/{card_id}",
    context_path = "/api/v1/trees",
    request_body = TreeStructure,
    responses(
        (status = 200, description = "Card successfully removed from tree."),
        (status = 400, description = "Tree or card with given id cannot be found or the new structure is not valid."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("tree_id" = i32, Path),
        ("card_id" = i32, Path),
    )
)]
/// Remove a card from a tree.
///
/// Removes the card from the database table `cards`, and overwrites the current tree structure,
/// if the provided new structure is still valid when the card is removed.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `new_structure` - The new tree structure, as [`TreeStructure`], not containing the deleted card.
///
/// `tree_id` - The tree on which to remove the card.
///
/// `card_id` - The card to remove from the tree.
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`SerdeException`](ApiError::SerdeException)
/// or [`DatabaseException`](ApiError::DatabaseException) instead.
async fn remove_card_from_tree(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path((tree_id, card_id)): Path<(i32, i32)>,
    Json(new_structure): Json<TreeStructure>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;

    let mut transaction = db.begin().await?;

    // First, we remove the card from the database, within the transaction.
    remove_card(&mut *transaction, card_id).await?;

    // Then, we check if the structure is still valid with the card being removed.
    // This ensures the card is actually removed from the structure.
    update_structure_if_valid(&mut transaction, tree_id, new_structure).await?;

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
    put,
    path = "/{tree_id}",
    context_path = "/api/v1/trees",
    request_body = TreeData,
    responses(
        (status = 200, description = "Tree data updated successfully."),
        (status = 400, description = "Tree with given id cannot be found, invalid ISBN provided, non-existent language provided or non-existent subject provided."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("tree_id" = i32, Path),
    )
)]
/// Update the data of an existing tree in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_id` - The tree to update.
///
/// `new_data` - The new data of the tree, as [`TreeData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if `new_data` contains an invalid ISBN.
///
/// Returns [`MissingLanguage`](ApiError::MissingLanguage) if `new_data` contains a non-existent language.
///
/// Returns [`MissingSubject`](ApiError::MissingSubject) if `new_data` contains a non-existent subject.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with `tree_id` exists.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the tree for the given `tree_id`.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_tree_data(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
    Json(new_data): Json<TreeData>,
) -> Result<(), ApiError> {
    new_data.validate_inputs()?;
    let mut transaction = db.begin().await?;

    // Verify the tree's existence and the user's permissions.
    verify_tree_permissions(&mut transaction, tree_id, &claims).await?;

    if !language_exists(&mut *transaction, &new_data.language_code).await? {
        return Err(ApiError::MissingLanguage(new_data.language_code.clone()));
    }
    if let Some(subject_id) = &new_data.subject_id
        && !subject_id_exists(&mut *transaction, *subject_id).await?
    {
        return Err(ApiError::MissingSubject(*subject_id));
    }

    db::trees::update_tree_data(&mut transaction, tree_id, new_data).await?;

    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/{tree_id}",
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
/// `claims` - The session cookie of the user, as an [`Option`]. Necessary if the user
/// is retrieving an unpublished tree.
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
/// and the user is not its owner, or an admin.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_tree_with_cards(
    claims: Option<Claims>,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
) -> Result<Json<TreeWithCards>, ApiError> {
    verify_tree_accessible_by_user(get_connection!(db), tree_id, claims.as_ref()).await?;

    let (metadata, structure) = db_get_tree_all_data(get_connection!(db), tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

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
    db: &mut PgConnection,
    tree_id: i32,
    claims: &Claims,
) -> Result<(), ApiError> {
    // Check whether the tree exists.
    let owner_id = get_tree_owner(&mut *db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    if claims.account_type == AccountType::Admin
        || claims.sub == owner_id
        || is_co_author_of_tree(db, &tree_id, &claims.sub).await?
    {
        Ok(())
    } else {
        Err(ApiError::NoPermission)
    }
}

#[utoipa::path(
    get,
    path = "",
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
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

    let mut transaction = db.begin().await?;

    let trees_count = get_queried_count(&mut *transaction, &filters, claims.as_ref()).await?;
    if trees_count == 0 {
        return Ok(Json(TreeListResponse {
            total_trees: 0,
            trees: Vec::new(),
        }));
    }
    let trees = get_queried_list(&mut *transaction, &page, &filters, claims.as_ref()).await?;

    transaction.commit().await?;

    let list = TreeListResponse {
        total_trees: i32::try_from(trees_count).map_err(|_| ApiError::DatabaseMemoryCap)?,
        trees,
    };

    Ok(Json(list))
}

#[derive(Serialize, Deserialize, ToSchema, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum AuthorizeCoAuthorResponse {
    /// Return body signaling that the user has been successfully added to the tree as co-author.
    Success,
    /// Return body signaling that the given username does not correspond with a user in the database.
    UsernameDoesNotExist,
    /// Return body signaling that the given user is already a co-author to the tree, or the owner of the tree.
    UserAlreadyAuthorized,
}

#[utoipa::path(
    post,
    path = "/{tree_id}/co-authors/{username}",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Successfully attempted to add co-author to tree.", body = AuthorizeCoAuthorResponse, content_type = "application/json"),
        (status = 400, description = "Tree with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("tree_id" = i32, Path),
        ("username" = String, Path),
        ("session" = Claims, Cookie),
    )
)]
/// Add a user to a tree to make them a co-author.
///
/// Only the owner of the given tree or an admin can add a user to become a co-author of the tree.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_id` - The id of the tree for which a co-author will be added.
///
/// `username` - The user to become co-author.
///
/// # Returns
/// [`UsernameDoesNotExist`](AuthorizeCoAuthorResponse::UsernameDoesNotExist) if no user with the corresponding given username can be found, or
/// [`UserAlreadyAuthorized`](AuthorizeCoAuthorResponse::UserAlreadyAuthorized) if the user has already been authorized as co-author, or is the owner of the tree, otherwise
/// [`Success`](AuthorizeCoAuthorResponse::Success), meaning the user has successfully been authorized as co-author.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to add a co-author to the tree.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if the tree to which the user is added as co-author does not exist.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn authorize_co_author(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path((tree_id, username)): Path<(i32, String)>,
) -> Result<Json<AuthorizeCoAuthorResponse>, ApiError> {
    let owner_id = get_tree_owner(&db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    // A user can only add a co-author to their own tree, unless they are an admin.
    if claims.sub != owner_id && claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    let Some(user_id) = get_user_by_name(&db, &username).await? else {
        return Ok(Json(AuthorizeCoAuthorResponse::UsernameDoesNotExist));
    };

    if user_id == owner_id || is_co_author_of_tree(&db, &tree_id, &user_id).await? {
        return Ok(Json(AuthorizeCoAuthorResponse::UserAlreadyAuthorized));
    }

    add_co_author(&db, tree_id, user_id).await?;

    Ok(Json(AuthorizeCoAuthorResponse::Success))
}

#[utoipa::path(
    delete,
    path = "/{tree_id}/co-authors/{username}",
    context_path = "/api/v1/trees",
    responses(
        (status = 200, description = "Successfully removed co-author from tree."),
        (status = 400, description = "Tree with given id cannot be found or the given username cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("tree_id" = i32, Path),
        ("username" = String, Path),
        ("session" = Claims, Cookie),
    )
)]
/// Remove a co-author from a tree.
///
/// Only the owner of the given tree or an admin can do this.
///
/// If the given user was already not a co-author of the given tree,
/// this function will still return a 200 OK status.
/// This is also the case when trying to remove the owner of a tree as co-author,
/// because the owner of tree can never become a co-author.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_id` - The id of the tree for which a co-author will be removed.
///
/// `username` - The user to revoke co-author rights from.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to remove a co-author from the tree.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if the tree from which the user is removed as co-author does not exist.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn deauthorize_co_author(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path((tree_id, username)): Path<(i32, String)>,
) -> Result<(), ApiError> {
    let owner_id = get_tree_owner(&db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    // A user can only remove a co-author to their own tree, unless they are an admin.
    if claims.sub != owner_id && claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    // The frontend knows what all possible allowed usernames for this endpoint call are.
    // So we can just return an ApiError here in the case where the username does not exist.
    let Some(user_id) = get_user_by_name(&db, &username).await? else {
        return Err(ApiError::MissingUsername(username));
    };

    remove_co_author(&db, tree_id, user_id).await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/languages",
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
    post,
    path = "/languages",
    context_path = "/api/v1/trees",
    request_body = Language,
    responses(
        (status = 200, description = "Successfully added a new language."),
        (status = 400, description = "The language already exists."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Add a new language.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `language` - The language to add to the database, as [`Language`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user is not an admin.
///
/// Returns [`LanguageExists`](ApiError::LanguageExists) if the language already exists.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_new_language(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(language): Json<Language>,
) -> Result<(), ApiError> {
    if claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    let mut transaction = db.begin().await?;

    if language_exists(&mut *transaction, &language.code).await? {
        return Err(ApiError::LanguageExists(language.code));
    }

    add_language(&mut *transaction, &language).await?;

    transaction.commit().await?;
    Ok(())
}

#[utoipa::path(
    put,
    path = "/languages/{code}",
    context_path = "/api/v1/trees",
    request_body = String,
    responses(
        (status = 200, description = "Successfully updated the language."),
        (status = 400, description = "The language does not exist."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("code" = String, Path),
    )
)]
/// Updates an existing language.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `code` - The code of the language to update.
///
/// `new_name` - The new name for the language.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user is not an admin.
///
/// Returns [`MissingLanguage`](ApiError::MissingLanguage) if the given language code does not exist.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_language(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(code): Path<String>,
    Json(new_name): Json<String>,
) -> Result<(), ApiError> {
    if claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    let mut transaction = db.begin().await?;

    if !language_exists(&mut *transaction, &code).await? {
        return Err(ApiError::MissingLanguage(code));
    }

    db::trees::update_language(&mut *transaction, &code, &new_name).await?;

    transaction.commit().await?;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/subjects",
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

#[utoipa::path(
    post,
    path = "/subjects",
    context_path = "/api/v1/trees",
    request_body = SubjectData,
    responses(
        (status = 200, description = "Successfully added a new subject."),
        (status = 400, description = "The subject already exists or contains a non-existent language code."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Add a new subject.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `subject` - The subject to add to the database, as [`SubjectData`].
///
/// # Returns
/// The id of the newly added subject.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user is not an admin.
///
/// Returns [`SubjectExists`](ApiError::SubjectExists) if the subject already exists.
///
/// Returns [`MissingLanguage`](ApiError::MissingLanguage) if the new subject contains a non-existent
/// language code.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_new_subject(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(subject_data): Json<SubjectData>,
) -> Result<Json<i32>, ApiError> {
    if claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    let mut transaction = db.begin().await?;

    if subject_exists(&mut *transaction, &subject_data).await? {
        return Err(ApiError::SubjectExists(subject_data.name));
    }
    if !language_exists(&mut *transaction, &subject_data.language_code).await? {
        return Err(ApiError::MissingLanguage(subject_data.language_code));
    }

    let id = add_subject(&mut *transaction, &subject_data).await?;

    transaction.commit().await?;
    Ok(Json(id))
}

#[utoipa::path(
    put,
    path = "/subjects/{id}",
    context_path = "/api/v1/trees",
    request_body = SubjectData,
    responses(
        (status = 200, description = "Successfully updated the subject."),
        (status = 400, description = "The subject does not exist or the new data contains a non-existent language code."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("id" = i32, Path),
    )
)]
/// Updates an existing subject.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `id` - The id of the subject to update.
///
/// `new_data` - The new subject data, as [`SubjectData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user is not an admin.
///
/// Returns [`MissingSubject`](ApiError::LanguageExists) if the given subject id does not exist.
///
/// Returns [`MissingLanguage`](ApiError::MissingLanguage) if the new subject data contains a non-existent
/// language code.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_subject(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(new_data): Json<SubjectData>,
) -> Result<(), ApiError> {
    if claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    let mut transaction = db.begin().await?;

    if !subject_id_exists(&mut *transaction, id).await? {
        return Err(ApiError::MissingSubject(id));
    }
    if !language_exists(&mut *transaction, &new_data.language_code).await? {
        return Err(ApiError::MissingLanguage(new_data.language_code));
    }

    db::trees::update_subject(&mut *transaction, id, &new_data).await?;

    transaction.commit().await?;
    Ok(())
}

#[utoipa::path(
    post,
    path = "/{tree_id}/duplicate",
    context_path = "/api/v1/trees",

    responses(
        (status = 200, description = "Successfully duplicated the tree.", body = i32),
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
/// Duplicates a tree.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `tree_id` - Tree to duplicate.
///
/// # Returns
/// The id assigned to the new tree.
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with `tree_id` exists.
///
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user is not logged in.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to duplicate the tree
/// for the given `tree_id`.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn duplicate_tree(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(tree_id): Path<i32>,
) -> Result<Json<i32>, ApiError> {
    let mut transaction = db.begin().await?;

    let (metadata, structure) = db_get_tree_all_data(&mut transaction, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    if !metadata.data.published {
        verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;
    }
    let mut new_data = metadata.data;
    new_data.published = false;
    let this_user = get_user(&db, claims.sub).await?;
    new_data.title = format!(
        "{} {}",
        Message::CopyOf.localized(&this_user.data.language),
        new_data.title
    );
    if new_data.title.chars().count() > TREE_TITLE_MAX_LENGTH {
        new_data.title = new_data.title[0..(TREE_TITLE_MAX_LENGTH - 1)].to_owned() + "…";
    }

    let tree_add_data = TreeAddData {
        tree_data: new_data,
        tree_structure: structure.clone(),
    };
    let new_tree_id = add_tree(&mut transaction, &tree_add_data, claims.sub).await?;

    let mut new_structure: Vec<RootNode> = Vec::with_capacity(structure.root_nodes.len());
    for root_node in structure.root_nodes {
        let mut duplicate_children: Vec<Node> = Vec::with_capacity((root_node.children).len());
        for child in &root_node.children {
            duplicate_children.push(child.duplicate(&mut transaction, new_tree_id).await?);
        }
        new_structure.push(RootNode {
            label: root_node.label.clone(),
            children: duplicate_children,
        });
    }

    update_structure_if_valid(
        &mut transaction,
        new_tree_id,
        TreeStructure {
            root_nodes: new_structure,
        },
    )
    .await?;

    transaction.commit().await?;

    Ok(Json(new_tree_id))
}
