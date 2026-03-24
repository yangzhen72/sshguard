<script setup lang="ts">
import { computed } from 'vue';
import { useTerminalsStore } from '../stores/terminals';
import { useSftpStore } from '../stores/sftp';

const terminalsStore = useTerminalsStore();
const sftpStore = useSftpStore();

const connectedCount = computed(() => terminalsStore.tabs.length);
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
      <span class="status-item">v0.2.0</span>
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
</style>
