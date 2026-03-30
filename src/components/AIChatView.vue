<script setup lang="ts">
import { useAIStore } from '../stores/ai';
import AIMessageList from './AIMessageList.vue';
import AIInput from './AIInput.vue';

const aiStore = useAIStore();

const handleSend = async (message: string) => {
  await aiStore.sendMessage(message);
};

const copyMessage = (content: string) => {
  navigator.clipboard.writeText(content);
};
</script>

<template>
  <div class="chat-view">
    <AIMessageList :messages="aiStore.messages" @copy="copyMessage" />
    <AIInput @send="handleSend" />
  </div>
</template>

<style scoped>
.chat-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
