<script setup lang="ts">
import { useTerminalsStore } from '../stores/terminals';

const terminalsStore = useTerminalsStore();
</script>

<template>
  <div class="tab-bar">
    <div class="tabs">
      <div
        v-for="tab in terminalsStore.tabs"
        :key="tab.id"
        :class="['tab', { active: tab.id === terminalsStore.activeTabId }]"
        @click="terminalsStore.setActiveTab(tab.id)"
      >
        <span class="tab-title">{{ tab.title }}</span>
        <span class="tab-close" @click.stop="terminalsStore.closeTab(tab.id)">×</span>
      </div>
    </div>
    <button class="add-tab">+</button>
  </div>
</template>

<style scoped>
.tab-bar {
  display: flex;
  align-items: center;
  background: #16162a;
  border-bottom: 1px solid #2a2a5e;
  height: 40px;
  padding: 0 8px;
}

.tabs {
  display: flex;
  flex: 1;
  overflow-x: auto;
}

.tab {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  background: #1e1e3f;
  border-radius: 4px 4px 0 0;
  margin-right: 2px;
  cursor: pointer;
  gap: 8px;
  min-width: 120px;
  max-width: 200px;
}

.tab.active {
  background: #2a2a5e;
}

.tab-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tab-close {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.tab-close:hover {
  background: rgba(255, 255, 255, 0.1);
}

.add-tab {
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  color: #888;
  cursor: pointer;
  font-size: 20px;
}

.add-tab:hover {
  color: #fff;
}
</style>
