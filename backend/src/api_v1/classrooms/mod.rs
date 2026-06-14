/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use axum::Router;
use axum::routing::{get, post};
use crate::state::ArcState;

pub mod teachers;
pub mod students;
pub mod lessons;

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn classrooms() -> Router<ArcState> {
    Router::new()
        .route("/", post(teachers::add_new_classroom))
        .route("/{classroom_id}/lessons", post(teachers::add_new_lesson))
        .route("/{classroom_id}/student-dash", get(students::get_student_dashboard))
        .route("/{classroom_id}/teacher-dash", get(teachers::get_teacher_dashboard))
        .route("/login", post(students::classroom_login))
}
