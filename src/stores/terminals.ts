import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ServerConfig } from './servers';

export interface TerminalTab {
  id: string;
  serverId: string;
  title: string;
  sessionId: number;
}

export const useTerminalsStore = defineStore('terminals', () => {
  const tabs = ref<TerminalTab[]>([]);
  const activeTabId = ref<string | null>(null);

  const createTab = async (server: ServerConfig) => {
    const tabId = crypto.randomUUID();
    
    let password: string | null = null;
    let keyFile: string | null = null;
    
    if (server.authType === 'password' && server.encryptedPassword) {
      password = server.encryptedPassword;
    } else if (server.authType === 'keyfile') {
      keyFile = server.keyFilePath || null;
    }
    
    const sessionId = await invoke<number>('connect', {
      host: server.host,
      port: server.port,
      username: server.username,
      authType: server.authType,
      password,
      keyFile
    });
    
    await invoke('create_pty', { sessionId });
    
    tabs.value.push({
      id: tabId,
      serverId: server.id,
      title: server.name,
      sessionId
    });
    activeTabId.value = tabId;
    return tabId;
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
