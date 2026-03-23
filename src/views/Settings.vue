<script setup lang="ts">
import { ref } from 'vue';
import { NForm, NFormItem, NInput, NSwitch, NSelect, NButton, NCard } from 'naive-ui';

const settings = ref({
  theme: 'dark',
  fontSize: 14,
  fontFamily: 'Menlo, Monaco, monospace',
  cursorStyle: 'block',
  cursorBlink: true,
  scrollback: 10000,
  masterPassword: '',
});

const themeOptions = [
  { label: 'Dark', value: 'dark' },
  { label: 'Light', value: 'light' },
];

const handleSave = () => {
  localStorage.setItem('sshguard-settings', JSON.stringify(settings.value));
};

const handleLoadSettings = () => {
  const saved = localStorage.getItem('sshguard-settings');
  if (saved) {
    settings.value = JSON.parse(saved);
  }
};
</script>

<template>
  <div class="settings-view">
    <h2>Settings</h2>
    
    <n-card title="Appearance" class="settings-card">
      <n-form label-placement="left" label-width="150">
        <n-form-item label="Theme">
          <n-select v-model:value="settings.theme" :options="themeOptions" />
        </n-form-item>
        <n-form-item label="Font Size">
          <n-input-number v-model:value="settings.fontSize" :min="10" :max="24" />
        </n-form-item>
        <n-form-item label="Font Family">
          <n-input v-model:value="settings.fontFamily" />
        </n-form-item>
      </n-form>
    </n-card>
    
    <n-card title="Terminal" class="settings-card">
      <n-form label-placement="left" label-width="150">
        <n-form-item label="Cursor Style">
          <n-select v-model:value="settings.cursorStyle" 
            :options="[{label:'Block',value:'block'},{label:'Line',value:'line'},{label:'Underline',value:'underline'}]" />
        </n-form-item>
        <n-form-item label="Cursor Blink">
          <n-switch v-model:value="settings.cursorBlink" />
        </n-form-item>
        <n-form-item label="Scrollback">
          <n-input-number v-model:value="settings.scrollback" :min="1000" :max="100000" />
        </n-form-item>
      </n-form>
    </n-card>
    
    <n-card title="Security" class="settings-card">
      <n-form label-placement="left" label-width="150">
        <n-form-item label="Master Password">
          <n-input type="password" v-model:value="settings.masterPassword" placeholder="Set master password" />
        </n-form-item>
      </n-form>
    </n-card>
    
    <div class="settings-actions">
      <n-button type="primary" @click="handleSave">Save Settings</n-button>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}

.settings-view h2 {
  margin-bottom: 24px;
}

.settings-card {
  margin-bottom: 16px;
}

.settings-actions {
  margin-top: 24px;
  display: flex;
  justify-content: flex-end;
}
</style>
