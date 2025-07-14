import { api, ApiResponse } from '../utils/apiClient'

export interface Resource {
  id: number
  title?: string // 兼容旧字段
  name?: string  // 新字段
  description?: string
  author: string
  version?: string
  category?: string
  tags?: string[]
  status?: string
  file_url?: string
  cover_url?: string
  created_at?: string
  updated_at?: string
}

export const communityApi = {
  // 获取资源列表
  getResources: (params?: { page?: number; pageSize?: number; category?: string; status?: string; search?: string }): Promise<ApiResponse<{ list: Resource[]; total: number; page: number; pageSize: number; totalPages?: number }>> => {
    return api.get('/api/v1/packages', { params })
  },
  // 获取资源详情
  getResource: (id: number): Promise<ApiResponse<Resource>> => {
    return api.get(`/api/v1/packages/${id}`)
  },
  // 下载资源
  downloadResource: (id: number): Promise<ApiResponse<{ url: string }>> => {
    return api.get(`/api/v1/packages/${id}/download`)
  },
  // 创建资源
  createResource: (data: any): Promise<ApiResponse> => {
    return api.post('/api/v1/packages', data)
  },
  // 更新资源
  updateResource: (id: number, data: any): Promise<ApiResponse> => {
    return api.put(`/api/v1/packages/${id}`, data)
  },
  // 删除资源
  deleteResource: (id: number): Promise<ApiResponse> => {
    return api.delete(`/api/v1/packages/${id}`)
  },
  // 批量删除资源
  batchDeleteResources: (ids: number[]): Promise<ApiResponse> => {
    return api.post('/api/v1/packages/batch-delete', { ids })
  },
  // 更新资源状态
  updateResourceStatus: (id: number, status: string): Promise<ApiResponse> => {
    return api.put(`/api/v1/packages/${id}/status`, { status })
  },
  // 批量更新资源状态
  batchUpdateResourceStatus: (ids: number[], status: string): Promise<ApiResponse> => {
    return api.post('/api/v1/packages/batch-update-status', { ids, status })
  },
  // 获取资源统计
  getResourceStats: (): Promise<ApiResponse<{ total: number }>> => {
    return api.get('/api/v1/packages/stats')
  }
}

// 分类管理API（基于后端数据模拟）
export const categoryApi = {
  // 获取分类列表
  getCategories: async (params?: {
    enabled?: boolean
    search?: string
  }) => {
    const db = getDB()
    let categories = []
    let total = 0

    const res = await db.exec('SELECT * FROM category')
    categories = res[0] ? res[0].values : []
    total = categories.length

    // 搜索过滤
    if (params?.search) {
      categories = categories.filter(cat => 
        String(cat[1]).toLowerCase().includes(params.search!.toLowerCase())
      )
    }
    
    // 状态过滤
    if (params?.enabled !== undefined) {
      categories = categories.filter(cat => cat[6] === params.enabled)
    }
    
    return { code: 0, data: categories }
  },

  // 获取分类详情
  getCategory: async (id: number) => {
    const db = getDB()
    const res = await db.exec(`SELECT * FROM category WHERE id = ${id}`)
    if (res[0] && res[0].values.length > 0) {
      return { code: 0, data: res[0].values[0] }
    }
    return { code: 1, msg: '分类不存在' }
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
    const db = getDB()
    const res = await db.exec(`INSERT INTO category (名称, 描述, 图标, 颜色, 排序, 启用, 标签) VALUES ('${data.name}', '${data.description}', '${data.icon}', '${data.color}', ${data.sort}, ${data.enabled ? 1 : 0}, '${data.tags.join(',')}')`)
    return { code: 0, msg: '分类创建成功', data: { id: res[0].lastID, ...data } }
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
    const db = getDB()
    let updateSql = `UPDATE category SET 名称 = '${data.name || ''}'`
    if (data.description) updateSql += `, 描述 = '${data.description}'`
    if (data.icon) updateSql += `, 图标 = '${data.icon}'`
    if (data.color) updateSql += `, 颜色 = '${data.color}'`
    if (data.sort !== undefined) updateSql += `, 排序 = ${data.sort}`
    if (data.enabled !== undefined) updateSql += `, 启用 = ${data.enabled ? 1 : 0}`
    if (data.tags) updateSql += `, 标签 = '${data.tags.join(',')}'`
    updateSql += ` WHERE id = ${id}`
    const res = await db.exec(updateSql)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '分类更新成功', data: { id, ...data } } : { code: 1, msg: '分类不存在' }
  },

  // 删除分类（模拟）
  deleteCategory: async (id: number) => {
    const db = getDB()
    const res = await db.exec(`DELETE FROM category WHERE id = ${id}`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '分类删除成功' } : { code: 1, msg: '分类不存在' }
  },

  // 批量删除分类（模拟）
  batchDeleteCategories: async (ids: number[]) => {
    const db = getDB()
    const placeholders = ids.map(() => '?').join(',')
    const res = await db.exec(`DELETE FROM category WHERE id IN (${placeholders})`)
    return { code: 0, msg: '批量删除成功' }
  },

  // 更新分类状态（模拟）
  updateCategoryStatus: async (id: number, enabled: boolean) => {
    const db = getDB()
    const res = await db.exec(`UPDATE category SET 启用 = ${enabled ? 1 : 0} WHERE id = ${id}`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '状态更新成功' } : { code: 1, msg: '分类不存在' }
  },

  // 批量更新分类状态（模拟）
  batchUpdateCategoryStatus: async (ids: number[], enabled: boolean) => {
    const db = getDB()
    const placeholders = ids.map(() => '?').join(',')
    const res = await db.exec(`UPDATE category SET 启用 = ${enabled ? 1 : 0} WHERE id IN (${placeholders})`)
    return { code: 0, msg: '批量状态更新成功' }
  },

  // 更新分类排序（模拟）
  updateCategorySort: async (id: number, sort: number) => {
    const db = getDB()
    const res = await db.exec(`UPDATE category SET 排序 = ${sort} WHERE id = ${id}`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '排序更新成功' } : { code: 1, msg: '分类不存在' }
  },

  // 获取分类统计
  getCategoryStats: async () => {
    const db = getDB()
    const res = await db.exec('SELECT COUNT(*) FROM category')
    const total = res[0] ? res[0].values[0][0] : 0
    return { code: 0, data: { total } }
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
    const db = getDB()
    let users = []
    let total = 0

    const res = await db.exec('SELECT * FROM user')
    users = res[0] ? res[0].values : []
    total = users.length

    // 搜索过滤
    if (params.search) {
      users = users.filter(user => 
        String(user[1]).toLowerCase().includes(params.search!.toLowerCase()) ||
        String(user[2]).toLowerCase().includes(params.search!.toLowerCase())
      )
    }
    
    // 角色过滤
    if (params.role) {
      users = users.filter(user => {
        if (params.role === 'admin') return user[5] // is_admin
        if (params.role === 'user') return !user[5] // is_admin
        return true
      })
    }
    
    // 状态过滤
    if (params.status) {
      users = users.filter(user => {
        if (params.status === 'banned') return user[6] // banned
        if (params.status === 'active') return !user[6] // banned
        return true
      })
    }
    
    // 分页处理
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
  },

  // 获取用户详情
  getUser: async (id: number) => {
    const db = getDB()
    const res = await db.exec(`SELECT * FROM user WHERE id = ${id}`)
    if (res[0] && res[0].values.length > 0) {
      return { code: 0, data: res[0].values[0] }
    }
    return { code: 1, msg: '用户不存在' }
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
    const db = getDB()
    const res = await db.exec(`INSERT INTO user (用户名, 邮箱, 密码, 角色, 状态, 简介, 头像URL) VALUES ('${data.username}', '${data.email}', '${data.password}', '${data.role}', '${data.status}', '${data.bio || ''}', '${data.avatar ? URL.createObjectURL(data.avatar) : ''}')`)
    return { code: 0, msg: '用户创建成功', data: { id: res[0].lastID, ...data } }
  },

  // 更新用户
  updateUser: async (id: number, data: {
    nickname?: string
    password?: string
    avatar?: File
  }) => {
    const username = getCurrentUsername()
    const db = getDB()
    let updateSql = `UPDATE user SET 用户名 = '${data.nickname || username}'`
    if (data.password) updateSql += `, 密码 = '${data.password}'`
    if (data.avatar) updateSql += `, 头像URL = '${URL.createObjectURL(data.avatar)}'`
    updateSql += ` WHERE id = ${id}`
    const res = await db.exec(updateSql)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '用户更新成功' } : { code: 1, msg: '用户不存在' }
  },

  // 删除用户（模拟）
  deleteUser: async (id: number) => {
    const username = getCurrentUsername()
    const db = getDB()
    const res = await db.exec(`DELETE FROM user WHERE id = ${id} AND 用户名 = '${username}'`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '用户删除成功' } : { code: 1, msg: '用户不存在或无权限' }
  },

  // 批量删除用户（模拟）
  batchDeleteUsers: async (ids: number[]) => {
    const username = getCurrentUsername()
    const db = getDB()
    const placeholders = ids.map(() => '?').join(',')
    const res = await db.exec(`DELETE FROM user WHERE id IN (${placeholders}) AND 用户名 = '${username}'`, ids)
    return { code: 0, msg: '批量删除成功' }
  },

  // 更新用户状态
  updateUserStatus: async (id: number, status: string) => {
    const username = getCurrentUsername()
    const db = getDB()
    const res = await db.exec(`UPDATE user SET 状态 = '${status}' WHERE id = ${id} AND 用户名 = '${username}'`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '状态更新成功' } : { code: 1, msg: '用户不存在或无权限' }
  },

  // 批量更新用户状态
  batchUpdateUserStatus: async (ids: number[], status: string) => {
    const username = getCurrentUsername()
    const db = getDB()
    const placeholders = ids.map(() => '?').join(',')
    const res = await db.exec(`UPDATE user SET 状态 = '${status}' WHERE id IN (${placeholders}) AND 用户名 = '${username}'`, ids)
    return { code: 0, data: res.map(r => ({ id: r.changes, result: r.changes > 0 ? 'success' : 'failed' })) }
  },

  // 更新用户角色
  updateUserRole: async (id: number, role: string) => {
    const username = getCurrentUsername()
    const db = getDB()
    const res = await db.exec(`UPDATE user SET 角色 = '${role}' WHERE id = ${id} AND 用户名 = '${username}'`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '角色更新成功' } : { code: 1, msg: '用户不存在或无权限' }
  },

  // 更新用户星级
  updateUserStar: async (id: number, star: number) => {
    const username = getCurrentUsername()
    const db = getDB()
    const res = await db.exec(`UPDATE user SET 星级 = ${star} WHERE id = ${id} AND 用户名 = '${username}'`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '星级更新成功' } : { code: 1, msg: '用户不存在或无权限' }
  },

  // 获取用户统计
  getUserStats: async () => {
    const db = getDB()
    const res = await db.exec('SELECT COUNT(*) FROM user')
    const total = res[0] ? res[0].values[0][0] : 0
    return { code: 0, data: { total } }
  }
}

// 社区统计API
export const communityStatsApi = {
  // 获取社区总览统计
  getOverviewStats: async () => {
    const db = getDB()
    const res = await db.exec('SELECT COUNT(*) FROM resource, user')
    const totalResources = res[0] ? res[0].values[0][0] : 0
    const totalUsers = res[1] ? res[1].values[0][0] : 0
    return { code: 0, data: { totalResources, totalUsers } }
  },

  // 获取资源统计
  getResourceStats: async (params?: {
    period?: string
    category?: string
  }) => {
    const db = getDB()
    let sql = 'SELECT COUNT(*) FROM resource'
    if (params?.category) {
      sql += ` WHERE 分类 = '${params.category}'`
    }
    const res = await db.exec(sql)
    const total = res[0] ? res[0].values[0][0] : 0
    return { code: 0, data: { total } }
  },

  // 获取用户统计
  getUserStats: async (params?: {
    period?: string
    role?: string
  }) => {
    const db = getDB()
    let sql = 'SELECT COUNT(*) FROM user'
    if (params?.role) {
      sql += ` WHERE 角色 = '${params.role}'`
    }
    const res = await db.exec(sql)
    const total = res[0] ? res[0].values[0][0] : 0
    return { code: 0, data: { total } }
  },

  // 获取分类统计
  getCategoryStats: async (params?: {
    period?: string
  }) => {
    const db = getDB()
    const res = await db.exec('SELECT COUNT(*) FROM category')
    const total = res[0] ? res[0].values[0][0] : 0
    return { code: 0, data: { total } }
  },

  // 获取热门资源
  getHotResources: async (params?: {
    period?: string
    limit?: number
  }) => {
    const db = getDB()
    let sql = 'SELECT * FROM resource ORDER BY 下载次数 DESC'
    if (params?.limit) {
      sql += ` LIMIT ${params.limit}`
    }
    const res = await db.exec(sql)
    return { code: 0, data: res[0] ? res[0].values : [] }
  },

  // 获取活跃用户
  getActiveUsers: async (params?: {
    period?: string
    limit?: number
  }) => {
    const db = getDB()
    let sql = 'SELECT * FROM user ORDER BY 活跃度 DESC'
    if (params?.limit) {
      sql += ` LIMIT ${params.limit}`
    }
    const res = await db.exec(sql)
    return { code: 0, data: res[0] ? res[0].values : [] }
  }
}

// 社区内容审核API
export const communityModerationApi = {
  // 获取待审核内容
  getPendingContent: async (params: {
    page?: number
    pageSize?: number
    type?: 'resource' | 'comment' | 'user'
  }) => {
    const db = getDB()
    let sql = 'SELECT * FROM moderation_queue'
    if (params.type) {
      sql += ` WHERE 类型 = '${params.type}'`
    }
    const res = await db.exec(sql)
    return { code: 0, data: res[0] ? res[0].values : [] }
  },

  // 审核资源
  reviewResource: async (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    const db = getDB()
    const res = await db.exec(`UPDATE moderation_queue SET 状态 = '${data.action}', 审核理由 = '${data.reason || ''}' WHERE id = ${id}`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '资源审核成功' } : { code: 1, msg: '审核不存在' }
  },

  // 审核评论
  reviewComment: async (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    const db = getDB()
    const res = await db.exec(`UPDATE moderation_queue SET 状态 = '${data.action}', 审核理由 = '${data.reason || ''}' WHERE id = ${id}`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '评论审核成功' } : { code: 1, msg: '审核不存在' }
  },

  // 审核用户
  reviewUser: async (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    const db = getDB()
    const res = await db.exec(`UPDATE moderation_queue SET 状态 = '${data.action}', 审核理由 = '${data.reason || ''}' WHERE id = ${id}`)
    return res[0] && res[0].changes > 0 ? { code: 0, msg: '用户审核成功' } : { code: 1, msg: '审核不存在' }
  },

  // 批量审核
  batchReview: async (data: {
    ids: number[]
    type: 'resource' | 'comment' | 'user'
    action: 'approve' | 'reject'
    reason?: string
  }) => {
    const db = getDB()
    const placeholders = data.ids.map(() => '?').join(',')
    const res = await db.exec(`UPDATE moderation_queue SET 状态 = '${data.action}', 审核理由 = '${data.reason || ''}' WHERE id IN (${placeholders}) AND 类型 = '${data.type}'`)
    return { code: 0, data: res.map(r => ({ id: r.changes, result: r.changes > 0 ? 'success' : 'failed' })) }
  },

  // 获取审核统计
  getModerationStats: async () => {
    const db = getDB()
    const res = await db.exec('SELECT COUNT(*) FROM moderation_queue')
    const total = res[0] ? res[0].values[0][0] : 0
    return { code: 0, data: { total } }
  }
}

export default {
  communityApi,
  categoryApi,
  communityUserApi,
  communityStatsApi,
  communityModerationApi
} 