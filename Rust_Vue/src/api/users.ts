import { api, ApiResponse } from '../utils/apiClient'

// 用户信息类型
export interface User {
  id: number
  username: string
  email: string
  role: string
  status: number
  created_at: string
  updated_at: string
}

// 用户列表查询参数
export interface UserQueryParams {
  page?: number
  pageSize?: number
  role?: string
  status?: string
  search?: string
}

// 用户列表响应
export interface UserListResponse {
  list: User[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

// 更新用户请求
export interface UpdateUserRequest {
  role?: string
  status?: number
}

// 用户API
export const userApi = {
  // 获取用户列表
  getUsers: (params?: UserQueryParams): Promise<ApiResponse<UserListResponse>> => {
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())
    if (params?.role) queryParams.append('role', params.role)
    if (params?.status) queryParams.append('status', params.status)
    if (params?.search) queryParams.append('search', params.search)

    return api.get(`/api/v1/users?${queryParams.toString()}`)
  },

  // 获取单个用户
  getUser: (id: number): Promise<ApiResponse<User>> => {
    return api.get(`/api/v1/users/${id}`)
  },

  // 更新用户
  updateUser: (id: number, data: UpdateUserRequest): Promise<ApiResponse<User>> => {
    return api.put(`/api/v1/users/${id}`, data)
  },

  // 删除用户
  deleteUser: (id: number): Promise<ApiResponse<null>> => {
    return api.delete(`/api/v1/users/${id}`)
  },

  // 获取当前用户信息
  getCurrentUser: (): Promise<ApiResponse<User>> => {
    return api.get('/api/v1/auth/user-info')
  },
} 