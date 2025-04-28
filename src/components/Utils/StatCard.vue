<script setup lang="ts">
import { NCard, NStatistic } from 'naive-ui'

interface Props {
  title: string
  value: string | number
  color?: string
  icon?: string
}

defineProps<Props>()
</script>

<template>
  <NCard>
    <NStatistic :label="title">
      <template #prefix>
        <slot name="prefix" />
      </template>
      <span :class="color ? `text-${color}-600` : 'text-gray-900'">
        {{ value }}
      </span>
      <template #suffix>
        <slot name="suffix" />
      </template>
    </NStatistic>
  </NCard>
</template> 