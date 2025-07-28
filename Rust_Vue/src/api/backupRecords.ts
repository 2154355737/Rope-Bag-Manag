import { api } from '../utils/apiClient'

/**
 * 备份记录类型
 */
export interface BackupRecord {
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

/**
 * 备份统计类型
 */
export interface BackupStats {
  total_backups: number
  success_backups: number
  failed_backups: number
  total_size: number
  last_backup_time?: string
  next_scheduled_backup?: string
}

/**
 * 获取备份记录列表
 * @param params 分页参数
 * @returns 备份记录列表及总数
 */
export function getBackupRecords(params: { page?: number; size?: number }) {
  return api.get('/api/v1/admin/backups', {
    params: {
      page: params.page || 1,
      page_size: params.size || 20
    }
  })
}

/**
 * 获取备份统计数据
 * @returns 备份统计数据
 */
export function getBackupStats() {
  return api.get('/api/v1/admin/backup/stats')
}

/**
 * 创建新备份
 * @param data 备份数据
 * @returns 创建结果
 */
export function createBackup(data: { backup_type: string; description?: string }) {
  return api.post('/api/v1/admin/backup', data)
}

/**
 * 获取备份详情
 * @param id 备份ID
 * @returns 备份详情
 */
export function getBackupDetail(id: string) {
  return api.get(`/api/v1/admin/backup/${id}`)
}

/**
 * 删除备份记录
 * @param id 备份ID
 * @returns 删除结果
 */
export function deleteBackupRecord(id: string) {
  return api.delete(`/api/v1/admin/backup/${id}`)
}

/**
 * 批量删除备份记录
 * @param ids 备份ID数组
 * @returns 删除结果
 */
export function batchDeleteBackupRecords(ids: string[]) {
  return api.post('/api/v1/admin/backup/batch-delete', {
    backup_ids: ids
  })
}

/**
 * 恢复备份
 * @param id 备份ID
 * @returns 恢复结果
 */
export function restoreBackup(id: string) {
  return api.post(`/api/v1/admin/backup/${id}/restore`, {
    confirm: true
  })
}

/**
 * 获取备份文件下载URL
 * @param id 备份ID
 * @returns 下载URL
 */
export function getBackupDownloadUrl(id: string) {
  return `/api/v1/admin/backup/${id}/download`
}

/**
 * 下载备份文件（返回二进制流）
 * @param id 备份ID
 * @returns 文件二进制流
 */
export function downloadBackup(id: string) {
  return api.get(`/api/v1/admin/backup/${id}/download`, {
    responseType: 'blob'
  })
}

/**
 * 配置自动备份
 * @param config 自动备份配置
 * @returns 配置结果
 */
export function configureAutoBackup(config: {
  enabled: boolean
  frequency: string
  time: string
  day?: number
  retain_count: number
  auto_clean: boolean
}) {
  return api.post('/api/v1/admin/backup-schedule', config)
}

/**
 * 获取自动备份配置
 * @returns 自动备份配置
 */
export function getAutoBackupConfig() {
  return api.get('/api/v1/admin/backup-schedule')
} 