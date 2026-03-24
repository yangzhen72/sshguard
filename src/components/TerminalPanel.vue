<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { WebLinksAddon } from 'xterm-addon-web-links';
import { useTerminalsStore } from '../stores/terminals';
import { useSftpStore } from '../stores/sftp';
import { invoke } from '@tauri-apps/api/core';
import 'xterm/css/xterm.css';

const terminalsStore = useTerminalsStore();
const sftpStore = useSftpStore();
const terminalRef = ref<HTMLDivElement | null>(null);
const terminals = ref<Map<string, { term: Terminal, fitAddon: FitAddon, inputBuffer: string }>>(new Map());

const CD_PATTERN = /^\s*cd\s+(.+)$/;

const initTerminal = (tabId: string) => {
  if (!terminalRef.value) return;
  
  const term = new Terminal({
    cursorBlink: true,
    fontSize: 14,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
    theme: {
      background: '#000000',
      foreground: '#00ff88',
      cursor: '#00ffff',
      selectionBackground: 'rgba(0, 255, 255, 0.3)'
    },
    scrollback: 10000
  });
  
  const fitAddon = new FitAddon();
  term.loadAddon(fitAddon);
  term.loadAddon(new WebLinksAddon());
  
  let inputBuffer = '';
  
  term.onData(data => {
    const tab = terminalsStore.tabs.find(t => t.id === tabId);
    if (tab?.sessionId) {
      if (data === '\r') {
        const trimmed = inputBuffer.trim();
        if (CD_PATTERN.test(trimmed)) {
          const match = trimmed.match(CD_PATTERN);
          if (match && sftpStore.currentSessionId) {
            const targetPath = match[1].trim();
            sftpStore.syncFromTerminal(targetPath);
          }
        }
        invoke('send_pty_data', { sessionId: tab.sessionId, data: new TextEncoder().encode(inputBuffer + '\n') });
        inputBuffer = '';
      } else if (data === '\u007f') {
        if (inputBuffer.length > 0) {
          inputBuffer = inputBuffer.slice(0, -1);
          term.write('\b \b');
        }
      } else {
        inputBuffer += data;
        invoke('send_pty_data', { sessionId: tab.sessionId, data: new TextEncoder().encode(data) });
      }
    }
  });
  
  terminals.value.set(tabId, { term, fitAddon, inputBuffer });
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
  <div class="terminal-panel cyberpunk-terminal scanline">
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
