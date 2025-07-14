import { api, ApiResponse } from '../utils/apiClient'

export interface SystemSettings {
  theme: any
  system_mode: string
  feature_flags: any
  backend_config: any
  backup_settings: any
  global_announcement: any
}

export const settingsApi = {
  // 获取系统设置
  getSettings: (): Promise<ApiResponse<SystemSettings>> => {
    return api.get('/api/v1/settings')
  },
  // 更新系统设置
  updateSettings: (settings: Partial<SystemSettings>): Promise<ApiResponse> => {
    return api.put('/api/v1/settings', settings)
  },
  // 获取单项设置
  getSetting: (key: string): Promise<ApiResponse<any>> => {
    return api.get(`/api/v1/settings/${key}`)
  },
  // 更新单项设置
  updateSetting: (key: string, value: any): Promise<ApiResponse> => {
    return api.put(`/api/v1/settings/${key}`, { value })
  },
  // 重置设置
  resetSettings: (): Promise<ApiResponse> => {
    return api.post('/api/v1/settings/reset')
  }
} 