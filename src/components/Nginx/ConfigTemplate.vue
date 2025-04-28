<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Template {
  id: string
  name: string
  description: string
  content: string
  type: 'main' | 'site' | 'custom'
}

const templates = ref<Template[]>([])
const selectedTemplate = ref<Template | null>(null)
const loading = ref(false)
const error = ref('')

const fetchTemplates = async () => {
  try {
    loading.value = true
    error.value = ''
    templates.value = await invoke('get_config_templates')
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取模板列表失败'
  } finally {
    loading.value = false
  }
}

const applyTemplate = async (template: Template) => {
  try {
    loading.value = true
    error.value = ''
    await invoke('apply_config_template', {
      templateId: template.id
    })
    selectedTemplate.value = template
  } catch (err) {
    error.value = err instanceof Error ? err.message : '应用模板失败'
  } finally {
    loading.value = false
  }
}

const createTemplate = async (name: string, description: string, content: string, type: 'main' | 'site' | 'custom') => {
  try {
    loading.value = true
    error.value = ''
    await invoke('create_config_template', {
      name,
      description,
      content,
      type
    })
    await fetchTemplates()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '创建模板失败'
  } finally {
    loading.value = false
  }
}

const deleteTemplate = async (templateId: string) => {
  try {
    loading.value = true
    error.value = ''
    await invoke('delete_config_template', {
      templateId
    })
    await fetchTemplates()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '删除模板失败'
  } finally {
    loading.value = false
  }
}

fetchTemplates()
</script>

<template>
  <div class="space-y-4">
    <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
      {{ error }}
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <div
        v-for="template in templates"
        :key="template.id"
        class="bg-white rounded-lg shadow p-6 hover:shadow-md transition-shadow"
      >
        <h3 class="text-lg font-medium text-gray-900">{{ template.name }}</h3>
        <p class="mt-2 text-sm text-gray-500">{{ template.description }}</p>
        
        <div class="mt-4 flex justify-between items-center">
          <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
            :class="{
              'bg-blue-100 text-blue-800': template.type === 'main',
              'bg-green-100 text-green-800': template.type === 'site',
              'bg-purple-100 text-purple-800': template.type === 'custom'
            }"
          >
            {{ template.type === 'main' ? '主配置' : template.type === 'site' ? '站点配置' : '自定义配置' }}
          </span>
          
          <div class="flex space-x-2">
            <button
              @click="applyTemplate(template)"
              class="text-sm text-blue-600 hover:text-blue-800"
            >
              应用
            </button>
            <button
              @click="deleteTemplate(template.id)"
              class="text-sm text-red-600 hover:text-red-800"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="mt-6">
      <button
        @click="$emit('create-template')"
        class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
      >
        创建新模板
      </button>
    </div>
  </div>
</template> 