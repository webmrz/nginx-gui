<script setup lang="ts">
import { ref, onMounted, onUnmounted, h, watch, nextTick, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  NCard,
  NButton,
  NIcon,
  NSpace,
  NSelect,
  NInput,
  NModal,
  NPopconfirm,
  useMessage,
  type SelectOption,
  NSpin,
  NDropdown,
  type DropdownOption
} from 'naive-ui'
import {
  RefreshOutline,
  TrashOutline,
  SearchOutline,
  ArrowDownOutline,
  ArrowUpOutline,
  FolderOpenOutline,
  CopyOutline
} from '@vicons/ionicons5'
import ErrorAlert from '../components/Utils/ErrorAlert.vue'
import LoadingSpinner from '../components/Utils/LoadingSpinner.vue'

const selectedLogType = ref<string>('')
const logs = ref<string[]>([])
const loading = ref(false)
const error = ref('')
const autoRefresh = ref(true)
const message = useMessage()
let refreshInterval: NodeJS.Timeout
const showClearDialog = ref(false)

// 搜索和过滤
const searchQuery = ref('')
const logLevel = ref<string>('')
const pageSize = ref(1000) // 一次加载更多行

const logTypeOptions: SelectOption[] = [
  { label: '访问日志', value: 'access' },
  { label: '错误日志', value: 'error' },
  { label: '服务日志', value: 'service' }
]

const logLevelOptions: SelectOption[] = [
  { label: '全部', value: '' },
  { label: '调试', value: 'debug' },
  { label: '信息', value: 'info' },
  { label: '警告', value: 'warn' },
  { label: '错误', value: 'error' }
]

const logContainerRef = ref<HTMLElement | null>(null)

// 右键菜单相关
const showContextMenu = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const selectedLine = ref<string>('')

const handleContextMenu = (event: MouseEvent, line: string) => {
  event.preventDefault()
  contextMenuX.value = event.clientX
  contextMenuY.value = event.clientY
  selectedLine.value = line
  showContextMenu.value = true
}

const hideContextMenu = () => {
  showContextMenu.value = false
}

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    message.success('已复制到剪贴板')
  } catch (err) {
    message.error('复制失败')
  }
}

const contextMenuOptions = computed<DropdownOption[]>(() => [
  {
    label: '复制',
    key: 'copy',
    icon: () => h(NIcon, null, { default: () => h(CopyOutline) }),
    onClick: () => copyToClipboard(selectedLine.value)
  },
  {
    label: '清空日志',
    key: 'clear',
    icon: () => h(NIcon, null, { default: () => h(TrashOutline) }),
    onClick: () => {
      showClearDialog.value = true
      hideContextMenu()
    }
  },
  {
    label: '打开日志文件夹',
    key: 'openFolder',
    icon: () => h(NIcon, null, { default: () => h(FolderOpenOutline) }),
    onClick: () => {
      openLogFolder()
      hideContextMenu()
    }
  }
])

// 动态计算日志容器高度
const logContainerHeight = computed(() => {
  // 获取视口高度
  const viewportHeight = window.innerHeight
  // 减去顶部操作区域高度（约150px）和其他边距（约50px）
  return `${viewportHeight - 200}px`
})

// 监听窗口大小变化，更新日志容器高度
const updateLogContainerHeight = () => {
  if (logContainerRef.value) {
    logContainerRef.value.style.height = logContainerHeight.value
  }
}

const fetchLogs = async () => {
  if (!selectedLogType.value) return
  try {
    loading.value = true
    error.value = ''
    console.log(`[DEBUG] 开始获取日志，类型: ${selectedLogType.value}, 行数: ${pageSize.value}`)
    const result = await invoke<string>('get_nginx_logs', {
      logType: selectedLogType.value,
      lines: pageSize.value,
      search: searchQuery.value || undefined,
      level: logLevel.value || undefined
    })
    logs.value = result.split('\n')
    console.log(`[INFO] 成功获取日志，共 ${logs.value.length} 行`)
    await nextTick()
    scrollToBottom()
  } catch (err) {
    console.error('[ERROR] 获取日志失败:', err)
    error.value = err instanceof Error ? err.message : '获取日志失败'
    message.error('获取日志失败')
  } finally {
    loading.value = false
  }
}

const clearLogs = async () => {
  if (!selectedLogType.value) {
    message.error('请先选择要清除的日志类型')
    return
  }
  
  try {
    loading.value = true
    error.value = ''
    console.log(`[DEBUG] 开始清除日志，类型: ${selectedLogType.value}`)
    
    // 检查日志文件是否存在
    const logExists = await invoke('check_log_exists', {
      logType: selectedLogType.value
    })
    console.log(`[DEBUG] 日志文件存在检查结果:`, logExists)
    
    // 尝试清除日志，最多重试3次
    let retryCount = 0
    const maxRetries = 3
    let lastError = null
    
    while (retryCount < maxRetries) {
      try {
        await invoke('clear_logs', {
          logType: selectedLogType.value
        })
        console.log('[DEBUG] 清除日志命令执行成功')
        await fetchLogs()
        message.success('日志清除成功')
        break
      } catch (err) {
        lastError = err
        retryCount++
        console.error(`[ERROR] 清除日志失败 (尝试 ${retryCount}/${maxRetries}):`, err)
        if (retryCount < maxRetries) {
          // 等待1秒后重试
          await new Promise(resolve => setTimeout(resolve, 1000))
        }
      }
    }
    
    if (retryCount === maxRetries && lastError) {
      throw lastError
    }
  } catch (err) {
    console.error('[ERROR] 清除日志失败:', err)
    error.value = err instanceof Error ? err.message : '清除日志失败'
    message.error(`清除日志失败: ${error.value}`)
  } finally {
    loading.value = false
    showClearDialog.value = false
  }
}

const openLogFolder = async () => {
  try {
    await invoke('open_log_folder')
    message.success('日志文件夹已打开')
  } catch (err) {
    error.value = err instanceof Error ? err.message : '打开日志文件夹失败'
    message.error('打开日志文件夹失败')
  }
}

const toggleAutoRefresh = () => {
  autoRefresh.value = !autoRefresh.value
  if (autoRefresh.value) {
    startAutoRefresh()
    message.success('已开启自动刷新')
  } else {
    stopAutoRefresh()
    message.info('已关闭自动刷新')
  }
}

const startAutoRefresh = () => {
  refreshInterval = setInterval(fetchLogs, 5000)
}

const stopAutoRefresh = () => {
  if (refreshInterval) {
    clearInterval(refreshInterval)
  }
}

const scrollToBottom = () => {
  nextTick(() => {
    if (logContainerRef.value) {
      logContainerRef.value.scrollTop = logContainerRef.value.scrollHeight
    }
  })
}
const scrollToTop = () => {
  if (logContainerRef.value) {
    logContainerRef.value.scrollTop = 0
  }
}

watch(logs, () => {
  scrollToBottom()
})

onMounted(() => {
  selectedLogType.value = 'access'
  fetchLogs()
  if (autoRefresh.value) {
    startAutoRefresh()
  }
  // 添加窗口大小变化监听
  window.addEventListener('resize', updateLogContainerHeight)
  // 初始化高度
  updateLogContainerHeight()
})

onUnmounted(() => {
  stopAutoRefresh()
  // 移除窗口大小变化监听
  window.removeEventListener('resize', updateLogContainerHeight)
})

// 点击页面任意位置关闭右键菜单
const handleClickOutside = (event: MouseEvent) => {
  if (showContextMenu.value) {
    hideContextMenu()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<template>
  <div class="space-y-6">
    <div class="flex justify-between items-center">
      <NSpace>
        <NSelect
          v-model:value="selectedLogType"
          :options="logTypeOptions"
          style="width: 150px"
          placeholder="选择日志类型"
          @update:value="fetchLogs"
        />
        <NInput
          v-model:value="searchQuery"
          placeholder="搜索日志..."
          clearable
          @keyup.enter="fetchLogs"
        >
          <template #prefix>
            <NIcon>
              <SearchOutline />
            </NIcon>
          </template>
        </NInput>
        <NSelect
          v-model:value="logLevel"
          :options="logLevelOptions"
          style="width: 120px"
          placeholder="日志级别"
          @update:value="fetchLogs"
        />
        <!-- <NButton
          :type="autoRefresh ? 'primary' : 'default'"
          @click="toggleAutoRefresh"
        >
          <template #icon>
            <NIcon>
              <RefreshOutline />
            </NIcon>
          </template>
          自动刷新
        </NButton> -->
        <NButton
          type="info"
          @click="openLogFolder"
        >
          <template #icon>
            <NIcon>
              <FolderOpenOutline />
            </NIcon>
          </template>
          打开日志文件夹
        </NButton>
        <NButton
          type="error"
          @click="showClearDialog = true"
        >
          <template #icon>
            <NIcon>
              <TrashOutline />
            </NIcon>
          </template>
          清空日志
        </NButton>
      </NSpace>
    </div>
    <ErrorAlert :error="error" />
    <NCard>
      <NSpin :show="loading">
        <div class="relative">
          <div class="absolute right-4 top-4 z-10 flex flex-col gap-3">
            <NButton size="medium" @click="scrollToTop" circle type="primary">
              <template #icon><NIcon><ArrowUpOutline /></NIcon></template>
            </NButton>
            <NButton size="medium" @click="scrollToBottom" circle type="primary">
              <template #icon><NIcon><ArrowDownOutline /></NIcon></template>
            </NButton>
          </div>
          <div
            ref="logContainerRef"
            class="log-container-wrapper overflow-auto border rounded bg-black text-white font-mono"
            :style="{ height: logContainerHeight }"
          >
            <div 
              v-for="(line, idx) in logs" 
              :key="idx" 
              class="flex items-start px-2 py-0.5 hover:bg-gray-800 cursor-pointer" 
              :class="{ 'bg-red-900 font-bold text-red-200': /error|ERROR/.test(line) }"
              @contextmenu="handleContextMenu($event, line)"
            >
              <span class="w-12 text-right select-none text-gray-400 pr-2">{{ idx + 1 }}</span>
              <span class="whitespace-pre-line break-all">{{ line }}</span>
            </div>
          </div>
        </div>
      </NSpin>
    </NCard>
    <!-- 清空日志对话框 -->
    <NModal
      v-model:show="showClearDialog"
      preset="dialog"
      title="清空日志"
      positive-text="清空"
      negative-text="取消"
      @positive-click="clearLogs"
    >
      <p>确定要清空当前日志吗？此操作不可恢复。</p>
    </NModal>
    
    <!-- 右键菜单 -->
    <NDropdown
      v-model:show="showContextMenu"
      :x="contextMenuX"
      :y="contextMenuY"
      :options="contextMenuOptions"
      trigger="manual"
    />
  </div>
</template>

<style scoped>
.log-container-wrapper {
  /* 让日志区域自适应高度，铺满除顶部操作区外的屏幕 */
  transition: height 0.2s;
}
</style> 