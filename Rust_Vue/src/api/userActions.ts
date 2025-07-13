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

// 用户行为相关接口类型
export interface UserAction {
  id: string
  user_id: string
  username: string
  nickname: string
  action_type: string
  action_name: string
  target_type: string
  target_id: string
  target_name: string
  details: Record<string, any>
  ip_address: string
  user_agent: string
  create_time: string
}

export interface UserActionQuery {
  user_id?: string
  action_type?: string
  target_type?: string
  start_time?: string
  end_time?: string
  page?: number
  size?: number
}

// 获取用户行为记录
export function getUserActions(params?: UserActionQuery): Promise<ApiResponse<{
  actions: UserAction[]
  total: number
  page: number
  size: number
}>> {
  return request({
    url: '/api/user-actions',
    method: 'GET',
    params
  })
}

// 记录用户行为
export function logUserAction(data: {
  action_type: string
  action_name: string
  target_type: string
  target_id: string
  target_name: string
  details?: Record<string, any>
}): Promise<ApiResponse<UserAction>> {
  return request({
    url: '/api/user-actions',
    method: 'POST',
    data
  })
}

// 获取用户行为统计
export function getUserActionStats(params?: {
  user_id?: string
  action_type?: string
  start_time?: string
  end_time?: string
}): Promise<ApiResponse<{
  total_actions: number
  action_types: Record<string, number>
  daily_stats: Array<{
    date: string
    count: number
  }>
}>> {
  return request({
    url: '/api/user-actions/stats',
    method: 'GET',
    params
  })
}

// 删除用户行为记录
export function deleteUserAction(id: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/user-actions/${id}`,
    method: 'DELETE'
  })
}

// 批量删除用户行为记录
export function batchDeleteUserActions(ids: string[]): Promise<ApiResponse<any>> {
  return request({
    url: '/api/user-actions/batch',
    method: 'DELETE',
    data: { ids }
  })
}

// 导出用户行为记录
export function exportUserActions(params?: UserActionQuery): Promise<ApiResponse<{
  download_url: string
  filename: string
}>> {
  return request({
    url: '/api/user-actions/export',
    method: 'GET',
    params
  })
} 