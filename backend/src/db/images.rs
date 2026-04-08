/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::images::ImageAddData;

use crate::errors::ApiError;
use sqlx::{PgExecutor, query};

/// Add a new image to the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool).
///
/// `image_data` - The data assigned to the new image as [`ImageAddData`].
///
/// # Returns
/// The URL on which the image can be found if succesful.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_image(db: impl PgExecutor<'_>, image_data: &ImageAddData) -> Result<(), ApiError> {
    query!(
        "INSERT INTO images (filename, hash)
        VALUES ($1, $2)",
        image_data.file_name,
        image_data.hash,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

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
/// The `file_name` of the image if the image exists in the database.
///
/// # Error
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
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(file_name.map(|r| r.filename))
}
