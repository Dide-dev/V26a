/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::Validated;
use crate::errors::ApiError;
use crate::limits::{CLASSROOM_DESCRIPTION_MAX_LENGTH, CLASSROOM_TITLE_MAX_LENGTH};
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

/// The basic data of the classroom that is filled in by the user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomAddData {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl Validated for ClassroomAddData {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.name.chars().count() > CLASSROOM_TITLE_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "title",
                CLASSROOM_TITLE_MAX_LENGTH,
            ));
        }
        if self.description.chars().count() > CLASSROOM_DESCRIPTION_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "description",
                CLASSROOM_DESCRIPTION_MAX_LENGTH,
            ));
        }
        Ok(())
    }
}

/// The generated data that is created after the creation of the classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomGeneratedData {
    pub id: i32,
    pub join_code: String,
    pub created_at: DateTime<Utc>,
    pub owner_id: i32,
}

/// The data that is returned to the user after creation of the classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomAddReturnData {
    pub id: i32,
    pub join_code: String,
}

/// The data that is returned to the user when requesting all data.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomAllData{
    pub data: ClassroomAddData,
    pub gen_data: ClassroomGeneratedData,
}
