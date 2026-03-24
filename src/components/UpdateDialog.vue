<script setup lang="ts">
import { NModal, NCard, NButton, NSpace, NProgress } from 'naive-ui';
import { useUpdateStore } from '../stores/update';

const updateStore = useUpdateStore();

const formatReleaseNotes = (notes: string | null): string => {
  if (!notes) return '暂无更新说明';
  return notes.slice(0, 500) + (notes.length > 500 ? '...' : '');
};
</script>

<template>
  <NModal v-model:show="updateStore.showDialog" :mask-closable="false">
    <NCard
      class="update-dialog"
      :bordered="false"
    >
      <template #header>
        <div class="dialog-header">
          <span class="icon">🎉</span>
          <span class="title">新版本可用</span>
        </div>
      </template>
      
      <div class="update-content">
        <div class="version-info">
          <span class="current">当前版本：v{{ updateStore.updateInfo?.current_version }}</span>
          <span class="arrow">→</span>
          <span class="latest">最新版本：v{{ updateStore.updateInfo?.latest_version }}</span>
        </div>
        
        <div class="release-notes-section">
          <div class="section-label">更新内容</div>
          <pre class="release-notes">{{ formatReleaseNotes(updateStore.updateInfo?.release_notes || null) }}</pre>
        </div>
        
        <NProgress
          v-if="updateStore.isDownloading"
          type="line"
          :percentage="updateStore.downloadProgress"
          indicator="outside"
          status="success"
        />
      </div>
      
      <template #footer>
        <NSpace justify="end">
          <NButton @click="updateStore.dismissUpdate">
            忽略此版本
          </NButton>
          <NButton @click="updateStore.closeDialog">
            稍后提醒
          </NButton>
          <NButton
            type="primary"
            class="cyberpunk-button"
            :loading="updateStore.isDownloading"
            @click="updateStore.downloadUpdate"
          >
            立即下载
          </NButton>
        </NSpace>
      </template>
    </NCard>
  </NModal>
</template>

<style scoped>
.update-dialog {
  max-width: 480px;
  background: var(--bg-secondary);
  border: 1px solid var(--neon-cyan);
  box-shadow: var(--glow-cyan);
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.icon {
  font-size: 24px;
}

.title {
  font-size: 18px;
  font-weight: bold;
  color: var(--neon-cyan);
}

.update-content {
  margin: 16px 0;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.version-info {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
}

.current {
  color: var(--text-secondary);
}

.arrow {
  color: var(--neon-cyan);
}

.latest {
  color: var(--neon-magenta);
  font-weight: bold;
}

.release-notes-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-label {
  font-size: 12px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.release-notes {
  background: var(--bg-primary);
  padding: 12px;
  border-radius: 4px;
  border: 1px solid var(--border-default);
  white-space: pre-wrap;
  word-break: break-all;
  font-size: 13px;
  color: var(--text-primary);
  max-height: 180px;
  overflow-y: auto;
  margin: 0;
}
</style>
