<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Settings {
  theme: 'light' | 'dark' | 'system'
  language: 'zh' | 'en'
  autoStart: boolean
  logLevel: 'info' | 'warn' | 'error' | 'debug'
  maxLogSize: number
}

const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'save', settings: Settings): void
}>()

const isOpen = ref(props.modelValue)
const settings = ref<Settings>({
  theme: 'system',
  language: 'zh',
  autoStart: false,
  logLevel: 'info',
  maxLogSize: 10
})
const loading = ref(false)
const error = ref('')

watch(() => props.modelValue, (value) => {
  isOpen.value = value
})

watch(isOpen, (value) => {
  emit('update:modelValue', value)
})

const loadSettings = async () => {
  try {
    loading.value = true
    error.value = ''
    const result = await invoke('get_settings')
    settings.value = result as Settings
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取设置失败'
  } finally {
    loading.value = false
  }
}

const saveSettings = async () => {
  try {
    loading.value = true
    error.value = ''
    await invoke('save_settings', { settings: settings.value })
    emit('save', settings.value)
    isOpen.value = false
  } catch (err) {
    error.value = err instanceof Error ? err.message : '保存设置失败'
  } finally {
    loading.value = false
  }
}

const close = () => {
  isOpen.value = false
}

watch(isOpen, (value) => {
  if (value) {
    loadSettings()
  }
})
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 bg-gray-600 bg-opacity-50 flex items-center justify-center"
  >
    <div class="bg-white rounded-lg p-6 max-w-md w-full">
      <h2 class="text-xl font-bold mb-4">设置</h2>

      <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
        {{ error }}
      </div>

      <div v-if="loading" class="flex justify-center items-center h-32">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
      </div>

      <div v-else class="space-y-4">
        <!-- 主题设置 -->
        <div>
          <label class="block text-sm font-medium text-gray-700">主题</label>
          <Select
            v-model="settings.theme"
            :options="[
              { label: '浅色', value: 'light' },
              { label: '深色', value: 'dark' },
              { label: '跟随系统', value: 'system' }
            ]"
          />
        </div>

        <!-- 语言设置 -->
        <div>
          <label class="block text-sm font-medium text-gray-700">语言</label>
          <Select
            v-model="settings.language"
            :options="[
              { label: '中文', value: 'zh' },
              { label: 'English', value: 'en' }
            ]"
          />
        </div>

        <!-- 自动启动 -->
        <div class="flex items-center">
          <input
            v-model="settings.autoStart"
            type="checkbox"
            class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
          />
          <label class="ml-2 block text-sm text-gray-900">开机自动启动</label>
        </div>

        <!-- 日志级别 -->
        <div>
          <label class="block text-sm font-medium text-gray-700">日志级别</label>
          <Select
            v-model="settings.logLevel"
            :options="[
              { label: '调试', value: 'debug' },
              { label: '信息', value: 'info' },
              { label: '警告', value: 'warn' },
              { label: '错误', value: 'error' }
            ]"
          />
        </div>

        <!-- 最大日志大小 -->
        <div>
          <label class="block text-sm font-medium text-gray-700">
            最大日志大小 (MB)
          </label>
          <Input
            v-model.number="settings.maxLogSize"
            type="number"
            min="1"
            max="100"
          />
        </div>

        <!-- 按钮 -->
        <div class="flex justify-end space-x-3">
          <Button
            @click="close"
            variant="secondary"
          >
            取消
          </Button>
          <Button
            @click="saveSettings"
            :loading="loading"
          >
            保存
          </Button>
        </div>
      </div>
    </div>
  </div>
</template> 