<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { NCard, NButton, NIcon, NSpace, NTabs, NTabPane } from 'naive-ui'
import {
  AddOutline,
  CloudUploadOutline,
  RefreshOutline
} from '@vicons/ionicons5'
import LoadingSpinner from '../components/Utils/LoadingSpinner.vue'
import ErrorAlert from '../components/Utils/ErrorAlert.vue'
import { ConfigFile } from '../types/nginx'
import ConfigTable from '../components/Nginx/ConfigTable.vue'
import ConfigDialogs from '../components/Nginx/ConfigDialogs.vue'
import ProjectConfig from '../components/Nginx/ProjectConfig.vue'

const loading = ref(false)
const error = ref('')
const configs = ref<ConfigFile[]>([])
const showCreateDialog = ref(false)
const showImportDialog = ref(false)
const showEditDialog = ref(false)
const selectedConfig = ref<ConfigFile | null>(null)

const fetchConfigs = async () => {
  try {
    loading.value = true
    error.value = ''
    const result = await invoke('get_config_files')
    configs.value = result as ConfigFile[]
  } catch (err) {
    loading.value = false
    error.value = err instanceof Error ? err.message : '获取配置文件列表失败'
  } finally {
    loading.value = false
  }
}

const createConfig = async (type: string, name: string) => {
  try {
    loading.value = true
    error.value = ''
    await invoke('create_config_file', { type, name })
    showCreateDialog.value = false
    await fetchConfigs()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '创建配置文件失败'
  } finally {
    loading.value = false
  }
}

const editConfig = (config: ConfigFile) => {
  selectedConfig.value = config
  showEditDialog.value = true
}

const exportConfig = async (config: ConfigFile) => {
  try {
    loading.value = true
    error.value = ''
    await invoke('export_config_file', { path: config.path })
  } catch (err) {
    error.value = err instanceof Error ? err.message : '导出配置文件失败'
  } finally {
    loading.value = false
  }
}

const deleteConfig = async (config: ConfigFile) => {
  try {
    loading.value = true
    error.value = ''
    await invoke('delete_config_file', { path: config.path })
    await fetchConfigs()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '删除配置文件失败'
  } finally {
    loading.value = false
  }
}

const importConfig = async (file: File) => {
  try {
    loading.value = true
    error.value = ''
    const content = await file.text()
    await invoke('import_config_file', {
      file_name: file.name,
      config_type: 'custom',
      content
    })
    showImportDialog.value = false
    await fetchConfigs()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '导入配置文件失败'
  } finally {
    loading.value = false
  }
}

const handleConfigSave = async () => {
  await fetchConfigs()
}

const handleConfigTest = async () => {
  await fetchConfigs()
}

onMounted(() => {
  fetchConfigs()
})
</script>

<template>
  <div class="space-y-6">
    <NTabs type="line" animated>
      <NTabPane name="configs" tab="配置文件">
        <div class="space-y-6">
          <div class="flex justify-between items-center">
            <h1 class="text-2xl font-bold text-gray-900">配置文件管理</h1>
            <NSpace>
              <NButton type="primary" @click="showCreateDialog = true">
                <template #icon>
                  <NIcon>
                    <AddOutline />
                  </NIcon>
                </template>
                新建配置
              </NButton>
              <NButton type="info" @click="showImportDialog = true">
                <template #icon>
                  <NIcon>
                    <CloudUploadOutline />
                  </NIcon>
                </template>
                导入配置
              </NButton>
              <NButton type="default" @click="fetchConfigs">
                <template #icon>
                  <NIcon>
                    <RefreshOutline />
                  </NIcon>
                </template>
                刷新
              </NButton>
            </NSpace>
          </div>

          <ErrorAlert :error="error" />

          <NCard>
            <NSpace vertical>
              <LoadingSpinner v-if="loading" />

              <ConfigTable
                :configs="configs"
                :loading="loading"
                @edit="editConfig"
                @export="exportConfig"
                @delete="deleteConfig"
              />
            </NSpace>
          </NCard>

          <ConfigDialogs
            v-model:showCreateDialog="showCreateDialog"
            v-model:showImportDialog="showImportDialog"
            v-model:showEditDialog="showEditDialog"
            :selectedConfig="selectedConfig"
            @create="createConfig"
            @import="importConfig"
            @save="handleConfigSave"
            @test="handleConfigTest"
          />
        </div>
      </NTabPane>

      <NTabPane name="projects" tab="项目配置">
        <ProjectConfig />
      </NTabPane>
    </NTabs>
  </div>
</template> 