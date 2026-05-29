/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::classrooms::{ClassroomAddData, LessonAddData};
use crate::errors::ApiError;
use chrono::Utc;
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

/// Get the id of the owner of a classroom.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `classroom_id` - The id of the classroom of which the owner is queried.
///
/// # Returns
/// The id of the owner of the classroom as a [`Some`] if the classroom exists, or a [`None`] if it doesn't.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_classroom_owner(
    db: impl PgExecutor<'_>,
    classroom_id: i32,
) -> Result<Option<i32>, ApiError> {
    let owner = query!("SELECT owner_id FROM classrooms WHERE id = $1", classroom_id,)
        .fetch_optional(db)
        .await?
        .map(|r| r.owner_id);

    Ok(owner)
}

/// Adds a lesson to the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `lesson_add_data` - The data for the lesson in the form of [`LessonAddData`].
///
/// `classroom_id` - The id of the classroom to which the lesson will belong.
///
/// # Returns
/// The id of the lesson.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn add_lesson(
    db: &mut PgConnection,
    lesson_add_data: &LessonAddData,
    classroom_id: i32,
) -> Result<i32, ApiError> {
    let order_number: Option<i32> = query!(
        "SELECT MAX(order_number) FROM lessons WHERE classroom_id=$1",
        classroom_id,
    ).fetch_one(&mut *db).await?.max;
    let id: i32 = query!(
        "INSERT INTO lessons (classroom_id, order_number, name, description, published) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        classroom_id,
        (order_number.unwrap_or(0))+1,
        lesson_add_data.name,
        lesson_add_data.description,
        lesson_add_data.published,
    ).fetch_one(&mut *db).await?.id;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use crate::db::classrooms::{add_classroom, add_lesson, generate_random_code};
    use crate::db::test::{initialize_classroom, initialize_db};
    use crate::db_structs::classrooms::{ClassroomAddData, LessonAddData};
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

    #[sqlx::test]
    async fn test_add_lesson(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), user_id).await;

        let lesson_add_data = LessonAddData {
            name: "Lesson".to_string(),
            description: "My very first lesson".to_string(),
            published: false,
        };

        let id1 = add_lesson(get_connection_unwrap!(db), &lesson_add_data, classroom_id).await.unwrap();
        let id2 = add_lesson(get_connection_unwrap!(db), &lesson_add_data, classroom_id).await.unwrap();
        let found_lesson_data1 = query!("SELECT * FROM lessons WHERE id = $1", id1).fetch_one(&db).await.unwrap();
        let found_lesson_data2 = query!("SELECT  * FROM lessons WHERE id = $1", id2).fetch_one(&db).await.unwrap();

        let found_lesson_add_data = LessonAddData {
            name: found_lesson_data1.name,
            description: found_lesson_data1.description,
            published: found_lesson_data1.published,
        };

        assert_eq!(found_lesson_add_data, lesson_add_data);
        assert_eq!(found_lesson_data1.order_number + 1, found_lesson_data2.order_number);
    }
}