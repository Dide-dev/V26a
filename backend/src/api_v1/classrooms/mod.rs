/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ratelimits::{
    ADD_LESSON_RATELIMIT, CLASSROOM_LOGIN_RATELIMIT, CREATE_CLASSROOM_RATELIMIT,
};
use crate::state::ArcState;
use axum::Router;
use axum::routing::{get, post};

pub mod students;
pub mod teachers;

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn classrooms() -> Router<ArcState> {
    Router::new()
        .route(
            "/",
            post(teachers::add_new_classroom).layer(CREATE_CLASSROOM_RATELIMIT.to_layer()),
        )
        .route(
            "/{classroom_id}/lessons",
            post(teachers::add_new_lesson).layer(ADD_LESSON_RATELIMIT.to_layer()),
        )
        .route(
            "/{classroom_id}/student/dashboard",
            get(students::get_student_dashboard),
        )
        .route(
            "/{classroom_id}/teacher/dashboard",
            get(teachers::get_teacher_dashboard),
        )
        .route(
            "/auth",
            post(students::classroom_login).layer(CLASSROOM_LOGIN_RATELIMIT.to_layer()),
        )
        .route(
            "/student/list",
            get(students::get_all_classrooms_as_student),
        )
        .route(
            "/teacher/list",
            get(teachers::get_all_classrooms_as_teacher),
        )
}
