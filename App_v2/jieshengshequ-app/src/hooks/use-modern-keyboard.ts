/**
 * 现代化键盘处理 React Hook
 */

import { useEffect, useState, useCallback, useRef } from 'react'
import { keyboardManager, type KeyboardState } from '@/utils/modernKeyboard'

interface UseKeyboardOptions {
  /** 是否启用调试模式 */
  debug?: boolean
  /** 是否自动处理输入框滚动 */
  autoScroll?: boolean
  /** 键盘状态变化回调 */
  onStateChange?: (state: KeyboardState) => void
}

interface UseKeyboardReturn {
  /** 键盘状态 */
  state: KeyboardState
  /** 是否键盘打开 */
  isOpen: boolean
  /** 键盘高度 */
  height: number
  /** 强制更新键盘状态 */
  forceUpdate: () => void
  /** 模拟键盘（仅用于测试） */
  simulate: (height: number) => void
}

/**
 * 现代化键盘处理 Hook
 */
export function useModernKeyboard(options: UseKeyboardOptions = {}): UseKeyboardReturn {
  const { debug = false, autoScroll = true, onStateChange } = options
  
  const [state, setState] = useState<KeyboardState>(keyboardManager.getState())
  const callbackRef = useRef(onStateChange)
  callbackRef.current = onStateChange

  // 订阅键盘状态变化
  useEffect(() => {
    const unsubscribe = keyboardManager.onStateChange((newState) => {
      setState(newState)
      callbackRef.current?.(newState)
    })

    return unsubscribe
  }, [])



  // 强制更新
  const forceUpdate = useCallback(() => {
    keyboardManager.forceUpdate()
  }, [])

  // 模拟键盘（测试用）
  const simulate = useCallback((height: number) => {
    keyboardManager.simulateKeyboard(height)
  }, [])

  return {
    state,
    isOpen: state.isOpen,
    height: state.height,
    forceUpdate,
    simulate
  }
}

/**
 * 输入框自动滚动 Hook
 */
export function useInputAutoScroll(elementRef: React.RefObject<HTMLElement>) {
  const { isOpen, height } = useModernKeyboard()

  useEffect(() => {
    const element = elementRef.current
    if (!element || !isOpen) return

    // 添加现代输入元素类
    element.classList.add('modern-input-element')

    // 确保元素可见
    const ensureVisible = () => {
      const rect = element.getBoundingClientRect()
      const viewportHeight = window.innerHeight
      const targetTop = viewportHeight - height - 60

      if (rect.bottom > targetTop) {
        element.scrollIntoView({
          behavior: 'smooth',
          block: 'nearest',
          inline: 'nearest'
        })
      }
    }

    // 延迟执行，等待键盘动画完成
    const timer = setTimeout(ensureVisible, 100)

    return () => {
      clearTimeout(timer)
      element.classList.remove('modern-input-element')
    }
  }, [isOpen, height, elementRef])
}

/**
 * 键盘状态监听 Hook
 */
export function useKeyboardListener(callback: (state: KeyboardState) => void) {
  const callbackRef = useRef(callback)
  callbackRef.current = callback

  useEffect(() => {
    return keyboardManager.onStateChange((state) => {
      callbackRef.current(state)
    })
  }, [])
} 