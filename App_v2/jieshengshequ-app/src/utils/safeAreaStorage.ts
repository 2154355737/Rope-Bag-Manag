/**
 * 安全区域配置存储工具
 */

import { SafeAreaConfig } from '@/components/safe-area-provider'

const STORAGE_KEY = 'jieshengshequ-safe-area-config'

/**
 * 保存安全区域配置到本地存储
 */
export const saveSafeAreaConfig = (config: SafeAreaConfig): boolean => {
  try {
    // 创建一个副本，不保存预览模式状态
    const configToSave = { ...config, previewMode: false }
    localStorage.setItem(STORAGE_KEY, JSON.stringify(configToSave))
    console.log('💾 安全域配置已保存到本地存储:', configToSave)
    return true
  } catch (error) {
    console.error('❌ 保存安全域配置失败:', error)
    return false
  }
}

/**
 * 从本地存储加载安全区域配置
 */
export const loadSafeAreaConfig = (defaultConfig: SafeAreaConfig): SafeAreaConfig => {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      const parsedConfig = JSON.parse(saved) as Partial<SafeAreaConfig>
      const loadedConfig = { ...defaultConfig, ...parsedConfig }
      
      // 应用启动时不自动开启预览模式
      loadedConfig.previewMode = false
      
      console.log('🔧 从本地存储加载安全域配置:', loadedConfig)
      return loadedConfig
    }
  } catch (error) {
    console.error('❌ 解析安全域配置失败:', error)
    // 清除损坏的配置
    removeSafeAreaConfig()
  }
  
  console.log('🔧 使用默认安全域配置:', defaultConfig)
  return defaultConfig
}

/**
 * 删除本地存储中的安全区域配置
 */
export const removeSafeAreaConfig = (): boolean => {
  try {
    localStorage.removeItem(STORAGE_KEY)
    console.log('🗑️ 已清除本地存储的安全域配置')
    return true
  } catch (error) {
    console.error('❌ 清除本地存储配置失败:', error)
    return false
  }
}

/**
 * 验证当前配置是否与本地存储一致
 */
export const isConfigSavedToStorage = (config: SafeAreaConfig): boolean => {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      const savedConfig = JSON.parse(saved)
      return (
        savedConfig.topMargin === config.topMargin &&
        savedConfig.bottomMargin === config.bottomMargin &&
        savedConfig.leftMargin === config.leftMargin &&
        savedConfig.rightMargin === config.rightMargin &&
        savedConfig.autoDetect === config.autoDetect
      )
    }
  } catch (error) {
    console.error('❌ 验证配置保存状态失败:', error)
  }
  return false
}

/**
 * 获取配置存储信息
 */
export const getStorageInfo = () => {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    return {
      hasConfig: !!saved,
      size: saved ? saved.length : 0,
      lastModified: new Date().toISOString() // 这里简化处理，实际可以存储时间戳
    }
  } catch (error) {
    console.error('❌ 获取存储信息失败:', error)
    return {
      hasConfig: false,
      size: 0,
      lastModified: null
    }
  }
} 