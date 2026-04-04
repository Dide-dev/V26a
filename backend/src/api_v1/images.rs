/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;

use crate::db::images::{add_image, check_image_existence};
use crate::db_structs::images::ImageAddData;

use crate::errors::ApiError;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Router, extract::Multipart};
use infer;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tracing::info;
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
/// [`create_dir_all`] returns an error, according to its documentation, if:
/// * User lacks permissions to create directory at `path`.
/// * A parent of the given path doesn’t exist.
/// * `path` already exists.
/// * Any directory specified in `path` does not exist and could not be created.
async fn add_new_image(
    State(config): State<Config>,
    Extension(db): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<String, ApiError> {
    info!("Starting image upload");
    let upload_directory = &config.upload_dir;
    info!("Upload directory: {:?}", upload_directory);

    tokio::fs::create_dir_all(upload_directory)
        .await
        .map_err(|_| ApiError::DatabaseException)?;
    info!("Create dir all finished");
    let mut image_data: Option<Vec<u8>> = None;
    let mut original_filename: Option<String> = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::InvalidDataFound)?
    {
        info!("Field name: {:?}", field.name());
        info!("File name: {:?}", field.file_name());
        if field.name().unwrap_or("") == "file" {
            original_filename = field.file_name().map(|s| s.to_string());
            let bytes = field
                .bytes()
                .await
                .map_err(|_| ApiError::InvalidDataFound)?;
            image_data = Some(bytes.to_vec());
        }
    }

    let image_data = image_data.ok_or(ApiError::InvalidDataFound)?;
    info!("File size: {}", image_data.len());

    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
    if image_data.len() > MAX_FILE_SIZE {
        return Err(ApiError::InvalidDataFound);
    }

    let image_type = infer::get(&image_data);
    match image_type {
        Some(it) if it.mime_type().starts_with("image/") => {}
        _ => return Err(ApiError::InvalidDataFound),
    }

    let mut hasher = Sha256::new();
    hasher.update(&image_data);
    let hash = hasher.finalize();
    let hash = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    info!("Generated hash: {}", hash);
    let existing = check_image_existence(&db, &hash).await?;
    if let Some(file_name) = existing {
        info!("Duplicate file, giving existing URL");
        return Ok(format!("/uploads/{}", file_name));
    }

    let extension = original_filename
        .as_ref()
        .and_then(|name| name.split('.').last())
        .unwrap_or("bin");
    let uuid = Uuid::new_v4();
    let filename = format!("{uuid}.{extension}");
    let filepath = format!("{upload_directory}/{filename}");

    tokio::fs::write(&filepath, &image_data)
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    info!("Saving file to: {}", filepath);
    let url = format!("/uploads/{}", filename);

    let image_add_data = ImageAddData {
        file_name: filename,
        hash,
    };
    add_image(&db, &image_add_data).await?;

    Ok(url)

    //TODO REWRITE DATABASE EXCEPTIONS.
}
