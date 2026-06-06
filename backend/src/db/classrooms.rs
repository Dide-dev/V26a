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
use rand::rngs::SysRng;
use sqlx::{Acquire, PgConnection, PgExecutor, query, query_scalar};

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

/// Get the classroom to which a lesson belongs.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `lesson_id` - The id of the classroom of which the owner is queried.
///
/// # Returns
/// The id of the classroom as a [`Some`] if the lesson exists, or a [`None`] if it doesn't.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_classroom_from_lesson(
    db: impl PgExecutor<'_>,
    lesson_id: i32,
) -> Result<Option<i32>, ApiError> {
    let classroom_id = query!("SELECT classroom_id FROM lessons WHERE id = $1", lesson_id)
        .fetch_optional(db)
        .await?
        .map(|r| r.classroom_id);
    Ok(classroom_id)
}

/// Adds an existing tree to a lesson in the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `lesson_id` - The id of the lesson to which the tree will belong.
///
/// `tree_id` - The id of the tree which should be added to the lesson.
///
/// # Returns
/// Nothing
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException).
pub async fn add_existing_tree_to_lesson(
    db: impl PgExecutor<'_>,
    lesson_id: i32,
    tree_id: i32,
) -> Result<(), ApiError> {
    query!(
        "INSERT INTO lesson_trees (lesson_id, tree_id) VALUES ($1, $2)",
        lesson_id,
        tree_id,
    )
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::classrooms::{
        add_classroom, add_existing_tree_to_lesson, add_lesson, generate_random_code,
        get_all_classrooms, get_classroom_count, get_classroom_owner,
    };
    use crate::db::test::{initialize_classroom, initialize_db};
    use crate::db_structs::classrooms::{ClassroomData, LessonAddData};
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

        let _ = initialize_classroom(get_connection_unwrap!(db), user_id).await;

        assert_eq!(get_classroom_count(&db, user_id).await.unwrap(), 1);
    }

    #[sqlx::test]
    async fn test_get_classroom_owner(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), user_id).await;

        let found_owner_id = get_classroom_owner(get_connection_unwrap!(db), classroom_id)
            .await
            .unwrap();
        assert_eq!(found_owner_id, Some(user_id));
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

    #[sqlx::test]
    async fn test_add_existing_tree_to_lesson(db: PgPool) {
        let (user_id, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), user_id).await;
        let lesson_add_data = LessonAddData {
            name: "Lesson".to_string(),
            description: "This is a test lesson".to_string(),
            published: false,
        };
        let lesson_id = add_lesson(get_connection_unwrap!(db), &lesson_add_data, classroom_id)
            .await
            .unwrap();

        add_existing_tree_to_lesson(get_connection_unwrap!(db), lesson_id, tree_id)
            .await
            .expect("");

        let found_tree_id_in_lesson = query!(
            "SELECT tree_id FROM lesson_trees WHERE lesson_id = $1",
            lesson_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .tree_id;

        assert_eq!(found_tree_id_in_lesson, tree_id);
    }
}
