<script setup lang="ts">
import { useTerminalsStore } from '../stores/terminals';

const terminalsStore = useTerminalsStore();

const handleTabClick = (tabId: string) => {
  terminalsStore.setActiveTab(tabId);
};

const handleCloseTab = (tabId: string, event: MouseEvent) => {
  event.stopPropagation();
  terminalsStore.closeTab(tabId);
};
</script>

<template>
  <div class="tab-bar">
    <div
      v-for="tab in terminalsStore.tabs"
      :key="tab.id"
      class="tab"
      :class="{ active: tab.id === terminalsStore.activeTabId }"
      @click="handleTabClick(tab.id)"
    >
      <span class="tab-icon">💻</span>
      <span class="tab-title">{{ tab.title }}</span>
      <span 
        class="tab-close" 
        @click="handleCloseTab(tab.id, $event)"
        title="关闭"
      >
        ✕
      </span>
    </div>
    
    <!-- 新标签提示 -->
    <div v-if="terminalsStore.tabs.length === 0" class="empty-tabs">
      <span>双击服务器连接</span>
    </div>
  </div>
</template>

<style scoped>
.tab-bar {
  height: var(--tabbar-height);
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-default);
  display: flex;
  align-items: flex-end;
  padding: 0 var(--spacing-sm);
  overflow-x: auto;
}

.tab-bar::-webkit-scrollbar {
  height: 4px;
}

.tab-bar::-webkit-scrollbar-thumb {
  background: var(--border-default);
  border-radius: 2px;
}

.tab {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-bottom: none;
  border-radius: var(--radius-md) var(--radius-md) 0 0;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  margin-right: 2px;
  max-width: 180px;
  transition: all 0.15s ease;
  user-select: none;
}

.tab:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.tab.active {
  background: var(--bg-primary);
  color: var(--text-primary);
  border-color: var(--border-light);
}

.tab-icon {
  font-size: 12px;
}

.tab-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.tab-close {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
  opacity: 0.6;
  transition: all 0.15s ease;
  font-size: 10px;
}

.tab-close:hover {
  opacity: 1;
  background: rgba(255, 255, 255, 0.1);
  color: var(--error);
}

.empty-tabs {
  display: flex;
  align-items: center;
  height: 100%;
  padding: 0 var(--spacing-md);
  color: var(--text-muted);
  font-size: 13px;
}
</style>
