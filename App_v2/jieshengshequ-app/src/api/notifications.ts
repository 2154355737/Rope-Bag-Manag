import { http } from './client'

export interface Notification {
  id: number
  user_id: number
  title: string
  content: string
  link?: string
  notif_type?: string
  related_type?: string
  related_id?: number
  is_read: boolean
  created_at: string
}

export interface NotificationQuery {
  page?: number
  size?: number
}

// 获取通知列表
export async function getNotifications(params?: NotificationQuery): Promise<{
  list: Notification[]
}> {
  return http.get<{ list: Notification[] }>('/notifications', params)
}

// 获取未读通知数量
export async function getUnreadCount(): Promise<{ count: number }> {
  return http.get<{ count: number }>('/notifications/unread-count')
}

// 标记单个通知为已读
export async function markAsRead(id: number): Promise<void> {
  return http.post(`/notifications/${id}/read`)
}

// 标记所有通知为已读
export async function markAllAsRead(): Promise<void> {
  return http.post('/notifications/mark-all-read')
}

// 删除单个通知（如果后端支持）
export async function deleteNotification(id: number): Promise<void> {
  return http.delete(`/notifications/${id}`)
}

// 批量删除已读通知
export async function deleteReadNotifications(): Promise<{ deleted_count: number }> {
  return http.delete<{ deleted_count: number }>('/notifications/delete-read')
} 