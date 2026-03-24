<script setup lang="ts">
import { onMounted } from 'vue';
import Sidebar from '../components/Sidebar.vue';
import TabBar from '../components/TabBar.vue';
import TerminalPanel from '../components/TerminalPanel.vue';
import SftpPanel from '../components/SftpPanel.vue';
import BatchCommandPanel from '../components/BatchCommandPanel.vue';
import UpdateDialog from '../components/UpdateDialog.vue';
import { useUpdateStore } from '../stores/update';

const updateStore = useUpdateStore();

onMounted(() => {
  updateStore.checkForUpdates();
});
</script>

<template>
  <div class="main-layout">
    <Sidebar class="sidebar" />
    
    <div class="content-area">
      <div class="main-workspace">
        <TabBar class="tab-bar" />
        <TerminalPanel class="terminal-area" />
      </div>
      
      <SftpPanel class="sftp-panel" />
    </div>
    
    <BatchCommandPanel class="batch-panel" />
    
    <UpdateDialog />
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
}

.sidebar {
  flex-shrink: 0;
}

.content-area {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.main-workspace {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tab-bar {
  flex-shrink: 0;
}

.terminal-area {
  flex: 1;
}

.sftp-panel {
  flex-shrink: 0;
  border-left: 1px solid var(--border-default);
}

.batch-panel {
  position: fixed;
  bottom: 40px;
  right: 16px;
  z-index: 100;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
}
</style>
