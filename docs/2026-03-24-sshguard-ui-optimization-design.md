# SSHGuard UI/UX 优化设计方案

**版本**: 1.0  
**日期**: 2026-03-24  
**项目**: SSHGuard  
**目标**: UI 视觉优化、终端/SFTP 联动、批量管理、自动更新

---

## 1. 概述

本设计方案为 SSHGuard 提供全面的用户体验提升，包括 Cyberpunk 视觉风格、终端/SFTP 深度联动、批量服务器管理以及自动更新功能。

## 2. UI 视觉优化 - Cyberpunk 风格

### 2.1 配色方案

| 元素 | 颜色 | 说明 |
|------|------|------|
| 主背景 | `#0a0a0f` | 深邃黑色 |
| 次背景 | `#12121a` | 卡片/面板背景 |
| 边框 | `#1a1a2e` | 默认边框 |
| 主色调 | `#00ffff` | 青色，霓虹效果 |
| 辅助色 | `#ff00ff` | 品红，hover 效果 |
| 强调色 | `#8b00ff` | 紫色，特殊元素 |
| 文字主色 | `#e0e0e0` | 亮灰文字 |
| 文字次色 | `#808080` | 暗灰文字 |
| 成功色 | `#00ff88` | 绿色 |
| 错误色 | `#ff3366` | 红色 |
| 警告色 | `#ffaa00` | 橙色 |

### 2.2 组件样式

#### 霓虹边框
- 默认状态：`1px solid #1a1a2e`
- Hover 状态：`1px solid #00ffff` + `box-shadow: 0 0 10px #00ffff40`
- Active 状态：`1px solid #ff00ff` + `box-shadow: 0 0 15px #ff00ff60`

#### 按钮
- 背景渐变：`linear-gradient(135deg, #00ffff 0%, #8b00ff 100%)`
- Hover：发光效果增强 + 轻微放大 `scale(1.02)`
- 圆角：`4px`

#### 终端面板
- 背景：`#000000`（纯黑）
- 文字：`#00ff88`（绿色）或 `#00ffff`（青色）
- 扫描线效果：`repeating-linear-gradient` 实现

#### 侧边栏
- 宽度：`280px`（可调整）
- 服务器分组：发光分隔线
- 服务器项：hover 时左侧出现青色竖线

### 2.3 实现方式
- 使用 CSS 变量（CSS Custom Properties）管理主题
- 全局样式文件：`src/styles/cyberpunk.css`
- 组件内使用 `class` 或 Naive UI 的 `themeOverrides`

---

## 3. 终端/SFTP 联动

### 3.1 功能描述
当用户在终端中执行 `cd <path>` 命令时，SFTP 面板自动切换到对应目录。

### 3.2 实现方案

#### 检测 `cd` 命令
```typescript
// 在 TerminalPanel 中拦截用户输入
// 正则匹配：/^cd\s+(.+)$/ 或 /^\s*cd\s+(.+)$/
// 提取路径部分
```

#### 路径同步
1. 解析 `cd` 命令获取目标目录
2. 如果是绝对路径直接切换
3. 如果是相对路径，基于当前 SFTP 路径计算
4. 触发 SFTP store 的 `navigateTo()` 方法
5. 更新状态栏显示当前同步状态

#### 状态同步
| 状态 | 显示 | 说明 |
|------|------|------|
| 已同步 | `同步 | ~/path` | SFTP 已跟随终端 |
| 同步中 | `同步中...` | 正在切换目录 |
| 未连接 | `--` | 无活动终端或未连接 |
| 同步失败 | `同步失败` | 路径不存在 |

### 3.3 交互细节
- 只在成功执行 `cd` 后同步（需要检测 shell 响应）
- 支持 `cd -` 返回上一个目录
- 忽略 `cd` 后面没有路径的情况

---

## 4. 批量管理功能

### 4.1 服务器多选

#### UI 交互
- **Ctrl + 点击**： Toggle 单个服务器选择
- **Shift + 点击**：范围选择（从上次选中到当前）
- **Ctrl + A**：全选当前分组
- 多选后工具栏显示已选数量

#### 视觉反馈
- 选中项：背景色 `#00ffff20` + 左侧青色边框
- 已选计数 badge 显示在批量操作按钮上

### 4.2 批量连接

#### 功能
- 选中多个服务器 → 点击「批量连接」按钮
- 依次打开每个服务器的终端标签
- 每个标签标题显示服务器名称

#### 标签命名
- 单服务器：`服务器名`
- 多服务器：`服务器名1 + 2 等`（标签过多时压缩）

### 4.3 批量命令

#### UI 组件
- 新增「批量命令」面板（可折叠）
- 勾选要执行命令的服务器列表
- 命令输入框（支持多行）
- 「发送到所有」按钮

#### 执行流程
1. 收集所有选中的活动会话
2. 依次向每个会话发送命令
3. 实时显示每个会话的执行状态
4. 执行完成后显示汇总结果

---

## 5. 自动更新功能

### 5.1 技术方案

使用 Tauri 的 `tauri-plugin-updater` 或自行实现 GitHub Releases 检查。

### 5.2 更新流程

```
启动 → 检测更新 → [有新版本?] → 显示更新对话框
                                     ↓
                              用户确认 → 下载更新 → 安装并重启
                                     ↓
                              用户忽略 → 继续使用
```

### 5.3 GitHub Releases 集成

#### 检查更新
- API：`https://api.github.com/repos/{owner}/{repo}/releases/latest`
- 比较语义化版本号（当前版本 vs 最新版本）

#### Release 格式要求
```json
{
  "tag_name": "v0.2.0",
  "name": "SSHGuard v0.2.0",
  "body": "Release notes here...",
  "assets": [
    {
      "name": "SSHGuard_0.2.0_x64-setup.exe",
      "browser_download_url": "..."
    }
  ]
}
```

### 5.4 UI 组件

#### 更新对话框
```
┌─────────────────────────────────────┐
│  🎉 新版本可用                        │
├─────────────────────────────────────┤
│  当前版本：v0.1.0                    │
│  最新版本：v0.2.0                    │
│                                     │
│  更新内容：                          │
│  • 修复了连接稳定性问题               │
│  • 新增批量管理功能                   │
│  • UI 界面优化                       │
│                                     │
│  [ 立即更新 ]  [ 稍后提醒 ]  [ 忽略 ] │
└─────────────────────────────────────┘
```

#### 下载进度
- 模态对话框 + 进度条
- 显示下载百分比和速度
- 完成后提示重启

### 5.5 实现位置
- Rust 后端：`src-tauri/src/commands/update.rs`
- 前端 Store：`src/stores/update.ts`
- 前端组件：`src/components/UpdateDialog.vue`

---

## 6. 技术实现清单

### 6.1 文件变更

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/styles/cyberpunk.css` | 新增 | Cyberpunk 主题样式 |
| `src/styles/variables.css` | 新增 | CSS 变量定义 |
| `src/components/Sidebar.vue` | 修改 | 多选支持 + 样式 |
| `src/components/TerminalPanel.vue` | 修改 | cd 命令检测 |
| `src/components/SftpPanel.vue` | 修改 | 同步状态显示 |
| `src/components/BatchCommandPanel.vue` | 新增 | 批量命令面板 |
| `src/components/UpdateDialog.vue` | 新增 | 更新对话框 |
| `src/stores/sftp.ts` | 修改 | 添加同步方法 |
| `src/stores/update.ts` | 新增 | 更新状态管理 |
| `src/views/MainLayout.vue` | 修改 | 集成批量面板 |
| `src-tauri/src/commands/update.rs` | 新增 | 检查更新命令 |
| `src-tauri/src/lib.rs` | 修改 | 注册更新命令 |
| `src-tauri/Cargo.toml` | 修改 | 添加依赖 |

### 6.2 新增依赖

**前端：**
- 无需新增（使用现有 Naive UI + 自定义 CSS）

**后端 (Cargo.toml)：**
```toml
tauri-plugin-updater = "2"  # 或自行实现
reqwest = { version = "0.12", features = ["json"] }
semver = "1"
```

---

## 7. 设计稿/原型

暂无 Figma/草稿。可基于以下路径实现：
1. 先实现 CSS 变量和基础样式
2. 逐步应用样式到各组件
3. 使用浏览器开发者工具实时调整

---

## 8. 风险与注意事项

1. **Cyberpunk 风格可能过于花哨**：建议提供主题开关，用户可选择保留原 Naive UI 暗色主题
2. **cd 命令检测可能误判**：需要测试多种场景（相对路径、特殊字符、空格等）
3. **GitHub API 限速**：生产环境建议添加缓存或使用付费 API
4. **更新安装权限**：Windows UAC 可能弹出确认对话框

---

## 9. 后续迭代

- [ ] 主题切换器（Cyberpunk / 简约 / 原生）
- [ ] 自定义快捷键
- [ ] 命令历史同步
- [ ] 云同步配置（参考 iShellPro）
