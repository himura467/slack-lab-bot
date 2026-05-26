use super::port::SlackClient;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActivityServiceError {
    #[error("failed to fetch channel history: {0}")]
    FetchHistory(String),
    #[error("failed to send reminder: {0}")]
    SendReminder(String),
}

pub trait ActivityService {
    async fn fetch_active_users(
        &self,
        oldest_ts: i64,
    ) -> Result<HashSet<String>, ActivityServiceError>;
    async fn send_reminder(&self, user_ids: &[String]) -> Result<(), ActivityServiceError>;
}

pub struct Service<C> {
    slack: C,
    channel_id: String,
}

impl<C: SlackClient> Service<C> {
    pub fn new(slack: C, channel_id: String) -> Self {
        Self { slack, channel_id }
    }
}

impl<C: SlackClient> ActivityService for Service<C> {
    async fn fetch_active_users(
        &self,
        oldest_ts: i64,
    ) -> Result<HashSet<String>, ActivityServiceError> {
        let messages = self
            .slack
            .fetch_history(&self.channel_id, oldest_ts)
            .await
            .map_err(|e| ActivityServiceError::FetchHistory(e.to_string()))?;

        Ok(messages
            .into_iter()
            .filter(|m| m.subtype.is_none())
            .filter_map(|m| m.user)
            .collect())
    }

    async fn send_reminder(&self, user_ids: &[String]) -> Result<(), ActivityServiceError> {
        if user_ids.is_empty() {
            return Ok(());
        }
        let mentions = user_ids
            .iter()
            .map(|id| format!("<@{id}>"))
            .collect::<Vec<_>>()
            .join(" ");
        let text =
            format!("{mentions}\n週報を提出してください！\nPlease submit your weekly report!");
        self.slack
            .send_message(&self.channel_id, &text)
            .await
            .map_err(|e| ActivityServiceError::SendReminder(e.to_string()))
    }
}
