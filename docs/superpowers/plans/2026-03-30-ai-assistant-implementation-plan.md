# AI 助手实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**目标:** 实现一个浮动窗口 AI 助手面板，支持多模型对话、命令执行、服务器操作

**架构:** 
- 前端使用 Vue 3 + Pinia，通过 Tauri IPC 与 Rust 后端通信
- Rust 后端实现 API 客户端抽象层，支持多种 AI 模型
- 命令执行复用现有的 ssh/sftp 模块

**技术栈:**
- Frontend: Vue 3, Pinia, TypeScript
- Backend: Rust, reqwest, serde
- API: Claude, OpenAI, 通义千问, MiniMax, DeepSeek

---

## 文件结构

### 新增文件
```
src/
├── components/
│   ├── AIFloatingPanel.vue     # 主浮动面板容器
│   ├── AIChatView.vue          # ChatGPT 风格视图
│   ├── AITerminalView.vue      # 终端风格视图
│   ├── AIMessageList.vue       # 消息列表组件
│   ├── AIInput.vue             # 输入框组件
│   └── AISettings.vue          # 设置面板
├── stores/
│   └── ai.ts                   # AI 状态管理
└── types/
    └── ai.ts                   # AI 类型定义

src-tauri/src/
├── commands/
│   └── ai.rs                   # AI 相关命令
├── ai/
│   ├── mod.rs                  # 模块入口
│   ├── client.rs               # API 客户端抽象
│   ├── anthropic.rs            # Claude 实现
│   ├── openai.rs               # OpenAI 实现
│   ├── qwen.rs                 # 通义千问实现
│   ├── minimax.rs              # MiniMax 实现
│   └── deepseek.rs             # DeepSeek 实现
└── storage/
    └── encrypted_config.rs     # 加密配置存储
```

---

## Phase 1: 基础框架

### Task 1: AI 类型定义和 Store

**Files:**
- Create: `src/types/ai.ts`
- Create: `src/stores/ai.ts`
- Modify: `src/stores/index.ts` (add AI store export)

- [ ] **Step 1: 创建 AI 类型定义**

```typescript
// src/types/ai.ts
export type AIProvider = 'anthropic' | 'openai' | 'qwen' | 'minimax' | 'deepseek';

export type MessageRole = 'user' | 'assistant' | 'system';

export interface AIMessage {
  id: string;
  role: MessageRole;
  content: string;
  timestamp: number;
}

export interface AIConfig {
  provider: AIProvider;
  apiKey: string;
  model: string;
  baseUrl?: string;
}

export interface AIState {
  isOpen: boolean;
  config: AIConfig | null;
  messages: AIMessage[];
  isLoading: boolean;
  style: 'chatgpt' | 'terminal' | 'split';
}
```

- [ ] **Step 2: 创建 AI Store**

```typescript
// src/stores/ai.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { AIMessage, AIConfig, AIProvider } from '../types/ai';

export const useAIStore = defineStore('ai', () => {
  const isOpen = ref(false);
  const config = ref<AIConfig | null>(null);
  const messages = ref<AIMessage[]>([]);
  const isLoading = ref(false);
  const style = ref<'chatgpt' | 'terminal' | 'split'>('chatgpt');

  const toggle = () => { isOpen.value = !isOpen.value; };
  const open = () => { isOpen.value = true; };
  const close = () => { isOpen.value = false; };
  
  const setConfig = (newConfig: AIConfig) => { config.value = newConfig; };
  const setStyle = (newStyle: typeof style.value) => { style.value = newStyle; };
  
  const addMessage = (message: AIMessage) => { messages.value.push(message); };
  const clearMessages = () => { messages.value = []; };

  return {
    isOpen, config, messages, isLoading, style,
    toggle, open, close, setConfig, setStyle, addMessage, clearMessages
  };
});
```

- [ ] **Step 3: Commit**

```bash
git add src/types/ai.ts src/stores/ai.ts
git commit -m "feat(ai): add AI types and store"
```

---

### Task 2: AI 浮动面板基础组件

**Files:**
- Create: `src/components/AIFloatingPanel.vue`
- Create: `src/components/AIMessageList.vue`
- Create: `src/components/AIInput.vue`

- [ ] **Step 1: 创建 AIInput 组件**

```vue
<!-- src/components/AIInput.vue -->
<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  (e: 'send', message: string): void;
}>();

const input = ref('');

const handleSend = () => {
  if (input.value.trim()) {
    emit('send', input.value);
    input.value = '';
  }
};
</script>

<template>
  <div class="ai-input">
    <textarea
      v-model="input"
      placeholder="输入消息..."
      rows="2"
      @keydown.enter.exact.prevent="handleSend"
    ></textarea>
    <button @click="handleSend" :disabled="!input.trim()">发送</button>
  </div>
</template>

<style scoped>
.ai-input {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-top: 1px solid var(--border-default);
}
.ai-input textarea {
  flex: 1;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: 4px;
  color: var(--text-primary);
  resize: none;
  padding: 8px;
}
.ai-input button {
  padding: 8px 16px;
  background: var(--accent-primary);
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
}
.ai-input button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
```

- [ ] **Step 2: 创建 AIMessageList 组件**

```vue
<!-- src/components/AIMessageList.vue -->
<script setup lang="ts">
import type { AIMessage } from '../types/ai';

defineProps<{
  messages: AIMessage[];
}>();
</script>

<template>
  <div class="message-list">
    <div
      v-for="msg in messages"
      :key="msg.id"
      class="message"
      :class="msg.role"
    >
      <div class="message-content">{{ msg.content }}</div>
    </div>
  </div>
</template>

<style scoped>
.message-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}
.message {
  margin-bottom: 12px;
  padding: 8px 12px;
  border-radius: 8px;
}
.message.user {
  background: var(--accent-primary);
  color: white;
  margin-left: 20%;
}
.message.assistant {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  margin-right: 20%;
}
.message-content {
  white-space: pre-wrap;
  word-break: break-word;
}
</style>
```

- [ ] **Step 3: 创建 AIFloatingPanel 组件**

```vue
<!-- src/components/AIFloatingPanel.vue -->
<script setup lang="ts">
import { computed, ref } from 'vue';
import { useAIStore } from '../stores/ai';
import AIMessageList from './AIMessageList.vue';
import AIInput from './AIInput.vue';
import AISettings from './AISettings.vue';
import AITerminalView from './AITerminalView.vue';
import AISplitView from './AISplitView.vue';

const aiStore = useAIStore();
const currentView = ref<'chat' | 'settings'>('chat');

const panelStyle = computed(() => ({
  transform: aiStore.isOpen ? 'translateX(0)' : 'translateX(100%)',
  width: `${aiStore.panelWidth}px`,
});

const handleSend = async (message: string) => {
  await aiStore.sendMessage(message);
};

const copyMessage = (content: string) => {
  navigator.clipboard.writeText(content);
};
</script>

<template>
  <div class="floating-panel" :style="panelStyle">
    <div class="panel-header">
      <span>🤖 AI 助手</span>
      <button @click="aiStore.close">×</button>
    </div>
    
    <div class="panel-tabs">
      <button 
        @click="currentView = 'chat'" 
        :class="{ active: currentView === 'chat' }"
      >对话</button>
      <button 
        @click="currentView = 'settings'" 
        :class="{ active: currentView === 'settings' }"
      >设置</button>
    </div>
    
    <div class="panel-content">
      <template v-if="currentView === 'chat'">
        <AITerminalView 
          v-if="aiStore.style === 'terminal'" 
          :messages="aiStore.messages" 
        />
        <AISplitView 
          v-else-if="aiStore.style === 'split'" 
          :messages="aiStore.messages"
          @send="handleSend"
        />
        <template v-else>
          <AIMessageList :messages="aiStore.messages" @copy="copyMessage" />
          <AIInput @send="handleSend" />
        </template>
      </template>
      <AISettings v-else />
    </div>
    
    <div class="resize-handle" @mousedown="startResize"></div>
  </div>
</template>

<script lang="ts">
export default {
  methods: {
    startResize(e: MouseEvent) {
      const startX = e.clientX;
      const startWidth = this.aiStore.panelWidth;
      
      const onMouseMove = (e: MouseEvent) => {
        const delta = startX - e.clientX;
        this.aiStore.setPanelWidth(startWidth + delta);
      };
      
      const onMouseUp = () => {
        document.removeEventListener('mousemove', onMouseMove);
        document.removeEventListener('mouseup', onMouseUp);
      };
      
      document.addEventListener('mousemove', onMouseMove);
      document.addEventListener('mouseup', onMouseUp);
    }
  }
};
</script>

<style scoped>
.floating-panel {
  position: fixed;
  right: 0;
  top: 0;
  bottom: 0;
  min-width: 300px;
  max-width: 600px;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  transition: transform 0.3s ease;
  z-index: 1000;
}
.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-default);
  font-weight: 600;
}
.panel-header button {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: var(--text-secondary);
}
.panel-tabs {
  display: flex;
  border-bottom: 1px solid var(--border-default);
}
.panel-tabs button {
  flex: 1;
  padding: 8px;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
}
.panel-tabs button.active {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}
.panel-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.resize-handle {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  cursor: col-resize;
}
.resize-handle:hover {
  background: var(--accent-primary);
}
</style>
```

- [ ] **Step 4: 在 App.vue 中引入 AIFloatingPanel**

```bash
git add src/components/AIFloatingPanel.vue src/components/AIMessageList.vue src/components/AIInput.vue src/App.vue
git commit -m "feat(ai): add AI floating panel components"
```

---

### Task 3: Rust AI 模块基础结构

**Files:**
- Create: `src-tauri/src/ai/mod.rs`
- Create: `src-tauri/src/ai/client.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: 添加 Cargo 依赖**

```toml
# src-tauri/Cargo.toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
futures-util = "0.3"
async-trait = "0.1"
once_cell = "1.19"
```

- [ ] **Step 2: 创建 AI 客户端 trait**

```rust
// src-tauri/src/ai/client.rs
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
```

- [ ] **Step 3: 创建 AI 模块入口**

```rust
// src-tauri/src/ai/mod.rs
pub mod client;
pub mod anthropic;
pub mod openai;
pub mod qwen;
pub mod minimax;
pub mod deepseek;

pub use client::{AIClient, AIError, ChatCompletionRequest, Message};
```

- [ ] **Step 4: 修改 lib.rs 注册 AI 模块**

```rust
// src-tauri/src/lib.rs
mod commands;
mod storage;
mod ssh;
mod sftp;
mod ai;

use storage::database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    log::info!("Starting SSHGuard application");

    if let Err(e) = database::init_database() {
        log::error!("Failed to initialize database: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::servers::get_servers,
            commands::servers::add_server,
            commands::servers::update_server,
            commands::servers::delete_server,
            commands::ssh::connect,
            commands::ssh::create_pty,
            commands::ssh::disconnect,
            commands::ssh::send_pty_data,
            commands::ssh::read_pty_data,
            commands::sftp::list_directory,
            commands::sftp::download_file,
            commands::sftp::upload_file,
            commands::update::check_for_updates,
            commands::update::download_and_install,
            commands::ai::chat,
            commands::ai::set_config,
        ])
        .setup(|_app| {
            log::info!("Application setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/ai/mod.rs src-tauri/src/ai/client.rs src-tauri/src/lib.rs src-tauri/Cargo.toml
git commit -m "feat(ai): add Rust AI module structure"
```

---

### Task 4: Claude API 实现

**Files:**
- Create: `src-tauri/src/ai/anthropic.rs`
- Create: `src-tauri/src/commands/ai.rs`
- Modify: `src-tauri/src/ai/client.rs`

- [ ] **Step 1: 更新 client.rs 添加全局配置存储**

```rust
// src-tauri/src/ai/client.rs
use std::sync::RwLock;
use once_cell::sync::Lazy;

pub static AI_CONFIG: Lazy<RwLock<AIConfig>> = Lazy::new(|| RwLock::new(AIConfig::default()));

#[derive(Debug, Clone, Default)]
pub struct AIConfig {
    pub provider: String,
    pub api_key: String,
    pub model: String,
}
```

- [ ] **Step 2: 创建 Claude 客户端（支持流式）**

```rust
// src-tauri/src/ai/anthropic.rs
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};
use super::{AIClient, AIError, ChatCompletionRequest, Message, Result, AI_CONFIG};

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

pub struct AnthropicStream {
    inner: Pin<Box<dyn StreamExt<Item = Result<String>> + Send>>,
}

impl StreamExt for AnthropicStream {
    type Item = Result<String>;
}

impl AnthropicStream {
    pub fn new(stream: Pin<Box<dyn StreamExt<Item = reqwest::Result<String>> + Send>>) -> Self {
        Self {
            inner: stream.map(|r| r.map_err(|e| AIError::Network(e))),
        }
    }
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
```

- [ ] **Step 3: 创建 AI 命令（使用配置的模型）**

```rust
// src-tauri/src/commands/ai.rs
use serde::{Deserialize, Serialize};
use tauri::command;
use crate::ai::{self, AnthropicClient, OpenAIClient, QwenClient, MiniMaxClient, DeepSeekClient, AIClient, ChatCompletionRequest, Message, AI_CONFIG};

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
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/ai/anthropic.rs src-tauri/src/commands/ai.rs
git commit -m "feat(ai): implement Claude API client"
```

---

## Phase 2: 多模型支持

### Task 5: OpenAI API 实现

**Files:**
- Create: `src-tauri/src/ai/openai.rs`

- [ ] **Step 1: 创建 OpenAI 客户端**

```rust
// src-tauri/src/ai/openai.rs
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
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
        Err(AIError::Api("Streaming not implemented".to_string()))
    }
}
```

- [ ] **Step 2: 更新 commands/ai.rs 支持 OpenAI**

```rust
// 在 set_config 中添加:
"openai" => {
    let client = OpenAIClient::new(config.api_key);
    unsafe {
        AI_GLOBAL_CLIENT = Some(Box::new(client));
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/ai/openai.rs src-tauri/src/commands/ai.rs
git commit -m "feat(ai): add OpenAI API support"
```

---

### Task 6: 通义千问、MiniMax、DeepSeek 实现

**Files:**
- Create: `src-tauri/src/ai/qwen.rs`
- Create: `src-tauri/src/ai/minimax.rs`
- Create: `src-tauri/src/ai/deepseek.rs`

- [ ] **Step 1: 创建通义千问客户端（添加 serde::Serialize）**

```rust
// src-tauri/src/ai/qwen.rs
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
```

- [ ] **Step 2: 创建 MiniMax 客户端（添加 serde::Serialize）**

```rust
// src-tauri/src/ai/minimax.rs
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
```

- [ ] **Step 3: 创建 DeepSeek 客户端（添加 serde::Serialize）**

```rust
// src-tauri/src/ai/deepseek.rs
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
```

- [ ] **Step 4: 更新 ai/mod.rs**

```rust
pub mod anthropic;
pub mod openai;
pub mod qwen;
pub mod minimax;
pub mod deepseek;
```

- [ ] **Step 5: 更新 commands/ai.rs**

```rust
use crate::ai::{self, AnthropicClient, OpenAIClient, QwenClient, MiniMaxClient, DeepSeekClient, AIClient, ChatCompletionRequest, Message};

// 在 set_config 中添加:
"qwen" => {
    let client = QwenClient::new(config.api_key);
    unsafe { AI_GLOBAL_CLIENT = Some(Box::new(client)); }
}
"minimax" => {
    let client = MiniMaxClient::new(config.api_key);
    unsafe { AI_GLOBAL_CLIENT = Some(Box::new(client)); }
}
"deepseek" => {
    let client = DeepSeekClient::new(config.api_key);
    unsafe { AI_GLOBAL_CLIENT = Some(Box::new(client)); }
}
```

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/ai/qwen.rs src-tauri/src/ai/minimax.rs src-tauri/src/ai/deepseek.rs src-tauri/src/ai/mod.rs src-tauri/src/commands/ai.rs
git commit -m "feat(ai): add Qwen, MiniMax, DeepSeek API support"
```

---

## Phase 3: 增强功能

### Task 7: 消息持久化和复制功能

**Files:**
- Modify: `src/stores/ai.ts`
- Modify: `src/components/AIMessageList.vue`

- [ ] **Step 1: 添加消息持久化和复制功能到 Store**

```typescript
// src/stores/ai.ts
const MESSAGES_KEY = 'ai_messages';

const loadMessages = () => {
  const stored = localStorage.getItem(MESSAGES_KEY);
  if (stored) {
    messages.value = JSON.parse(stored);
  }
};

const saveMessages = () => {
  localStorage.setItem(MESSAGES_KEY, JSON.stringify(messages.value));
};

const sendMessage = async (content: string) => {
  addMessage({
    id: crypto.randomUUID(),
    role: 'user',
    content,
    timestamp: Date.now(),
  });
  saveMessages();
  
  isLoading.value = true;
  
  try {
    const response = await invoke<string>('chat', { message: content });
    addMessage({
      id: crypto.randomUUID(),
      role: 'assistant',
      content: response,
      timestamp: Date.now(),
    });
    saveMessages();
  } catch (e) {
    addMessage({
      id: crypto.randomUUID(),
      role: 'assistant',
      content: `错误: ${e}`,
      timestamp: Date.now(),
    });
    saveMessages();
  } finally {
    isLoading.value = false;
  }
};

// 初始化时加载
loadMessages();
```

- [ ] **Step 2: 添加复制按钮到 AIMessageList**

```vue
<!-- AIMessageList.vue 添加 -->
<button class="copy-btn" @click="emit('copy', msg.content)" title="复制">📋</button>

<script setup>
const emit = defineEmits<{
  (e: 'copy', content: string): void;
}>();
</script>
```

- [ ] **Step 3: Commit**

```bash
git add src/stores/ai.ts src/components/AIMessageList.vue
git commit -m "feat(ai): add message persistence and copy functionality"
```

---

### Task 8: 终端风格和分屏视图组件

**Files:**
- Create: `src/components/AITerminalView.vue`
- Create: `src/components/AISplitView.vue`

- [ ] **Step 1: 创建 AITerminalView 组件**

```vue
<!-- src/components/AITerminalView.vue -->
<script setup lang="ts">
import type { AIMessage } from '../types/ai';

defineProps<{
  messages: AIMessage[];
}>();
</script>

<template>
  <div class="terminal-view">
    <div class="terminal-output">
      <div v-for="msg in messages" :key="msg.id" class="terminal-line">
        <span class="prompt" v-if="msg.role === 'user'">$ </span>
        <span class="response-indicator" v-else>> </span>
        <span class="content">{{ msg.content }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.terminal-view {
  flex: 1;
  background: #0d1117;
  color: #c9d1d9;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 13px;
  overflow-y: auto;
}
.terminal-output { padding: 12px; }
.terminal-line { margin-bottom: 8px; line-height: 1.4; }
.prompt { color: #58a6ff; }
.response-indicator { color: #7ee787; }
.content { white-space: pre-wrap; word-break: break-word; }
</style>
```

- [ ] **Step 2: 创建 AISplitView 组件**

```vue
<!-- src/components/AISplitView.vue -->
<script setup lang="ts">
import { ref } from 'vue';
import type { AIMessage } from '../types/ai';
import AIMessageList from './AIMessageList.vue';
import AIInput from './AIInput.vue';
import TerminalPanel from './TerminalPanel.vue';

defineProps<{ messages: AIMessage[] }>();
const emit = defineEmits<{ (e: 'send', message: string): void }>();
const rightPanel = ref<'terminal' | 'files'>('terminal');
const handleSend = (message: string) => emit('send', message);
</script>

<template>
  <div class="split-view">
    <div class="split-left">
      <AIMessageList :messages="messages" />
      <AIInput @send="handleSend" />
    </div>
    <div class="split-divider"></div>
    <div class="split-right">
      <div class="right-tabs">
        <button :class="{ active: rightPanel === 'terminal' }" @click="rightPanel = 'terminal'">终端</button>
        <button :class="{ active: rightPanel === 'files' }" @click="rightPanel = 'files'">文件</button>
      </div>
      <div class="right-content">
        <TerminalPanel v-if="rightPanel === 'terminal'" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.split-view { flex: 1; display: flex; overflow: hidden; }
.split-left { flex: 1; display: flex; flex-direction: column; }
.split-divider { width: 4px; background: var(--border-default); cursor: col-resize; }
.split-right { width: 50%; display: flex; flex-direction: column; }
.right-tabs { display: flex; border-bottom: 1px solid var(--border-default); }
.right-tabs button { flex: 1; padding: 8px; background: transparent; border: none; color: var(--text-secondary); cursor: pointer; }
.right-tabs button.active { background: var(--bg-tertiary); color: var(--text-primary); }
.right-content { flex: 1; overflow: hidden; }
</style>
```

- [ ] **Step 3: Commit**

```bash
git add src/components/AITerminalView.vue src/components/AISplitView.vue
git commit -m "feat(ai): add terminal and split view components"
```

---

### Task 9: 设置面板

**Files:**
- Create: `src/components/AISettings.vue`

- [ ] **Step 1: 创建 AISettings 组件**

```vue
<!-- src/components/AISettings.vue -->
<script setup lang="ts">
import { ref } from 'vue';
import { useAIStore } from '../stores/ai';
import { invoke } from '@tauri-apps/api/core';

const aiStore = useAIStore();
const providers = [
  { label: 'Claude (Anthropic)', value: 'anthropic' },
  { label: 'GPT (OpenAI)', value: 'openai' },
  { label: '通义千问', value: 'qwen' },
  { label: 'MiniMax', value: 'minimax' },
  { label: 'DeepSeek', value: 'deepseek' },
];
const modelOptions: Record<string, string[]> = {
  anthropic: ['claude-3-5-sonnet-20241022', 'claude-3-opus-20240229'],
  openai: ['gpt-4o', 'gpt-4-turbo', 'gpt-3.5-turbo'],
  qwen: ['qwen-plus', 'qwen-turbo', 'qwen-max'],
  minimax: ['MiniMax-Text-01'],
  deepseek: ['deepseek-chat', 'deepseek-coder'],
};
const selectedProvider = ref('anthropic');
const apiKey = ref('');
const selectedModel = ref('claude-3-5-sonnet-20241022');
const selectedStyle = ref<'chatgpt' | 'terminal' | 'split'>('chatgpt');

const saveConfig = async () => {
  await invoke('set_config', {
    config: { provider: selectedProvider.value, api_key: apiKey.value, model: selectedModel.value }
  });
  aiStore.setStyle(selectedStyle.value);
};
</script>

<template>
  <div class="ai-settings">
    <h3>设置</h3>
    <div class="form-group">
      <label>AI 提供商</label>
      <select v-model="selectedProvider">
        <option v-for="p in providers" :key="p.value" :value="p.value">{{ p.label }}</option>
      </select>
    </div>
    <div class="form-group">
      <label>API Key</label>
      <input type="password" v-model="apiKey" placeholder="输入 API Key" />
    </div>
    <div class="form-group">
      <label>模型</label>
      <select v-model="selectedModel">
        <option v-for="m in modelOptions[selectedProvider]" :key="m" :value="m">{{ m }}</option>
      </select>
    </div>
    <div class="form-group">
      <label>界面风格</label>
      <select v-model="selectedStyle">
        <option value="chatgpt">ChatGPT 风格</option>
        <option value="terminal">终端风格</option>
        <option value="split">分屏布局</option>
      </select>
    </div>
    <button @click="saveConfig" class="save-btn">保存</button>
  </div>
</template>

<style scoped>
.ai-settings { padding: 16px; }
.form-group { margin-bottom: 12px; }
.form-group label { display: block; font-size: 12px; color: var(--text-secondary); margin-bottom: 4px; }
.form-group select, .form-group input { width: 100%; padding: 8px; background: var(--bg-primary); border: 1px solid var(--border-default); border-radius: 4px; color: var(--text-primary); }
.save-btn { width: 100%; padding: 8px; background: var(--accent-primary); border: none; border-radius: 4px; color: white; cursor: pointer; }
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/AISettings.vue
git commit -m "feat(ai): add AI settings panel"
```

---

### Task 10: 快捷键、命令确认和面板位置记忆

**Files:**
- Modify: `src/App.vue`
- Modify: `src/stores/ai.ts`
- Modify: `src-tauri/src/commands/ai.rs`

- [ ] **Step 1: 添加快捷键支持到 App.vue**

```typescript
// src/App.vue
import { onMounted, onUnmounted } from 'vue';
import { useAIStore } from './stores/ai';

const aiStore = useAIStore();

const handleKeydown = (e: KeyboardEvent) => {
  if (e.ctrlKey && e.shiftKey && e.key === 'A') {
    e.preventDefault();
    aiStore.toggle();
  }
  if (e.key === 'Escape' && aiStore.isOpen) {
    aiStore.close();
  }
};

onMounted(() => window.addEventListener('keydown', handleKeydown));
onUnmounted(() => window.removeEventListener('keydown', handleKeydown));
```

- [ ] **Step 2: 添加面板位置记忆到 Store**

```typescript
// src/stores/ai.ts
const POSITION_KEY = 'ai_panel_position';
const panelPosition = ref({ right: 0, top: 0 });

const loadPosition = () => {
  const stored = localStorage.getItem(POSITION_KEY);
  if (stored) panelPosition.value = JSON.parse(stored);
};

const savePosition = () => {
  localStorage.setItem(POSITION_KEY, JSON.stringify(panelPosition.value));
};

loadPosition();
```

- [ ] **Step 3: 添加命令确认功能**

```rust
// src-tauri/src/commands/ai.rs

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandConfirmation {
    pub command: String,
    pub confirmed: bool,
}

#[command]
pub async fn execute_command_with_confirmation(
    command: String,
    session_id: String,
) -> Result<CommandConfirmation, String> {
    // 返回命令供前端显示确认对话框
    Ok(CommandConfirmation {
        command,
        confirmed: false,
    })
}

#[command]
pub async fn confirm_and_execute(
    command: String,
    session_id: String,
) -> Result<String, String> {
    crate::ssh::send_pty_data(&session_id, format!("{}\n", command).as_bytes())
        .map_err(|e| e.to_string())?;
    Ok("Command executed".to_string())
}
```

- [ ] **Step 4: Commit**

```bash
git add src/App.vue src/stores/ai.ts src-tauri/src/commands/ai.rs
git commit -m "feat(ai): add hotkeys, panel position memory, and command confirmation"
```

---

## Phase 4: 服务器操作和文件浏览

### Task 11: 服务器状态查询

**Files:**
- Modify: `src-tauri/src/commands/ai.rs`

- [ ] **Step 1: 添加服务器状态查询命令**

```rust
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
        crate::ssh::send_pty_data(&session_id, format!("{}\n", cmd).as_bytes())
            .map_err(|e| e.to_string())?;
        // 等待并读取响应
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        if let Ok(output) = crate::ssh::read_pty_data(&session_id, 1000) {
            results.push(String::from_utf8_lossy(&output).to_string());
        }
    }
    
    Ok(results.join("\n"))
}
```

- [ ] **Step 2: Commit**

```bash
git add src-tauri/src/commands/ai.rs
git commit -m "feat(ai): add server status query command"
```

---

### Task 12: AIChatView 和 AI 对话视图

**Files:**
- Create: `src/components/AIChatView.vue`

- [ ] **Step 1: 创建 AIChatView 组件**

```vue
<!-- src/components/AIChatView.vue -->
<script setup lang="ts">
import { useAIStore } from '../stores/ai';
import AIMessageList from './AIMessageList.vue';
import AIInput from './AIInput.vue';

const aiStore = useAIStore();

const handleSend = async (message: string) => {
  await aiStore.sendMessage(message);
};

const copyMessage = (content: string) => {
  navigator.clipboard.writeText(content);
};
</script>

<template>
  <div class="chat-view">
    <AIMessageList :messages="aiStore.messages" @copy="copyMessage" />
    <AIInput @send="handleSend" />
  </div>
</template>

<style scoped>
.chat-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/AIChatView.vue
git commit -m "feat(ai): add AIChatView component"
```

---

## 总结

实现计划包含 12 个任务：

| Phase | Task | 内容 |
|-------|------|------|
| 1 | 1 | AI 类型定义和 Store |
| 1 | 2 | AI 浮动面板基础组件 |
| 1 | 3 | Rust AI 模块基础结构 |
| 1 | 4 | Claude API 实现 |
| 2 | 5 | OpenAI API 实现 |
| 2 | 6 | 通义千问、MiniMax、DeepSeek 实现 |
| 3 | 7 | 消息持久化和复制功能 |
| 3 | 8 | 终端风格和分屏视图组件 |
| 3 | 9 | 设置面板 |
| 3 | 10 | 快捷键、命令确认和面板位置记忆 |
| 4 | 11 | 服务器状态查询 |
| 4 | 12 | AIChatView 组件 |