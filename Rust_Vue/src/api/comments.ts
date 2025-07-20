import { api, ApiResponse } from '../utils/apiClient'

export interface Comment {
  id: number
  content: string
  user_id: number
  target_id: number
  target_type: string
  created_at: string
  updated_at: string
  likes: number
  dislikes: number
  status: string
  author_name?: string
  resource_id?: number // 兼容旧字段
}

export const commentApi = {
  // 获取资源评论
  getComments: (resourceId: number, params?: { page?: number; pageSize?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    return api.get(`/api/v1/resources/${resourceId}/comments`, { params })
  },
  // 创建评论
  createComment: (data: { content: string; target_id: number; target_type: string; parent_id?: number }): Promise<ApiResponse<Comment>> => {
    return api.post('/api/v1/comments', data)
  },
  // 更新评论
  updateComment: (commentId: number, data: { content?: string; status?: string }): Promise<ApiResponse<Comment>> => {
    return api.put(`/api/v1/comments/${commentId}`, data)
  },
  // 删除评论
  deleteComment: (commentId: number): Promise<ApiResponse> => {
    return api.delete(`/api/v1/comments/${commentId}`)
  },
  // 批量删除
  batchDeleteComments: (commentIds: number[]): Promise<ApiResponse> => {
    return api.post('/api/v1/comments/batch-delete', { ids: commentIds })
  },
  // 管理员获取所有评论
  getAllComments: (params?: { page?: number; pageSize?: number; resource_id?: number; user_id?: number; search?: string; status?: string; target_type?: string; start_date?: Date; end_date?: Date }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    return api.get('/api/v1/comments', { params })
  },
  // 获取资源评论
  getPackageComments: (packageId: number, params?: { page?: number; size?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    // 过滤掉空字符串参数
    const cleanParams = { ...params }
    Object.keys(cleanParams).forEach(key => {
      if (cleanParams[key] === '') {
        delete cleanParams[key]
      }
    })
    return api.get(`/api/v1/packages/${packageId}/comments`, { params: cleanParams })
  },
  // 获取用户评论
  getUserComments: (userId: number, params?: { page?: number; size?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    // 过滤掉空字符串参数
    const cleanParams = { ...params }
    Object.keys(cleanParams).forEach(key => {
      if (cleanParams[key] === '') {
        delete cleanParams[key]
      }
    })
    return api.get(`/api/v1/users/${userId}/comments`, { params: cleanParams })
  }
} 