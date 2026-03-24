<script setup lang="ts">
import MainLayout from "./views/MainLayout.vue";
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <n-dialog-provider>
        <MainLayout class="cyberpunk-app" />
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script lang="ts">
import { darkTheme } from "naive-ui";
</script>

<style>
@import './styles/cyberpunk.css';

html, body, #app {
  margin: 0;
  padding: 0;
  height: 100%;
  overflow: hidden;
  font-family: 'Segoe UI', system-ui, sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
}
</style>
