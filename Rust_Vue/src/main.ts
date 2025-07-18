import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

// 导入主题样式
import './assets/base.css'
import './assets/global.css'
import './assets/element-plus-dark.css'
import './assets/theme-variables.css'

// 导入主应用组件
import App from './App.vue'

// 导入路由
import router from './router'

// 导入工具函数
import { initTheme } from './utils/theme'

// 创建应用
const app = createApp(App)

// 注册Element Plus图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

// 使用插件
app.use(ElementPlus)
app.use(router)

// 初始化主题系统
const cleanupTheme = initTheme()

// 挂载应用
app.mount('#app')

// 清理函数
window.addEventListener('beforeunload', () => {
  if (cleanupTheme) {
    cleanupTheme()
  }
})

// 导出应用实例（用于开发调试）
export default app
