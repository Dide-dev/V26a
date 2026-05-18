/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use axum::{
    Extension,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::oneshot::Receiver;

use crate::{
    db_structs::session::Claims,
    errors::ApiError,
    state::{ArcState, TreeEditLock},
};

/// Requires to be wrapped in an [`axum::routing::any`],
/// see the docs of [`WebSocketUpgrade`] for why.
pub async fn aquire_edit_lock(
    ws: WebSocketUpgrade,
    claims: Claims,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Path(tree_id): Path<i32>,
) -> Result<Response, ApiError> {
    let user_id = claims.sub;
    super::verify_tree_permissions(crate::get_connection!(db), tree_id, claims).await?;

    Ok(ws.on_upgrade(move |socket| handle_socket_without_lock(socket, state, tree_id, user_id)))
}

#[derive(Deserialize, Serialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum FrontendMessage {
    Force,
    SetActive { new_active_state: bool },
}

#[derive(Serialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum BackendMessage {
    LockAchieved,
    LockFailed { active_user: i32 },
    KickedOutBy { user_id: i32 },
}

impl FrontendMessage {
    /// Returns [`None`] if the message could not be parsed,
    /// or if the socket closed or had an error.
    ///
    /// This function *is* cancellation safe.
    pub async fn try_receive(socket: &mut WebSocket) -> Option<Self> {
        let response = socket.recv().await?;

        if let Ok(message) = response
            && let Message::Text(txt) = message
            && let Ok(parsed) = serde_json::from_str(txt.as_str())
        {
            Some(parsed)
        } else {
            None
        }
    }
}

impl BackendMessage {
    pub async fn try_send(&self, socket: &mut WebSocket) {
        let Ok(txt) = serde_json::to_string(self) else {
            // This never happens, but because it is rust we have to handle it somehow
            unreachable!()
        };
        socket.send(Message::Text(txt.into())).await.ok();
    }
}

/// This function is **NOT** cancellation safe,
/// because it calls [`handle_socket_with_lock`]
async fn handle_socket_without_lock(
    mut socket: WebSocket,
    state: ArcState,
    tree_id: i32,
    user_id: i32,
) {
    let (my_sender, my_receiver) = tokio::sync::oneshot::channel();

    let is_active = Arc::new(AtomicBool::new(true));
    let my_tree_lock = TreeEditLock {
        user_id,
        kick_out_sender: my_sender,
        is_active: is_active.clone(),
    };

    // Try lock
    {
        use std::collections::hash_map::Entry;
        let mut tree_edit_locks = state.tree_edit_locks.lock().await;

        match tree_edit_locks.entry(tree_id) {
            Entry::Occupied(mut occupied_entry) => {
                let old_lock = occupied_entry.get();
                if !old_lock.is_active.load(Ordering::Relaxed) {
                    let old_lock = occupied_entry.insert(my_tree_lock);
                    old_lock.kick_out_sender.send(user_id).ok();

                    // We must drop the MutexGuard since the `handle_socket_with_lock` will run for a long time.
                    std::mem::drop(tree_edit_locks);
                    return handle_socket_with_lock(socket, state, is_active, tree_id, my_receiver)
                        .await;
                }

                let message = BackendMessage::LockFailed {
                    active_user: old_lock.user_id,
                };
                message.try_send(&mut socket).await;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(my_tree_lock);

                // We must drop the MutexGuard since the `handle_socket_with_lock` will run for a long time.
                std::mem::drop(tree_edit_locks);
                return handle_socket_with_lock(socket, state, is_active, tree_id, my_receiver)
                    .await;
            }
        }
    }

    while let Some(msg) = FrontendMessage::try_receive(&mut socket).await {
        match msg {
            FrontendMessage::Force => {
                let mut tree_edit_locks = state.tree_edit_locks.lock().await;

                let previous = tree_edit_locks.insert(tree_id, my_tree_lock);
                if let Some(previous) = previous {
                    previous.kick_out_sender.send(user_id).ok();
                    // By using this `if let Some(...)`,
                    // the variable `previous` is automatically dropped.
                }

                std::mem::drop(tree_edit_locks);

                return handle_socket_with_lock(socket, state, is_active, tree_id, my_receiver)
                    .await;
            }
            FrontendMessage::SetActive { new_active_state } => {
                // This would be very unusual, but we will play along
                is_active.store(new_active_state, Ordering::Relaxed);
            }
        }
    }

    // Socket closed without any lock, nothing to do here
}

/// This function is **NOT** cancellation safe,
/// as it will clean up the `state.tree_edit_locks[tree_id]` before returning when done.
async fn handle_socket_with_lock(
    mut socket: WebSocket,
    state: ArcState,
    is_active: Arc<AtomicBool>,
    tree_id: i32,
    mut my_receiver: Receiver<i32>,
) {
    BackendMessage::LockAchieved.try_send(&mut socket).await;

    loop {
        tokio::select! {
            msg = FrontendMessage::try_receive(&mut socket) => {
                let Some(msg) = msg else {
                    // Client disconnected or something else went wrong
                    break;
                };

                match msg {
                    FrontendMessage::Force => {
                        // Nothing to do here, tree is already locked by this user
                    },
                    FrontendMessage::SetActive { new_active_state } => {
                        is_active.store(new_active_state, Ordering::Relaxed);
                    },
                }
            }
            msg = &mut my_receiver => {
                // There should always be a message in the channel before closing it,
                // if not just let the browser treat it as if the server went offline.
                if let Ok(other_user) = msg {
                    let message = BackendMessage::KickedOutBy { user_id: other_user };
                    message.try_send(&mut socket).await;
                }

                // Dropping it closes the connection
                std::mem::drop(socket);
                break;
            }
        }
    }

    // Clean up the state.tree_edit_locks[tree_id], if not already overwritten by somebody else
    if !my_receiver.is_terminated() {
        let mut tree_edit_locks = state.tree_edit_locks.lock().await;

        // We need to check again since the `await` above could have taken some time.
        if !my_receiver.is_terminated() {
            tree_edit_locks.remove(&tree_id);
        }
    }
}
