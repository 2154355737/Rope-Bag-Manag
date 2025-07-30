// 认证相关工具函数
import apiClient, { api, type ApiResponse } from './apiClient'
import CookieManager from './cookie'

// 获取登录状态（改进版本）
export function isLoggedIn() {
  // 优先检查Token（更重要的认证标识）
  const hasToken = !!getToken()
  if (!hasToken) return false
  
  // 如果有Token，进一步检查登录状态标识
  const loginStatus = CookieManager.getLoginStatus()
  return hasToken && loginStatus
}

// 获取用户信息（改进版本）
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

// 清除登录状态
export function clearLoginStatus() {
  CookieManager.clearAllAuthData()
  localStorage.removeItem('token')
  
  // 设置标记，防止API继续调用
  if (typeof window !== 'undefined') {
    (window as any).isLoggingOut = false // 重置标记
  }
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

// 改进的用户信息检查函数
export function checkAuthStatus(): { hasToken: boolean; hasUserInfo: boolean; userInfo: any } {
  const token = getToken()
  const userInfo = getUserInfo()
  
  return {
    hasToken: !!token,
    hasUserInfo: !!userInfo,
    userInfo
  }
}

// 验证服务器端登录状态（新增）
export async function verifyServerAuth(): Promise<boolean> {
  try {
    const response = await api.get('/v1/auth/verify')
    return response.code === 0
  } catch (error) {
    console.warn('服务器认证验证失败:', error)
    return false
  }
}

// 刷新用户信息（从后端API获取最新数据）
export function refreshUserInfo() {
  // 如果没有token，直接返回null，避免不必要的API调用
  const token = getToken()
  if (!token) {
    console.log('🚫 无token，跳过用户信息获取')
    return Promise.resolve(null)
  }

  return api.get('/v1/auth/user-info')
    .then((responseData: ApiResponse) => {
      if (responseData.code === 0 && responseData.data) {
        CookieManager.setUserInfo(responseData.data)
        if (typeof window !== 'undefined') {
          (window as any).lastUserInfoValid = true
        }
        return responseData.data
      } else {
        if (typeof window !== 'undefined') {
          (window as any).lastUserInfoValid = false
        }
        CookieManager.clearAllAuthData()
        return null
      }
    })
    .catch((err: any) => {
      console.warn('获取用户信息失败:', err.response?.status)
      if (typeof window !== 'undefined') {
        (window as any).lastUserInfoValid = false
      }
      CookieManager.clearAllAuthData()
      return null
    })
}

// 应用启动时的状态恢复（新增）
export async function restoreAuthState(): Promise<{ isAuthenticated: boolean; userInfo: any }> {
  const { hasToken, hasUserInfo, userInfo } = checkAuthStatus()
  
  console.log('🔄 恢复认证状态:', { hasToken, hasUserInfo, userInfo: userInfo?.username })
  
  // 如果有Token但没有用户信息，尝试从服务器获取
  if (hasToken && !hasUserInfo) {
    console.log('📡 Token存在但用户信息缺失，尝试从服务器获取...')
    try {
      const freshUserInfo = await refreshUserInfo()
      if (freshUserInfo) {
        console.log('✅ 用户信息恢复成功:', freshUserInfo.username)
        return { isAuthenticated: true, userInfo: freshUserInfo }
      }
    } catch (error) {
      console.warn('⚠️ 用户信息恢复失败:', error)
    }
  }
  
  // 如果有完整的认证信息，信任本地状态（避免页面刷新时的认证循环）
  if (hasToken && hasUserInfo) {
    console.log('✅ 本地认证信息完整，信任本地状态')
    // 页面刷新时优先信任本地存储，避免不必要的服务器验证
    // 如果token真的无效，会在后续API调用时被服务器拒绝并自动清除
    return { isAuthenticated: true, userInfo }
  }
  
  return { isAuthenticated: hasToken && hasUserInfo, userInfo }
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
  console.log('🚪 开始退出登录...')
  
  try {
    // 调用服务器API清除HttpOnly Cookie
    const response = await fetch('/api/v1/auth/logout', {
      method: 'POST',
      credentials: 'include' // 发送Cookie
    })
    
    if (response.ok) {
      console.log('✅ 服务器Cookie已清除')
    } else {
      console.warn('⚠️ 服务器退出登录返回非200状态:', response.status)
    }
  } catch (error) {
    console.warn('⚠️ 服务器退出登录失败，但本地数据已清除:', error)
  } finally {
    // 清除所有本地认证数据
    CookieManager.clearAllAuthData()
    localStorage.removeItem('token')
    
    // 设置标记，防止API继续调用
    if (typeof window !== 'undefined') {
      (window as any).isLoggingOut = true
    }
    
    // 直接跳转到登录页，避免页面刷新导致的闪烁
    console.log('🚀 跳转到登录页')
    window.location.href = '/login'
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