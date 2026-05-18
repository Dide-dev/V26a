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
use sqlx::PgPool;

use crate::{db_structs::session::Claims, errors::ApiError, state::ArcState};

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

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, tree_id)))
}

/// This function is **NOT** cancellation safe,
/// as it will clean up the `state.tree_edit_locks[tree_id]` before returning when done.
async fn handle_socket(mut socket: WebSocket, state: ArcState, tree_id: i32) {
    let (my_sender, mut my_receiver) = tokio::sync::oneshot::channel();

    {
        let mut tree_edit_locks = state.tree_edit_locks.lock().await;

        // By not using the return result of this function,
        // we ensure that if any previous sender was present, it has been dropped.
        tree_edit_locks.insert(tree_id, my_sender);

        // Immediately drop the lock after the operation is complete
    }

    let mut is_active = true;
    loop {
        tokio::select! {
            msg = socket.recv() => {
                let Some(Ok(msg)) = msg else {
                    // Client disconnected
                    break;
                };

                if let Message::Text(text) = msg && let Ok(new_is_active) = serde_json::from_str(text.as_str()) {
                    is_active = new_is_active;
                    println!("Is active: {is_active}");
                }
                // If the message could not be parsed, then it is
                // probably someone trying to reverse engineer this.
                // Let's just ignore the message to mess with them.
            }
            _msg = &mut my_receiver => {
                // Messages currently always have the meaning to disconnect I think
                break;
            }
        }
    }

    // Clean up the state.tree_edit_locks[tree_id], if not already overwritten by somebody else
    let mut tree_edit_locks = state.tree_edit_locks.lock().await;

    // Ensure that if a different thread has aquired the lock AND the browser disconnected at the same time,
    // we don't remove the sender that was already placed in place of my_sender.
    if !my_receiver.is_terminated() {
        tree_edit_locks.remove(&tree_id);
    }
}
