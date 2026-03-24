<script setup lang="ts">
import { ref, computed } from 'vue';
import { NInput, NButton, NTooltip, NBadge, NSpace } from 'naive-ui';
import ServerTree from './ServerTree.vue';
import ServerForm from './ServerForm.vue';
import { useServersStore } from '../stores/servers';
import { useTerminalsStore } from '../stores/terminals';

const serversStore = useServersStore();
const terminalsStore = useTerminalsStore();
const searchQuery = ref('');
const showAddDialog = ref(false);
const selectedServers = ref<string[]>([]);

const selectedCount = computed(() => selectedServers.value.length);

const handleAddServer = () => {
  showAddDialog.value = true;
};

const handleSelectionChange = (keys: string[]) => {
  selectedServers.value = keys.filter(k => !k.includes('/'));
};

const handleBatchConnect = () => {
  selectedServers.value.forEach(id => {
    const server = serversStore.servers.find(s => s.id === id);
    if (server) {
      terminalsStore.createTab(server);
    }
  });
};

const handleClearSelection = () => {
  selectedServers.value = [];
};
</script>

<template>
  <aside class="sidebar cyberpunk-sidebar">
    <div class="sidebar-header">
      <n-input 
        v-model:value="searchQuery" 
        placeholder="搜索服务器..."
        clearable
        size="small"
        class="cyberpunk-input"
      >
        <template #prefix>
          <span>🔍</span>
        </template>
      </n-input>
      <n-tooltip trigger="hover">
        <template #trigger>
          <n-button size="small" @click="handleAddServer" class="cyberpunk-button">+</n-button>
        </template>
        添加服务器
      </n-tooltip>
    </div>
    
    <div v-if="selectedCount > 0" class="batch-actions">
      <n-space align="center">
        <n-badge :value="selectedCount" type="info">
          <n-button size="small" @click="handleBatchConnect" class="cyberpunk-button">
            批量连接
          </n-button>
        </n-badge>
        <n-button size="tiny" @click="handleClearSelection">
          取消
        </n-button>
      </n-space>
    </div>
    
    <div class="server-list">
      <ServerTree 
        :search="searchQuery"
        :selected-keys="selectedServers"
        @update:selected-keys="handleSelectionChange"
        multiple
      />
    </div>
    
    <div class="sidebar-footer">
      <n-button text style="width: 100%">⚙️ 设置</n-button>
    </div>
    
    <n-modal v-model:show="showAddDialog">
      <ServerForm @close="showAddDialog = false" />
    </n-modal>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 280px;
  min-width: 200px;
  max-width: 400px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-header {
  padding: 12px;
  display: flex;
  gap: 8px;
  border-bottom: 1px solid var(--border-default);
}

.server-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.sidebar-footer {
  padding: 12px;
  border-top: 1px solid var(--border-default);
}

.batch-actions {
  padding: 8px 12px;
  background: rgba(0, 255, 255, 0.1);
  border-bottom: 1px solid var(--neon-cyan);
}
</style>
