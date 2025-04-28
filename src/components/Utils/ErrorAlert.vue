<script setup lang="ts">
import { NAlert } from 'naive-ui'

interface Props {
  error?: string
  type?: 'error' | 'warning' | 'info' | 'success'
  title?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'error',
  title: '错误'
})
</script>

<template>
  <NAlert
    v-if="error"
    :type="type"
    :title="title"
    :description="error"
    class="mb-4"
  />
</template> 