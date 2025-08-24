import { defHttp } from '/@/utils/http/axios'

export interface CommentItem {
  id: number
  user_id?: number
  target_type?: string
  target_id?: number
  content: string
  status: 'Active' | 'Hidden' | 'Deleted'
  created_at?: string
}

export interface CommentListResponse {
  list: CommentItem[]
  total: number
  page?: number
  size?: number
}

const Api = {
  Comments: '/api/v1/comments',
  BatchStatus: '/api/v1/comments/batch-status',
  BatchDelete: '/api/v1/comments/batch-delete',
}

export function getComments(params?: { page?: number; size?: number; status?: string; target_type?: string; target_id?: number; user_id?: number; search?: string }) {
  return defHttp.get<CommentListResponse>({ url: Api.Comments, params })
}

export function deleteComment(id: number) {
  return defHttp.delete<any>({ url: `${Api.Comments}/${id}` })
}

export function updateCommentsStatus(ids: number[], status: 'Active' | 'Hidden' | 'Deleted') {
  return defHttp.post<any>({ url: Api.BatchStatus, data: { ids, status } })
}

export function deleteCommentsBatch(ids: number[]) {
  return defHttp.delete<any>({ url: Api.BatchDelete, data: { ids } })
} 