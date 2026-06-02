/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::classrooms::{ClassroomAddData, ClassroomAllData};
use crate::db_structs::pagination::PaginationOptions;
use crate::errors::ApiError;
use chrono::Utc;
use rand::prelude::*;
use rand::rngs::SysRng;
use sqlx::{PgConnection, PgExecutor, query, query_as, query_scalar};

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

    let mut rng = StdRng::try_from_rng(&mut SysRng).unwrap_or_else(|_| {
        StdRng::from_seed({
            let unix = Utc::now().timestamp_millis().to_be_bytes();
            let mut array: [u8; 32] = [42; 32];
            for chunk in array.chunks_mut(unix.len()) {
                let len = chunk.len().min(unix.len());
                chunk[..len].copy_from_slice(&unix[..len]);
            }
            array
        })
    });
    let mut number: String;

    loop {
        number = (0..6)
            .map(|_| allowed_characters.choose(&mut rng).unwrap())
            .collect::<String>();

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

/// Get the list of all classrooms at a given page.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `page` - Which page is requested.
///
/// `id` - The id of the user for whom we need to return the amount of classrooms.
///
/// # Returns
/// A list with the classrooms at the requested page,
///
/// # Errors
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_all_classrooms(
    db: impl PgExecutor<'_>,
    id: i32,
    page: &PaginationOptions,
) -> Result<Vec<ClassroomAllData>, ApiError> {
    let limit = i64::from(page.get_size());
    let offset = i64::from((page.get_page() - 1) * page.get_size());
    let classrooms = query_as!(
        ClassroomAllData,
        "SELECT name, description, id, join_code, created_at, owner_id FROM classrooms
         WHERE owner_id = $3
         LIMIT $1 OFFSET $2",
        limit,
        offset,
        id
    )
    .fetch_all(db)
    .await?;

    if classrooms.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    Ok(classrooms)
}

/// Gets the amount of classrooms a user owns.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `id` - The id of the user for whom we need to return the amount of classrooms.
///
/// # Returns
/// The amount of classrooms a user owns,
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_classroom_count(db: impl PgExecutor<'_>, id: i32) -> Result<i64, ApiError> {
    let classroom_count: i64 =
        query_scalar!("SELECT COUNT(*) FROM classrooms WHERE owner_id = $1", id)
            .fetch_one(db)
            .await?
            .ok_or(ApiError::DatabaseException)?;

    Ok(classroom_count)
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
