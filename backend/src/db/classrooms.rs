/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::classrooms::{
    ClassroomData, ClassroomGeneratedData, ClassroomMetaData, LessonAddData,
};
use crate::db_structs::pagination::PaginationOptions;
use crate::errors::ApiError;
use chrono::{DateTime, Utc};
use rand::prelude::*;
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
        "SELECT name FROM classroom_users WHERE classroom_id = $1 AND id_within_classroom = $2",
        classroom_id,
        user_id
    )
    .fetch_optional(db)
    .await?)
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
    let owner = query!(
        "SELECT owner_id FROM classrooms WHERE id = $1",
        classroom_id,
    )
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
#[allow(clippy::cast_possible_truncation)]
pub async fn add_lesson(
    db: &mut PgConnection,
    lesson_add_data: &LessonAddData,
    classroom_id: i32,
) -> Result<i32, ApiError> {
    let mut transaction = db.begin().await?;
    let order_number: i32 = query_scalar!(
        "SELECT COUNT(*) FROM lessons WHERE classroom_id=$1",
        classroom_id,
    )
    .fetch_one(&mut *transaction)
    .await?
    .unwrap_or(0) as i32;
    let id: i32 = query!(
        "INSERT INTO lessons (classroom_id, order_number, name, description, published) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        classroom_id,
        (order_number + 1),
        lesson_add_data.name,
        lesson_add_data.description,
        lesson_add_data.published,
    ).fetch_one(&mut *transaction).await?.id;
    transaction.commit().await?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use crate::db::classrooms::{
        add_classroom, add_lesson, add_new_user_to_classroom, generate_random_code,
        get_all_classrooms, get_classroom_count, get_classroom_owner, get_name_by_number,
        validate_join_code,
    };
    use crate::db::test::{initialize_classroom, initialize_db};
    use crate::db_structs::classrooms::{ClassroomData, LessonAddData};
    use crate::db_structs::pagination::PaginationOptions;
    use crate::get_connection_unwrap;
    use sqlx::{PgPool, query};

    #[sqlx::test]
    async fn test_add_classroom(db: PgPool) {
        let (claims, _) = initialize_db(get_connection_unwrap!(db)).await;

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        let (id, _) = add_classroom(&db, &classroom_add_data, claims.sub, &join_code)
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
    async fn test_validate_join_code(db: PgPool) {
        let (claims, _) = initialize_db(get_connection_unwrap!(db)).await;

        let join_code = "TESTIN";

        assert!(validate_join_code(&db, join_code).await.is_err());

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let (id, _) = add_classroom(&db, &classroom_add_data, claims.sub, join_code)
            .await
            .unwrap();

        let retrieved_id = validate_join_code(&db, join_code).await;

        assert!(retrieved_id.is_ok());
        assert_eq!(retrieved_id.unwrap(), id);
    }

    #[sqlx::test]
    async fn test_add_new_user_to_classroom(db: PgPool) {
        let (claims, _) = initialize_db(get_connection_unwrap!(db)).await;

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        let (id, _) = add_classroom(&db, &classroom_add_data, claims.sub, &join_code)
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

    #[sqlx::test]
    async fn test_get_all_classrooms(db: PgPool) {
        let (claims, _) = initialize_db(get_connection_unwrap!(db)).await;

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        let (classroom_id, _) = add_classroom(&db, &classroom_add_data, claims.sub, &join_code)
            .await
            .unwrap();

        let pagination_options = PaginationOptions::new(1, 5, 1000, 500).unwrap();

        let classrooms = get_all_classrooms(&db, claims.sub, &pagination_options)
            .await
            .unwrap();

        assert_eq!(classroom_id, classrooms[0].gen_data.id);
        assert_eq!(join_code, classrooms[0].gen_data.join_code);
        assert_eq!(classroom_add_data.name, classrooms[0].data.name);
        assert_eq!(
            classroom_add_data.description,
            classrooms[0].data.description
        );
        assert_eq!(claims.sub, classrooms[0].gen_data.owner_id);
    }

    #[sqlx::test]
    async fn test_get_classroom_count(db: PgPool) {
        let (claims, _) = initialize_db(get_connection_unwrap!(db)).await;
        assert_eq!(get_classroom_count(&db, claims.sub).await.unwrap(), 0);

        let classroom_add_data = ClassroomData {
            name: "Classroom".to_owned(),
            description: "This is a test classroom".to_owned(),
        };

        let join_code = generate_random_code(get_connection_unwrap!(db))
            .await
            .unwrap();

        add_classroom(&db, &classroom_add_data, claims.sub, &join_code)
            .await
            .unwrap();

        assert_eq!(get_classroom_count(&db, claims.sub).await.unwrap(), 1);
    }

    #[sqlx::test]
    async fn test_get_classroom_owner(db: PgPool) {
        let (claims, _) = initialize_db(get_connection_unwrap!(db)).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), claims.sub).await;

        let found_owner_id = get_classroom_owner(get_connection_unwrap!(db), classroom_id)
            .await
            .unwrap();
        assert_eq!(found_owner_id, Some(claims.sub));
    }

    #[sqlx::test]
    async fn test_add_lesson(db: PgPool) {
        let (claims, _) = initialize_db(get_connection_unwrap!(db)).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), claims.sub).await;

        let lesson_add_data = LessonAddData {
            name: "Lesson".to_string(),
            description: "My very first lesson".to_string(),
            published: false,
        };

        let id1 = add_lesson(get_connection_unwrap!(db), &lesson_add_data, classroom_id)
            .await
            .unwrap();
        let id2 = add_lesson(get_connection_unwrap!(db), &lesson_add_data, classroom_id)
            .await
            .unwrap();
        let found_lesson_data1 = query!("SELECT * FROM lessons WHERE id = $1", id1)
            .fetch_one(&db)
            .await
            .unwrap();
        let found_lesson_data2 = query!("SELECT  * FROM lessons WHERE id = $1", id2)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_lesson_add_data = LessonAddData {
            name: found_lesson_data1.name,
            description: found_lesson_data1.description,
            published: found_lesson_data1.published,
        };

        assert_eq!(found_lesson_add_data, lesson_add_data);
        assert_eq!(
            found_lesson_data1.order_number + 1,
            found_lesson_data2.order_number
        );
    }
}
