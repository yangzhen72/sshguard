# AI 助手功能设计

**日期**: 2026-03-30
**状态**: 已批准
**功能**: AI 助手

## 1. 功能概述

AI 助手是一个浮动窗口面板，提供自然语言对话、SSH 命令生成与执行、服务器状态查询、文件操作等全功能 AI 辅助能力。

## 2. 支持的模型

| 模型 | 厂商 | API Endpoint |
|------|------|--------------|
| Claude 3.5 Sonnet | Anthropic | https://api.anthropic.com/v1/messages |
| GPT-4o | OpenAI | https://api.openai.com/v1/chat/completions |
| 通义千问 (Qwen) | 阿里云 | https://dashscope.aliyuncs.com/compatible-mode/v1/chat |
| MiniMax | MiniMax | https://api.minimax.chat/v1/text/chatcompletion_v2 |
| DeepSeek V3 | 深度求索 | https://api.deepseek.com/v1/chat/completions |

## 3. UI 设计

### 3.1 浮动窗口
- 从屏幕右侧滑出，宽度可调（300px-600px）
- 可拖拽位置，记忆用户偏好
- 关闭按钮隐藏面板，快捷键可呼出
- 标题栏显示当前模型和状态

### 3.2 界面布局
```
┌─────────────────────────────┐
│ 🤖 AI 助手    [─][□][×]   │  <- 标题栏
├──────────┬──────────────────┤
│ 对话     │                  │
│ 历史     │   聊天区域        │
│          │                  │
│ 命令历史  │   用户输入框      │
│          │   [发送] [停止]   │
│ 设置     │                  │
└──────────┴──────────────────┘
```

### 3.3 可配置界面风格
1. **ChatGPT 风格** - 现代化卡片式对话，气泡式消息
2. **终端风格** - 黑色背景，等宽字体，命令提示符样式
3. **分屏布局** - 左侧对话，右侧可切换终端/文件视图

## 4. 功能模块

### 4.1 对话功能
- 发送文本消息给 AI
- 流式响应显示
- 消息历史记录（本地存储）
- 复制消息内容

### 4.2 命令执行
- AI 生成 SSH 命令
- 用户确认后执行
- 在终端面板显示执行结果
- 支持 sudo 命令（需要权限配置）

### 4.3 服务器操作
- 查询服务器状态
- 查看资源使用情况
- 执行简单运维操作

### 4.4 文件操作
- 浏览远程文件
- 生成文件操作命令
- 用户确认后执行

### 4.5 设置管理
- API Provider 选择
- API Key 配置（加密存储）
- 模型选择
- 界面风格选择
- 快捷键配置

## 5. 技术架构

### 5.1 前端结构
```
src/
├── components/
│   ├── AIFloatingPanel.vue    # 主面板
│   ├── AIChatView.vue         # 聊天视图
│   ├── AITerminalView.vue     # 终端风格视图
│   ├── AISplitView.vue        # 分屏视图
│   ├── AIMessageList.vue      # 消息列表
│   ├── AIInput.vue            # 输入框
│   └── AISettings.vue         # 设置面板
├── stores/
│   └── ai.ts                  # AI 状态管理
└── types/
    └── ai.ts                  # AI 相关类型
```

### 5.2 Rust 后端结构
```
src-tauri/src/
├── commands/
│   └── ai.rs                  # AI 相关命令
├── ai/
│   ├── mod.rs                 # AI 模块入口
│   ├── client.rs              # API 客户端抽象
│   ├── anthropic.rs           # Claude 实现
│   ├── openai.rs              # OpenAI 实现
│   ├── qwen.rs                # 通义千问实现
│   ├── minimax.rs             # MiniMax 实现
│   └── deepseek.rs            # DeepSeek 实现
└── storage/
    └── encrypted_config.rs    # 加密配置存储
```

### 5.3 数据流
1. 用户在 AI 面板输入消息
2. 前端通过 Tauri IPC 调用 Rust 命令
3. Rust 后端根据选择的模型调用对应 API
4. API 返回流式响应
5. 前端实时显示响应内容
6. 如需执行命令，通过现有 ssh/sftp 模块执行

## 6. 安全考虑

1. **API Key 加密存储**：使用 age 或 AES-GCM 加密
2. **命令执行确认**：AI 生成的命令需用户确认
3. **敏感操作限制**：危险命令（如 rm -rf）需二次确认
4. **网络隔离**：API 通信使用 HTTPS

## 7. 实现优先级

### Phase 1 - 基础框架
- AI 面板 UI 组件
- 基本对话功能
- API 客户端抽象层
- Claude API 实现

### Phase 2 - 多模型支持
- OpenAI API 实现
- 通义千问 API 实现
- MiniMax API 实现
- DeepSeek API 实现

### Phase 3 - 增强功能
- 命令执行功能
- 服务器操作功能
- 多种界面风格
- 设置管理

## 8. 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+Shift+A` | 打开/关闭 AI 面板 |
| `Enter` | 发送消息 |
| `Shift+Enter` | 换行 |
| `Escape` | 关闭面板 |