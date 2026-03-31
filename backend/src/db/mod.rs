/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

pub mod auth;
pub mod cards;
pub mod trees;
pub mod users;

/// Get a [`PgConnection`](sqlx::PgConnection) from a [`PgPool`](sqlx::PgPool).
///
/// # Errors
/// May return a [`DatabaseException`](crate::errors::ApiError::DatabaseException) if the connection
/// can not be acquired.
#[macro_export]
macro_rules! get_connection {
    ( $pool:expr ) => {
        &mut *($pool
            .acquire()
            .await
            .map_err(|_| ApiError::DatabaseException)?)
    };
}

/// Get a [`PgConnection`](sqlx::PgConnection) from a [`PgPool`](sqlx::PgPool).
///
/// **Warning:** might panic. If not writing tests, use the [`get_connection`] macro instead.
/// Exists only for use in tests, where the return type does not and should not match
/// Result<_, [`ApiError`](crate::errors::ApiError)>.
///
/// # Panic
/// May panic if the connection cannot be acquired.
#[cfg(test)]
#[macro_export]
macro_rules! get_connection_unwrap {
    ( $pool:expr ) => {
        &mut *($pool.acquire().await.unwrap())
    };
}

#[cfg(test)]
pub mod test {
    use crate::db_structs::cards::CardAddData;
    use crate::db_structs::trees::{RootNode, TreeData, TreeStructure};
    use crate::db_structs::users::UserData;
    use sqlx::{PgConnection, PgExecutor, query};

    /// Initialize a database with a user and a tree.
    ///
    /// # Returns
    /// A tuple containing the `user_id` as its first value and the `tree_id` as its second value.
    pub async fn initialize_db(db: &mut PgConnection) -> (i32, i32) {
        let user_id = initialize_basic_user(&mut *db).await;
        let tree_id = initialize_empty_tree(&mut *db, user_id).await;

        (user_id, tree_id)
    }

    /// Initialize a user in the database.
    ///
    /// # Returns
    /// The `user_id` associated with the newly added user.
    pub async fn initialize_basic_user(db: impl PgExecutor<'_>) -> i32 {
        query!(
            "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING id",
            "test@email.com",
            "password_hash",
            "name"
        )
        .fetch_one(db)
        .await
        .unwrap()
        .id
    }

    /// Initialize a user in the database with the provided user data.
    ///
    /// # Returns
    /// The `user_id` associated with the newly added user.
    pub async fn initialize_user(db: impl PgExecutor<'_>, user_data: &UserData) -> i32 {
        let password_hash = bcrypt::hash(user_data.password.clone(), 6).unwrap();

        query!(
            "INSERT INTO users (email, password_hash, name, nickname, about_me, email_visible, language) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
            user_data.email,
            password_hash,
            user_data.name,
            user_data.nickname,
            user_data.about_me,
            user_data.email_visible,
            user_data.language.to_string(),
        )
        .fetch_one(db)
        .await
        .unwrap()
        .id
    }

    /// Initialize an empty tree in the database.
    ///
    /// # Returns
    /// The `tree_id` associated with the newly added tree.
    pub async fn initialize_empty_tree(db: impl PgExecutor<'_>, owner_id: i32) -> i32 {
        let data = TreeData {
            title: "title".to_string(),
            description: None,
            book_name: None,
            book_edition: None,
            book_isbn: None,
            book_chapter: None,
            book_exercise: None,
            assignment_text: None,
            published: false,
        };

        let structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![],
            }],
        };

        initialize_tree_with_data(db, owner_id, &data, &structure).await
    }

    /// Initialize a tree in the database.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `owner_id` - The `user_id` associated with the intended owner of the tree.
    /// Must exist as an `id` in the `users` table.
    ///
    /// `tree_data` - The data added to the tree as [`TreeData`].
    ///
    /// `tree_structure` - The structure added to the tree as [`TreeStructure`].
    ///
    /// # Returns
    /// The `tree_id` associated with the newly added tree.
    pub async fn initialize_tree_with_data(
        db: impl PgExecutor<'_>,
        owner_id: i32,
        tree_data: &TreeData,
        tree_structure: &TreeStructure,
    ) -> i32 {
        let structure = serde_json::to_value(tree_structure).unwrap();

        query!(
            "INSERT INTO trees (owner_id, published, title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, structure) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id",
            owner_id,
            tree_data.published,
            tree_data.title,
            tree_data.description,
            tree_data.book_name,
            tree_data.book_edition,
            tree_data.book_isbn,
            tree_data.book_chapter,
            tree_data.book_exercise,
            tree_data.assignment_text,
            structure,
        )
            .fetch_one(db)
            .await
            .unwrap()
            .id
    }

    /// Initialize a card into the database.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
    ///
    /// `tree_id` - The id associated with the tree the card should be added to.
    /// Must exist as an `id` in the `trees` table.
    ///
    /// # Returns
    /// The `card_id` associated with the newly added card.
    pub async fn initialize_card(db: impl PgExecutor<'_>, tree_id: i32) -> i32 {
        let card_add_data = CardAddData {
            tree_id,
            front: "front".to_owned(),
            back: "back".to_owned(),
        };

        initialize_card_with_data(db, &card_add_data).await
    }

    /// Initialize a card into the database with the provided card data.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `card_data` - The data added to the card as [`CardAddData`].
    ///
    /// # Returns
    /// The `card_id` associated with the newly added card.
    pub async fn initialize_card_with_data(
        db: impl PgExecutor<'_>,
        card_data: &CardAddData,
    ) -> i32 {
        query!(
            "INSERT INTO cards (tree_id, back, front) VALUES ($1, $2, $3) RETURNING id",
            card_data.tree_id,
            card_data.back,
            card_data.front,
        )
        .fetch_one(db)
        .await
        .unwrap()
        .id
    }
}
