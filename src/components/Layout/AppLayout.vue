<script setup lang="ts">
import { ref, computed, h, watch, onMounted } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import AppSidebar from './AppSidebar.vue'
import AppMain from './AppMain.vue'
import StatusIndicator from '../Nginx/StatusIndicator.vue'
import { 
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NButton,
  NSpace,
  NIcon,
  NMenu,
  NAvatar,
  NDropdown,
  NConfigProvider,
  darkTheme,
  zhCN,
  dateZhCN,
  useMessage
} from 'naive-ui'
import { 
  MenuOutline,
  PersonOutline,
  SettingsOutline,
  LogOutOutline,
  MoonOutline,
  SunnyOutline
} from '@vicons/ionicons5'

const route = useRoute()
const collapsed = ref(false)
const isDark = ref(false)

const message = useMessage()

// 从系统设置中获取主题设置
const loadTheme = async () => {
  try {
    const settings = await invoke('get_settings')
    const theme = (settings as any).theme
    if (theme === 'dark') {
      isDark.value = true
    } else if (theme === 'light') {
      isDark.value = false
    } else if (theme === 'system') {
      // 检查系统主题
      const isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches
      isDark.value = isDarkMode
    }
  } catch (err) {
    console.error('获取主题设置失败:', err)
  }
}

// 监听系统主题变化
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
mediaQuery.addEventListener('change', (e) => {
  const settings = invoke('get_settings')
  if ((settings as any).theme === 'system') {
    isDark.value = e.matches
  }
})

const theme = computed(() => isDark.value ? darkTheme : null)

const userMenuOptions = [
  {
    label: '个人设置',
    key: 'settings',
    icon: () => h(NIcon, null, { default: () => h(SettingsOutline) })
  },
  {
    label: '退出登录',
    key: 'logout',
    icon: () => h(NIcon, null, { default: () => h(LogOutOutline) })
  }
]

const handleUserMenuSelect = (key: string) => {
  switch (key) {
    case 'settings':
      // 处理设置
      break
    case 'logout':
      // 处理退出
      break
  }
}

// 切换主题
const toggleTheme = async () => {
  isDark.value = !isDark.value
  try {
    const settings = await invoke('get_settings') as any
    await invoke('save_settings', {
      settings: {
        ...settings,
        theme: isDark.value ? 'dark' : 'light'
      }
    })
    message.success(isDark.value ? '已切换到深色主题' : '已切换到浅色主题')
  } catch (err) {
    console.error('保存主题设置失败:', err)
    message.error('保存主题设置失败')
  }
}

onMounted(() => {
  loadTheme()
})
</script>

<template>
  <NConfigProvider
    :theme="theme"
    :locale="zhCN"
    :date-locale="dateZhCN"
  >
    <NLayout class="h-screen">
      <NLayoutHeader class="h-16 px-4 flex items-center justify-between border-b">
        <div class="flex items-center">
          <NButton
            quaternary
            circle
            @click="collapsed = !collapsed"
          >
            <template #icon>
              <NIcon>
                <MenuOutline />
              </NIcon>
            </template>
          </NButton>
          <span class="ml-4 text-xl font-bold">Nginx GUI</span>
        </div>
        
        <NSpace>
          <StatusIndicator />
          
          <NButton
            quaternary
            circle
            @click="toggleTheme"
          >
            <template #icon>
              <NIcon>
                <component :is="isDark ? SunnyOutline : MoonOutline" />
              </NIcon>
            </template>
          </NButton>
        </NSpace>
      </NLayoutHeader>

      <NLayout has-sider>
        <NLayoutSider
          :collapsed="collapsed"
          :collapsed-width="64"
          :width="240"
          show-trigger
          @collapse="collapsed = true"
          @expand="collapsed = false"
        >
          <AppSidebar :collapsed="collapsed" />
        </NLayoutSider>

        <NLayoutContent>
          <AppMain>
            <slot />
          </AppMain>
        </NLayoutContent>
      </NLayout>
    </NLayout>
  </NConfigProvider>
</template> 