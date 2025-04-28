<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  modelValue: string | number
  label?: string
  type?: 'text' | 'password' | 'email' | 'number' | 'url'
  placeholder?: string
  error?: string
  disabled?: boolean
  required?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void
}>()

const inputValue = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const inputClasses = computed(() => {
  return [
    'block w-full rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm',
    props.error
      ? 'border-red-300 text-red-900 placeholder-red-300'
      : 'border-gray-300',
    props.disabled ? 'bg-gray-100 cursor-not-allowed' : 'bg-white'
  ].join(' ')
})
</script>

<template>
  <div>
    <label v-if="label" class="block text-sm font-medium text-gray-700">
      {{ label }}
      <span v-if="required" class="text-red-500">*</span>
    </label>
    <div class="mt-1">
      <input
        v-model="inputValue"
        :type="type || 'text'"
        :placeholder="placeholder"
        :disabled="disabled"
        :class="inputClasses"
      />
    </div>
    <p v-if="error" class="mt-2 text-sm text-red-600">
      {{ error }}
    </p>
  </div>
</template> 