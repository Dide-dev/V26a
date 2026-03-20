#![cfg(test)]

use crate::db_structs::trees::{TreeData, TreeMetadata, TreeWithCards};
use sqlx::PgPool;
use std::collections::BTreeMap;

#[sqlx::test]
async fn test_new_empty_tree(db: PgPool) {
    let (server, _user_id) = crate::tests::api_v1::setup_login(db).await;

    let tree_data = TreeData {
        title: "Test tree".to_string(),
        description: None,
        book_name: None,
        book_edition: None,
        book_isbn: None,
        book_chapter: None,
        book_exercise: None,
        assignment_text: None,
        published: false,
    };
    let response = server.post("/trees/add").json(&tree_data).await;
    response.assert_status_ok();
    let id: i32 = response.json();

    let response = server.get(&format!("/trees/get-with-cards/{id}")).await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();
    assert_eq!(full_tree.data, tree_data);
    assert_eq!(full_tree.gen_data.id, id);
    assert_eq!(full_tree.cards, BTreeMap::new());
}

#[sqlx::test]
async fn test_retrieving_trees_by_author(db: PgPool) {
    let (server, user_id) = crate::tests::api_v1::setup_login(db).await;

    let response = server.get(&format!("/trees/get-by-author/{user_id}")).await;
    response.assert_status_ok();
    let vec: Vec<TreeMetadata> = response.json();
    let empty: Vec<TreeMetadata> = Vec::new();
    assert_eq!(empty, vec);

    let tree_data = TreeData {
        title: "Test tree".to_string(),
        description: None,
        book_name: None,
        book_edition: None,
        book_isbn: None,
        book_chapter: None,
        book_exercise: None,
        assignment_text: None,
        published: false,
    };
    let response = server.post("/trees/add").json(&tree_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let response = server.get(&format!("/trees/get-by-author/{user_id}")).await;
    response.assert_status_ok();
    let vec: Vec<TreeMetadata> = response.json();
    let tree_metadata: &TreeMetadata = vec.first().unwrap();
    assert_eq!(tree_id, tree_metadata.gen_data.id);
    assert_eq!(user_id, tree_metadata.gen_data.owner_id);
}
