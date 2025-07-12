import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { 
  getRedirectPath, 
  logRouteNavigation, 
  preloadRoutes,
  checkAuthStatus,
  debugRouteInfo
} from '../utils/router'

// 路由类型定义
export interface RouteMeta {
  title?: string
  requiresAuth?: boolean
  requiresAdmin?: boolean
  layout?: 'desktop' | 'independent'
  preload?: boolean
  device?: 'desktop' | 'all'
}

// 预加载重要页面
const preloadImportantPages = () => {
  preloadRoutes(routes)
}

const routes: RouteRecordRaw[] = [
  // 首页重定向
  { 
    path: '/', 
    redirect: '/community',
    meta: { title: '首页' }
  },

  // 认证相关路由
  { 
    path: '/login', 
    component: () => import('../views/desktop/Login.vue'),
    meta: { 
      title: '登录',
      layout: 'independent',
      device: 'all'
    }
  },

  // 资源社区路由（独立布局）
  { 
    path: '/community', 
    component: () => import('../views/community/desktop/CommunityHome.vue'),
    meta: { 
      title: '资源社区',
      layout: 'independent',
      device: 'all',
      preload: true,
      requiresAuth: false
    }
  },
  { 
    path: '/community/resource/:id', 
    component: () => import('../views/community/desktop/CommunityHome.vue'),
    meta: { 
      title: '资源详情',
      layout: 'independent',
      device: 'all',
      requiresAuth: false
    }
  },
  { 
    path: '/community/hot', 
    component: () => import('../views/community/desktop/CommunityHome.vue'),
    meta: { 
      title: '热门资源',
      layout: 'independent',
      device: 'all',
      requiresAuth: false
    }
  },
  { 
    path: '/community/latest', 
    component: () => import('../views/community/desktop/CommunityHome.vue'),
    meta: { 
      title: '最新资源',
      layout: 'independent',
      device: 'all',
      requiresAuth: false
    }
  },
  { 
    path: '/community/category/:category', 
    component: () => import('../views/community/desktop/CommunityHome.vue'),
    meta: { 
      title: '分类资源',
      layout: 'independent',
      device: 'all',
      requiresAuth: false
    }
  },

  // 后台管理路由（桌面端）
  { 
    path: '/dashboard', 
    component: () => import('../views/desktop/Dashboard.vue'),
    meta: { 
      requiresAuth: true, 
      title: '仪表盘',
      layout: 'desktop',
      device: 'desktop',
      preload: true
    }
  },
  { 
    path: '/users', 
    component: () => import('../views/desktop/UserManage.vue'),
    meta: { 
      requiresAuth: true, 
      title: '用户管理',
      layout: 'desktop',
      device: 'desktop',
      preload: true
    }
  },
  { 
    path: '/packages', 
    component: () => import('../views/desktop/PackageManage.vue'),
    meta: { 
      requiresAuth: true, 
      title: '资源管理',
      layout: 'desktop',
      device: 'desktop',
      preload: true
    }
  },
  { 
    path: '/logs', 
    component: () => import('../views/desktop/LogView.vue'),
    meta: { 
      requiresAuth: true, 
      title: '日志查看',
      layout: 'desktop',
      device: 'desktop'
    }
  },
  { 
    path: '/stats', 
    component: () => import('../views/desktop/Stats.vue'),
    meta: { 
      requiresAuth: true, 
      title: '统计信息',
      layout: 'desktop',
      device: 'desktop',
      preload: true
    }
  },
  { 
    path: '/theme-settings', 
    component: () => import('../views/desktop/ThemeSettings.vue'),
    meta: { 
      requiresAuth: true, 
      title: '主题设置',
      layout: 'desktop',
      device: 'desktop'
    }
  },



  // 404 页面
  { 
    path: '/:pathMatch(.*)*', 
    component: () => import('../views/desktop/Dashboard.vue'),
    meta: { 
      title: '页面未找到',
      layout: 'desktop',
      device: 'all'
    }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 路由守卫
router.beforeEach((to: any, from: any, next: any) => {
  // 记录路由导航开始
  logRouteNavigation(to, from, 'start')
  
  // 调试路由信息
  debugRouteInfo(to, from)
  
  // 设置页面标题
  if (to.meta?.title) {
    document.title = `${to.meta.title} - 绳包管理系统`
  }

  // 检查是否需要重定向
  const redirectPath = getRedirectPath(to, from)
  if (redirectPath) {
    console.log('🔄 路由重定向:', { from: to.path, to: redirectPath })
    logRouteNavigation(to, from, 'redirect')
    next(redirectPath)
    return
  }
  
  // 记录路由导航完成
  logRouteNavigation(to, from, 'complete')
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