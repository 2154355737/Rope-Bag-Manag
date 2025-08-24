import { defHttp } from '/@/utils/http/axios'

// 绳包管理器Dashboard API接口

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

export interface UserStats {
  user_id: number
  username: string
  package_count: number
  comment_count: number
  last_active: string
  role: string
}

export interface Category {
  id: number
  name: string
  description?: string
  enabled: boolean
  subscription_locked: boolean
  created_at: string
  updated_at?: string
}

export interface BackupInfo {
  id: string
  filename: string
  file_path: string
  file_size: number
  backup_type: string
  status: string
  description?: string
  backup_time: string
  created_by?: number
  created_by_name?: string
}

export interface SystemLog {
  id: number
  level: string
  message: string
  timestamp: string
  details?: string
}

export interface HealthStatus {
  status: string
  message: string
  timestamp: string
}

// API接口定义
const Api = {
  GetStats: '/api/v1/admin/stats',
  GetUserRegistrationTrend: '/api/v1/admin/user-registration-trend',
  GetUserStats: '/api/v1/admin/user-stats',
  GetCategories: '/api/v1/categories',
  CreateCategory: '/api/v1/categories',
  UpdateCategory: (id: number) => `/api/v1/categories/${id}`,
  DeleteCategory: (id: number) => `/api/v1/categories/${id}`,
  GetLogs: '/api/v1/admin/logs',
  GetBackups: '/api/v1/admin/backups',
  CreateBackup: '/api/v1/admin/backup',
  DeleteBackup: (id: string) => `/api/v1/admin/backup/${id}`,
  GetBackupStats: '/api/v1/admin/backup/stats',
  HealthCheck: '/health',
  // 新增：站内通知广播
  BroadcastNotifications: '/api/v1/admin/notifications/broadcast',
}

/**
 * @description: 获取系统统计数据
 */
export const getStats = () => {
  return defHttp.get<Stats>({ url: Api.GetStats })
}

/**
 * @description: 获取用户统计数据
 */
export const getUserStats = (params?: { page?: number; size?: number }) => {
  return defHttp.get<{ list: UserStats[]; total: number }>({
    url: Api.GetUserStats,
    params,
  })
}

/**
 * @description: 获取分类列表
 */
export const getCategories = () => {
  return defHttp.get<any>({ url: Api.GetCategories }).then((res: any) => res?.list || [])
}

export const createCategory = (data: { name: string; description?: string; enabled?: boolean; subscription_locked?: boolean }) => {
  return defHttp.post<any>({ url: Api.CreateCategory, data })
}

export const updateCategory = (id: number, data: { name?: string; description?: string; enabled?: boolean; subscription_locked?: boolean }) => {
  return defHttp.put<any>({ url: Api.UpdateCategory(id), data })
}

export const deleteCategory = (id: number) => {
  return defHttp.delete<any>({ url: Api.DeleteCategory(id) })
}

/**
 * @description: 获取用户操作记录
 */
export const getUserActions = (params?: { page?: number; size?: number }) => {
  return defHttp.get<{ list: any[]; total: number }>({
    url: '/api/v1/admin/user-actions',
    params,
  })
}

/**
 * @description: 获取系统日志
 */
export const getSystemLogs = (params?: { page?: number; page_size?: number; level?: string; search?: string; start_time?: string; end_time?: string }) => {
  return defHttp.get<{ list: SystemLog[]; total: number }>({
    url: Api.GetLogs,
    params,
  })
}

export const deleteLog = (id: number) => {
  return defHttp.delete<any>({ url: `/api/v1/admin/logs/${id}` })
}

export const batchDeleteLogs = (ids: number[]) => {
  return defHttp.post<any>({ url: `/api/v1/admin/logs/batch-delete`, data: { ids } })
}

export const clearLogs = () => {
  return defHttp.post<any>({ url: `/api/v1/admin/logs/clear` })
}

/**
 * @description: 获取备份列表
 */
export const getBackups = (params?: { page?: number; page_size?: number }) => {
  return defHttp.get<{ list: BackupInfo[]; total: number }>({ url: Api.GetBackups, params })
}

/**
 * @description: 创建备份
 */
export const createBackup = (data: { backup_type: string; description?: string }) => {
  return defHttp.post<any>({ url: Api.CreateBackup, data })
}

/**
 * @description: 删除备份
 */
export const deleteBackup = (id: string) => {
  return defHttp.delete<any>({ url: Api.DeleteBackup(id) })
}

/**
 * @description: 获取备份统计
 */
export const getBackupStats = () => {
  return defHttp.get<{
    total_backups: number
    success_backups: number
    failed_backups: number
    total_size: number
    last_backup_time?: string
    next_scheduled_backup?: string
  }>({ url: Api.GetBackupStats })
}

/**
 * @description: 健康检查
 */
export const healthCheck = () => {
  return defHttp.get<HealthStatus>({ url: Api.HealthCheck })
}

// 用户注册趋势
export const getUserRegistrationTrend = () => {
  return defHttp.get<{ list: { date: string; count: number }[] }>({ url: Api.GetUserRegistrationTrend })
}

/**
 * @description: 站内通知广播
 */
export const broadcastNotifications = (data: {
  target: 'all' | 'subscribers' | 'single'
  category_id?: number
  email?: string
  username?: string
  user_id?: number
  title: string
  content: string
  link?: string
  notif_type?: string
  related_type?: string
  related_id?: number
}) => {
  return defHttp.post<any>({ url: Api.BroadcastNotifications, data })
} 