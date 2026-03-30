<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  (e: 'send', message: string): void;
}>();

const input = ref('');

const handleSend = () => {
  if (input.value.trim()) {
    emit('send', input.value);
    input.value = '';
  }
};
</script>

<template>
  <div class="ai-input">
    <textarea
      v-model="input"
      placeholder="输入消息..."
      rows="2"
      @keydown.enter.exact.prevent="handleSend"
    ></textarea>
    <button @click="handleSend" :disabled="!input.trim()">发送</button>
  </div>
</template>

<style scoped>
.ai-input {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-top: 1px solid var(--border-default);
}
.ai-input textarea {
  flex: 1;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: 4px;
  color: var(--text-primary);
  resize: none;
  padding: 8px;
}
.ai-input button {
  padding: 8px 16px;
  background: var(--accent-primary);
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
}
.ai-input button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>