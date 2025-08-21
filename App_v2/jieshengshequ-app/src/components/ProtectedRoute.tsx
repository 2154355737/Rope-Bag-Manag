import React from 'react'
import { Navigate, useLocation } from 'react-router-dom'
import { useAuth } from '@/contexts/AuthContext'
import { Loader2 } from 'lucide-react'

interface ProtectedRouteProps {
  children: React.ReactNode
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ children }) => {
  const { isAuthenticated, loading } = useAuth()
  const location = useLocation()

  // 如果还在检查认证状态，显示加载状态
  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-background">
        <div className="text-center">
          <Loader2 className="h-8 w-8 animate-spin mx-auto mb-4" />
          <p className="text-muted-foreground">检查登录状态...</p>
        </div>
      </div>
    )
  }

  // 如果未认证，重定向到登录页面
  if (!isAuthenticated) {
    return (
      <Navigate 
        to={`/login?redirect=${encodeURIComponent(location.pathname + location.search)}`} 
        replace 
      />
    )
  }

  // 如果已认证，渲染子组件
  return <>{children}</>
}

export default ProtectedRoute 