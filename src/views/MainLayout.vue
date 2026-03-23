<script setup lang="ts">
import Sidebar from '../components/Sidebar.vue';
</script>

<template>
  <div class="main-layout">
    <Sidebar />
    <main class="content">Content</main>
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
}
.content {
  flex: 1;
  background: #1a1a2e;
}
</style>