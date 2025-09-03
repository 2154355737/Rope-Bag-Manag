/**
 * 应用版本信息常量
 */
export const APP_VERSION = {
  /** 应用版本号 */
  VERSION: '2.1.8',
  
  /** 构建编号 */
  BUILD_NUMBER: '218',
  
  /** 构建日期 */
  BUILD_DATE: '2025-09-04',
  
  /** 发布日期 */
  RELEASE_DATE: '2025年9月4日',
  
  /** 版本代号 */
  CODE_NAME: 'Avatar Navigation',
  
  /** API版本 */
  API_VERSION: 'v1',
  
  /** 最小支持的后端版本 */
  MIN_BACKEND_VERSION: '2.0.0'
} as const

/**
 * 版本比较工具
 */
export class VersionUtils {
  /**
   * 比较两个版本号
   * @param version1 版本号1
   * @param version2 版本号2
   * @returns 1: version1 > version2, 0: 相等, -1: version1 < version2
   */
  static compare(version1: string, version2: string): number {
    const v1Parts = version1.split('.').map(Number)
    const v2Parts = version2.split('.').map(Number)
    
    const maxLength = Math.max(v1Parts.length, v2Parts.length)
    
    for (let i = 0; i < maxLength; i++) {
      const v1Part = v1Parts[i] || 0
      const v2Part = v2Parts[i] || 0
      
      if (v1Part > v2Part) return 1
      if (v1Part < v2Part) return -1
    }
    
    return 0
  }
  
  /**
   * 检查是否为更新版本
   * @param currentVersion 当前版本
   * @param targetVersion 目标版本
   * @returns 是否为更新版本
   */
  static isNewer(currentVersion: string, targetVersion: string): boolean {
    return this.compare(targetVersion, currentVersion) > 0
  }
  
  /**
   * 格式化版本号显示
   * @param version 版本号
   * @param showPrefix 是否显示v前缀
   * @returns 格式化后的版本号
   */
  static format(version: string, showPrefix: boolean = true): string {
    return showPrefix ? `v${version}` : version
  }
}

/**
 * 获取完整的版本信息字符串
 */
export function getVersionInfo(): string {
  return `${APP_VERSION.VERSION} (${APP_VERSION.BUILD_NUMBER})`
}

/**
 * 获取版本显示名称
 */
export function getVersionDisplayName(): string {
  return `v${APP_VERSION.VERSION} "${APP_VERSION.CODE_NAME}"`
} 