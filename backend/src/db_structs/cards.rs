/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::Validated;
use crate::errors::ApiError;
use crate::limits::{
    CARD_BACK_MAX_LENGTH, CARD_BACK_MIN_LENGTH, CARD_FRONT_MAX_LENGTH, CARD_LIBRARY_TAG_MAX_LENGTH,
    CARD_LIBRARY_TAG_MIN_LENGTH, CARD_LIBRARY_TITLE_MAX_LENGTH, CARD_LIBRARY_TITLE_MIN_LENGTH,
};
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

/// The basic data of the card
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub front: String,
    pub back: String,
}

impl Validated for CardData {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.front.chars().count() > CARD_FRONT_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "front",
                CARD_FRONT_MAX_LENGTH,
            ));
        }
        if self.back.chars().count() > CARD_BACK_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded("back", CARD_BACK_MAX_LENGTH));
        }

        if self.back.chars().count() < CARD_BACK_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet("back", CARD_BACK_MIN_LENGTH));
        }

        Ok(())
    }
}

/// Data for the API-call to add a card to the library.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LibraryCardAddData {
    pub title: String,
    pub tags: Vec<String>,
}
impl Validated for LibraryCardAddData {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalTag`](ApiError::IllegalTag) when a tag contains an illegal character.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.title.chars().count() > CARD_LIBRARY_TITLE_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "title",
                CARD_LIBRARY_TITLE_MAX_LENGTH,
            ));
        }

        if self.title.chars().count() < CARD_LIBRARY_TITLE_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet(
                "title",
                CARD_LIBRARY_TITLE_MIN_LENGTH,
            ));
        }

        for tag in &self.tags {
            if tag.chars().count() > CARD_LIBRARY_TAG_MAX_LENGTH {
                return Err(ApiError::InputLengthExceeded(
                    "tag",
                    CARD_LIBRARY_TAG_MAX_LENGTH,
                ));
            }
            if tag.chars().count() < CARD_LIBRARY_TAG_MIN_LENGTH {
                return Err(ApiError::InputLengthNotMet(
                    "tag",
                    CARD_LIBRARY_TAG_MIN_LENGTH,
                ));
            }
            for char in tag.chars() {
                let legal =
                    char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.';
                if !legal {
                    return Err(ApiError::IllegalTag);
                }
            }
        }

        Ok(())
    }
}

/// Data that is returned when cards are requested from the card library.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LibraryCardGetData {
    pub card_id: i32,
    pub title: String,
    pub owner_id: i32,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub front: String,
    pub back: String,
}

/// The filters that can be applied when requesting cards from the card library.
#[derive(serde::Deserialize, ToSchema, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CardFilters {
    pub owner_id: Option<i32>,

    #[serde(default)]
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}
