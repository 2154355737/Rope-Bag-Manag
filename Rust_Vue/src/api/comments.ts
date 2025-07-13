import axios from 'axios'
import type { ApiResponse } from './types'

// 创建API客户端
const api = axios.create({
  baseURL: 'http://127.0.0.1:15202',
  timeout: 15000,
})

// 通用请求函数
const request = async (config: any) => {
  try {
    const response = await api(config)
    return response.data
  } catch (error) {
    console.error('API请求失败:', error)
    throw error
  }
}

// 评论相关接口类型
export interface Comment {
  id: string
  user_id: string
  username: string
  nickname: string
  content: string
  resource_id: string
  resource_name: string
  create_time: string
  update_time: string
  status: string
  likes: number
  replies: Comment[]
}

export interface CommentForm {
  content: string
  resource_id: string
  parent_id?: string
}

// 获取评论列表
export function getComments(params?: {
  resource_id?: string
  user_id?: string
  status?: string
  page?: number
  size?: number
}): Promise<ApiResponse<{
  comments: Comment[]
  total: number
  page: number
  size: number
}>> {
  return request({
    url: '/api/comments',
    method: 'GET',
    params
  })
}

// 添加评论
export function addComment(data: CommentForm): Promise<ApiResponse<Comment>> {
  return request({
    url: '/api/comments',
    method: 'POST',
    data
  })
}

// 更新评论
export function updateComment(id: string, data: Partial<CommentForm>): Promise<ApiResponse<Comment>> {
  return request({
    url: `/api/comments/${id}`,
    method: 'PUT',
    data
  })
}

// 删除评论
export function deleteComment(id: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/comments/${id}`,
    method: 'DELETE'
  })
}

// 审核评论
export function reviewComment(id: string, status: string, reason?: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/comments/${id}/review`,
    method: 'POST',
    data: { status, reason }
  })
}

// 点赞评论
export function likeComment(id: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/comments/${id}/like`,
    method: 'POST'
  })
}

// 回复评论
export function replyComment(id: string, data: CommentForm): Promise<ApiResponse<Comment>> {
  return request({
    url: `/api/comments/${id}/reply`,
    method: 'POST',
    data
  })
} 