<script setup lang="ts">
import { computed } from 'vue';
import { useSftpStore } from '../stores/sftp';
import { NButton, NProgress, NSpace, NTag } from 'naive-ui';

const sftpStore = useSftpStore();

const formatSize = (bytes: number) => {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
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
    default: return '--';
  }
});

const syncStatusType = computed(() => {
  switch (sftpStore.syncStatus) {
    case 'synced': return 'success';
    case 'syncing': return 'warning';
    case 'error': return 'error';
    default: return 'default';
  }
});
</script>

<template>
  <div class="sftp-panel cyberpunk-terminal">
    <div class="sftp-toolbar">
      <n-space>
        <n-button size="small" @click="sftpStore.navigateUp()" class="neon-border">↑</n-button>
        <n-button size="small" @click="sftpStore.listDirectory(sftpStore.currentPath)" class="neon-border">↻</n-button>
      </n-space>
      <span class="current-path">{{ sftpStore.currentPath }}</span>
      <n-tag :type="syncStatusType" size="small" class="sync-tag">
        {{ syncStatusText }}
      </n-tag>
    </div>
    
    <div class="file-list">
      <table>
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
              {{ file.name }}
            </td>
            <td>{{ file.isDirectory ? '-' : formatSize(file.size) }}</td>
            <td>{{ formatDate(file.modified) }}</td>
            <td>{{ file.permissions }}</td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="sftpStore.isLoading" class="loading">
        <n-progress type="line" />
      </div>
    </div>
    
    <div class="sftp-status">
      <span>{{ sftpStore.files.length }} 个项目</span>
    </div>
  </div>
</template>

<style scoped>
.sftp-panel {
  width: 350px;
  min-width: 250px;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
}

.sftp-toolbar {
  padding: 8px;
  border-bottom: 1px solid var(--border-default);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.current-path {
  font-size: 12px;
  color: var(--text-secondary);
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.sync-tag {
  font-size: 10px;
}

.file-list {
  flex: 1;
  overflow-y: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  text-align: left;
  padding: 8px;
  border-bottom: 1px solid var(--border-default);
}

th {
  background: var(--bg-tertiary);
  font-weight: 500;
  position: sticky;
  top: 0;
  color: var(--text-primary);
}

tr {
  cursor: pointer;
}

tr:hover {
  background: rgba(0, 255, 255, 0.05);
}

tr.selected {
  background: rgba(0, 255, 255, 0.15);
}

.file-icon {
  margin-right: 8px;
}

.sftp-status {
  padding: 8px;
  border-top: 1px solid var(--border-default);
  font-size: 12px;
  color: var(--text-secondary);
}

.loading {
  padding: 20px;
  display: flex;
  justify-content: center;
}
</style>
