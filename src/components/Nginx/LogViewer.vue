<script setup lang="ts">
import { ref, onMounted, onUnmounted, h, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  NCard,
  NButton,
  NIcon,
  NSpace,
  NSelect,
  NInput,
  NSpin,
  NAlert,
  NScrollbar,
  NMessageProvider,
  useMessage,
  type SelectOption,
  NInputNumber,
  NTooltip
} from 'naive-ui'
import {
  SearchOutline,
  RefreshOutline,
  DownloadOutline,
  TrashOutline,
  FilterOutline,
  ArrowUpOutline,
  ArrowDownOutline,
  FolderOpenOutline
} from '@vicons/ionicons5'
import LoadingSpinner from '../Utils/LoadingSpinner.vue'

interface LogEntry {
  timestamp: string
  level: 'info' | 'warn' | 'error' | 'debug'
  message: string
  source: string
}

const props = defineProps<{
  nginxService: any
}>()

const logs = ref<LogEntry[]>([])
const loading = ref(false)
const error = ref('')
const message = useMessage()
const autoRefresh = ref(false)
const refreshInterval = ref<NodeJS.Timeout>()
const searchQuery = ref('')
const selectedLevel = ref<string>('')
const selectedSource = ref<string>('')
const selectedLogType = ref<string>('access')
const lines = ref(100)
const logType = ref('error')
const numLines = ref(100)
const searchText = ref('')
const logLevel = ref<string | null>(null)
const logLines = ref<string[]>([])
const logContainer = ref<HTMLElement | null>(null)
const canScrollUp = ref(false)
const canScrollDown = ref(false)

const logTypeOptions: SelectOption[] = [
  { label: '访问日志', value: 'access' },
  { label: '错误日志', value: 'error' },
  { label: '服务日志', value: 'service' }
]

const levelOptions: SelectOption[] = [
  { label: '全部', value: '' },
  { label: '信息', value: 'info' },
  { label: '警告', value: 'warn' },
  { label: '错误', value: 'error' },
  { label: '调试', value: 'debug' }
]

const sourceOptions = ref<SelectOption[]>([])

const fetchLogs = async () => {
  try {
    loading.value = true
    error.value = ''
    const result = await invoke('get_logs', {
      nginxService: props.nginxService,
      logType: logType.value,
      numLine: numLines.value,
      search: searchText.value || undefined,
      level: logLevel.value || undefined
    })
    logLines.value = (result as string).split('\n')
    
    // 更新来源选项
    const sources = new Set<string>()
    logs.value.forEach(log => sources.add(log.source))
    sourceOptions.value = [
      { label: '全部', value: '' },
      ...Array.from(sources).map(source => ({ label: source, value: source }))
    ]
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取日志失败'
  } finally {
    loading.value = false
  }
}

const clearLogs = async () => {
  try {
    loading.value = true
    error.value = ''
    await invoke('clear_logs', {
      logType: logType.value
    })
    message.success('日志已清空')
    await fetchLogs()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '清空日志失败'
  } finally {
    loading.value = false
  }
}

const exportLogs = async () => {
  try {
    loading.value = true
    error.value = ''
    await invoke('export_logs')
    message.success('日志导出成功')
  } catch (err) {
    error.value = err instanceof Error ? err.message : '导出日志失败'
  } finally {
    loading.value = false
  }
}

const openLogFolder = async () => {
  try {
    await invoke('open_log_folder')
    message.success('日志文件夹已打开')
  } catch (err) {
    error.value = err instanceof Error ? err.message : '打开日志文件夹失败'
  }
}

const toggleAutoRefresh = () => {
  autoRefresh.value = !autoRefresh.value
  if (autoRefresh.value) {
    refreshInterval.value = setInterval(fetchLogs, 5000)
    message.success('已开启自动刷新')
  } else {
    if (refreshInterval.value) {
      clearInterval(refreshInterval.value)
    }
    message.info('已关闭自动刷新')
  }
}

const handleScroll = () => {
  if (!logContainer.value) return
  
  const { scrollTop, scrollHeight, clientHeight } = logContainer.value
  canScrollUp.value = scrollTop > 0
  canScrollDown.value = scrollTop + clientHeight < scrollHeight
}

const scrollToTop = () => {
  if (logContainer.value) {
    logContainer.value.scrollTop = 0
  }
}

const scrollToBottom = () => {
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight
  }
}

onMounted(() => {
  fetchLogs()
  setTimeout(() => {
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  }, 200)
})

onUnmounted(() => {
  if (refreshInterval.value) {
    clearInterval(refreshInterval.value)
  }
})

watch(autoRefresh, (newValue) => {
  if (newValue) {
    fetchLogs()
  }
})

const handleSearch = () => {
  fetchLogs()
}

const handleClear = () => {
  searchQuery.value = ''
  selectedLevel.value = ''
  selectedSource.value = ''
  fetchLogs()
}

const handleDownload = async () => {
  try {
    const content = logs.value.map(log => 
      `[${log.timestamp}] ${log.level.toUpperCase()} ${log.source}: ${log.message}`
    ).join('\n')
    
    const blob = new Blob([content], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `nginx-logs-${new Date().toISOString()}.txt`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
    
    message.success('日志下载成功')
  } catch (err) {
    message.error('日志下载失败')
  }
}

const getLogLevelClass = (line: string) => {
  if (line.toLowerCase().includes('error')) return 'text-red-500'
  if (line.toLowerCase().includes('warn')) return 'text-yellow-500'
  if (line.toLowerCase().includes('info')) return 'text-blue-500'
  if (line.toLowerCase().includes('debug')) return 'text-gray-500'
  return ''
}

// 监听日志类型变化
watch(logType, () => {
  fetchLogs()
})

// 监听行数变化
watch(numLines, () => {
  fetchLogs()
})

// 监听搜索文本变化
watch(searchText, () => {
  fetchLogs()
})

// 监听日志级别变化
watch(logLevel, () => {
  fetchLogs()
})

// 自动滚动到底部
watch(logLines, () => {
  if (logContainer.value) {
    setTimeout(() => {
      logContainer.value!.scrollTop = logContainer.value!.scrollHeight
    }, 100)
  }
})
</script>

<template>
  <NMessageProvider>
    <NCard title="日志查看器">
      <template #header-extra>
        <NSpace>
          <NButton
            :type="autoRefresh ? 'primary' : 'default'"
            @click="toggleAutoRefresh"
          >
            <template #icon>
              <NIcon>
                <RefreshOutline />
              </NIcon>
            </template>
            自动刷新
          </NButton>
          <NButton type="warning" @click="clearLogs">
            <template #icon>
              <NIcon>
                <TrashOutline />
              </NIcon>
            </template>
            清空
          </NButton>
          <NButton type="info" @click="openLogFolder">
            <template #icon>
              <NIcon>
                <FolderOpenOutline />
              </NIcon>
            </template>
            打开文件夹
          </NButton>
        </NSpace>
      </template>

      <NSpace vertical>
        <NSpace>
          <NSelect
            v-model:value="logType"
            :options="logTypeOptions"
            placeholder="选择日志类型"
            style="width: 120px"
          />
          <NInputNumber
            v-model:value="numLines"
            :min="10"
            :max="1000"
            :step="10"
            placeholder="行数"
            style="width: 100px"
          />
          <NInput
            v-model:value="searchText"
            placeholder="搜索关键词"
            clearable
            style="width: 200px"
          >
            <template #prefix>
              <NIcon>
                <SearchOutline />
              </NIcon>
            </template>
          </NInput>
          <NSelect
            v-model:value="logLevel"
            :options="levelOptions"
            placeholder="日志级别"
            clearable
            style="width: 120px"
          />
          <NButton
            size="small"
            @click="scrollToTop"
            title="滚动到顶部"
          >
            <template #icon>
              <NIcon>
                <ArrowUpOutline />
              </NIcon>
            </template>
            顶部
          </NButton>
          <NButton
            size="small"
            @click="scrollToBottom"
            title="滚动到底部"
          >
            <template #icon>
              <NIcon>
                <ArrowDownOutline />
              </NIcon>
            </template>
            底部
          </NButton>
        </NSpace>

        <NSpin :show="loading">
          <NAlert
            v-if="error"
            type="error"
            :title="error"
            class="mb-4"
          />

          <div class="flex justify-between items-center mb-2">
            <div class="text-sm text-gray-500">
              共 {{ logLines.length }} 行日志
            </div>
          </div>

          <div class="log-container-wrapper">
            <NScrollbar>
              <div
                ref="logContainer"
                class="log-container font-mono text-sm"
                :class="{ 'loading': loading }"
                @scroll="handleScroll"
              >
                <div
                  v-for="(line, index) in logLines"
                  :key="index"
                  class="log-line"
                  :class="getLogLevelClass(line)"
                >
                  <span class="log-line-number">{{ index + 1 }}</span>
                  <span class="log-line-content">{{ line }}</span>
                </div>
              </div>
            </NScrollbar>
          </div>
        </NSpin>
      </NSpace>
    </NCard>
  </NMessageProvider>
</template>

<style scoped>
.log-container {
  font-family: monospace;
  font-size: 14px;
  line-height: 1.5;
}

.log-entry {
  display: grid;
  grid-template-columns: 180px 80px 120px 1fr;
  gap: 8px;
  padding: 4px;
  border-bottom: 1px solid #eee;
}

.log-entry:hover {
  background-color: #f5f5f5;
}

.log-timestamp {
  color: #666;
}

.log-level {
  font-weight: bold;
}

.level-info {
  color: #1890ff;
}

.level-warn {
  color: #faad14;
}

.level-error {
  color: #f5222d;
}

.level-debug {
  color: #52c41a;
}

.log-source {
  color: #722ed1;
}

.log-message {
  color: #333;
}

.log-viewer {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.log-container-wrapper {
  height: calc(100vh - 250px);
  min-height: 400px;
}

.log-container {
  height: 100%;
  overflow-y: auto;
  background-color: #1a1a1a;
  color: #fff;
  padding: 1rem;
  border-radius: 4px;
  font-family: 'Fira Code', monospace;
}

.log-line {
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
  padding: 2px 0;
  display: flex;
  align-items: flex-start;
}

.log-line-number {
  color: #666;
  padding-right: 1rem;
  user-select: none;
  min-width: 3rem;
  text-align: right;
}

.log-line-content {
  flex: 1;
}

.log-container.loading {
  opacity: 0.7;
  pointer-events: none;
}

/* 自定义滚动条样式 */
.log-container::-webkit-scrollbar {
  width: 8px;
}

.log-container::-webkit-scrollbar-track {
  background: #2a2a2a;
  border-radius: 4px;
}

.log-container::-webkit-scrollbar-thumb {
  background: #4a4a4a;
  border-radius: 4px;
}

.log-container::-webkit-scrollbar-thumb:hover {
  background: #5a5a5a;
}

/* 错误日志高亮 */
.text-red-500 {
  color: #f56c6c;
  font-weight: bold;
}

.text-yellow-500 {
  color: #e6a23c;
}

.text-blue-500 {
  color: #409eff;
}

.text-gray-500 {
  color: #909399;
}
</style> 