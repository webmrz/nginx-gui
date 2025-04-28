<template>
  <div class="dashboard-container p-6 space-y-8">
    <!-- 顶部操作栏 -->
    <div class="flex justify-between items-center">
      <div class="flex items-center space-x-4">
        <h1 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
          仪表盘
        </h1>
        <NText depth="3" class="text-sm">
          实时监控 Nginx 服务状态
        </NText>
      </div>
      <NSpace size="large">
        <NButton
          type="primary"
          :disabled="isRunning || loading"
          @click="startService"
          class="min-w-[120px]"
          size="large"
        >
          <template #icon>
            <NIcon>
              <PlayOutline />
            </NIcon>
          </template>
          {{ loading ? '处理中...' : '启动服务' }}
        </NButton>
        <NButton
          type="error"
          :disabled="!isRunning || loading"
          @click="stopService"
          class="min-w-[120px]"
          size="large"
        >
          <template #icon>
            <NIcon>
              <StopOutline />
            </NIcon>
          </template>
          {{ loading ? '处理中...' : '停止服务' }}
        </NButton>
        <NButton
          type="warning"
          :disabled="!isRunning || loading"
          @click="restartService"
          class="min-w-[120px]"
          size="large"
        >
          <template #icon>
            <NIcon>
              <RefreshOutline />
            </NIcon>
          </template>
          {{ loading ? '处理中...' : '重启服务' }}
        </NButton>
      </NSpace>
    </div>

    <!-- 服务状态与系统资源信息 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <!-- 服务状态卡片 -->
      <NCard title="服务信息" class="h-full shadow-sm hover:shadow-md transition-all duration-300 bg-white/50 backdrop-blur-sm">
        <NGrid :cols="2" :x-gap="16" :y-gap="16">
          <NGridItem>
            <NStatistic label="运行状态" class="statistic-card">
              <template #prefix>
                <NIcon size="24" class="text-blue-500">
                  <ServerOutline />
                </NIcon>
              </template>
              <NText :type="isRunning ? 'success' : 'error'" class="text-lg font-medium">
                {{ statusText }}
              </NText>
            </NStatistic>
          </NGridItem>
          <NGridItem>
            <NStatistic label="版本" class="statistic-card">
              <template #prefix>
                <NIcon size="24" class="text-purple-500">
                  <SpeedometerOutline />
                </NIcon>
              </template>
              <span class="text-lg font-medium">{{ version || '未知' }}</span>
            </NStatistic>
          </NGridItem>
          <NGridItem>
            <NStatistic label="运行时间" class="statistic-card">
              <template #prefix>
                <NIcon size="24" class="text-green-500">
                  <TimeOutline />
                </NIcon>
              </template>
              <span class="text-lg font-medium">{{ uptime || '0s' }}</span>
            </NStatistic>
          </NGridItem>
          <NGridItem>
            <NStatistic label="初始化状态" class="statistic-card">
              <template #prefix>
                <NIcon size="24" class="text-orange-500">
                  <DocumentTextOutline />
                </NIcon>
              </template>
              <NText :type="initialized ? 'success' : 'warning'" class="text-lg font-medium">
                {{ initialized ? '已初始化' : '未初始化' }}
              </NText>
            </NStatistic>
          </NGridItem>
        </NGrid>
      </NCard>

      <!-- 系统资源卡片 -->
      <NCard title="系统资源" class="h-full shadow-sm hover:shadow-md transition-all duration-300 bg-white/50 backdrop-blur-sm">
        <NGrid :cols="2" :x-gap="16" :y-gap="16">
          <NGridItem>
            <NStatistic label="CPU 使用率" class="statistic-card">
              <template #prefix>
                <NIcon size="24" class="text-red-500">
                  <HardwareChipOutline />
                </NIcon>
              </template>
              <NProgress
                type="line"
                :percentage="parseFloat(cpuUsage)"
                :color="parseFloat(cpuUsage) > 80 ? '#D03050' : parseFloat(cpuUsage) > 50 ? '#F0A020' : '#18A058'"
                :height="12"
                :border-radius="6"
                :show-indicator="true"
                :indicator-text-color="parseFloat(cpuUsage) > 80 ? '#D03050' : parseFloat(cpuUsage) > 50 ? '#F0A020' : '#18A058'"
                :indicator="{
                  textColor: parseFloat(cpuUsage) > 80 ? '#D03050' : parseFloat(cpuUsage) > 50 ? '#F0A020' : '#18A058',
                  content: cpuUsage + '%'
                }"
                class="mt-2"
              />
            </NStatistic>
          </NGridItem>
          <NGridItem>
            <NStatistic label="内存使用率" class="statistic-card">
              <template #prefix>
                <NIcon size="24" class="text-blue-500">
                  <SaveOutline />
                </NIcon>
              </template>
              <NProgress
                type="line"
                :percentage="parseFloat(memoryUsage)"
                :color="parseFloat(memoryUsage) > 80 ? '#D03050' : parseFloat(memoryUsage) > 50 ? '#F0A020' : '#18A058'"
                :height="12"
                :border-radius="6"
                :show-indicator="true"
                :indicator-text-color="parseFloat(memoryUsage) > 80 ? '#D03050' : parseFloat(memoryUsage) > 50 ? '#F0A020' : '#18A058'"
                :indicator="{
                  textColor: parseFloat(memoryUsage) > 80 ? '#D03050' : parseFloat(memoryUsage) > 50 ? '#F0A020' : '#18A058',
                  content: memoryUsage + '%'
                }"
                class="mt-2"
              />
            </NStatistic>
          </NGridItem>
        </NGrid>
      </NCard>
    </div>

    <!-- 流量统计卡片 -->
    <NCard title="流量统计" class="shadow-sm hover:shadow-md transition-all duration-300 bg-white/50 backdrop-blur-sm">
      <NGrid :cols="3" :x-gap="16" :y-gap="16">
        <NGridItem>
          <NStatistic label="活动连接" class="statistic-card">
            <template #prefix>
              <NIcon size="24" class="text-blue-500">
                <ServerOutline />
              </NIcon>
            </template>
            <span class="text-2xl font-bold">{{ isRunning ? activeConnections : 0 }}</span>
          </NStatistic>
        </NGridItem>
        <NGridItem>
          <NStatistic label="每秒请求数" class="statistic-card">
            <template #prefix>
              <NIcon size="24" class="text-green-500">
                <SpeedometerOutline />
              </NIcon>
            </template>
            <span class="text-2xl font-bold">{{ isRunning ? requestsPerSecond : 0 }}</span>
          </NStatistic>
        </NGridItem>
        <NGridItem>
          <NStatistic label="总请求数" class="statistic-card">
            <template #prefix>
              <NIcon size="24" class="text-purple-500">
                <DocumentTextOutline />
              </NIcon>
            </template>
            <span class="text-2xl font-bold">{{ isRunning ? totalConnections : 0 }}</span>
          </NStatistic>
        </NGridItem>
      </NGrid>

      <NAlert v-if="!isRunning" type="info" class="mt-6">
        <template #icon>
          <NIcon>
            <InformationCircleOutline />
          </NIcon>
        </template>
        当 Nginx 未运行时，流量统计数据将显示为零。启动 Nginx 后可查看实时统计数据。
      </NAlert>
    </NCard>

    <!-- 状态提示 -->
    <NAlert v-if="loading" type="info" class="mt-6">
      <template #icon>
        <NIcon>
          <WarningOutline />
        </NIcon>
      </template>
      正在加载数据，请稍候...
    </NAlert>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { useNginxStore } from '../stores/nginx'
import { storeToRefs } from 'pinia'
import { 
  NButton,
  NSpace,
  NIcon,
  NCard,
  NStatistic,
  NProgress,
  NGrid,
  NGridItem,
  NText,
  NAlert,
  NSpin,
  NTime,
  useMessage
} from 'naive-ui'
import {
  PlayOutline,
  StopOutline,
  RefreshOutline,
  ServerOutline,
  SpeedometerOutline,
  HardwareChipOutline,
  SaveOutline,
  InformationCircleOutline,
  TimeOutline,
  DocumentTextOutline,
  WarningOutline
} from '@vicons/ionicons5'

const message = useMessage()
const nginxStore = useNginxStore()
const { 
  isRunning, 
  statusText, 
  version, 
  uptime, 
  cpuUsage, 
  memoryUsage, 
  activeConnections, 
  totalConnections, 
  requestsPerSecond, 
  loading, 
  initialized 
} = storeToRefs(nginxStore)

// 服务控制方法
const startService = async () => {
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

const stopService = async () => {
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

const restartService = async () => {
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

// 自动刷新状态
let refreshInterval: number | null = null

onMounted(async () => {
  try {
    const result = await nginxStore.initialize()
    if (!result.success) {
      message.error(result.message || '初始化失败')
    } else {
      // 只有在初始化成功后才开始自动刷新
      refreshInterval = window.setInterval(async () => {
        // try {
        //   await nginxStore.fetchStatus()
        // } catch (error) {
        //   console.error('状态刷新失败:', error)
        // }
      }, 5000) // 每5秒刷新一次
    }
  } catch (error) {
    message.error('初始化失败：' + (error as Error).message)
  }
})

onUnmounted(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval)
    refreshInterval = null
  }
})
</script>

<style scoped>
.dashboard-container { 
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.statistic-card {
  @apply p-4 rounded-lg bg-white/80 backdrop-blur-sm shadow-sm hover:shadow-md transition-all duration-300;
}

:deep(.n-card) {
  @apply border-0;
}

:deep(.n-card-header) {
  @apply text-lg font-medium text-gray-800;
}

:deep(.n-statistic-label) {
  @apply text-sm text-gray-600;
}

:deep(.n-statistic-value) {
  @apply text-2xl font-bold text-gray-900;
}

:deep(.n-progress) {
  @apply mt-2;
}

:deep(.n-progress-graph) {
  @apply rounded-full;
}

:deep(.n-progress-text) {
  @apply text-sm font-medium;
}
</style> 