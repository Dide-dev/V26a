/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::errors::ApiError;

pub mod auth;
pub mod cards;
pub mod classrooms;
pub mod feedback;
pub mod mail;
pub mod pagination;
pub mod session;
pub mod trees;
pub mod users;

/// A trait that can be implemented on structs that have inputs that need to be validated.
pub trait Validated {
    /// Validates the inputs for a struct, returning [`Ok`] if valid, or an [`ApiError`] if not.
    fn validate_inputs(&self) -> Result<(), ApiError>;
}
