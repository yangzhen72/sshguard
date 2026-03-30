<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import MainLayout from "./views/MainLayout.vue";
import AIFloatingPanel from "./components/AIFloatingPanel.vue";
import { useAIStore } from "./stores/ai";

const aiStore = useAIStore();

const handleKeydown = (e: KeyboardEvent) => {
  if (e.ctrlKey && e.shiftKey && e.key === 'A') {
    e.preventDefault();
    aiStore.toggle();
  }
  if (e.key === 'Escape' && aiStore.isOpen) {
    aiStore.close();
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <n-dialog-provider>
        <MainLayout class="cyberpunk-app" />
        <AIFloatingPanel />
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script lang="ts">
import { darkTheme } from "naive-ui";
</script>

<style>
@import './styles/cyberpunk.css';

html, body, #app {
  margin: 0;
  padding: 0;
  height: 100%;
  overflow: hidden;
  font-family: 'Segoe UI', system-ui, sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
}
</style>
