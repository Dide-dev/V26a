/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;

use crate::api_v1::session::Claims;
use crate::db::images::{add_image, check_image_existence};
use crate::db_structs::images::ImageAddData;

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

/// Add an image to the database.
///
/// # Returns
/// A permanent URL on which the image can be found.
/// 
/// # Error
/// May return a  [`FileTooBigException`](ApiError::FileTooBigException) if the uploaded file was too big.
/// 
/// May return a  [`CreateDirException`](ApiError::CreateDirException) if the upload directory cannot be found and created.
/// 
/// May return a [`NoImageBodyProvidedException`](ApiError::NoImageBodyProvidedException) if the body of the uploaded image is empty.
/// 
/// May return a [`InvalidImageType`](ApiError:InvalidImageType) if the uploaded image is not of a image/ MIME-type.
/// 
/// May return a [`FileWriteException`](ApiError:FileWriteException) if the uploaded image cannot be written to the uploads directory.
/// 
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_image(
    claims: Claims,
    State(config): State<Config>,
    Extension(db): Extension<PgPool>,
    body: Bytes,
) -> Result<String, ApiError> {
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
    if body.len() > MAX_FILE_SIZE {
        return Err(ApiError::FileTooBigException);
    }

    let upload_directory = &config.upload_dir;

    tokio::fs::create_dir_all(upload_directory)
        .await
        .map_err(|_| ApiError::CreateDirException)?;

    let image_type = infer::get(&body).ok_or(ApiError::NoImageBodyProvidedException)?;
    if !image_type.mime_type().starts_with("image/") {
        return Err(ApiError::InvalidImageType);
    }
    let extension = image_type.extension();

    let mut hasher = Sha256::new();
    hasher.update(&body);
    let hash = hasher.finalize();

    let hash = hash.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    });

    if let Some(existing_filename) = check_image_existence(&db, &hash).await? {
        return Ok(format!("/uploads/{existing_filename}"));
    }

    let uuid = Uuid::new_v4();
    let filename = format!("{uuid}.{extension}");
    let filepath = format!("{upload_directory}/{filename}");

    tokio::fs::write(&filepath, &body)
        .await
        .map_err(|_| ApiError::FileWriteException)?;

    let image_add_data = ImageAddData {
        file_name: filename.clone(),
        hash,
    };

    add_image(&db, &image_add_data, claims.sub).await?;

    Ok(format!("/uploads/{filename}"))
}
