import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ServiceInfo {
  status: string
  version: string | null
  uptime: string | null
  cpu_usage: string | null
  memory_usage: string | null
  active_connections: number | null
  total_connections: number | null
  requests_per_second: number | null
}

export interface LogInfo {
  type: string
  content: string
  exists: boolean
}

export const useNginxStore = defineStore('nginx', () => {
  // 状态
  const serviceInfo = ref<ServiceInfo>({
    status: 'stopped',
    version: null,
    uptime: null,
    cpu_usage: null,
    memory_usage: null,
    active_connections: null,
    total_connections: null,
    requests_per_second: null
  })

  const logs = ref<Record<string, LogInfo>>({
    access: { type: 'access', content: '', exists: false },
    error: { type: 'error', content: '', exists: false },
    service: { type: 'service', content: '', exists: false }
  })

  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // 计算属性
  const isRunning = computed(() => serviceInfo.value.status === 'running')
  const isStopped = computed(() => serviceInfo.value.status === 'stopped')
  const isChecking = computed(() => serviceInfo.value.status === 'checking')

  // 方法
  const fetchServiceInfo = async () => {
    try {
      isLoading.value = true
      error.value = null
      const data = await invoke<ServiceInfo>('get_service_info')
      serviceInfo.value = data
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取服务信息失败'
      console.error('获取服务信息失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  const startService = async () => {
    try {
      isLoading.value = true
      error.value = null
      await invoke('start_nginx')
      await fetchServiceInfo()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '启动服务失败'
      console.error('启动服务失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  const stopService = async () => {
    try {
      isLoading.value = true
      error.value = null
      await invoke('stop_nginx')
      await fetchServiceInfo()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '停止服务失败'
      console.error('停止服务失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  const restartService = async () => {
    try {
      isLoading.value = true
      error.value = null
      await invoke('restart_nginx')
      await fetchServiceInfo()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '重启服务失败'
      console.error('重启服务失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  const fetchLogs = async (type: string, lines: number = 100, search?: string, level?: string) => {
    try {
      isLoading.value = true
      error.value = null
      const content = await invoke<string>('get_nginx_logs', { 
        logType: type, 
        lines, 
        search, 
        level 
      })
      logs.value[type].content = content
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取日志失败'
      console.error('获取日志失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  const clearLogs = async (type: string) => {
    try {
      isLoading.value = true
      error.value = null
      await invoke('clear_logs', { logType: type })
      logs.value[type].content = ''
    } catch (e) {
      error.value = e instanceof Error ? e.message : '清空日志失败'
      console.error('清空日志失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  const openLogFolder = async () => {
    try {
      isLoading.value = true
      error.value = null
      await invoke('open_log_folder')
    } catch (e) {
      error.value = e instanceof Error ? e.message : '打开日志文件夹失败'
      console.error('打开日志文件夹失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  const checkLogExists = async (type: string) => {
    try {
      isLoading.value = true
      error.value = null
      const exists = await invoke<boolean>('check_log_exists', { logType: type })
      logs.value[type].exists = exists
    } catch (e) {
      error.value = e instanceof Error ? e.message : '检查日志文件失败'
      console.error('检查日志文件失败:', e)
    } finally {
      isLoading.value = false
    }
  }

  return {
    // 状态
    serviceInfo,
    logs,
    isLoading,
    error,

    // 计算属性
    isRunning,
    isStopped,
    isChecking,

    // 方法
    fetchServiceInfo,
    startService,
    stopService,
    restartService,
    fetchLogs,
    clearLogs,
    openLogFolder,
    checkLogExists
  }
}) 