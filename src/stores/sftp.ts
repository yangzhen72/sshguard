import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface SftpFile {
  name: string;
  path: string;
  isDirectory: boolean;
  size: number;
  modified: number;
  permissions: string;
}

export type SyncStatus = 'idle' | 'syncing' | 'synced' | 'error';

export const useSftpStore = defineStore('sftp', () => {
  const currentSessionId = ref<number | null>(null);
  const currentPath = ref('/');
  const files = ref<SftpFile[]>([]);
  const selectedFile = ref<string | null>(null);
  const isLoading = ref(false);
  const syncStatus = ref<SyncStatus>('idle');
  const lastError = ref<string | null>(null);
  const previousPath = ref('/');

  const setSession = (sessionId: number) => {
    currentSessionId.value = sessionId;
    syncStatus.value = 'idle';
  };

  const listDirectory = async (path: string) => {
    if (!currentSessionId.value) return;
    isLoading.value = true;
    try {
      files.value = await invoke<SftpFile[]>('list_directory', { 
        sessionId: currentSessionId.value, 
        path 
      });
      previousPath.value = currentPath.value;
      currentPath.value = path;
      syncStatus.value = 'synced';
      lastError.value = null;
    } catch (e) {
      syncStatus.value = 'error';
      lastError.value = String(e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  const navigateTo = async (path: string) => {
    syncStatus.value = 'syncing';
    await listDirectory(path);
  };

  const navigateUp = async () => {
    if (!currentSessionId.value) return;
    const parent = currentPath.value.split('/').slice(0, -1).join('/') || '/';
    await navigateTo(parent);
  };

  const syncFromTerminal = async (terminalPath: string) => {
    if (!currentSessionId.value) return;
    
    syncStatus.value = 'syncing';
    try {
      let targetPath = terminalPath.trim();
      
      if (targetPath === '-') {
        syncStatus.value = 'idle';
        return;
      }
      
      if (!targetPath.startsWith('/')) {
        if (targetPath === '..') {
          const parts = currentPath.value.split('/').filter(Boolean);
          parts.pop();
          targetPath = '/' + parts.join('/');
        } else if (targetPath === '.') {
          targetPath = currentPath.value;
        } else {
          const basePath = currentPath.value === '/' ? '' : currentPath.value;
          targetPath = basePath + '/' + targetPath;
        }
      }
      
      await listDirectory(targetPath);
    } catch (e) {
      syncStatus.value = 'error';
      lastError.value = String(e);
    }
  };

  const downloadFile = async (remote: string, local: string) => {
    if (!currentSessionId.value) return;
    await invoke('download_file', { sessionId: currentSessionId.value, remote, local });
  };

  const uploadFile = async (local: string, remote: string) => {
    if (!currentSessionId.value) return;
    await invoke('upload_file', { sessionId: currentSessionId.value, local, remote });
  };

  return {
    currentSessionId,
    currentPath,
    files,
    selectedFile,
    isLoading,
    syncStatus,
    lastError,
    previousPath,
    setSession,
    listDirectory,
    navigateTo,
    navigateUp,
    syncFromTerminal,
    downloadFile,
    uploadFile
  };
});
