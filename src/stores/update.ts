import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

export interface UpdateInfo {
  current_version: string;
  latest_version: string;
  has_update: boolean;
  release_notes: string | null;
  download_url: string | null;
}

export const useUpdateStore = defineStore('update', {
  state: () => ({
    updateInfo: null as UpdateInfo | null,
    isChecking: false,
    isDownloading: false,
    downloadProgress: 0,
    downloadMessage: '',
    showDialog: false,
    error: null as string | null,
  }),
  
  actions: {
    async checkForUpdates() {
      this.isChecking = true;
      this.error = null;
      try {
        const info = await invoke<UpdateInfo>('check_for_updates');
        this.updateInfo = info;
        if (info.has_update) {
          this.showDialog = true;
        }
      } catch (e) {
        this.error = String(e);
        console.error('Update check failed:', e);
      } finally {
        this.isChecking = false;
      }
    },
    
    closeDialog() {
      this.showDialog = false;
    },
    
    dismissUpdate() {
      this.showDialog = false;
      if (this.updateInfo) {
        localStorage.setItem('update_dismissed', this.updateInfo.latest_version);
      }
    },
    
    async downloadAndInstall() {
      if (!this.updateInfo?.download_url) {
        this.error = 'No download URL available';
        return;
      }
      
      this.isDownloading = true;
      this.downloadProgress = 0;
      this.downloadMessage = '正在下载更新...';
      this.error = null;
      
      try {
        const result = await invoke<string>('download_and_install', {
          url: this.updateInfo.download_url
        });
        this.downloadMessage = result;
        this.isDownloading = false;
        // App will restart or close after installation
      } catch (e: any) {
        this.error = String(e);
        this.isDownloading = false;
        this.downloadMessage = '';
      }
    }
  }
});
