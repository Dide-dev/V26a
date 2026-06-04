/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::Validated;
use crate::errors::ApiError;
use crate::limits::{
    CLASSROOM_DESCRIPTION_MAX_LENGTH, CLASSROOM_TITLE_MAX_LENGTH, CLASSROOM_TITLE_MIN_LENGTH, LESSON_DESCRIPTION_MAX_LENGTH,
    LESSON_TITLE_MAX_LENGTH,
};
use sqlx::FromRow;
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
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
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

        if self.name.chars().count() < CLASSROOM_TITLE_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet(
                "title",
                CLASSROOM_TITLE_MIN_LENGTH,
            ));
        }

        Ok(())
    }
}

/// The data that is returned to the user after creation of the classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomGeneratedData {
    pub id: i32,
    pub join_code: String,
}

/// The basic data of the lesson that is filled in by the user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LessonAddData {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub published: bool,
}

impl Validated for LessonAddData {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.name.len() > LESSON_TITLE_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "name",
                LESSON_TITLE_MAX_LENGTH,
            ));
        }
        if self.description.len() > LESSON_DESCRIPTION_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "description",
                LESSON_DESCRIPTION_MAX_LENGTH,
            ));
        }
        Ok(())
    }
}
