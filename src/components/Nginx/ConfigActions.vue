<script setup lang="ts">
import { h } from 'vue'
import { NButton, NIcon, NSpace, NPopconfirm, NTooltip } from 'naive-ui'
import {
  DownloadOutline,
  TrashOutline,
  CodeOutline
} from '@vicons/ionicons5'
import { ConfigFile } from '../../types/nginx'

const props = defineProps<{
  config: ConfigFile
}>()

const emit = defineEmits<{
  (e: 'edit', config: ConfigFile): void
  (e: 'export', config: ConfigFile): void
  (e: 'delete', config: ConfigFile): void
}>()

const handleEdit = () => emit('edit', props.config)
const handleExport = () => emit('export', props.config)
const handleDelete = () => emit('delete', props.config)
</script>

<template>
  <NSpace justify="center">
    <NTooltip trigger="hover">
      <template #trigger>
        <NButton size="small" type="primary" @click="handleEdit">
          <template #icon>
            <NIcon>
              <CodeOutline />
            </NIcon>
          </template>
        </NButton>
      </template>
      编辑
    </NTooltip>

    <NTooltip trigger="hover">
      <template #trigger>
        <NButton size="small" type="success" @click="handleExport">
          <template #icon>
            <NIcon>
              <DownloadOutline />
            </NIcon>
          </template>
        </NButton>
      </template>
      导出
    </NTooltip>

    <NPopconfirm @positive-click="handleDelete">
      <template #trigger>
        <NButton size="small" type="error">
          <template #icon>
            <NIcon>
              <TrashOutline />
            </NIcon>
          </template>
        </NButton>
      </template>
      确定要删除这个配置文件吗？
    </NPopconfirm>
  </NSpace>
</template> 