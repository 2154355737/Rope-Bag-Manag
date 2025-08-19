/**
 * 基于Web/CSS的安全区域管理器
 * 使用CSS的env()函数和纯样式方案，不进行任何原生高度计算
 */
import { Capacitor } from '@capacitor/core'

export interface SafeAreaConfig {
  keyboardVisible: boolean
}

export interface SafeAreaTheme {
  statusBarBg: string
  navigationBarBg: string
}

// 预设主题
export const SAFE_AREA_THEMES = {
  light: { statusBarBg: '#ffffff', navigationBarBg: '#ffffff' },
  dark: { statusBarBg: '#000000', navigationBarBg: '#000000' },
  primary: { statusBarBg: '#2563eb', navigationBarBg: '#2563eb' },
  transparent: { statusBarBg: 'transparent', navigationBarBg: 'transparent' },
  custom: { statusBarBg: '', navigationBarBg: '' }
} as const

export type SafeAreaThemeName = keyof typeof SAFE_AREA_THEMES

class SafeAreaManager {
  private config: SafeAreaConfig = {
    keyboardVisible: false
  }
  
  private isInitialized = false
  private keyboardObserver: ResizeObserver | null = null

  /**
   * 初始化安全区域管理器
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) return

    try {
      // 设置初始CSS变量
      this.applyCSSVariables()
      
      // 启动键盘监听
      this.setupKeyboardDetection()
      
      this.isInitialized = true
      console.log('✅ 安全区域管理器初始化完成（纯CSS方案）', this.config)
    } catch (error) {
      console.error('❌ 安全区域管理器初始化失败:', error)
      // 使用默认配置
      this.applyCSSVariables()
    }
  }

  /**
   * 应用CSS变量 - 只处理键盘状态
   */
  private applyCSSVariables(): void {
    const root = document.documentElement
    
    // 移除所有原生高度设置，完全依赖CSS env()函数
    root.style.setProperty('--keyboard-visible', this.config.keyboardVisible ? '1' : '0')
    
    // 移除原生高度变量
    root.style.removeProperty('--status-bar-height')
    root.style.removeProperty('--navigation-bar-height')
  }

  /**
   * 设置主题 - 只设置颜色
   */
  setTheme(theme: SafeAreaThemeName | SafeAreaTheme): void {
    let themeConfig: SafeAreaTheme
    
    if (typeof theme === 'string') {
      themeConfig = SAFE_AREA_THEMES[theme]
    } else {
      themeConfig = theme
    }
    
    // 只设置颜色，不设置高度
    const root = document.documentElement
    root.style.setProperty('--status-bar-bg', themeConfig.statusBarBg)
    root.style.setProperty('--navigation-bar-bg', themeConfig.navigationBarBg)
    
    console.log('🎨 安全区域主题已更新:', themeConfig)
  }

  /**
   * 设置自定义颜色
   */
  setColors(statusBarBg?: string, navigationBarBg?: string): void {
    const root = document.documentElement
    
    if (statusBarBg) root.style.setProperty('--status-bar-bg', statusBarBg)
    if (navigationBarBg) root.style.setProperty('--navigation-bar-bg', navigationBarBg)
    
    console.log('🎨 安全区域颜色已更新:', {
      statusBarBg,
      navigationBarBg
    })
  }

  /**
   * 键盘检测
   */
  private setupKeyboardDetection(): void {
    // 方法1: 使用Visual Viewport API (现代浏览器)
    if ('visualViewport' in window) {
      const viewport = window.visualViewport!
      
      const handleViewportChange = () => {
        const isKeyboardVisible = viewport.height < window.innerHeight * 0.75
        this.updateKeyboardState(isKeyboardVisible)
      }
      
      viewport.addEventListener('resize', handleViewportChange)
      viewport.addEventListener('scroll', handleViewportChange)
    }
    
    // 方法2: 监听input focus/blur事件
    document.addEventListener('focusin', (e) => {
      if (this.isInputElement(e.target as Element)) {
        setTimeout(() => this.updateKeyboardState(true), 300)
      }
    })
    
    document.addEventListener('focusout', () => {
      setTimeout(() => this.updateKeyboardState(false), 300)
    })
    
    // 方法3: 窗口resize监听 (fallback)
    let initialHeight = window.innerHeight
    
    window.addEventListener('resize', () => {
      const currentHeight = window.innerHeight
      const heightDiff = initialHeight - currentHeight
      const isKeyboardVisible = heightDiff > 150 // 阈值
      
      this.updateKeyboardState(isKeyboardVisible)
    })
  }

  /**
   * 判断是否为输入元素
   */
  private isInputElement(element: Element): boolean {
    if (!element) return false
    
    const tagName = element.tagName.toLowerCase()
    const inputTypes = ['input', 'textarea', 'select']
    
    return inputTypes.includes(tagName) || 
           element.getAttribute('contenteditable') === 'true'
  }

  /**
   * 更新键盘状态
   */
  private updateKeyboardState(visible: boolean): void {
    if (this.config.keyboardVisible === visible) return
    
    this.config.keyboardVisible = visible
    this.applyCSSVariables()
    
    // 更新body类名
    document.body.classList.toggle('keyboard-visible', visible)
    
    console.log('⌨️ 键盘状态更新:', visible ? '显示' : '隐藏')
  }

  /**
   * 启用/禁用调试模式
   */
  setDebugMode(enabled: boolean): void {
    document.body.classList.toggle('debug-safe-area', enabled)
    console.log('🐛 安全区域调试模式:', enabled ? '启用' : '禁用')
  }

  /**
   * 获取当前配置
   */
  getConfig(): SafeAreaConfig {
    return { ...this.config }
  }

  /**
   * 清理资源
   */
  destroy(): void {
    if (this.keyboardObserver) {
      this.keyboardObserver.disconnect()
      this.keyboardObserver = null
    }
    
    this.isInitialized = false
    console.log('🧹 安全区域管理器已清理')
  }
}

// 导出单例实例
export const safeAreaManager = new SafeAreaManager()

// 便捷函数
export const initializeSafeArea = () => safeAreaManager.initialize()
export const setSafeAreaTheme = (theme: SafeAreaThemeName | SafeAreaTheme) => safeAreaManager.setTheme(theme)
export const setSafeAreaColors = (statusBarBg?: string, navigationBarBg?: string) => safeAreaManager.setColors(statusBarBg, navigationBarBg)
export const setSafeAreaDebug = (enabled: boolean) => safeAreaManager.setDebugMode(enabled) 