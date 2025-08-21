import { http } from './client'

export interface Message {
  id: number
  sender_id: number
  receiver_id: number
  content: string
  message_type: 'text' | 'image' | 'file'
  is_read: boolean
  created_at: string
  sender?: {
    id: number
    username: string
    nickname?: string
    avatar?: string
  }
  receiver?: {
    id: number
    username: string
    nickname?: string
    avatar?: string
  }
}

export interface Conversation {
  id: number
  participant_id: number
  participant_username: string
  participant_nickname?: string
  participant_avatar?: string
  last_message?: string
  last_message_time?: string
  unread_count: number
  is_pinned: boolean
  is_online: boolean
  created_at: string
}

export interface SendMessageRequest {
  receiver_id: number
  content: string
  message_type?: 'text' | 'image' | 'file'
}

// 获取对话列表
export async function getConversations(): Promise<Conversation[]> {
  return http.get<Conversation[]>('/messages/conversations')
}

// 获取与指定用户的消息历史
export async function getMessages(userId: number, page = 1, pageSize = 20): Promise<{
  list: Message[]
  total: number
  page: number
  page_size: number
}> {
  return http.get('/messages', { user_id: userId, page, page_size: pageSize })
}

// 发送消息
export async function sendMessage(data: SendMessageRequest): Promise<Message> {
  return http.post<Message>('/messages', data)
}

// 标记消息为已读
export async function markAsRead(messageIds: number[]): Promise<void> {
  return http.post('/messages/read', { message_ids: messageIds })
}

// 标记与指定用户的所有消息为已读
export async function markConversationAsRead(userId: number): Promise<void> {
  return http.post('/messages/read-conversation', { user_id: userId })
}

// 删除消息
export async function deleteMessage(messageId: number): Promise<void> {
  return http.delete(`/messages/${messageId}`)
}

// 删除对话（删除与指定用户的所有消息）
export async function deleteConversation(userId: number): Promise<void> {
  return http.delete(`/messages/conversation/${userId}`)
}

// 置顶/取消置顶对话
export async function togglePinConversation(userId: number, pinned: boolean): Promise<void> {
  return http.post('/messages/pin-conversation', { user_id: userId, pinned })
}

// 获取未读消息数量
export async function getUnreadCount(): Promise<{ count: number }> {
  return http.get<{ count: number }>('/messages/unread-count')
}

// 搜索对话
export async function searchConversations(query: string): Promise<Conversation[]> {
  return http.get<Conversation[]>('/messages/search-conversations', { q: query })
}

// 搜索消息
export async function searchMessages(query: string, userId?: number): Promise<Message[]> {
  return http.get<Message[]>('/messages/search', { q: query, user_id: userId })
} 