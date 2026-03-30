<script setup lang="ts">
import type { AIMessage } from '../types/ai';

defineProps<{
  messages: AIMessage[];
}>();

const emit = defineEmits<{
  (e: 'copy', content: string): void;
}>();

const copyMessage = (content: string) => {
  emit('copy', content);
};
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
      <button class="copy-btn" @click="copyMessage(msg.content)" title="复制">📋</button>
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
.message {
  position: relative;
}
.copy-btn {
  position: absolute;
  top: 4px;
  right: 4px;
  background: rgba(0,0,0,0.3);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s;
}
.message:hover .copy-btn {
  opacity: 1;
}
</style>