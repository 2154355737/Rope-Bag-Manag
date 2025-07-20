import { api, ApiResponse } from '../utils/apiClient'

export interface SystemSettings {
  theme: {
    community_theme: string
    admin_theme: string
  }
  system_mode: string
  feature_flags: {
    enable_registration: boolean
    enable_community: boolean
    enable_upload: boolean
    enable_comments: boolean
    enable_qq_binding: boolean
  }
  backend_config: {
    proxy_address: string
    api_timeout: number
    max_upload_size: number
  }
  backup_settings: {
    enable_auto_backup: boolean
    backup_interval_hours: number
    backup_location: string
    max_backup_files: number
  }
  global_announcement: {
    enabled: boolean
    title: string
    content: string
    type_: string
    priority: number
  }
}

export interface ThemeSettings {
  primary_color: string
  secondary_color: string
  dark_mode: boolean
  font_size: string
  language: string
}

export const settingsApi = {
  // 获取系统设置
  getSettings: (): Promise<ApiResponse<Record<string, string>>> => {
    return api.get('/api/v1/admin/settings')
  },
  
  // 更新系统设置
  updateSettings: (settings: Partial<SystemSettings>): Promise<ApiResponse> => {
    return api.post('/api/v1/admin/settings', settings)
  },
  
  // 获取单项设置
  getSetting: (key: string): Promise<ApiResponse<{key: string, value: string}>> => {
    return api.get(`/api/v1/admin/settings/${key}`)
  },
  
  // 更新单项设置
  updateSetting: (key: string, value: any): Promise<ApiResponse> => {
    return api.post(`/api/v1/admin/settings/${key}`, { value })
  },
  
  // 重置设置
  resetSettings: (): Promise<ApiResponse> => {
    return api.post('/api/v1/admin/settings/reset')
  },
  
  // 获取主题设置
  getThemeSettings: (): Promise<ApiResponse<ThemeSettings>> => {
    return api.get('/api/v1/admin/theme-settings')
  },
  
  // 更新主题设置
  updateThemeSettings: (settings: ThemeSettings): Promise<ApiResponse> => {
    return api.post('/api/v1/admin/theme-settings', settings)
  }
} 