import { userApi, statsApi } from '../api'
import { announcementApi } from '../api/announcements'
import { commentApi } from '../api/comments'
import { settingsApi } from '../api/settings'
import { backupApi } from '../api/backupRecords'
import { userActionApi } from '../api/userActions'
import { resourceRecordApi } from '../api/resourceRecords'
import { communityApi } from '../api/community'

// API客户端工具
export const apiClient = {
  // 用户相关API
  user: userApi,
  
  // 统计相关API
  stats: statsApi,
  
  // 公告相关API
  announcement: announcementApi,
  
  // 评论相关API
  comment: commentApi,
  
  // 设置相关API
  settings: settingsApi,
  
  // 备份相关API
  backup: backupApi,
  
  // 用户行为相关API
  userAction: userActionApi,
  
  // 资源记录相关API
  resourceRecord: resourceRecordApi,
  
  // 社区相关API
  community: communityApi,
  
  // 通用方法
  utils: {
    // 获取当前用户信息
    getCurrentUser: () => {
      const userInfo = localStorage.getItem('userInfo')
      if (userInfo) {
        try {
          return JSON.parse(userInfo)
        } catch (e) {
          console.warn('解析用户信息失败')
          return null
        }
      }
      return null
    },
    
    // 设置用户信息
    setCurrentUser: (userInfo: any) => {
      localStorage.setItem('userInfo', JSON.stringify(userInfo))
    },
    
    // 清除用户信息
    clearCurrentUser: () => {
      localStorage.removeItem('userInfo')
    },
    
    // 检查用户是否已登录
    isLoggedIn: () => {
      return !!localStorage.getItem('userInfo')
    },
    
    // 检查用户权限
    hasPermission: (permission: string) => {
      const user = apiClient.utils.getCurrentUser()
      if (!user) return false
      
      // 管理员拥有所有权限
      if (user.role === 'admin') return true
      
      // 版主拥有部分权限
      if (user.role === 'moderator') {
        const moderatorPermissions = [
          'manage_announcements',
          'manage_comments',
          'view_logs',
          'manage_users'
        ]
        return moderatorPermissions.includes(permission)
      }
      
      // 普通用户权限
      if (user.role === 'user') {
        const userPermissions = [
          'view_resources',
          'create_resources',
          'edit_own_resources',
          'delete_own_resources',
          'create_comments',
          'delete_own_comments'
        ]
        return userPermissions.includes(permission)
      }
      
      return false
    },
    
    // 记录用户行为
    logAction: async (action: string, detail?: string) => {
      try {
        await userActionApi.logUserAction(action, detail)
      } catch (error) {
        console.warn('记录用户行为失败:', error)
      }
    },
    
    // 记录资源操作
    logResourceAction: async (resourceId: number, action: string) => {
      try {
        await resourceRecordApi.logResourceAction(resourceId, action)
      } catch (error) {
        console.warn('记录资源操作失败:', error)
      }
    }
  }
}

// 导出所有API
export {
  userApi,
  statsApi,
  announcementApi,
  commentApi,
  settingsApi,
  backupApi,
  userActionApi,
  resourceRecordApi,
  communityApi
}

// 默认导出API客户端
export default apiClient 