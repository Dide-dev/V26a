#![cfg(test)]

use crate::db::cards::{add_card, update_card};
use crate::db::trees::{add_tree, db_get_tree_all_data, get_tree_owner, update_tree_structure};
use crate::db::users::add_user;
use crate::db_structs::cards::{CardAddData, CardUpdateData};
use crate::db_structs::trees::{Node, RootNode, TreeData, TreeStructure};
use crate::db_structs::users::{UserData, UserLanguage};
use crate::errors::ApiError;
use crate::get_connection;
use sqlx::{PgPool, query};

#[sqlx::test]
async fn test_db_functions(db: PgPool) {
    /// Test functions from the `db` folder with arguments that should succeed,
    /// simulating the happy path.
    ///
    /// # Tested functions
    /// `cards.rs` - [`add_card`] & [`update_card`].
    ///
    /// `trees.rs` - [`add_tree`], [`update_tree_structure`].
    ///
    /// `users.rs` - [`add_user`].
    async fn happy_path(db: PgPool) -> Result<(), ApiError> {
        //-----------------//
        // Test `add_user` //
        //-----------------//
        let user_data = UserData {
            email: "test@mail.com".to_owned(),
            password: "password".to_owned(),
            name: "username".to_owned(),
            nickname: None,
            about_me: None,
            email_visible: false,
            language: UserLanguage::Nl,
        };

        let user_id = add_user(get_connection!(db), user_data).await;
        assert!(user_id.is_ok());
        let user_id = user_id?;

        let user_is_added = query!("SELECT EXISTS (SELECT 1 FROM users WHERE id = $1)", user_id,)
            .fetch_one(&db)
            .await;
        assert!(user_is_added.is_ok());
        let user_is_added = user_is_added.unwrap();
        assert!(user_is_added.exists.unwrap());

        //-----------------//
        // Test `add_tree` //
        //-----------------//
        let tree_data = TreeData {
            title: "tree".to_owned(),
            description: None,
            book_name: None,
            book_edition: None,
            book_isbn: None,
            book_chapter: None,
            book_exercise: None,
            assignment_text: None,
            published: false,
        };

        let tree_id = add_tree(&db, &tree_data, user_id).await;
        assert!(tree_id.is_ok());
        let tree_id = tree_id?;

        let tree_is_added = query!("SELECT EXISTS (SELECT 1 FROM trees WHERE id = $1)", tree_id,)
            .fetch_one(&db)
            .await;
        assert!(tree_is_added.is_ok());
        let tree_is_added = tree_is_added.unwrap();
        assert!(tree_is_added.exists.unwrap());

        //-----------------//
        // Test `add_card` //
        //-----------------//
        let card_add_data = CardAddData {
            tree_id,
            front: "spoiler".to_string(),
            back: "Dumbledore dies".to_string(),
        };

        let card_id = add_card(&db, card_add_data).await;
        assert!(card_id.is_ok());
        let card_id = card_id?;

        let card_is_added = query!("SELECT EXISTS (SELECT 1 FROM cards WHERE id = $1)", card_id,)
            .fetch_one(&db)
            .await;
        assert!(card_is_added.is_ok());
        let card_is_added = card_is_added.unwrap();
        assert!(card_is_added.exists.unwrap());

        //------------------------------//
        // Test `update_tree_structure` //
        //------------------------------//
        let new_tree_structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![Node {
                    card_id,
                    children: vec![],
                }],
            }],
        };

        let void = update_tree_structure(&db, tree_id, &new_tree_structure).await;
        assert!(void.is_ok());

        let updated_tree_structure = query!("SELECT structure FROM trees WHERE id = $1", tree_id,)
            .fetch_one(&db)
            .await;
        assert!(updated_tree_structure.is_ok());

        let updated_tree_structure =
            serde_json::from_value::<TreeStructure>(updated_tree_structure.unwrap().structure)
                .unwrap();

        assert_eq!(new_tree_structure, updated_tree_structure);

        //--------------------//
        // Test `update_card` //
        //--------------------//
        let card_update_data = CardUpdateData {
            card_id,
            new_front: "not a spoiler".to_string(),
            new_back: "Harry is the Chosen One".to_string(),
        };

        let void = update_card(get_connection!(db), &card_update_data).await;
        assert!(void.is_ok());

        let updated_card_data = query!("SELECT front, back FROM cards WHERE id = $1", card_id,)
            .fetch_one(&db)
            .await;
        let updated_card_data = updated_card_data.unwrap();

        assert_eq!(&card_update_data.new_front, &updated_card_data.front);
        assert_eq!(&card_update_data.new_back, &updated_card_data.back);

        //-----------------------------//
        // Test `db_get_tree_all_data` //
        //-----------------------------//

        let all_tree_data = db_get_tree_all_data(&db, tree_id).await;
        assert!(all_tree_data.is_ok());

        let all_tree_data = all_tree_data?;
        assert!(all_tree_data.is_some());

        let (found_tree_data, found_tree_gen_data, found_tree_structure) = all_tree_data.unwrap();
        assert_eq!(tree_data, found_tree_data);
        assert_eq!(tree_id, found_tree_gen_data.id);
        assert_eq!(user_id, found_tree_gen_data.owner_id);
        assert_eq!(new_tree_structure, found_tree_structure);

        //-----------------------//
        // Test `get_tree_owner` //
        //-----------------------//

        let tree_owner_id = get_tree_owner(&db, tree_id).await;
        assert!(tree_owner_id.is_ok());
        let tree_owner_id = tree_owner_id?;
        assert!(tree_owner_id.is_some());
        let tree_owner_id = tree_owner_id.unwrap();

        assert_eq!(user_id, tree_owner_id);

        Ok(())
    }

    happy_path(db).await.unwrap();
}
