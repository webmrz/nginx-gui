<script setup lang="ts">
import { onMounted } from 'vue'
import { NButton, NCard, NIcon, NSpin, NAlert, useMessage } from 'naive-ui'
import { PlayOutline, StopOutline, RefreshOutline } from '@vicons/ionicons5'
import { useNginxStore } from '../../stores/nginx'
import { storeToRefs } from 'pinia'

const message = useMessage()
const nginxStore = useNginxStore()
const { isRunning, loading, error } = storeToRefs(nginxStore)

onMounted(() => {
  console.log('ServiceStatus component mounted')
  nginxStore.fetchStatus().catch(err => {
    console.error('获取状态失败:', err)
  })
})

const handleStart = async () => {
  try {
    const result = await nginxStore.startService()
    if (result.success) {
      message.success('服务启动成功')
    } else {
      message.error(result.message || '服务启动失败')
    }
  } catch (error) {
    message.error('服务启动失败：' + (error as Error).message)
  }
}

const handleStop = async () => {
  try {
    const result = await nginxStore.stopService()
    if (result.success) {
      message.success('服务停止成功')
    } else {
      message.error(result.message || '服务停止失败')
    }
  } catch (error) {
    message.error('服务停止失败：' + (error as Error).message)
  }
}

const handleRestart = async () => {
  try {
    const result = await nginxStore.restartService()
    if (result.success) {
      message.success('服务重启成功')
    } else {
      message.error(result.message || '服务重启失败')
    }
  } catch (error) {
    message.error('服务重启失败：' + (error as Error).message)
  }
}

const handleRefresh = async () => {
  try {
    await nginxStore.fetchStatus()
  } catch (error) {
    message.error('刷新状态失败：' + (error as Error).message)
  }
}
</script>

<template>
  <NCard title="服务状态" class="service-status">
    <template #header-extra>
      <NButton @click="handleRefresh" :loading="loading">
        <template #icon>
          <NIcon><RefreshOutline /></NIcon>
        </template>
        刷新
      </NButton>
    </template>

    <NSpin :show="loading">
      <div v-if="error" class="error-message">
        <NAlert type="error" :title="error" />
      </div>

      <div class="status-content">
        <div class="status-info">
          <span>当前状态：</span>
          <span :class="['status', isRunning ? 'running' : 'stopped']">
            {{ isRunning ? '运行中' : '已停止' }}
          </span>
        </div>

        <div class="action-buttons">
          <NButton 
            type="primary" 
            @click="handleStart" 
            :disabled="isRunning || loading"
          >
            <template #icon>
              <NIcon><PlayOutline /></NIcon>
            </template>
            启动
          </NButton>

          <NButton 
            type="error" 
            @click="handleStop" 
            :disabled="!isRunning || loading"
          >
            <template #icon>
              <NIcon><StopOutline /></NIcon>
            </template>
            停止
          </NButton>

          <NButton 
            type="warning" 
            @click="handleRestart" 
            :disabled="!isRunning || loading"
          >
            <template #icon>
              <NIcon><RefreshOutline /></NIcon>
            </template>
            重启
          </NButton>
        </div>
      </div>
    </NSpin>
  </NCard>
</template>

<style scoped>
.service-status {
  margin-bottom: 20px;
}

.status-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.status-info {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 16px;
}

.status {
  font-weight: bold;
}

.status.running {
  color: #10B981;
}

.status.stopped {
  color: #EF4444;
}

.action-buttons {
  display: flex;
  gap: 10px;
}

.error-message {
  margin-bottom: 16px;
}
</style> 