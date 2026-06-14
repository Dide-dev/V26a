/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::state::ArcState;
use axum::Router;
use axum::routing::{get, post};

pub mod students;
pub mod teachers;

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn classrooms() -> Router<ArcState> {
    Router::new()
        .route("/", post(teachers::add_new_classroom))
        .route("/{classroom_id}/lessons", post(teachers::add_new_lesson))
        .route(
            "/{classroom_id}/dashboard-student",
            get(students::get_student_dashboard),
        )
        .route(
            "/{classroom_id}/dashboard-teacher",
            get(teachers::get_teacher_dashboard),
        )
        .route("/login", post(students::classroom_login))
        .route(
            "/list-student",
            get(students::get_all_classrooms_as_student),
        )
        .route(
            "/list-teacher",
            get(teachers::get_all_classrooms_as_teacher),
        )
}
