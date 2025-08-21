import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { User, getLocalUser, isLoggedIn, logout as logoutAPI } from '@/api/auth'

interface AuthState {
  isAuthenticated: boolean
  user: User | null
  loading: boolean
}

interface AuthContextType extends AuthState {
  login: (user: User, token: string) => void
  logout: () => Promise<void>
  updateUser: (user: User) => void
  checkAuth: () => void
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export const useAuth = () => {
  const context = useContext(AuthContext)
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}

interface AuthProviderProps {
  children: ReactNode
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [authState, setAuthState] = useState<AuthState>({
    isAuthenticated: false,
    user: null,
    loading: true
  })

  // 检查认证状态
  const checkAuth = () => {
    const authenticated = isLoggedIn()
    const user = getLocalUser()
    
    setAuthState({
      isAuthenticated: authenticated,
      user: user,
      loading: false
    })
  }

  // 初始化时检查认证状态
  useEffect(() => {
    checkAuth()
  }, [])

  // 登录
  const login = (user: User, token: string) => {
    setAuthState({
      isAuthenticated: true,
      user: user,
      loading: false
    })
  }

  // 退出登录
  const logout = async () => {
    try {
      await logoutAPI()
    } catch (error) {
      console.error('Logout API error:', error)
    } finally {
      setAuthState({
        isAuthenticated: false,
        user: null,
        loading: false
      })
    }
  }

  // 更新用户信息
  const updateUser = (user: User) => {
    localStorage.setItem('user', JSON.stringify(user))
    setAuthState(prev => ({
      ...prev,
      user: user
    }))
  }

  const value: AuthContextType = {
    ...authState,
    login,
    logout,
    updateUser,
    checkAuth
  }

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  )
} 