/// The basic data of the card, including the tree it belongs to.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub tree_id: i32,
    pub front: String,
    pub back: String,
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardGeneratedData {
    pub id: i32,
    pub tree_id: i32,
    pub created_at: DateTime<Utc>,
    pub last_edited: DateTime<Utc>,
}