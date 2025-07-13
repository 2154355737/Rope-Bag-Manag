import axios from 'axios'
import type { ApiResponse } from './types'

// 创建API客户端
const api = axios.create({
  baseURL: 'http://127.0.0.1:15202',
  timeout: 15000,
})

// 通用请求函数
const request = async (config: any) => {
  try {
    const response = await api(config)
    return response.data
  } catch (error) {
    console.error('API请求失败:', error)
    throw error
  }
}

// 备份记录相关接口类型
export interface BackupRecord {
  id: string
  filename: string
  file_path: string
  file_size: number
  backup_type: string
  backup_time: string
  status: string
  description: string
  created_by: string
  restore_time?: string
  restore_by?: string
}

export interface BackupRecordQuery {
  backup_type?: string
  status?: string
  start_time?: string
  end_time?: string
  created_by?: string
  page?: number
  size?: number
}

// 获取备份记录列表
export function getBackupRecords(params?: BackupRecordQuery): Promise<ApiResponse<{
  records: BackupRecord[]
  total: number
  page: number
  size: number
}>> {
  return request({
    url: '/api/backup-records',
    method: 'GET',
    params
  })
}

// 创建备份
export function createBackup(data: {
  backup_type: string
  description?: string
}): Promise<ApiResponse<BackupRecord>> {
  return request({
    url: '/api/backup-records',
    method: 'POST',
    data
  })
}

// 下载备份
export function downloadBackup(id: string): Promise<ApiResponse<{
  download_url: string
  filename: string
}>> {
  return request({
    url: `/api/backup-records/${id}/download`,
    method: 'GET'
  })
}

// 恢复备份
export function restoreBackup(id: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/backup-records/${id}/restore`,
    method: 'POST'
  })
}

// 删除备份记录
export function deleteBackupRecord(id: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/backup-records/${id}`,
    method: 'DELETE'
  })
}

// 批量删除备份记录
export function batchDeleteBackupRecords(ids: number[]): Promise<ApiResponse<any>> {
  return request({
    url: '/api/backup-records/batch',
    method: 'POST', // 改为POST
    data: { ids }
  })
}

// 获取备份统计信息
export function getBackupStats(): Promise<ApiResponse<{
  total_backups: number
  success_backups: number
  failed_backups: number
  total_size: number
  backup_types: Record<string, number>
  daily_stats: Array<{
    date: string
    count: number
    size: number
  }>
}>> {
  return request({
    url: '/api/backup-records/stats',
    method: 'GET'
  })
}

// 配置自动备份
export function configureAutoBackup(data: {
  enable_auto_backup: boolean
  backup_interval_hours: number
  backup_location: string
  max_backup_files: number
}): Promise<ApiResponse<any>> {
  return request({
    url: '/api/backup-records/configure',
    method: 'POST',
    data
  })
} 