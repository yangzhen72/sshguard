use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use once_cell::sync::Lazy;
use thiserror::Error;

pub static AI_CONFIG: Lazy<RwLock<AIConfig>> = Lazy::new(|| RwLock::new(AIConfig::default()));

#[derive(Debug, Clone, Default)]
pub struct AIConfig {
    pub provider: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Error)]
pub enum AIError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, AIError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[async_trait]
pub trait AIClient: Send + Sync {
    async fn chat(&self, request: ChatCompletionRequest) -> Result<String>;
    async fn chat_streaming(&self, request: ChatCompletionRequest) -> Result<Vec<String>>;
}
