<script setup lang="ts">
import Sidebar from '../components/Sidebar.vue';
import TabBar from '../components/TabBar.vue';
import TerminalPanel from '../components/TerminalPanel.vue';
</script>

<template>
  <div class="main-layout">
    <Sidebar />
    <div class="terminal-area">
      <TabBar />
      <TerminalPanel />
    </div>
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
}

.terminal-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
