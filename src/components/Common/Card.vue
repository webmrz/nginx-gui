<script setup lang="ts">
defineProps<{
  title?: string
  padding?: boolean
}>()

const slots = defineSlots<{
  default?: (props: {}) => any
  header?: (props: {}) => any
  footer?: (props: {}) => any
}>()
</script>

<template>
  <div class="bg-white rounded-lg shadow">
    <!-- 头部 -->
    <div v-if="title || slots.header" class="border-b px-4 py-3">
      <slot name="header">
        <h3 class="text-lg font-semibold text-gray-900">{{ title }}</h3>
      </slot>
    </div>

    <!-- 内容 -->
    <div :class="{ 'p-4': padding !== false }">
      <slot />
    </div>

    <!-- 底部 -->
    <div v-if="slots.footer" class="border-t px-4 py-3">
      <slot name="footer" />
    </div>
  </div>
</template> 