/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::create_token;
use crate::db::classrooms::{
    add_new_user_to_classroom, get_all_classrooms_from_ids, get_classroom_count_from_ids,
    get_classroom_metadata_from_id, get_lessons_with_tree_ids_from_classroom, get_name_by_number,
    validate_join_code,
};
use crate::db_structs::classrooms::{
    ClassroomListResponse, ClassroomLoginResponse, ClassroomStudentDashboardData,
    ClassroomUserLoginData,
};
use crate::db_structs::pagination::OptionalPaginationOptions;
use crate::db_structs::session::{AllClassroomClaims, ClaimsType, ClassroomClaims};
use crate::errors::ApiError;
use crate::get_connection;
use crate::state::ArcState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::{CookieJar, Query};
use chrono::Utc;
use sqlx::PgPool;
use time::Duration;

#[utoipa::path(
    post,
    path = "/auth",
    context_path = "/api/v1/classrooms",
    request_body = ClassroomUserLoginData,
    responses(
        (status = 200, description = "Succesfully logged in or incorrect join code or missing user number", body = ClassroomLoginResponse),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("classroom_sessions" = AllClassroomClaims, Cookie)
    )
)]
/// Logs a user in to a classroom. The user in the front end fills in a join code and a username.
/// If the username is just a number, treat it as a returning user. Otherwise, treat it as a new user.
/// This function checks the validity of the join-code and then either creates or logs in a user.
/// Then, user info and session cookies are created, so the user doesn't have to log in every time.
///
/// # Arguments
/// `jar` - The collection of cookies of the user.
///
/// `all_classroom_claims` - Retrieves the collection of classroom sessions the user already has, if they exist.
///
/// `classroom_login_data` - The credentials provided to log in, as [`ClassroomUserLoginData`].
/// Contains the join code and a username.
///
/// # Returns
/// The provided [`CookieJar`] wherein a `session` and `user_info` cookie has been added (if login successful).
///
/// A [`ClassroomLoginResponse`] which, if successful, returns the classroom id, the user number and the user name.
/// (login successful if join-code correct and user number exists or new user created).
/// Otherwise, returns an error that is handleable on the frontend.
///
/// # Errors
/// May return one of these errors:
/// - [`DatabaseException`](ApiError::DatabaseException)
/// - [`TokenCreationException`](ApiError::TokenCreationException)
/// - [`SerdeException`](ApiError::SerdeException)
pub(super) async fn classroom_login(
    jar: CookieJar,
    all_classroom_claims: Option<AllClassroomClaims>,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(classroom_login_data): Json<ClassroomUserLoginData>,
) -> Result<(CookieJar, Json<ClassroomLoginResponse>), ApiError> {
    // Validate the join code. Also checks if the connected classroom is joinable.
    let Some(classroom_id) = validate_join_code(&db, &classroom_login_data.join_code).await? else {
        return Ok((
            jar,
            Json(ClassroomLoginResponse::JoinCodeIncorrect {
                join_code: classroom_login_data.join_code,
            }),
        ));
    };

    // Create a (num, name) pair either by creating a new user number or
    // retrieving the name connected to the existing user number.
    let (num, name): (i32, String) = match classroom_login_data.username.trim().parse::<i32>() {
        // Username is num.
        Ok(num) => (num, {
            let Some(name) = get_name_by_number(&db, classroom_id, num).await? else {
                return Ok((
                    jar,
                    Json(ClassroomLoginResponse::MissingUser { user_number: num }),
                ));
            };
            name
        }),
        // Username is name.
        Err(_) => (
            add_new_user_to_classroom(
                get_connection!(db),
                classroom_id,
                classroom_login_data.username.trim(),
            )
            .await?,
            classroom_login_data.username.trim().to_owned(),
        ),
    };

    // Retrieve the config and session duration
    let config = &state.config;
    let session_duration = config.session_duration;

    let new_classroom_claims = ClassroomClaims {
        id: classroom_id,
        num,
        name: name.clone(),
    };

    // Get the updated or created classroom claims
    let (all_classroom_claims, token) = get_classroom_claims_create_session(
        &jar,
        &db,
        all_classroom_claims,
        new_classroom_claims.clone(),
        session_duration,
    )
    .await?;

    // Create the user info
    let user_info = serde_json::to_string(&all_classroom_claims)?;

    // Age of a cookie (same for the two cookies)
    let time = Duration::seconds(config.session_duration);

    // Create classroom_sessions cookie.
    let cookie = Cookie::build((AllClassroomClaims::get_cookie_name(), token))
        .path("/api")
        .secure(true)
        .max_age(time)
        .http_only(true);

    // Create classroom_user_infos cookie
    let user_cookie = Cookie::build(("classroom_user_infos", user_info))
        .path("/")
        .secure(true)
        .max_age(time)
        .http_only(false);

    // Put both cookies in the jar and return it.
    // This also overwrites any cookies with the same name already in the jar.
    Ok((
        jar.add(cookie).add(user_cookie),
        Json(ClassroomLoginResponse::Success(new_classroom_claims)),
    ))
}

/// This function is to be used only in the `classroom_login` function.
/// It checks if there are already valid active classroom sessions for the user.
/// If they exist, then add or replace a new session for the user, utilizing the same token.
/// Else, create a new token and add the new session to it.
async fn get_classroom_claims_create_session(
    jar: &CookieJar,
    db: &PgPool,
    all_classroom_claims: Option<AllClassroomClaims>,
    classroom_claims: ClassroomClaims,
    session_duration: i64,
) -> Result<(AllClassroomClaims, String), ApiError> {
    if let Some(mut all_classroom_claims) = all_classroom_claims {
        let token = AllClassroomClaims::extract_token(jar)?;
        let expires_at = Utc::now() + chrono::Duration::seconds(session_duration);

        ClassroomClaims::create_session(
            get_connection!(db),
            &token,
            expires_at,
            &(classroom_claims.id, classroom_claims.num),
        )
        .await?;

        all_classroom_claims
            .claims
            .retain(|claims| claims.id != classroom_claims.id);
        all_classroom_claims.claims.push(classroom_claims);

        Ok((all_classroom_claims, token))
    } else {
        // Create the authorization token.
        let (token, expires_at) = create_token(session_duration);
        let all_classroom_claims = AllClassroomClaims {
            claims: vec![classroom_claims],
        };

        AllClassroomClaims::create_session(
            get_connection!(db),
            &token,
            expires_at,
            &all_classroom_claims.claims,
        )
        .await?;

        Ok((all_classroom_claims, token))
    }
}

#[utoipa::path(
    get,
    path = "/student/list",
    context_path = "/api/v1/classrooms",
    responses(
        (status = 200, description = "Successfully retrieved all classrooms.", body = ClassroomListResponse),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = AllClassroomClaims, Cookie),
        ("page" = OptionalPaginationOptions, Query),
    )
)]
/// Get the list of all classrooms a student has access to.
///
/// # Arguments
/// `all_classroom_claims` - The session cookie of the user.
///
/// `page` - The requested page, given as [`OptionalPaginationOptions`] through a query.
///
/// # Returns
/// All the user's classrooms and the number of classrooms as [`ClassroomListResponse`].
///
/// # Errors
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if the limit on the database memory disallows the classroom count.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub(super) async fn get_all_classrooms_as_student(
    all_classroom_claims: AllClassroomClaims,
    Extension(db): Extension<PgPool>,
    Query(page): Query<OptionalPaginationOptions>,
) -> Result<Json<ClassroomListResponse>, ApiError> {
    let page = page.to_some_with_default(1, 50, 1000, 100)?;

    let mut transaction = db.begin().await?;

    let ids = all_classroom_claims
        .claims
        .into_iter()
        .map(|c| c.id)
        .collect();

    let classroom_count = get_classroom_count_from_ids(&mut *transaction, &ids).await?;
    if classroom_count == 0 {
        return Ok(Json(ClassroomListResponse {
            total_classrooms: 0,
            classrooms: Vec::new(),
        }));
    }

    let classrooms = get_all_classrooms_from_ids(&mut *transaction, &ids, &page).await?;
    transaction.commit().await?;
    let list = ClassroomListResponse {
        total_classrooms: i32::try_from(classroom_count)
            .map_err(|_| ApiError::DatabaseMemoryCap)?,
        classrooms,
    };

    Ok(Json(list))
}

#[utoipa::path(
    get,
    path = "/{classroom_id}/student/dashboard",
    context_path = "/api/v1/classrooms",
    responses(
        (status = 200, description = "Successfully retrieved classroom data.", body = ClassroomStudentDashboardData),
        (status = 400, description = "Classroom with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = ClassroomClaims, Cookie),
        ("classroom_id" = i32, Path),
    )
)]
/// Retrieves all information necessary to construct the classroom student dashboard on the frontend.
///
/// # Arguments
/// `_claims` - The student classroom session, to make sure the student has access to this classroom.
///
/// `classroom_id` - The classroom id retrieved from the path.
///
/// # Returns
/// Returns a [`ClassroomStudentDashboardData`] if successful.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// Returns [`InvalidSession`](ApiError::InvalidSession) if the session cannot be retrieved.
///
/// Returns [`ClassroomIdRetrievalError`](ApiError::ClassroomIdRetrievalError) if the
/// classroom id could not be retrieved from the api uri.
///
/// May also return a [`DatabaseException`](ApiError::DatabaseException).
pub(super) async fn get_student_dashboard(
    _claims: ClassroomClaims,
    Extension(db): Extension<PgPool>,
    Path(classroom_id): Path<i32>,
) -> Result<Json<ClassroomStudentDashboardData>, ApiError> {
    let metadata = get_classroom_metadata_from_id(&db, classroom_id).await?;
    let lessons =
        get_lessons_with_tree_ids_from_classroom(get_connection!(db), classroom_id, None).await?;

    Ok(Json(ClassroomStudentDashboardData { metadata, lessons }))
}
