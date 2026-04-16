/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use utoipa::ToSchema;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackData {
    pub email: Option<String>,
    pub positive: Option<String>,
    pub negative: Option<String>,
    pub additional: Option<String>,
    pub grade: Option<i32>,
}
