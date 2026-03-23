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

export const useSftpStore = defineStore('sftp', () => {
  const currentSessionId = ref<number | null>(null);
  const currentPath = ref('/');
  const files = ref<SftpFile[]>([]);
  const selectedFile = ref<string | null>(null);
  const isLoading = ref(false);

  const setSession = (sessionId: number) => {
    currentSessionId.value = sessionId;
  };

  const listDirectory = async (path: string) => {
    if (!currentSessionId.value) return;
    isLoading.value = true;
    try {
      files.value = await invoke<SftpFile[]>('list_directory', { 
        sessionId: currentSessionId.value, 
        path 
      });
      currentPath.value = path;
    } finally {
      isLoading.value = false;
    }
  };

  const navigateTo = async (path: string) => {
    await listDirectory(path);
  };

  const navigateUp = async () => {
    if (!currentSessionId.value) return;
    const parent = currentPath.value.split('/').slice(0, -1).join('/') || '/';
    await navigateTo(parent);
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
    setSession,
    listDirectory,
    navigateTo,
    navigateUp,
    downloadFile,
    uploadFile
  };
});
