/**
 * 简化的键盘状态Hook - 避免抽搐问题
 */

import { useEffect, useState, useCallback, useRef } from 'react'
import { getKeyboardState } from '@/utils/simpleKeyboard'

interface SimpleKeyboardState {
  isOpen: boolean
  height: number
}

interface UseKeyboardOptions {
  enabled?: boolean
  debounceMs?: number
}

interface UseKeyboardResult {
  isOpen: boolean
  height: number
  isClosing: boolean
  isOpening: boolean
  state: SimpleKeyboardState
}

export const useModernKeyboard = (options: UseKeyboardOptions = {}): UseKeyboardResult => {
  const { enabled = true, debounceMs = 100 } = options
  
  const [state, setState] = useState<SimpleKeyboardState>(() => getKeyboardState())
  const [isClosing, setIsClosing] = useState(false)
  const [isOpening, setIsOpening] = useState(false)
  
  const lastHeightRef = useRef(state.height)
  const debounceTimeoutRef = useRef<number | null>(null)

  const updateState = useCallback(() => {
    if (!enabled) return

    const newState = getKeyboardState()
    
    // 检测状态变化
    const wasOpen = lastHeightRef.current > 0
    const isNowOpen = newState.height > 0
    
    if (wasOpen !== isNowOpen) {
      if (isNowOpen) {
        setIsOpening(true)
        setIsClosing(false)
        setTimeout(() => setIsOpening(false), 300)
      } else {
        setIsClosing(true)
        setIsOpening(false)
        setTimeout(() => setIsClosing(false), 300)
      }
    }
    
    lastHeightRef.current = newState.height
    setState(newState)
  }, [enabled])

  const debouncedUpdate = useCallback(() => {
    if (debounceTimeoutRef.current) {
      clearTimeout(debounceTimeoutRef.current)
    }
    
    debounceTimeoutRef.current = window.setTimeout(() => {
      updateState()
      debounceTimeoutRef.current = null
    }, debounceMs)
  }, [updateState, debounceMs])

  useEffect(() => {
    if (!enabled) return

    // 初始状态更新
    updateState()

    // 轮询检查状态变化（简化方案）
    const interval = setInterval(() => {
      debouncedUpdate()
    }, 200)

    // 监听resize事件
    const handleResize = () => {
      debouncedUpdate()
    }

    window.addEventListener('resize', handleResize)
    
    // 监听visual viewport变化
    if ('visualViewport' in window && window.visualViewport) {
      window.visualViewport.addEventListener('resize', handleResize)
    }

    return () => {
      clearInterval(interval)
      window.removeEventListener('resize', handleResize)
      
      if ('visualViewport' in window && window.visualViewport) {
        window.visualViewport.removeEventListener('resize', handleResize)
      }
      
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current)
      }
    }
  }, [enabled, debouncedUpdate, updateState])

  return {
    isOpen: state.isOpen,
    height: state.height,
    isClosing,
    isOpening,
    state
  }
}

export default useModernKeyboard 