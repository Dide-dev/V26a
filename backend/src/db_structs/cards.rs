/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

/// The basic data of the card, including the tree it belongs to.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardAddData {
    pub tree_id: i32,
    pub front: String,
    pub back: String,
}

/// The basic data required to update an existing card.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdateData {
    pub card_id: i32,
    pub new_front: String,
    pub new_back: String,
}

/// The basic data of the card
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CardGetData {
    pub front: String,
    pub back: String,
}

/// Data for the API-call to add a card to the library.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LibraryCardAddData {
    pub title: String,
    pub tags: Vec<String>,
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
