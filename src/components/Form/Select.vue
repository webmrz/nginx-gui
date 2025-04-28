<script setup lang="ts">
import { computed } from 'vue'

interface Option {
  label: string
  value: string | number
  disabled?: boolean
}

const props = defineProps<{
  modelValue: string | number
  label?: string
  options: Option[]
  placeholder?: string
  error?: string
  disabled?: boolean
  required?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void
}>()

const selectValue = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const selectClasses = computed(() => {
  return [
    'block w-full rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm',
    props.error
      ? 'border-red-300 text-red-900'
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
      <select
        v-model="selectValue"
        :disabled="disabled"
        :class="selectClasses"
      >
        <option v-if="placeholder" value="" disabled>
          {{ placeholder }}
        </option>
        <option
          v-for="option in options"
          :key="option.value"
          :value="option.value"
          :disabled="option.disabled"
        >
          {{ option.label }}
        </option>
      </select>
    </div>
    <p v-if="error" class="mt-2 text-sm text-red-600">
      {{ error }}
    </p>
  </div>
</template> 