/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::cards::{CardAddData, CardGetData, CardUpdateData, LibraryCardAddData};
use crate::db_structs::trees::{TreeAddData, TreeData, TreeStructure, TreeWithCards};
use crate::tests::api_v1::setup_login;
use sqlx::PgPool;
use std::collections::BTreeMap;

#[sqlx::test]
async fn test_new_card(db: PgPool) {
    let server = setup_login(db).await;

    let card_add_data = CardAddData {
        tree_id: 0,
        front: "front".to_owned(),
        back: "back".to_owned(),
    };

    let response = server.post("/cards/add").json(&card_add_data).await;
    response.assert_status_bad_request();

    let tree_data = TreeData::default();
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure: TreeStructure { root_nodes: vec![] },
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let card_add_data = CardAddData {
        tree_id,
        front: "front".to_owned(),
        back: "back".to_owned(),
    };

    let response = server.post("/cards/add").json(&card_add_data).await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    let mut expected_btree = BTreeMap::new();
    expected_btree.insert(
        card_id,
        CardGetData {
            front: card_add_data.front,
            back: card_add_data.back,
        },
    );

    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();
    assert_eq!(full_tree.cards, expected_btree);
}

#[sqlx::test]
async fn test_update_existing_card(db: PgPool) {
    let server = setup_login(db).await;

    let tree_data = TreeData::default();
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure: TreeStructure { root_nodes: vec![] },
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let card_add_data = CardAddData {
        tree_id,
        front: "front".to_owned(),
        back: "back".to_owned(),
    };

    let response = server.post("/cards/add").json(&card_add_data).await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    let card_update_data = CardUpdateData {
        card_id,
        new_front: "FRONT".to_owned(),
        new_back: "BACK".to_owned(),
    };

    let response = server.post("/cards/update").json(&card_update_data).await;
    response.assert_status_ok();

    let mut expected_btree = BTreeMap::new();
    expected_btree.insert(
        card_id,
        CardGetData {
            front: card_update_data.new_front,
            back: card_update_data.new_back,
        },
    );

    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();
    assert_eq!(full_tree.cards, expected_btree);
}

#[sqlx::test]
async fn test_library_card_add(db: PgPool) {
    let server = setup_login(db).await;

    let library_card_add_data = LibraryCardAddData {
        title: "title".to_owned(),
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let response = server
        .post(&format!("/cards/{}/add-to-library", 1))
        .json(&library_card_add_data)
        .await;
    response.assert_status_bad_request();

    let tree_data = TreeData::default();
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure: TreeStructure { root_nodes: vec![] },
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let card_add_data = CardAddData {
        tree_id,
        front: "front".to_owned(),
        back: "back".to_owned(),
    };

    let response = server.post("/cards/add").json(&card_add_data).await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    let library_card_add_data = LibraryCardAddData {
        title: "title".to_owned(),
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let response = server
        .post(&format!("/cards/{}/add-to-library", card_id))
        .json(&library_card_add_data)
        .await;
    response.assert_status_ok();

    // TODO: Cannot test retrieving a card from cards library yet as that's not implemented yet.
    // It's out of scope for 189.
}
