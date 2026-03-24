<script setup lang="ts">
import { useUpdateStore } from '../stores/update';

const updateStore = useUpdateStore();

const formatReleaseNotes = (notes: string | null): string => {
  if (!notes) return '暂无更新说明';
  return notes.slice(0, 500) + (notes.length > 500 ? '...' : '');
};
</script>

<template>
  <div v-if="updateStore.showDialog" class="modal-overlay" @click.self="updateStore.closeDialog">
    <div class="modal-content">
      <div class="modal-header">
        <div class="header-title">
          <span class="icon">🎉</span>
          <span>新版本可用</span>
        </div>
        <button class="close-btn" @click="updateStore.closeDialog">✕</button>
      </div>
      
      <div class="modal-body">
        <div class="version-info">
          <span class="version current">v{{ updateStore.updateInfo?.current_version }}</span>
          <span class="arrow">→</span>
          <span class="version latest">v{{ updateStore.updateInfo?.latest_version }}</span>
        </div>
        
        <div class="release-notes-section">
          <div class="section-label">更新内容</div>
          <pre class="release-notes">{{ formatReleaseNotes(updateStore.updateInfo?.release_notes || null) }}</pre>
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn" @click="updateStore.dismissUpdate">
          忽略此版本
        </button>
        <button class="btn" @click="updateStore.closeDialog">
          稍后提醒
        </button>
        <button 
          class="btn primary" 
          :disabled="updateStore.isDownloading"
          @click="updateStore.downloadUpdate"
        >
          {{ updateStore.isDownloading ? '下载中...' : '立即下载' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  width: 420px;
  max-width: 90vw;
  box-shadow: var(--shadow-lg);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border-default);
}

.header-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.icon {
  font-size: 20px;
}

.close-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--radius-sm);
  font-size: 14px;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.modal-body {
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.version-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  justify-content: center;
  font-size: 16px;
}

.version {
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
}

.version.current {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
}

.version.latest {
  background: rgba(0, 152, 255, 0.2);
  color: var(--accent-primary);
  font-weight: 600;
}

.arrow {
  color: var(--text-muted);
}

.release-notes-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.section-label {
  font-size: 12px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.release-notes {
  background: var(--bg-primary);
  padding: var(--spacing-md);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-default);
  white-space: pre-wrap;
  word-break: break-all;
  font-size: 13px;
  color: var(--text-primary);
  max-height: 200px;
  overflow-y: auto;
  margin: 0;
  line-height: 1.5;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border-default);
}

.btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.15s ease;
}

.btn:hover {
  background: var(--bg-hover);
}

.btn.primary {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: #fff;
}

.btn.primary:hover {
  background: var(--accent-hover);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
