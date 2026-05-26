use thiserror::Error;

#[derive(Debug, Error)]
pub enum SlackError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("conversations.history error: {0}")]
    ConversationsHistory(String),
    #[error("chat.postMessage error: {0}")]
    ChatPostMessage(String),
}
