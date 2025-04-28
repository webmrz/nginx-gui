<script setup lang="ts">
import { computed, h, Component } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import { 
  NMenu,
  NIcon,
  MenuOption
} from 'naive-ui'
import {
  HomeOutline,
  DocumentTextOutline,
  SettingsOutline,
  JournalOutline
} from '@vicons/ionicons5'

interface Props {
  collapsed: boolean
}

const props = defineProps<Props>()
const route = useRoute()

const renderIcon = (icon: Component) => {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions: MenuOption[] = [
  {
    label: '仪表盘',
    key: 'dashboard',
    icon: renderIcon(HomeOutline),
    path: '/'
  },
  {
    label: '配置文件',
    key: 'configs',
    icon: renderIcon(DocumentTextOutline),
    path: '/configs'
  },
  {
    label: '日志查看',
    key: 'logs',
    icon: renderIcon(JournalOutline),
    path: '/logs'
  },
  {
    label: '系统设置',
    key: 'settings',
    icon: renderIcon(SettingsOutline),
    path: '/settings'
  }
]

const selectedKey = computed(() => {
  return route.path === '/' ? 'dashboard' : route.path.slice(1)
})
</script>

<template>
  <NMenu
    :collapsed="collapsed"
    :collapsed-width="64"
    :collapsed-icon-size="22"
    :options="menuOptions"
    :value="selectedKey"
    :indent="18"
    :root-indent="18"
    :inverted="false"
    @update:value="(key) => $router.push(menuOptions.find(option => option.key === key)?.path || '/')"
  />
</template> 