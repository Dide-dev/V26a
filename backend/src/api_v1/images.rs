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

    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
    // let mut image_data: Option<Vec<u8>> = None;
    let original_filename: Option<String>;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::InvalidDataFound)?
    {
        info!("Field name: {:?}", field.name());
        info!("File name: {:?}", field.file_name());
        if field.name().unwrap_or("") == "file" {
            info!("{:?} equals file", field.name());
            original_filename = field.file_name().map(|s| s.to_string());
            info!("Original filename: {:?}", original_filename);
            // let bytes = field
            //     .bytes()
            //     .await
            //     .map_err(|_| ApiError::InvalidDataFound)?;
            // image_data = Some(bytes.to_vec());

            let uuid = Uuid::new_v4();
            let temp_filename = format!("{uuid}.tmp");
            let temp_filepath = format!("{upload_directory}/{temp_filename}");

            info!("temp_filename: {temp_filename}");
            info!("temp_filepath: {temp_filepath}");

            let mut file = File::create(&temp_filepath)
                .await
                .map_err(|_| ApiError::DatabaseException)?;

            info!("file: {:?}", file);

            let mut hasher = Sha256::new();
            let mut size = 0;

            let mut first_chunk: Option<Vec<u8>> = None;

            let mut field = field;

            while let Some(chunk) = field
                .chunk()
                .await
                .map_err(|_| ApiError::InvalidDataFound)?
            {
                //info!("chunk: {:?}", chunk);
                size += chunk.len();

                if size > MAX_FILE_SIZE {
                    info!("File size of {size} is too big");
                    tokio::fs::remove_file(&temp_filepath).await.ok();
                    return Err(ApiError::InvalidDataFound);
                }

                if first_chunk.is_none() {
                    first_chunk = Some(chunk.to_vec());
                }

                hasher.update(&chunk);

                file.write_all(&chunk)
                    .await
                    .map_err(|_| ApiError::DatabaseException)?;
            }

            let image_type = infer::get(first_chunk.as_ref().ok_or(ApiError::InvalidDataFound)?);
            match image_type {
                Some(it) if it.mime_type().starts_with("image/") => {}
                _ => return Err(ApiError::InvalidDataFound),
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
                .map_err(|_| ApiError::DatabaseException)?;
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
    Err(ApiError::InvalidDataFound)
}

// let image_data = image_data.ok_or(ApiError::InvalidDataFound)?;
// info!("File size: {}", image_data.len());

// const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
// if image_data.len() > MAX_FILE_SIZE {
//     return Err(ApiError::InvalidDataFound);
// }

// let image_type = infer::get(&image_data);
// match image_type {
//     Some(it) if it.mime_type().starts_with("image/") => {}
//     _ => return Err(ApiError::InvalidDataFound),
// }

// let mut hasher = Sha256::new();
// hasher.update(&image_data);
// let hash = hasher.finalize();
// let hash = hash
//     .iter()
//     .map(|b| format!("{:02x}", b))
//     .collect::<String>();
// info!("Generated hash: {}", hash);
// let existing = check_image_existence(&db, &hash).await?;
// if let Some(file_name) = existing {
//     info!("Duplicate file, giving existing URL");
//     return Ok(format!("/uploads/{}", file_name));
// }

// let extension = original_filename
//     .as_ref()
//     .and_then(|name| name.split('.').last())
//     .unwrap_or("bin");
// let uuid = Uuid::new_v4();
// let filename = format!("{uuid}.{extension}");
// let filepath = format!("{upload_directory}/{filename}");

// tokio::fs::write(&filepath, &image_data)
//     .await
//     .map_err(|_| ApiError::DatabaseException)?;

// info!("Saving file to: {}", filepath);
// let url = format!("/uploads/{}", filename);

// let image_add_data = ImageAddData {
//     file_name: filename,
//     hash,
// };
// add_image(&db, &image_add_data).await?;

// Ok(url)

// //TODO REWRITE DATABASE EXCEPTIONS.
// let uuid = Uuid::new_v4();
// let filename = format!("{uuid}.bin");
// let filepath = format!("{upload_directory}/{filename}");

// let mut file = tokio::fs::File::create(&filepath)
//     .await
//     .map_err(|_| ApiError::DatabaseException)?;

// let mut hasher = Sha256::new();
// let mut size = 0;

// while let Some(chunk) = field
//     .chuck()
//     .await
//     .map_err(|_| ApiError::InvalidDataFound)?
// {
//     size += chunck.len;

//     if size > MAX_FILE_SIZE {
//         return Err(ApiError::InvalidDataFound);
//     }

//     hasher.update(&chunk);

//     file.write_all(&chunk).await
// }

// let hash = hasher.finalize();
// let hash = format!("{:x}", hash);

// if let Some(file_name) = existing {
//     remove_file(filename);
//     info!("Duplicate file, giving existing URL");
//     return Ok(format!("/uploads/{}", file_name));
// }
