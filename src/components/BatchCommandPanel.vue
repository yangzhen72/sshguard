<script setup lang="ts">
import { ref, computed } from 'vue';
import { NInput, NButton, NTag } from 'naive-ui';
import { useServersStore } from '../stores/servers';
import { useTerminalsStore } from '../stores/terminals';

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
      await fetch(`send_pty_data`, {
        method: 'POST',
        body: JSON.stringify({ sessionId: tab.sessionId, data: Array.from(data) })
      });
    }
  }
  commandInput.value = '';
};
</script>

<template>
  <div class="batch-panel cyberpunk-sidebar">
    <div class="batch-header">
      <span class="batch-title">📡 批量操作</span>
      <NTag v-if="selectedForBatch.length > 0" type="info" size="small">
        {{ selectedForBatch.length }} 台
      </NTag>
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
          <Checkbox 
            :checked="selectedForBatch.includes(server.id)" 
            size="small"
          />
          <span class="server-name">{{ server.name }}</span>
        </div>
        <div v-if="connectedServers.length === 0" class="empty-tip">
          暂无已连接服务器
        </div>
      </div>
      <NButton 
        v-if="connectedServers.length > 0" 
        size="tiny" 
        @click="handleSelectAll"
        class="select-all-btn"
      >
        全选
      </NButton>
    </div>
    
    <div class="batch-section">
      <div class="section-label">批量连接</div>
      <NButton 
        class="cyberpunk-button batch-connect-btn"
        :disabled="selectedForBatch.length === 0"
        @click="handleBatchConnect"
      >
        连接 {{ selectedForBatch.length }} 台服务器
      </NButton>
    </div>
    
    <div class="batch-section">
      <div class="section-label">发送命令</div>
      <NInput
        v-model:value="commandInput"
        type="textarea"
        placeholder="输入要发送的命令..."
        :rows="3"
        class="cyberpunk-input"
      />
      <NButton 
        class="cyberpunk-button send-btn"
        :disabled="selectedForBatch.length === 0 || !commandInput"
        @click="handleSendBatch"
      >
        发送到 {{ selectedForBatch.length }} 台服务器
      </NButton>
    </div>
  </div>
</template>

<style scoped>
.batch-panel {
  width: 280px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.batch-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-default);
}

.batch-title {
  font-weight: bold;
  color: var(--neon-cyan);
}

.batch-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-label {
  font-size: 12px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.server-list {
  max-height: 120px;
  overflow-y: auto;
  border: 1px solid var(--border-default);
  border-radius: 4px;
  padding: 4px;
}

.server-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.server-item:hover {
  background: rgba(0, 255, 255, 0.1);
}

.server-item.selected {
  background: rgba(0, 255, 255, 0.15);
}

.server-name {
  font-size: 13px;
  color: var(--text-primary);
}

.empty-tip {
  text-align: center;
  padding: 12px;
  color: var(--text-secondary);
  font-size: 12px;
}

.select-all-btn {
  align-self: flex-start;
}

.batch-connect-btn {
  width: 100%;
}

.send-btn {
  width: 100%;
}
</style>
