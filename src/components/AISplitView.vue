<script setup lang="ts">
import { ref } from 'vue';
import { useTerminalsStore } from '../stores/terminals';
import { invoke } from '@tauri-apps/api/core';
import AIMessageList from './AIMessageList.vue';
import AIInput from './AIInput.vue';
import { useAIStore } from '../stores/ai';

const aiStore = useAIStore();
const terminalsStore = useTerminalsStore();
const rightPanel = ref<'terminal' | 'files'>('terminal');
const terminalInput = ref('');
const terminalOutput = ref<string[]>([]);

defineProps<{ messages?: any[] }>();
const emit = defineEmits<{ (e: 'send', message: string): void }>();

const handleSend = (message: string) => emit('send', message);

const sendTerminalCommand = async () => {
  if (!terminalInput.value.trim()) return;
  
  const cmd = terminalInput.value;
  terminalOutput.value.push(`$ ${cmd}`);
  terminalInput.value = '';
  
  const activeTab = terminalsStore.tabs.find(t => t.id === terminalsStore.activeTabId);
  if (activeTab?.sessionId) {
    try {
      await invoke('send_pty_data', { 
        sessionId: activeTab.sessionId, 
        data: new TextEncoder().encode(cmd + '\n') 
      });
    } catch (e) {
      terminalOutput.value.push(`Error: ${e}`);
    }
  } else {
    terminalOutput.value.push('No active SSH session');
  }
};
</script>

<template>
  <div class="split-view">
    <div class="split-left">
      <AIMessageList :messages="aiStore.messages" />
      <AIInput @send="handleSend" />
    </div>
    <div class="split-divider"></div>
    <div class="split-right">
      <div class="right-tabs">
        <button :class="{ active: rightPanel === 'terminal' }" @click="rightPanel = 'terminal'">终端</button>
        <button :class="{ active: rightPanel === 'files' }" @click="rightPanel = 'files'">文件</button>
      </div>
      <div class="right-content">
        <div v-if="rightPanel === 'terminal'" class="terminal-panel">
          <div class="terminal-output">
            <div v-for="(line, i) in terminalOutput" :key="i" class="terminal-line">{{ line }}</div>
          </div>
          <div class="terminal-input-area">
            <span class="prompt">$</span>
            <input 
              v-model="terminalInput" 
              @keydown.enter="sendTerminalCommand"
              class="terminal-input"
              placeholder="Type command..."
            />
          </div>
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
.split-right { flex: 1; display: flex; flex-direction: column; }
.right-tabs { display: flex; border-bottom: 1px solid var(--border-default); }
.right-tabs button { flex: 1; padding: 8px; background: transparent; border: none; color: var(--text-secondary); cursor: pointer; }
.right-tabs button.active { background: var(--bg-tertiary); color: var(--text-primary); }
.right-content { flex: 1; overflow: hidden; display: flex; flex-direction: column; }
.terminal-panel { flex: 1; display: flex; flex-direction: column; background: #0d1117; }
.terminal-output { flex: 1; padding: 12px; overflow-y: auto; font-family: 'Consolas', monospace; font-size: 13px; color: #c9d1d9; }
.terminal-line { margin-bottom: 4px; }
.terminal-input-area { display: flex; align-items: center; padding: 8px 12px; border-top: 1px solid #30363d; background: #161b22; }
.prompt { color: #58a6ff; margin-right: 8px; font-family: 'Consolas', monospace; }
.terminal-input { flex: 1; background: transparent; border: none; outline: none; color: #c9d1d9; font-family: 'Consolas', monospace; font-size: 13px; }
.files-placeholder { flex: 1; display: flex; align-items: center; justify-content: center; color: var(--text-muted); }
</style>
