<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const props = defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'toggle'): void
}>()

const router = useRouter()
const route = useRoute()

const navigation = [
  { name: '仪表盘', path: '/' },
  { name: 'Nginx 版本', path: '/versions' },
  { name: '配置文件', path: '/configs' },
  { name: 'SSL 证书', path: '/certificates' },
  { name: '服务控制', path: '/service' },
  { name: '高级设置', path: '/advanced' }
]

const isActive = (path: string) => {
  return route.path === path
}
</script>

<template>
  <aside
    :class="{
      'w-64': isOpen,
      'w-0': !isOpen
    }"
    class="fixed top-0 left-0 h-screen bg-white shadow-lg transition-all duration-300"
  >
    <div class="h-full flex flex-col">
      <!-- Logo -->
      <div class="p-4 border-b">
        <h1 class="text-xl font-bold text-gray-800">Nginx GUI</h1>
      </div>

      <!-- 导航菜单 -->
      <nav class="flex-1 overflow-y-auto">
        <ul class="p-2">
          <li
            v-for="item in navigation"
            :key="item.path"
            class="mb-1"
          >
            <router-link
              :to="item.path"
              :class="{
                'bg-blue-50 text-blue-600': isActive(item.path),
                'text-gray-700 hover:bg-gray-50': !isActive(item.path)
              }"
              class="flex items-center px-4 py-2 rounded-md transition-colors"
            >
              <span class="ml-3">{{ item.name }}</span>
            </router-link>
          </li>
        </ul>
      </nav>

      <!-- 折叠按钮 -->
      <div class="p-4 border-t">
        <button
          @click="emit('toggle')"
          class="w-full flex items-center justify-center px-4 py-2 text-gray-600 hover:bg-gray-50 rounded-md transition-colors"
        >
          <span class="mr-2">{{ isOpen ? '收起' : '展开' }}</span>
          <svg
            :class="{
              'transform rotate-180': !isOpen
            }"
            class="w-4 h-4 transition-transform"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M15 19l-7-7 7-7"
            />
          </svg>
        </button>
      </div>
    </div>
  </aside>
</template> 