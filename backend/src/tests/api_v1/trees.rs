/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::trees::verify_tree_permissions;
use crate::db::trees::is_co_author_of_tree;
use crate::db_structs::cards::{CardAddData, CardData, CardUpdateData};
use crate::db_structs::session::Claims;
use crate::db_structs::trees::{
    Language, Node, RootNode, Subject, TreeAddData, TreeData, TreeListResponse, TreeMetadata,
    TreeRemoveCardData, TreeStructure, TreeUpdateData, TreeUpdateStructureData, TreeWithCards,
};
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::get_connection_unwrap;
use crate::tests::api_v1::{
    create_tree, login, register, register_admin, setup, setup_login, setup_with_options,
};
use axum_test::TestServer;
use sqlx::PgPool;
use std::collections::{BTreeMap, HashMap};

async fn add_multiple_trees(test_server: &TestServer, trees_data: &[TreeData]) {
    let basic_tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };

    // Add in reverse order because trees are ordered with newest on top
    for t in trees_data {
        test_server
            .post("/trees/add")
            .json(&TreeAddData {
                tree_data: t.clone(),
                tree_structure: basic_tree_structure.clone(),
            })
            .await
            .assert_status_ok();
    }
}

#[sqlx::test]
async fn test_new_empty_tree(db: PgPool) {
    let server = setup_login(db).await;

    let tree_data = TreeData::default();
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
    assert_eq!(full_tree.metadata.data, tree_add_data.tree_data);
    assert_eq!(full_tree.metadata.gen_data.id, id);
    assert_eq!(full_tree.cards, BTreeMap::new());
}

#[sqlx::test]
async fn test_new_invalid_tree(db: PgPool) {
    let server = setup_login(db).await;

    // Create tree with invalid language
    let tree_data = TreeData {
        language_code: "NO".to_string(),
        ..Default::default()
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
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingLanguage("NO".to_string()).to_string());

    // Create tree with invalid subject
    let tree_data = TreeData {
        subject_id: Some(420),
        ..Default::default()
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
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingSubject(420).to_string());
}

#[sqlx::test]
async fn test_new_empty_tree_with_data(db: PgPool) {
    let server = setup_login(db).await;

    let tree_data = TreeData {
        title: "Test tree".to_string(),
        description: "This is used for testing".to_string(),
        language_code: "en".to_string(),
        tags: vec!["geometry".to_string(), "difficult".to_string()],
        subject_id: Some(2),
        book_name: "Introduction to mathematics".to_string(),
        book_author: "Leonhard Euler".to_string(),
        book_edition: "4b".to_string(),
        book_isbn: Some("9780136019701".to_string()),
        book_chapter: "5".to_string(),
        book_exercise: "27a".to_string(),
        assignment_text: "Try to solve the following equation".to_string(),
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
    assert_eq!(full_tree.metadata.data, tree_add_data.tree_data);
    assert_eq!(full_tree.metadata.gen_data.id, id);
    assert_eq!(full_tree.cards, BTreeMap::new());
}

#[sqlx::test]
async fn test_unpublished_trees(db: PgPool) {
    let conn = get_connection_unwrap!(db);
    let mut server = setup_login(db).await;

    let tree_id: i32 = create_tree(&server).await;

    let endpoint = format!("/trees/get-with-cards/{tree_id}");
    server.get(&endpoint).await.assert_status_ok();

    server.clear_cookies();
    let response = server.get(&endpoint).await;
    response.assert_status_unauthorized();
    assert_eq!(response.text(), ApiError::MissingAuthentication.to_string());

    let second_user_email = "second_user@localhost";
    register(&server, second_user_email.to_owned()).await;
    login(&server, second_user_email.to_owned()).await;
    let response = server.get(&endpoint).await;
    response.assert_status_forbidden();
    assert_eq!(response.text(), ApiError::NoPermission.to_string());

    let admin_user_email = "admin@localhost";
    register_admin(&server, conn, admin_user_email.to_owned()).await;
    login(&server, admin_user_email.to_owned()).await;
    let response = server.get(&endpoint).await;
    response.assert_status_ok();
}

#[sqlx::test]
async fn test_deleting_trees_works(db: PgPool) {
    let server = setup_login(db).await;

    let tree_id: i32 = create_tree(&server).await;

    // We also add a card, to ensure that it does not prevent the tree from being deleted.
    let card_add_data = CardAddData {
        tree_id,
        data: CardData {
            front: "abc".to_owned(),
            back: "def".to_owned(),
        },
    };
    let response = server.post("/cards/add").json(&card_add_data).await;
    response.assert_status_ok();
    let card_id: i32 = response.json();

    server
        .delete(&format!("/trees/remove/{tree_id}"))
        .await
        .assert_status_ok();

    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingTree(tree_id).to_string());

    // Just in case check that the card also doesn't exist anymore.
    // This should never happen as the database has a foreign key for this.
    let response = server
        .post("/cards/update")
        .json(&CardUpdateData {
            card_id,
            data: CardData {
                front: String::new(),
                back: String::new(),
            },
        })
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingCard(card_id).to_string());
}

#[sqlx::test]
async fn test_deleting_trees_denied(db: PgPool) {
    let mut server = setup_login(db).await;

    let tree_add_data = TreeAddData {
        tree_data: TreeData {
            published: true,
            ..Default::default()
        },
        tree_structure: TreeStructure { root_nodes: vec![] },
    };
    let tree_id: i32 = server.post("/trees/add").json(&tree_add_data).await.json();

    server.clear_cookies();
    server
        .delete(&format!("/trees/remove/{tree_id}"))
        .await
        .assert_status_unauthorized();

    let response = server
        .get(&format!("/trees/get-with-cards/{tree_id}"))
        .await;
    response.assert_status_ok();
}

#[sqlx::test]
async fn test_retrieving_owned_trees(db: PgPool) {
    let server = setup_login(db).await;
    let response = server
        .get("/trees/get-list?page=1&size=100&showOwnPrivate=true")
        .await;
    response.assert_status_ok();
    let vec: TreeListResponse = response.json();
    let empty: TreeListResponse = TreeListResponse {
        total_trees: 0,
        trees: Vec::new(),
    };
    assert_eq!(empty, vec);

    let tree_data = TreeData::default();
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

    let response = server
        .get("/trees/get-list?page=1&size=100&showOwnPrivate=true")
        .await;
    response.assert_status_ok();
    let vec: TreeListResponse = response.json();
    let tree_metadata: &TreeMetadata = vec.trees.first().unwrap();
    assert_eq!(tree_id, tree_metadata.gen_data.id);
    assert_eq!(tree_add_data.tree_data, tree_metadata.data);
}

#[sqlx::test]
async fn test_remove_card(db: PgPool) {
    let server = setup_login(db).await;

    let tree_data = TreeData::default();
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
        data: CardData {
            front: String::new(),
            back: "heuristic".to_string(),
        },
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
        CardData {
            front: card_add_data.data.front,
            back: card_add_data.data.back,
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
        tree_id,
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
        tree_id,
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
async fn test_filter_tree_list(db: PgPool) {
    let server = setup(db);
    let emails = ["test@one".to_owned(), "test@two".to_owned()];
    // Only take the user_id's.
    let account_ids = [
        register(&server, emails[0].clone()).await.0,
        register(&server, emails[1].clone()).await.0,
    ];
    login(&server, emails[0].clone()).await;

    let trees_data = vec![
        TreeData {
            published: true,
            ..Default::default()
        },
        TreeData {
            book_isbn: Some("9780061120084".to_string()),
            published: true,
            ..Default::default()
        },
        TreeData {
            published: false,
            ..Default::default()
        },
    ];

    add_multiple_trees(&server, &trees_data).await;

    // A search with an invalid isbn
    let response = server
        .get("/trees/get-list?page=1&size=1&book=not-an-isbn")
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::InvalidIsbn.to_string());

    // A search that yields no results
    let response = server
        .get("/trees/get-list?page=1&size=1&book=978-0-306-40615-7")
        .await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    assert_eq!(requested_trees.trees, Vec::new());

    // All trees (including private)
    let response = server
        .get("/trees/get-list?page=1&size=100&showOwnPrivate=true")
        .await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    // Reverse because the received trees are in reverse order (newest first).
    let rev_tree_data: Vec<TreeData> = trees_data.iter().rev().cloned().collect();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(requested_trees_data, rev_tree_data);

    // Newest public tree
    let response = server.get("/trees/get-list?page=1&size=1").await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(requested_trees_data, vec![trees_data[1].clone()]);

    // Second newest public tree
    let response = server.get("/trees/get-list?page=2&size=1").await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(requested_trees_data, vec![trees_data[0].clone()]);

    login(&server, emails[1].clone()).await;
    let other_user_trees_data = vec![
        TreeData {
            book_isbn: Some("9780306406157".to_string()),
            published: true,
            ..Default::default()
        },
        TreeData {
            published: false,
            ..Default::default()
        },
    ];

    add_multiple_trees(&server, &other_user_trees_data).await;

    // All public trees
    let response = server.get("/trees/get-list?page=1&size=100").await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(
        requested_trees_data,
        vec![
            other_user_trees_data[0].clone(),
            trees_data[1].clone(),
            trees_data[0].clone()
        ]
    );

    // Multiple books
    let response = server
        .get("/trees/get-list?page=1&size=100&book=978-0-061-12008-4&book=978-0-306-40615-7")
        .await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(
        requested_trees_data,
        vec![other_user_trees_data[0].clone(), trees_data[1].clone(),]
    );

    // Multiple books, incorrectly formatted
    let response = server
        .get("/trees/get-list?page=1&size=100&book=978-0-061-12008-4,978-0-306-40615-7")
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::InvalidIsbn.to_string());

    // All public trees from user 0
    let response = server
        .get(
            format!(
                "/trees/get-list?page=1&size=100&ownerId={}&showOwnPrivate=true",
                account_ids[0]
            )
            .as_str(),
        )
        .await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(
        requested_trees_data,
        vec![trees_data[1].clone(), trees_data[0].clone()]
    );

    login(&server, emails[0].clone()).await;

    // All trees from user 0 using filter
    let response = server
        .get(
            format!(
                "/trees/get-list?page=1&size=100&ownerId={}&showOwnPrivate=true",
                account_ids[0]
            )
            .as_str(),
        )
        .await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(requested_trees_data, rev_tree_data);

    // All public trees from user 1
    let response = server
        .get(
            format!(
                "/trees/get-list?page=1&size=100&ownerId={}&showOwnPrivate=true",
                account_ids[1]
            )
            .as_str(),
        )
        .await;
    response.assert_status_ok();
    let requested_trees: TreeListResponse = response.json();
    let requested_trees_data = requested_trees
        .trees
        .into_iter()
        .map(|t| t.data)
        .collect::<Vec<_>>();
    assert_eq!(requested_trees_data, vec![other_user_trees_data[0].clone()]);
}

#[sqlx::test]
async fn test_update_and_get_tree(db: PgPool) {
    let server = setup_login(db).await;

    let tree_data = TreeData::default();
    let tree_add_data = TreeAddData {
        tree_data: tree_data.clone(),
        tree_structure: TreeStructure { root_nodes: vec![] },
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let tree_id: i32 = response.json();

    let card_get_data = CardData {
        front: String::new(),
        back: "heuristic".to_string(),
    };
    let card_add_data = CardAddData {
        tree_id,
        data: CardData {
            front: card_get_data.front.clone(),
            back: card_get_data.back.clone(),
        },
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
    assert_eq!(tree_with_cards_valid.metadata.data, tree_data);
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

    let tree_id: i32 = create_tree(&server).await;

    // Update the metadata
    let new_tree_data = TreeData {
        title: "Updated tree".to_string(),
        language_code: "en".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        subject_id: Some(1),
        description: "Updated description".to_string(),
        book_name: "Updated book".to_string(),
        book_author: "Updated author".to_string(),
        book_edition: "2nd".to_string(),
        book_isbn: Some("9780136019701".to_string()),
        book_chapter: "3".to_string(),
        book_exercise: "5b".to_string(),
        assignment_text: "Updated assignment".to_string(),
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
    assert_eq!(full_tree.metadata.data, new_tree_data);
}

#[sqlx::test]
async fn test_get_languages(db: PgPool) {
    let server = setup(db);

    let response = server.get("/trees/get-all-languages").await;
    response.assert_status_ok();

    let languages: Vec<Language> = response.json();
    assert_ne!(languages.len(), 0);
}

#[sqlx::test]
async fn test_get_subjects(db: PgPool) {
    let server = setup(db);

    let response = server.get("/trees/get-all-subjects").await;
    response.assert_status_ok();

    let subjects: HashMap<String, Vec<Subject>> = response.json();
    assert_ne!(subjects.len(), 0);
}

#[sqlx::test]
async fn test_duplicate_tree(db: PgPool) {
    let server = setup(db);

    // Create two accounts and log in as one of them.
    register(&server, "1@gmail.com".to_owned()).await;
    let owner_id2 = register(&server, "2@gmail.com".to_owned()).await.0;
    login(&server, "1@gmail.com".to_owned()).await;

    let tree_data = TreeData {
        title: "test".to_owned(),
        published: true,
        ..Default::default()
    };

    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };
    let tree_structure_add_data = tree_structure.clone();
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure: tree_structure_add_data,
    };
    let response = server.post("/trees/add").json(&tree_add_data).await;
    response.assert_status_ok();
    let id: i32 = response.json();
    login(&server, "2@gmail.com".to_owned()).await;

    let response = server.post("/trees/1/duplicate").await;
    response.assert_status_ok();
    let id2: i32 = response.json();
    assert_eq!(id2, id + 1);

    let response = server.get(&format!("/trees/get-with-cards/{id2}")).await;
    let tree_with_cards: TreeWithCards = response.json();
    let tree_data2 = tree_with_cards.metadata.data;
    let expected_tree_data = TreeData {
        title: "Copy of test".to_owned(),
        ..Default::default()
    };
    assert_eq!(tree_data2, expected_tree_data);
    let tree_gen_data = tree_with_cards.metadata.gen_data;
    assert_eq!(tree_gen_data.owner_id, owner_id2);
    let tree_structure2 = tree_with_cards.structure;
    assert_eq!(
        tree_structure.root_nodes.len(),
        tree_structure2.root_nodes.len()
    );
}

#[sqlx::test]
async fn test_verify_tree_permission_owner(db: PgPool) {
    let server = setup_login(db.clone()).await;

    let tree_id: i32 = create_tree(&server).await;

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Author,
    };

    let result = verify_tree_permissions(get_connection_unwrap!(db), tree_id, &claims).await;

    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_verify_tree_permission_admin(db: PgPool) {
    let server = setup_login(db.clone()).await;

    let tree_id: i32 = create_tree(&server).await;

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Admin,
    };

    let result = verify_tree_permissions(get_connection_unwrap!(db), tree_id, &claims).await;

    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_verify_tree_permission_co_author(db: PgPool) {
    let server = setup_login(db.clone()).await;

    let tree_id: i32 = create_tree(&server).await;

    let co_author_register_data = register(&server, "co_author@email.com".to_owned()).await.1;
    server
        .post(&format!(
            "/trees/{tree_id}/authorize/{}",
            co_author_register_data.data.name
        ))
        .await;

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Author,
    };

    let result = verify_tree_permissions(get_connection_unwrap!(db), tree_id, &claims).await;

    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_authorize_co_author_success(db: PgPool) {
    let server = setup_login(db.clone()).await;

    let tree_id: i32 = create_tree(&server).await;

    let (co_author_id, co_author_register_data) =
        register(&server, "co_author@email.com".to_owned()).await;
    let response = server
        .post(&format!(
            "/trees/{tree_id}/authorize/{}",
            co_author_register_data.data.name
        ))
        .await;

    response.assert_status_ok();

    let is_co_author = is_co_author_of_tree(&db, &tree_id, &co_author_id)
        .await
        .unwrap();

    assert!(is_co_author);
}

#[sqlx::test]
async fn test_non_owner_cannot_authorize_co_author(db: PgPool) {
    let server = setup_login(db.clone()).await;

    let tree_id: i32 = create_tree(&server).await;

    register(&server, "not_tree_owner@email.com".to_owned()).await;
    login(&server, "not_tree_owner@email.com".to_owned()).await;

    let co_author_register_data = register(&server, "co_author@email.com".to_owned()).await.1;
    let response = server
        .post(&format!(
            "/trees/{tree_id}/authorize/{}",
            co_author_register_data.data.name
        ))
        .await;

    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_cannot_authorize_non_existing_user(db: PgPool) {
    let server = setup_login(db.clone()).await;

    let tree_id: i32 = create_tree(&server).await;

    let co_author_username = "Non-existent_user".to_owned();
    let response = server
        .post(&format!("/trees/{tree_id}/authorize/{co_author_username}"))
        .await;
    response.assert_status_bad_request();
}

#[sqlx::test]
async fn test_acquire_lock(db: PgPool) {
    use crate::api_v1::trees::edit_lock::{BackendMessage, FrontendMessage};
    let server = setup_with_options(db.clone(), true);
    let email1 = "a@localhost".to_owned();
    let email2 = "b@localhost".to_owned();
    let user_id1 = register(&server, email1.clone()).await;
    let user_id2 = register(&server, email2.clone()).await;

    login(&server, email1.clone()).await;
    let tree_id = create_tree(&server).await;
    server
        .post(&format!("/trees/{tree_id}/authorize/{user_id2}"))
        .await
        .assert_status_ok();

    let create_ws = async || {
        let response = server
            .get_websocket(&format!("/trees/{tree_id}/acquire-edit-lock"))
            .await;
        response.assert_status_switching_protocols();
        response.into_websocket().await
    };

    // First user 1 has the lock
    let mut ws1 = create_ws().await;
    ws1.assert_receive_json(&BackendMessage::LockAchieved).await;

    // User 2 first fails, then forces
    // (also enforcing that the default active_status is set to true)
    login(&server, email2.clone()).await;
    let mut ws2 = create_ws().await;
    ws2.assert_receive_json(&BackendMessage::LockFailed {
        active_user: user_id1,
    })
    .await;
    ws2.send_json(&FrontendMessage::Force).await;
    ws1.assert_receive_json(&BackendMessage::KickedOutBy { user_id: user_id2 })
        .await;
    ws2.assert_receive_json(&BackendMessage::LockAchieved).await;

    // User 2 becomes inactive, user 1 kicks out user 2 without forcing
    ws2.send_json(&FrontendMessage::SetActive {
        new_active_state: false,
    })
    .await;
    login(&server, email1.clone()).await;
    ws1 = create_ws().await;
    ws1.assert_receive_json(&BackendMessage::LockAchieved).await;
    ws2.assert_receive_json(&BackendMessage::KickedOutBy { user_id: user_id1 })
        .await;

    // User 1 closes the websocket (by for example leaving page),
    // the lock is gone, so user 2 doesn't need to force the lock.
    ws1.close().await;
    ws2 = create_ws().await;
    ws2.assert_receive_json(&BackendMessage::LockAchieved).await;
}
