<script setup lang="ts">
import { ref } from 'vue'
import { NModal, NForm, NFormItem, NSelect, NInput } from 'naive-ui'
import { ConfigFile } from '../../types/nginx'
import ConfigEditor from './ConfigEditor.vue'

const props = defineProps<{
  showCreateDialog: boolean
  showImportDialog: boolean
  showEditDialog: boolean
  selectedConfig: ConfigFile | null
}>()

const emit = defineEmits<{
  (e: 'update:showCreateDialog', value: boolean): void
  (e: 'update:showImportDialog', value: boolean): void
  (e: 'update:showEditDialog', value: boolean): void
  (e: 'create', type: string, name: string): void
  (e: 'import', file: File): void
  (e: 'save'): void
  (e: 'test'): void
}>()

const newConfigType = ref<'main' | 'site' | 'custom'>('site')
const newConfigName = ref('')
const selectedFile = ref<File | null>(null)

const handleFileChange = (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    selectedFile.value = input.files[0]
  }
}

const handleCreate = () => {
  emit('create', newConfigType.value, newConfigName.value)
  newConfigName.value = ''
}

const handleImport = () => {
  if (selectedFile.value) {
    emit('import', selectedFile.value)
    selectedFile.value = null
  }
}
</script>

<template>
  <!-- 新建配置对话框 -->
  <NModal
    :show="showCreateDialog"
    preset="dialog"
    title="新建配置文件"
    positive-text="创建"
    negative-text="取消"
    @positive-click="handleCreate"
    @update:show="(val) => emit('update:showCreateDialog', val)"
  >
    <NForm>
      <NFormItem label="配置类型">
        <NSelect
          v-model:value="newConfigType"
          :options="[
            { label: '主配置', value: 'main' },
            { label: '站点配置', value: 'site' },
            { label: '自定义配置', value: 'custom' }
          ]"
        />
      </NFormItem>
      <NFormItem label="配置名称">
        <NInput
          v-model:value="newConfigName"
          placeholder="请输入配置名称"
        />
      </NFormItem>
    </NForm>
  </NModal>

  <!-- 导入配置对话框 -->
  <NModal
    :show="showImportDialog"
    preset="dialog"
    title="导入配置文件"
    positive-text="导入"
    negative-text="取消"
    @positive-click="handleImport"
    @update:show="(val) => emit('update:showImportDialog', val)"
  >
    <NForm>
      <NFormItem label="选择文件">
        <input
          type="file"
          accept=".conf"
          @change="handleFileChange"
          class="block w-full text-sm text-gray-500
            file:mr-4 file:py-2 file:px-4
            file:rounded-md file:border-0
            file:text-sm file:font-semibold
            file:bg-blue-50 file:text-blue-700
            hover:file:bg-blue-100"
        />
      </NFormItem>
    </NForm>
  </NModal>

  <!-- 编辑配置对话框 -->
  <NModal
    :show="showEditDialog"
    preset="card"
    title="编辑配置"
    style="width: 800px"
    @update:show="(val) => emit('update:showEditDialog', val)"
  >
    <ConfigEditor
      v-if="selectedConfig"
      :path="selectedConfig.path"
      :title="selectedConfig.name"
      @save="emit('save')"
      @test="emit('test')"
    />
  </NModal>
</template> 