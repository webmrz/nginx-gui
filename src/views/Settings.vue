<template>
  <div class="space-y-6">
    <div class="flex justify-between items-center">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">设置</h1>
    </div>

    <NTabs type="line" animated>
      <NTabPane name="general" tab="常规设置">
        <NCard>
          <NForm
            ref="formRef"
            :model="formValue"
            :rules="rules"
            label-placement="left"
            label-width="auto"
            require-mark-placement="right-hanging"
          >
            <NFormItem label="内置 Nginx 路径" path="builtinNginxPath">
              <NInput
                v-model:value="builtinNginxPath"
                disabled
                placeholder="内置 Nginx 路径"
              />
            </NFormItem>
            <NFormItem label="Nginx 安装路径" path="nginxPath">
              <NInputGroup>
                <NInput
                  v-model:value="formValue.nginxPath"
                  placeholder="请输入 Nginx 安装路径"
                />
                <NButton @click="handleSelectNginxPath">
                  选择
                </NButton>
              </NInputGroup>
              <div class="mt-1 text-sm text-gray-500">
                如果为空，将使用内置 Nginx
              </div>
            </NFormItem>
            <NFormItem label="配置文件路径" path="configPath">
              <NInputGroup>
                <NInput
                  v-model:value="formValue.configPath"
                  placeholder="请输入配置文件路径"
                />
                <NButton @click="handleSelectConfigPath">
                  选择
                </NButton>
              </NInputGroup>
              <div class="mt-1 text-sm text-gray-500">
                如果为空，将使用内置 Nginx 的配置路径
              </div>
            </NFormItem>
            <NFormItem label="自动启动" path="autoStart">
              <NSwitch v-model:value="formValue.autoStart" />
            </NFormItem>
            <NFormItem label="日志级别" path="logLevel">
              <NSelect
                v-model:value="formValue.logLevel"
                :options="logLevelOptions"
              />
            </NFormItem>
            <NFormItem label="主题" path="theme">
              <NSelect
                v-model:value="formValue.theme"
                :options="themeOptions"
              />
            </NFormItem>
            <NFormItem>
              <NButton
                type="primary"
                :loading="loading"
                @click="handleSave"
              >
                保存设置
              </NButton>
            </NFormItem>
          </NForm>
        </NCard>
      </NTabPane>

      <NTabPane name="version" tab="版本管理">
        <VersionManager />
      </NTabPane>
    </NTabs>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import {
  NCard,
  NForm,
  NFormItem,
  NInput,
  NButton,
  NTabs,
  NTabPane,
  NInputGroup,
  NSwitch,
  NSelect,
  useMessage
} from 'naive-ui'
import VersionManager from '../components/Nginx/VersionManager.vue'
import { appDataDir } from '@tauri-apps/api/path'

const loading = ref(false)
const message = useMessage()
const formRef = ref<HTMLFormElement | null>(null)
const builtinNginxPath = ref('')
const formValue = ref({
  nginxPath: '',
  configPath: '',
  autoStart: false,
  logLevel: 'info',
  theme: 'light'
})

const logLevelOptions = [
  { label: '调试', value: 'debug' },
  { label: '信息', value: 'info' },
  { label: '警告', value: 'warn' },
  { label: '错误', value: 'error' }
]

const themeOptions = [
  { label: '浅色', value: 'light' },
  { label: '深色', value: 'dark' },
  { label: '跟随系统', value: 'system' }
]

const rules = {
  nginxPath: {
    required: false,
    message: '请输入 Nginx 安装路径',
    trigger: 'blur'
  },
  configPath: {
    required: false,
    message: '请输入配置文件路径',
    trigger: 'blur'
  },
  logLevel: {
    required: true,
    message: '请选择日志级别',
    trigger: 'change'
  },
  theme: {
    required: true,
    message: '请选择主题',
    trigger: 'change'
  }
}

const loadSettings = async () => {
  try {
    loading.value = true
    const result = await invoke('get_settings')
    formValue.value = result as any
    // 获取内置Nginx路径
    builtinNginxPath.value = await invoke('get_builtin_nginx_path')
  } catch (err) {
    console.error(err)
    message.error(err instanceof Error ? err.message : '获取设置失败')
  } finally {
    loading.value = false
  }
}

const handleSave = async () => {
  try {
    loading.value = true
    await invoke('save_settings', {
      settings: formValue.value
    })
    message.success('设置已保存')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '保存设置失败')
  } finally {
    loading.value = false
  }
}

const handleSelectNginxPath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择 Nginx 安装路径'
    })
    if (selected) {
      formValue.value.nginxPath = selected as string
    }
  } catch (err) {
    message.error(err instanceof Error ? err.message : '选择路径失败')
  }
}

const handleSelectConfigPath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择配置文件路径'
    })
    if (selected) {
      formValue.value.configPath = selected as string
    }
  } catch (err) {
    message.error(err instanceof Error ? err.message : '选择路径失败')
  }
}

onMounted(() => {
  loadSettings()
})
</script> 