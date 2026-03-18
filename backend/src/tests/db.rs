#![cfg(test)]

use crate::db::cards::{add_card, update_card};
use crate::db::trees::add_tree;
use crate::db::users::add_user;
use crate::db_structs::cards::{CardAddData, CardUpdateData};
use crate::db_structs::trees::TreeData;
use crate::db_structs::users::{UserData, UserLanguage};
use crate::errors::ApiError;
use crate::get_connection;
use sqlx::{PgPool, query};

#[sqlx::test]
async fn test_db_functions(db: PgPool) {
    async fn happy_path(db: PgPool) -> Result<(), ApiError> {
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

        let tree_id = add_tree(&db, tree_data, user_id).await;
        assert!(tree_id.is_ok());
        let tree_id = tree_id?;

        let card_add_data = CardAddData {
            tree_id,
            front: "spoiler".to_string(),
            back: "Dumbledore dies".to_string(),
        };

        let card_id = add_card(&db, card_add_data).await;
        assert!(card_id.is_ok());
        let card_id = card_id?;

        let card_update_data = CardUpdateData {
            card_id,
            new_front: "not a spoiler".to_string(),
            new_back: "Harry is the Chosen One".to_string(),
        };

        let void = update_card(get_connection!(db), card_update_data).await;
        assert!(void.is_ok());

        Ok(())
    }

    happy_path(db).await.unwrap();
}
