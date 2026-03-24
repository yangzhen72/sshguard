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
    
    async downloadUpdate() {
      if (!this.updateInfo?.download_url) return;
      
      this.isDownloading = true;
      this.downloadProgress = 0;
      
      try {
        window.open(this.updateInfo.download_url, '_blank');
        this.isDownloading = false;
        this.showDialog = false;
      } catch (e) {
        this.error = String(e);
        this.isDownloading = false;
      }
    }
  }
});
