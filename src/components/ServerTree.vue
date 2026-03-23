<script setup lang="ts">
import { computed } from 'vue';
import { NTree } from 'naive-ui';
import { useServersStore } from '../stores/servers';

const props = defineProps<{
  search: string;
}>();

const serversStore = useServersStore();

const treeData = computed(() => {
  const servers = serversStore.filteredServers(props.search);
  const groups: Record<string, any> = {};
  
  servers.forEach(server => {
    if (!groups[server.group]) {
      groups[server.group] = {
        key: server.group,
        label: server.group,
        isDirectory: true,
        children: []
      };
    }
    groups[server.group].children.push({
      key: server.id,
      label: server.name,
      server
    });
  });
  
  return Object.values(groups);
});

const handleSelect = (keys: string[]) => {
  if (keys.length > 0) {
    const key = keys[0];
    if (!key.includes('/')) {
      serversStore.selectServer(key);
    }
  }
};
</script>

<template>
  <n-tree
    :data="treeData"
    block-line
    expand-on-click
    @update:selected-keys="handleSelect"
  />
</template>