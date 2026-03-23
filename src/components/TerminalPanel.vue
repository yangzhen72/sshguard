<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { WebLinksAddon } from 'xterm-addon-web-links';
import { useTerminalsStore } from '../stores/terminals';
import { invoke } from '@tauri-apps/api/core';
import 'xterm/css/xterm.css';

const terminalsStore = useTerminalsStore();
const terminalRef = ref<HTMLDivElement | null>(null);
const terminals = ref<Map<string, { term: Terminal, fitAddon: FitAddon }>>(new Map());

const initTerminal = (tabId: string) => {
  if (!terminalRef.value) return;
  
  const term = new Terminal({
    cursorBlink: true,
    fontSize: 14,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
    theme: {
      background: '#1a1a2e',
      foreground: '#e0e0e0',
      cursor: '#e0e0e0'
    },
    scrollback: 10000
  });
  
  const fitAddon = new FitAddon();
  term.loadAddon(fitAddon);
  term.loadAddon(new WebLinksAddon());
  
  term.onData(data => {
    const tab = terminalsStore.tabs.find(t => t.id === tabId);
    if (tab) {
      invoke('send_pty_data', { sessionId: tab.sessionId, data });
    }
  });
  
  terminals.value.set(tabId, { term, fitAddon });
};

const attachTerminal = (tabId: string) => {
  const terminalData = terminals.value.get(tabId);
  if (terminalData && terminalRef.value) {
    terminalRef.value.innerHTML = '';
    terminalData.term.open(terminalRef.value);
    terminalData.fitAddon.fit();
  }
};

watch(() => terminalsStore.activeTabId, async (newId) => {
  if (newId) {
    await nextTick();
    if (!terminals.value.has(newId)) {
      initTerminal(newId);
    }
    attachTerminal(newId);
  }
});

onMounted(() => {
  if (terminalsStore.activeTabId) {
    initTerminal(terminalsStore.activeTabId);
    attachTerminal(terminalsStore.activeTabId);
  }
});
</script>

<template>
  <div class="terminal-panel">
    <div ref="terminalRef" class="terminal-container"></div>
  </div>
</template>

<style scoped>
.terminal-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.terminal-container {
  flex: 1;
  padding: 8px;
}
</style>
