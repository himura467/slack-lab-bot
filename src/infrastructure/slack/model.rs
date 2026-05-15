use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Message {
    pub subtype: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize)]
pub struct ResponseMetadata {
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct ConversationsHistoryResponse {
    pub ok: bool,
    pub messages: Option<Vec<Message>>,
    pub response_metadata: Option<ResponseMetadata>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ChatPostMessageRequest<'a> {
    pub channel: &'a str,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct ChatPostMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
}
