/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::classrooms::ClassroomAddData;
use crate::errors::ApiError;
use chrono::Utc;
use rand::prelude::*;
use rand::rngs::SysRng;
use sqlx::{Connection, PgConnection, PgExecutor, query, query_scalar};

/// In a loop, generates a random join code consisting of 6 characters and checks if it already exists.
///
/// # Arguments
/// `db` - A [`&mut PgConnection`](PgConnection) to the Postgres database.
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
/// `classroom_add_data` - The basic data for the classroom filled in by the user.
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
    Ok(
        query!(
            "INSERT INTO classrooms (owner_id, name, description, join_code) VALUES ($1, $2, $3, $4) RETURNING id",
            owner_id,
            classroom_add_data.name,
            classroom_add_data.description,
            join_code
        )
        .fetch_one(db)
        .await?
        .id
    )
}

/// Validates the join code for a classroom.
/// Also checks if the classroom is joinable.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `join_code` - The unique join code for the classroom.
///
/// # Returns
/// The id of the classroom if the join code is correct and the classroom is joinable.
///
/// # Errors
/// If the join code is not correct or the classroom is not joinable, returns a [`WrongClassroomJoinCode`](ApiError::WrongClassroomJoinCode).
///
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn validate_join_code(db: impl PgExecutor<'_>, join_code: &str) -> Result<i32, ApiError> {
    let classroom = query!(
        "SELECT id, joinable FROM classrooms WHERE join_code = $1",
        join_code
    )
    .fetch_optional(db)
    .await?
    .ok_or(ApiError::WrongClassroomJoinCode(join_code.to_string()))?;

    if !classroom.joinable {
        return Err(ApiError::WrongClassroomJoinCode(join_code.to_string()));
    }
    Ok(classroom.id)
}

/// Adds a new user to the `classroom_users` database table for some classroom.
/// Uses a transaction to ensure no two users get the same assigned number.
///
/// # Arguments
/// `db` - A [`&mut PgConnection`](PgConnection) to the Postgres database.
///
/// `classroom_id` - The id of the classroom.
///
/// `name` - The name of the new user. Doesn't have to be unique.
///
/// # Returns
/// The unique number assigned to the user.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn add_new_user_to_classroom(
    db: &mut PgConnection,
    classroom_id: i32,
    name: &str,
) -> Result<i32, ApiError> {
    let mut transaction = (*db).begin().await?;

    let num = query!(
        "SELECT student_id_increment FROM classrooms WHERE id = $1 FOR NO KEY UPDATE",
        classroom_id
    )
    .fetch_one(&mut *transaction)
    .await?
    .student_id_increment;

    query!(
        "UPDATE classrooms SET student_id_increment = $1 WHERE id = $2",
        num + 1,
        classroom_id
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    query!(
        "INSERT INTO classroom_users (classroom_id, name, id_within_classroom) VALUES ($1, $2, $3)",
        classroom_id,
        name,
        num
    )
    .execute(&mut *db)
    .await?;

    Ok(num)
}

/// Retrieves the name connected to a user number in a classroom.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `classroom_id` - The id of the classroom.
///
/// `user_id` - The user number. Is unique within a classroom.
///
/// # Returns
/// The name assigned to the user in a [`Option`] wrapper.
/// If the user number doesn't exist, returns `None`.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn get_name_by_number(
    db: impl PgExecutor<'_>,
    classroom_id: i32,
    user_id: i32,
) -> Result<Option<String>, ApiError> {
    Ok(query_scalar!(
        "SELECT name FROM classroom_users WHERE classroom_id = $1 AND id = $2",
        classroom_id,
        user_id
    )
    .fetch_optional(db)
    .await?)
}

#[cfg(test)]
mod tests {
    use crate::db::classrooms::{
        add_classroom, add_new_user_to_classroom, generate_random_code, get_name_by_number,
        validate_join_code,
    };
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

    #[sqlx::test]
    async fn test_validate_join_code(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;

        let join_code = "TESTIN";

        assert!(validate_join_code(&db, join_code).await.is_err());

        let classroom_add_data = ClassroomAddData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let id = add_classroom(&db, &classroom_add_data, user_id, join_code)
            .await
            .unwrap();

        let retrieved_id = validate_join_code(&db, join_code).await;

        assert!(retrieved_id.is_ok());
        assert_eq!(retrieved_id.unwrap(), id);
    }

    #[sqlx::test]
    async fn test_add_new_user_to_classroom(db: PgPool) {
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

        let name = "Gelicenseerd radiozendamateur".to_owned();

        let user_num = add_new_user_to_classroom(get_connection_unwrap!(db), id, &name).await;

        assert!(user_num.is_ok());

        let found_user_name = get_name_by_number(&db, id, user_num.unwrap())
            .await
            .unwrap();

        assert_eq!(found_user_name, Some(name));
    }
}
