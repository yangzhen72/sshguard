<script setup lang="ts">
import { ref, watch } from 'vue';
import { VueMonacoEditor } from '@guolao/vue-monaco-editor';
import { NButton, NSpace } from 'naive-ui';

const props = defineProps<{
  filePath: string;
  content: string;
  language?: string;
}>();

const emit = defineEmits(['save', 'close']);

const editorContent = ref(props.content);
const isDirty = ref(false);

watch(() => props.content, (newContent) => {
  editorContent.value = newContent;
});

const handleSave = () => {
  emit('save', editorContent.value);
  isDirty.value = false;
};

const handleChange = () => {
  isDirty.value = true;
};

const getLanguage = (path: string) => {
  const ext = path.split('.').pop()?.toLowerCase() || '';
  const langMap: Record<string, string> = {
    'js': 'javascript',
    'ts': 'typescript',
    'py': 'python',
    'rs': 'rust',
    'json': 'json',
    'html': 'html',
    'css': 'css',
    'md': 'markdown',
    'sh': 'shell',
    'yaml': 'yaml',
    'yml': 'yaml',
  };
  return langMap[ext] || 'plaintext';
};
</script>

<template>
  <div class="file-editor">
    <div class="editor-toolbar">
      <span class="file-name">{{ filePath }}</span>
      <n-space>
        <n-button size="small" @click="handleSave" :disabled="!isDirty">Save</n-button>
        <n-button size="small" @click="emit('close')">Close</n-button>
      </n-space>
    </div>
    <VueMonacoEditor
      v-model:value="editorContent"
      :language="language || getLanguage(filePath)"
      theme="vs-dark"
      :options="{
        minimap: { enabled: true },
        lineNumbers: 'on',
        folding: true,
        fontSize: 14,
        scrollBeyondLastLine: false,
        automaticLayout: true,
      }"
      @change="handleChange"
    />
  </div>
</template>

<style scoped>
.file-editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #1e1e3f;
}

.editor-toolbar {
  display: flex;
  justify-content: space-between;
  padding: 8px 12px;
  background: #16162a;
  border-bottom: 1px solid #2a2a5e;
}

.file-name {
  font-size: 13px;
  color: #888;
}
</style>
