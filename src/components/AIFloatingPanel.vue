<script setup lang="ts">
import { computed } from 'vue';
import { useAIStore } from '../stores/ai';
import AIChatView from './AIChatView.vue';
import AITerminalView from './AITerminalView.vue';
import AISplitView from './AISplitView.vue';

const aiStore = useAIStore();

const panelStyle = computed(() => ({
  transform: aiStore.isOpen ? 'translateX(0)' : 'translateX(100%)',
  width: `${aiStore.panelWidth}px`,
}));

const handleSend = async (message: string) => {
  await aiStore.sendMessage(message);
};
</script>

<template>
  <div class="floating-panel" :style="panelStyle">
    <div class="panel-header">
      <span>🤖 AI 助手</span>
      <button @click="aiStore.close">×</button>
    </div>
    <AIChatView v-if="aiStore.style === 'chatgpt'" />
    <AITerminalView v-else-if="aiStore.style === 'terminal'" />
    <AISplitView v-else-if="aiStore.style === 'split'" @send="handleSend" />
  </div>
</template>

<style scoped>
.floating-panel {
  position: fixed;
  right: 0;
  top: 0;
  bottom: 0;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  transition: transform 0.3s ease;
  z-index: 1000;
}
.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-default);
  font-weight: 600;
}
.panel-header button {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: var(--text-secondary);
}
</style>