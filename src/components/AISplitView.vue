<script setup lang="ts">
import { ref } from 'vue';
import type { AIMessage } from '../types/ai';
import AIMessageList from './AIMessageList.vue';
import AIInput from './AIInput.vue';

defineProps<{ messages: AIMessage[] }>();
const emit = defineEmits<{ (e: 'send', message: string): void }>();
const rightPanel = ref<'terminal' | 'files'>('terminal');
const handleSend = (message: string) => emit('send', message);
</script>

<template>
  <div class="split-view">
    <div class="split-left">
      <AIMessageList :messages="messages" />
      <AIInput @send="handleSend" />
    </div>
    <div class="split-divider"></div>
    <div class="split-right">
      <div class="right-tabs">
        <button :class="{ active: rightPanel === 'terminal' }" @click="rightPanel = 'terminal'">终端</button>
        <button :class="{ active: rightPanel === 'files' }" @click="rightPanel = 'files'">文件</button>
      </div>
      <div class="right-content">
        <div v-if="rightPanel === 'terminal'" class="terminal-placeholder">
          终端视图 - 已连接
        </div>
        <div v-else class="files-placeholder">
          文件浏览视图
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.split-view { flex: 1; display: flex; overflow: hidden; }
.split-left { flex: 1; display: flex; flex-direction: column; }
.split-divider { width: 4px; background: var(--border-default); cursor: col-resize; }
.split-right { width: 50%; display: flex; flex-direction: column; }
.right-tabs { display: flex; border-bottom: 1px solid var(--border-default); }
.right-tabs button { flex: 1; padding: 8px; background: transparent; border: none; color: var(--text-secondary); cursor: pointer; }
.right-tabs button.active { background: var(--bg-tertiary); color: var(--text-primary); }
.right-content { flex: 1; overflow: hidden; display: flex; align-items: center; justify-content: center; color: var(--text-muted); }
</style>
