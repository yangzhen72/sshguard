<script setup lang="ts">
import { ref, watch } from 'vue';
import { useAIStore } from '../stores/ai';
import { invoke } from '@tauri-apps/api/core';

const aiStore = useAIStore();
const providers = [
  { label: 'Claude (Anthropic)', value: 'anthropic' },
  { label: 'GPT (OpenAI)', value: 'openai' },
  { label: '通义千问', value: 'qwen' },
  { label: 'MiniMax', value: 'minimax' },
  { label: 'DeepSeek', value: 'deepseek' },
];
const modelOptions: Record<string, string[]> = {
  anthropic: ['claude-3-5-sonnet-20241022', 'claude-3-opus-20240229'],
  openai: ['gpt-4o', 'gpt-4-turbo', 'gpt-3.5-turbo'],
  qwen: ['qwen-plus', 'qwen-turbo', 'qwen-max'],
  minimax: ['MiniMax-Text-01'],
  deepseek: ['deepseek-chat', 'deepseek-coder'],
};

const selectedProvider = ref(aiStore.config?.provider || 'anthropic');
const apiKey = ref(aiStore.config?.apiKey || '');
const selectedModel = ref(aiStore.config?.model || modelOptions['anthropic'][0]);
const selectedStyle = ref(aiStore.style);

watch(selectedProvider, (val) => {
  selectedModel.value = modelOptions[val][0];
});

const saveConfig = async () => {
  try {
    await invoke('set_config', {
      config: {
        provider: selectedProvider.value,
        api_key: apiKey.value,
        model: selectedModel.value,
      }
    });
    aiStore.setConfig({
      provider: selectedProvider.value as any,
      apiKey: apiKey.value,
      model: selectedModel.value,
    });
    aiStore.setStyle(selectedStyle.value);
  } catch (e) {
    console.error('Failed to save config:', e);
  }
};
</script>

<template>
  <div class="ai-settings">
    <h3>设置</h3>
    <div class="form-group">
      <label>AI 提供商</label>
      <select v-model="selectedProvider">
        <option v-for="p in providers" :key="p.value" :value="p.value">{{ p.label }}</option>
      </select>
    </div>
    <div class="form-group">
      <label>API Key</label>
      <input type="password" v-model="apiKey" placeholder="输入 API Key" />
    </div>
    <div class="form-group">
      <label>模型</label>
      <select v-model="selectedModel">
        <option v-for="m in modelOptions[selectedProvider]" :key="m" :value="m">{{ m }}</option>
      </select>
    </div>
    <div class="form-group">
      <label>界面风格</label>
      <select v-model="selectedStyle">
        <option value="chatgpt">ChatGPT 风格</option>
        <option value="terminal">终端风格</option>
        <option value="split">分屏布局</option>
      </select>
    </div>
    <button @click="saveConfig" class="save-btn">保存</button>
  </div>
</template>

<style scoped>
.ai-settings { padding: 16px; }
.form-group { margin-bottom: 12px; }
.form-group label { display: block; font-size: 12px; color: var(--text-secondary); margin-bottom: 4px; }
.form-group select, .form-group input { width: 100%; padding: 8px; background: var(--bg-primary); border: 1px solid var(--border-default); border-radius: 4px; color: var(--text-primary); }
.save-btn { width: 100%; padding: 8px; background: var(--accent-primary); border: none; border-radius: 4px; color: white; cursor: pointer; }
</style>
