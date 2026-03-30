use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::{AIClient, AIError, ChatCompletionRequest, Message, Result};

pub struct OpenAIClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

#[async_trait]
impl AIClient for OpenAIClient {
    async fn chat(&self, request: ChatCompletionRequest) -> Result<String> {
        let openai_request = OpenAIRequest {
            model: request.model,
            messages: request.messages,
            stream: false,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::Api(response.text().await?));
        }

        let openai_response: OpenAIResponse = response.json().await?;
        Ok(openai_response.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    async fn chat_streaming(&self, _request: ChatCompletionRequest) -> Result<Vec<String>> {
        Err(AIError::Api("Streaming not implemented for OpenAI".to_string()))
    }
}