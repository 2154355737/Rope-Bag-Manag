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

// 公告相关接口类型
export interface Announcement {
  id: string
  title: string
  content: string
  type_: string
  priority: number
  status: string
  start_time: string
  end_time: string
  created_by: string
  created_time: string
  updated_time: string
  view_count: number
  is_global: boolean
}

export interface AnnouncementForm {
  title: string
  content: string
  type_: string
  priority: number
  start_time: string
  end_time: string
  is_global: boolean
}

export interface AnnouncementQuery {
  type_?: string
  status?: string
  is_global?: boolean
  created_by?: string
  page?: number
  size?: number
}

// 获取公告列表
export function getAnnouncements(params?: AnnouncementQuery): Promise<ApiResponse<{
  announcements: Announcement[]
  total: number
  page: number
  size: number
}>> {
  return request({
    url: '/api/announcements',
    method: 'GET',
    params
  })
}

// 获取单个公告
export function getAnnouncement(id: string): Promise<ApiResponse<Announcement>> {
  return request({
    url: `/api/announcements/${id}`,
    method: 'GET'
  })
}

// 创建公告
export function createAnnouncement(data: AnnouncementForm): Promise<ApiResponse<Announcement>> {
  return request({
    url: '/api/announcements',
    method: 'POST',
    data
  })
}

// 更新公告
export function updateAnnouncement(id: string, data: Partial<AnnouncementForm>): Promise<ApiResponse<Announcement>> {
  return request({
    url: `/api/announcements/${id}`,
    method: 'PUT',
    data
  })
}

// 删除公告
export function deleteAnnouncement(id: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/announcements/${id}`,
    method: 'DELETE'
  })
}

// 批量删除公告
export function batchDeleteAnnouncements(ids: string[]): Promise<ApiResponse<any>> {
  return request({
    url: '/api/announcements/batch',
    method: 'DELETE',
    data: { ids }
  })
}

// 发布/取消发布公告
export function toggleAnnouncementStatus(id: string, status: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/announcements/${id}/status`,
    method: 'PUT',
    data: { status }
  })
}

// 获取活跃公告
export function getActiveAnnouncements(): Promise<ApiResponse<Announcement[]>> {
  return request({
    url: '/api/announcements/active',
    method: 'GET'
  })
}

// 获取公告统计
export function getAnnouncementStats(): Promise<ApiResponse<{
  total_announcements: number
  active_announcements: number
  global_announcements: number
  announcement_types: Record<string, number>
  daily_stats: Array<{
    date: string
    count: number
  }>
}>> {
  return request({
    url: '/api/announcements/stats',
    method: 'GET'
  })
} 