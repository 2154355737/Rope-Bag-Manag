import { defHttp } from '/@/utils/http/axios'

export interface PostItem {
  id: number
  title: string
  content: string
  author_id?: number
  category_id?: number
  status: 'Draft' | 'Published' | 'Archived' | 'Deleted'
  is_pinned?: boolean
  is_featured?: boolean
  created_at?: string
  updated_at?: string
}

export interface PostListResponse {
  list: PostItem[]
  total: number
  page?: number
  page_size?: number
}

const Api = {
  Posts: '/api/v1/posts',
}

export function getPosts(params?: { page?: number; page_size?: number; category_id?: number; author_id?: number; status?: string; search?: string; is_pinned?: boolean; is_featured?: boolean }) {
  return defHttp.get<PostListResponse>({ url: Api.Posts, params })
}

export function createPost(data: Partial<PostItem> & { title: string; content: string }) {
  return defHttp.post<any>({ url: Api.Posts, data })
}

export function updatePost(id: number, data: Partial<PostItem>) {
  return defHttp.put<any>({ url: `${Api.Posts}/${id}`, data })
}

export function deletePost(id: number) {
  return defHttp.delete<any>({ url: `${Api.Posts}/${id}` })
}

export function getPostDetail(id: number) {
  return defHttp.get<PostItem>({ url: `${Api.Posts}/${id}` })
} 