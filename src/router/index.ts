import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '../views/Dashboard.vue'
import Configs from '../views/Configs.vue'
import Logs from '../views/Logs.vue'
import Settings from '../views/Settings.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: Dashboard
    },
    {
      path: '/configs',
      name: 'configs',
      component: Configs
    },
    {
      path: '/logs',
      name: 'logs',
      component: Logs
    },
    {
      path: '/settings',
      name: 'settings',
      component: Settings
    }
  ]
})

export default router 