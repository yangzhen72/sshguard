<script setup lang="ts">
import type { AIMessage } from '../types/ai';

defineProps<{
  messages: AIMessage[];
}>();
</script>

<template>
  <div class="terminal-view">
    <div class="terminal-output">
      <div v-for="msg in messages" :key="msg.id" class="terminal-line">
        <span class="prompt" v-if="msg.role === 'user'">$ </span>
        <span class="response-indicator" v-else>> </span>
        <span class="content">{{ msg.content }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.terminal-view {
  flex: 1;
  background: #0d1117;
  color: #c9d1d9;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 13px;
  overflow-y: auto;
}
.terminal-output { padding: 12px; }
.terminal-line { margin-bottom: 8px; line-height: 1.4; }
.prompt { color: #58a6ff; }
.response-indicator { color: #7ee787; }
.content { white-space: pre-wrap; word-break: break-word; }
</style>
