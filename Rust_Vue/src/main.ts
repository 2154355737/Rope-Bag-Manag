import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

// 导入字体
import './assets/fonts.css'

// 导入主题样式 - 按优先级顺序引入
import './assets/base.css'              // 基础样式
import './assets/global.css'            // 全局样式
import './assets/theme-variables.css'   // 主题变量系统（优先级最高）
import './assets/quick-fixes.css'       // 快速修复样式
import './assets/element-plus-dark.css' // Element Plus深色主题
import './assets/admin-common.css'      // 管理页面通用样式
import './assets/home-styles.css'       // 首页样式

// 导入主应用组件
import App from './App.vue'

// 导入路由
import router from './router'

// 导入工具函数
import { initTheme } from './utils/theme'

// 导入资源记录服务
import { resourceLogger } from './utils/loggerService'

// 导入会话追踪器
import sessionTracker from './utils/sessionTracker'

// 导入认证相关函数
import { refreshUserInfo } from './utils/auth'

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

// 将资源记录服务添加到全局属性中
app.config.globalProperties.$resourceLogger = resourceLogger
// 也可以通过provide/inject使用
app.provide('resourceLogger', resourceLogger)

// 暂时完全禁用会话追踪器，直到问题解决
// sessionTracker.init()

// 页面加载时自动调用refreshUserInfo，确保lastUserInfoValid正确。
refreshUserInfo()
  .then(() => {
    console.log('用户信息刷新完成')
  })
  .catch(() => {
    console.warn('用户信息刷新失败')
  })

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
