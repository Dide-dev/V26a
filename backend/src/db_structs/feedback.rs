/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::{Validated, validate_email};
use crate::errors::ApiError;
use crate::limits::{FEEDBACK_MAX_LENGTH, FEEDBACK_MIN_LENGTH};
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

impl Validated for FeedbackData {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalEmailAddress`](ApiError::IllegalEmailAddress) when the email address does not have at least 1 '@'-symbol
    /// with at least 1 character in front of it and behind it.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.email != String::new() {
            validate_email(&self.email)?;
        }

        if self.feedback.chars().count() > FEEDBACK_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "feedback",
                FEEDBACK_MAX_LENGTH,
            ));
        }

        if self.feedback.chars().count() < FEEDBACK_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet("feedback", FEEDBACK_MIN_LENGTH));
        }

        Ok(())
    }
}
