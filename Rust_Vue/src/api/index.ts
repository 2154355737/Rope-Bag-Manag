import { initDB, getAllUsers, getStats } from '../utils/sqlite'

// 用户相关API
export const getUsers = async () => {
  await initDB()
  return getAllUsers()
}

// 用户API对象
export const userApi = {
  // 获取用户列表
  getUsers: async (params?: {
    page?: number
    pageSize?: number
    role?: string
    status?: string
    search?: string
  }) => {
    await initDB()
    const users = getAllUsers()
    
    // 模拟分页和过滤
    let filteredUsers = users
    const page = params?.page || 1
    const pageSize = params?.pageSize || 20
    const start = (page - 1) * pageSize
    const end = start + pageSize
    
    return {
      code: 0,
      message: 'success',
      data: {
        list: filteredUsers.slice(start, end),
        total: filteredUsers.length,
        page,
        pageSize,
        totalPages: Math.ceil(filteredUsers.length / pageSize)
      }
    }
  },

  // 获取用户信息
  getUser: async (username: string) => {
    await initDB()
    // 模拟用户数据
    return {
      code: 0,
      message: 'success',
      data: {
        username,
        nickname: username,
        role: 'admin',
        star: 5,
        banned: false,
        create_time: new Date().toISOString(),
        last_login_time: new Date().toISOString()
      }
    }
  },

  // 用户登录
  login: async (username: string, password: string) => {
    await initDB()
    // 模拟登录验证
    if (username === 'admin' && password === 'admin') {
      return {
        code: 0,
        message: '登录成功',
        data: {
          user: {
            username,
            nickname: username,
            role: 'admin',
            star: 5,
            banned: false
          },
          token: 'mock-token-' + Date.now()
        }
      }
    }
    return {
      code: 1,
      message: '用户名或密码错误'
    }
  }
}

// 统计API对象
export const statsApi = {
  // 获取统计数据
  getStats: async () => {
    await initDB()
    const stats = getStats()
    
    return {
      code: 0,
      message: 'success',
      data: {
        total_users: stats.total_users,
        total_packages: stats.total_packages,
        total_comments: stats.total_comments,
        active_users: stats.active_users,
        new_users_today: stats.new_users_today,
        new_packages_today: stats.new_packages_today,
        system_status: stats.system_status,
        uptime: stats.uptime
      }
    }
  }
}

// 获取最近活动
export const getRecentActivities = async () => {
  // 模拟最近活动数据
  return {
    code: 0,
    message: 'success',
    data: {
      activities: [
        {
          id: 1,
          type: 'user_login',
          user: 'admin',
          message: '管理员登录系统',
          timestamp: new Date().toISOString(),
          details: { ip: '192.168.1.1' }
        },
        {
          id: 2,
          type: 'package_upload',
          user: 'user1',
          message: '上传了新绳包资源',
          timestamp: new Date(Date.now() - 3600000).toISOString(),
          details: { package_name: '示例绳包' }
        },
        {
          id: 3,
          type: 'comment_add',
          user: 'user2',
          message: '发表了新评论',
          timestamp: new Date(Date.now() - 7200000).toISOString(),
          details: { resource_id: 1 }
        }
      ]
    }
  }
}

// 菜单相关API
export const getMenuItems = async () => {
  // 模拟菜单数据
  return {
    code: 0,
    message: 'success',
    data: {
      menu_items: [
        { path: '/admin', title: '仪表盘', icon: 'House', badge: 0 },
        { path: '/admin/users', title: '用户管理', icon: 'User', badge: 0 },
        { path: '/admin/packages', title: '资源管理', icon: 'Box', badge: 0 },
        { path: '/admin/comments', title: '评论管理', icon: 'ChatDotRound', badge: 0 },
        { path: '/admin/user-actions', title: '行为记录', icon: 'Monitor', badge: 0 },
        { path: '/admin/resource-records', title: '资源记录', icon: 'Files', badge: 0 },
        { path: '/admin/backup', title: '数据备份', icon: 'Download', badge: 0 },
        { path: '/admin/announcements', title: '公告管理', icon: 'Bell', badge: 0 },
        { path: '/admin/logs', title: '日志查看', icon: 'Document', badge: 0 },
        { path: '/admin/stats', title: '统计信息', icon: 'DataAnalysis', badge: 0 },
        { path: '/admin/theme-settings', title: '系统设置', icon: 'Setting', badge: 0 }
      ]
    }
  }
}

// 其他API同理，全部用sqlite.ts的模拟数据API实现 