<script setup lang="ts">
import type { AIMessage } from '../types/ai';

defineProps<{
  messages: AIMessage[];
}>();
</script>

<template>
  <div class="message-list">
    <div
      v-for="msg in messages"
      :key="msg.id"
      class="message"
      :class="msg.role"
    >
      <div class="message-content">{{ msg.content }}</div>
    </div>
  </div>
</template>

<style scoped>
.message-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}
.message {
  margin-bottom: 12px;
  padding: 8px 12px;
  border-radius: 8px;
}
.message.user {
  background: var(--accent-primary);
  color: white;
  margin-left: 20%;
}
.message.assistant {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  margin-right: 20%;
}
.message-content {
  white-space: pre-wrap;
  word-break: break-word;
}
</style>