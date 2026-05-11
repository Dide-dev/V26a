/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use sqlx::{PgExecutor, query};

/// Add a new image to the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `file_name` - The filename generated for the image.
///
/// `hash` - The hash of the image.
///
/// `owner_id` - The id of the user that added the image originally.
///
/// # Returns
/// Nothing
///
/// # Errors
/// May return [`ImageAddFail`](ApiError::ImageAddFail) instead.
pub async fn add_image(
    db: impl PgExecutor<'_>,
    file_name: &str,
    hash: &str,
    owner_id: i32,
) -> Result<(), ApiError> {
    query!(
        "INSERT INTO images (hash, owner_id, filename)
        VALUES ($1, $2, $3)",
        hash,
        owner_id,
        file_name,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::ImageAddFail)?;

    Ok(())
}

/// Check whether an image already exists in the database based on its hash.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool).
///
/// `hash` - The hash of the image.
///
/// # Returns
/// The `file_name` of the image if the image exists in the database, or None if it does not.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn check_image_existence(
    db: impl PgExecutor<'_>,
    hash: &String,
) -> Result<Option<String>, ApiError> {
    let file_name = query!(
        "SELECT filename
        FROM images
        WHERE hash = $1",
        hash
    )
    .fetch_optional(db)
    .await?;

    Ok(file_name.map(|r| r.filename))
}
