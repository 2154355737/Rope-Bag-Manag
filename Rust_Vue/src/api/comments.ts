import { api, ApiResponse } from '../utils/apiClient'

export interface Comment {
  id: number
  content: string
  user_id: number
  resource_id: number
  created_at: string
  author_name: string
}

export const commentApi = {
  // 获取资源评论
  getComments: (resourceId: number, params?: { page?: number; pageSize?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    return api.get(`/api/v1/resources/${resourceId}/comments`, { params })
  },
  // 创建评论
  createComment: (data: { content: string; resource_id: number }): Promise<ApiResponse> => {
    return api.post('/api/v1/comments', data)
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
  getAllComments: (params?: { page?: number; pageSize?: number; resource_id?: number; user_id?: number; search?: string }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    return api.get('/api/v1/comments', { params })
  }
} 