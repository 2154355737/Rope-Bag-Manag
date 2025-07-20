import { api, ApiResponse } from '../utils/apiClient'

// 系统日志类型
export interface SystemLog {
  id: number
  level: string
  message: string
  timestamp: string
  details?: any
}

// 日志API
export const logsApi = {
  // 获取系统日志
  getLogs: (params?: {
    page?: number
    pageSize?: number
    level?: string
    search?: string
  }): Promise<ApiResponse<{
    list: SystemLog[]
    total: number
    page: number
    page_size: number
  }>> => {
    // 构建查询参数
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())
    if (params?.level) queryParams.append('level', params.level)
    if (params?.search) queryParams.append('search', params.search)

    return api.get(`/api/v1/admin/logs?${queryParams.toString()}`)
  },
  
  // 导出日志（可选实现）
  exportLogs: (params?: {
    level?: string
    search?: string
    start_date?: string
    end_date?: string
  }): Promise<Blob> => {
    // 构建查询参数
    const queryParams = new URLSearchParams()
    if (params?.level) queryParams.append('level', params.level)
    if (params?.search) queryParams.append('search', params.search)
    if (params?.start_date) queryParams.append('start_date', params.start_date)
    if (params?.end_date) queryParams.append('end_date', params.end_date)

    return api.get(`/api/v1/admin/logs/export?${queryParams.toString()}`, {
      responseType: 'blob'
    })
  },
  
  // 清除日志（可选实现）
  clearLogs: (params?: {
    level?: string
    before_date?: string
  }): Promise<ApiResponse<{ deleted_count: number }>> => {
    return api.post('/api/v1/admin/logs/clear', params || {})
  }
} 