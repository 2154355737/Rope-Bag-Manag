// JWT工具函数
export function isTokenExpired(token: string): boolean {
  try {
    // 解析JWT token
    const payload = JSON.parse(atob(token.split('.')[1]))
    const currentTime = Math.floor(Date.now() / 1000)
    
    // 检查是否过期 (exp字段是秒级时间戳)
    return payload.exp < currentTime
  } catch (error) {
    console.error('解析JWT token失败:', error)
    return true // 如果解析失败，认为token无效
  }
}

// 获取token过期时间
export function getTokenExpireTime(token: string): Date | null {
  try {
    const payload = JSON.parse(atob(token.split('.')[1]))
    return new Date(payload.exp * 1000) // 转换为毫秒级时间戳
  } catch (error) {
    console.error('解析JWT token失败:', error)
    return null
  }
}

// 检查token是否即将过期 (默认5分钟内)
export function isTokenExpiringSoon(token: string, minutesBeforeExpiry: number = 5): boolean {
  try {
    const payload = JSON.parse(atob(token.split('.')[1]))
    const currentTime = Math.floor(Date.now() / 1000)
    const expiryThreshold = payload.exp - (minutesBeforeExpiry * 60)
    
    return currentTime >= expiryThreshold
  } catch (error) {
    console.error('解析JWT token失败:', error)
    return true
  }
} 