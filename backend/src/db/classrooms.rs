/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use rand::prelude::*;
use rand::rngs::SysRng;
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
    let allowed_letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let allowed_numbers: Vec<char> = "0123456789".chars().collect();
    let chance_character_is_letter = 0.5;

    let mut rng = StdRng::try_from_rng(&mut SysRng).unwrap();
    let mut number: String;

    loop {
        number = (0..6)
            .map(|_| {
                if rng.random_bool(chance_character_is_letter) {
                    allowed_letters.choose(&mut rng).unwrap()
                } else {
                    allowed_numbers.choose(&mut rng).unwrap()
                }
            })
            .collect::<String>();

        let exists = query!(
            "SELECT EXISTS(SELECT * FROM classrooms WHERE join_code = $1)",
            number
        )
        .fetch_one(&mut *db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .exists
        .unwrap();

        if exists {
            break;
        }
    }

    Ok(number)
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
    name: &String,
    description: &String,
    owner_id: i32,
    join_code: &String,
) -> Result<i32, ApiError> {
    let id: i32 = query!(
        "INSERT INTO classrooms (owner_id, nickname, description, join_code) VALUES ($1, $2, $3, $4) RETURNING id",
        owner_id,
        name,
        description,
        join_code
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
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
            description: Option::from(String::new()),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        let description = classroom_add_data.description.unwrap_or_default();

        let id = add_classroom(
            &db,
            &classroom_add_data.name,
            &description,
            user_id,
            &join_code,
        )
        .await
        .unwrap();

        let found_classroom_data = query!("SELECT * FROM classrooms WHERE id = $1", id)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(found_classroom_data.nickname, classroom_add_data.name);
        assert_eq!(found_classroom_data.description, description);
        assert_eq!(found_classroom_data.join_code, join_code);
    }
}
