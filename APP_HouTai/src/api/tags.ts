import { defHttp } from '/@/utils/http/axios'

export interface TagItem {
  id: number
  name: string
  description?: string
  use_count?: number
  created_at?: string
}

export interface TagListResponse {
  list: TagItem[]
  total: number
  page?: number
  page_size?: number
}

const Api = {
  Tags: '/api/v1/tags',
}

export function getTags(params?: { page?: number; page_size?: number; search?: string; sort_by?: string; sort_order?: string }) {
  return defHttp.get<TagListResponse>({ url: Api.Tags, params })
}

export function createTag(data: { name: string; description?: string }) {
  return defHttp.post<any>({ url: Api.Tags, data })
}

export function updateTag(id: number, data: { name?: string; description?: string }) {
  return defHttp.put<any>({ url: `${Api.Tags}/${id}`, data })
}

export function deleteTag(id: number) {
  return defHttp.delete<any>({ url: `${Api.Tags}/${id}` })
} 