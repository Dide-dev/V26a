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

        prune_unverified_accounts(arc_state.clone()).await;
    }
}

pub async fn prune_unverified_accounts(arc_state: ArcState) {
    let mut pending_accounts = arc_state.pending_accounts.lock().await;
    let before_count = pending_accounts.len();
    pending_accounts.retain(|_, p| p.expires > Utc::now().timestamp());
    let difference = before_count - pending_accounts.len();
    info!("Pruning unverified accounts: {difference} pending accounts removed.");
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::db_structs::users::{PendingRegisterData, RegisterData};
    use crate::state::ArcState;
    use crate::tasks::state_cleaner::prune_unverified_accounts;
    use chrono::Utc;

    #[tokio::test]
    async fn test_prune_unverified_accounts() {
        dotenvy::dotenv().unwrap();
        let config = Config::load();
        let state = ArcState::new(config);
        let register_data = RegisterData::test_values();

        {
            let mut pending_accounts = state.pending_accounts.lock().await;
            pending_accounts.insert(
                "a".to_owned(),
                PendingRegisterData {
                    register_data: register_data.clone(),
                    expires: 0,
                },
            );
            pending_accounts.insert(
                "b".to_owned(),
                PendingRegisterData {
                    register_data: register_data.clone(),
                    expires: Utc::now().timestamp() + 60,
                },
            );
        }

        prune_unverified_accounts(state.clone()).await;

        let pending_accounts = state.pending_accounts.lock().await;
        assert!(!pending_accounts.contains_key("a"));
        assert!(pending_accounts.contains_key("b"));
    }
}
