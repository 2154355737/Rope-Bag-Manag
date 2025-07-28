// 认证相关工具函数
import apiClient from './apiClient'
import CookieManager from './cookie'

// 获取登录状态
export function isLoggedIn() {
  // 优先检查Cookie中的Token（服务器设置的HttpOnly）
  const hasToken = !!getToken()
  return hasToken && CookieManager.getLoginStatus()
}

// 获取用户信息
export function getUserInfo() {
  return CookieManager.getUserInfo()
}

// 获取Token（支持多种来源）
export function getToken() {
  // 1. 优先从Authorization头获取（localStorage）
  const localToken = localStorage.getItem('token')
  if (localToken) return localToken
  
  // 2. 从非HttpOnly Cookie获取（向后兼容）
  return CookieManager.getAuthToken()
}

// 设置Token（前端仍需保存用于Authorization头）
export function setToken(token: string) {
  // 保存到localStorage用于Authorization头
  localStorage.setItem('token', token)
  // 注意：HttpOnly Cookie由服务器自动设置，前端不需要操作
}

export function clearToken() {
  localStorage.removeItem('token')
  CookieManager.clearAuthToken()
}

// 设置登录状态（保存完整用户信息）
export function setLoginStatus(username: string, token?: string, fullUserInfo?: any) {
  CookieManager.setLoginStatus(true)
  CookieManager.set('username', username)
  
  if (token) {
    // 保存到localStorage用于Authorization头
    localStorage.setItem('token', token)
  }
  
  // 保存完整用户信息
  const userInfo = fullUserInfo ? {
    ...fullUserInfo,
    loginTime: new Date().toISOString()
  } : {
    username,
    loginTime: new Date().toISOString()
  }
  CookieManager.setUserInfo(userInfo)
}

// 清除登录状态
export function clearLoginStatus() {
  CookieManager.clearLoginStatus()
  CookieManager.clearUserInfo()
  localStorage.removeItem('token')
}

// 刷新用户信息（从后端API获取最新数据）
export function refreshUserInfo() {
  return apiClient.get('/v1/auth/user-info')
    .then(res => {
      if (res.code === 0 && res.data) {
        CookieManager.setUserInfo(res.data)
        if (typeof window !== 'undefined') {
          (window as any).lastUserInfoValid = true
        }
        return res.data
      } else {
        if (typeof window !== 'undefined') {
          (window as any).lastUserInfoValid = false
        }
        CookieManager.clearAllAuthData()
        return null
      }
    })
    .catch(err => {
      if (typeof window !== 'undefined') {
        (window as any).lastUserInfoValid = false
      }
      CookieManager.clearAllAuthData()
      return null
    })
}

// 检查登录是否过期
export function isLoginExpired(): boolean {
  const userInfo = getUserInfo()
  if (!userInfo?.loginTime) return true
  
  const loginTime = new Date(userInfo.loginTime)
  const now = new Date()
  const expireTime = 24 * 60 * 60 * 1000 // 24小时
  
  return now.getTime() - loginTime.getTime() > expireTime
}

// 安全退出登录（调用服务器API清除HttpOnly Cookie）
export async function logout() {
  try {
    // 调用服务器API清除HttpOnly Cookie
    const response = await fetch('/api/v1/auth/logout', {
      method: 'POST',
      credentials: 'include' // 发送Cookie
    })
    
    if (response.ok) {
      console.log('服务器Cookie已清除')
    }
  } catch (error) {
    console.warn('服务器退出登录失败，仅清除本地数据:', error)
  } finally {
    // 清除所有本地认证数据
    CookieManager.clearAllAuthData()
    localStorage.removeItem('token')
  }
}

// 记住我功能
export function setRememberMe(remember: boolean, credentials?: { username: string, password: string }) {
  CookieManager.setRememberMe(remember, credentials)
}

// 获取记住的凭据
export function getRememberCredentials() {
  return CookieManager.getRememberCredentials()
} 