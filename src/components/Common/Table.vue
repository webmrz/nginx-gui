<script setup lang="ts">
import { computed } from 'vue'

interface Column {
  key: string
  label: string
  width?: string
  align?: 'left' | 'center' | 'right'
}

const props = defineProps<{
  columns: Column[]
  data: any[]
  loading?: boolean
  emptyText?: string
}>()

const emit = defineEmits<{
  (e: 'row-click', row: any): void
}>()

const slots = defineSlots<{
  default?: (props: {}) => any
  'header-actions'?: (props: {}) => any
  'row-actions'?: (props: { row: any }) => any
}>()

const columnStyles = computed(() => {
  return props.columns.map(col => ({
    width: col.width,
    textAlign: col.align || 'left'
  }))
})
</script>

<template>
  <div class="overflow-x-auto">
    <table class="min-w-full divide-y divide-gray-200">
      <!-- 表头 -->
      <thead class="bg-gray-50">
        <tr>
          <th
            v-for="(col, index) in columns"
            :key="col.key"
            :style="columnStyles[index]"
            class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
          >
            {{ col.label }}
          </th>
          <th
            v-if="slots['row-actions']"
            class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider"
          >
            <slot name="header-actions" />
          </th>
        </tr>
      </thead>

      <!-- 表格内容 -->
      <tbody class="bg-white divide-y divide-gray-200">
        <!-- 加载状态 -->
        <tr v-if="loading">
          <td :colspan="columns.length + (slots['row-actions'] ? 1 : 0)" class="px-6 py-4">
            <div class="flex justify-center items-center">
              <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
            </div>
          </td>
        </tr>

        <!-- 空状态 -->
        <tr v-else-if="!data.length">
          <td :colspan="columns.length + (slots['row-actions'] ? 1 : 0)" class="px-6 py-4 text-center text-gray-500">
            {{ emptyText || '暂无数据' }}
          </td>
        </tr>

        <!-- 数据行 -->
        <tr
          v-for="row in data"
          :key="row.id"
          class="hover:bg-gray-50 cursor-pointer"
          @click="emit('row-click', row)"
        >
          <td
            v-for="col in columns"
            :key="col.key"
            class="px-6 py-4 whitespace-nowrap text-sm text-gray-500"
          >
            <slot :name="`cell-${col.key}`" :row="row">
              {{ row[col.key] }}
            </slot>
          </td>
          <td
            v-if="slots['row-actions']"
            class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium"
          >
            <slot name="row-actions" :row="row" />
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template> 