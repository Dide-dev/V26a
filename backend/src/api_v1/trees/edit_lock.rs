/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

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
use utoipa::ToSchema;

use crate::{
    db_structs::session::Claims,
    errors::ApiError,
    state::{ArcState, TreeEditLock},
};

#[utoipa::path(
    get,
    path = "/edit-lock",
    context_path = "/api/v1/trees/{tree_id}",
    tag="crate::api_v1::trees",
    request_body = FrontendMessage,
    responses(
        (
            status = 101,
            description = "Websocket started. All the messages will be serialized as json into text. See the examples for the types of messages that can be sent.",
            body = BackendMessage,
        ),
        (status = 400, description = "Connection header did not include 'upgrade' or tree_id could not be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// If the user has permissions for the given `tree_id`,
/// opens a websocket which can be used to acquire the lock.
///
/// If the websocket ever closes, then the lock is considered to be lost.
///
/// Note that the request body and responses are not the actual http GET request and response bodies,
/// but they are there for linking to the messages that can be sent over the websocket.
///
/// # Arguments
/// `claims` - The claims of the user who sent this request.
///
/// `state` - State containing the `tree_edit_locks`.
///
/// `tree_id` - The tree of which the edit lock gets acquired.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission
/// for the given `tree_id`.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with `tree_id` exists.
///
pub async fn acquire_edit_lock(
    ws: WebSocketUpgrade,
    claims: Claims,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Path(tree_id): Path<i32>,
) -> Result<Response, ApiError> {
    let user_id = claims.sub;
    super::verify_tree_permissions(crate::get_connection!(db), tree_id, &claims).await?;

    Ok(ws.on_upgrade(move |socket| handle_socket_without_lock(socket, state, tree_id, user_id)))
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
/// All possible messages that the frontend can send to the backend.
pub enum FrontendMessage {
    /// Will force acquire the lock, kicking out any potential other user holding the lock
    Force,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
/// All possible messages that the backend can send to the frontend.
pub enum BackendMessage {
    /// Lock achieved, the tree is safe to edit.
    LockAchieved,
    /// Could not achive lock, because another user is active.
    LockFailed { active_user: i32 },
    /// Used to have a lock, but another user kicked you out.
    /// The websocket will be closed by the backend after this message was sent.
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
///
/// This function handles a websocket that does not (yet) have a lock.
/// If no one holds the lock, this function will immediately set the lock and call `handle_socket_with_lock`.
///
/// Otherwise, it will wait until the `FrontendMessage::Force` is received.
async fn handle_socket_without_lock(
    mut socket: WebSocket,
    state: ArcState,
    tree_id: i32,
    user_id: i32,
) {
    let (my_sender, my_receiver) = tokio::sync::oneshot::channel();

    let my_tree_lock = TreeEditLock {
        user_id,
        kick_out_sender: my_sender,
    };

    // Try lock
    {
        use std::collections::hash_map::Entry;
        let mut tree_edit_locks = state.tree_edit_locks.lock().await;

        match tree_edit_locks.entry(tree_id) {
            Entry::Occupied(occupied_entry) => {
                // Locking failed because someone else is has it.

                let message = BackendMessage::LockFailed {
                    active_user: occupied_entry.get().user_id,
                };
                message.try_send(&mut socket).await;
            }
            Entry::Vacant(vacant_entry) => {
                // Locking succeeded!
                vacant_entry.insert(my_tree_lock);

                // We must drop the MutexGuard since the `handle_socket_with_lock` will run for a long time.
                drop(tree_edit_locks);
                return handle_socket_with_lock(socket, state, tree_id, my_receiver).await;
            }
        }
    }

    // If other messages are added, this should turn into a loop.
    // Currently the only message would return out of the loop.
    if let Some(msg) = FrontendMessage::try_receive(&mut socket).await {
        match msg {
            FrontendMessage::Force => {
                let mut tree_edit_locks = state.tree_edit_locks.lock().await;

                let previous = tree_edit_locks.insert(tree_id, my_tree_lock);
                if let Some(previous) = previous {
                    previous.kick_out_sender.send(user_id).ok();
                    // By using this `if let Some(...)`,
                    // the variable `previous` is automatically dropped.
                }

                // We must drop the MutexGuard since the `handle_socket_with_lock` will run for a long time.
                drop(tree_edit_locks);

                // The function should return with this the result of this function,
                // but currently there would be a clippy warning since this is not a loop.
                handle_socket_with_lock(socket, state, tree_id, my_receiver).await;
            }
        }
    }

    // Socket closed without any lock, nothing to do here
}

/// This function is **NOT** cancellation safe,
/// as it will clean up the `state.tree_edit_locks[tree_id]` before returning when done.
///
/// This function handles a websocket that already has the lock.
/// So it will ignore the [`FrontendMessage::Force`] message and will update
async fn handle_socket_with_lock(
    mut socket: WebSocket,
    state: ArcState,
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
                drop(socket);
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
