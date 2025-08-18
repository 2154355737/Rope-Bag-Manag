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
    const saved = localStorage.getItem('jieshengshequ-safe-area-config')
    if (saved) {
      try {
        return { ...defaultConfig, ...JSON.parse(saved) }
      } catch (error) {
        console.error('解析安全域配置失败:', error)
        return defaultConfig
      }
    }
    return defaultConfig
  })

  // 应用安全域配置到CSS变量
  const applyConfig = (newConfig: SafeAreaConfig) => {
    const root = document.documentElement
    
    if (newConfig.autoDetect) {
      // 使用系统检测的安全区域
      root.style.removeProperty('--custom-safe-area-top')
      root.style.removeProperty('--custom-safe-area-bottom')
      root.style.removeProperty('--custom-safe-area-left')
      root.style.removeProperty('--custom-safe-area-right')
    } else {
      // 使用自定义边距
      root.style.setProperty('--custom-safe-area-top', `${newConfig.topMargin}px`)
      root.style.setProperty('--custom-safe-area-bottom', `${newConfig.bottomMargin}px`)
      root.style.setProperty('--custom-safe-area-left', `${newConfig.leftMargin}px`)
      root.style.setProperty('--custom-safe-area-right', `${newConfig.rightMargin}px`)
    }
    
    // 预览模式样式
    if (newConfig.previewMode) {
      document.body.classList.add('safe-area-preview')
      // 添加预览指示器
      addPreviewIndicator(newConfig)
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
    indicator.innerHTML = `
      <div style="text-align: center; font-size: 12px; line-height: 1.4;">
        <div>🛡️ 安全区域预览</div>
        <div style="margin-top: 4px; opacity: 0.8;">
          ${config.autoDetect ? '自动检测' : `顶部:${config.topMargin}px 底部:${config.bottomMargin}px`}
        </div>
      </div>
    `
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
    localStorage.setItem('jieshengshequ-safe-area-config', JSON.stringify(newConfig))
  }

  // 重置配置
  const resetConfig = () => {
    setConfig(defaultConfig)
    applyConfig(defaultConfig)
    localStorage.removeItem('jieshengshequ-safe-area-config')
  }

  // 初始化时应用配置
  useEffect(() => {
    applyConfig(config)
    
    // 清理函数
    return () => {
      removePreviewIndicator()
    }
  }, [])

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