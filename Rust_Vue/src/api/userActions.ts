import { 
  initDB, 
  query, 
  execute 
} from '@/utils/sqlite'
import type { ApiResponse, PaginatedResponse } from './types'

// 获取当前用户名的辅助函数
function getCurrentUsername(): string {
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

// 用户行为记录管理API
export const userActionApi = {
  // 获取用户行为记录
  getUserActions: async (params?: {
    page?: number
    pageSize?: number
    user_id?: number
    action?: string
    start_date?: string
    end_date?: string
    search?: string
  }): Promise<ApiResponse<PaginatedResponse<any>>> => {
    await initDB()
    
    let whereConditions: string[] = []
    let queryParams: any[] = []
    
    if (params?.user_id) {
      whereConditions.push('al.user_id = ?')
      queryParams.push(params.user_id)
    }
    
    if (params?.action) {
      whereConditions.push('al.action = ?')
      queryParams.push(params.action)
    }
    
    if (params?.start_date) {
      whereConditions.push('al.created_at >= ?')
      queryParams.push(params.start_date)
    }
    
    if (params?.end_date) {
      whereConditions.push('al.created_at <= ?')
      queryParams.push(params.end_date)
    }
    
    const whereClause = whereConditions.length > 0 ? `WHERE ${whereConditions.join(' AND ')}` : ''
    const baseSql = `
      SELECT al.*, u.nickname as user_name 
      FROM action_log al 
      LEFT JOIN user u ON al.user_id = u.id 
      ${whereClause}
      ORDER BY al.created_at DESC
    `
    
    const { data, total } = await import('@/utils/sqlite').then(m => m.paginateQuery(
      baseSql, 
      params?.page || 1, 
      params?.pageSize || 10, 
      queryParams
    ))
    
    // 转换为标准格式
    const actionList = data.map(action => ({
      id: action[0],
      user_id: action[1],
      action: action[2],
      detail: action[3],
      created_at: action[4],
      user_name: action[5] || '未知用户'
    }))
    
    return {
      code: 0,
      msg: '获取用户行为记录成功',
      data: {
        list: actionList,
        total,
        page: params?.page || 1,
        size: params?.pageSize || 10
      }
    }
  },

  // 记录用户行为
  logUserAction: async (action: string, detail?: string): Promise<ApiResponse> => {
    await initDB()
    
    const username = getCurrentUsername()
    const user = await import('@/utils/sqlite').then(m => m.getUserByUsername(username))
    
    if (!user) {
      return {
        code: 1,
        msg: '用户不存在'
      }
    }
    
    const sql = `INSERT INTO action_log (user_id, action, detail) VALUES (?, ?, ?)`
    const result = execute(sql, [user[0], action, detail || ''])
    
    if (result > 0) {
      return {
        code: 0,
        msg: '行为记录成功'
      }
    } else {
      return {
        code: 1,
        msg: '行为记录失败'
      }
    }
  },

  // 删除用户行为记录
  deleteUserAction: async (actionId: number): Promise<ApiResponse> => {
    await initDB()
    
    const result = execute('DELETE FROM action_log WHERE id = ?', [actionId])
    
    if (result > 0) {
      return {
        code: 0,
        msg: '行为记录删除成功'
      }
    } else {
      return {
        code: 1,
        msg: '行为记录删除失败'
      }
    }
  },

  // 批量删除用户行为记录
  batchDeleteUserActions: async (actionIds: number[]): Promise<ApiResponse> => {
    await initDB()
    
    const placeholders = actionIds.map(() => '?').join(',')
    const result = execute(`DELETE FROM action_log WHERE id IN (${placeholders})`, actionIds)
    
    if (result > 0) {
      return {
        code: 0,
        msg: '批量删除行为记录成功'
      }
    } else {
      return {
        code: 1,
        msg: '批量删除行为记录失败'
      }
    }
  },

  // 清空用户行为记录
  clearUserActions: async (params?: {
    user_id?: number
    before_date?: string
  }): Promise<ApiResponse> => {
    await initDB()
    
    let whereConditions: string[] = []
    let queryParams: any[] = []
    
    if (params?.user_id) {
      whereConditions.push('user_id = ?')
      queryParams.push(params.user_id)
    }
    
    if (params?.before_date) {
      whereConditions.push('created_at < ?')
      queryParams.push(params.before_date)
    }
    
    const whereClause = whereConditions.length > 0 ? `WHERE ${whereConditions.join(' AND ')}` : ''
    const sql = `DELETE FROM action_log ${whereClause}`
    
    const result = execute(sql, queryParams)
    
    return {
      code: 0,
      msg: `清空行为记录成功，共删除 ${result} 条记录`
    }
  },

  // 获取行为统计
  getActionStats: async (params?: {
    user_id?: number
    start_date?: string
    end_date?: string
  }): Promise<ApiResponse<any>> => {
    await initDB()
    
    let whereConditions: string[] = []
    let queryParams: any[] = []
    
    if (params?.user_id) {
      whereConditions.push('user_id = ?')
      queryParams.push(params.user_id)
    }
    
    if (params?.start_date) {
      whereConditions.push('created_at >= ?')
      queryParams.push(params.start_date)
    }
    
    if (params?.end_date) {
      whereConditions.push('created_at <= ?')
      queryParams.push(params.end_date)
    }
    
    const whereClause = whereConditions.length > 0 ? `WHERE ${whereConditions.join(' AND ')}` : ''
    const sql = `
      SELECT action, COUNT(*) as count 
      FROM action_log 
      ${whereClause}
      GROUP BY action 
      ORDER BY count DESC
    `
    
    const stats = query(sql, queryParams)
    
    return {
      code: 0,
      msg: '获取行为统计成功',
      data: stats.map(stat => ({
        action: stat[0],
        count: stat[1]
      }))
    }
  }
} 