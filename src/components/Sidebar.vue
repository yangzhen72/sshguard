<script setup lang="ts">
import { ref } from 'vue';
import { NInput, NButton, NTooltip } from 'naive-ui';
import ServerTree from './ServerTree.vue';
import ServerForm from './ServerForm.vue';

const searchQuery = ref('');
const showAddDialog = ref(false);

const handleAddServer = () => {
  showAddDialog.value = true;
};
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <n-input 
        v-model:value="searchQuery" 
        placeholder="Search servers..."
        clearable
        size="small"
      >
        <template #prefix>
          <span>🔍</span>
        </template>
      </n-input>
      <n-tooltip trigger="hover">
        <template #trigger>
          <n-button size="small" @click="handleAddServer">+</n-button>
        </template>
        Add Server
      </n-tooltip>
    </div>
    
    <div class="server-list">
      <ServerTree :search="searchQuery" />
    </div>
    
    <div class="sidebar-footer">
      <n-button text style="width: 100%">⚙️ Settings</n-button>
    </div>
    
    <n-modal v-model:show="showAddDialog">
      <ServerForm @close="showAddDialog = false" />
    </n-modal>
  </div>
</template>

<style scoped>
.sidebar {
  width: 250px;
  min-width: 200px;
  max-width: 400px;
  background: #1e1e3f;
  border-right: 1px solid #2a2a5e;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-header {
  padding: 12px;
  display: flex;
  gap: 8px;
  border-bottom: 1px solid #2a2a5e;
}

.server-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.sidebar-footer {
  padding: 12px;
  border-top: 1px solid #2a2a5e;
}
</style>