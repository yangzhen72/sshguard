<script setup lang="ts">
import { ref, computed } from 'vue';
import { useServersStore } from '../stores/servers';
import { useTerminalsStore } from '../stores/terminals';
import { invoke } from '@tauri-apps/api/core';

const serversStore = useServersStore();
const terminalsStore = useTerminalsStore();

const selectedForBatch = ref<string[]>([]);
const commandInput = ref('');

const connectedServers = computed(() => {
  return serversStore.servers.filter(s => 
    terminalsStore.tabs.some(t => t.serverId === s.id)
  );
});

const handleSelectAll = () => {
  selectedForBatch.value = connectedServers.value.map(s => s.id);
};

const handleToggleServer = (serverId: string) => {
  const idx = selectedForBatch.value.indexOf(serverId);
  if (idx >= 0) {
    selectedForBatch.value.splice(idx, 1);
  } else {
    selectedForBatch.value.push(serverId);
  }
};

const handleBatchConnect = () => {
  selectedForBatch.value.forEach(serverId => {
    const server = serversStore.servers.find(s => s.id === serverId);
    if (server && !terminalsStore.tabs.some(t => t.serverId === serverId)) {
      terminalsStore.createTab(server);
    }
  });
};

const handleSendBatch = async () => {
  for (const serverId of selectedForBatch.value) {
    const tab = terminalsStore.tabs.find(t => t.serverId === serverId);
    if (tab) {
      const encoder = new TextEncoder();
      const data = encoder.encode(commandInput.value + '\n');
      await invoke('send_pty_data', { 
        sessionId: tab.sessionId, 
        data: Array.from(data) 
      });
    }
  }
  commandInput.value = '';
};
</script>

<template>
  <div class="batch-panel">
    <div class="batch-header">
      <span class="batch-title">📡 批量操作</span>
      <span v-if="selectedForBatch.length > 0" class="batch-count">
        {{ selectedForBatch.length }} 台
      </span>
    </div>
    
    <div class="batch-section">
      <div class="section-label">已连接服务器</div>
      <div class="server-list">
        <div 
          v-for="server in connectedServers" 
          :key="server.id"
          class="server-item"
          :class="{ selected: selectedForBatch.includes(server.id) }"
          @click="handleToggleServer(server.id)"
        >
          <span class="checkbox" :class="{ checked: selectedForBatch.includes(server.id) }">
            {{ selectedForBatch.includes(server.id) ? '✓' : '' }}
          </span>
          <span class="server-name">{{ server.name }}</span>
        </div>
        <div v-if="connectedServers.length === 0" class="empty-tip">
          暂无已连接服务器
        </div>
      </div>
      <button 
        v-if="connectedServers.length > 0" 
        class="btn-link"
        @click="handleSelectAll"
      >
        全选
      </button>
    </div>
    
    <div class="batch-section">
      <button 
        class="btn"
        :disabled="selectedForBatch.length === 0"
        @click="handleBatchConnect"
      >
        连接 {{ selectedForBatch.length }} 台服务器
      </button>
    </div>
    
    <div class="batch-section">
      <div class="section-label">发送命令</div>
      <textarea
        v-model="commandInput"
        placeholder="输入要发送的命令..."
        class="command-input"
        rows="3"
      ></textarea>
      <button 
        class="btn primary"
        :disabled="selectedForBatch.length === 0 || !commandInput"
        @click="handleSendBatch"
      >
        发送到 {{ selectedForBatch.length }} 台服务器
      </button>
    </div>
  </div>
</template>

<style scoped>
.batch-panel {
  width: 280px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  box-shadow: var(--shadow-lg);
}

.batch-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px solid var(--border-default);
}

.batch-title {
  font-weight: 600;
  color: var(--accent-primary);
  font-size: 14px;
}

.batch-count {
  font-size: 12px;
  padding: 2px 8px;
  background: rgba(0, 152, 255, 0.2);
  color: var(--accent-primary);
  border-radius: 10px;
}

.batch-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.section-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.server-list {
  max-height: 120px;
  overflow-y: auto;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
}

.server-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  cursor: pointer;
  transition: background 0.15s ease;
}

.server-item:hover {
  background: var(--bg-hover);
}

.server-item.selected {
  background: rgba(0, 152, 255, 0.1);
}

.checkbox {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--border-default);
  border-radius: 3px;
  font-size: 10px;
  color: var(--accent-primary);
}

.checkbox.checked {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: #fff;
}

.server-name {
  font-size: 13px;
  color: var(--text-primary);
}

.empty-tip {
  text-align: center;
  padding: var(--spacing-md);
  color: var(--text-muted);
  font-size: 12px;
}

.btn-link {
  background: none;
  border: none;
  color: var(--accent-primary);
  font-size: 12px;
  cursor: pointer;
  text-align: left;
  padding: 0;
}

.btn-link:hover {
  text-decoration: underline;
}

.command-input {
  width: 100%;
  padding: var(--spacing-sm);
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 13px;
  font-family: inherit;
  resize: none;
}

.command-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.command-input::placeholder {
  color: var(--text-muted);
}

.btn {
  width: 100%;
  padding: var(--spacing-sm);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.15s ease;
}

.btn:hover:not(:disabled) {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.primary {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: #fff;
}

.btn.primary:hover:not(:disabled) {
  background: var(--accent-hover);
}
</style>
