<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import ServerTree from './ServerTree.vue';
import ServerForm from './ServerForm.vue';
import { useServersStore } from '../stores/servers';
import { useTerminalsStore } from '../stores/terminals';

const serversStore = useServersStore();
const terminalsStore = useTerminalsStore();
const searchQuery = ref('');
const showServerForm = ref(false);
const selectedServers = ref<string[]>([]);

const selectedCount = computed(() => selectedServers.value.length);

const handleSelectionChange = (keys: string[]) => {
  selectedServers.value = keys;
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

const handleAddServer = () => {
  showServerForm.value = true;
};

const handleConnectServer = (serverId: string) => {
  const server = serversStore.servers.find(s => s.id === serverId);
  if (server) {
    terminalsStore.createTab(server);
  }
};

watch(() => serversStore.servers, () => {
  // Force re-render when servers change
}, { deep: true });
</script>

<template>
  <aside class="sidebar">
    <!-- 搜索框 -->
    <div class="sidebar-header">
      <div class="sidebar-search">
        <span class="sidebar-search-icon">🔍</span>
        <input 
          v-model="searchQuery"
          type="text"
          placeholder="搜索服务器..."
        />
      </div>
    </div>
    
    <!-- 服务器列表 -->
    <div class="sidebar-content">
      <ServerTree 
        :search="searchQuery"
        :selected-keys="selectedServers"
        @update:selected-keys="handleSelectionChange"
        @connect="handleConnectServer"
      />
    </div>
    
    <!-- 批量操作栏 -->
    <div v-if="selectedCount > 0" class="batch-bar">
      <span class="batch-info">{{ selectedCount }} 已选中</span>
      <button class="batch-btn primary" @click="handleBatchConnect">
        批量连接
      </button>
      <button class="batch-btn" @click="handleClearSelection">
        取消
      </button>
    </div>
    
    <!-- 底部按钮 -->
    <div class="sidebar-footer">
      <button class="footer-btn" @click="handleAddServer" title="添加服务器">
        ➕ 添加
      </button>
      <button class="footer-btn" title="设置">
        ⚙️ 设置
      </button>
    </div>
    
    <!-- 添加服务器弹窗 -->
    <ServerForm 
      v-model:show="showServerForm"
      @close="showServerForm = false"
    />
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  min-width: var(--sidebar-min-width);
  max-width: var(--sidebar-max-width);
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-header {
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--border-default);
}

.sidebar-search {
  position: relative;
}

.sidebar-search input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  padding-left: 32px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 13px;
  transition: border-color 0.2s ease;
}

.sidebar-search input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.sidebar-search input::placeholder {
  color: var(--text-muted);
}

.sidebar-search-icon {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 12px;
}

.sidebar-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-sm);
}

.batch-bar {
  padding: var(--spacing-sm) var(--spacing-md);
  background: rgba(0, 152, 255, 0.1);
  border-top: 1px solid var(--accent-primary);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.batch-info {
  flex: 1;
  font-size: 12px;
  color: var(--accent-primary);
}

.batch-btn {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.15s ease;
}

.batch-btn:hover {
  background: var(--bg-hover);
}

.batch-btn.primary {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: #fff;
}

.batch-btn.primary:hover {
  background: var(--accent-hover);
}

.sidebar-footer {
  padding: var(--spacing-sm);
  border-top: 1px solid var(--border-default);
  display: flex;
  gap: var(--spacing-xs);
}

.footer-btn {
  flex: 1;
  padding: var(--spacing-sm);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.15s ease;
}

.footer-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}
</style>
