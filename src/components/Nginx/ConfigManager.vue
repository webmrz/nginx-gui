<script setup lang="ts">
import { ref, onMounted, h } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  NCard,
  NButton,
  NIcon,
  NSpace,
  NDataTable,
  NModal,
  NInput,
  NSelect,
  NForm,
  NFormItem,
  NSpin,
  NAlert,
  NPopconfirm,
  NMessageProvider,
  useMessage
} from 'naive-ui'
import {
  AddOutline,
  CreateOutline,
  TrashOutline,
  SaveOutline,
  CodeSlashOutline,
  FileTrayOutline
} from '@vicons/ionicons5'

interface ConfigFile {
  name: string
  path: string
  type: 'main' | 'site' | 'other'
  content: string
}

const configs = ref<ConfigFile[]>([])
const selectedConfig = ref<ConfigFile | null>(null)
const showEditModal = ref(false)
const showCreateModal = ref(false)
const loading = ref(false)
const error = ref('')
const message = useMessage()

const configTypeOptions = [
  { label: '主配置', value: 'main' },
  { label: '站点配置', value: 'site' },
  { label: '其他配置', value: 'other' }
]

const columns = [
  {
    title: '名称',
    key: 'name'
  },
  {
    title: '类型',
    key: 'type',
    render: (row: ConfigFile) => {
      const typeMap = {
        main: '主配置',
        site: '站点配置',
        other: '其他配置'
      }
      return typeMap[row.type]
    }
  },
  {
    title: '路径',
    key: 'path'
  },
  {
    title: '操作',
    key: 'actions',
    render: (row: ConfigFile) => {
      return h(NSpace, null, {
        default: () => [
          h(
            NButton,
            {
              size: 'small',
              type: 'primary',
              onClick: () => editConfig(row)
            },
            {
              icon: () => h(NIcon, null, { default: () => h(CreateOutline) }),
              default: () => '编辑'
            }
          ),
          h(
            NPopconfirm,
            {
              onPositiveClick: () => deleteConfig(row)
            },
            {
              trigger: () =>
                h(
                  NButton,
                  {
                    size: 'small',
                    type: 'error'
                  },
                  {
                    icon: () => h(NIcon, null, { default: () => h(TrashOutline) }),
                    default: () => '删除'
                  }
                ),
              default: () => '确定要删除此配置文件吗？'
            }
          )
        ]
      })
    }
  }
]

const fetchConfigs = async () => {
  try {
    loading.value = true
    error.value = ''
    const result = await invoke('list_configs')
    configs.value = result as ConfigFile[]
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取配置文件列表失败'
  } finally {
    loading.value = false
  }
}

const editConfig = (config: ConfigFile) => {
  selectedConfig.value = { ...config }
  showEditModal.value = true
}

const saveConfig = async () => {
  if (!selectedConfig.value) return

  try {
    loading.value = true
    error.value = ''
    await invoke('save_config', {
      config: selectedConfig.value
    })
    message.success('保存成功')
    showEditModal.value = false
    await fetchConfigs()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '保存配置文件失败'
  } finally {
    loading.value = false
  }
}

const deleteConfig = async (config: ConfigFile) => {
  try {
    loading.value = true
    error.value = ''
    await invoke('delete_config', {
      path: config.path
    })
    message.success('删除成功')
    await fetchConfigs()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '删除配置文件失败'
  } finally {
    loading.value = false
  }
}

const createConfig = async () => {
  if (!selectedConfig.value) return

  try {
    loading.value = true
    error.value = ''
    await invoke('create_config', {
      config: selectedConfig.value
    })
    message.success('创建成功')
    showCreateModal.value = false
    await fetchConfigs()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '创建配置文件失败'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchConfigs()
})
</script>

<template>
  <NMessageProvider>
    <NCard title="配置文件管理">
      <template #header-extra>
        <NButton type="primary" @click="showCreateModal = true">
          <template #icon>
            <NIcon>
              <AddOutline />
            </NIcon>
          </template>
          新建配置
        </NButton>
      </template>

      <NSpin :show="loading">
        <NAlert
          v-if="error"
          type="error"
          :title="error"
          class="mb-4"
        />

        <NDataTable
          :columns="columns"
          :data="configs"
          :bordered="true"
          :single-line="false"
        />
      </NSpin>

      <!-- 编辑配置模态框 -->
      <NModal
        v-model:show="showEditModal"
        preset="card"
        title="编辑配置"
        style="width: 800px"
      >
        <NForm
          v-if="selectedConfig"
          :model="selectedConfig"
          label-placement="left"
          label-width="80"
        >
          <NFormItem label="名称">
            <NInput v-model:value="selectedConfig.name" />
          </NFormItem>
          <NFormItem label="类型">
            <NSelect
              v-model:value="selectedConfig.type"
              :options="configTypeOptions"
            />
          </NFormItem>
          <NFormItem label="内容">
            <NInput
              v-model:value="selectedConfig.content"
              type="textarea"
              :rows="15"
            />
          </NFormItem>
        </NForm>
        <template #footer>
          <NSpace justify="end">
            <NButton @click="showEditModal = false">取消</NButton>
            <NButton type="primary" @click="saveConfig">
              <template #icon>
                <NIcon>
                  <SaveOutline />
                </NIcon>
              </template>
              保存
            </NButton>
          </NSpace>
        </template>
      </NModal>

      <!-- 新建配置模态框 -->
      <NModal
        v-model:show="showCreateModal"
        preset="card"
        title="新建配置"
        style="width: 800px"
      >
        <NForm
          v-if="selectedConfig"
          :model="selectedConfig"
          label-placement="left"
          label-width="80"
        >
          <NFormItem label="名称">
            <NInput v-model:value="selectedConfig.name" />
          </NFormItem>
          <NFormItem label="类型">
            <NSelect
              v-model:value="selectedConfig.type"
              :options="configTypeOptions"
            />
          </NFormItem>
          <NFormItem label="内容">
            <NInput
              v-model:value="selectedConfig.content"
              type="textarea"
              :rows="15"
            />
          </NFormItem>
        </NForm>
        <template #footer>
          <NSpace justify="end">
            <NButton @click="showCreateModal = false">取消</NButton>
            <NButton type="primary" @click="createConfig">
              <template #icon>
                <NIcon>
                  <AddOutline />
                </NIcon>
              </template>
              创建
            </NButton>
          </NSpace>
        </template>
      </NModal>
    </NCard>
  </NMessageProvider>
</template> 