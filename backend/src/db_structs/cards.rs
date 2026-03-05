/// The basic data of the card, including the tree it belongs to.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub tree_id: i32,
    pub card_id: Option<i32>,
    pub front: String,
    pub back: String,
}
