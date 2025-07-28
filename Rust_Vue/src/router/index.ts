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

// 全局前置守卫
router.beforeEach((to, from, next) => {
  // 页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} - 绳包管理系统`
  } else {
    document.title = '绳包管理系统'
  }
  
  // 检查页面权限
  const authStatus = checkAuthStatus()
  
  // 先记录访问日志
  logRouteNavigation(to)
  
  // 页面权限检查
  const requiredRole = to.meta.requiresRole
  const userRole = localStorage.getItem('userRole') || 'guest'
  
  if (requiredRole && requiredRole !== userRole) {
    next({ path: '/forbidden', replace: true })
    return
  }
  
  // 检查登录状态
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth)
  const isLoggedIn = localStorage.getItem('token')
  
  if (requiresAuth && !isLoggedIn) {
    next({ path: '/login', query: { redirect: to.fullPath } })
    return
  }
  
  next()
})

// 全局后置钩子 - 添加详细访问记录
router.afterEach((to, from) => {
  // 如果用户已登录，记录更详细的页面访问信息
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
})

export default router 