<script setup lang="ts">
import { NSpin } from 'naive-ui'

interface Props {
  show?: boolean
  size?: 'small' | 'medium' | 'large'
  description?: string
}

const props = withDefaults(defineProps<Props>(), {
  show: false,
  size: 'medium',
  description: '加载中...'
})
</script>

<template>
  <NSpin
    :show="show"
    :size="size"
    :description="description"
  />
</template> 