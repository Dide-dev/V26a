/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::cards::{CardData, LibraryCardAddData, LibraryCardGetData};
use crate::db_structs::trees::{TreeAddData, TreeData, TreeStructure};
use crate::tests::api_v1::setup_login;
use sqlx::PgPool;

#[sqlx::test]
async fn test_library_card_add(db: PgPool) {
    let server = setup_login(db).await;

    let library_card_add_data = LibraryCardAddData {
        title: "title".to_owned(),
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let missing_card_id = 1;

    let response = server
        .post(&format!("/library/cards/{missing_card_id}"))
        .json(&library_card_add_data)
        .await;
    response.assert_status_bad_request();

    let tree_data = TreeData::default();
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure: TreeStructure { root_nodes: vec![] },
    };
    let response = server.post("/trees").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let card_data = CardData {
        front: "front".to_owned(),
        back: "back".to_owned(),
    };

    let response = server
        .post(&format!("/trees/{tree_id}/cards"))
        .json(&card_data)
        .await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    let library_card_add_data = LibraryCardAddData {
        title: "title".to_owned(),
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let response = server
        .post(&format!("/library/cards/{card_id}"))
        .json(&library_card_add_data)
        .await;
    response.assert_status_ok();

    let response = server.get("/library/cards?tags=tag1&tags=tag2").await;
    response.assert_status_ok();
    let library_card_get_data: Vec<LibraryCardGetData> = response.json();
    assert_eq!(library_card_get_data.len(), 1);

    let retrieved_library_card = &library_card_get_data[0];

    assert_eq!(library_card_add_data.title, retrieved_library_card.title);
    assert_eq!(library_card_add_data.tags, retrieved_library_card.tags);
    assert_eq!(card_data.front, retrieved_library_card.front);
    assert_eq!(card_data.back, retrieved_library_card.back);
}
