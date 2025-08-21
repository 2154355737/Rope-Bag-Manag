import { useCallback } from 'react'

/**
 * 自定义Hook：处理按钮点击后自动失焦
 * 用于解决按钮点击后保持焦点高亮的问题
 */
export const useAutoBlur = () => {
  const handleClick = useCallback((callback?: () => void) => {
    return (e: React.MouseEvent<HTMLButtonElement>) => {
      // 执行回调函数
      if (callback) {
        callback()
      }
      
      // 自动失去焦点，避免高亮残留
      setTimeout(() => {
        e.currentTarget.blur()
      }, 0)
    }
  }, [])

  const handleClickWithEvent = useCallback((callback?: (e: React.MouseEvent<HTMLButtonElement>) => void) => {
    return (e: React.MouseEvent<HTMLButtonElement>) => {
      // 执行回调函数
      if (callback) {
        callback(e)
      }
      
      // 自动失去焦点，避免高亮残留
      setTimeout(() => {
        e.currentTarget.blur()
      }, 0)
    }
  }, [])

  return { handleClick, handleClickWithEvent }
}

export default useAutoBlur 