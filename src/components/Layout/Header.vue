<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { useNginxStore } from '../../stores/nginx'
import { storeToRefs } from 'pinia'
import { useMessage } from 'naive-ui'

const emit = defineEmits<{
  (e: 'toggle-sidebar'): void
}>()

const message = useMessage()
const nginxStore = useNginxStore()
const { isRunning, statusText, loading } = storeToRefs(nginxStore)

let statusCheckInterval: number | null = null
const isInitialized = ref(false)

const toggleNginx = async () => {
  try {
    if (isRunning.value) {
      const result = await nginxStore.stopService()
      if (result.success) {
        message.success('服务停止成功')
      } else {
        message.error(result.message || '服务停止失败')
      }
    } else {
      const result = await nginxStore.startService()
      if (result.success) {
        message.success('服务启动成功')
      } else {
        message.error(result.message || '服务启动失败')
      }
    }
  } catch (error) {
    message.error('操作失败：' + (error as Error).message)
  }
}

const startStatusCheck = () => {
  if (statusCheckInterval) {
    clearInterval(statusCheckInterval)
  }
  
  statusCheckInterval = window.setInterval(async () => {
    // try {
    //   await nginxStore.fetchStatus()
    // } catch (error) {
    //   console.error('状态检查失败:', error)
    // }
  }, 5000)
}

const initialize = async () => {
  if (!isInitialized.value) {
    try {
      const result = await nginxStore.initialize()
      if (result.success) {
        isInitialized.value = true
        startStatusCheck()
      } else {
        message.error(result.message || '初始化失败')
      }
    } catch (error) {
      message.error('初始化失败：' + (error as Error).message)
    }
  }
}

onMounted(() => {
  initialize()
})

onUnmounted(() => {
  if (statusCheckInterval) {
    clearInterval(statusCheckInterval)
    statusCheckInterval = null
  }
})
</script>

<template>
  <header class="bg-white shadow">
    <div class="px-4 py-3 flex items-center justify-between">
      <!-- 左侧：菜单按钮 -->
      <button
        @click="emit('toggle-sidebar')"
        class="p-2 text-gray-600 hover:bg-gray-100 rounded-md transition-colors"
      >
        <svg
          class="w-6 h-6"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 6h16M4 12h16M4 18h16"
          />
        </svg>
      </button>

      <!-- 右侧：Nginx 状态和控制按钮 -->
      <div class="flex items-center space-x-4">
        <div class="flex items-center">
          <span
            :class="{
              'bg-green-500': isRunning,
              'bg-red-500': !isRunning && !loading,
              'bg-yellow-500': loading
            }"
            class="w-3 h-3 rounded-full mr-2"
          ></span>
          <span class="text-sm text-gray-600">
            {{ statusText }}
          </span>
        </div>

        <button
          @click="toggleNginx"
          :disabled="loading"
          :class="{
            'bg-green-500 hover:bg-green-600': !isRunning,
            'bg-red-500 hover:bg-red-600': isRunning
          }"
          class="px-4 py-2 text-white rounded-md transition-colors disabled:opacity-50"
        >
          {{ loading ? '处理中...' : isRunning ? '停止' : '启动' }}
        </button>
      </div>
    </div>
  </header>
</template> 