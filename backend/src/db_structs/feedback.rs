/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackData {
    pub email: String,
    pub feedback: String,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackGetData {
    pub feedback_id: i32,
    pub email: String,
    pub feedback: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackListResponse {
    pub feedback_total: i32,
    pub feedback: Vec<FeedbackGetData>,
}
