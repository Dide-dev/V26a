/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::structs::cards::CardData;
use crate::structs::trees::{TreeAddData, TreeData, TreeStructure, TreeWithCards};
use crate::tests::api_v1::setup_login;
use sqlx::PgPool;
use std::collections::BTreeMap;

#[sqlx::test]
async fn test_new_card(db: PgPool) {
    let server = setup_login(db).await;

    let missing_tree_id = 0;
    let card_data = CardData {
        front: "front".to_owned(),
        back: "back".to_owned(),
    };

    let response = server
        .post(&format!("/trees/{missing_tree_id}/cards"))
        .json(&card_data)
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

    let response = server
        .post(&format!("/trees/{tree_id}/cards"))
        .json(&card_data)
        .await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    let mut expected_btree = BTreeMap::new();
    expected_btree.insert(
        card_id,
        CardData {
            front: card_data.front,
            back: card_data.back,
        },
    );

    let response = server.get(&format!("/trees/{tree_id}")).await;
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

    let card_update_data = CardData {
        front: "FRONT".to_owned(),
        back: "BACK".to_owned(),
    };

    let response = server
        .put(&format!("/cards/{card_id}"))
        .json(&card_update_data)
        .await;
    response.assert_status_ok();

    let mut expected_btree = BTreeMap::new();
    expected_btree.insert(
        card_id,
        CardData {
            front: card_update_data.front,
            back: card_update_data.back,
        },
    );

    let response = server.get(&format!("/trees/{tree_id}")).await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();
    assert_eq!(full_tree.cards, expected_btree);
}
