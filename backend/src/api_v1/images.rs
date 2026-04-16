/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;

use crate::api_v1::session::Claims;
use crate::db::images::{add_image, check_image_existence};

use crate::errors::ApiError;
use axum::body::Bytes;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Router};
use infer;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::fmt::Write;
use uuid::Uuid;

/// Creates an `Axum` [`Router`] for the images category.
pub fn images() -> Router<Config> {
    Router::new().route("/add", post(add_new_image))
}

#[utoipa::path(
    post,
    path = "/add",
    context_path = "/api/v1/images",
    request_body(
        content(
            ("image/png"),
            ("image/jpeg"),
            ("image/apng"),
            ("image/avif"),
            ("image/gif"),
            ("image/svg+xml"),
            ("image/webp"),
        ),
    ),
    responses(
        (status = 200, description = "Image successfully added"),
        (status = 400, description = ApiError::NoImageBodyProvidedException.to_string()),
        (status = 400, description = ApiError::InvalidImageType.to_string()),
        (status = 500, description = ApiError::CreateDirException.to_string()),
        (status = 500, description = ApiError::FileWriteException.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Add an image to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `body` - The uploaded image as [`Bytes`].
/// 
/// # Returns
/// A permanent URL on which the image can be found.
///
/// # Errors
/// May return a [`CreateDirException`](ApiError::CreateDirException) if the upload directory cannot be found and created.
///
/// May return a [`NoImageBodyProvidedException`](ApiError::NoImageBodyProvidedException) if the body of the uploaded image is empty.
///
/// May return a [`InvalidImageType`](ApiError::InvalidImageType) if the uploaded image is not of a image/ MIME-type.
///
/// May return a [`FileWriteException`](ApiError::FileWriteException) if the uploaded image cannot be written to the uploads directory.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_image(
    claims: Claims,
    State(config): State<Config>,
    Extension(db): Extension<PgPool>,
    body: Bytes,
) -> Result<String, ApiError> {
    let upload_directory = &config.upload_dir;

    // First check whether the directory exists, creates it if not.
    ensure_directory_existence(upload_directory).await?;

    // Checks whether the image is of MIME-type image/ and saves the image extension.
    let image_type = validate_image(&body)?;
    let extension = image_type.extension().to_string();

    // Creates a hash of the image.
    let hash = hash_image(&body);

    // Checks whether the image already exists in de database based on its hash.
    // If so, returns the URL where the already existing image can be found.
    if let Some(existing_filename) = check_image_existence(&db, &hash).await? {
        return Ok(format!("/uploads/{existing_filename}"));
    }

    // Create image filename using a uuid and extension.
    let uuid = Uuid::new_v4();
    let filename = format!("{uuid}.{extension}");
    let filepath = format!("{upload_directory}/{filename}");

    // Add the image metadata to the database in transaction.
    let mut transaction = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    add_image(&mut *transaction, &filename, &hash, claims.sub).await?;

    // Write the image to the directory.
    tokio::fs::write(&filepath, &body)
        .await
        .map_err(|_| ApiError::FileWriteException)?;

    // Finalize transaction.
    transaction
        .commit()
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    // Return where the image can be found
    Ok(format!("/uploads/{filename}"))
}

/// Check whether the upload directory configured in config exists. If not, creates the directory.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return an [`CreateDirException`](ApiError::CreateDirException) if the upload directory can not be found and created.
async fn ensure_directory_existence(upload_directory: &str) -> Result<(), ApiError> {
    tokio::fs::create_dir_all(upload_directory)
        .await
        .map_err(|_| ApiError::CreateDirException)?;

    Ok(())
}

/// Check whether the image type is a valid image of MIME-type image/.
///
/// # Returns
/// The type of the image.
///
/// # Errors
/// May return a [`NoImageBodyProvidedException`](ApiError::NoImageBodyProvidedException) if the body of the uploaded image is empty.
///
/// May return a [`InvalidImageType`](ApiError::InvalidImageType) if the uploaded image is not of a image/ MIME-type.
fn validate_image(image: &Bytes) -> Result<infer::Type, ApiError> {
    let image_type = infer::get(image).ok_or(ApiError::NoImageBodyProvidedException)?;
    if !image_type.mime_type().starts_with("image/") {
        return Err(ApiError::InvalidImageType);
    }
    Ok(image_type)
}

/// Creates a Sha256 hash of the image used as argument.
///
/// # Returns
/// A string containing the hash of the image.
fn hash_image(body: &Bytes) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body);
    let hash = hasher.finalize();

    hash.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    })
}
