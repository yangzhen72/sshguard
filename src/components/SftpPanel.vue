<script setup lang="ts">
import { useSftpStore } from '../stores/sftp';
import { NButton, NProgress, NSpace } from 'naive-ui';

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
</script>

<template>
  <div class="sftp-panel">
    <div class="sftp-toolbar">
      <n-space>
        <n-button size="small" @click="sftpStore.navigateUp()">↑ Up</n-button>
        <n-button size="small" @click="sftpStore.listDirectory(sftpStore.currentPath)">↻ Refresh</n-button>
      </n-space>
      <span class="current-path">{{ sftpStore.currentPath }}</span>
    </div>
    
    <div class="file-list">
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Size</th>
            <th>Modified</th>
            <th>Permissions</th>
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
      <span>{{ sftpStore.files.length }} items</span>
    </div>
  </div>
</template>

<style scoped>
.sftp-panel {
  width: 350px;
  min-width: 250px;
  background: #1e1e3f;
  border-left: 1px solid #2a2a5e;
  display: flex;
  flex-direction: column;
}

.sftp-toolbar {
  padding: 8px;
  border-bottom: 1px solid #2a2a5e;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.current-path {
  font-size: 12px;
  color: #888;
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
  border-bottom: 1px solid #2a2a5e;
}

th {
  background: #16162a;
  font-weight: 500;
  position: sticky;
  top: 0;
}

tr {
  cursor: pointer;
}

tr:hover {
  background: rgba(255, 255, 255, 0.05);
}

tr.selected {
  background: rgba(64, 158, 255, 0.2);
}

.file-icon {
  margin-right: 8px;
}

.sftp-status {
  padding: 8px;
  border-top: 1px solid #2a2a5e;
  font-size: 12px;
  color: #888;
}

.loading {
  padding: 20px;
  display: flex;
  justify-content: center;
}
</style>
