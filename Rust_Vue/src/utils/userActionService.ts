import { userActionApi } from '../api/userActions'
import { getToken, getUserInfo } from './auth'

// 全局变量：只有/user-info验证通过后才为true
if (typeof window !== 'undefined') {
  (window as any).lastUserInfoValid = false
}

function isReallyAuthenticated(): boolean {
  const token = getToken()
  const userInfo = getUserInfo()
  // 只有token存在、userInfo存在且lastUserInfoValid为true才算真正认证
  return !!token && !!userInfo && !!userInfo.username && (typeof window !== 'undefined' ? (window as any).lastUserInfoValid : false)
}

/**
 * 用户行为记录服务
 * 用于在应用的各个部分记录用户行为，方便后续分析和统计
 */
export const userActionService = {
  /**
   * 记录用户登录行为
   * @param username 用户名
   * @param success 是否成功
   * @param details 额外详情信息
   */
  logLogin: (username: string, success: boolean, details?: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过登录行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Login',
      details || `用户 ${username} ${success ? '成功' : '失败'}登录`,
      'Auth',
      undefined
    )
  },

  /**
   * 记录用户登出行为
   * @param username 用户名
   */
  logLogout: (username: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过登出行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Logout',
      `用户 ${username} 登出系统`,
      'Auth',
      undefined
    )
  },

  /**
   * 记录用户注册行为
   * @param username 用户名
   * @param success 是否成功
   * @param details 额外详情信息
   */
  logRegister: (username: string, success: boolean, details?: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过注册行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Register',
      details || `用户 ${username} ${success ? '成功' : '失败'}注册`,
      'Auth',
      undefined
    )
  },

  /**
   * 记录资源查看行为
   * @param resourceType 资源类型，如'Package'
   * @param resourceId 资源ID
   * @param details 额外详情
   */
  logView: (resourceType: string, resourceId: number, details?: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过资源查看行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'View',
      details || `查看${resourceType}`,
      resourceType,
      resourceId
    )
  },

  /**
   * 记录资源下载行为
   * @param resourceType 资源类型，如'Package'
   * @param resourceId 资源ID
   * @param details 额外详情
   */
  logDownload: (resourceType: string, resourceId: number, details?: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过资源下载行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Download',
      details || `下载${resourceType}`,
      resourceType,
      resourceId
    )
  },

  /**
   * 记录资源上传行为
   * @param resourceType 资源类型，如'Package'
   * @param resourceId 资源ID
   * @param details 额外详情
   */
  logUpload: (resourceType: string, resourceId: number, details?: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过资源上传行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Upload',
      details || `上传${resourceType}`,
      resourceType,
      resourceId
    )
  },

  /**
   * 记录评论行为
   * @param targetType 评论目标类型，如'Package'
   * @param targetId 评论目标ID
   * @param details 额外详情
   */
  logComment: (targetType: string, targetId: number, details?: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过评论行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Comment',
      details || `评论${targetType}`,
      targetType,
      targetId
    )
  },

  /**
   * 记录搜索行为
   * @param keyword 搜索关键词
   * @param category 搜索分类
   * @param details 额外详情
   */
  logSearch: (keyword: string, category?: string, details?: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过搜索行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Search',
      details || `搜索${category ? `${category}分类下的` : ''}关键词: ${keyword}`,
      'Search',
      undefined
    )
  },

  /**
   * 记录管理员操作
   * @param actionType 操作类型
   * @param details 操作详情
   */
  logAdminAction: (actionType: string, details: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过管理员操作记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      'Admin',
      details,
      actionType,
      undefined
    )
  },

  /**
   * 记录页面访问
   * @param page 页面路径
   */
  logPageView: (page: string) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过页面访问记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    
    return userActionApi.logUserAction(
      'PageView',
      `访问页面: ${page}`,
      'Navigation',
      undefined
    )
  },

  /**
   * 通用的行为记录方法
   * @param actionType 行为类型
   * @param details 详情
   * @param targetType 目标类型
   * @param targetId 目标ID
   */
  logAction: (actionType: string, details: string, targetType?: string, targetId?: number) => {
    if (!isReallyAuthenticated()) {
      console.log('跳过行为记录：未真正认证')
      return Promise.resolve({ code: 0, message: '未认证', data: null })
    }
    return userActionApi.logUserAction(
      actionType,
      details,
      targetType,
      targetId
    )
  }
}

// 默认导出
export default userActionService 