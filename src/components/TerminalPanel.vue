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
const terminals = ref<Map<string, { term: Terminal, fitAddon: FitAddon }>>(new Map());

const CD_PATTERN = /^\s*cd\s+(.+)$/;

const initTerminal = (tabId: string) => {
  if (!terminalRef.value) return;
  
  const term = new Terminal({
    cursorBlink: true,
    fontSize: 14,
    fontFamily: 'Consolas, Monaco, "Courier New", monospace',
    theme: {
      background: '#000000',
      foreground: '#e0e0e0',
      cursor: '#0098ff',
      cursorAccent: '#000000',
      selectionBackground: 'rgba(0, 152, 255, 0.3)',
      black: '#000000',
      red: '#f44336',
      green: '#4caf50',
      yellow: '#ff9800',
      blue: '#2196f3',
      magenta: '#9c27b0',
      cyan: '#00bcd4',
      white: '#e0e0e0',
      brightBlack: '#666666',
      brightRed: '#ef5350',
      brightGreen: '#69f0ae',
      brightYellow: '#ffe082',
      brightBlue: '#64b5f6',
      brightMagenta: '#ba68c8',
      brightCyan: '#84ffff',
      brightWhite: '#ffffff',
    },
    scrollback: 10000,
    allowTransparency: false,
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
  padding: var(--spacing-sm);
  background: var(--bg-primary);
}

.terminal-container {
  flex: 1;
  background: var(--terminal-bg);
  border-radius: var(--radius-md);
  overflow: hidden;
}
</style>
