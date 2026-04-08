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
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
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
        .map_err(|_| ApiError::CreateDirException)?;
    info!("Create dir all finished");

    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
    // let mut image_data: Option<Vec<u8>> = None;
    let original_filename: Option<String>;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::MultipartException)?
    {
        info!("Field name: {:?}", field.name());
        info!("File name: {:?}", field.file_name());
        if field.name().unwrap_or("") == "file" {
            info!("{:?} equals file", field.name());
            original_filename = field.file_name().map(|s| s.to_string());
            info!("Original filename: {:?}", original_filename);
            let uuid = Uuid::new_v4();
            let temp_filename = format!("{uuid}.tmp");
            let temp_filepath = format!("{upload_directory}/{temp_filename}");

            info!("temp_filename: {temp_filename}");
            info!("temp_filepath: {temp_filepath}");

            let mut file = File::create(&temp_filepath)
                .await
                .map_err(|_| ApiError::FileWriteException)?;

            info!("file: {:?}", file);

            let mut hasher = Sha256::new();
            let mut size = 0;

            let mut first_chunk: Option<Vec<u8>> = None;

            let mut field = field;

            while let Some(chunk) = field
                .chunk()
                .await
                .map_err(|_| ApiError::MultipartException)?
            {
                size += chunk.len();

                if size > MAX_FILE_SIZE {
                    info!("File size of {size} is too big");
                    tokio::fs::remove_file(&temp_filepath).await.ok();
                    return Err(ApiError::FileToBigException);
                }

                if first_chunk.is_none() {
                    first_chunk = Some(chunk.to_vec());
                }

                hasher.update(&chunk);

                file.write_all(&chunk)
                    .await
                    .map_err(|_| ApiError::FileWriteException)?;
            }

            let image_type = infer::get(first_chunk.as_ref().ok_or(ApiError::InvalidDataFound)?);
            match image_type {
                Some(it) if it.mime_type().starts_with("image/") => {}
                _ => return Err(ApiError::InvalidImageType),
            }

            let hash = hasher.finalize();
            let hash = hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            info!("Generated hash: {}", hash);

            if let Some(existing_filename) = check_image_existence(&db, &hash).await? {
                tokio::fs::remove_file(&temp_filepath).await.ok();
                info!("Duplicate file, giving existing URL");
                return Ok(format!("{upload_directory}/{existing_filename}"));
            }

            let extension = original_filename
                .as_ref()
                .and_then(|name| name.split('.').last())
                .unwrap_or("bin");

            info!("File extension: {extension}");

            let filename = format!("{uuid}.{extension}");
            let filepath = format!("{upload_directory}/{filename}");

            tokio::fs::rename(&temp_filepath, &filepath)
                .await
                .map_err(|_| ApiError::FileRenameException)?;
            info!("temp file renamed from {temp_filepath} to {filepath}");
            let image_add_data = ImageAddData {
                file_name: filename.clone(),
                hash,
            };

            info!("Saving file to: {}", filepath);
            add_image(&db, &image_add_data).await?;

            return Ok(format!("{filepath}"));
        }
    }
    Err(ApiError::DatabaseException)
}

