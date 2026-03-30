<script setup lang="ts">
import { computed } from 'vue';
import { useTerminalsStore } from '../stores/terminals';
import { useSftpStore } from '../stores/sftp';
import { useUpdateStore } from '../stores/update';

const terminalsStore = useTerminalsStore();
const sftpStore = useSftpStore();
const updateStore = useUpdateStore();

const connectedCount = computed(() => terminalsStore.tabs.length);

const checkUpdate = () => {
  updateStore.checkForUpdates();
};

const updateBtnText = () => {
  return updateStore.isChecking ? '...' : '↻';
};
</script>

<template>
  <footer class="status-bar">
    <div class="status-left">
      <span class="status-item">
        <span class="indicator" :class="{ active: connectedCount > 0 }"></span>
        <span>{{ connectedCount > 0 ? `${connectedCount} 个连接` : '未连接' }}</span>
      </span>
    </div>
    
    <div class="status-center">
      <span v-if="sftpStore.syncStatus === 'syncing'" class="status-item syncing">
        🔄 同步中...
      </span>
      <span v-else-if="sftpStore.syncStatus === 'synced'" class="status-item synced">
        ✓ 已同步
      </span>
    </div>
    
    <div class="status-right">
      <span class="status-item">UTF-8</span>
      <span class="status-item version">v0.4.0</span>
      <button class="update-btn" @click="checkUpdate" :disabled="updateStore.isChecking" title="检查更新">
        {{ updateBtnText() }} 更新
      </button>
    </div>
  </footer>
</template>

<style scoped>
.status-bar {
  height: var(--statusbar-height);
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-default);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-md);
  font-size: 11px;
  color: var(--text-secondary);
}

.status-left,
.status-center,
.status-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.status-left {
  flex: 1;
}

.status-center {
  flex: 1;
  justify-content: center;
}

.status-right {
  flex: 1;
  justify-content: flex-end;
}

.status-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-muted);
}

.indicator.active {
  background: var(--success);
}

.syncing {
  color: var(--warning);
}

.synced {
  color: var(--success);
}

.version {
  color: var(--text-muted);
}

.update-btn {
  background: transparent;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 11px;
  padding: 2px 6px;
  transition: all 0.15s ease;
}

.update-btn:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--accent-primary);
  border-color: var(--accent-primary);
}

.update-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
