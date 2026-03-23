<script setup lang="ts">
import { ref } from 'vue';
import { NForm, NFormItem, NInput, NInputNumber, NSelect, NButton, NSpace } from 'naive-ui';
import { useServersStore } from '../stores/servers';

const emit = defineEmits(['close']);
const serversStore = useServersStore();

const formData = ref({
  name: '',
  group: 'Default',
  host: '',
  port: 22,
  username: '',
  authType: 'password' as 'password' | 'keyfile' | 'agent',
  password: '',
  keyFilePath: '',
  tags: [] as string[]
});

const authOptions = [
  { label: 'Password', value: 'password' },
  { label: 'Key File', value: 'keyfile' },
  { label: 'SSH Agent', value: 'agent' }
];

const handleSubmit = async () => {
  await serversStore.addServer(formData.value);
  emit('close');
};
</script>

<template>
  <div class="server-form">
    <h3>Add Server</h3>
    <n-form label-placement="left" label-width="100">
      <n-form-item label="Name">
        <n-input v-model:value="formData.name" placeholder="My Server" />
      </n-form-item>
      <n-form-item label="Group">
        <n-input v-model:value="formData.group" placeholder="Production" />
      </n-form-item>
      <n-form-item label="Host">
        <n-input v-model:value="formData.host" placeholder="192.168.1.100" />
      </n-form-item>
      <n-form-item label="Port">
        <n-input-number v-model:value="formData.port" :min="1" :max="65535" />
      </n-form-item>
      <n-form-item label="Username">
        <n-input v-model:value="formData.username" placeholder="root" />
      </n-form-item>
      <n-form-item label="Auth Type">
        <n-select v-model:value="formData.authType" :options="authOptions" />
      </n-form-item>
      <n-form-item v-if="formData.authType === 'password'" label="Password">
        <n-input type="password" v-model:value="formData.password" />
      </n-form-item>
      <n-form-item v-if="formData.authType === 'keyfile'" label="Key File">
        <n-input v-model:value="formData.keyFilePath" placeholder="~/.ssh/id_rsa" />
      </n-form-item>
    </n-form>
    <n-space justify="end">
      <n-button @click="emit('close')">Cancel</n-button>
      <n-button type="primary" @click="handleSubmit">Save</n-button>
    </n-space>
  </div>
</template>

<style scoped>
.server-form {
  background: #1e1e3f;
  padding: 24px;
  border-radius: 8px;
  width: 450px;
}
</style>