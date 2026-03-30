use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::{AIClient, AIError, ChatCompletionRequest, Result};

pub struct AnthropicClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.anthropic.com/v1".to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicStreamEvent {
    type_: String,
    content_block: Option<AnthropicContent>,
    text: Option<String>,
}

#[async_trait]
impl AIClient for AnthropicClient {
    async fn chat(&self, request: ChatCompletionRequest) -> Result<String> {
        let anthropic_request = AnthropicRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| AnthropicMessage {
                role: m.role,
                content: m.content,
            }).collect(),
            max_tokens: 4096,
            stream: false,
        };

        let response = self.client
            .post(&format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::Api(response.text().await?));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        Ok(anthropic_response.content.first()
            .map(|c| c.text.clone())
            .unwrap_or_default())
    }

    async fn chat_streaming(&self, request: ChatCompletionRequest) -> Result<Vec<String>> {
        let anthropic_request = AnthropicRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| AnthropicMessage {
                role: m.role,
                content: m.content,
            }).collect(),
            max_tokens: 4096,
            stream: true,
        };

        let response = self.client
            .post(&format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::Api(response.text().await?));
        }

        let mut stream = response.bytes_stream();
        let mut results = Vec::new();
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(AIError::Network)?;
            if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            return Ok(results);
                        }
                        if let Ok(event) = serde_json::from_str::<AnthropicStreamEvent>(data) {
                            if let Some(text) = event.text {
                                results.push(text);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
}
