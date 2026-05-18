/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, oneshot};

use crate::config::Config;

/// This needs to be a wrapper type, so that the `api_v1/session.rs` can
/// implement `FromRequestParts<ArcState>` for `Option<Claims>`.
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
        self.0.deref()
    }
}

pub struct TreeEditLock {
    /// Id of user currently holding the lock.
    pub user_id: i32,

    /// Send a message here to kick out the user.
    /// If this struct is dropped, the user will also be kicked.
    /// The content of the message is the user who kicked you out.
    pub kick_out_sender: oneshot::Sender<i32>,

    /// Indicates if the user is tabbed out or not.
    /// Is an arc atomic bool, so that changing this value doesn't need to lock the entire HashMap.
    pub is_active: Arc<std::sync::atomic::AtomicBool>,
}

/// This struct is meant to be wrapped in a [`Arc`],
/// so it can be cheaply copied.
/// Use the [`ArcState`] type for this.
pub struct State {
    pub config: Config,

    pub tree_edit_locks: Mutex<HashMap<i32, TreeEditLock>>,
    // Later, we will add fields for
    // sessions, email validation.
}

impl ArcState {
    pub fn new(config: Config) -> ArcState {
        ArcState(Arc::new(State {
            config,
            tree_edit_locks: Mutex::default(),
        }))
    }
}
