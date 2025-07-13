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

// 设置相关接口类型
export interface ThemeSettings {
  community_theme: string
  admin_theme: string
}

export interface FeatureFlags {
  enable_registration: boolean
  enable_community: boolean
  enable_upload: boolean
  enable_comments: boolean
  enable_qq_binding: boolean
}

export interface BackendConfig {
  proxy_address: string
  api_timeout: number
  max_upload_size: number
}

export interface BackupSettings {
  enable_auto_backup: boolean
  backup_interval_hours: number
  backup_location: string
  max_backup_files: number
}

export interface GlobalAnnouncement {
  enabled: boolean
  title: string
  content: string
  type_: string
  start_time: string
  end_time: string
  priority: number
}

export interface SystemSettings {
  theme: ThemeSettings
  system_mode: string
  feature_flags: FeatureFlags
  backend_config: BackendConfig
  backup_settings: BackupSettings
  global_announcement: GlobalAnnouncement
}

// 获取系统设置
export function getSettings(): Promise<ApiResponse<SystemSettings>> {
  return request({
    url: '/api/settings',
    method: 'GET'
  })
}

// 更新系统设置
export function updateSettings(settings: SystemSettings): Promise<ApiResponse<any>> {
  return request({
    url: '/api/settings',
    method: 'POST',
    data: settings
  })
}

// 获取系统状态
export function getSystemStatus(): Promise<ApiResponse<any>> {
  return request({
    url: '/api/system-status',
    method: 'GET'
  })
}

// 检查功能开关
export function checkFeature(feature: string): Promise<ApiResponse<boolean>> {
  return request({
    url: '/api/check-feature',
    method: 'GET',
    params: { feature }
  })
} 