import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { 
  getRedirectPath, 
  logRouteNavigation, 
  preloadRoutes,
  checkAuthStatus,
  debugRouteInfo
} from '../utils/router'
import { getUserInfo, getToken, refreshUserInfo, isLoginExpired } from '../utils/auth'
import { resourceLogger } from '../utils/loggerService'

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
  {
    path: '/forgot-password',
    component: () => import('../views/auth/ForgotPassword.vue'),
    meta: { title: '忘记密码', layout: 'independent', device: 'all' }
  },
  {
    path: '/auth/reset-password',
    component: () => import('../views/auth/ResetPassword.vue'),
    meta: { title: '重置密码', layout: 'independent', device: 'all' }
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
  {
    path: '/admin/mail-settings',
    component: () => import('../views/admin/MailSettings.vue'),
    meta: { title: '邮件设置', requiresAuth: true, requiresAdmin: true, layout: 'desktop', device: 'desktop', roles: ['admin'] }
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
    component: () => import('../views/NotFound.vue'),
    meta: { title: '页面未找到', layout: 'independent', device: 'all' }
  }
]

// 创建路由实例
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

// 全局前置守卫 - 简化版本，解决循环跳转问题
router.beforeEach(async (to, from, next) => {
  // 页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} - 绳包管理系统`
  } else {
    document.title = '绳包管理系统'
  }
  
  // 获取认证状态和用户信息
  const token = getToken()
  const userInfo = getUserInfo()
  const isAuthenticated = !!token && !!userInfo
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth)
  
  // 添加调试信息
  console.log('🔍 路由守卫调试:', {
    to: to.path,
    from: from.path,
    token: token ? '存在' : '无',
    userInfo: userInfo ? `${userInfo.username}(${userInfo.role})` : '无',
    isAuthenticated,
    requiresAuth
  })
  
  // 1. 如果是登录页面
  if (to.path === '/login') {
    if (isAuthenticated && userInfo?.role) {
      // 已登录用户访问登录页，重定向到对应后台
      let redirectPath = '/user'
      if (userInfo.role === 'admin') {
        redirectPath = '/admin'
      } else if (userInfo.role === 'elder') {
        redirectPath = '/elder'
      }
      console.log(`✅ 已登录用户重定向: ${userInfo.role} -> ${redirectPath}`)
      return next({ path: redirectPath, replace: true })
    } else {
      // 未登录用户访问登录页，允许
      return next()
    }
  }
  
  // 2. 需要认证的页面
  if (requiresAuth) {
    if (!isAuthenticated) {
    console.warn(`🚫 未认证访问被拦截: ${to.path}`)
    return next({ 
      path: '/login', 
      query: { redirect: to.fullPath },
      replace: true 
    })
  }
  
    // 检查封禁状态
    if (userInfo?.ban_status && userInfo.ban_status !== 'normal') {
    console.warn(`🚫 封禁用户访问被拦截: ${userInfo.username} (${userInfo.ban_status})`)
    // 清除认证信息
    await logout()
    return next({ 
      path: '/login', 
      query: { 
        error: 'banned',
        message: userInfo.ban_status === 'suspended' ? '账户已被暂停' : '账户已被封禁'
      },
      replace: true 
    })
  }
  
    // 检查角色权限
  const requiredRoles = to.meta.roles as string[]
  if (requiredRoles && requiredRoles.length > 0) {
    const userRole = userInfo?.role || 'guest'
  
    if (!requiredRoles.includes(userRole)) {
      console.warn(`🚫 角色权限不足: 需要 ${requiredRoles.join('|')}, 当前 ${userRole}`)
      return next({ path: '/403', replace: true })
    }
  }
  
    // 检查管理员权限
    if (to.meta.requiresAdmin && userInfo?.role !== 'admin') {
      console.warn(`🚫 管理员权限不足: 当前角色 ${userInfo?.role}`)
      return next({ path: '/403', replace: true })
    }
  }
  
  // 3. 记录路由访问日志（只有在用户已认证时才记录）
  if (isAuthenticated && token && userInfo?.username) {
    logRouteNavigation(to)
  }
  
  // 4. 允许访问
  console.log(`✅ 路由访问允许: ${to.path}`)
  next()
})

// 全局后置钩子 - 添加详细访问记录
router.afterEach((to, from) => {
  // 如果用户已登录，记录更详细的页面访问信息
  // 暂时禁用所有用户行为记录，避免疯狂请求问题
  /*
  if (localStorage.getItem('isLoggedIn') === 'true') {
    const userInfo = getUserInfo();
    const username = userInfo?.username || '未知用户';
    const fromPath = from.path;
    const toPath = to.path;
    const toName = to.meta.title || toPath;
    
    // 记录更详细的导航信息，包括来源页面
    import('../utils/userActionService').then(({ default: userActionService }) => {
      userActionService.logAction(
        'Navigation', 
        `从"${fromPath}"导航到"${toName}"`, 
        'Page', 
        undefined
      ).catch(err => console.error('记录导航行为失败:', err));
    }).catch(err => console.error('导入服务失败:', err));
    
    // 记录特定页面访问
    if (to.path.includes('/resource/')) {
      const resourceId = parseInt(to.params.id as string);
      if (!isNaN(resourceId)) {
        import('../utils/userActionService').then(({ default: userActionService }) => {
          userActionService.logView('Package', resourceId, `查看资源详情`)
            .catch(err => console.error('记录资源查看行为失败:', err));
        }).catch(err => console.error('导入服务失败:', err));
      }
    }
    
    // 记录分类页面访问
    if (to.path.includes('/category/')) {
      const category = to.params.category as string;
      if (category) {
        import('../utils/userActionService').then(({ default: userActionService }) => {
          userActionService.logAction(
            'BrowseCategory', 
            `浏览"${category}"分类`, 
            'Category', 
            undefined
          ).catch(err => console.error('记录分类浏览行为失败:', err));
        }).catch(err => console.error('导入服务失败:', err));
      }
    }
  }
  */
})

export default router 

// 添加安全相关的辅助函数
function getDeviceType(): string {
  const userAgent = navigator.userAgent
  if (/mobile/i.test(userAgent)) return 'mobile'
  if (/tablet/i.test(userAgent)) return 'tablet'
  return 'desktop'
}

async function logout() {
  const { logout } = await import('../utils/auth')
  await logout()
} 