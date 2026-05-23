use super::model::Message;

pub trait SlackClient {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn fetch_messages(
        &self,
        channel_id: &str,
        oldest_ts: i64,
    ) -> Result<Vec<Message>, Self::Error>;
    async fn post_message(&self, channel_id: &str, text: &str) -> Result<(), Self::Error>;
}
