use super::error::SlackError;
use super::model::{ChatPostMessageRequest, ChatPostMessageResponse, ConversationsHistoryResponse};
use crate::domain::model::Message;
use crate::service::port::SlackClient;
use reqwest::Client as HttpClient;

#[derive(Clone)]
pub struct Client {
    http: HttpClient,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            http: HttpClient::new(),
            token,
        }
    }
}

impl SlackClient for Client {
    type Error = SlackError;

    async fn fetch_history(
        &self,
        channel_id: &str,
        oldest_ts: i64,
    ) -> Result<Vec<Message>, SlackError> {
        let mut messages = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let mut query = vec![
                ("channel", channel_id.to_string()),
                ("limit", "999".to_string()),
                ("oldest", oldest_ts.to_string()),
            ];
            if let Some(c) = &cursor {
                query.push(("cursor", c.clone()));
            }

            let res: ConversationsHistoryResponse = self
                .http
                .get("https://slack.com/api/conversations.history")
                .bearer_auth(&self.token)
                .query(&query)
                .send()
                .await?
                .json()
                .await?;
            if !res.ok {
                return Err(SlackError::ConversationsHistory(res.error.unwrap()));
            }

            messages.extend(res.messages.unwrap().into_iter().map(|m| Message {
                subtype: m.subtype,
                user: m.user,
            }));

            cursor = res
                .response_metadata
                .and_then(|m| m.next_cursor)
                .filter(|c| !c.is_empty());
            if cursor.is_none() {
                break;
            }
        }

        Ok(messages)
    }

    async fn send_message(&self, channel_id: &str, text: &str) -> Result<(), SlackError> {
        let body = ChatPostMessageRequest {
            channel: channel_id,
            text: Some(text.to_string()),
        };

        let res: ChatPostMessageResponse = self
            .http
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        if !res.ok {
            return Err(SlackError::ChatPostMessage(res.error.unwrap()));
        }

        Ok(())
    }
}
