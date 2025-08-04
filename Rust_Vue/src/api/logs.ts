import { api, ApiResponse } from '../utils/apiClient'

// 日志相关类型
export interface LogEntry {
  id: string
  level: string
  message: string
  timestamp: string
  source: string
  details?: Record<string, any>
}

export interface LogQueryParams {
  page?: number
  pageSize?: number
  level?: string
}

export interface LogListResponse {
  list: LogEntry[]
  total: number
  page: number
  page_size: number
}

// 日志API
export const logsApi = {
  // 获取日志列表
  getLogs: (params?: LogQueryParams): Promise<ApiResponse<LogListResponse>> => {
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())  // 使用page_size
    if (params?.level) queryParams.append('level', params.level)

    return api.get(`/v1/admin/logs?${queryParams.toString()}`)
  },

  // 清除日志
  clearLogs: (level?: string): Promise<ApiResponse<null>> => {
    const data = level ? { level } : {}
    return api.post('/v1/admin/logs/clear', data)
  },

  // 删除日志
  deleteLog: (id: string): Promise<ApiResponse<null>> => {
    return api.delete(`/v1/admin/logs/${id}`)
  },

  // 获取日志统计
  getLogStats: (): Promise<ApiResponse<{
    total: number
    by_level: Record<string, number>
    recent_errors: number
  }>> => {
    return api.get('/v1/admin/logs/stats')
  }
} 