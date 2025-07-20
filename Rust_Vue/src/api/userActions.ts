import { api, ApiResponse } from '../utils/apiClient'

export interface UserAction {
  id: number
  user_id: number
  action_type: string
  target_type?: string
  target_id?: number
  details?: string
  ip_address?: string
  user_agent?: string
  created_at: string
}

export interface UserActionStats {
  total_actions: number
  active_users: number
  by_day: Array<{date: string, count: number}>
  by_type: Array<{action_type: string, count: number}>
}

export const userActionApi = {
  // 获取用户行为记录
  getUserActions: (params?: { page?: number; page_size?: number; user_id?: number; action_type?: string; start_time?: string; end_time?: string }): Promise<ApiResponse<{ actions: UserAction[]; total: number; page: number; pageSize: number }>> => {
    return api.get('/api/v1/user-actions', { params })
  },
  // 记录用户行为
  logUserAction: (action_type: string, details?: string, target_type?: string, target_id?: number): Promise<ApiResponse<UserAction>> => {
    return api.post('/api/v1/user-actions', { action_type, details, target_type, target_id })
  },
  // 删除用户行为记录
  deleteUserAction: (actionId: number): Promise<ApiResponse> => {
    return api.delete(`/api/v1/user-actions/${actionId}`)
  },
  // 批量删除用户行为记录
  batchDeleteUserActions: (actionIds: number[]): Promise<ApiResponse> => {
    return api.delete('/api/v1/user-actions/batch', { data: { action_ids: actionIds } })
  },
  // 获取行为统计
  getActionStats: (params?: { user_id?: number; start_time?: string; end_time?: string }): Promise<ApiResponse<UserActionStats>> => {
    return api.get('/api/v1/user-actions/stats', { params })
  }
} 