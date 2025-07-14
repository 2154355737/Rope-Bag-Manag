import { 
  initDB, 
  getSetting, 
  setSetting 
} from '@/utils/sqlite'
import type { ApiResponse, SystemSettings } from './types'

// 设置管理API
export const settingsApi = {
  // 获取系统设置
  getSettings: async (): Promise<ApiResponse<SystemSettings>> => {
    await initDB()
    
    try {
      const theme = getSetting('theme')
      const systemMode = getSetting('system_mode')
      const featureFlags = getSetting('feature_flags')
      const backupSettings = getSetting('backup_settings')
      
      const settings: SystemSettings = {
        theme: theme ? JSON.parse(theme) : {
          community_theme: 'light',
          admin_theme: 'dark'
        },
        system_mode: systemMode || 'production',
        feature_flags: featureFlags ? JSON.parse(featureFlags) : {
          enable_registration: true,
          enable_community: true,
          enable_upload: true,
          enable_comments: true,
          enable_qq_binding: false
        },
        backend_config: {
          proxy_address: '',
          api_timeout: 30000,
          max_upload_size: 10485760 // 10MB
        },
        backup_settings: backupSettings ? JSON.parse(backupSettings) : {
          enable_auto_backup: true,
          backup_interval_hours: 24,
          backup_location: 'local',
          max_backup_files: 10
        },
        global_announcement: {
          enabled: false,
          title: '',
          content: '',
          type_: 'info',
          start_time: '',
          end_time: '',
          priority: 0
        }
      }
      
      return {
        code: 0,
        msg: '获取系统设置成功',
        data: settings
      }
    } catch (error) {
      console.error('获取系统设置失败:', error)
      return {
        code: 1,
        msg: '获取系统设置失败'
      }
    }
  },

  // 更新系统设置
  updateSettings: async (settings: Partial<SystemSettings>): Promise<ApiResponse> => {
    await initDB()
    
    try {
      if (settings.theme) {
        setSetting('theme', JSON.stringify(settings.theme))
      }
      
      if (settings.system_mode) {
        setSetting('system_mode', settings.system_mode)
      }
      
      if (settings.feature_flags) {
        setSetting('feature_flags', JSON.stringify(settings.feature_flags))
      }
      
      if (settings.backup_settings) {
        setSetting('backup_settings', JSON.stringify(settings.backup_settings))
      }
      
      return {
        code: 0,
        msg: '系统设置更新成功'
      }
    } catch (error) {
      console.error('更新系统设置失败:', error)
      return {
        code: 1,
        msg: '更新系统设置失败'
      }
    }
  },

  // 获取特定设置
  getSetting: async (key: string): Promise<ApiResponse<any>> => {
    await initDB()
    
    try {
      const value = getSetting(key)
      
      if (value === null) {
        return {
          code: 1,
          msg: '设置不存在'
        }
      }
      
      // 尝试解析JSON
      let parsedValue
      try {
        parsedValue = JSON.parse(value)
      } catch {
        parsedValue = value
      }
      
      return {
        code: 0,
        msg: '获取设置成功',
        data: parsedValue
      }
    } catch (error) {
      console.error('获取设置失败:', error)
      return {
        code: 1,
        msg: '获取设置失败'
      }
    }
  },

  // 更新特定设置
  updateSetting: async (key: string, value: any): Promise<ApiResponse> => {
    await initDB()
    
    try {
      const stringValue = typeof value === 'string' ? value : JSON.stringify(value)
      setSetting(key, stringValue)
      
      return {
        code: 0,
        msg: '设置更新成功'
      }
    } catch (error) {
      console.error('更新设置失败:', error)
      return {
        code: 1,
        msg: '更新设置失败'
      }
    }
  },

  // 重置设置到默认值
  resetSettings: async (): Promise<ApiResponse> => {
    await initDB()
    
    try {
      const defaultSettings = {
        theme: {
          community_theme: 'light',
          admin_theme: 'dark'
        },
        system_mode: 'production',
        feature_flags: {
          enable_registration: true,
          enable_community: true,
          enable_upload: true,
          enable_comments: true,
          enable_qq_binding: false
        },
        backup_settings: {
          enable_auto_backup: true,
          backup_interval_hours: 24,
          backup_location: 'local',
          max_backup_files: 10
        }
      }
      
      setSetting('theme', JSON.stringify(defaultSettings.theme))
      setSetting('system_mode', defaultSettings.system_mode)
      setSetting('feature_flags', JSON.stringify(defaultSettings.feature_flags))
      setSetting('backup_settings', JSON.stringify(defaultSettings.backup_settings))
      
      return {
        code: 0,
        msg: '设置重置成功'
      }
    } catch (error) {
      console.error('重置设置失败:', error)
      return {
        code: 1,
        msg: '重置设置失败'
      }
    }
  }
} 