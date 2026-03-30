use serde::{Deserialize, Serialize};
use tauri::command;
use crate::ai::{AIClient, ChatCompletionRequest, Message, AI_CONFIG, AIConfig};
use crate::ai::anthropic::AnthropicClient;
use crate::ai::openai::OpenAIClient;
use crate::ai::qwen::QwenClient;
use crate::ai::minimax::MiniMaxClient;
use crate::ai::deepseek::DeepSeekClient;

static mut AI_GLOBAL_CLIENT: Option<Box<dyn AIClient + Send + Sync>> = None;

#[command]
pub async fn set_config(config: AIConfig) -> Result<(), String> {
    let client: Box<dyn AIClient + Send + Sync> = match config.provider.as_str() {
        "anthropic" => Box::new(AnthropicClient::new(config.api_key.clone())),
        "openai" => Box::new(OpenAIClient::new(config.api_key.clone())),
        "qwen" => Box::new(QwenClient::new(config.api_key.clone())),
        "minimax" => Box::new(MiniMaxClient::new(config.api_key.clone())),
        "deepseek" => Box::new(DeepSeekClient::new(config.api_key.clone())),
        _ => return Err("Unsupported provider".to_string()),
    };
    
    unsafe {
        AI_GLOBAL_CLIENT = Some(client);
    }
    
    {
        let mut cfg = AI_CONFIG.write().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        *cfg = config;
    }
    
    Ok(())
}

static mut AI_GLOBAL_CLIENT: Option<Box<dyn AIClient + Send + Sync>> = None;

#[command]
pub async fn set_config(config: AIConfig) -> Result<(), String> {
    let client: Box<dyn AIClient + Send + Sync> = match config.provider.as_str() {
        "anthropic" => Box::new(AnthropicClient::new(config.api_key.clone())),
        "openai" => Box::new(OpenAIClient::new(config.api_key.clone())),
        "qwen" => Box::new(QwenClient::new(config.api_key.clone())),
        "minimax" => Box::new(MiniMaxClient::new(config.api_key.clone())),
        "deepseek" => Box::new(DeepSeekClient::new(config.api_key.clone())),
        _ => return Err("Unsupported provider".to_string()),
    };
    
    unsafe {
        AI_GLOBAL_CLIENT = Some(client);
    }
    
    {
        let mut cfg = AI_CONFIG.write().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        *cfg = config;
    }
    
    Ok(())
}

#[command]
pub async fn chat(message: String) -> Result<String, String> {
    let config = {
        let cfg = AI_CONFIG.read().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
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
        let cfg = AI_CONFIG.read().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
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

#[command]
pub async fn query_server_status(session_id: String) -> Result<String, String> {
    let commands = vec![
        "echo '=== CPU ===' && top -bn1 | head -5",
        "echo '=== Memory ===' && free -h",
        "echo '=== Disk ===' && df -h",
        "echo '=== Uptime ===' && uptime",
    ];
    
    let mut results = Vec::new();
    for cmd in commands {
        if let Err(e) = crate::ssh::send_pty_data(&session_id, format!("{}\n", cmd).as_bytes()) {
            results.push(format!("Error: {}", e));
            continue;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        if let Ok(output) = crate::ssh::read_pty_data(&session_id, 1000) {
            results.push(String::from_utf8_lossy(&output).to_string());
        }
    }
    
    Ok(results.join("\n"))
}
