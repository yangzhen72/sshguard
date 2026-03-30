use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::{AIClient, AIError, ChatCompletionRequest, Message, Result};

pub struct DeepSeekClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl DeepSeekClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.deepseek.com/v1".to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: Message,
}

#[async_trait]
impl AIClient for DeepSeekClient {
    async fn chat(&self, request: ChatCompletionRequest) -> Result<String> {
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&DeepSeekRequest {
                model: request.model,
                messages: request.messages,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::Api(response.text().await?));
        }

        let resp: DeepSeekResponse = response.json().await?;
        Ok(resp.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    async fn chat_streaming(&self, _request: ChatCompletionRequest) -> Result<Vec<String>> {
        Err(AIError::Api("Streaming not implemented".to_string()))
    }
}