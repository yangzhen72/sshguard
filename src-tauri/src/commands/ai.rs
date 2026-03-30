use serde::{Deserialize, Serialize};
use tauri::command;
use crate::ai::{self, AnthropicClient, AIClient, ChatCompletionRequest, Message, AI_CONFIG};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIConfig {
    pub provider: String,
    pub api_key: String,
    pub model: String,
}

static mut AI_GLOBAL_CLIENT: Option<Box<dyn AIClient + Send + Sync>> = None;

#[command]
pub async fn set_config(config: AIConfig) -> Result<(), String> {
    let client: Box<dyn AIClient + Send + Sync> = match config.provider.as_str() {
        "anthropic" => Box::new(AnthropicClient::new(config.api_key.clone())),
        _ => return Err("Unsupported provider".to_string()),
    };
    
    unsafe {
        AI_GLOBAL_CLIENT = Some(client);
    }
    
    {
        let mut cfg = AI_CONFIG.write().map_err(|e| e.to_string())?;
        *cfg = config;
    }
    
    Ok(())
}

#[command]
pub async fn chat(message: String) -> Result<String, String> {
    let config = {
        let cfg = AI_CONFIG.read().map_err(|e| e.to_string())?;
        cfg.clone()
    };
    
    let request = ChatCompletionRequest {
        model: config.model,
        messages: vec![Message {
            role: "user".to_string(),
            content: message,
        }],
        stream: false,
    };

    unsafe {
        if let Some(ref client) = AI_GLOBAL_CLIENT {
            client.chat(request).await.map_err(|e| e.to_string())
        } else {
            Err("AI client not configured".to_string())
        }
    }
}

#[command]
pub async fn chat_streaming(message: String) -> Result<Vec<String>, String> {
    let config = {
        let cfg = AI_CONFIG.read().map_err(|e| e.to_string())?;
        cfg.clone()
    };
    
    let request = ChatCompletionRequest {
        model: config.model,
        messages: vec![Message {
            role: "user".to_string(),
            content: message,
        }],
        stream: true,
    };

    unsafe {
        if let Some(ref client) = AI_GLOBAL_CLIENT {
            client.chat_streaming(request).await.map_err(|e| e.to_string())
        } else {
            Err("AI client not configured".to_string())
        }
    }
}
