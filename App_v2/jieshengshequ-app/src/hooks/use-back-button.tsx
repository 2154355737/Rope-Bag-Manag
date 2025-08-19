import { useEffect, useCallback } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { addBackButtonHandler } from '@/utils/backButton'
import { toast } from 'sonner'

interface UseBackButtonOptions {
  /**
   * 自定义返回处理函数
   * 返回 true 表示已处理，false 表示继续默认行为
   */
  onBack?: () => boolean | Promise<boolean>
  
  /**
   * 处理器优先级，数字越大优先级越高
   */
  priority?: number
  
  /**
   * 是否启用双击退出功能（仅在首页生效）
   */
  enableDoubleBackExit?: boolean
  
  /**
   * 双击退出的时间间隔（毫秒）
   */
  doubleBackExitInterval?: number
}

let lastBackPress = 0

export const useBackButton = (options: UseBackButtonOptions = {}) => {
  const navigate = useNavigate()
  const location = useLocation()
  const {
    onBack,
    priority = 10,
    enableDoubleBackExit = true,
    doubleBackExitInterval = 2000
  } = options

  const handleBack = useCallback(async (): Promise<boolean> => {
    // 如果有自定义处理函数，先执行它
    if (onBack) {
      const handled = await onBack()
      if (handled) {
        return true
      }
    }

    // 检查当前路径
    const currentPath = location.pathname
    console.log('当前路径:', currentPath)

    // 如果在首页，处理双击退出逻辑
    if (currentPath === '/' || currentPath === '/home') {
      if (enableDoubleBackExit) {
        const now = Date.now()
        if (now - lastBackPress < doubleBackExitInterval) {
          // 双击退出
          console.log('双击退出应用')
          return false // 让系统处理退出
        } else {
          // 第一次按返回键，显示提示
          lastBackPress = now
          toast.info('再按一次返回键退出应用', {
            duration: 2000,
          })
          return true // 已处理
        }
      } else {
        // 不启用双击退出，直接退出
        return false
      }
    }

    // 特殊页面处理
    const authPages = ['/login', '/register', '/forgot-password', '/terms']
    if (authPages.includes(currentPath)) {
      // 认证页面返回到首页
      navigate('/', { replace: true })
      return true
    }

    // 详情页面返回逻辑
    if (currentPath.startsWith('/post/') || 
        currentPath.startsWith('/resource/') || 
        currentPath.startsWith('/announcement/')) {
      // 返回到首页
      navigate('/', { replace: false })
      return true
    }

    // 设置相关页面返回到个人页面
    const settingsPages = ['/settings', '/help', '/about', '/privacy']
    if (settingsPages.includes(currentPath)) {
      navigate('/profile', { replace: false })
      return true
    }

    // 其他页面返回到首页
    if (currentPath !== '/' && currentPath !== '/home') {
      navigate('/', { replace: false })
      return true
    }

    // 默认行为
    return false
  }, [navigate, location, onBack, enableDoubleBackExit, doubleBackExitInterval])

  useEffect(() => {
    // 注册返回键处理器
    const removeHandler = addBackButtonHandler(handleBack, priority)

    // 清理函数
    return () => {
      removeHandler()
    }
  }, [handleBack, priority])

  return {
    /**
     * 手动触发返回逻辑
     */
    triggerBack: handleBack,
    
    /**
     * 当前路径
     */
    currentPath: location.pathname
  }
} 