/// The basic data of the card, including the tree it belongs to.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardAddData {
    pub tree_id: i32,
    pub front: String,
    pub back: String,
}

/// The basic data required to update an existing card.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdateData {
    pub card_id: i32,
    pub new_front: String,
    pub new_back: String,
}
