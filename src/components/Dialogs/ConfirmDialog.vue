<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  modelValue: boolean
  title?: string
  message: string
  confirmText?: string
  cancelText?: string
  type?: 'info' | 'warning' | 'danger'
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'confirm'): void
  (e: 'cancel'): void
}>()

const isOpen = ref(props.modelValue)

watch(() => props.modelValue, (value) => {
  isOpen.value = value
})

watch(isOpen, (value) => {
  emit('update:modelValue', value)
})

const close = () => {
  isOpen.value = false
  emit('cancel')
}

const confirm = () => {
  isOpen.value = false
  emit('confirm')
}

const typeClasses = {
  info: 'bg-blue-100 text-blue-800',
  warning: 'bg-yellow-100 text-yellow-800',
  danger: 'bg-red-100 text-red-800'
}

const buttonClasses = {
  info: 'bg-blue-600 hover:bg-blue-700',
  warning: 'bg-yellow-600 hover:bg-yellow-700',
  danger: 'bg-red-600 hover:bg-red-700'
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 bg-gray-600 bg-opacity-50 flex items-center justify-center"
  >
    <div class="bg-white rounded-lg p-6 max-w-md w-full">
      <!-- 标题 -->
      <div class="flex items-center mb-4">
        <div
          v-if="type"
          :class="typeClasses[type]"
          class="p-2 rounded-full mr-3"
        >
          <svg
            class="w-6 h-6"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              v-if="type === 'info'"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
            <path
              v-if="type === 'warning'"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            />
            <path
              v-if="type === 'danger'"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        </div>
        <h3 class="text-lg font-semibold text-gray-900">
          {{ title || '确认操作' }}
        </h3>
      </div>

      <!-- 消息 -->
      <p class="text-gray-600 mb-6">
        {{ message }}
      </p>

      <!-- 按钮 -->
      <div class="flex justify-end space-x-3">
        <Button
          @click="close"
          variant="secondary"
        >
          {{ cancelText || '取消' }}
        </Button>
        <Button
          @click="confirm"
          :class="buttonClasses[type || 'info']"
        >
          {{ confirmText || '确认' }}
        </Button>
      </div>
    </div>
  </div>
</template> 