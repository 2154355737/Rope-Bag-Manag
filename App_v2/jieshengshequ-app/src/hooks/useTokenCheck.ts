import { useEffect } from 'react'
import { useAuth } from '@/contexts/AuthContext'
import { getToken } from '@/api/client'
import { isTokenExpired, isTokenExpiringSoon } from '@/utils/jwt'
import { toast } from '@/hooks/use-toast'

// Token检查hook
export function useTokenCheck() {
  const { logout, isAuthenticated } = useAuth()

  useEffect(() => {
    if (!isAuthenticated) return

    const checkToken = () => {
      const token = localStorage.getItem('token')
      if (!token) return

      try {
        // 检查token是否已过期
        if (isTokenExpired(token)) {
          console.warn('Token已过期，自动退出登录')
          toast({
            title: "登录已过期",
            description: "您的登录已过期，请重新登录",
            variant: "destructive"
          })
          logout()
          return
        }

        // 检查token是否即将过期（5分钟内）
        if (isTokenExpiringSoon(token, 5)) {
          toast({
            title: "登录即将过期",
            description: "您的登录将在5分钟内过期，建议重新登录",
            variant: "default"
          })
        }
      } catch (error) {
        console.error('Token检查失败:', error)
        logout()
      }
    }

    // 立即检查一次
    checkToken()

    // 每分钟检查一次token状态
    const interval = setInterval(checkToken, 60 * 1000)

    return () => clearInterval(interval)
  }, [isAuthenticated, logout])
}

export default useTokenCheck 