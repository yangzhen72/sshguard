<script setup lang="ts">
import { ref } from 'vue';
import { useServersStore } from '../stores/servers';

withDefaults(defineProps<{
  show: boolean;
}>(), {
  show: false,
});

const emit = defineEmits(['close', 'update:show']);

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
  { label: '密码', value: 'password' },
  { label: '私钥文件', value: 'keyfile' },
  { label: 'SSH Agent', value: 'agent' }
];

const handleSubmit = async () => {
  await serversStore.addServer(formData.value);
  handleClose();
};

const handleClose = () => {
  emit('update:show', false);
  emit('close');
  // Reset form
  formData.value = {
    name: '',
    group: 'Default',
    host: '',
    port: 22,
    username: '',
    authType: 'password',
    password: '',
    keyFilePath: '',
    tags: []
  };
};
</script>

<template>
  <div v-if="show" class="modal-overlay" @click.self="handleClose">
    <div class="modal-content">
      <div class="modal-header">
        <h3>添加服务器</h3>
        <button class="close-btn" @click="handleClose">✕</button>
      </div>
      
      <div class="modal-body">
        <div class="form-row">
          <label>服务器名称</label>
          <input 
            v-model="formData.name" 
            type="text" 
            placeholder="My Server"
            class="form-input"
          />
        </div>
        
        <div class="form-row">
          <label>分组</label>
          <input 
            v-model="formData.group" 
            type="text" 
            placeholder="Production"
            class="form-input"
          />
        </div>
        
        <div class="form-row-group">
          <div class="form-row">
            <label>主机地址</label>
            <input 
              v-model="formData.host" 
              type="text" 
              placeholder="192.168.1.100"
              class="form-input"
            />
          </div>
          <div class="form-row narrow">
            <label>端口</label>
            <input 
              v-model.number="formData.port" 
              type="number" 
              min="1"
              max="65535"
              class="form-input"
            />
          </div>
        </div>
        
        <div class="form-row">
          <label>用户名</label>
          <input 
            v-model="formData.username" 
            type="text" 
            placeholder="root"
            class="form-input"
          />
        </div>
        
        <div class="form-row">
          <label>认证方式</label>
          <select v-model="formData.authType" class="form-select">
            <option v-for="opt in authOptions" :key="opt.value" :value="opt.value">
              {{ opt.label }}
            </option>
          </select>
        </div>
        
        <div v-if="formData.authType === 'password'" class="form-row">
          <label>密码</label>
          <input 
            v-model="formData.password" 
            type="password" 
            placeholder="Password"
            class="form-input"
          />
        </div>
        
        <div v-if="formData.authType === 'keyfile'" class="form-row">
          <label>私钥路径</label>
          <input 
            v-model="formData.keyFilePath" 
            type="text" 
            placeholder="~/.ssh/id_rsa"
            class="form-input"
          />
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn" @click="handleClose">取消</button>
        <button class="btn primary" @click="handleSubmit">保存</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  width: 420px;
  max-width: 90vw;
  box-shadow: var(--shadow-lg);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border-default);
}

.modal-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--radius-sm);
  font-size: 14px;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.modal-body {
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-row {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-row-group {
  display: flex;
  gap: var(--spacing-md);
}

.form-row-group .form-row {
  flex: 1;
}

.form-row-group .form-row.narrow {
  flex: 0 0 100px;
}

.form-row label {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.form-input,
.form-select {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 13px;
  transition: border-color 0.2s ease;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.form-input::placeholder {
  color: var(--text-muted);
}

.form-select {
  cursor: pointer;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border-default);
}

.btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.15s ease;
}

.btn:hover {
  background: var(--bg-hover);
}

.btn.primary {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: #fff;
}

.btn.primary:hover {
  background: var(--accent-hover);
}
</style>
