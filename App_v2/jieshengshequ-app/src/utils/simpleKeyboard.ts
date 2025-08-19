/**
 * 简化的键盘处理系统 - 避免DOM操作抽搐问题
 */

import { Capacitor } from '@capacitor/core'
import { Keyboard } from '@capacitor/keyboard'

interface SimpleKeyboardState {
  isOpen: boolean
  height: number
}

class SimpleKeyboardManager {
  private state: SimpleKeyboardState = {
    isOpen: false,
    height: 0
  }

  private isInitialized = false
  private debounceTimer: number | null = null

  constructor() {
    this.initialize()
  }

  private initialize() {
    if (this.isInitialized) return
    
    console.log('🚀 初始化简化键盘管理器...')
    
    if (Capacitor.isNativePlatform()) {
      this.initializeNativeKeyboard()
    } else {
      this.initializeWebKeyboard()
    }

    this.isInitialized = true
  }

  private initializeNativeKeyboard() {
    try {
      // 键盘显示事件
      Keyboard.addListener('keyboardWillShow', (info) => {
        this.handleKeyboardShow(info.keyboardHeight)
      })

      // 键盘隐藏事件
      Keyboard.addListener('keyboardWillHide', () => {
        this.handleKeyboardHide()
      })

      console.log('✅ 原生键盘监听器已设置')
    } catch (error) {
      console.error('❌ 设置原生键盘监听器失败:', error)
    }
  }

  private initializeWebKeyboard() {
    // 使用Visual Viewport API（如果可用）
    if ('visualViewport' in window && window.visualViewport) {
      window.visualViewport.addEventListener('resize', () => {
        this.handleViewportChange()
      })
      console.log('✅ Web键盘监听器已设置（Visual Viewport）')
    }
  }

  private handleViewportChange() {
    if (!window.visualViewport) return

    const viewport = window.visualViewport
    const keyboardHeight = window.innerHeight - viewport.height

    if (keyboardHeight > 150) { // 键盘可能打开
      this.handleKeyboardShow(keyboardHeight)
    } else { // 键盘可能关闭
      this.handleKeyboardHide()
    }
  }

  private handleKeyboardShow(keyboardHeight: number) {
    // 防抖处理，避免频繁触发
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer)
    }

    this.debounceTimer = window.setTimeout(() => {
      if (this.state.isOpen && Math.abs(this.state.height - keyboardHeight) < 50) {
        return // 高度变化不大，忽略
      }

      console.log('⌨️ 键盘显示，高度:', keyboardHeight)

      this.state.isOpen = true
      this.state.height = keyboardHeight

      // 只更新CSS变量，让CSS处理所有样式
      this.updateCSSVariables(keyboardHeight)

      // 只添加必要的CSS类
      document.body.classList.add('keyboard-open', 'modern-keyboard-open')

      this.debounceTimer = null
    }, 50) // 50ms防抖
  }

  private handleKeyboardHide() {
    // 防抖处理
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer)
    }

    this.debounceTimer = window.setTimeout(() => {
      if (!this.state.isOpen) {
        return // 已经是关闭状态
      }

      console.log('⌨️ 键盘隐藏')

      this.state.isOpen = false
      this.state.height = 0

      // 只更新CSS变量
      this.updateCSSVariables(0)

      // 只移除CSS类
      document.body.classList.remove('keyboard-open', 'modern-keyboard-open')

      this.debounceTimer = null
    }, 100) // 100ms防抖
  }

  private updateCSSVariables(keyboardHeight: number) {
    const root = document.documentElement
    
    // 设置键盘高度变量
    root.style.setProperty('--keyboard-height', `${keyboardHeight}px`)
    
    // 设置键盘状态变量
    root.style.setProperty('--keyboard-open', keyboardHeight > 0 ? '1' : '0')
    
    console.log('🔧 更新CSS变量:', {
      '--keyboard-height': `${keyboardHeight}px`,
      '--keyboard-open': keyboardHeight > 0 ? '1' : '0'
    })
  }

  // 公共API
  public getState(): SimpleKeyboardState {
    return { ...this.state }
  }

  public isKeyboardOpen(): boolean {
    return this.state.isOpen
  }

  public getKeyboardHeight(): number {
    return this.state.height
  }

  public destroy() {
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer)
      this.debounceTimer = null
    }

    if (Capacitor.isNativePlatform()) {
      Keyboard.removeAllListeners()
    }

    this.isInitialized = false
    console.log('🧹 键盘管理器已销毁')
  }
}

// 全局实例
let keyboardManager: SimpleKeyboardManager | null = null

// 初始化函数
export const initializeSimpleKeyboard = () => {
  if (keyboardManager) {
    keyboardManager.destroy()
  }
  keyboardManager = new SimpleKeyboardManager()
  return keyboardManager
}

// 获取键盘状态
export const getKeyboardState = () => {
  return keyboardManager?.getState() || { isOpen: false, height: 0 }
}

// 清理函数
export const destroyKeyboard = () => {
  if (keyboardManager) {
    keyboardManager.destroy()
    keyboardManager = null
  }
} 