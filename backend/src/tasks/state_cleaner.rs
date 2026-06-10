/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::users::PendingAccountsExt;
use crate::state::ArcState;
use std::time::Duration;
use tokio::time;
use tracing::log::info;

pub async fn task_handler(arc_state: ArcState) {
    let mut interval = time::interval(Duration::from_hours(1));
    interval.tick().await;

    loop {
        interval.tick().await;

        prune_unverified_accounts(arc_state.clone()).await;
    }
}

pub async fn prune_unverified_accounts(arc_state: ArcState) {
    let mut pending_accounts = arc_state.pending_accounts.lock().await;
    pending_accounts.remove_expired();
    info!("Automatically pruning unverified accounts.");
}
