/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::classrooms::ClassroomAddData;
use crate::errors::ApiError;
use rand::prelude::*;
use sqlx::{PgConnection, PgExecutor, query};

/// In a loop, generates a random join code consisting of 6 characters and checks if it already exists.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// # Returns
/// A unique join code.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn generate_random_code(db: &mut PgConnection) -> Result<String, ApiError> {
    let allowed_characters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();

    let mut number: String;

    loop {
        number = {
            let mut rng = rand::rng();
            (0..6)
                .map(|_| allowed_characters.choose(&mut rng).unwrap())
                .collect::<String>()
        };

        let exists = query!(
            "SELECT EXISTS(SELECT * FROM classrooms WHERE join_code = $1)",
            number
        )
        .fetch_one(&mut *db)
        .await?
        .exists
        .unwrap();

        if !exists {
            return Ok(number);
        }
    }
}

/// Adds a classroom to the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `name` - The nickname for the classroom.
///
/// `description` - The description of the classroom.
///
/// `owner_id` - The id of the user that creates the classroom and will be its owner.
///
/// `join_code` - The unique code that students can join the classroom with.
///
/// # Returns
/// The id of the classroom.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn add_classroom(
    db: impl PgExecutor<'_>,
    classroom_add_data: &ClassroomAddData,
    owner_id: i32,
    join_code: &str,
) -> Result<i32, ApiError> {
    let id: i32 = query!(
        "INSERT INTO classrooms (owner_id, name, description, join_code) VALUES ($1, $2, $3, $4) RETURNING id",
        owner_id,
        classroom_add_data.name,
        classroom_add_data.description,
        join_code
    )
    .fetch_one(db)
    .await?
    .id;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use crate::db::classrooms::{add_classroom, generate_random_code};
    use crate::db::test::initialize_db;
    use crate::db_structs::classrooms::ClassroomAddData;
    use crate::get_connection_unwrap;
    use sqlx::{PgPool, query};

    #[sqlx::test]
    async fn test_add_classroom(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;

        let classroom_add_data = ClassroomAddData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        let id = add_classroom(&db, &classroom_add_data, user_id, &join_code)
            .await
            .unwrap();

        let found_classroom_data = query!("SELECT * FROM classrooms WHERE id = $1", id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_classroom_add_data = ClassroomAddData {
            name: found_classroom_data.name,
            description: found_classroom_data.description,
        };

        assert_eq!(found_classroom_add_data, classroom_add_data);
        assert_eq!(found_classroom_data.join_code, join_code);
    }
}
