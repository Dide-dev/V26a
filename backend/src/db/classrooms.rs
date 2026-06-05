/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::classrooms::{ClassroomData, ClassroomGeneratedData, ClassroomMetaData};
use crate::db_structs::pagination::PaginationOptions;
use crate::errors::ApiError;
use rand::prelude::*;
use sqlx::{PgConnection, PgExecutor, query, query_scalar};
use chrono::{DateTime, Utc};

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
    classroom_add_data: &ClassroomData,
    owner_id: i32,
    join_code: &str,
) -> Result<(i32, DateTime<Utc>), ApiError> {
    let record = query!(
        "INSERT INTO classrooms (owner_id, name, description, join_code) VALUES ($1, $2, $3, $4) RETURNING id, created_at",
        owner_id,
        classroom_add_data.name,
        classroom_add_data.description,
        join_code
    )
    .fetch_one(db)
    .await?;

    Ok((record.id, record.created_at))
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
/// A list with the classrooms at the requested page.
///
/// # Errors
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_all_classrooms(
    db: impl PgExecutor<'_>,
    id: i32,
    page: &PaginationOptions,
) -> Result<Vec<ClassroomMetaData>, ApiError> {
    let limit = i64::from(page.get_size());
    let offset = i64::from((page.get_page() - 1) * page.get_size());
    let records = query!(
        "SELECT name, description, id, join_code, created_at, owner_id FROM classrooms
         WHERE owner_id = $3
         LIMIT $1 OFFSET $2",
        limit,
        offset,
        id
    )
    .fetch_all(db)
    .await?;

    if records.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    let classrooms = records
        .into_iter()
        .map(|r| ClassroomMetaData {
            data: ClassroomData {
                name: r.name,
                description: r.description,
            },
            gen_data: ClassroomGeneratedData {
                id: r.id,
                join_code: r.join_code,
                created_at: r.created_at,
                owner_id: r.owner_id,
            },
        })
        .collect::<Vec<ClassroomMetaData>>();

    Ok(classrooms)
}

/// Gets the number of classrooms a user owns.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `id` - The id of the user for whom we need to return the number of classrooms.
///
/// # Returns
/// The number of classrooms a user owns.
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
    use crate::db::classrooms::{
        add_classroom, generate_random_code, get_all_classrooms, get_classroom_count,
    };
    use crate::db::test::initialize_db;
    use crate::db_structs::classrooms::ClassroomData;
    use crate::db_structs::pagination::PaginationOptions;
    use crate::get_connection_unwrap;
    use sqlx::{PgPool, query};

    #[sqlx::test]
    async fn test_add_classroom(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        let (id, _) = add_classroom(&db, &classroom_add_data, user_id, &join_code)
            .await
            .unwrap();

        let found_classroom_data = query!("SELECT * FROM classrooms WHERE id = $1", id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_classroom_add_data = ClassroomData {
            name: found_classroom_data.name,
            description: found_classroom_data.description,
        };

        assert_eq!(found_classroom_add_data, classroom_add_data);
        assert_eq!(found_classroom_data.join_code, join_code);
    }

    #[sqlx::test]
    async fn test_get_all_classrooms(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        let (classroom_id, _) = add_classroom(&db, &classroom_add_data, user_id, &join_code)
            .await
            .unwrap();

        let pagination_options = PaginationOptions::new(1, 5, 1000, 500).unwrap();

        let classrooms = get_all_classrooms(&db, user_id, &pagination_options)
            .await
            .unwrap();

        assert_eq!(classroom_id, classrooms[0].gen_data.id);
        assert_eq!(join_code, classrooms[0].gen_data.join_code);
        assert_eq!(classroom_add_data.name, classrooms[0].data.name);
        assert_eq!(
            classroom_add_data.description,
            classrooms[0].data.description
        );
        assert_eq!(user_id, classrooms[0].gen_data.owner_id);
    }

    #[sqlx::test]
    async fn test_get_classroom_count(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;
        assert_eq!(get_classroom_count(&db, user_id).await.unwrap(), 0);

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        add_classroom(&db, &classroom_add_data, user_id, &join_code)
            .await
            .unwrap();

        assert_eq!(get_classroom_count(&db, user_id).await.unwrap(), 1);
    }
}
