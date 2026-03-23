<script setup lang="ts">
</script>

<template>
  <div class="main-layout">
    <aside class="sidebar">Sidebar</aside>
    <main class="content">Content</main>
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
}
.sidebar {
  width: 250px;
  background: #16162a;
}
.content {
  flex: 1;
  background: #1a1a2e;
}
</style>
