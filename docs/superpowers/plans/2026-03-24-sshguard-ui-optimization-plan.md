# SSHGuard UI/UX 优化实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**目标:** 为 SSHGuard 实现 Cyberpunk UI、终端/SFTP 联动、批量管理功能和自动更新

**架构:** 
- CSS 变量系统管理主题色彩
- Pinia Store 管理终端/SFTP 同步状态
- Tauri Command 处理 GitHub Releases API 调用
- 组件级别实现批量操作 UI

**技术栈:** Vue 3 + Pinia + Naive UI + Tauri 2 + TypeScript + Rust

---

## 文件结构

### 新增文件
- `src/styles/variables.css` - CSS 变量定义
- `src/styles/cyberpunk.css` - Cyberpunk 主题样式
- `src/components/BatchCommandPanel.vue` - 批量命令面板
- `src/components/UpdateDialog.vue` - 更新对话框
- `src/stores/update.ts` - 更新状态管理
- `src-tauri/src/commands/update.rs` - Rust 更新命令

### 修改文件
- `src/App.vue` - 引入主题样式
- `src/components/Sidebar.vue` - 多选支持
- `src/components/TerminalPanel.vue` - cd 命令检测
- `src/components/SftpPanel.vue` - 同步状态
- `src/stores/sftp.ts` - 添加同步方法
- `src/views/MainLayout.vue` - 集成批量面板
- `src-tauri/src/lib.rs` - 注册更新命令
- `src-tauri/Cargo.toml` - 添加依赖

---

## 第一阶段：CSS 主题系统

### Task 1: 创建 CSS 变量文件

**Files:**
- Create: `src/styles/variables.css`

- [ ] **Step 1: 创建 CSS 变量文件**

```css
:root {
  /* 背景色 */
  --bg-primary: #0a0a0f;
  --bg-secondary: #12121a;
  --bg-tertiary: #1a1a2e;
  
  /* 边框色 */
  --border-default: #1a1a2e;
  --border-hover: #00ffff;
  --border-active: #ff00ff;
  
  /* 霓虹色调 */
  --neon-cyan: #00ffff;
  --neon-magenta: #ff00ff;
  --neon-purple: #8b00ff;
  
  /* 文字色 */
  --text-primary: #e0e0e0;
  --text-secondary: #808080;
  
  /* 状态色 */
  --success: #00ff88;
  --error: #ff3366;
  --warning: #ffaa00;
  
  /* 发光效果 */
  --glow-cyan: 0 0 10px #00ffff40;
  --glow-magenta: 0 0 15px #ff00ff60;
  
  /* 终端颜色 */
  --terminal-bg: #000000;
  --terminal-text: #00ff88;
  
  /* 渐变 */
  --gradient-button: linear-gradient(135deg, #00ffff 0%, #8b00ff 100%);
}
```

- [ ] **Step 2: 提交**

```bash
git add src/styles/variables.css
git commit -m "feat: add CSS variables for theme system"
```

---

### Task 2: 创建 Cyberpunk 主题样式

**Files:**
- Create: `src/styles/cyberpunk.css`

- [ ] **Step 1: 创建 Cyberpunk 样式文件**

```css
@import './variables.css';

/* 全局样式 */
.cyberpunk-app {
  background: var(--bg-primary);
  color: var(--text-primary);
}

/* 霓虹边框效果 */
.neon-border {
  border: 1px solid var(--border-default);
  transition: all 0.3s ease;
}

.neon-border:hover {
  border-color: var(--neon-cyan);
  box-shadow: var(--glow-cyan);
}

/* 按钮样式 */
.cyberpunk-button {
  background: var(--gradient-button);
  border: none;
  border-radius: 4px;
  color: #000;
  font-weight: bold;
  padding: 8px 16px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.cyberpunk-button:hover {
  transform: scale(1.02);
  box-shadow: var(--glow-cyan);
}

/* 终端面板 */
.cyberpunk-terminal {
  background: var(--terminal-bg);
  border: 1px solid var(--neon-cyan);
  box-shadow: var(--glow-cyan);
}

/* 侧边栏 */
.cyberpunk-sidebar {
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-default);
}

/* 服务器列表项 */
.server-item {
  padding: 8px 12px;
  border-left: 3px solid transparent;
  transition: all 0.2s ease;
}

.server-item:hover {
  border-left-color: var(--neon-cyan);
  background: rgba(0, 255, 255, 0.1);
}

.server-item.selected {
  background: rgba(0, 255, 255, 0.2);
  border-left-color: var(--neon-cyan);
}

/* 扫描线效果 */
.scanline::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: repeating-linear-gradient(
    0deg,
    transparent,
    transparent 2px,
    rgba(0, 255, 255, 0.03) 2px,
    rgba(0, 255, 255, 0.03) 4px
  );
  pointer-events: none;
}

/* 批量选中状态 */
.server-item.batch-selected {
  background: rgba(0, 255, 255, 0.15);
  border-left: 3px solid var(--neon-cyan);
}
```

- [ ] **Step 2: 提交**

```bash
git add src/styles/cyberpunk.css
git commit -m "feat: add Cyberpunk theme styles"
```

---

## 第二阶段：组件样式改造

### Task 3: 修改 App.vue 引入主题

**Files:**
- Modify: `src/App.vue`

- [ ] **Step 1: 查看当前 App.vue**

```bash
cat src/App.vue
```

- [ ] **Step 2: 添加主题样式引入**

```vue
<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NDialogProvider } from 'naive-ui'
import MainLayout from './views/MainLayout.vue'
</script>

<template>
  <NConfigProvider :theme="darkTheme">
    <NMessageProvider>
      <NDialogProvider>
        <MainLayout class="cyberpunk-app" />
      </NDialogProvider>
    </NMessageProvider>
  </NConfigProvider>
</template>

<style>
@import './styles/cyberpunk.css';

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', system-ui, sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
}
</style>
```

- [ ] **Step 3: 提交**

```bash
git add src/App.vue
git commit -m "feat: import Cyberpunk theme in App.vue"
```

---

### Task 4: 修改 Sidebar.vue 支持多选

**Files:**
- Modify: `src/components/Sidebar.vue`

- [ ] **Step 1: 查看当前 Sidebar.vue**

```bash
cat src/components/Sidebar.vue
```

- [ ] **Step 2: 添加多选状态和批量按钮**

```vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import { NButton, NBadge } from 'naive-ui'
import ServerTree from './ServerTree.vue'
import ServerForm from './ServerForm.vue'
import { useServersStore } from '../stores/servers'

const serversStore = useServersStore()
const showServerForm = ref(false)
const selectedServers = ref<string[]>([])

const selectedCount = computed(() => selectedServers.value.length)

const handleSelectionChange = (keys: string[]) => {
  selectedServers.value = keys
}

const handleBatchConnect = () => {
  selectedServers.value.forEach(id => {
    serversStore.connect(id)
  })
}
</script>

<template>
  <aside class="cyberpunk-sidebar">
    <div class="sidebar-header">
      <NButton @click="showServerForm = true" type="primary" class="cyberpunk-button">
        + 添加服务器
      </NButton>
    </div>
    
    <div class="search-box">
      <input 
        v-model="serversStore.searchQuery"
        placeholder="搜索服务器..."
        class="cyberpunk-input"
      />
    </div>
    
    <div v-if="selectedCount > 0" class="batch-actions">
      <NBadge :value="selectedCount" type="info">
        <NButton size="small" @click="handleBatchConnect">
          批量连接
        </NButton>
      </NBadge>
      <NButton size="small" @click="selectedServers = []">
        取消
      </NButton>
    </div>
    
    <ServerTree
      :selected-keys="selectedServers"
      @update:selected-keys="handleSelectionChange"
      multiple
    />
    
    <ServerForm
      v-model:show="showServerForm"
      @saved="showServerForm = false"
    />
  </aside>
</template>

<style scoped>
.sidebar-header {
  padding: 12px;
  border-bottom: 1px solid var(--border-default);
}

.search-box {
  padding: 8px 12px;
}

.cyberpunk-input {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  color: var(--text-primary);
  border-radius: 4px;
}

.cyberpunk-input:focus {
  outline: none;
  border-color: var(--neon-cyan);
  box-shadow: var(--glow-cyan);
}

.batch-actions {
  padding: 8px 12px;
  display: flex;
  gap: 8px;
  align-items: center;
  background: rgba(0, 255, 255, 0.1);
  border-bottom: 1px solid var(--neon-cyan);
}
</style>
```

- [ ] **Step 3: 提交**

```bash
git add src/components/Sidebar.vue
git commit -m "feat: add multi-select and batch connect to Sidebar"
```

---

## 第三阶段：终端/SFTP 联动

### Task 5: 修改 SFTP Store 添加同步方法

**Files:**
- Modify: `src/stores/sftp.ts`

- [ ] **Step 1: 查看当前 sftp store**

```bash
cat src/stores/sftp.ts
```

- [ ] **Step 2: 添加同步状态和同步方法**

```typescript
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export const useSftpStore = defineStore('sftp', {
  state: () => ({
    sessionId: null as string | null,
    currentPath: '~',
    syncStatus: 'idle' as 'idle' | 'syncing' | 'synced' | 'error',
    lastError: null as string | null,
  }),
  
  actions: {
    async syncFromTerminal(terminalPath: string) {
      this.syncStatus = 'syncing'
      try {
        // 处理相对路径
        let targetPath = terminalPath
        if (!terminalPath.startsWith('/')) {
          // 相对路径，基于当前路径计算
          if (terminalPath === '..') {
            const parts = this.currentPath.split('/')
            parts.pop()
            targetPath = parts.join('/') || '/'
          } else if (terminalPath === '-') {
            // cd - 返回上一个目录，暂时忽略
            this.syncStatus = 'idle'
            return
          } else {
            targetPath = this.currentPath.replace(/[^\/]*$/, '') + terminalPath
          }
        }
        
        // 验证目录是否存在
        await invoke('list_directory', {
          sessionId: this.sessionId,
          path: targetPath
        })
        
        this.currentPath = targetPath
        this.syncStatus = 'synced'
      } catch (e) {
        this.syncStatus = 'error'
        this.lastError = String(e)
      }
    },
    
    async navigateTo(path: string) {
      this.syncStatus = 'syncing'
      try {
        await invoke('list_directory', {
          sessionId: this.sessionId,
          path
        })
        this.currentPath = path
        this.syncStatus = 'synced'
      } catch (e) {
        this.syncStatus = 'error'
        this.lastError = String(e)
      }
    },
    
    async navigateUp() {
      const parts = this.currentPath.split('/')
      parts.pop()
      await this.navigateTo(parts.join('/') || '/')
    }
  }
})
```

- [ ] **Step 3: 提交**

```bash
git add src/stores/sftp.ts
git commit -m "feat: add terminal/SFTP sync methods to sftp store"
```

---

### Task 6: 修改 TerminalPanel 检测 cd 命令

**Files:**
- Modify: `src/components/TerminalPanel.vue`

- [ ] **Step 1: 查看当前 TerminalPanel.vue**

```bash
cat src/components/TerminalPanel.vue
```

- [ ] **Step 2: 添加 cd 命令检测逻辑**

在 TerminalPanel 的 script setup 中添加：

```typescript
import { onMounted, onUnmounted, ref } from 'vue'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import { WebLinksAddon } from 'xterm-addon-web-links'
import { invoke } from '@tauri-apps/api/core'
import { useSftpStore } from '../stores/sftp'
import 'xterm/css/xterm.css'

const sftpStore = useSftpStore()
const terminalRef = ref<HTMLDivElement>()

// cd 命令正则
const CD_PATTERN = /^\s*cd\s+(.+)$/

const handleData = async (data: string) => {
  // 检测 cd 命令
  const match = data.match(CD_PATTERN)
  if (match && sftpStore.sessionId) {
    const targetPath = match[1].trim()
    await sftpStore.syncFromTerminal(targetPath)
  }
  
  // 发送到后端
  const sessionId = props.tab.sessionId
  if (sessionId) {
    await invoke('send_pty_data', {
      sessionId,
      data: new TextEncoder().encode(data)
    })
  }
}

// 在 terminal 实例初始化时添加 data 监听
const initTerminal = () => {
  const terminal = new Terminal({
    theme: { background: '#000000', foreground: '#00ff88' }
  })
  
  terminal.onData(handleData)  // 添加这一行
  // ... 其他初始化
}
```

- [ ] **Step 3: 提交**

```bash
git add src/components/TerminalPanel.vue
git commit -m "feat: detect cd command in terminal for SFTP sync"
```

---

### Task 7: 修改 SftpPanel 显示同步状态

**Files:**
- Modify: `src/components/SftpPanel.vue`

- [ ] **Step 1: 在 SftpPanel 中显示同步状态**

```vue
<script setup lang="ts">
import { computed } from 'vue'
import { NButton, NTag } from 'naive-ui'
import { useSftpStore } from '../stores/sftp'

const sftpStore = useSftpStore()

const syncStatusText = computed(() => {
  switch (sftpStore.syncStatus) {
    case 'synced': return '已同步'
    case 'syncing': return '同步中...'
    case 'error': return '同步失败'
    default: return '--'
  }
})

const syncStatusType = computed(() => {
  switch (sftpStore.syncStatus) {
    case 'synced': return 'success'
    case 'syncing': return 'warning'
    case 'error': return 'error'
    default: return 'default'
  }
})
</script>

<template>
  <div class="sftp-panel cyberpunk-terminal">
    <div class="sftp-toolbar">
      <NButton size="small" @click="sftpStore.navigateUp()">↑</NButton>
      <NButton size="small" @click="sftpStore.listDirectory()">刷新</NButton>
      <span class="current-path">{{ sftpStore.currentPath }}</span>
      <NTag :type="syncStatusType" size="small">
        {{ syncStatusText }}
      </NTag>
    </div>
    <!-- 文件列表 ... -->
  </div>
</template>
```

- [ ] **Step 2: 提交**

```bash
git add src/components/SftpPanel.vue
git commit -m "feat: display SFTP sync status in panel"
```

---

## 第四阶段：批量命令面板

### Task 8: 创建 BatchCommandPanel 组件

**Files:**
- Create: `src/components/BatchCommandPanel.vue`

- [ ] **Step 1: 创建批量命令面板组件**

```vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import { NInput, NButton, NCheckbox, NCollapse, NCollapseItem } from 'naive-ui'
import { useServersStore } from '../stores/servers'
import { useTerminalsStore } from '../stores/terminals'

const serversStore = useServersStore()
const terminalsStore = useTerminalsStore()

const selectedForBatch = ref<string[]>([])
const commandInput = ref('')

const connectedServers = computed(() => {
  return serversStore.servers.filter(s => terminalsStore.hasSession(s.id))
})

const handleSelectAll = () => {
  selectedForBatch.value = connectedServers.value.map(s => s.id)
}

const handleSendBatch = async () => {
  for (const serverId of selectedForBatch.value) {
    const tab = terminalsStore.tabs.find(t => t.serverId === serverId)
    if (tab?.sessionId) {
      await terminalsStore.sendCommand(tab.sessionId, commandInput.value)
    }
  }
}
</script>

<template>
  <NCollapse class="batch-panel">
    <NCollapseItem title="批量命令">
      <div class="batch-content">
        <div class="batch-header">
          <span>已连接服务器 ({{ connectedServers.length }})</span>
          <NButton size="tiny" @click="handleSelectAll">全选</NButton>
        </div>
        
        <div class="server-list">
          <NCheckbox
            v-for="server in connectedServers"
            :key="server.id"
            :value="server.id"
            v-model:checked="selectedForBatch"
          >
            {{ server.name }}
          </NCheckbox>
        </div>
        
        <NInput
          v-model:value="commandInput"
          type="textarea"
          placeholder="输入要发送的命令..."
          :rows="3"
        />
        
        <NButton
          type="primary"
          class="cyberpunk-button"
          :disabled="selectedForBatch.length === 0"
          @click="handleSendBatch"
        >
          发送到 {{ selectedForBatch.length }} 台服务器
        </NButton>
      </div>
    </NCollapseItem>
  </NCollapse>
</template>

<style scoped>
.batch-panel {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
}

.batch-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.server-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
</style>
```

- [ ] **Step 2: 提交**

```bash
git add src/components/BatchCommandPanel.vue
git commit -m "feat: add BatchCommandPanel component"
```

---

## 第五阶段：自动更新功能

### Task 9: 添加 Rust 更新命令

**Files:**
- Create: `src-tauri/src/commands/update.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: 添加依赖到 Cargo.toml**

```toml
[dependencies]
# ... existing deps
reqwest = { version = "0.12", features = ["json"] }
semver = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

- [ ] **Step 2: 创建 update.rs**

```rust
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
    pub release_notes: Option<String>,
    pub download_url: Option<String>,
}

pub async fn check_for_updates() -> Result<UpdateInfo, String> {
    let current_version = env!("CARGO_PKG_VERSION");
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/yangzhen72/sshguard/releases/latest")
        .header("User-Agent", "SSHGuard")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()));
    }
    
    let release: GitHubRelease = response.json().await.map_err(|e| e.to_string())?;
    
    let latest_version = release.tag_name.trim_start_matches('v');
    let has_update = Version::parse(latest_version)
        .map(|v| v > Version::parse(current_version).unwrap_or_else(|_| Version::parse("0.0.0").unwrap()))
        .unwrap_or(false);
    
    let download_url = release.assets.and_then(|assets| {
        assets.into_iter().find(|a| a.name.contains("setup.exe")).map(|a| a.browser_download_url)
    });
    
    Ok(UpdateInfo {
        current_version: current_version.to_string(),
        latest_version: latest_version.to_string(),
        has_update,
        release_notes: release.body,
        download_url,
    })
}
```

- [ ] **Step 3: 修改 lib.rs 注册命令**

```rust
mod commands;
mod storage;
mod ssh;
mod sftp;

pub use commands::update;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ... existing code
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // ... existing handlers
            commands::update::check_for_updates,
        ])
        // ...
}
```

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/commands/update.rs src-tauri/src/lib.rs src-tauri/Cargo.toml
git commit -m "feat: add GitHub releases update checker"
```

---

### Task 10: 创建 Update Store 和 Dialog

**Files:**
- Create: `src/stores/update.ts`
- Create: `src/components/UpdateDialog.vue`

- [ ] **Step 1: 创建 update store**

```typescript
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

interface UpdateInfo {
  current_version: string
  latest_version: string
  has_update: boolean
  release_notes: string | null
  download_url: string | null
}

export const useUpdateStore = defineStore('update', {
  state: () => ({
    updateInfo: null as UpdateInfo | null,
    isChecking: false,
    isDownloading: false,
    downloadProgress: 0,
    showDialog: false,
  }),
  
  actions: {
    async checkForUpdates() {
      this.isChecking = true
      try {
        const info = await invoke<UpdateInfo>('check_for_updates')
        this.updateInfo = info
        if (info.has_update) {
          this.showDialog = true
        }
      } catch (e) {
        console.error('Update check failed:', e)
      } finally {
        this.isChecking = false
      }
    },
    
    closeDialog() {
      this.showDialog = false
    },
    
    dismissUpdate() {
      this.showDialog = false
      localStorage.setItem('update_dismissed', this.updateInfo?.latest_version)
    }
  }
})
```

- [ ] **Step 2: 创建 UpdateDialog 组件**

```vue
<script setup lang="ts">
import { NModal, NCard, NButton, NProgress, NSpace } from 'naive-ui'
import { useUpdateStore } from '../stores/update'

const updateStore = useUpdateStore()
</script>

<template>
  <NModal v-model:show="updateStore.showDialog">
    <NCard
      title="🎉 新版本可用"
      :bordered="false"
      class="update-dialog"
    >
      <template #header-extra>
        <span class="version-info">
          {{ updateStore.updateInfo?.current_version }} → {{ updateStore.updateInfo?.latest_version }}
        </span>
      </template>
      
      <div class="update-content">
        <h4>更新内容:</h4>
        <pre class="release-notes">{{ updateStore.updateInfo?.release_notes || '暂无更新说明' }}</pre>
        
        <NProgress
          v-if="updateStore.isDownloading"
          type="line"
          :percentage="updateStore.downloadProgress"
          indicator="outside"
        />
      </div>
      
      <template #footer>
        <NSpace justify="end">
          <NButton @click="updateStore.dismissUpdate">忽略此版本</NButton>
          <NButton @click="updateStore.closeDialog">稍后提醒</NButton>
          <NButton
            type="primary"
            class="cyberpunk-button"
            :loading="updateStore.isDownloading"
            @click="handleUpdate"
          >
            立即更新
          </NButton>
        </NSpace>
      </template>
    </NCard>
  </NModal>
</template>

<style scoped>
.update-dialog {
  max-width: 500px;
}

.version-info {
  color: var(--text-secondary);
  font-size: 12px;
}

.update-content {
  margin: 16px 0;
}

.release-notes {
  background: var(--bg-primary);
  padding: 12px;
  border-radius: 4px;
  white-space: pre-wrap;
  max-height: 200px;
  overflow-y: auto;
  font-size: 13px;
}
</style>
```

- [ ] **Step 3: 提交**

```bash
git add src/stores/update.ts src/components/UpdateDialog.vue
git commit -m "feat: add update dialog and store"
```

---

## 第六阶段：集成和测试

### Task 11: 集成批量面板到 MainLayout

**Files:**
- Modify: `src/views/MainLayout.vue`

- [ ] **Step 1: 添加 BatchCommandPanel 到布局**

```vue
<script setup lang="ts">
import Sidebar from '../components/Sidebar.vue'
import TabBar from '../components/TabBar.vue'
import TerminalPanel from '../components/TerminalPanel.vue'
import SftpPanel from '../components/SftpPanel.vue'
import BatchCommandPanel from '../components/BatchCommandPanel.vue'
import StatusBar from '../components/StatusBar.vue'
import UpdateDialog from '../components/UpdateDialog.vue'
import { useUpdateStore } from '../stores/update'
import { onMounted } from 'vue'

const updateStore = useUpdateStore()

onMounted(() => {
  // 检查更新
  updateStore.checkForUpdates()
})
</script>

<template>
  <div class="main-layout">
    <Sidebar class="sidebar" />
    
    <div class="content">
      <TabBar />
      <TerminalPanel class="terminal-area" />
      <SftpPanel class="sftp-area" />
      <BatchCommandPanel class="batch-panel" />
    </div>
    
    <StatusBar />
    <UpdateDialog />
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
}

.sidebar {
  width: 280px;
  flex-shrink: 0;
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.terminal-area {
  flex: 1;
}

.sftp-area {
  height: 300px;
  border-top: 1px solid var(--border-default);
}

.batch-panel {
  position: fixed;
  bottom: 32px;
  right: 16px;
  width: 320px;
}
</style>
```

- [ ] **Step 2: 提交**

```bash
git add src/views/MainLayout.vue
git commit -m "feat: integrate batch panel and update dialog"
```

---

### Task 12: 最终构建测试

- [ ] **Step 1: 运行前端构建**

```bash
npm run build
```

- [ ] **Step 2: 运行 Tauri 构建**

```bash
npm run tauri build -- --target x86_64-pc-windows-msvc
```

- [ ] **Step 3: 验证构建产物**

```bash
ls -la src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/
ls -la src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/
```

- [ ] **Step 4: 提交所有更改**

```bash
git status
git log --oneline -20
```

---

## 总结

以上计划包含 12 个任务，分 6 个阶段实施：

1. **CSS 主题系统** (Task 1-2)
2. **组件样式改造** (Task 3-4)
3. **终端/SFTP 联动** (Task 5-7)
4. **批量命令面板** (Task 8)
5. **自动更新功能** (Task 9-10)
6. **集成测试** (Task 11-12)

每个任务都有清晰的步骤和提交点，确保增量式开发。
