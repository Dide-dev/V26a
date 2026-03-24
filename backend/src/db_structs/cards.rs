/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

/// The basic data of the card, including the tree it belongs to.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardAddData {
    pub tree_id: i32,
    pub front: String,
    pub back: String,
}

/// The basic data required to update an existing card.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdateData {
    pub card_id: i32,
    pub new_front: String,
    pub new_back: String,
}

/// The basic data of the card
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardGetData {
    pub front: String,
    pub back: String,
}
