<script setup lang="ts">
import { computed, ref } from 'vue';
import { useServersStore } from '../stores/servers';
import { useTerminalsStore } from '../stores/terminals';

const props = defineProps<{
  search: string;
  selectedKeys?: string[];
}>();

const emit = defineEmits<{
  (e: 'update:selected-keys', keys: string[]): void;
  (e: 'connect', serverId: string): void;
}>();

const serversStore = useServersStore();
const terminalsStore = useTerminalsStore();

const collapsedGroups = ref<Set<string>>(new Set());

// Group servers by group name
const groupedServers = computed(() => {
  const groups: Record<string, typeof serversStore.servers> = {};
  
  serversStore.servers.forEach(server => {
    if (!groups[server.group]) {
      groups[server.group] = [];
    }
    groups[server.group].push(server);
  });
  
  return groups;
});

// Filter groups and servers based on search
const filteredGroups = computed(() => {
  if (!props.search) {
    return groupedServers.value;
  }
  
  const query = props.search.toLowerCase();
  const filtered: Record<string, typeof serversStore.servers> = {};
  
  Object.entries(groupedServers.value).forEach(([group, servers]) => {
    const matchingServers = servers.filter(s => 
      s.name.toLowerCase().includes(query) ||
      s.host.toLowerCase().includes(query) ||
      s.tags.some(t => t.toLowerCase().includes(query))
    );
    
    if (matchingServers.length > 0) {
      filtered[group] = matchingServers;
    }
  });
  
  return filtered;
});

const toggleGroup = (groupName: string) => {
  if (collapsedGroups.value.has(groupName)) {
    collapsedGroups.value.delete(groupName);
  } else {
    collapsedGroups.value.add(groupName);
  }
};

const isGroupCollapsed = (groupName: string) => {
  return collapsedGroups.value.has(groupName);
};

const isServerConnected = (serverId: string) => {
  return terminalsStore.tabs.some(t => t.serverId === serverId);
};

const handleServerClick = (serverId: string, event: MouseEvent) => {
  if (event.ctrlKey || event.metaKey) {
    // Multi-select with Ctrl
    const keys = props.selectedKeys ? [...props.selectedKeys] : [];
    const index = keys.indexOf(serverId);
    if (index >= 0) {
      keys.splice(index, 1);
    } else {
      keys.push(serverId);
    }
    emit('update:selected-keys', keys);
  } else {
    // Single select
    emit('update:selected-keys', [serverId]);
  }
};

const handleServerDoubleClick = (serverId: string) => {
  emit('connect', serverId);
};
</script>

<template>
  <div class="server-tree">
    <div 
      v-for="(servers, groupName) in filteredGroups" 
      :key="groupName"
      class="server-group"
    >
      <!-- 分组标题 -->
      <div 
        class="group-header"
        :class="{ collapsed: isGroupCollapsed(groupName) }"
        @click="toggleGroup(groupName)"
      >
        <span class="group-icon">{{ isGroupCollapsed(groupName) ? '▶' : '▼' }}</span>
        <span class="group-name">{{ groupName }}</span>
        <span class="group-count">{{ servers.length }}</span>
      </div>
      
      <!-- 服务器列表 -->
      <div v-if="!isGroupCollapsed(groupName)" class="group-servers">
        <div
          v-for="server in servers"
          :key="server.id"
          class="server-item"
          :class="{ 
            selected: selectedKeys?.includes(server.id),
            connected: isServerConnected(server.id)
          }"
          @click="handleServerClick(server.id, $event)"
          @dblclick="handleServerDoubleClick(server.id)"
        >
          <span class="server-status" :class="{ connected: isServerConnected(server.id) }"></span>
          <span class="server-icon">💻</span>
          <span class="server-name">{{ server.name }}</span>
        </div>
      </div>
    </div>
    
    <!-- 空状态 -->
    <div v-if="Object.keys(filteredGroups).length === 0" class="empty-state">
      <span>没有找到服务器</span>
    </div>
  </div>
</template>

<style scoped>
.server-tree {
  font-size: 13px;
}

.server-group {
  margin-bottom: var(--spacing-xs);
}

.group-header {
  display: flex;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  cursor: pointer;
  color: var(--text-secondary);
  border-radius: var(--radius-sm);
  user-select: none;
  transition: all 0.15s ease;
}

.group-header:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.group-icon {
  font-size: 10px;
  margin-right: var(--spacing-xs);
  width: 12px;
}

.group-name {
  flex: 1;
  font-weight: 500;
}

.group-count {
  font-size: 11px;
  color: var(--text-muted);
  background: var(--bg-tertiary);
  padding: 0 var(--spacing-xs);
  border-radius: 10px;
}

.group-servers {
  margin-top: 2px;
}

.server-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  padding-left: calc(var(--spacing-lg) + var(--spacing-sm));
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: all 0.15s ease;
}

.server-item:hover {
  background: var(--bg-hover);
}

.server-item.selected {
  background: rgba(0, 152, 255, 0.15);
}

.server-item.selected .server-name {
  color: var(--accent-primary);
}

.server-status {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  margin-right: var(--spacing-sm);
  background: var(--text-muted);
  transition: background 0.2s ease;
}

.server-status.connected {
  background: var(--success);
}

.server-icon {
  font-size: 12px;
  margin-right: var(--spacing-xs);
}

.server-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-state {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--text-muted);
  font-size: 13px;
}
</style>
