# SSHGuard

一款面向 Windows 的 SSH 连接管理工具，基于 Tauri 2 + Vue 3 构建。

![SSHGuard](https://img.shields.io/badge/Windows-SSH%20Manager-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Tauri](https://img.shields.io/badge/Tauri-2.0-4B32C3)
![Vue](https://img.shields.io/badge/Vue-3.4-42b883)

## 功能特性

### 🔗 连接管理
- 多服务器分组管理（树形结构）
- 服务器标签系统
- 搜索过滤功能
- 支持密码、私钥、SSH Agent 三种认证方式
- 连接配置加密存储

### 💻 终端
- 多标签页终端
- xterm.js 渲染
- 会话保持与重连
- 终端跟随功能（自动检测 `cd` 命令切换目录）
- 自定义配色方案

### 📁 SFTP 文件管理
- 远程文件浏览器
- 文件上传/下载
- 断点续传
- 目录批量上传/下载
- 文件编辑（内置 Monaco 代码编辑器）
- 代码高亮、行号、折叠

### 🔒 安全
- SQLite 数据库加密存储
- 用户主密码保护
- SSH 凭据不存储明文

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2.x |
| 前端 | Vue 3 + TypeScript + Vite |
| UI 组件 | Naive UI |
| 终端 | xterm.js |
| 代码编辑器 | Monaco Editor |
| 后端 | Rust |
| SSH/SFTP | ssh2 crate |
| 数据库 | SQLite (rusqlite) |
| 加密 | age |

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Windows 10/11

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
# 启动前端开发服务器
npm run dev

# 或启动完整 Tauri 开发模式
npm run tauri dev
```

### 构建应用

```bash
# 构建前端
npm run build

# 构建 Tauri 应用
npm run tauri build
```

构建完成后，安装包位于 `src-tauri/target/release/bundle/` 目录。

## 项目结构

```
sshguard/
├── src/                      # Vue 前端源码
│   ├── components/           # UI 组件
│   │   ├── Sidebar.vue      # 服务器列表侧边栏
│   │   ├── ServerTree.vue    # 服务器树形列表
│   │   ├── ServerForm.vue    # 服务器配置表单
│   │   ├── TabBar.vue        # 终端标签页
│   │   ├── TerminalPanel.vue # 终端面板
│   │   ├── SftpPanel.vue     # SFTP 文件面板
│   │   ├── FileEditor.vue    # 代码编辑器
│   │   └── StatusBar.vue     # 状态栏
│   ├── stores/              # Pinia 状态管理
│   │   ├── servers.ts        # 服务器配置状态
│   │   ├── terminals.ts      # 终端会话状态
│   │   └── sftp.ts          # SFTP 状态
│   ├── views/               # 页面视图
│   │   ├── MainLayout.vue   # 主布局
│   │   └── Settings.vue     # 设置页面
│   ├── styles/              # 全局样式
│   └── types/               # TypeScript 类型定义
├── src-tauri/               # Rust 后端源码
│   └── src/
│       ├── commands/         # Tauri 命令
│       │   ├── servers.rs    # 服务器 CRUD
│       │   ├── ssh.rs       # SSH 连接
│       │   └── sftp.rs      # SFTP 操作
│       ├── storage/          # 存储模块
│       │   ├── database.rs  # SQLite 操作
│       │   └── encryption.rs # 加密模块
│       ├── ssh/              # SSH 模块
│       │   ├── session.rs   # SSH 会话管理
│       │   └── pty.rs       # PTY 处理
│       └── sftp/
│           └── client.rs    # SFTP 客户端
├── package.json
├── vite.config.ts
├── tsconfig.json
└── SPEC.md                  # 项目规格说明
```

## 使用指南

### 添加服务器

1. 点击侧边栏 `+` 按钮
2. 填写服务器信息（名称、主机、端口、用户名）
3. 选择认证方式（密码/私钥/SSH Agent）
4. 点击保存

### 连接服务器

1. 在左侧服务器列表选择要连接的服务器
2. 双击或点击连接按钮
3. 终端标签页将自动创建并连接

### SFTP 文件传输

1. 连接服务器后，右侧 SFTP 面板显示远程文件
2. 双击文件夹进入目录
3. 右键文件进行上传/下载操作

## 键盘快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+T` | 新建终端标签 |
| `Ctrl+W` | 关闭当前标签 |
| `Ctrl+Tab` | 切换到下一个标签 |
| `Ctrl+Shift+Tab` | 切换到上一个标签 |
| `Ctrl+L` | 清空终端 |
| `F5` | 刷新 SFTP 目录 |

## 配置说明

### 数据存储

配置文件存储在：
- Windows: `%APPDATA%\com.sshguard\SSHGuard\`

### 数据库加密

首次启动会要求设置主密码，用于加密存储的 SSH 凭据。

## 开发相关

### 添加新依赖

**前端 (npm):**
```bash
npm install <package>
```

**后端 (Cargo):**
编辑 `src-tauri/Cargo.toml`，然后运行：
```bash
cd src-tauri && cargo build
```

### 代码规范

- 前端遵循 Vue 3 Composition API 风格
- 后端遵循 Rust 所有权和生命周期规则
- 所有公共 API 需添加文档注释

## License

MIT License
