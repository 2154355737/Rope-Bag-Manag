import { 
  getPackages, 
  addPackage, 
  updatePackage, 
  deletePackage, 
  downloadPackage,
  getUsers,
  adminSetUser,
  adminSetStar,
  adminBanUser,
  getStats
} from './index'

// 获取当前用户名的辅助函数
function getCurrentUsername() {
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin'
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  return username
}

// 社区资源管理API
export const communityApi = {
  // 获取资源列表
  getResources: async (params: {
    page?: number
    pageSize?: number
    category?: string
    status?: string
    search?: string
  }) => {
    const res = await getPackages()
    if (res.code === 0 && res.data) {
      let resources = res.data.绳包列表 || []
      
      // 搜索过滤
      if (params.search) {
        resources = resources.filter((item: any) => 
          item.绳包名称.toLowerCase().includes(params.search!.toLowerCase()) ||
          item.简介.toLowerCase().includes(params.search!.toLowerCase()) ||
          item.作者.toLowerCase().includes(params.search!.toLowerCase())
        )
      }
      
      // 分类过滤
      if (params.category && params.category !== 'all') {
        // 这里可以根据实际需求实现分类过滤
        // 目前后端没有分类字段，可以基于作者或其他字段进行过滤
      }
      
      // 分页处理
      const total = resources.length
      const page = params.page || 1
      const pageSize = params.pageSize || 12
      const start = (page - 1) * pageSize
      const end = start + pageSize
      const paginatedResources = resources.slice(start, end)
      
      return {
        code: 0,
        data: {
          resources: paginatedResources,
          total,
          page,
          pageSize,
          totalPages: Math.ceil(total / pageSize)
        }
      }
    }
    return res
  },

  // 获取资源详情
  getResource: async (id: number) => {
    const res = await getPackages()
    if (res.code === 0 && res.data) {
      const resource = res.data.绳包列表.find((item: any) => item.id === id)
      if (resource) {
        return { code: 0, data: resource }
      }
    }
    return { code: 1, msg: '资源不存在' }
  },

  // 下载资源
  downloadResource: async (id: number) => {
    return await downloadPackage(id)
  },

  // 创建资源
  createResource: async (data: {
    title: string
    description: string
    category: string
    tags: string[]
    status: string
    file?: File
    cover?: File
  }) => {
    const username = getCurrentUsername()
    return await addPackage({
      name: data.title,
      author: username,
      version: '1.0.0',
      desc: data.description,
      url: data.file ? URL.createObjectURL(data.file) : '',
      username
    })
  },

  // 更新资源
  updateResource: async (id: number, data: {
    title?: string
    description?: string
    category?: string
    tags?: string[]
    status?: string
    file?: File
    cover?: File
  }) => {
    const username = getCurrentUsername()
    return await updatePackage({
      id,
      name: data.title || '',
      author: username,
      version: '1.0.0',
      desc: data.description || '',
      url: data.file ? URL.createObjectURL(data.file) : '',
      username
    })
  },

  // 删除资源
  deleteResource: async (id: number) => {
    const username = getCurrentUsername()
    return await deletePackage(id, username)
  },

  // 批量删除资源
  batchDeleteResources: async (ids: number[]) => {
    const results = []
    for (const id of ids) {
      const result = await deleteResource(id)
      results.push({ id, result })
    }
    return { code: 0, data: results }
  },

  // 更新资源状态
  updateResourceStatus: async (id: number, status: string) => {
    // 后端暂不支持状态更新，这里可以扩展
    return { code: 0, msg: '状态更新成功' }
  },

  // 批量更新资源状态
  batchUpdateResourceStatus: async (ids: number[], status: string) => {
    // 后端暂不支持状态更新，这里可以扩展
    return { code: 0, msg: '批量状态更新成功' }
  },

  // 获取资源统计
  getResourceStats: async () => {
    const res = await getStats()
    return res
  }
}

// 分类管理API（基于后端数据模拟）
export const categoryApi = {
  // 获取分类列表
  getCategories: async (params?: {
    enabled?: boolean
    search?: string
  }) => {
    // 从资源数据中提取分类信息
    const res = await getPackages()
    if (res.code === 0 && res.data) {
      const resources = res.data.绳包列表 || []
      const categories = new Map()
      
      // 基于作者创建分类
      resources.forEach((resource: any) => {
        const author = resource.作者
        if (!categories.has(author)) {
          categories.set(author, {
            id: categories.size + 1,
            name: author,
            description: `${author}的资源`,
            icon: '📦',
            color: '#409EFF',
            sort: categories.size + 1,
            enabled: true,
            tags: [],
            count: 0
          })
        }
        categories.get(author).count++
      })
      
      let categoryList = Array.from(categories.values())
      
      // 搜索过滤
      if (params?.search) {
        categoryList = categoryList.filter(cat => 
          cat.name.toLowerCase().includes(params.search!.toLowerCase())
        )
      }
      
      // 状态过滤
      if (params?.enabled !== undefined) {
        categoryList = categoryList.filter(cat => cat.enabled === params.enabled)
      }
      
      return { code: 0, data: categoryList }
    }
    return res
  },

  // 获取分类详情
  getCategory: async (id: number) => {
    const res = await categoryApi.getCategories()
    if (res.code === 0 && res.data) {
      const category = res.data.find((cat: any) => cat.id === id)
      return category ? { code: 0, data: category } : { code: 1, msg: '分类不存在' }
    }
    return res
  },

  // 创建分类（模拟）
  createCategory: async (data: {
    name: string
    description: string
    icon: string
    color: string
    sort: number
    enabled: boolean
    tags: string[]
  }) => {
    // 这里可以扩展后端支持分类管理
    return { code: 0, msg: '分类创建成功', data }
  },

  // 更新分类（模拟）
  updateCategory: async (id: number, data: {
    name?: string
    description?: string
    icon?: string
    color?: string
    sort?: number
    enabled?: boolean
    tags?: string[]
  }) => {
    // 这里可以扩展后端支持分类管理
    return { code: 0, msg: '分类更新成功', data: { id, ...data } }
  },

  // 删除分类（模拟）
  deleteCategory: async (id: number) => {
    // 这里可以扩展后端支持分类管理
    return { code: 0, msg: '分类删除成功' }
  },

  // 批量删除分类（模拟）
  batchDeleteCategories: async (ids: number[]) => {
    // 这里可以扩展后端支持分类管理
    return { code: 0, msg: '批量删除成功' }
  },

  // 更新分类状态（模拟）
  updateCategoryStatus: async (id: number, enabled: boolean) => {
    // 这里可以扩展后端支持分类管理
    return { code: 0, msg: '状态更新成功' }
  },

  // 批量更新分类状态（模拟）
  batchUpdateCategoryStatus: async (ids: number[], enabled: boolean) => {
    // 这里可以扩展后端支持分类管理
    return { code: 0, msg: '批量状态更新成功' }
  },

  // 更新分类排序（模拟）
  updateCategorySort: async (id: number, sort: number) => {
    // 这里可以扩展后端支持分类管理
    return { code: 0, msg: '排序更新成功' }
  },

  // 获取分类统计
  getCategoryStats: async () => {
    const res = await categoryApi.getCategories()
    if (res.code === 0 && res.data) {
      const stats = {
        total: res.data.length,
        enabled: res.data.filter((cat: any) => cat.enabled).length,
        disabled: res.data.filter((cat: any) => !cat.enabled).length,
        totalResources: res.data.reduce((sum: number, cat: any) => sum + cat.count, 0)
      }
      return { code: 0, data: stats }
    }
    return res
  }
}

// 社区用户管理API
export const communityUserApi = {
  // 获取用户列表
  getUsers: async (params: {
    page?: number
    pageSize?: number
    role?: string
    status?: string
    search?: string
  }) => {
    const res = await getUsers()
    if (res.code === 0 && res.data) {
      let users = Object.entries(res.data).map(([username, user]: [string, any]) => ({
        id: username,
        username,
        ...user
      }))
      
      // 搜索过滤
      if (params.search) {
        users = users.filter(user => 
          user.username.toLowerCase().includes(params.search!.toLowerCase()) ||
          user.nickname.toLowerCase().includes(params.search!.toLowerCase())
        )
      }
      
      // 角色过滤
      if (params.role) {
        users = users.filter(user => {
          if (params.role === 'admin') return user.is_admin
          if (params.role === 'user') return !user.is_admin
          return true
        })
      }
      
      // 状态过滤
      if (params.status) {
        users = users.filter(user => {
          if (params.status === 'banned') return user.banned
          if (params.status === 'active') return !user.banned
          return true
        })
      }
      
      // 分页处理
      const total = users.length
      const page = params.page || 1
      const pageSize = params.pageSize || 20
      const start = (page - 1) * pageSize
      const end = start + pageSize
      const paginatedUsers = users.slice(start, end)
      
      return {
        code: 0,
        data: {
          users: paginatedUsers,
          total,
          page,
          pageSize,
          totalPages: Math.ceil(total / pageSize)
        }
      }
    }
    return res
  },

  // 获取用户详情
  getUser: async (id: number) => {
    const res = await getUsers()
    if (res.code === 0 && res.data) {
      const user = res.data[id]
      return user ? { code: 0, data: { id, ...user } } : { code: 1, msg: '用户不存在' }
    }
    return res
  },

  // 创建用户（模拟）
  createUser: async (data: {
    username: string
    email: string
    password: string
    role: string
    status: string
    bio?: string
    avatar?: File
  }) => {
    // 这里可以扩展后端支持用户注册
    return { code: 0, msg: '用户创建成功', data }
  },

  // 更新用户
  updateUser: async (id: number, data: {
    nickname?: string
    password?: string
    avatar?: File
  }) => {
    const username = getCurrentUsername()
    const admin_username = 'muteduanxing'
    const admin_password = 'ahk12378dx'
    
    const updateData: any = { target: id.toString() }
    
    if (data.nickname) updateData.nickname = data.nickname
    if (data.password) updateData.password = data.password
    
    return await adminSetUser({
      ...updateData,
      admin_username,
      admin_password
    })
  },

  // 删除用户（模拟）
  deleteUser: async (id: number) => {
    // 这里可以扩展后端支持用户删除
    return { code: 0, msg: '用户删除成功' }
  },

  // 批量删除用户（模拟）
  batchDeleteUsers: async (ids: number[]) => {
    // 这里可以扩展后端支持用户删除
    return { code: 0, msg: '批量删除成功' }
  },

  // 更新用户状态
  updateUserStatus: async (id: number, status: string) => {
    const username = getCurrentUsername()
    const admin_username = 'muteduanxing'
    const admin_password = 'ahk12378dx'
    
    if (status === 'banned') {
      return await adminBanUser(id.toString(), true, admin_username, admin_password)
    } else if (status === 'active') {
      return await adminBanUser(id.toString(), false, admin_username, admin_password)
    }
    
    return { code: 1, msg: '无效的状态' }
  },

  // 批量更新用户状态
  batchUpdateUserStatus: async (ids: number[], status: string) => {
    const results = []
    for (const id of ids) {
      const result = await updateUserStatus(id, status)
      results.push({ id, result })
    }
    return { code: 0, data: results }
  },

  // 更新用户角色
  updateUserRole: async (id: number, role: string) => {
    const username = getCurrentUsername()
    const admin_username = 'muteduanxing'
    const admin_password = 'ahk12378dx'
    
    if (role === 'admin') {
      return await setAdmin(id.toString(), true, admin_username, admin_password)
    } else if (role === 'user') {
      return await setAdmin(id.toString(), false, admin_username, admin_password)
    }
    
    return { code: 1, msg: '无效的角色' }
  },

  // 更新用户星级
  updateUserStar: async (id: number, star: number) => {
    const username = getCurrentUsername()
    const admin_username = 'muteduanxing'
    const admin_password = 'ahk12378dx'
    
    return await adminSetStar(id.toString(), star, admin_username, admin_password)
  },

  // 获取用户统计
  getUserStats: async () => {
    const res = await getUsers()
    if (res.code === 0 && res.data) {
      const users = Object.values(res.data)
      const stats = {
        total: users.length,
        admin: users.filter((user: any) => user.is_admin).length,
        user: users.filter((user: any) => !user.is_admin).length,
        banned: users.filter((user: any) => user.banned).length,
        active: users.filter((user: any) => !user.banned).length
      }
      return { code: 0, data: stats }
    }
    return res
  }
}

// 社区统计API
export const communityStatsApi = {
  // 获取社区总览统计
  getOverviewStats: () => {
    return request.get('/api/community/stats/overview')
  },

  // 获取资源统计
  getResourceStats: (params?: {
    period?: string
    category?: string
  }) => {
    return request.get('/api/community/stats/resources', { params })
  },

  // 获取用户统计
  getUserStats: (params?: {
    period?: string
    role?: string
  }) => {
    return request.get('/api/community/stats/users', { params })
  },

  // 获取分类统计
  getCategoryStats: (params?: {
    period?: string
  }) => {
    return request.get('/api/community/stats/categories', { params })
  },

  // 获取热门资源
  getHotResources: (params?: {
    period?: string
    limit?: number
  }) => {
    return request.get('/api/community/stats/hot-resources', { params })
  },

  // 获取活跃用户
  getActiveUsers: (params?: {
    period?: string
    limit?: number
  }) => {
    return request.get('/api/community/stats/active-users', { params })
  }
}

// 社区内容审核API
export const communityModerationApi = {
  // 获取待审核内容
  getPendingContent: (params: {
    page?: number
    pageSize?: number
    type?: 'resource' | 'comment' | 'user'
  }) => {
    return request.get('/api/community/moderation/pending', { params })
  },

  // 审核资源
  reviewResource: (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    return request.post(`/api/community/moderation/resources/${id}/review`, data)
  },

  // 审核评论
  reviewComment: (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    return request.post(`/api/community/moderation/comments/${id}/review`, data)
  },

  // 审核用户
  reviewUser: (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    return request.post(`/api/community/moderation/users/${id}/review`, data)
  },

  // 批量审核
  batchReview: (data: {
    ids: number[]
    type: 'resource' | 'comment' | 'user'
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    return request.post('/api/community/moderation/batch-review', data)
  },

  // 获取审核统计
  getModerationStats: () => {
    return request.get('/api/community/moderation/stats')
  }
}

export default {
  communityApi,
  categoryApi,
  communityUserApi,
  communityStatsApi,
  communityModerationApi
} 