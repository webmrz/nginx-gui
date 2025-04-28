<template>
  <div class="status-indicator">
    <div class="status-dot" :class="statusClass"></div>
    <span class="status-text">{{ statusText }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useNginxStore } from '../stores/nginx'
import { storeToRefs } from 'pinia'

const nginxStore = useNginxStore()
const { isRunning, loading } = storeToRefs(nginxStore)

const statusClass = computed(() => {
  if (loading.value) return 'loading'
  return isRunning.value ? 'running' : 'stopped'
})

const statusText = computed(() => {
  if (loading.value) return '检查中...'
  return isRunning.value ? '运行中' : '未运行'
})
</script>

<style scoped>
.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  transition: background-color 0.3s ease;
}

.status-dot.running {
  background-color: #10B981;
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
}

.status-dot.stopped {
  background-color: #EF4444;
  box-shadow: 0 0 8px rgba(239, 68, 68, 0.5);
}

.status-dot.loading {
  background-color: #F59E0B;
  box-shadow: 0 0 8px rgba(245, 158, 11, 0.5);
  animation: pulse 1.5s infinite;
}

.status-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color);
}

@keyframes pulse {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.7;
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}
</style> 