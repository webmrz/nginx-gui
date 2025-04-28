<template>
  <div class="project-config">
    <div class="flex justify-between items-center mb-4">
      <h3 class="text-lg font-medium">项目配置</h3>
      <NSpace>
        <NButton
          type="primary"
          @click="showCreateDialog = true"
        >
          <template #icon>
            <NIcon>
              <AddOutline />
            </NIcon>
          </template>
          新建项目
        </NButton>
        <NButton
          type="info"
          @click="showTemplateDialog = true"
        >
          <template #icon>
            <NIcon>
              <DocumentTextOutline />
            </NIcon>
          </template>
          模板管理
        </NButton>
      </NSpace>
    </div>

    <NDataTable
      :columns="columns"
      :data="projects"
      :bordered="true"
      :striped="true"
      :loading="loading"
    />

    <!-- 新建项目对话框 -->
    <NModal
      v-model:show="showCreateDialog"
      preset="dialog"
      title="新建项目"
      positive-text="创建"
      negative-text="取消"
      @positive-click="createProject"
    >
      <NForm
        ref="formRef"
        :model="formValue"
        :rules="rules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
      >
        <NFormItem label="项目名称" path="name">
          <NInput v-model:value="formValue.name" placeholder="请输入项目名称" />
        </NFormItem>
        <NFormItem label="域名" path="domain">
          <NInput v-model:value="formValue.domain" placeholder="请输入域名" />
        </NFormItem>
        <NFormItem label="端口" path="port">
          <NInputNumber v-model:value="formValue.port" :min="1" :max="65535" />
        </NFormItem>
        <NFormItem label="网站根目录" path="root">
          <NInput v-model:value="formValue.root" placeholder="请输入网站根目录" />
        </NFormItem>
        <NFormItem label="默认首页" path="index">
          <NDynamicInput
            v-model:value="formValue.index"
            placeholder="请输入默认首页"
          />
        </NFormItem>
        <NFormItem label="PHP支持" path="php">
          <NSwitch v-model:value="formValue.php" />
        </NFormItem>
        <NFormItem label="SSL支持" path="ssl">
          <NSwitch v-model:value="formValue.ssl" />
        </NFormItem>
        <NFormItem label="备注" path="remark">
          <NInput
            v-model:value="formValue.remark"
            type="textarea"
            placeholder="请输入备注"
          />
        </NFormItem>
      </NForm>
    </NModal>

    <!-- 模板管理对话框 -->
    <NModal
      v-model:show="showTemplateDialog"
      preset="card"
      title="模板管理"
      style="width: 800px"
    >
      <NSpace vertical>
        <NSpace>
          <NButton
            type="primary"
            @click="showCreateTemplateDialog = true"
          >
            <template #icon>
              <NIcon>
                <AddOutline />
              </NIcon>
            </template>
            新建模板
          </NButton>
        </NSpace>

        <NDataTable
          :columns="templateColumns"
          :data="templates"
          :bordered="true"
          :striped="true"
        />
      </NSpace>
    </NModal>

    <!-- 新建模板对话框 -->
    <NModal
      v-model:show="showCreateTemplateDialog"
      preset="dialog"
      title="新建模板"
      positive-text="创建"
      negative-text="取消"
      @positive-click="createTemplate"
    >
      <NForm
        ref="templateFormRef"
        :model="templateFormValue"
        :rules="templateRules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
      >
        <NFormItem label="模板名称" path="name">
          <NInput v-model:value="templateFormValue.name" placeholder="请输入模板名称" />
        </NFormItem>
        <NFormItem label="模板内容" path="content">
          <NInput
            v-model:value="templateFormValue.content"
            type="textarea"
            :autosize="{ minRows: 10, maxRows: 20 }"
            placeholder="请输入模板内容"
            class="font-mono"
          />
        </NFormItem>
        <NFormItem label="备注" path="remark">
          <NInput
            v-model:value="templateFormValue.remark"
            type="textarea"
            placeholder="请输入备注"
          />
        </NFormItem>
      </NForm>
    </NModal>
  </div>
</template>

<script setup lang="ts">
import { ref, h } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  NButton,
  NIcon,
  NSpace,
  NDataTable,
  NModal,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NDynamicInput,
  NSwitch,
  useMessage
} from 'naive-ui'
import {
  AddOutline,
  DocumentTextOutline,
  TrashOutline,
  PencilOutline
} from '@vicons/ionicons5'
import { ProjectConfig, ProjectTemplate } from '../../types/nginx'

const loading = ref(false)
const message = useMessage()
const projects = ref<ProjectConfig[]>([])
const templates = ref<ProjectTemplate[]>([])

// 新建项目相关
const showCreateDialog = ref(false)
const formRef = ref<HTMLFormElement | null>(null)
const formValue = ref({
  name: '',
  domain: '',
  port: 80,
  root: '',
  index: ['index.html', 'index.php'],
  php: false,
  ssl: false,
  remark: ''
})

const rules = {
  name: {
    required: true,
    message: '请输入项目名称',
    trigger: 'blur'
  },
  domain: {
    required: true,
    message: '请输入域名',
    trigger: 'blur'
  },
  port: {
    required: true,
    message: '请输入端口',
    trigger: 'blur'
  },
  root: {
    required: true,
    message: '请输入网站根目录',
    trigger: 'blur'
  }
}

// 模板管理相关
const showTemplateDialog = ref(false)
const showCreateTemplateDialog = ref(false)
const templateFormRef = ref<HTMLFormElement | null>(null)
const templateFormValue = ref({
  name: '',
  content: '',
  remark: ''
})

const templateRules = {
  name: {
    required: true,
    message: '请输入模板名称',
    trigger: 'blur'
  },
  content: {
    required: true,
    message: '请输入模板内容',
    trigger: 'blur'
  }
}

const columns = [
  {
    title: '项目名称',
    key: 'name'
  },
  {
    title: '域名',
    key: 'domain'
  },
  {
    title: '端口',
    key: 'port'
  },
  {
    title: '网站根目录',
    key: 'root'
  },
  {
    title: 'PHP支持',
    key: 'php',
    render: (row: ProjectConfig) => {
      return row.php ? '是' : '否'
    }
  },
  {
    title: 'SSL支持',
    key: 'ssl',
    render: (row: ProjectConfig) => {
      return row.ssl ? '是' : '否'
    }
  },
  {
    title: '备注',
    key: 'remark'
  },
  {
    title: '操作',
    key: 'actions',
    render: (row: ProjectConfig) => {
      return h(NSpace, { justify: 'center' }, [
        h(
          NButton,
          {
            size: 'small',
            type: 'primary',
            onClick: () => editProject(row)
          },
          {
            icon: () => h(NIcon, null, { default: () => h(PencilOutline) })
          }
        ),
        h(
          NButton,
          {
            size: 'small',
            type: 'error',
            onClick: () => deleteProject(row)
          },
          {
            icon: () => h(NIcon, null, { default: () => h(TrashOutline) })
          }
        )
      ])
    }
  }
]

const templateColumns = [
  {
    title: '模板名称',
    key: 'name'
  },
  {
    title: '备注',
    key: 'remark'
  },
  {
    title: '操作',
    key: 'actions',
    render: (row: ProjectTemplate) => {
      return h(NSpace, { justify: 'center' }, [
        h(
          NButton,
          {
            size: 'small',
            type: 'primary',
            onClick: () => editTemplate(row)
          },
          {
            icon: () => h(NIcon, null, { default: () => h(PencilOutline) })
          }
        ),
        h(
          NButton,
          {
            size: 'small',
            type: 'error',
            onClick: () => deleteTemplate(row)
          },
          {
            icon: () => h(NIcon, null, { default: () => h(TrashOutline) })
          }
        )
      ])
    }
  }
]

// 项目相关方法
const fetchProjects = async () => {
  try {
    loading.value = true
    const result = await invoke('get_projects')
    projects.value = result as ProjectConfig[]
  } catch (err) {
    message.error(err instanceof Error ? err.message : '获取项目列表失败')
  } finally {
    loading.value = false
  }
}

const createProject = async () => {
  try {
    loading.value = true
    await invoke('create_project', { project: formValue.value })
    showCreateDialog.value = false
    await fetchProjects()
    message.success('创建项目成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '创建项目失败')
  } finally {
    loading.value = false
  }
}

const editProject = async (project: ProjectConfig) => {
  // TODO: 实现编辑项目功能
}

const deleteProject = async (project: ProjectConfig) => {
  try {
    loading.value = true
    await invoke('delete_project', { id: project.id })
    await fetchProjects()
    message.success('删除项目成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '删除项目失败')
  } finally {
    loading.value = false
  }
}

// 模板相关方法
const fetchTemplates = async () => {
  try {
    loading.value = true
    const result = await invoke('get_templates')
    templates.value = result as ProjectTemplate[]
  } catch (err) {
    message.error(err instanceof Error ? err.message : '获取模板列表失败')
  } finally {
    loading.value = false
  }
}

const createTemplate = async () => {
  try {
    loading.value = true
    await invoke('create_template', { template: templateFormValue.value })
    showCreateTemplateDialog.value = false
    await fetchTemplates()
    message.success('创建模板成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '创建模板失败')
  } finally {
    loading.value = false
  }
}

const editTemplate = async (template: ProjectTemplate) => {
  // TODO: 实现编辑模板功能
}

const deleteTemplate = async (template: ProjectTemplate) => {
  try {
    loading.value = true
    await invoke('delete_template', { id: template.id })
    await fetchTemplates()
    message.success('删除模板成功')
  } catch (err) {
    message.error(err instanceof Error ? err.message : '删除模板失败')
  } finally {
    loading.value = false
  }
}

// 初始化
fetchProjects()
fetchTemplates()
</script>

<style scoped>
.project-config {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style> 