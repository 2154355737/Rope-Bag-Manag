import React, { createContext, useContext, useEffect, useState } from 'react'

export interface SafeAreaConfig {
  topMargin: number
  bottomMargin: number
  leftMargin: number
  rightMargin: number
  autoDetect: boolean
  previewMode: boolean
}

interface SafeAreaContextType {
  config: SafeAreaConfig
  updateConfig: (updates: Partial<SafeAreaConfig>) => void
  resetConfig: () => void
  applyConfig: (config: SafeAreaConfig) => void
}

const defaultConfig: SafeAreaConfig = {
  topMargin: 0,
  bottomMargin: 0,
  leftMargin: 0,
  rightMargin: 0,
  autoDetect: true,
  previewMode: false
}

const SafeAreaContext = createContext<SafeAreaContextType | undefined>(undefined)

export const useSafeArea = () => {
  const context = useContext(SafeAreaContext)
  if (!context) {
    throw new Error('useSafeArea must be used within a SafeAreaProvider')
  }
  return context
}

interface SafeAreaProviderProps {
  children: React.ReactNode
}

export const SafeAreaProvider: React.FC<SafeAreaProviderProps> = ({ children }) => {
  const [config, setConfig] = useState<SafeAreaConfig>(() => {
    // 从本地存储加载配置
    try {
      const saved = localStorage.getItem('jieshengshequ-safe-area-config')
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
      localStorage.removeItem('jieshengshequ-safe-area-config')
    }
    
    console.log('🔧 使用默认安全域配置:', defaultConfig)
    return defaultConfig
  })

  // 保存配置到本地存储
  const saveConfigToStorage = (config: SafeAreaConfig) => {
    try {
      // 创建一个副本，不保存预览模式状态
      const configToSave = { ...config, previewMode: false }
      localStorage.setItem('jieshengshequ-safe-area-config', JSON.stringify(configToSave))
      console.log('💾 安全域配置已保存到本地存储:', configToSave)
    } catch (error) {
      console.error('❌ 保存安全域配置失败:', error)
    }
  }

  // 应用安全域配置到CSS变量
  const applyConfig = (newConfig: SafeAreaConfig) => {
    const root = document.documentElement
    
    // 动态导入平台检测
    import('@/utils/platform').then(({ isAndroid }) => {
      const isAndroidPlatform = isAndroid()
      
      if (newConfig.autoDetect) {
        // 自动检测模式：移除自定义变量，使用系统env()值
        root.style.removeProperty('--custom-safe-area-top')
        root.style.removeProperty('--custom-safe-area-bottom')
        root.style.removeProperty('--custom-safe-area-left')
        root.style.removeProperty('--custom-safe-area-right')
        
        // 恢复系统安全区域变量，让CSS使用默认的env()值
        root.style.removeProperty('--safe-area-system-top')
        root.style.removeProperty('--safe-area-system-bottom')
        root.style.removeProperty('--safe-area-system-left')
        root.style.removeProperty('--safe-area-system-right')
        
        console.log('🔄 自动检测模式：使用系统安全区域', { platform: isAndroidPlatform ? 'Android' : 'Other' })
      } else {
        // 手动配置模式：设置自定义边距，禁用系统安全区域
        root.style.setProperty('--custom-safe-area-top', `${newConfig.topMargin}px`)
        root.style.setProperty('--custom-safe-area-bottom', `${newConfig.bottomMargin}px`)
        root.style.setProperty('--custom-safe-area-left', `${newConfig.leftMargin}px`)
        root.style.setProperty('--custom-safe-area-right', `${newConfig.rightMargin}px`)
        
        // 在手动模式下，将系统安全区域设置为0，完全使用自定义值
        // Android设备需要特别强制禁用系统值
        root.style.setProperty('--safe-area-system-top', '0px')
        root.style.setProperty('--safe-area-system-bottom', '0px')
        root.style.setProperty('--safe-area-system-left', '0px')
        root.style.setProperty('--safe-area-system-right', '0px')
        
        // Android特殊处理：添加强制覆盖标记
        if (isAndroidPlatform) {
          root.classList.add('android-manual-safe-area')
          root.style.setProperty('--android-force-override', '1')
          console.log('🤖 Android手动模式：强制禁用系统安全区域')
        } else {
          root.classList.remove('android-manual-safe-area')
          root.style.removeProperty('--android-force-override')
        }
        
        console.log('🔧 手动配置模式：禁用系统安全区域，使用自定义配置:', {
          platform: isAndroidPlatform ? 'Android' : 'Other',
          top: newConfig.topMargin,
          bottom: newConfig.bottomMargin,
          left: newConfig.leftMargin,
          right: newConfig.rightMargin
        })
      }
    }).catch(console.error)
    
    // 预览模式样式
    if (newConfig.previewMode) {
      document.body.classList.add('safe-area-preview')
      // 延迟添加预览指示器，确保CSS变量已生效
      setTimeout(() => addPreviewIndicator(newConfig), 100)
    } else {
      document.body.classList.remove('safe-area-preview')
      removePreviewIndicator()
    }
  }

  // 添加预览指示器
  const addPreviewIndicator = (config: SafeAreaConfig) => {
    removePreviewIndicator() // 先移除已有的
    
    const indicator = document.createElement('div')
    indicator.className = 'safe-area-indicator'
    
    // 获取当前实际的安全区域值
    const computedStyle = getComputedStyle(document.documentElement)
    let topValue = '0px'
    let bottomValue = '0px'
    
    if (config.autoDetect) {
      // 自动检测模式：尝试获取系统env()值
      const testElement = document.createElement('div')
      testElement.style.position = 'fixed'
      testElement.style.top = '0'
      testElement.style.left = '0'
      testElement.style.width = '1px'
      testElement.style.height = '1px'
      testElement.style.paddingTop = 'env(safe-area-inset-top)'
      testElement.style.paddingBottom = 'env(safe-area-inset-bottom)'
      testElement.style.visibility = 'hidden'
      document.body.appendChild(testElement)
      
      const rect = testElement.getBoundingClientRect()
      const style = getComputedStyle(testElement)
      topValue = style.paddingTop || '0px'
      bottomValue = style.paddingBottom || '0px'
      
      document.body.removeChild(testElement)
    } else {
      // 手动模式：使用配置值
      topValue = `${config.topMargin}px`
      bottomValue = `${config.bottomMargin}px`
    }
    
    indicator.innerHTML = `
      <div style="
        text-align: center; 
        font-size: 12px; 
        line-height: 1.4;
        background: rgba(0, 0, 0, 0.9);
        color: white;
        padding: 8px 12px;
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
      ">
        <div style="font-weight: 600;">🛡️ 安全区域预览</div>
        <div style="margin-top: 4px; opacity: 0.9; font-size: 11px;">
          ${config.autoDetect 
            ? `自动检测: 顶部${topValue} 底部${bottomValue}` 
            : `手动配置: 顶部${config.topMargin}px 底部${config.bottomMargin}px`
          }
        </div>
      </div>
    `
    
    // 设置指示器样式
    Object.assign(indicator.style, {
      position: 'fixed',
      top: '50%',
      left: '50%',
      transform: 'translate(-50%, -50%)',
      zIndex: '10001',
      pointerEvents: 'none',
      fontFamily: 'system-ui, -apple-system, sans-serif'
    })
    
    document.body.appendChild(indicator)
  }

  // 移除预览指示器
  const removePreviewIndicator = () => {
    const existing = document.querySelector('.safe-area-indicator')
    if (existing) {
      existing.remove()
    }
  }

  // 更新配置
  const updateConfig = (updates: Partial<SafeAreaConfig>) => {
    const newConfig = { ...config, ...updates }
    setConfig(newConfig)
    applyConfig(newConfig)
    
    // 保存到本地存储
    saveConfigToStorage(newConfig)
  }

  // 重置配置
  const resetConfig = () => {
    console.log('🔄 重置安全域配置为默认值')
    setConfig(defaultConfig)
    applyConfig(defaultConfig)
    // 清除本地存储
    try {
      localStorage.removeItem('jieshengshequ-safe-area-config')
      console.log('🗑️ 已清除本地存储的安全域配置')
    } catch (error) {
      console.error('❌ 清除本地存储配置失败:', error)
    }
  }

  // 初始化时应用配置
  useEffect(() => {
    console.log('🚀 安全域提供者初始化，应用配置:', config)
    
    // 延迟应用配置，确保DOM完全准备好
    const timeoutId = setTimeout(() => {
      applyConfig(config)
      console.log('✅ 安全域配置已应用到CSS')
    }, 50)
    
    // 监听屏幕方向变化
    const handleOrientationChange = () => {
      console.log('📱 屏幕方向变化，重新应用配置')
      setTimeout(() => applyConfig(config), 300) // 延迟重新应用，等待布局稳定
    }
    
    window.addEventListener('orientationchange', handleOrientationChange)
    window.addEventListener('resize', handleOrientationChange)
    
    // 清理函数
    return () => {
      clearTimeout(timeoutId)
      removePreviewIndicator()
      window.removeEventListener('orientationchange', handleOrientationChange)
      window.removeEventListener('resize', handleOrientationChange)
    }
  }, [])

  // 当配置变化时重新应用（但不包括预览模式的临时变化）
  useEffect(() => {
    if (config) {
      console.log('🔄 配置状态变化，重新应用:', config)
      applyConfig(config)
    }
  }, [config.autoDetect, config.topMargin, config.bottomMargin, config.leftMargin, config.rightMargin])

  const value: SafeAreaContextType = {
    config,
    updateConfig,
    resetConfig,
    applyConfig
  }

  return (
    <SafeAreaContext.Provider value={value}>
      {children}
    </SafeAreaContext.Provider>
  )
} 