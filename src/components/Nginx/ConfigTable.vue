<script setup lang="ts">
import { h } from 'vue'
import { NDataTable, NIcon } from 'naive-ui'
import { DocumentTextOutline } from '@vicons/ionicons5'
import { ConfigFile } from '../../types/nginx'
import ConfigActions from './ConfigActions.vue'

const props = defineProps<{
  configs: ConfigFile[]
  loading: boolean
}>()

const emit = defineEmits<{
  (e: 'edit', config: ConfigFile): void
  (e: 'export', config: ConfigFile): void
  (e: 'delete', config: ConfigFile): void
}>()

const columns = [
  {
    title: '名称',
    key: 'name',
    width: 200,
    render: (row: ConfigFile) => {
      return h('div', { class: 'flex items-center gap-2' }, [
        h(NIcon, { size: 18 }, { default: () => h(DocumentTextOutline) }),
        h('span', row.name)
      ])
    }
  },
  {
    title: '类型',
    key: 'type',
    width: 100,
    render: (row: ConfigFile) => {
      const typeLabels = {
        main: '主配置',
        site: '站点配置',
        custom: '自定义配置'
      }
      return typeLabels[row.type]
    }
  },
  {
    title: '路径',
    key: 'path',
    ellipsis: {
      tooltip: true
    }
  },
  {
    title: '大小',
    key: 'size',
    width: 100
  },
  {
    title: '最后修改',
    key: 'lastModified',
    width: 180
  },
  {
    title: '状态',
    key: 'status',
    width: 100,
    render: (row: ConfigFile) => {
      const statusColors = {
        valid: 'text-green-500',
        invalid: 'text-red-500',
        unknown: 'text-gray-500'
      }
      const statusLabels = {
        valid: '有效',
        invalid: '无效',
        unknown: '未知'
      }
      return h('span', {
        class: statusColors[row.status]
      }, statusLabels[row.status])
    }
  },
  {
    title: '操作',
    key: 'actions',
    width: 200,
    render: (row: ConfigFile) => {
      return h(ConfigActions, {
        config: row,
        onEdit: () => emit('edit', row),
        onExport: () => emit('export', row),
        onDelete: () => emit('delete', row)
      })
    }
  }
]
</script>

<template>
  <NDataTable
    :columns="columns"
    :data="configs"
    :bordered="true"
    :striped="true"
    :single-line="false"
    :loading="loading"
  />
</template> 