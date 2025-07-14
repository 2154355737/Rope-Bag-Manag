import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { 
  getRedirectPath, 
  logRouteNavigation, 
  preloadRoutes,
  checkAuthStatus,
  debugRouteInfo
} from '../utils/router'
import { getUserInfo } from '../utils/auth'

// 路由类型定义
export interface RouteMeta {
  title?: string
  requiresAuth?: boolean
  requiresAdmin?: boolean
  layout?: 'desktop' | 'independent'
  preload?: boolean
  device?: 'desktop' | 'all'
  roles?: string[] // 新增：允许访问的角色
}

// 预加载重要页面
const preloadImportantPages = () => {
  preloadRoutes(routes)
}

const routes: RouteRecordRaw[] = [
  // 首页重定向
  { 
    path: '/', 
    redirect: '/home',
    meta: { title: '首页' }
  },
  // 主站相关
  {
    path: '/home',
    component: () => import('../views/main/Home.vue'),
    meta: { title: '资源主站', layout: 'independent', device: 'all', preload: true }
  },
  {
    path: '/category/:category',
    component: () => import('../views/main/Category.vue'),
    meta: { title: '资源分类', layout: 'independent', device: 'all' }
  },
  {
    path: '/resource/:id',
    component: () => import('../views/main/ResourceDetail.vue'),
    meta: { title: '资源详情', layout: 'independent', device: 'all' }
  },
  {
    path: '/resource/:id/comments',
    component: () => import('../views/main/ResourceComment.vue'),
    meta: { title: '资源评论', layout: 'independent', device: 'all' }
  },
  // 登录
  {
    path: '/login',
    component: () => import('../views/auth/Login.vue'),
    meta: { title: '登录', layout: 'independent', device: 'all' }
  },
  // 注册
  {
    path: '/register',
    component: () => import('../views/auth/Register.vue'),
    meta: { title: '注册', layout: 'independent', device: 'all' }
  },
  // 管理员后台
  {
    path: '/admin',
    component: () => import('../views/admin/Dashboard.vue'),
    meta: { title: '管理员后台', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', preload: true, roles: ['admin'] }
  },
  {
    path: '/admin/users',
    component: () => import('../views/admin/UserManage.vue'),
    meta: { title: '用户管理', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin'] }
  },
  {
    path: '/admin/packages',
    component: () => import('../views/admin/PackageManage.vue'),
    meta: { title: '资源管理', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin', 'moderator'] }
  },
  {
    path: '/admin/logs',
    component: () => import('../views/admin/LogView.vue'),
    meta: { title: '日志查看', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin'] }
  },
  {
    path: '/admin/stats',
    component: () => import('../views/admin/Stats.vue'),
    meta: { title: '统计信息', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin', 'moderator'] }
  },
  {
    path: '/admin/theme-settings',
    component: () => import('../views/admin/ThemeSettings.vue'),
    meta: { title: '系统设置', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin'] }
  },
  {
    path: '/admin/comments',
    component: () => import('../views/admin/CommentManage.vue'),
    meta: { title: '评论管理', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin', 'moderator'] }
  },
  {
    path: '/admin/user-actions',
    component: () => import('../views/admin/UserActions.vue'),
    meta: { title: '用户行为记录', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin'] }
  },
  {
    path: '/admin/resource-records',
    component: () => import('../views/admin/ResourceRecord.vue'),
    meta: { title: '资源记录', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin'] }
  },
  {
    path: '/admin/backup',
    component: () => import('../views/admin/BackupManage.vue'),
    meta: { title: '备份管理', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin'] }
  },
  {
    path: '/admin/announcements',
    component: () => import('../views/admin/AnnouncementManage.vue'),
    meta: { title: '公告管理', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop' }
  },
  {
    path: '/admin/user-action-log',
    component: () => import('../views/admin/UserActionLog.vue'),
    meta: { title: '用户操作日志', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop' }
  },
  // 元老后台
  {
    path: '/elder',
    component: () => import('../views/elder/Dashboard.vue'),
    meta: { title: '元老后台', requiresAuth: true, layout: 'elder', device: 'desktop', roles: ['elder'] }
  },
  {
    path: '/elder/resources',
    component: () => import('../views/elder/ResourceManage.vue'),
    meta: { title: '资源管理', requiresAuth: true, layout: 'elder', device: 'desktop', roles: ['elder'] }
  },
  {
    path: '/elder/profile',
    component: () => import('../views/elder/Profile.vue'),
    meta: { title: '个人信息', requiresAuth: true, layout: 'elder', device: 'desktop', roles: ['elder'] }
  },
  {
    path: '/elder/comments',
    component: () => import('../views/elder/ElderComments.vue'),
    meta: { title: '我的评论', requiresAuth: true, layout: 'elder', device: 'desktop', roles: ['elder'] }
  },
  {
    path: '/elder/my-resources',
    component: () => import('../views/elder/ElderMyResources.vue'),
    meta: { title: '我的资源', requiresAuth: true, layout: 'elder', device: 'desktop', roles: ['elder'] }
  },
  // 普通用户后台
  {
    path: '/user',
    component: () => import('../views/user/Dashboard.vue'),
    meta: { title: '个人中心', requiresAuth: true, layout: 'user', device: 'desktop', roles: ['user'] }
  },
  {
    path: '/user/resources',
    component: () => import('../views/user/UserResources.vue'),
    meta: { title: '我的资源', requiresAuth: true, layout: 'user', device: 'desktop', roles: ['user'] }
  },
  {
    path: '/user/profile',
    component: () => import('../views/user/UserProfile.vue'),
    meta: { title: '个人信息', requiresAuth: true, layout: 'user', device: 'desktop', roles: ['user'] }
  },
  {
    path: '/user/comments',
    component: () => import('../views/user/UserComments.vue'),
    meta: { title: '我的评论', requiresAuth: true, layout: 'user', device: 'desktop', roles: ['user'] }
  },
  // 403无权限页面
  {
    path: '/403',
    component: () => import('../views/Forbidden.vue'),
    meta: { title: '无权限', layout: 'independent', device: 'all' }
  },
  // 404 页面
  { 
    path: '/:pathMatch(.*)*',
    component: () => import('../views/admin/Dashboard.vue'),
    meta: { title: '页面未找到' }
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

  // 登录校验：未登录不能访问后台页面（包括所有子路由）
  const token = localStorage.getItem('loginToken')
  if ((to.path.startsWith('/admin') || to.path.startsWith('/user')) && !token) {
    return next({ path: '/login', replace: true })
  }

  // 路由守卫：角色权限控制
  const user = getUserInfo()
  const userRole = user?.role || 'guest'
  const routeRoles = to.meta.roles as string[] | undefined
  if (routeRoles && !routeRoles.includes(userRole)) {
    return next('/403')
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