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
      // 自动检测模式：清除自定义变量，让系统env()值生效
      root.style.removeProperty('--custom-safe-area-top')
      root.style.removeProperty('--custom-safe-area-bottom')
      root.style.removeProperty('--custom-safe-area-left')
      root.style.removeProperty('--custom-safe-area-right')
      
      // 设置标识变量，表示使用自动检测
      root.style.setProperty('--safe-area-auto-detect', '1')
    } else {
      // 手动配置模式：设置自定义边距
      root.style.setProperty('--custom-safe-area-top', `${newConfig.topMargin}px`)
      root.style.setProperty('--custom-safe-area-bottom', `${newConfig.bottomMargin}px`)
      root.style.setProperty('--custom-safe-area-left', `${newConfig.leftMargin}px`)
      root.style.setProperty('--custom-safe-area-right', `${newConfig.rightMargin}px`)
      
      // 移除自动检测标识
      root.style.removeProperty('--safe-area-auto-detect')
    }
    
    // 预览模式样式
    if (newConfig.previewMode) {
      document.body.classList.add('safe-area-preview')
      // 延迟添加预览指示器，确保CSS变量已生效
      setTimeout(() => addPreviewIndicator(newConfig), 50)
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
    
    // 监听屏幕方向变化
    const handleOrientationChange = () => {
      setTimeout(() => applyConfig(config), 300) // 延迟重新应用，等待布局稳定
    }
    
    window.addEventListener('orientationchange', handleOrientationChange)
    window.addEventListener('resize', handleOrientationChange)
    
    // 清理函数
    return () => {
      removePreviewIndicator()
      window.removeEventListener('orientationchange', handleOrientationChange)
      window.removeEventListener('resize', handleOrientationChange)
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