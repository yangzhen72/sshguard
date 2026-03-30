use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::{AIClient, AIError, ChatCompletionRequest, Message, Result};

pub struct MiniMaxClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl MiniMaxClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.minimax.chat/v1".to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct MiniMaxRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct MiniMaxResponse {
    choices: Vec<MiniMaxChoice>,
}

#[derive(Debug, Deserialize)]
struct MiniMaxChoice {
    messages: Vec<MiniMaxMessageContent>,
}

#[derive(Debug, Deserialize)]
struct MiniMaxMessageContent {
    text: String,
}

#[async_trait]
impl AIClient for MiniMaxClient {
    async fn chat(&self, request: ChatCompletionRequest) -> Result<String> {
        let response = self.client
            .post(&format!("{}/text/chatcompletion_v2", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&MiniMaxRequest {
                model: request.model,
                messages: request.messages,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::Api(response.text().await?));
        }

        let resp: MiniMaxResponse = response.json().await?;
        Ok(resp.choices.first()
            .and_then(|c| c.messages.first())
            .map(|m| m.text.clone())
            .unwrap_or_default())
    }

    async fn chat_streaming(&self, _request: ChatCompletionRequest) -> Result<Vec<String>> {
        Err(AIError::Api("Streaming not implemented".to_string()))
    }
}