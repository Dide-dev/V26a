/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::state::ArcState;
use chrono::Utc;
use std::time::Duration;
use tokio::time;
use tracing::log::info;

pub async fn task_handler(arc_state: ArcState) {
    let mut interval = time::interval(Duration::from_hours(1));
    interval.tick().await;

    loop {
        interval.tick().await;

        {
            let mut pending_accounts = arc_state.pending_accounts.lock().await;
            let before_count = pending_accounts.len();
            pending_accounts.retain(|_, p| p.expires < Utc::now().timestamp());
            let difference = before_count - pending_accounts.len();
            info!("Pruning unverified accounts: {difference} pending accounts removed.");
        }
    }
}
