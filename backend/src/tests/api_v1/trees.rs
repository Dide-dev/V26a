/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::cards::{CardAddData, CardGetData};
use crate::db_structs::trees::{
    Node, RootNode, TreeAddData, TreeData, TreeListResponse, TreeMetadata, TreeRemoveCardData,
    TreeStructure, TreeUpdateData, TreeUpdateStructureData, TreeWithCards,
};
use crate::errors::ApiError;
use crate::tests::api_v1::setup_login;
use sqlx::PgPool;
use std::collections::BTreeMap;

#[sqlx::test]
async fn test_new_empty_tree(db: PgPool) {
    let server = setup_login(db).await;

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
    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure,
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let id: i32 = response.json();

    let response = server.get(&format!("/trees/get-with-cards/{id}")).await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();
    assert_eq!(full_tree.data, tree_add_data.tree_data);
    assert_eq!(full_tree.gen_data.id, id);
    assert_eq!(full_tree.cards, BTreeMap::new());
}

#[sqlx::test]
async fn test_new_empty_tree_with_data(db: PgPool) {
    let server = setup_login(db).await;

    let tree_data = TreeData {
        title: "Test tree".to_string(),
        description: Some("This is used for testing".to_string()),
        book_name: Some("Introduction to mathematics".to_string()),
        book_edition: Some("4b".to_string()),
        book_isbn: Some("9780136019701".to_string()),
        book_chapter: Some("5".to_string()),
        book_exercise: Some("27a".to_string()),
        assignment_text: Some("Try to solve the following equation".to_string()),
        published: true,
    };
    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure,
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let id: i32 = response.json();

    let response = server.get(&format!("/trees/get-with-cards/{id}")).await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();
    assert_eq!(full_tree.data, tree_add_data.tree_data);
    assert_eq!(full_tree.gen_data.id, id);
    assert_eq!(full_tree.cards, BTreeMap::new());
}

#[sqlx::test]
async fn test_retrieving_owned_trees(db: PgPool) {
    let server = setup_login(db).await;

    let response = server.get("/trees/get-mine-all").await;
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
    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure,
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let response = server.get("/trees/get-mine-all").await;
    response.assert_status_ok();
    let vec: Vec<TreeMetadata> = response.json();
    let tree_metadata: &TreeMetadata = vec.first().unwrap();
    assert_eq!(tree_id, tree_metadata.gen_data.id);
    assert_eq!(tree_add_data.tree_data, tree_metadata.data);
}

#[sqlx::test]
async fn test_remove_card(db: PgPool) {
    let server = setup_login(db).await;

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
    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure,
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let card_add_data = CardAddData {
        tree_id,
        front: String::new(),
        back: "heuristic".to_string(),
    };
    let response = server.post("/cards/add").json(&card_add_data).await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();

    let mut expected_btree = BTreeMap::new();
    expected_btree.insert(
        card_id,
        CardGetData {
            front: card_add_data.front,
            back: card_add_data.back,
        },
    );

    assert_eq!(full_tree.cards, expected_btree);

    let tree_structure_with_card = TreeStructure {
        root_nodes: vec![RootNode {
            label: "Orientation".to_owned(),
            children: vec![Node {
                card_id,
                children: vec![],
            }],
        }],
    };

    let tree_update_data = TreeUpdateStructureData {
        id: tree_id,
        new_structure: tree_structure_with_card.clone(),
    };

    let response = server
        .post("/trees/update-structure")
        .json(&tree_update_data)
        .await;
    response.assert_status_ok();

    // Attempt to remove the card from the database without removing it from the tree
    // and assert an appropriate error is returned.
    let invalid_card_removal_data = TreeRemoveCardData {
        id: tree_id,
        card_id,
        new_structure: tree_structure_with_card,
    };

    let response = server
        .post("/trees/remove-card")
        .json(&invalid_card_removal_data)
        .await;
    response.assert_status_bad_request();
    assert_eq!(response.text(), ApiError::MismatchedStructure.to_string());

    let tree_structure_without_card = TreeStructure {
        root_nodes: vec![RootNode {
            label: "Orientation".to_owned(),
            children: vec![],
        }],
    };

    let valid_card_removal_data = TreeRemoveCardData {
        id: tree_id,
        card_id,
        new_structure: tree_structure_without_card,
    };

    let response = server
        .post("/trees/remove-card")
        .json(&valid_card_removal_data)
        .await;
    response.assert_status_ok();

    // Run same removal for a second time and assert an appropriate error is returned.
    let response = server
        .post("/trees/remove-card")
        .json(&valid_card_removal_data)
        .await;
    response.assert_status_bad_request();
    assert_eq!(response.text(), ApiError::MissingCard(card_id).to_string());

    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();

    // The structure should be updated to match the `new_structure` field.
    assert_eq!(full_tree.structure, valid_card_removal_data.new_structure);
    // The card should be removed from the tree.
    assert_eq!(full_tree.cards, BTreeMap::new());
}

#[sqlx::test]
async fn test_filter_tree(db: PgPool) {
    let server = setup_login(db).await;

    let trees_data = vec![
        TreeData {
            title: "Test tree".to_string(),
            description: None,
            book_name: None,
            book_edition: None,
            book_isbn: None,
            book_chapter: None,
            book_exercise: None,
            assignment_text: None,
            published: true,
        },
        TreeData {
            title: "Test tree 2".to_string(),
            description: None,
            book_name: Some("vwo 4".to_string()),
            book_edition: None,
            book_isbn: None,
            book_chapter: None,
            book_exercise: None,
            assignment_text: None,
            published: true,
        },
    ];
    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };

    // Add in reverse order because trees are ordered with newest on top
    server
        .post("/trees/add")
        .json(&TreeAddData {
            tree_data: trees_data[1].clone(),
            tree_structure: tree_structure.clone(),
        })
        .await
        .assert_status_ok();

    server
        .post("/trees/add")
        .json(&TreeAddData {
            tree_data: trees_data[0].clone(),
            tree_structure,
        })
        .await
        .assert_status_ok();

    let first_tree = vec![trees_data[0].clone()];
    let second_tree = vec![trees_data[1].clone()];

    let response = server.get("/trees/get-list?page=1&size=100").await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    assert_eq!(requested_trees.trees, trees_data);

    let response = server.get("/trees/get-list?page=1&size=1").await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    assert_eq!(requested_trees.trees, first_tree);

    let response = server.get("/trees/get-list?page=2&size=1").await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    assert_eq!(requested_trees.trees, second_tree);

    // Filter on book with space
    let response = server.get("/trees/get-list?page=1&size=2&book=vwo+4").await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    assert_eq!(requested_trees.trees, second_tree);
}

#[sqlx::test]
async fn test_update_and_get_tree(db: PgPool) {
    let server = setup_login(db).await;

    let tree_data = TreeData {
        title: "Test tree 2".to_string(),
        description: None,
        book_name: None,
        book_edition: None,
        book_isbn: None,
        book_chapter: None,
        book_exercise: None,
        assignment_text: None,
        published: false,
    };
    let tree_add_data = TreeAddData {
        tree_data: tree_data.clone(),
        tree_structure: TreeStructure { root_nodes: vec![] },
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let card_get_data = CardGetData {
        front: String::new(),
        back: "heuristic".to_string(),
    };
    let card_add_data = CardAddData {
        tree_id,
        front: card_get_data.front.clone(),
        back: card_get_data.back.clone(),
    };
    let response = server.post("/cards/add").json(&card_add_data).await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    let structure_valid = TreeStructure {
        root_nodes: vec![RootNode {
            label: "bla".to_string(),
            children: vec![Node {
                card_id,
                children: vec![],
            }],
        }],
    };

    let mut structure_duplicate_card = structure_valid.clone();
    let node = structure_duplicate_card.root_nodes[0].children[0].clone();
    structure_duplicate_card.root_nodes[0].children.push(node);

    let mut structure_invalid_card = structure_valid.clone();
    structure_invalid_card.root_nodes[0].children[0].card_id = -999;

    // First test happy path
    let response = server
        .post("/trees/update-structure")
        .json(&TreeUpdateStructureData {
            id: tree_id,
            new_structure: structure_valid.clone(),
        })
        .await;
    response.assert_status_ok();

    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_ok();
    let tree_with_cards_valid: TreeWithCards = response.json();
    assert_eq!(tree_with_cards_valid.structure, structure_valid);
    assert_eq!(tree_with_cards_valid.data, tree_data);
    assert!(
        tree_with_cards_valid
            .cards
            .iter()
            .eq(std::iter::once((&card_id, &card_get_data)))
    );

    // Try updating with duplicate card, ensure that the structure did not change
    let response = server
        .post("/trees/update-structure")
        .json(&TreeUpdateStructureData {
            id: tree_id,
            new_structure: structure_duplicate_card.clone(),
        })
        .await;
    response.assert_status_not_ok();
    response.assert_text(ApiError::MismatchedStructure.to_string());

    server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await
        .assert_json(&tree_with_cards_valid);

    // Try updating with invalid card, ensure that the structure did not change
    let response = server
        .post("/trees/update-structure")
        .json(&TreeUpdateStructureData {
            id: tree_id,
            new_structure: structure_invalid_card.clone(),
        })
        .await;
    response.assert_status_not_ok();
    response.assert_text(ApiError::MismatchedStructure.to_string());

    server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await
        .assert_json(&tree_with_cards_valid);
}

#[sqlx::test]
async fn test_update_metadata(db: PgPool) {
    let server = setup_login(db).await;

    // Create a tree
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
    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure,
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    // Update the metadata`
    let new_tree_data = TreeData {
        title: "Updated tree".to_string(),
        description: Some("Updated description".to_string()),
        book_name: Some("Updated book".to_string()),
        book_edition: Some("2nd".to_string()),
        book_isbn: Some("9780136019701".to_string()),
        book_chapter: Some("3".to_string()),
        book_exercise: Some("5b".to_string()),
        assignment_text: Some("Updated assignment".to_string()),
        published: true,
    };
    let tree_update_data: TreeUpdateData = TreeUpdateData {
        id: tree_id,
        new_data: new_tree_data.clone(),
    };
    let response = server
        .post("/trees/update-data")
        .json(&tree_update_data)
        .await;
    response.assert_status_ok();

    // Verify the metadata was updated
    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_ok();
    let full_tree: TreeWithCards = response.json();
    assert_eq!(full_tree.data, new_tree_data);
}
