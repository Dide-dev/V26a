/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db::classrooms::{
    add_classroom, add_lesson, add_new_user_to_classroom, generate_random_code, get_classroom_owner,
    get_name_by_number, validate_join_code,
};
use crate::db_structs::Validated;
use crate::db_structs::classrooms::{
    ClassroomData, ClassroomGeneratedData, ClassroomLoginResponse, ClassroomUserLoginData,
    LessonAddData,
};
use crate::db_structs::session::{AllClassroomClaims, Claims, ClaimsType, ClassroomClaims};
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::{ArcState, get_connection};

use axum::extract::{Path, State};
use axum::routing::post;
use axum::{Extension, Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use chrono::Utc;
use jsonwebtoken::{Header, Validation, decode, encode};
use sqlx::{PgConnection, PgPool};

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn classrooms() -> Router<ArcState> {
    Router::new()
        .route("/", post(add_new_classroom))
        .route("/{classroom_id}/lessons", post(add_new_lesson))
        .route("/login", post(classroom_login))
}

#[utoipa::path(
    post,
    path = "",
    context_path = "/api/v1/classrooms",
    request_body = ClassroomData,
    responses(
        (status = 200, description = "Successfully added new classroom.", body = ClassroomGeneratedData),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie)
    )
)]
/// Adds a classroom to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `classroom_add_data` - The data used to create the classroom, as [`ClassroomData`].
///
/// # Returns
/// The id assigned to the new classroom and the generated join code in a [`ClassroomGeneratedData`].
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_classroom(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(classroom_add_data): Json<ClassroomData>,
) -> Result<Json<ClassroomGeneratedData>, ApiError> {
    classroom_add_data.validate_inputs()?;
    let join_code = generate_random_code(get_connection!(db)).await?;

    let (id, created_at) = add_classroom(&db, &classroom_add_data, claims.sub, &join_code).await?;

    Ok(Json(ClassroomGeneratedData {
        id,
        join_code,
        created_at,
        owner_id: claims.sub,
    }))
}

#[utoipa::path(
    post,
    path = "/login",
    context_path = "/api/v1/classrooms",
    request_body = ClassroomUserLoginData,
    responses(
        (status = 200, description = "Successfully logged in", body = ClassroomLoginResponse),
        (status = 400, description = "The join code is not connected to a joinable classroom."),
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
/// This API-function should be the only function that deals with the entire `classroom_sessions` cookie.
/// All other functions will contain /classrooms/{id}, so those can use `ClassroomClaims` where
/// only the information for that classroom is provided.
///
/// # Arguments
/// `jar` - The collection of cookies of the user.
///
/// `classroom_login_data` - The credentials provided to log in, as [`ClassroomUserLoginData`].
/// Contains the join code and a username.
///
/// # Returns
/// The provided [`CookieJar`] wherein a `session` and `user_info` cookie has been added (if login successful).
///
/// A [`ClassroomLoginResponse`] which returns the classroom id, the user number and the user name.
/// (login successful if join-code correct and user number exists or new user created)
///
/// # Errors
/// May return one of these errors:
/// - [`DatabaseException`](ApiError::DatabaseException)
/// - [`WrongClassroomJoinCode`](ApiError::WrongClassroomJoinCode)
/// - [`TokenCreationException`](ApiError::TokenCreationException)
/// - [`SerdeException`](ApiError::SerdeException)
async fn classroom_login(
    jar: CookieJar,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(classroom_login_data): Json<ClassroomUserLoginData>,
) -> Result<(CookieJar, Json<ClassroomLoginResponse>), ApiError> {
    // Validate the join code. Also checks if the connected classroom is joinable.
    let classroom_id = validate_join_code(&db, &classroom_login_data.join_code).await?;

    // Create a (num, name) pair either by creating a new user number or
    // retrieving the name connected to the existing user number.
    let (num, name): (i32, String) = match classroom_login_data.username.trim().parse::<i32>() {
        // Username is num.
        Ok(num) => (num, {
            let name = get_name_by_number(&db, classroom_id, num).await?;
            name.ok_or(ApiError::MissingUser(num))?
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

    // Retrieve the config.
    let config = &state.config;

    // Define the classroom claims variable to put in the cookie.
    let all_classroom_claims = get_classroom_claims(&jar, &state, classroom_id, num, &name);

    // Create the authorization token.
    let token = encode(
        &Header::default(),
        &all_classroom_claims,
        &config.jwt_keys.encoding,
    )
    .map_err(|_| ApiError::TokenCreationException)?;

    // Create the user info
    let user_info = serde_json::to_string(&all_classroom_claims)?;

    // Age of a cookie (same for the two cookies)
    let time = time::Duration::new(config.session_duration, 0);

    // Create classroom_sessions cookie. Multiple sessions can be put into this one cookie,
    // however, only one session is allowed per classroom.
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
        Json(ClassroomLoginResponse {
            id: classroom_id,
            num,
            name,
        }),
    ))
}

/// This function is to be used only in the `classroom_login` function.
/// As opposed to `FromRequestParts` of `ClassroomClaims`, which retrieves only the appropriate
/// classroom session, this function extracts all active classroom sessions from the `CookieJar`.
/// This function also adds a new session to the collection of active sessions, after performing
/// operations on the collection of sessions to make sure the resulting collection of active sessions is valid.
fn get_classroom_claims(
    jar: &CookieJar,
    state: &ArcState,
    classroom_id: i32,
    num: i32,
    name: &str,
) -> AllClassroomClaims {
    let config = &state.config;
    let all_sessions = jar.get(AllClassroomClaims::get_cookie_name());

    let new_classroom_claims = ClassroomClaims {
        id: classroom_id,
        num,
        name: name.to_owned(),
        exp: Utc::now().timestamp() + config.session_duration,
    };
    let default_all_classroom_claims = AllClassroomClaims {
        sessions: vec![new_classroom_claims.clone()],
        exp: Utc::now().timestamp() + config.session_duration,
    };

    // If cookie doesn't exist, set classroom claims as default value.
    let Some(all_sessions) = all_sessions else {
        println!("Could not find cookie");
        return default_all_classroom_claims;
    };

    // Get the token data from the cookie.
    let token_data = decode::<AllClassroomClaims>(
        all_sessions.value(),
        &state.config.jwt_keys.decoding,
        &Validation::default(),
    )
    .ok();

    // If token data is not retrievable from the cookie, set classroom claims as default value.
    let Some(token_data) = token_data else {
        println!("Could not decode");
        return default_all_classroom_claims;
    };

    let mut claims = token_data.claims;

    // Filter out expired sessions
    claims.sessions.retain(|s| s.exp > Utc::now().timestamp());

    let existing_session_index = claims.sessions.iter().position(|s| s.id == classroom_id);
    if let Some(existing_session_index) = existing_session_index {
        claims.sessions[existing_session_index] = new_classroom_claims.clone();
    } else {
        // Sort the sessions on expiration date and take only the 9 latest.
        claims.sessions.sort_by_key(|s| -s.exp);
        claims
            .sessions
            .truncate(config.max_concurrent_classroom_sessions - 1);

        // Push the new session.
        claims.sessions.push(new_classroom_claims);
    }

    // Return the classroom claims variable and reset the exp.
    claims.exp = Utc::now().timestamp() + config.session_duration;
    claims
}

#[utoipa::path(
    post,
    path = "/{classroom_id}/lessons",
    context_path = "/api/v1/classrooms",
    request_body = LessonAddData,
    responses(
        (status = 200, description = "Successfully added new lesson."),
        (status = 400, description = "Classroom with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("classroom_id" = i32, Path),
    )
)]
/// Adds a lesson to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `lesson_add_data` - The data for the lesson in the form of [`LessonAddData`].
///
/// `classroom_id` - The id of the classroom to which the lesson needs to belong.
///
/// # Returns
/// The id assigned to the new lesson.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// Returns [`MissingClassroom`](ApiError::MissingClassroom) if the classroom to which the lesson should be added does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have the permission to add lessons to the classroom.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_lesson(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(classroom_id): Path<i32>,
    Json(lesson_add_data): Json<LessonAddData>,
) -> Result<Json<i32>, ApiError> {
    lesson_add_data.validate_inputs()?;
    // Check if the classroom exists and the user is permitted to perform this action.
    verify_classroom_permissions(get_connection!(db), classroom_id, claims).await?;
    let id = add_lesson(get_connection!(db), &lesson_add_data, classroom_id).await?;

    Ok(Json(id))
}

/// Checks whether the classroom exists and if it is owned by the user, or if the user is an admin.
/// As of now this function does not take into account eventual collaborators as this is currently not implemented for classrooms,
/// this function would need to be updated if this were the case in the future.
///
/// # Arguments
/// `classroom_id` - The id of the classroom whose ownership needs to be verified.
///
/// `claims` - The session cookie of the user, used to verify ownership.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to modify the classroom for the given `classroom_id`.
///
/// Returns [`MissingClassroom`](ApiError::MissingClassroom) if there is no classroom with `classroom_id`.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn verify_classroom_permissions(
    db: &mut PgConnection,
    classroom_id: i32,
    claims: Claims,
) -> Result<(), ApiError> {
    let owner_id = get_classroom_owner(&mut *db, classroom_id)
        .await?
        .ok_or(ApiError::MissingClassroom(classroom_id))?;
    if claims.account_type == AccountType::Admin || claims.sub == owner_id {
        Ok(())
    } else {
        Err(ApiError::NoPermission)
    }
}
