import { getDeviceType } from './device'
import { isLoggedIn, isLoginExpired, getUserInfo } from './auth'
import { RouteLocationNormalized } from 'vue-router'
import userActionService from './userActionService'

// 路由工具函数（简化版，仅支持桌面端）

/**
 * 获取适合当前设备的路径
 * @param path 原始路径
 * @returns 适配后的路径
 */
export function getDeviceAwarePath(path: string): string {
  // 移除移动端前缀
  if (path.startsWith('/mobile/')) {
    return path.replace('/mobile/', '/')
  }
  
  return path
}

/**
 * 检查路由是否需要认证
 * @param meta 路由元信息
 * @returns 是否需要认证
 */
export function requiresAuth(meta: any): boolean {
  return meta?.requiresAuth === true
}

/**
 * 检查路由是否需要管理员权限
 * @param meta 路由元信息
 * @returns 是否需要管理员权限
 */
export function requiresAdmin(meta: any): boolean {
  return meta?.requiresAdmin === true
}

/**
 * 获取路由的布局类型
 * @param meta 路由元信息
 * @returns 布局类型
 */
export function getRouteLayout(meta: any): 'desktop' | 'independent' {
  return meta?.layout || 'desktop'
}

/**
 * 检查路由是否适合当前设备
 * @param meta 路由元信息
 * @returns 是否适合当前设备
 */
export function isRouteDeviceCompatible(meta: any): boolean {
  const routeDevice = meta?.device || 'all'
  
  return routeDevice === 'all' || routeDevice === 'desktop'
}

/**
 * 获取登录后的默认路径
 * @returns 默认路径
 */
export function getDefaultPathAfterLogin(): string {
  return '/dashboard'
}

/**
 * 获取未登录时的默认路径
 * @returns 默认路径
 */
export function getDefaultPathForUnauthenticated(): string {
  return '/community'
}

/**
 * 检查用户认证状态
 * @returns 认证状态对象
 */
export function checkAuthStatus() {
  const isAuthenticated = isLoggedIn() && !isLoginExpired()
  
  return {
    isAuthenticated,
    isLoggedIn: isLoggedIn(),
    isExpired: isLoginExpired(),
    userInfo: isAuthenticated ? JSON.parse(localStorage.getItem('userInfo') || '{}') : null
  }
}

/**
 * 获取路由重定向路径
 * @param to 目标路由
 * @param from 来源路由
 * @returns 重定向路径或null
 */
export function getRedirectPath(to: any, from: any): string | null {
  // 首页重定向
  if (to.path === '/') {
    return '/community'
  }
  
  // 社区相关路由不需要重定向
  if (to.path.startsWith('/community')) {
    return null
  }
  
  // 登录页面特殊处理
  if (to.path === '/login') {
    const authStatus = checkAuthStatus()
    if (authStatus.isAuthenticated) {
      return getDefaultPathAfterLogin()
    }
    return null
  }
  
  // 移除移动端路由重定向
  if (to.path.startsWith('/mobile/')) {
    return to.path.replace('/mobile/', '/')
  }
  
  // 认证相关重定向
  if (to.meta?.requiresAuth) {
    const authStatus = checkAuthStatus()
    
    if (!authStatus.isAuthenticated) {
      return getDefaultPathForUnauthenticated()
    }
  }
  
  return null
}

/**
 * 记录路由导航
 */
export const logRouteNavigation = (to: RouteLocationNormalized) => {
  const toPath = to.path
  const toName = to.meta.title || toPath
  
  try {
    const userInfo = getUserInfo()
    console.log(`路由导航: ${toPath}, 页面: ${toName}`)
    
    // 记录用户页面访问
    if (userInfo && userInfo.id) {
      userActionService.logPageView(toPath)
        .catch(err => console.error('记录页面访问失败:', err))
    }
    
  } catch (error) {
    console.error('记录路由导航失败:', error)
  }
}

/**
 * 路由调试信息
 * @param to 目标路由
 * @param from 来源路由
 */
export function debugRouteInfo(to: any, from: any) {
  const authStatus = checkAuthStatus()
  
  console.group('🔍 路由调试信息')
  console.log('目标路由:', {
    path: to.path,
    name: to.name,
    meta: to.meta,
    layout: to.meta?.layout,
    device: to.meta?.device,
    requiresAuth: to.meta?.requiresAuth
  })
  
  console.log('来源路由:', {
    path: from.path,
    name: from.name,
    meta: from.meta
  })
  
  console.log('设备信息:', {
    deviceType: 'desktop',
    isMobile: false
  })
  
  console.log('认证状态:', authStatus)
  
  console.log('布局判断:', {
    needsIndependentLayout: to.meta?.layout === 'independent',
    needsDesktopLayout: to.meta?.layout === 'desktop'
  })
  
  console.groupEnd()
}

/**
 * 预加载路由组件
 * @param routes 路由配置
 */
export function preloadRoutes(routes: any[]) {
  const importantRoutes = routes.filter(route => route.meta?.preload)
  
  importantRoutes.forEach(route => {
    if (route.component && typeof route.component === 'function') {
      const componentLoader = route.component as () => Promise<any>
      componentLoader().catch(err => {
        console.warn(`预加载组件失败: ${route.path}`, err)
      })
    }
  })
} 