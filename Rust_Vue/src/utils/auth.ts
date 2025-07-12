// 认证相关工具函数

// 检查是否已登录
export function isLoggedIn(): boolean {
  return localStorage.getItem('isLoggedIn') === 'true'
}

// 获取用户信息
export function getUserInfo() {
  const userInfo = localStorage.getItem('userInfo')
  if (userInfo) {
    try {
      return JSON.parse(userInfo)
    } catch (e) {
      console.error('解析用户信息失败:', e)
      return null
    }
  }
  return null
}

// 设置登录状态
export function setLoginStatus(username: string) {
  localStorage.setItem('isLoggedIn', 'true')
  localStorage.setItem('userInfo', JSON.stringify({
    username,
    loginTime: new Date().toISOString()
  }))
}

// 清除登录状态
export function clearLoginStatus() {
  localStorage.removeItem('isLoggedIn')
  localStorage.removeItem('userInfo')
}

// 退出登录
export function logout() {
  clearLoginStatus()
}

// 检查登录是否过期（可选功能）
export function isLoginExpired(): boolean {
  const userInfo = getUserInfo()
  if (!userInfo || !userInfo.loginTime) {
    return true
  }
  
  const loginTime = new Date(userInfo.loginTime)
  const now = new Date()
  const diffHours = (now.getTime() - loginTime.getTime()) / (1000 * 60 * 60)
  
  // 24小时后过期
  return diffHours > 24
} 