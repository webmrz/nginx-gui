<script setup lang="ts">
import { ref } from 'vue'
import Sidebar from './Sidebar.vue'
import Header from './Header.vue'

const isSidebarOpen = ref(true)

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value
}
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <!-- 侧边栏 -->
    <Sidebar :is-open="isSidebarOpen" @toggle="toggleSidebar" />

    <!-- 主内容区 -->
    <div
      :class="{
        'ml-64': isSidebarOpen,
        'ml-0': !isSidebarOpen
      }"
      class="transition-all duration-300"
    >
      <!-- 顶部导航栏 -->
      <Header @toggle-sidebar="toggleSidebar" />

      <!-- 页面内容 -->
      <main class="p-4">
        <slot />
      </main>
    </div>
  </div>
</template> 