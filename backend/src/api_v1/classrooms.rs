/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db::classrooms::{
    add_classroom, add_lesson, add_new_user_to_classroom, generate_random_code,
    get_classroom_owner, get_name_by_number, validate_join_code,
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

use crate::api_v1::create_token;
use axum::extract::{Path, State};
use axum::routing::post;
use axum::{Extension, Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use chrono::Utc;
use sqlx::{PgConnection, PgPool};
use time::Duration;

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
async fn classroom_login(
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
            Json(ClassroomLoginResponse::JoinCodeIncorrect(classroom_login_data.join_code)),
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
                    Json(ClassroomLoginResponse::MissingUser(num)),
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
    // Check if the classroom exists and the user is permitted to manage it.
    verify_classroom_permissions(get_connection!(db), classroom_id, &claims).await?;
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
    claims: &Claims,
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
