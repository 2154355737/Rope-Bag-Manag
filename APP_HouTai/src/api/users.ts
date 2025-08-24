import { defHttp } from '/@/utils/http/axios'

// 用户管理相关类型
export interface UserItem {
  id: number
  username: string
  email?: string
  nickname?: string
  role: string
  star?: number
  ban_status?: string
  ban_reason?: string
  created_at?: string
  last_login?: string
}

export interface UserListResponse {
  list: UserItem[]
  total: number
  page?: number
  pageSize?: number
  totalPages?: number
}

// API 路径
enum Api {
  Users = '/api/v1/users',
  UsersBatch = '/api/v1/users/batch',
}

// 获取用户列表（分页）
export function getUsers(params?: { page?: number; page_size?: number; search?: string; role?: string; ban_status?: string }) {
  return defHttp.get<UserListResponse>({ url: Api.Users, params })
}

// 创建用户
export function createUser(data: { username: string; email: string; password: string; nickname?: string; qq_number?: string; verification_code: string }) {
  return defHttp.post<any>({ url: Api.Users, data })
}

// 更新用户
export function updateUser(id: number, data: Partial<Omit<UserItem, 'id'>>) {
  return defHttp.put<any>({ url: `${Api.Users}/${id}`, data })
}

// 删除用户
export function deleteUser(id: number) {
  return defHttp.delete<any>({ url: `${Api.Users}/${id}` })
}

// 批量删除
export function deleteUsersBatch(ids: number[]) {
  return defHttp.delete<any>({ url: Api.UsersBatch, data: { ids } })
} 