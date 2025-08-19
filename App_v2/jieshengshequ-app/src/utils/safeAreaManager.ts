/**
 * 基于Web/CSS的安全区域管理器
 * 使用CSS的env()函数和纯样式方案，结合Capacitor Keyboard插件进行键盘检测
 */
import { Capacitor } from '@capacitor/core'
import { Keyboard, KeyboardInfo } from '@capacitor/keyboard'

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
   * 键盘检测 - 使用Capacitor Keyboard插件
   */
  private setupKeyboardDetection(): void {
    // 检查是否在原生环境中运行
    if (Capacitor.isNativePlatform()) {
      this.setupCapacitorKeyboardDetection()
    } else {
      // Web环境下的备用检测方案
      this.setupWebKeyboardDetection()
    }
  }

  /**
   * 使用Capacitor Keyboard插件进行键盘检测
   */
  private setupCapacitorKeyboardDetection(): void {
    console.log('🚀 使用Capacitor Keyboard插件进行键盘检测')
    
    // 监听键盘即将显示
    Keyboard.addListener('keyboardWillShow', (info: KeyboardInfo) => {
      console.log('⌨️ Capacitor: 键盘即将显示', info)
      this.updateKeyboardState(true, info.keyboardHeight)
    })

    // 监听键盘已显示
    Keyboard.addListener('keyboardDidShow', (info: KeyboardInfo) => {
      console.log('⌨️ Capacitor: 键盘已显示', info)
      this.updateKeyboardState(true, info.keyboardHeight)
      // 确保当前焦点元素可见
      this.ensureActiveElementVisible()
    })

    // 监听键盘即将隐藏
    Keyboard.addListener('keyboardWillHide', () => {
      console.log('⌨️ Capacitor: 键盘即将隐藏')
      this.updateKeyboardState(false)
    })

    // 监听键盘已隐藏
    Keyboard.addListener('keyboardDidHide', () => {
      console.log('⌨️ Capacitor: 键盘已隐藏')
      this.updateKeyboardState(false)
    })

    // 页面可见性变化处理
    document.addEventListener('visibilitychange', () => {
      if (document.hidden) {
        console.log('📱 页面隐藏，重置键盘状态')
        this.updateKeyboardState(false)
      }
    })
  }

  /**
   * Web环境下的键盘检测备用方案
   */
  private setupWebKeyboardDetection(): void {
    console.log('🌐 使用Web环境键盘检测方案')
    
    let keyboardDetectionTimeout: number | null = null
    let focusedInput: Element | null = null
    
    // 监听input focus/blur事件
    document.addEventListener('focusin', (e) => {
      if (this.isInputElement(e.target as Element)) {
        focusedInput = e.target as Element
        if (keyboardDetectionTimeout) {
          clearTimeout(keyboardDetectionTimeout)
        }
        keyboardDetectionTimeout = setTimeout(() => {
          this.updateKeyboardState(true)
          this.ensureActiveElementVisible()
        }, 150) as unknown as number
      }
    })
    
    document.addEventListener('focusout', (e) => {
      if (keyboardDetectionTimeout) {
        clearTimeout(keyboardDetectionTimeout)
      }
      
      keyboardDetectionTimeout = setTimeout(() => {
        const activeElement = document.activeElement
        if (!activeElement || !this.isInputElement(activeElement)) {
          focusedInput = null
          this.updateKeyboardState(false)
        }
      }, 150) as unknown as number
    })
    
    // Visual Viewport API 辅助检测
    if ('visualViewport' in window) {
      const viewport = window.visualViewport!
      
      const handleViewportChange = () => {
        const heightRatio = viewport.height / window.innerHeight
        const isKeyboardVisible = heightRatio < 0.75
        
        if (focusedInput && this.isInputElement(focusedInput)) {
          this.updateKeyboardState(isKeyboardVisible)
        } else if (!isKeyboardVisible) {
          this.updateKeyboardState(false)
        }
      }
      
      viewport.addEventListener('resize', handleViewportChange)
    }
    
    // 页面状态处理
    document.addEventListener('visibilitychange', () => {
      if (document.hidden) {
        this.updateKeyboardState(false)
        focusedInput = null
      }
    })
    
    window.addEventListener('popstate', () => {
      setTimeout(() => {
        const activeElement = document.activeElement
        if (!activeElement || !this.isInputElement(activeElement)) {
          this.updateKeyboardState(false)
          focusedInput = null
        }
      }, 100)
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
  private updateKeyboardState(visible: boolean, keyboardHeight?: number): void {
    if (this.config.keyboardVisible === visible) return
    
    this.config.keyboardVisible = visible
    this.applyCSSVariables()
    
    // 更新body类名
    document.body.classList.toggle('keyboard-visible', visible)
    
    // 如果提供了键盘高度，可以用于更精确的布局调整
    if (keyboardHeight && visible) {
      document.documentElement.style.setProperty('--keyboard-height', `${keyboardHeight}px`)
    } else {
      document.documentElement.style.removeProperty('--keyboard-height')
    }
    
    console.log('⌨️ 键盘状态更新:', visible ? '显示' : '隐藏', {
      keyboardHeight: keyboardHeight || 'unknown',
      activeElement: document.activeElement?.tagName,
      bodyClasses: document.body.className,
      keyboardVisible: this.config.keyboardVisible
    })
  }

  /**
   * 确保当前活动元素（通常是输入框）在可视区域内
   */
  private ensureActiveElementVisible(): void {
    const activeElement = document.activeElement as HTMLElement
    if (activeElement && this.isInputElement(activeElement)) {
      // 延迟一点时间让键盘完全弹出，然后滚动到输入框
      setTimeout(() => {
        activeElement.scrollIntoView({
          behavior: 'smooth',
          block: 'center',
          inline: 'nearest'
        })
      }, 300)
    }
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
   * 强制重置键盘状态
   */
  forceResetKeyboard(): void {
    console.log('🔧 强制重置键盘状态')
    this.config.keyboardVisible = false
    this.applyCSSVariables()
    document.body.classList.remove('keyboard-visible')
  }

  /**
   * 清理资源
   */
  destroy(): void {
    if (this.keyboardObserver) {
      this.keyboardObserver.disconnect()
      this.keyboardObserver = null
    }
    
    // 清理Capacitor Keyboard监听器
    if (Capacitor.isNativePlatform()) {
      Keyboard.removeAllListeners()
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
export const forceResetKeyboard = () => safeAreaManager.forceResetKeyboard() 