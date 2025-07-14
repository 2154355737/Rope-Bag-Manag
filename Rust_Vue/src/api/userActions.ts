import { api, ApiResponse } from '../utils/apiClient'

export interface UserAction {
  id: number
  user_id: number
  action: string
  detail: string
  created_at: string
  user_name: string
}

export const userActionApi = {
  // 获取用户行为记录
  getUserActions: (params?: { page?: number; pageSize?: number; user_id?: number; action?: string; start_date?: string; end_date?: string; search?: string }): Promise<ApiResponse<{ list: UserAction[]; total: number; page: number; size: number }>> => {
    return api.get('/api/v1/user-actions', { params })
  },
  // 记录用户行为
  logUserAction: (action: string, detail?: string): Promise<ApiResponse> => {
    return api.post('/api/v1/user-actions', { action, detail })
  },
  // 删除用户行为记录
  deleteUserAction: (actionId: number): Promise<ApiResponse> => {
    return api.delete(`/api/v1/user-actions/${actionId}`)
  },
  // 批量删除用户行为记录
  batchDeleteUserActions: (actionIds: number[]): Promise<ApiResponse> => {
    return api.post('/api/v1/user-actions/batch-delete', { ids: actionIds })
  },
  // 清空用户行为记录
  clearUserActions: (params?: { user_id?: number; before_date?: string }): Promise<ApiResponse> => {
    return api.post('/api/v1/user-actions/clear', params)
  },
  // 获取行为统计
  getActionStats: (params?: { user_id?: number; start_date?: string; end_date?: string }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/user-actions/stats', { params })
  }
} 