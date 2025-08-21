import { api, ApiResponse } from '../utils/apiClient'

export interface SubscriptionRequest {
  category_id: number
  enabled: boolean
}

export interface UserSubscriptions {
  [categoryId: number]: boolean
}

export interface CategorySubscriptionStats {
  category_id: number
  category_name: string
  subscription_count: number
  subscribers: Subscriber[]
}

export interface Subscriber {
  user_id: number
  username: string
  nickname: string
  email: string
  subscribed_at: string
}

export interface NotificationRequest {
  category_id: number
  title: string
  content: string
}

export const subscriptionApi = {
  // 获取用户订阅状态
  getUserSubscriptions: (): Promise<ApiResponse<UserSubscriptions>> => {
    return api.get('/v1/subscriptions')
  },
  
  // 设置/取消资源类别订阅
  setSubscription: (data: SubscriptionRequest): Promise<ApiResponse<null>> => {
    return api.post('/v1/subscriptions/set', data)
  },

  // 管理员功能 - 获取分类订阅统计
  getCategorySubscriptions: (categoryId: number): Promise<ApiResponse<{count: number}>> => {
    return api.get(`/v1/admin/subscriptions/category/${categoryId}/stats`)
  },

  // 管理员功能 - 获取分类订阅者列表
  getCategorySubscribers: (categoryId: number): Promise<ApiResponse<{subscribers: Subscriber[]}>> => {
    return api.get(`/v1/admin/subscriptions/category/${categoryId}/subscribers`)
  },

  // 管理员功能 - 取消用户订阅
  adminUnsubscribe: (userId: number, categoryId: number): Promise<ApiResponse<null>> => {
    return api.delete(`/v1/admin/subscriptions/user/${userId}/category/${categoryId}`)
  },

  // 管理员功能 - 发送分类通知
  sendCategoryNotification: (data: NotificationRequest): Promise<ApiResponse<null>> => {
    return api.post('/v1/admin/subscriptions/notify', data)
  },

  // 管理员功能 - 导出订阅数据
  exportSubscriptions: (): Promise<ApiResponse<any>> => {
    return api.get('/v1/admin/subscriptions/export')
  },

  // 管理员功能 - 获取全部订阅统计
  getAllSubscriptionStats: (): Promise<ApiResponse<CategorySubscriptionStats[]>> => {
    return api.get('/v1/admin/subscriptions/stats')
  },

  // 管理员功能 - 切换分类订阅锁定状态
  toggleCategoryLock: (categoryId: number, locked: boolean): Promise<ApiResponse<null>> => {
    const endpoint = locked ? 'lock' : 'unlock'
    return api.post(`/v1/admin/subscriptions/category/${categoryId}/${endpoint}`)
  }
} 