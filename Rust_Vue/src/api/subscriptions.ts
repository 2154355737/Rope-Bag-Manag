import { api, ApiResponse } from '../utils/apiClient'

export interface SubscriptionRequest {
  category_id: number
  enabled: boolean
}

export const subscriptionApi = {
  // 设置/取消资源类别订阅
  setSubscription: (data: SubscriptionRequest): Promise<ApiResponse<null>> => {
    return api.post('/v1/subscriptions', data)
  },
} 