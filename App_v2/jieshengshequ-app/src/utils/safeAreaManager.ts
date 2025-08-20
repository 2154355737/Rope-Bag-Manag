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
  private scrollTimeout: number | null = null

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
    if (!activeElement || !this.isInputElement(activeElement)) {
      return
    }

    // 清除之前的滚动超时，防止重复滚动
    if (this.scrollTimeout) {
      clearTimeout(this.scrollTimeout)
    }

    // 延迟一点时间让键盘完全弹出和布局稳定
    this.scrollTimeout = setTimeout(() => {
      this.smoothScrollToElement(activeElement)
      this.scrollTimeout = null
    }, 200) as unknown as number
  }

  /**
   * 智能滚动到指定元素，避免不必要的页面跳动
   */
  private smoothScrollToElement(element: HTMLElement): void {
    // 再次检查元素是否仍然是活动元素（防止在延迟期间焦点发生变化）
    if (document.activeElement !== element) {
      console.log('🔄 元素焦点已变化，跳过滚动')
      return
    }

    const rect = element.getBoundingClientRect()
    const scrollContainer = this.findScrollContainer(element)
    
    if (!scrollContainer) {
      console.warn('⚠️ 未找到滚动容器')
      return
    }

    const containerRect = scrollContainer.getBoundingClientRect()
    const topNavHeight = this.getTopNavigationHeight()
    const bottomNavHeight = 64 // 底部导航栏高度
    const keyboardHeight = this.getKeyboardHeight()
    
    // 计算可视区域 (考虑键盘高度)
    const viewportHeight = window.innerHeight
    const effectiveViewportHeight = keyboardHeight > 0 ? viewportHeight - keyboardHeight : viewportHeight
    
    const visibleTop = Math.max(containerRect.top, 0) + topNavHeight + 10 // 10px 缓冲
    const visibleBottom = Math.min(containerRect.bottom, effectiveViewportHeight) - bottomNavHeight - 10 // 10px 缓冲
    const visibleHeight = visibleBottom - visibleTop
    
    // 如果可视区域太小，不进行滚动
    if (visibleHeight < 100) {
      console.log('⚠️ 可视区域太小，跳过滚动', { visibleHeight })
      return
    }
    
    // 检查元素是否已经在可视区域内
    const elementTop = rect.top
    const elementBottom = rect.bottom
    const elementHeight = rect.height
    
    // 如果元素完全在可视区域内，无需滚动
    if (elementTop >= visibleTop && elementBottom <= visibleBottom) {
      console.log('✅ 输入框已在可视区域内，无需滚动', {
        elementTop, elementBottom, visibleTop, visibleBottom
      })
      return
    }

    // 计算最小滚动距离
    let scrollOffset = 0
    
    if (elementTop < visibleTop) {
      // 元素在可视区域上方，需要向上滚动
      scrollOffset = elementTop - visibleTop
    } else if (elementBottom > visibleBottom) {
      // 元素在可视区域下方，需要向下滚动
      scrollOffset = elementBottom - visibleBottom
      
      // 如果元素太高，优先显示顶部
      if (elementHeight > visibleHeight) {
        scrollOffset = elementTop - visibleTop
      }
    }

    // 如果滚动距离太小，不进行滚动
    if (Math.abs(scrollOffset) < 10) {
      console.log('📏 滚动距离太小，跳过滚动', { scrollOffset })
      return
    }

    const newScrollTop = scrollContainer.scrollTop + scrollOffset
    
    // 确保滚动位置在有效范围内
    const maxScrollTop = scrollContainer.scrollHeight - scrollContainer.clientHeight
    const finalScrollTop = Math.max(0, Math.min(newScrollTop, maxScrollTop))

    // 执行平滑滚动
    scrollContainer.scrollTo({
      top: finalScrollTop,
      behavior: 'smooth'
    })

    console.log('📍 智能滚动到输入框', {
      elementRect: { top: rect.top, bottom: rect.bottom, height: rect.height },
      visibleArea: { top: visibleTop, bottom: visibleBottom, height: visibleHeight },
      scrollOffset,
      finalScrollTop,
      keyboardHeight,
      viewportHeight: effectiveViewportHeight
    })
  }

  /**
   * 查找元素的滚动容器
   */
  private findScrollContainer(element: HTMLElement): HTMLElement | null {
    let parent = element.parentElement
    
    while (parent) {
      const style = window.getComputedStyle(parent)
      const overflow = style.overflow + style.overflowY + style.overflowX
      
      if (/(auto|scroll)/.test(overflow)) {
        return parent
      }
      
      // 检查是否是我们的主要滚动容器
      if (parent.classList.contains('scroll-container') || parent.tagName === 'MAIN') {
        return parent
      }
      
      parent = parent.parentElement
    }
    
    // 如果没找到，返回document.documentElement
    return document.documentElement
  }

  /**
   * 获取顶部导航栏高度
   */
  private getTopNavigationHeight(): number {
    const topNavHeight = getComputedStyle(document.documentElement)
      .getPropertyValue('--top-navigation-height')
    return topNavHeight ? parseInt(topNavHeight) : 80
  }

  /**
   * 获取当前键盘高度
   */
  private getKeyboardHeight(): number {
    const keyboardHeight = getComputedStyle(document.documentElement)
      .getPropertyValue('--keyboard-height')
    return keyboardHeight ? parseInt(keyboardHeight) : 0
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
    
    // 清理滚动超时
    if (this.scrollTimeout) {
      clearTimeout(this.scrollTimeout)
      this.scrollTimeout = null
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