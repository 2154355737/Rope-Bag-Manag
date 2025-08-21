import { api, ApiResponse } from '../utils/apiClient'

export interface Comment {
  id: number
  user_id: number
  target_type: string
  target_id: number
  content: string
  status: string
  parent_id?: number
  likes: number
  dislikes: number
  pinned: boolean
  created_at: string
  updated_at: string
  author_name?: string
  username?: string
  author_role?: string
  author_avatar?: string
  author_qq?: string
  target_title?: string
}

export const commentApi = {
  // 获取资源评论
  getComments: (resourceId: number, params?: { page?: number; pageSize?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    return api.get(`/v1/resources/${resourceId}/comments`, { params })
  },

  // 获取帖子评论
  getPostComments: (postId: number, params?: { page?: number; size?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    const query = {
      target_type: 'Post',
      target_id: postId,
      ...(params || {})
    }
    return api.get('/v1/comments', { params: query })
  },
  // 创建评论
  createComment: (data: { content: string; target_id: number; target_type: string; parent_id?: number }): Promise<ApiResponse<Comment>> => {
    return api.post('/v1/comments', data)
  },
  // 更新评论
  updateComment: (commentId: number, data: { content?: string; status?: string }): Promise<ApiResponse<Comment>> => {
    return api.put(`/v1/comments/${commentId}`, data)
  },
  // 删除评论
  deleteComment: (commentId: number): Promise<ApiResponse> => {
    return api.delete(`/v1/comments/${commentId}`)
  },
  // 批量删除
  batchDeleteComments: (commentIds: number[]): Promise<ApiResponse> => {
    return api.post('/v1/comments/batch-delete', { ids: commentIds })
  },
  // 批量更新评论状态
  batchUpdateStatus: (commentIds: number[], status: 'Active' | 'Hidden' | 'Deleted'): Promise<ApiResponse> => {
    return api.put('/v1/comments/batch/status', { ids: commentIds, status })
  },
  // 管理员获取所有评论
  getAllComments: (params?: { page?: number; pageSize?: number; resource_id?: number; user_id?: number; search?: string; status?: string; target_type?: string; start_date?: Date; end_date?: Date }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    return api.get('/v1/comments', { params })
  },
  // 获取资源评论
  getPackageComments: (packageId: number, params?: { page?: number; size?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    // 过滤掉空字符串参数
    const cleanParams: { [key: string]: any } = { ...params }
    Object.keys(cleanParams).forEach(key => {
      if (cleanParams[key] === '') {
        delete cleanParams[key]
      }
    })
    return api.get(`/v1/packages/${packageId}/comments`, { params: cleanParams })
  },
  // 获取用户评论
  getUserComments: (userId: number, params?: { page?: number; size?: number }): Promise<ApiResponse<{ list: Comment[]; total: number; page: number; size: number }>> => {
    // 过滤掉空字符串参数
    const cleanParams: { [key: string]: any } = { ...params }
    Object.keys(cleanParams).forEach(key => {
      if (cleanParams[key] === '') {
        delete cleanParams[key]
      }
    })
    return api.get(`/v1/users/${userId}/comments`, { params: cleanParams })
  },
  // 点赞/取消点赞评论
  likeComment: (commentId: number, like: boolean = true): Promise<ApiResponse<number>> => {
    return api.post(`/v1/comments/${commentId}/like`, { like })
  },
  // 回复评论
  replyComment: (commentId: number, content: string): Promise<ApiResponse<Comment>> => {
    return api.post(`/v1/comments/${commentId}/reply`, { content })
  },

  // 置顶/取消置顶评论
  pinComment: (commentId: number, pinned: boolean): Promise<ApiResponse<Comment>> => {
    return api.put(`/v1/comments/${commentId}/pin`, { pinned })
  }
} 