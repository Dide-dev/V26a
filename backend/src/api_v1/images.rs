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
use std::fmt::Write;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
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
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
    let upload_directory = &config.upload_dir;

    tokio::fs::create_dir_all(upload_directory)
        .await
        .map_err(|_| ApiError::CreateDirException)?;

    let original_filename: Option<String>;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::MultipartException)?
    {
        if field.name().unwrap_or("") == "file" {
            original_filename = field.file_name().map(&str::to_string);
            let uuid = Uuid::new_v4();
            let temp_filename = format!("{uuid}.tmp");
            let temp_filepath = format!("{upload_directory}/{temp_filename}");

            let mut file = File::create(&temp_filepath)
                .await
                .map_err(|_| ApiError::FileWriteException)?;

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
                    tokio::fs::remove_file(&temp_filepath).await.ok();
                    return Err(ApiError::FileTooBigException);
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
            let hash = hash.iter().fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:02X}");
                output
            });

            if let Some(existing_filename) = check_image_existence(&db, &hash).await? {
                tokio::fs::remove_file(&temp_filepath).await.ok();
                return Ok(format!("{upload_directory}/{existing_filename}"));
            }

            let extension = original_filename
                .as_ref()
                .and_then(|name| name.split('.').next_back())
                .unwrap_or("bin");

            let filename = format!("{uuid}.{extension}");
            let filepath = format!("{upload_directory}/{filename}");

            tokio::fs::rename(&temp_filepath, &filepath)
                .await
                .map_err(|_| ApiError::FileRenameException)?;

            let image_add_data = ImageAddData {
                file_name: filename.clone(),
                hash,
            };

            add_image(&db, &image_add_data).await?;

            return Ok(filepath.clone());
        }
    }
    Err(ApiError::DatabaseException)
}
