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

// 资源记录管理API
export const resourceRecordApi = {
  // 获取资源记录
  getResourceRecords: async (params?: {
    page?: number
    pageSize?: number
    resource_id?: number
    user_id?: number
    action?: string
    start_date?: string
    end_date?: string
  }): Promise<ApiResponse<PaginatedResponse<any>>> => {
    await initDB()
    
    let whereConditions: string[] = []
    let queryParams: any[] = []
    
    if (params?.resource_id) {
      whereConditions.push('rr.resource_id = ?')
      queryParams.push(params.resource_id)
    }
    
    if (params?.user_id) {
      whereConditions.push('rr.user_id = ?')
      queryParams.push(params.user_id)
    }
    
    if (params?.action) {
      whereConditions.push('rr.action = ?')
      queryParams.push(params.action)
    }
    
    if (params?.start_date) {
      whereConditions.push('rr.created_at >= ?')
      queryParams.push(params.start_date)
    }
    
    if (params?.end_date) {
      whereConditions.push('rr.created_at <= ?')
      queryParams.push(params.end_date)
    }
    
    const whereClause = whereConditions.length > 0 ? `WHERE ${whereConditions.join(' AND ')}` : ''
    const baseSql = `
      SELECT rr.*, u.nickname as user_name, r.title as resource_title 
      FROM resource_record rr 
      LEFT JOIN user u ON rr.user_id = u.id 
      LEFT JOIN resource r ON rr.resource_id = r.id 
      ${whereClause}
      ORDER BY rr.created_at DESC
    `
    
    const { data, total } = await import('@/utils/sqlite').then(m => m.paginateQuery(
      baseSql, 
      params?.page || 1, 
      params?.pageSize || 10, 
      queryParams
    ))
    
    // 转换为标准格式
    const recordList = data.map(record => ({
      id: record[0],
      resource_id: record[1],
      user_id: record[2],
      action: record[3],
      created_at: record[4],
      user_name: record[5] || '未知用户',
      resource_title: record[6] || '未知资源'
    }))
    
    return {
      code: 0,
      msg: '获取资源记录成功',
      data: {
        list: recordList,
        total,
        page: params?.page || 1,
        size: params?.pageSize || 10
      }
    }
  },

  // 记录资源操作
  logResourceAction: async (resourceId: number, action: string): Promise<ApiResponse> => {
    await initDB()
    
    const username = getCurrentUsername()
    const user = await import('@/utils/sqlite').then(m => m.getUserByUsername(username))
    
    if (!user) {
      return {
        code: 1,
        msg: '用户不存在'
      }
    }
    
    const sql = `INSERT INTO resource_record (resource_id, user_id, action) VALUES (?, ?, ?)`
    const result = execute(sql, [resourceId, user[0], action])
    
    if (result > 0) {
      return {
        code: 0,
        msg: '资源操作记录成功'
      }
    } else {
      return {
        code: 1,
        msg: '资源操作记录失败'
      }
    }
  },

  // 删除资源记录
  deleteResourceRecord: async (recordId: number): Promise<ApiResponse> => {
    await initDB()
    
    const result = execute('DELETE FROM resource_record WHERE id = ?', [recordId])
    
    if (result > 0) {
      return {
        code: 0,
        msg: '资源记录删除成功'
      }
    } else {
      return {
        code: 1,
        msg: '资源记录删除失败'
      }
    }
  },

  // 批量删除资源记录
  batchDeleteResourceRecords: async (recordIds: number[]): Promise<ApiResponse> => {
    await initDB()
    
    const placeholders = recordIds.map(() => '?').join(',')
    const result = execute(`DELETE FROM resource_record WHERE id IN (${placeholders})`, recordIds)
    
    if (result > 0) {
      return {
        code: 0,
        msg: '批量删除资源记录成功'
      }
    } else {
      return {
        code: 1,
        msg: '批量删除资源记录失败'
      }
    }
  },

  // 清空资源记录
  clearResourceRecords: async (params?: {
    resource_id?: number
    user_id?: number
    before_date?: string
  }): Promise<ApiResponse> => {
    await initDB()
    
    let whereConditions: string[] = []
    let queryParams: any[] = []
    
    if (params?.resource_id) {
      whereConditions.push('resource_id = ?')
      queryParams.push(params.resource_id)
    }
    
    if (params?.user_id) {
      whereConditions.push('user_id = ?')
      queryParams.push(params.user_id)
    }
    
    if (params?.before_date) {
      whereConditions.push('created_at < ?')
      queryParams.push(params.before_date)
    }
    
    const whereClause = whereConditions.length > 0 ? `WHERE ${whereConditions.join(' AND ')}` : ''
    const sql = `DELETE FROM resource_record ${whereClause}`
    
    const result = execute(sql, queryParams)
    
    return {
      code: 0,
      msg: `清空资源记录成功，共删除 ${result} 条记录`
    }
  },

  // 获取资源操作统计
  getResourceActionStats: async (params?: {
    resource_id?: number
    user_id?: number
    start_date?: string
    end_date?: string
  }): Promise<ApiResponse<any>> => {
    await initDB()
    
    let whereConditions: string[] = []
    let queryParams: any[] = []
    
    if (params?.resource_id) {
      whereConditions.push('resource_id = ?')
      queryParams.push(params.resource_id)
    }
    
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
      FROM resource_record 
      ${whereClause}
      GROUP BY action 
      ORDER BY count DESC
    `
    
    const stats = query(sql, queryParams)
    
    return {
      code: 0,
      msg: '获取资源操作统计成功',
      data: stats.map(stat => ({
        action: stat[0],
        count: stat[1]
      }))
    }
  }
} 