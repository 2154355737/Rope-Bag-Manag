import Cookies from 'js-cookie'

// Cookie配置选项
interface CookieOptions {
  expires?: number | Date
  path?: string
  domain?: string
  secure?: boolean
  sameSite?: 'strict' | 'lax' | 'none'
  httpOnly?: boolean
}

// 默认Cookie配置
const DEFAULT_OPTIONS: CookieOptions = {
  expires: 7, // 7天过期
  path: '/',
  secure: window.location.protocol === 'https:', // 仅HTTPS时启用
  sameSite: 'lax' // CSRF保护
}

// Token专用配置（更严格的安全设置）
const TOKEN_OPTIONS: CookieOptions = {
  expires: 1, // 1天过期
  path: '/',
  secure: window.location.protocol === 'https:',
  sameSite: 'strict' // 严格的CSRF保护
}

export class CookieManager {
  
  /**
   * 设置Cookie
   */
  static set(name: string, value: string, options?: CookieOptions): void {
    const finalOptions = { ...DEFAULT_OPTIONS, ...options }
    Cookies.set(name, value, finalOptions)
  }

  /**
   * 获取Cookie
   */
  static get(name: string): string | undefined {
    return Cookies.get(name)
  }

  /**
   * 删除Cookie
   */
  static remove(name: string, options?: CookieOptions): void {
    const finalOptions = { ...DEFAULT_OPTIONS, ...options }
    Cookies.remove(name, finalOptions)
  }

  /**
   * 检查Cookie是否存在
   */
  static exists(name: string): boolean {
    return Cookies.get(name) !== undefined
  }

  /**
   * 获取所有Cookie
   */
  static getAll(): { [key: string]: string } {
    return Cookies.get()
  }

  // ==================== 认证相关的专用方法 ====================

  /**
   * 设置认证Token
   */
  static setAuthToken(token: string): void {
    this.set('auth_token', token, TOKEN_OPTIONS)
    // 同时保存到localStorage作为备选（向后兼容）
    localStorage.setItem('token', token)
  }

  /**
   * 获取认证Token
   */
  static getAuthToken(): string | null {
    // 优先从Cookie获取
    const cookieToken = this.get('auth_token')
    if (cookieToken) {
      return cookieToken
    }
    
    // 备选：从localStorage获取（向后兼容）
    return localStorage.getItem('token')
  }

  /**
   * 清除认证Token
   */
  static clearAuthToken(): void {
    this.remove('auth_token', TOKEN_OPTIONS)
    // 同时清除localStorage
    localStorage.removeItem('token')
  }

  /**
   * 设置用户信息
   */
  static setUserInfo(userInfo: any): void {
    this.set('user_info', JSON.stringify(userInfo), {
      ...DEFAULT_OPTIONS,
      expires: 7 // 用户信息保存7天
    })
    // 同时保存到localStorage（向后兼容）
    localStorage.setItem('userInfo', JSON.stringify(userInfo))
  }

  /**
   * 获取用户信息
   */
  static getUserInfo(): any | null {
    // 优先从Cookie获取
    const cookieUserInfo = this.get('user_info')
    if (cookieUserInfo) {
      try {
        return JSON.parse(cookieUserInfo)
      } catch {
        return null
      }
    }
    
    // 备选：从localStorage获取
    const localUserInfo = localStorage.getItem('userInfo')
    if (localUserInfo) {
      try {
        return JSON.parse(localUserInfo)
      } catch {
        return null
      }
    }
    
    return null
  }

  /**
   * 清除用户信息
   */
  static clearUserInfo(): void {
    this.remove('user_info')
    localStorage.removeItem('userInfo')
  }

  /**
   * 设置登录状态
   */
  static setLoginStatus(isLoggedIn: boolean): void {
    this.set('is_logged_in', isLoggedIn.toString(), DEFAULT_OPTIONS)
    localStorage.setItem('isLoggedIn', isLoggedIn.toString())
  }

  /**
   * 获取登录状态
   */
  static getLoginStatus(): boolean {
    // 优先从Cookie获取
    const cookieStatus = this.get('is_logged_in')
    if (cookieStatus) {
      return cookieStatus === 'true'
    }
    
    // 备选：从localStorage获取
    const localStatus = localStorage.getItem('isLoggedIn')
    return localStatus === 'true'
  }

  /**
   * 清除登录状态
   */
  static clearLoginStatus(): void {
    this.remove('is_logged_in')
    localStorage.removeItem('isLoggedIn')
  }

  /**
   * 完全清除所有认证相关数据
   */
  static clearAllAuthData(): void {
    this.clearAuthToken()
    this.clearUserInfo()
    this.clearLoginStatus()
    
    // 清除其他可能的认证相关数据
    this.remove('username')
    this.remove('user_role')
    localStorage.removeItem('username')
    localStorage.removeItem('userRole')
    localStorage.removeItem('loginToken')
  }

  /**
   * 设置记住我功能
   */
  static setRememberMe(remember: boolean, credentials?: { username: string, password: string }): void {
    if (remember && credentials) {
      this.set('remember_credentials', JSON.stringify(credentials), {
        expires: 30, // 30天
        path: '/',
        secure: window.location.protocol === 'https:',
        sameSite: 'strict'
      })
    } else {
      this.remove('remember_credentials')
    }
  }

  /**
   * 获取记住的凭据
   */
  static getRememberCredentials(): { username: string, password: string } | null {
    const remembered = this.get('remember_credentials')
    if (remembered) {
      try {
        return JSON.parse(remembered)
      } catch {
        return null
      }
    }
    return null
  }

  /**
   * 清除记住的凭据
   */
  static clearRememberCredentials(): void {
    this.remove('remember_credentials')
  }
}

// 默认导出
export default CookieManager 