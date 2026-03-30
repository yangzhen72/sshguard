import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { AIMessage, AIConfig } from '../types/ai';

export const useAIStore = defineStore('ai', () => {
  const isOpen = ref(false);
  const config = ref<AIConfig | null>(null);
  const messages = ref<AIMessage[]>([]);
  const isLoading = ref(false);
  const style = ref<'chatgpt' | 'terminal' | 'split'>('chatgpt');
  const panelWidth = ref(400);
  const panelPosition = ref({ right: 0, top: 0 });

  const toggle = () => { isOpen.value = !isOpen.value; };
  const open = () => { isOpen.value = true; };
  const close = () => { isOpen.value = false; };
  
  const setConfig = (newConfig: AIConfig) => { config.value = newConfig; };
  const setStyle = (newStyle: typeof style.value) => { style.value = newStyle; };
  const setPanelWidth = (width: number) => { 
    panelWidth.value = Math.max(300, Math.min(600, width));
  };
  const setPanelPosition = (pos: { right: number; top: number }) => {
    panelPosition.value = pos;
    savePosition();
  };
  
  const addMessage = (message: AIMessage) => { messages.value.push(message); };
  const clearMessages = () => { messages.value = []; };
  
  const loadMessages = () => {
    const stored = localStorage.getItem('ai_messages');
    if (stored) {
      try {
        messages.value = JSON.parse(stored);
      } catch {
        messages.value = [];
      }
    }
  };
  
  const saveMessages = () => {
    localStorage.setItem('ai_messages', JSON.stringify(messages.value));
  };

  const loadPosition = () => {
    const stored = localStorage.getItem('ai_panel_position');
    if (stored) {
      try {
        const pos = JSON.parse(stored);
        panelPosition.value = pos;
      } catch {
        panelPosition.value = { right: 0, top: 0 };
      }
    }
  };

  const savePosition = () => {
    localStorage.setItem('ai_panel_position', JSON.stringify(panelPosition.value));
  };

  const sendMessage = async (content: string) => {
    if (!config.value) throw new Error('AI not configured');
    
    addMessage({ id: crypto.randomUUID(), role: 'user', content, timestamp: Date.now() });
    saveMessages();
    isLoading.value = true;
    
    try {
      const response = await invoke<string>('chat', { message: content });
      addMessage({ id: crypto.randomUUID(), role: 'assistant', content: response, timestamp: Date.now() });
      saveMessages();
    } catch (e: any) {
      addMessage({ id: crypto.randomUUID(), role: 'assistant', content: `Error: ${e}`, timestamp: Date.now() });
      saveMessages();
    } finally {
      isLoading.value = false;
    }
  };

  loadMessages();
  loadPosition();

  return {
    isOpen, config, messages, isLoading, style, panelWidth, panelPosition,
    toggle, open, close, setConfig, setStyle, setPanelWidth, setPanelPosition,
    addMessage, clearMessages, sendMessage, loadMessages, saveMessages,
    loadPosition, savePosition
  };
});