import { api, ApiResponse } from '../utils/apiClient'

// 统计数据类型
export interface Stats {
  total_users: number
  total_packages: number
  total_comments: number
  active_users: number
  new_users_today: number
  new_packages_today: number
  system_status: string
  uptime: number
}

// 系统日志类型
export interface SystemLog {
  id: number
  level: string
  message: string
  timestamp: string
  details?: any
}

// 用户行为记录类型
export interface UserAction {
  id: number
  user_id: number
  username: string
  action: string
  details?: string
  ip_address?: string
  user_agent?: string
  created_at: string
}

// 管理员API
export const adminApi = {
  // 获取统计数据
  getStats: (): Promise<ApiResponse<Stats>> => {
    return api.get('/v1/admin/stats')
  },

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
    pageSize: number
  }>> => {
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())
    if (params?.level) queryParams.append('level', params.level)
    if (params?.search) queryParams.append('search', params.search)

    return api.get(`/v1/admin/logs?${queryParams.toString()}`)
  },

  // 获取用户行为记录
  getUserActions: (params?: {
    page?: number
    pageSize?: number
    user_id?: number
    action?: string
    search?: string
  }): Promise<ApiResponse<{
    list: UserAction[]
    total: number
    page: number
    pageSize: number
  }>> => {
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())
    if (params?.user_id) queryParams.append('user_id', params.user_id.toString())
    if (params?.action) queryParams.append('action', params.action)
    if (params?.search) queryParams.append('search', params.search)

    return api.get(`/v1/admin/user-actions?${queryParams.toString()}`)
  },

  // 创建数据备份
  createBackup: (): Promise<ApiResponse<{
    backup_id: string
    file_path: string
    size: number
    created_at: string
  }>> => {
    return api.post('/v1/admin/backup')
  },

  // 获取备份列表
  getBackups: (): Promise<ApiResponse<{
    list: Array<{
      backup_id: string
      file_path: string
      size: number
      created_at: string
    }>
  }>> => {
    return api.get('/v1/admin/backups')
  },

  // 下载备份文件
  downloadBackup: (backupId: string): Promise<ApiResponse<string>> => {
    return api.get(`/v1/admin/backup/${backupId}/download`)
  },

  // 删除备份
  deleteBackup: (backupId: string): Promise<ApiResponse<null>> => {
    return api.delete(`/v1/admin/backup/${backupId}`)
  },

  // 系统健康检查
  healthCheck: (): Promise<ApiResponse<{
    status: string
    message: string
    timestamp: string
    services: {
      database: string
      file_system: string
      memory: string
    }
  }>> => {
    return api.get('/health')
  },

  // 获取邮件SMTP设置
  getMailSettings: (): Promise<ApiResponse<any>> => {
    return api.get('/v1/admin/mail-settings')
  },

  // 更新邮件SMTP设置
  updateMailSettings: (config: Record<string, any>): Promise<ApiResponse<null>> => {
    return api.post('/v1/admin/mail-settings', config)
  },

  // 发送测试邮件
  sendTestEmail: (email: string): Promise<ApiResponse<null>> => {
    return api.post('/v1/admin/test-email', { email })
  },
} 