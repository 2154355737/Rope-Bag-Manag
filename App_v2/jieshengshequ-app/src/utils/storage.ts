/**
 * 本地存储工具类
 * 用于管理应用状态和用户偏好
 */

const STORAGE_KEYS = {
  FIRST_LAUNCH: 'jieshengshequ_first_launch',
  USER_PREFERENCES: 'jieshengshequ_user_preferences',
  THEME: 'jieshengshequ_theme',
} as const

export class StorageManager {
  /**
   * 检查是否是首次启动
   */
  static isFirstLaunch(): boolean {
    try {
      const hasLaunched = localStorage.getItem(STORAGE_KEYS.FIRST_LAUNCH)
      return hasLaunched === null
    } catch (error) {
      console.warn('无法读取localStorage，默认为首次启动:', error)
      return true
    }
  }

  /**
   * 标记应用已启动过
   */
  static markAsLaunched(): void {
    try {
      localStorage.setItem(STORAGE_KEYS.FIRST_LAUNCH, 'false')
      localStorage.setItem('jieshengshequ_launch_time', new Date().toISOString())
    } catch (error) {
      console.warn('无法写入localStorage:', error)
    }
  }

  /**
   * 重置首次启动状态（用于测试）
   */
  static resetFirstLaunch(): void {
    try {
      localStorage.removeItem(STORAGE_KEYS.FIRST_LAUNCH)
      localStorage.removeItem('jieshengshequ_launch_time')
    } catch (error) {
      console.warn('无法删除localStorage数据:', error)
    }
  }

  /**
   * 获取启动次数
   */
  static getLaunchCount(): number {
    try {
      const count = localStorage.getItem('jieshengshequ_launch_count')
      return count ? parseInt(count, 10) : 0
    } catch (error) {
      console.warn('无法读取启动次数:', error)
      return 0
    }
  }

  /**
   * 增加启动次数
   */
  static incrementLaunchCount(): number {
    try {
      const count = this.getLaunchCount() + 1
      localStorage.setItem('jieshengshequ_launch_count', count.toString())
      return count
    } catch (error) {
      console.warn('无法更新启动次数:', error)
      return 1
    }
  }

  /**
   * 保存用户偏好设置
   */
  static saveUserPreferences(preferences: Record<string, any>): void {
    try {
      const existing = this.getUserPreferences()
      const updated = { ...existing, ...preferences }
      localStorage.setItem(STORAGE_KEYS.USER_PREFERENCES, JSON.stringify(updated))
    } catch (error) {
      console.warn('无法保存用户偏好:', error)
    }
  }

  /**
   * 获取用户偏好设置
   */
  static getUserPreferences(): Record<string, any> {
    try {
      const preferences = localStorage.getItem(STORAGE_KEYS.USER_PREFERENCES)
      return preferences ? JSON.parse(preferences) : {}
    } catch (error) {
      console.warn('无法读取用户偏好:', error)
      return {}
    }
  }

  /**
   * 保存主题设置
   */
  static saveTheme(theme: string): void {
    try {
      localStorage.setItem(STORAGE_KEYS.THEME, theme)
    } catch (error) {
      console.warn('无法保存主题设置:', error)
    }
  }

  /**
   * 获取主题设置
   */
  static getTheme(): string | null {
    try {
      return localStorage.getItem(STORAGE_KEYS.THEME)
    } catch (error) {
      console.warn('无法读取主题设置:', error)
      return null
    }
  }

  /**
   * 清除所有应用数据（用于登出等场景）
   */
  static clearAppData(): void {
    try {
      Object.values(STORAGE_KEYS).forEach(key => {
        localStorage.removeItem(key)
      })
      // 保留首次启动标记
      this.markAsLaunched()
    } catch (error) {
      console.warn('无法清除应用数据:', error)
    }
  }
}

export default StorageManager 