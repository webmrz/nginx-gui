<template>
  <div class="version-manager">
    <div class="flex justify-between items-center mb-4">
      <h3 class="text-lg font-medium">Nginx 版本管理</h3>
      <NSpace>
        <NButton
          type="primary"
          @click="showInstallDialog = true"
        >
          <template #icon>
            <NIcon>
              <AddOutline />
            </NIcon>
          </template>
          安装新版本
        </NButton>
      </NSpace>
    </div>

    <NCard>
      <NSpace vertical>
        <LoadingSpinner v-if="loading" />

        <NDataTable
          :columns="columns"
          :data="versions"
          :bordered="true"
          :striped="true"
        />
      </NSpace>
    </NCard>

    <!-- 安装新版本对话框 -->
    <NModal
      v-model:show="showInstallDialog"
      preset="dialog"
      title="安装新版本"
      positive-text="安装"
      negative-text="取消"
      @positive-click="handleInstall"
    >
      <NForm
        ref="formRef"
        :model="formValue"
        :rules="rules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
      >
        <NFormItem label="版本" path="version">
          <NSelect
            v-model:value="formValue.version"
            :options="availableVersions"
            placeholder="请选择要安装的版本"
          />
        </NFormItem>
        <NFormItem label="安装路径" path="installPath">
          <NInput
            v-model:value="formValue.installPath"
            placeholder="请输入安装路径"
          />
        </NFormItem>
      </NForm>
    </NModal>
  </div>
</template>

<script setup lang="ts">
import { ref, h, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  NButton,
  NIcon,
  NSpace,
  NCard,
  NDataTable,
  NModal,
  NForm,
  NFormItem,
  NInput,
  NSelect,
  NPopconfirm,
  useMessage
} from 'naive-ui'
import {
  AddOutline,
  TrashOutline,
  CheckmarkOutline
} from '@vicons/ionicons5'
import LoadingSpinner from '../Utils/LoadingSpinner.vue'

const loading = ref(false)
const message = useMessage()
const versions = ref<any[]>([])
const showInstallDialog = ref(false)
const formRef = ref<HTMLFormElement | null>(null)
const formValue = ref({
  version: '',
  installPath: ''
})

const availableVersions = [
  { label: '1.24.0', value: '1.24.0' },
  { label: '1.25.3', value: '1.25.3' },
  { label: '1.26.0', value: '1.26.0' }
]

const rules = {
  version: {
    required: true,
    message: '请选择版本',
    trigger: 'blur'
  },
  installPath: {
    required: true,
    message: '请输入安装路径',
    trigger: 'blur'
  }
}

const columns = [
  {
    title: '版本号',
    key: 'version'
  },
  {
    title: '安装时间',
    key: 'installed_at',
    render: (row: any) => {
      return new Date(row.installed_at).toLocaleString()
    }
  },
  {
    title: '状态',
    key: 'is_current',
    render: (row: any) => {
      return row.is_current ? '当前版本' : ''
    }
  },
  {
    title: '操作',
    key: 'actions',
    render: (row: any) => {
      return h(NSpace, { justify: 'center' }, [
        !row.is_current && h(
          NButton,
          {
            size: 'small',
            type: 'primary',
            onClick: () => handleSwitch(row)
          },
          {
            icon: () => h(NIcon, null, { default: () => h(CheckmarkOutline) })
          }
        ),
        !row.is_current && h(
          NPopconfirm,
          {
            onPositiveClick: () => handleUninstall(row)
          },
          {
            default: () => '确定要卸载这个版本吗？',
            trigger: () => h(NButton, {
              size: 'small',
              type: 'error'
            }, {
              icon: () => h(NIcon, null, { default: () => h(TrashOutline) })
            })
          }
        )
      ])
    }
  }
]

const fetchVersions = async () => {
  try {
    loading.value = true
    const result = await invoke('get_nginx_versions')
    versions.value = result as any[]
  } catch (err) {
    message.error(err instanceof Error ? err.message : '获取版本列表失败')
  } finally {
    loading.value = false
  }
}

const handleInstall = async () => {
  try {
    loading.value = true
    await invoke('install_nginx_version', {
      version: formValue.value.version,
      installPath: formValue.value.installPath
    })
    showInstallDialog.value = false
    await fetchVersions()
    message.success('安装成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '安装失败')
  } finally {
    loading.value = false
  }
}

const handleSwitch = async (version: any) => {
  try {
    loading.value = true
    await invoke('set_nginx_version', {
      version: version.version
    })
    await fetchVersions()
    message.success('切换成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '切换失败')
  } finally {
    loading.value = false
  }
}

const handleUninstall = async (version: any) => {
  try {
    loading.value = true
    await invoke('uninstall_nginx_version', {
      version: version.version
    })
    await fetchVersions()
    message.success('卸载成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '卸载失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchVersions()
})
</script>

<style scoped>
.version-manager {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style> 