<script setup lang="ts">
import { ref } from 'vue';
import Sidebar from '../components/Sidebar.vue';
import TabBar from '../components/TabBar.vue';
import TerminalPanel from '../components/TerminalPanel.vue';
import SftpPanel from '../components/SftpPanel.vue';
import StatusBar from '../components/StatusBar.vue';
import UpdateDialog from '../components/UpdateDialog.vue';
import { useUpdateStore } from '../stores/update';

const updateStore = useUpdateStore();
const sftpExpanded = ref(false);

const toggleSftp = () => {
  sftpExpanded.value = !sftpExpanded.value;
};

updateStore.checkForUpdates();
</script>

<template>
  <div class="app-container">
    <!-- 顶部工具栏 -->
    <header class="toolbar">
      <div class="toolbar-logo">
        <span style="font-size: 18px;">🛡️</span>
        <span>SSHGuard</span>
      </div>
      <div class="toolbar-actions">
        <button class="btn" title="新建连接">+</button>
        <button class="btn" title="全局搜索">🔍</button>
      </div>
    </header>
    
    <!-- 主体布局 -->
    <div class="main-layout">
      <!-- 左侧边栏 -->
      <Sidebar />
      
      <!-- 内容区 -->
      <div class="content-area">
        <!-- 标签栏 -->
        <TabBar />
        
        <!-- 终端区域 -->
        <div class="terminal-wrapper">
          <TerminalPanel />
          
          <!-- SFTP 面板 (可折叠) -->
          <div class="sftp-panel" :class="{ expanded: sftpExpanded }">
            <div class="sftp-panel-header">
              <div class="sftp-toolbar">
                <button @click="toggleSftp" :class="{ active: sftpExpanded }">
                  {{ sftpExpanded ? '▼' : '▲' }} SFTP
                </button>
              </div>
              <SftpPanel v-if="sftpExpanded" />
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 状态栏 -->
    <StatusBar />
    
    <!-- 更新对话框 -->
    <UpdateDialog />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
}

.toolbar {
  height: var(--toolbar-height);
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-default);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-md);
}

.toolbar-logo {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-weight: 600;
  font-size: 14px;
  color: var(--accent-primary);
}

.toolbar-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.main-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.terminal-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.sftp-panel {
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-default);
  overflow: hidden;
  transition: height 0.3s ease;
}

.sftp-panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-md);
  height: var(--tabbar-height);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-default);
}

.sftp-panel-header .sftp-toolbar button {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.15s ease;
}

.sftp-panel-header .sftp-toolbar button:hover,
.sftp-panel-header .sftp-toolbar button.active {
  background: var(--bg-hover);
  color: var(--accent-primary);
  border-color: var(--accent-primary);
}
</style>
