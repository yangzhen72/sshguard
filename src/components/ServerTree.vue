<script setup lang="ts">
import { computed } from 'vue';
import { NTree } from 'naive-ui';
import { useServersStore } from '../stores/servers';

const props = defineProps<{
  search: string;
  selectedKeys?: string[];
  multiple?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:selected-keys', keys: string[]): void;
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

const internalSelectedKeys = computed(() => props.selectedKeys || []);

const handleSelect = (keys: string[]) => {
  if (props.multiple) {
    emit('update:selected-keys', keys);
  } else {
    if (keys.length > 0) {
      const key = keys[0];
      if (!key.includes('/')) {
        serversStore.selectServer(key);
      }
    }
    emit('update:selected-keys', keys);
  }
};
</script>

<template>
  <n-tree
    :data="treeData"
    :selected-keys="internalSelectedKeys"
    :multiple="multiple"
    block-line
    expand-on-click
    @update:selected-keys="handleSelect"
  />
</template>
