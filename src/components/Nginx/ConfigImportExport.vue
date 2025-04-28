<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'

const loading = ref(false)
const error = ref('')
const success = ref('')

const exportConfig = async () => {
  try {
    loading.value = true
    error.value = ''
    success.value = ''
    
    const filePath = await save({
      title: '导出配置文件',
      filters: [{
        name: 'Nginx Config',
        extensions: ['conf']
      }]
    })
    
    if (filePath) {
      await invoke('export_config', { filePath })
      success.value = '配置文件导出成功'
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : '导出配置文件失败'
  } finally {
    loading.value = false
  }
}

const importConfig = async () => {
  try {
    loading.value = true
    error.value = ''
    success.value = ''
    
    const filePath = await open({
      title: '导入配置文件',
      filters: [{
        name: 'Nginx Config',
        extensions: ['conf']
      }],
      multiple: false
    })
    
    if (filePath) {
      await invoke('import_config', { filePath })
      success.value = '配置文件导入成功'
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : '导入配置文件失败'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="space-y-4">
    <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
      {{ error }}
    </div>

    <div v-if="success" class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded">
      {{ success }}
    </div>

    <div class="flex space-x-4">
      <button
        @click="exportConfig"
        :disabled="loading"
        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        导出配置
      </button>
      <button
        @click="importConfig"
        :disabled="loading"
        class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        导入配置
      </button>
    </div>
  </div>
</template> 