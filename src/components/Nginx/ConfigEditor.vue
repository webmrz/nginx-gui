<template>
  <div class="config-editor">
    <div class="flex justify-between items-center mb-4">
      <h3 class="text-lg font-medium">{{ title }}</h3>
      <NSpace>
        <NButton
          type="primary"
          :loading="loading"
          @click="saveConfig"
        >
          <template #icon>
            <NIcon>
              <SaveOutline />
            </NIcon>
          </template>
          保存
        </NButton>
        <NButton
          type="info"
          @click="testConfig"
        >
          <template #icon>
            <NIcon>
              <CheckmarkOutline />
            </NIcon>
          </template>
          测试配置
        </NButton>
        <NButton
          type="warning"
          @click="formatConfig"
        >
          <template #icon>
            <NIcon>
              <CodeOutline />
            </NIcon>
          </template>
          格式化
        </NButton>
      </NSpace>
    </div>

    <NInput
      v-model:value="content"
      type="textarea"
      :autosize="{ minRows: 20, maxRows: 30 }"
      placeholder="请输入配置文件内容"
      class="font-mono"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  NButton,
  NIcon,
  NSpace,
  NInput,
  useMessage
} from 'naive-ui'
import {
  SaveOutline,
  CheckmarkOutline,
  CodeOutline
} from '@vicons/ionicons5'

const props = defineProps<{
  path: string
  title: string
}>()

const emit = defineEmits<{
  (e: 'save'): void
  (e: 'test'): void
}>()

const content = ref('')
const loading = ref(false)
const message = useMessage()

const loadConfig = async () => {
  try {
    loading.value = true
    const result = await invoke('read_config_file', {
      path: props.path
    })
    content.value = result as string
  } catch (err) {
    message.error(err instanceof Error ? err.message : '读取配置文件失败')
  } finally {
    loading.value = false
  }
}

const saveConfig = async () => {
  try {
    loading.value = true
    await invoke('save_nginx_config', {
      config: {
        name: props.title,
        path: props.path,
        content: content.value
      }
    })
    message.success('保存成功')
    emit('save')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '保存配置文件失败')
  } finally {
    loading.value = false
  }
}

const testConfig = async () => {
  try {
    loading.value = true
    const result = await invoke('test_config_validity', {
      path: props.path
    })
    if (result) {
      message.success('配置测试通过')
    } else {
      message.error('配置测试失败')
    }
    emit('test')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '测试配置文件失败')
  } finally {
    loading.value = false
  }
}

const formatConfig = async () => {
  try {
    loading.value = true
    const result = await invoke('format_config', {
      content: content.value
    })
    content.value = result as string
    message.success('格式化成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '格式化配置文件失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<style scoped>
.config-editor {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.n-input) {
  font-family: monospace;
}
</style> 