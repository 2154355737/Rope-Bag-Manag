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
  // 获取统计信息
  getStats: (): Promise<ApiResponse<Stats>> => {
    return api.get('/v1/admin/stats')
  },

  // 获取用户统计数据
  getUserStats: (): Promise<ApiResponse<{
    total_users: number
    active_users: number
    total_actions: number
  }>> => {
    return api.get('/v1/admin/user-stats')
  },

  // 获取系统日志
  getLogs: (params?: { page?: number; pageSize?: number; level?: string }): Promise<ApiResponse<{ logs: SystemLog[]; total: number; page: number; pageSize: number }>> => {
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())  // 使用page_size
    if (params?.level) queryParams.append('level', params.level)

    return api.get(`/v1/admin/logs?${queryParams.toString()}`)
  },

  // 获取用户操作记录
  getUserActions: (params?: { page?: number; pageSize?: number; userId?: number; action?: string }): Promise<ApiResponse<{ actions: UserAction[]; total: number; page: number; pageSize: number }>> => {
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())  // 使用page_size
    if (params?.userId) queryParams.append('user_id', params.userId.toString())
    if (params?.action) queryParams.append('action', params.action)

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

  // 获取下载安全统计
  getDownloadSecurityStats: (): Promise<ApiResponse<any>> => {
    return api.get('/v1/download-security/stats')
  },

  // 获取下载安全配置
  getDownloadSecurityConfig: (): Promise<ApiResponse<any>> => {
    return api.get('/v1/download-security/config')
  },

  // 更新下载安全配置
  updateDownloadSecurityConfig: (config: any): Promise<ApiResponse<any>> => {
    return api.put('/v1/download-security/config', config)
  },

  // 获取异常记录
  getDownloadAnomalies: (params?: { hours?: number }): Promise<ApiResponse<any>> => {
    const queryParams = new URLSearchParams()
    if (params?.hours) queryParams.append('hours', params.hours.toString())
    
    const queryString = queryParams.toString()
    const url = queryString ? `/v1/download-security/anomalies?${queryString}` : '/v1/download-security/anomalies'
    
    return api.get(url)
  },

  // IP封禁管理相关API
  getBanStats: (): Promise<ApiResponse<any>> => {
    return api.get('/v1/security-management/ban-stats')
  },

  getIpBans: (): Promise<ApiResponse<any>> => {
    return api.get('/v1/security-management/ip-bans')
  },

  banIp: (data: any): Promise<ApiResponse<any>> => {
    return api.post('/v1/security-management/ip-bans', data)
  },

  unbanIp: (data: any): Promise<ApiResponse<any>> => {
    return api.delete('/v1/security-management/ip-bans', { data })
  },

  deleteIpBan: (banId: number): Promise<ApiResponse<any>> => {
    return api.delete(`/v1/security-management/ip-bans/${banId}`)
  },

  getIpWhitelist: (): Promise<ApiResponse<any>> => {
    return api.get('/v1/security-management/ip-whitelist')
  },

  addIpToWhitelist: (data: any): Promise<ApiResponse<any>> => {
    return api.post('/v1/security-management/ip-whitelist', data)
  },

  removeIpFromWhitelist: (data: any): Promise<ApiResponse<any>> => {
    return api.delete('/v1/security-management/ip-whitelist', { data })
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