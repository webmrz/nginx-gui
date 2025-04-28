<script setup lang="ts">
import { 
  NLayoutContent,
  NSpace
} from 'naive-ui'

interface Props {
  title?: string
}

defineProps<Props>()
</script>

<template>
  <NLayoutContent class="p-6">
    <NSpace vertical :size="16">
      <slot />
    </NSpace>
  </NLayoutContent>
</template> 