import { api } from '../utils/apiClient'

/**
 * 公告类型定义
 */
export interface Announcement {
  id: number
  title: string
  content: string
  type_: string  // 'Info' | 'Warning' | 'Error' | 'Success'
  priority: number
  enabled: boolean
  start_time: string
  end_time?: string
  created_at: string
  updated_at: string
}

/**
 * 获取公告列表
 * @param params 分页参数
 */
export function getAnnouncements(params: { page?: number; size?: number }) {
  return api.get('/v1/admin/announcements', {
    params: {
      page: params.page || 1,
      page_size: params.size || 20
    }
  })
}

/**
 * 创建新公告
 * @param data 公告数据
 */
export function createAnnouncement(data: {
  title: string
  content: string
  type_: string
  priority: number
  enabled: boolean
  start_time: string
  end_time?: string
}) {
  return api.post('/v1/admin/announcements', data)
}

/**
 * 更新公告
 * @param id 公告ID
 * @param data 更新的公告数据
 */
export function updateAnnouncement(id: number, data: {
  title?: string
  content?: string
  type_?: string
  priority?: number
  enabled?: boolean
  start_time?: string
  end_time?: string
}) {
  return api.put(`/v1/admin/announcements/${id}`, data)
}

/**
 * 删除公告
 * @param id 公告ID
 */
export function deleteAnnouncement(id: number) {
  return api.delete(`/v1/admin/announcements/${id}`)
}

/**
 * 批量启用/禁用公告
 * @param ids 公告ID数组
 * @param enabled 是否启用
 */
export function batchUpdateAnnouncementStatus(ids: number[], enabled: boolean) {
  return api.put('/v1/admin/announcements/batch-status', {
    ids,
    enabled
  })
}

/**
 * 批量删除公告
 * @param ids 公告ID数组
 */
export function batchDeleteAnnouncements(ids: number[]) {
  return api.post('/v1/admin/announcements/batch-delete', {
    ids
  })
}

/**
 * 获取当前有效的公告
 */
export function getActiveAnnouncements() {
  return api.get('/v1/announcements/active')
}

// Export default object for convenience
export const announcementApi = {
  getAnnouncements,
  createAnnouncement,
  updateAnnouncement,
  deleteAnnouncement,
  batchUpdateAnnouncementStatus,
  batchDeleteAnnouncements,
  getActiveAnnouncements
}

export default announcementApi 