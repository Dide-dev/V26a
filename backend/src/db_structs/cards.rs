/// The basic data of the card, including the tree it belongs to.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardAddData {
    pub tree_id: i32,
    pub front: String,
    pub back: String,
}

/// The basic data required to update an existing card.
///
/// `new_front` and `new_back` may be [`None`] if they are not updated. If both are [`None`],
/// an [`ApiError::IncompleteRequest`](crate::errors::ApiError::IncompleteRequest) will be returned.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdateData {
    pub card_id: i32,
    pub new_front: Option<String>,
    pub new_back: Option<String>,
}
