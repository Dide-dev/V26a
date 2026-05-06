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
/// A nonexisting join code.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn generate_random_code(db: &mut PgConnection) -> Result<String, ApiError> {
    let allowed_letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let allowed_numbers: Vec<char> = "0123456789".chars().collect();
    let is_letter_chance = 0.5;

    let mut rng = StdRng::from_entropy();
    let mut number: String;

    loop {
        number = (0..6)
            .map(|_| {
                if rng.gen_bool(is_letter_chance) {
                    allowed_letters.choose(&mut rng).unwrap()
                } else {
                    allowed_numbers.choose(&mut rng).unwrap()
                }
            })
            .collect::<String>();

        let query = query!("SELECT * FROM classrooms WHERE join_code = $1", number)
            .fetch_optional(&mut *db)
            .await
            .map_err(|_| ApiError::DatabaseException)?;

        if query.is_none() {
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
/// `classroom_add_data` - The data used to create the classroom, as [`ClassroomAddData`].
///
/// `owner_id` - The id of the user that creates the classroom and will be its owner.
///
/// `join_code` - The unique code that students can join the classroom with.
///
/// # Returns
/// The unique join code for the classroom.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn add_classroom(
    db: impl PgExecutor<'_>,
    classroom_add_data: &ClassroomAddData,
    owner_id: i32,
    join_code: &String,
) -> Result<i32, ApiError> {
    let classroom_id = query!(
        "INSERT INTO classrooms (owner_id, name, join_code) VALUES ($1, $2, $3) RETURNING id",
        owner_id,
        classroom_add_data.name,
        join_code
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(classroom_id)
}
