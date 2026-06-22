/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::config::Config;
use crate::structs::users::{PasswordResetRequestData, PendingRegisterData};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

/// This needs to be a wrapper type, so that the `api_v1/session` can
/// implement `FromRequestParts<ArcState>` for the different optional sessions.
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

pub struct TreeEditLock {
    /// Id of user currently holding the lock.
    pub user_id: i32,

    /// Send a message here to kick out the user.
    /// If this struct is dropped, the user will also be kicked.
    /// The content of the message is the user who kicked you out.
    pub kick_out_sender: oneshot::Sender<i32>,
}

/// This struct is meant to be wrapped in a [`Arc`],
/// so it can be cheaply copied.
/// Use the [`ArcState`] type for this.
pub struct State {
    pub config: Config,
    pub tree_edit_locks: Mutex<HashMap<i32, TreeEditLock>>,
    pub pending_accounts: Mutex<HashMap<String, PendingRegisterData>>,
    pub reset_password_requests: Mutex<HashMap<String, PasswordResetRequestData>>,
}

impl ArcState {
    pub fn new(config: Config) -> ArcState {
        ArcState(Arc::new(State {
            config,
            tree_edit_locks: Mutex::default(),
            pending_accounts: Mutex::default(),
            reset_password_requests: Mutex::default(),
        }))
    }
}
