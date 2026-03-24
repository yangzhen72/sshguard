<script setup lang="ts">
import { computed } from 'vue';
import { useSftpStore } from '../stores/sftp';

const sftpStore = useSftpStore();

const formatSize = (bytes: number) => {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  return (bytes / (1024 * 1024 * 1024)).toFixed(1) + ' GB';
};

const formatDate = (ts: number) => {
  return new Date(ts * 1000).toLocaleDateString();
};

const handleFileClick = (file: any) => {
  sftpStore.selectedFile = file.path;
};

const handleFileDoubleClick = (file: any) => {
  if (file.isDirectory) {
    sftpStore.navigateTo(file.path);
  }
};

const syncStatusText = computed(() => {
  switch (sftpStore.syncStatus) {
    case 'synced': return '已同步';
    case 'syncing': return '同步中...';
    case 'error': return '同步失败';
    default: return '';
  }
});
</script>

<template>
  <div class="sftp-content">
    <!-- 工具栏 -->
    <div class="sftp-toolbar-row">
      <button class="toolbar-btn" @click="sftpStore.navigateUp()" title="上级目录">
        ⬆️
      </button>
      <button class="toolbar-btn" @click="sftpStore.listDirectory(sftpStore.currentPath)" title="刷新">
        🔄
      </button>
      <span class="sftp-path">{{ sftpStore.currentPath }}</span>
      <span v-if="syncStatusText" class="sync-status" :class="sftpStore.syncStatus">
        {{ syncStatusText }}
      </span>
    </div>
    
    <!-- 文件列表 -->
    <div class="sftp-table-wrapper">
      <table class="sftp-table">
        <thead>
          <tr>
            <th>名称</th>
            <th>大小</th>
            <th>修改时间</th>
            <th>权限</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="file in sftpStore.files"
            :key="file.path"
            :class="{ directory: file.isDirectory, selected: sftpStore.selectedFile === file.path }"
            @click="handleFileClick(file)"
            @dblclick="handleFileDoubleClick(file)"
          >
            <td>
              <span class="file-icon">{{ file.isDirectory ? '📁' : '📄' }}</span>
              <span class="file-name">{{ file.name }}</span>
            </td>
            <td class="file-size">{{ file.isDirectory ? '-' : formatSize(file.size) }}</td>
            <td class="file-date">{{ formatDate(file.modified) }}</td>
            <td class="file-perms">{{ file.permissions }}</td>
          </tr>
        </tbody>
      </table>
      
      <!-- 空状态 -->
      <div v-if="sftpStore.files.length === 0 && !sftpStore.isLoading" class="empty-state">
        <span>目录为空</span>
      </div>
      
      <!-- 加载状态 -->
      <div v-if="sftpStore.isLoading" class="loading-state">
        <span>加载中...</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sftp-content {
  height: calc(100% - 36px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sftp-toolbar-row {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-default);
}

.toolbar-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.15s ease;
}

.toolbar-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}

.sftp-path {
  flex: 1;
  font-size: 12px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sync-status {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
  background: var(--bg-tertiary);
}

.sync-status.synced {
  color: var(--success);
}

.sync-status.syncing {
  color: var(--warning);
}

.sync-status.error {
  color: var(--error);
}

.sftp-table-wrapper {
  flex: 1;
  overflow: auto;
}

.sftp-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.sftp-table th,
.sftp-table td {
  padding: var(--spacing-sm) var(--spacing-md);
  text-align: left;
  border-bottom: 1px solid var(--border-default);
  white-space: nowrap;
}

.sftp-table th {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  font-weight: 500;
  position: sticky;
  top: 0;
  z-index: 1;
}

.sftp-table tr:hover td {
  background: var(--bg-hover);
}

.sftp-table td:first-child {
  color: var(--text-primary);
}

.sftp-table tr.selected td {
  background: rgba(0, 152, 255, 0.1);
}

.sftp-table tr.selected td:first-child {
  color: var(--accent-primary);
}

.file-icon {
  margin-right: var(--spacing-xs);
}

.file-name {
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-size,
.file-date,
.file-perms {
  color: var(--text-secondary);
  font-size: 12px;
}

.file-perms {
  font-family: var(--font-mono);
}

.empty-state,
.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  color: var(--text-muted);
  font-size: 13px;
}
</style>
