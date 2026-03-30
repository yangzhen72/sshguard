use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::{AIClient, AIError, ChatCompletionRequest, Message, Result};

pub struct QwenClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl QwenClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1".to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct QwenRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct QwenResponse {
    choices: Vec<QwenChoice>,
}

#[derive(Debug, Deserialize)]
struct QwenChoice {
    message: Message,
}

#[async_trait]
impl AIClient for QwenClient {
    async fn chat(&self, request: ChatCompletionRequest) -> Result<String> {
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&QwenRequest {
                model: request.model,
                messages: request.messages,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::Api(response.text().await?));
        }

        let resp: QwenResponse = response.json().await?;
        Ok(resp.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    async fn chat_streaming(&self, _request: ChatCompletionRequest) -> Result<Vec<String>> {
        Err(AIError::Api("Streaming not implemented".to_string()))
    }
}