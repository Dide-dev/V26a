/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::Validated;
use crate::errors::ApiError;
use crate::limits::{CARD_BACK_MAX_LENGTH, CARD_FRONT_MAX_LENGTH, CARD_LIBRARY_TITLE_MAX_LENGTH};
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

/// The basic data of the card, including the tree it belongs to.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardAddData {
    pub tree_id: i32,
    pub data: CardData,
}

/// The basic data required to update an existing card.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdateData {
    pub card_id: i32,
    pub data: CardData,
}

/// The basic data of the card
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub front: String,
    pub back: String,
}

impl Validated for CardData {
    /// Returns a [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.front.len() > CARD_FRONT_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "front",
                CARD_FRONT_MAX_LENGTH,
            ));
        }
        if self.back.len() > CARD_BACK_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded("back", CARD_BACK_MAX_LENGTH));
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
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.title.len() > CARD_LIBRARY_TITLE_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "title",
                CARD_LIBRARY_TITLE_MAX_LENGTH,
            ));
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
