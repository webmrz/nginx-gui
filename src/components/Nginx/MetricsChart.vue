<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import * as echarts from 'echarts/core'
import { LineChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers'
import {
  NCard,
  NButton,
  NIcon,
  NSpace,
  NSelect,
  NSpin,
  NAlert,
  type SelectOption
} from 'naive-ui'
import {
  RefreshOutline,
  DownloadOutline
} from '@vicons/ionicons5'

echarts.use([
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
  LineChart,
  CanvasRenderer
])

const chartRef = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

interface MetricsData {
  timestamp: string
  cpuUsage: number
  memoryUsage: number
  requestsPerSecond: number
}

const metricsData = ref<MetricsData[]>([])
const maxDataPoints = 60 // 最多显示60个数据点
const error = ref('')
const loading = ref(false)
const autoRefresh = ref(true)
const selectedMetric = ref<string>('')
const selectedChartType = ref<'line' | 'bar' | 'pie'>('line')

const metricOptions: SelectOption[] = [
  { label: 'CPU 使用率', value: 'cpu' },
  { label: '内存使用率', value: 'memory' },
  { label: '请求数', value: 'requests' }
]

const chartTypeOptions: SelectOption[] = [
  { label: '折线图', value: 'line' },
  { label: '柱状图', value: 'bar' },
  { label: '饼图', value: 'pie' }
]

const initChart = () => {
  if (!chartRef.value) return

  chart = echarts.init(chartRef.value)
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross',
        label: {
          backgroundColor: '#6a7985'
        }
      }
    },
    legend: {
      data: ['CPU使用率', '内存使用率', '每秒请求数']
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: []
    },
    yAxis: [
      {
        type: 'value',
        name: '使用率(%)',
        min: 0,
        max: 100,
        position: 'left'
      },
      {
        type: 'value',
        name: '请求数',
        position: 'right'
      }
    ],
    series: [
      {
        name: 'CPU使用率',
        type: 'line',
        smooth: true,
        data: [],
        yAxisIndex: 0,
        itemStyle: {
          color: '#18A058'
        }
      },
      {
        name: '内存使用率',
        type: 'line',
        smooth: true,
        data: [],
        yAxisIndex: 0,
        itemStyle: {
          color: '#2080F0'
        }
      },
      {
        name: '每秒请求数',
        type: 'line',
        smooth: true,
        data: [],
        yAxisIndex: 1,
        itemStyle: {
          color: '#F0A020'
        }
      }
    ]
  }

  chart.setOption(option)
}

const updateChart = () => {
  if (!chart) return

  const timestamps = metricsData.value.map(item => item.timestamp)
  const cpuData = metricsData.value.map(item => item.cpuUsage)
  const memoryData = metricsData.value.map(item => item.memoryUsage)
  const requestsData = metricsData.value.map(item => item.requestsPerSecond)

  chart.setOption({
    xAxis: {
      data: timestamps
    },
    series: [
      {
        data: cpuData
      },
      {
        data: memoryData
      },
      {
        data: requestsData
      }
    ]
  })
}

const fetchData = async () => {
  try {
    loading.value = true
    error.value = ''
    const result = await invoke('get_dashboard_stats')
    const currentTime = new Date().toLocaleTimeString()
    
    metricsData.value.push({
      timestamp: currentTime,
      cpuUsage: parseFloat((result as any).cpuUsage),
      memoryUsage: parseFloat((result as any).memoryUsage),
      requestsPerSecond: (result as any).requestsPerSecond
    })

    // 保持数据点数量在限制范围内
    if (metricsData.value.length > maxDataPoints) {
      metricsData.value.shift()
    }

    updateChart()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取性能数据失败'
    console.error('获取性能数据失败:', err)
  } finally {
    loading.value = false
  }
}

const toggleAutoRefresh = () => {
  autoRefresh.value = !autoRefresh.value
  if (autoRefresh.value) {
    startAutoRefresh()
  } else {
    stopAutoRefresh()
  }
}

const startAutoRefresh = () => {
  refreshInterval = setInterval(fetchData, 5000)
}

const stopAutoRefresh = () => {
  if (refreshInterval) {
    clearInterval(refreshInterval)
  }
}

const exportChart = () => {
  if (!chart) return
  const dataUrl = chart.getDataURL()
  const link = document.createElement('a')
  link.download = `metrics-${new Date().toISOString()}.png`
  link.href = dataUrl
  link.click()
}

let refreshInterval: NodeJS.Timeout

onMounted(() => {
  initChart()
  fetchData()
  if (autoRefresh.value) {
    startAutoRefresh()
  }
})

onUnmounted(() => {
  stopAutoRefresh()
  if (chart) {
    chart.dispose()
  }
})

// 监听窗口大小变化
window.addEventListener('resize', () => {
  chart?.resize()
})
</script>

<template>
  <NCard title="性能指标">
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
        <NButton type="success" @click="exportChart">
          <template #icon>
            <NIcon>
              <DownloadOutline />
            </NIcon>
          </template>
          导出
        </NButton>
      </NSpace>
    </template>

    <NSpace vertical>
      <NSpace>
        <NSelect
          v-model:value="selectedMetric"
          :options="metricOptions"
          style="width: 150px"
          placeholder="选择指标"
          @update:value="fetchData"
        />
        <NSelect
          v-model:value="selectedChartType"
          :options="chartTypeOptions"
          style="width: 120px"
          placeholder="图表类型"
          @update:value="fetchData"
        />
      </NSpace>

      <NSpin :show="loading">
        <NAlert
          v-if="error"
          type="error"
          :title="error"
          class="mb-4"
        />

        <div class="h-80">
          <div ref="chartRef" class="w-full h-full"></div>
        </div>
      </NSpin>
    </NSpace>
  </NCard>
</template>

<style scoped>
.chart-container {
  width: 100%;
  height: 100%;
}
</style> 