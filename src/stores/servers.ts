import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface ServerConfig {
  id: string;
  name: string;
  group: string;
  host: string;
  port: number;
  username: string;
  authType: 'password' | 'keyfile' | 'agent';
  encryptedPassword?: string;
  keyFilePath?: string;
  tags: string[];
  createdAt: number;
  updatedAt: number;
}

export const useServersStore = defineStore('servers', () => {
  const servers = ref<ServerConfig[]>([]);
  const selectedServerId = ref<string | null>(null);

  const selectedServer = computed(() => {
    return servers.value.find(s => s.id === selectedServerId.value) || null;
  });

  const filteredServers = (search: string) => {
    if (!search) return servers.value;
    const lower = search.toLowerCase();
    return servers.value.filter(s => 
      s.name.toLowerCase().includes(lower) ||
      s.host.toLowerCase().includes(lower) ||
      s.tags.some(t => t.toLowerCase().includes(lower))
    );
  };

  const loadServers = async () => {
    try {
      servers.value = await invoke<ServerConfig[]>('get_servers');
    } catch (e) {
      console.error('Failed to load servers:', e);
      servers.value = [];
    }
  };

  const addServer = async (data: Partial<ServerConfig>) => {
    const config: ServerConfig = {
      id: crypto.randomUUID(),
      name: data.name || '',
      group: data.group || 'Default',
      host: data.host || '',
      port: data.port || 22,
      username: data.username || '',
      authType: data.authType || 'password',
      tags: data.tags || [],
      createdAt: Date.now(),
      updatedAt: Date.now()
    };
    await invoke('add_server', { config });
    servers.value.push(config);
  };

  const updateServer = async (config: ServerConfig) => {
    await invoke('update_server', { config });
    const idx = servers.value.findIndex(s => s.id === config.id);
    if (idx >= 0) servers.value[idx] = config;
  };

  const deleteServer = async (id: string) => {
    await invoke('delete_server', { id });
    servers.value = servers.value.filter(s => s.id !== id);
    if (selectedServerId.value === id) selectedServerId.value = null;
  };

  const selectServer = (id: string) => {
    selectedServerId.value = id;
  };

  return {
    servers,
    selectedServerId,
    selectedServer,
    filteredServers,
    loadServers,
    addServer,
    updateServer,
    deleteServer,
    selectServer
  };
});