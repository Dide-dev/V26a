/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::errors::ApiError;
use crate::limits::{
    CLASSROOM_DESCRIPTION_MAX_LENGTH, CLASSROOM_TITLE_MAX_LENGTH, CLASSROOM_TITLE_MIN_LENGTH,
    LESSON_DESCRIPTION_MAX_LENGTH, LESSON_TITLE_MAX_LENGTH, LESSON_TITLE_MIN_LENGTH,
};
use crate::structs::Validated;
use crate::structs::session::ClassroomClaims;
use crate::structs::trees::TreeMetadata;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

/// The basic data of the classroom that is filled in by the user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomAddData {
    /// The name of the to-be classroom.
    pub name: String,
    /// The description of the to-be classroom.
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

/// The data sent back to the user after creation of the classroom
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomGeneratedData {
    /// The id of the classroom.
    pub id: i32,
    /// The join-code of the classroom.
    pub join_code: String,
}

/// The data that is returned to the user when requesting all data.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomMetaData {
    /// The id of the classroom.
    pub id: i32,
    /// The name of the classroom.
    pub name: String,
    /// The description of the classroom.
    pub description: String,
    /// The timestamp of the creation of the classroom.
    pub created_at: DateTime<Utc>,
    /// The owner of the classroom's user id.
    pub owner_id: i32,
}

/// The data returned when requesting a list of classrooms.
#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomListResponse {
    /// The total amount of classrooms that are visible in the query.
    pub total_classrooms: i32,
    /// The collection of classroom metadata's returned to the user.
    pub classrooms: Vec<ClassroomMetaData>,
}

/// The data that is sent by the user to log in to a classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomUserLoginData {
    /// The join-code of the classroom.
    pub join_code: String,
    /// The user name or user number that the user wants to log in with.
    pub username: String,
}

/// The data that is sent to the user after successfully logging in to a classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum ClassroomLoginResponse {
    Success(ClassroomClaims),
    JoinCodeIncorrect { join_code: String },
    MissingUser { user_number: i32 },
}

/// The data to construct a teacher classroom dashboard sent to the user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomTeacherDashboardData {
    /// The metadata of the classroom. Does not contain the join-code.
    pub metadata: ClassroomMetaData,
    /// The collection of lessons in the classroom.
    pub lessons: Vec<LessonWithTrees>,
    /// The join-code of the classroom.
    pub join_code: String,
}

/// The data to construct a student classroom dashboard sent to the user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomStudentDashboardData {
    /// The metadata of the classroom. Does not contain the join-code.
    pub metadata: ClassroomMetaData,
    /// The collection of lessons in the classroom visible to students in the classroom.
    pub lessons: Vec<LessonWithTrees>,
}

/// The basic data of the lesson that is filled in by the user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LessonAddData {
    /// The name of the to-be lesson.
    pub name: String,
    /// The description of the to-be lesson.
    #[serde(default)]
    pub description: String,
    /// Whether the to-be lesson is published to students in the classroom.
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
        if self.name.chars().count() < LESSON_TITLE_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet(
                "title",
                LESSON_TITLE_MIN_LENGTH,
            ));
        }
        Ok(())
    }
}

/// All data on lessons, except the order number and classroom id.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LessonMetaData {
    /// The lesson id.
    pub id: i32,
    /// The position in the list of lessons.
    pub order_number: i32,
    /// The timestamp of the creation of the lesson.
    pub created_at: DateTime<Utc>,
    /// The name of the lesson.
    pub name: String,
    /// The description of the lesson.
    pub description: String,
    /// Whether the lesson is published to students in the classroom.
    pub published: bool,
}

/// The data of the lesson along with its trees.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LessonWithTrees {
    /// The metadata of the lesson.
    pub metadata: LessonMetaData,
    /// The collection of tree metadata's in the lesson.
    pub trees: Vec<TreeMetadata>,
}
