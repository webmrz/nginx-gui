import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 服务信息接口
export interface ServiceInfo {
  status: 'running' | 'stopped' | 'error'
  version?: string
  uptime?: string    
  cpu_usage: string | null
  memory_usage: string | null
  active_connections: number | null
  total_connections: number | null
  requests_per_second: number | null
}

// Nginx 状态接口
export interface NginxState {
  isRunning: boolean
  statusText: string
  version: string
  uptime: string
  cpuUsage: string
  memoryUsage: string
  activeConnections: number
  totalConnections: number
  requestsPerSecond: number
  initialized: boolean
  error: string
  loading: boolean
}

// 初始状态
const initialState: NginxState = {
  isRunning: false,
  statusText: '未知',
  version: '',
  uptime: '',
  cpuUsage: '0%',
  memoryUsage: '0%',
  activeConnections: 0,
  totalConnections: 0,
  requestsPerSecond: 0,
  initialized: false,
  error: '',
  loading: false
}

export const useNginxStore = defineStore('nginx', () => {
  // 状态
  const state = ref<NginxState>(initialState)
  
  // Getters
  const isRunning = computed(() => state.value.isRunning)
  const statusText = computed(() => state.value.statusText)
  const version = computed(() => state.value.version)
  const uptime = computed(() => state.value.uptime)
  const cpuUsage = computed(() => state.value.cpuUsage)
  const memoryUsage = computed(() => state.value.memoryUsage)
  const activeConnections = computed(() => state.value.activeConnections)
  const totalConnections = computed(() => state.value.totalConnections)
  const requestsPerSecond = computed(() => state.value.requestsPerSecond)
  const initialized = computed(() => state.value.initialized)
  const error = computed(() => state.value.error)
  const loading = computed(() => state.value.loading)
  
  // Actions
  const fetchStatus = async () => {
    try {
      state.value.loading = true
      const info = await invoke<ServiceInfo>('get_service_info')
      console.log(info,'=====')
      // 如果服务未运行，设置默认值
      if (info.status !== 'running') {
        state.value = {
          ...state.value,
          isRunning: false,
          statusText: '未运行',
          version: info.version || '未知',
          uptime: '0s',
          cpuUsage: '0%',
          memoryUsage: '0%',
          activeConnections: 0,
          totalConnections: 0,
          requestsPerSecond: 0,
          error: ''
        }
        return { success: true }
      }

      // 服务运行时更新所有信息
      state.value = {
        ...state.value,
        isRunning: true,
        statusText: '运行中',
        version: info.version || '未知',
        uptime: info.uptime || '0s',
        cpuUsage: info.cpu_usage || '0%',
        memoryUsage: info.memory_usage || '0%',
        activeConnections: info.active_connections || 0,
        totalConnections: info.total_connections || 0,
        requestsPerSecond: info.requests_per_second || 0,
        error: ''
      }
      console.log(state.value)
      return { success: true }
    } catch (error) {
      console.log(error)
      // 如果获取信息失败，假设服务未运行
      state.value = {
        ...state.value,
        isRunning: false,
        statusText: '未运行',
        version: '未知',
        uptime: '0s',
        cpuUsage: '0%',
        memoryUsage: '0%',
        activeConnections: 0,
        totalConnections: 0,
        requestsPerSecond: 0,
        error: error instanceof Error ? error.message : '获取状态失败'
      }
      return { success: false, message: state.value.error }
    } finally {
      state.value.loading = false
    }
  }

  const startService = async () => {
    try {
      state.value.loading = true
      await invoke('start_nginx')
      const result = await fetchStatus()
      return result
    } catch (error) {
      state.value.error = error instanceof Error ? error.message : '启动服务失败'
      return { success: false, message: state.value.error }
    } finally {
      state.value.loading = false
    }
  }

  const stopService = async () => {
    try {
      state.value.loading = true
      await invoke('stop_nginx')
      const result = await fetchStatus()
      return result
    } catch (error) {
      state.value.error = error instanceof Error ? error.message : '停止服务失败'
      return { success: false, message: state.value.error }
    } finally {
      state.value.loading = false
    }
  }

  const restartService = async () => {
    try {
      state.value.loading = true
      await invoke('restart_nginx')
      const result = await fetchStatus()
      return result
    } catch (error) {
      state.value.error = error instanceof Error ? error.message : '重启服务失败'
      return { success: false, message: state.value.error }
    } finally {
      state.value.loading = false
    }
  }

  const initialize = async () => {
    try {
      state.value.loading = true
      const result = await fetchStatus()
      state.value.initialized = true
      return result
    } catch (error) {
      state.value = {
        ...state.value,
        isRunning: false,
        statusText: '未运行',
        version: '未知',
        uptime: '0s',
        cpuUsage: '0%',
        memoryUsage: '0%',
        activeConnections: 0,
        totalConnections: 0,
        requestsPerSecond: 0,
        error: error instanceof Error ? error.message : '初始化失败',
        initialized: true
      }
      return { success: false, message: state.value.error }
    } finally {
      state.value.loading = false
    }
  }

  const resetState = () => {
    state.value = { ...initialState }
  }

  return {
    // 状态
    state,
    
    // Getters
    isRunning,
    statusText,
    version,
    uptime,
    cpuUsage,
    memoryUsage,
    activeConnections,
    totalConnections,
    requestsPerSecond,
    initialized,
    error,
    loading,
    
    // Actions
    fetchStatus,
    startService,
    stopService,
    restartService,
    initialize,
    resetState
  }
}) 