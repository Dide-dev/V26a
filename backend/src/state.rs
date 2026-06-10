/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::config::Config;
use crate::db_structs::users::PendingRegisterData;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// This needs to be a wrapper type, so that the `api_v1/claims` can
/// implement `FromRequestParts<ArcState>` for the different optional claims.
/// It is also useful for if we want to change this to not put the State in an arc,
/// or if we want to rename it.
///
/// The [`ArcState::clone`] implementation will always be 'cheap'.
#[derive(Clone)]
pub struct ArcState(Arc<State>);

// This is for allowing easy access to the fields of `State` through an `ArcState`.
impl std::ops::Deref for ArcState {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// This struct is meant to be wrapped in a [`Arc`],
/// so it can be cheaply copied.
/// Use the [`ArcState`] type for this.
pub struct State {
    pub config: Config,
    pub pending_accounts: Mutex<HashMap<String, PendingRegisterData>>,
}

impl ArcState {
    pub fn new(config: Config) -> ArcState {
        let pending_accounts = Mutex::default();

        ArcState(Arc::new(State {
            config,
            pending_accounts,
        }))
    }
}
