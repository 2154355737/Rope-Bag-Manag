import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { isLoggedIn, isLoginExpired } from '../utils/auth'
import { getDeviceType, shouldUseMobileVersion } from '../utils/device'

const routes: RouteRecordRaw[] = [
  { path: '/', redirect: '/login' },
  { path: '/login', component: () => import('../views/Login.vue') },
  { 
    path: '/dashboard', 
    component: () => import('../views/Dashboard.vue'),
    meta: { requiresAuth: true }
  },
  { 
    path: '/users', 
    component: () => import('../views/UserManage.vue'),
    meta: { requiresAuth: true }
  },
  { 
    path: '/users-mobile', 
    component: () => import('../views/UserManageMobile.vue'),
    meta: { requiresAuth: true }
  },
  { 
    path: '/packages', 
    component: () => import('../views/PackageManage.vue'),
    meta: { requiresAuth: true }
  },
  { 
    path: '/logs', 
    component: () => import('../views/LogView.vue'),
    meta: { requiresAuth: true }
  },
  { 
    path: '/stats', 
    component: () => import('../views/StatsView.vue'),
    meta: { requiresAuth: true }
  },
  { 
    path: '/device-test', 
    component: () => import('../views/DeviceTest.vue'),
    meta: { requiresAuth: true }
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 自动设备检测和页面切换
function autoRedirectToMobileVersion(to: any) {
  // 如果是用户管理页面且应该使用移动端版本
  if (to.path === '/users' && shouldUseMobileVersion()) {
    return '/users-mobile'
  }
  // 如果是移动端用户管理页面但应该使用桌面端版本
  if (to.path === '/users-mobile' && !shouldUseMobileVersion()) {
    return '/users'
  }
  return null
}

// 路由守卫
router.beforeEach((to: any, from: any, next: any) => {
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
  
  // 自动设备检测和页面切换
  const redirectPath = autoRedirectToMobileVersion(to)
  if (redirectPath) {
    next(redirectPath)
    return
  }
  
  next()
})

export default router 