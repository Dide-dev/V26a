/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
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

/// The data that is returned to the user after creation of the classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomGeneratedData {
    pub id: i32,
    pub join_code: String,
}

/// The data that is sent by the user to log in to a classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomUserLoginData {
    pub join_code: String,
    pub username: String,
}

/// The data that is sent to the user after successfully logging in to a classroom.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomLoginResponse {
    pub id: i32,
    pub num: i32,
    pub name: String,
}
