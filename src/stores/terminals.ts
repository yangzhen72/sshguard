import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ServerConfig } from './servers';

export interface TerminalTab {
  id: string;
  serverId: string;
  title: string;
  sessionId: string;
}

interface ConnectResult {
  session_id: string;
}

export const useTerminalsStore = defineStore('terminals', () => {
  const tabs = ref<TerminalTab[]>([]);
  const activeTabId = ref<string | null>(null);

  const createTab = async (server: ServerConfig) => {
    const tabId = crypto.randomUUID();
    
    const authInfo = buildAuthInfo(server);
    
    const result = await invoke<ConnectResult>('connect', {
      host: server.host,
      port: server.port,
      username: server.username,
      authInfo
    });
    
    await invoke<string>('create_pty', {
      sessionId: result.session_id,
      term: 'xterm-256color',
      cols: 80,
      rows: 24
    });
    
    tabs.value.push({
      id: tabId,
      serverId: server.id,
      title: server.name,
      sessionId: result.session_id
    });
    activeTabId.value = tabId;
    return tabId;
  };

  const buildAuthInfo = (server: ServerConfig) => {
    if (server.authType === 'password') {
      return {
        authType: 'password',
        password: server.encryptedPassword || ''
      };
    } else if (server.authType === 'keyfile') {
      return {
        authType: 'keyfile',
        keyFilePath: server.keyFilePath || '',
        passphrase: null
      };
    } else {
      return { authType: 'agent' };
    }
  };

  const closeTab = (id: string) => {
    const tab = tabs.value.find(t => t.id === id);
    if (tab) {
      invoke('disconnect', { sessionId: tab.sessionId });
    }
    tabs.value = tabs.value.filter(t => t.id !== id);
    if (activeTabId.value === id) {
      activeTabId.value = tabs.value[0]?.id || null;
    }
  };

  const setActiveTab = (id: string) => {
    activeTabId.value = id;
  };

  return {
    tabs,
    activeTabId,
    createTab,
    closeTab,
    setActiveTab
  };
});