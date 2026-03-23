<script setup lang="ts">
import { ref } from 'vue';

const connectionStatus = ref('Disconnected');
const transferProgress = ref<{ file: string; percent: number } | null>(null);
const encoding = ref('UTF-8');
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <span class="status-indicator" :class="{ connected: connectionStatus === 'Connected' }"></span>
      <span>{{ connectionStatus }}</span>
    </div>
    <div class="status-center">
      <span v-if="transferProgress">
        📤 {{ transferProgress.file }} - {{ transferProgress.percent }}%
      </span>
    </div>
    <div class="status-right">
      <span>{{ encoding }}</span>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  height: 24px;
  background: #16162a;
  border-top: 1px solid #2a2a5e;
  display: flex;
  align-items: center;
  padding: 0 12px;
  font-size: 12px;
  color: #888;
}

.status-left, .status-center, .status-right {
  flex: 1;
}

.status-center {
  text-align: center;
}

.status-right {
  text-align: right;
}

.status-indicator {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #666;
  margin-right: 6px;
}

.status-indicator.connected {
  background: #4ade80;
}
</style>