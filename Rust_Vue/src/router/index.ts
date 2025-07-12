import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { isLoggedIn, isLoginExpired } from '../utils/auth'
import { shouldUseMobileVersion } from '../utils/device'

// 动态懒加载组件，根据设备类型选择desktop或mobile版本
const lazyLoad = (component: string, preload = false) => {
  return () => {
    if (preload) {
      // 预加载时显示加载提示
      console.log(`预加载组件: ${component}`)
    }
    
    // 根据设备类型选择组件路径
    const isMobile = shouldUseMobileVersion()
    const componentPath = isMobile ? `../views/mobile/${component}.vue` : `../views/desktop/${component}.vue`
    
    return import(componentPath)
  }
}

// 动态懒加载组件（指定版本）
const lazyLoadVersion = (component: string, version: 'desktop' | 'mobile', preload = false) => {
  return () => {
    if (preload) {
      console.log(`预加载组件: ${component} (${version})`)
    }
    return import(`../views/${version}/${component}.vue`)
  }
}

const routes: RouteRecordRaw[] = [
  { path: '/', redirect: '/login' },
  { 
    path: '/login', 
    component: lazyLoad('Login'),
    meta: { title: '登录' }
  },
  { 
    path: '/dashboard', 
    component: lazyLoad('Dashboard'),
    meta: { 
      requiresAuth: true, 
      title: '仪表盘',
      preload: true // 预加载仪表盘
    }
  },
  { 
    path: '/users', 
    component: lazyLoad('UserManage'),
    meta: { 
      requiresAuth: true, 
      title: '用户管理',
      preload: true // 预加载用户管理
    }
  },
  { 
    path: '/packages', 
    component: lazyLoad('PackageManage'),
    meta: { 
      requiresAuth: true, 
      title: '绳包管理',
      preload: true // 预加载绳包管理
    }
  },
  { 
    path: '/logs', 
    component: lazyLoad('LogView'),
    meta: { 
      requiresAuth: true, 
      title: '日志查看'
    }
  },
  { 
    path: '/stats', 
    component: lazyLoad('Stats'),
    meta: { 
      requiresAuth: true, 
      title: '统计信息',
      preload: true // 预加载统计页面
    }
  },
  { 
    path: '/theme-settings', 
    component: lazyLoadVersion('ThemeSettings', 'desktop'),
    meta: { 
      requiresAuth: true, 
      title: '主题设置'
    }
  },
  // 移动端专用路由（可选，用于直接访问移动端版本）
  { 
    path: '/mobile/login', 
    component: lazyLoadVersion('Login', 'mobile'),
    meta: { title: '登录 (移动端)' }
  },
  { 
    path: '/mobile/dashboard', 
    component: lazyLoadVersion('Dashboard', 'mobile'),
    meta: { 
      requiresAuth: true, 
      title: '仪表盘 (移动端)'
    }
  },
  { 
    path: '/mobile/users', 
    component: lazyLoadVersion('UserManage', 'mobile'),
    meta: { 
      requiresAuth: true, 
      title: '用户管理 (移动端)'
    }
  },
  { 
    path: '/mobile/packages', 
    component: lazyLoadVersion('PackageManage', 'mobile'),
    meta: { 
      requiresAuth: true, 
      title: '绳包管理 (移动端)'
    }
  },
  { 
    path: '/mobile/logs', 
    component: lazyLoadVersion('LogView', 'mobile'),
    meta: { 
      requiresAuth: true, 
      title: '日志查看 (移动端)'
    }
  },
  { 
    path: '/mobile/stats', 
    component: lazyLoadVersion('Stats', 'mobile'),
    meta: { 
      requiresAuth: true, 
      title: '统计信息 (移动端)'
    }
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 预加载重要页面
const preloadImportantPages = () => {
  const importantRoutes = routes.filter(route => route.meta?.preload)
  importantRoutes.forEach(route => {
    if (route.component && typeof route.component === 'function') {
      // 调用懒加载函数来预加载组件
      const componentLoader = route.component as () => Promise<any>
      componentLoader().catch(err => {
        console.warn(`预加载组件失败: ${route.path}`, err)
      })
    }
  })
}

// 路由守卫
router.beforeEach((to: any, from: any, next: any) => {
  // 设置页面标题
  if (to.meta?.title) {
    document.title = `${to.meta.title} - 绳包管理系统`
  }
  
  // 检查路由是否需要登录
  if (to.meta.requiresAuth) {
    // 检查是否已登录
    if (!isLoggedIn()) {
      // 未登录，重定向到登录页
      next('/login')
      return
    }
    
    // 检查登录是否过期
    if (isLoginExpired()) {
      // 登录过期，清除状态并重定向到登录页
      localStorage.removeItem('isLoggedIn')
      localStorage.removeItem('userInfo')
      next('/login')
      return
    }
  }
  
  // 如果已登录且访问登录页，重定向到仪表盘
  if (to.path === '/login') {
    if (isLoggedIn() && !isLoginExpired()) {
      next('/dashboard')
      return
    }
  }
  
  next()
})

// 路由后置守卫，用于预加载
router.afterEach((to) => {
  // 在路由切换后预加载其他重要页面
  if (to.meta?.preload) {
    setTimeout(() => {
      preloadImportantPages()
    }, 1000) // 延迟1秒预加载
  }
})

export default router 