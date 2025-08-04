import { api, ApiResponse } from '../utils/apiClient'

// 用户信息类型
export interface User {
  id: number
  username: string
  email: string
  nickname?: string
  role: string
  status: number
  qq_number?: string | null
  avatar_url?: string | null
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
  nickname?: string
  qq_number?: string
  avatar_url?: string
  star?: number
  role?: string
  ban_status?: string
  ban_reason?: string | null
}

// 用户API
export const userApi = {
  // 获取用户列表
  getUsers: (params?: UserQueryParams): Promise<ApiResponse<UserListResponse>> => {
    const queryParams = new URLSearchParams()
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())  // 使用page_size发送给后端
    if (params?.role) queryParams.append('role', params.role)
    if (params?.status) queryParams.append('status', params.status)
    if (params?.search) queryParams.append('search', params.search)

    return api.get(`/v1/users?${queryParams.toString()}`)
  },

  // 获取单个用户
  getUser: (id: number): Promise<ApiResponse<User>> => {
    return api.get(`/v1/users/${id}`)
  },

  // 更新用户
  updateUser: (id: number, data: UpdateUserRequest): Promise<ApiResponse<User>> => {
    return api.put(`/v1/users/${id}`, data)
  },

  // 删除用户
  deleteUser: (id: number): Promise<ApiResponse<null>> => {
    return api.delete(`/v1/users/${id}`)
  },

  // 批量删除用户
  batchDeleteUsers: (usernames: string[]): Promise<ApiResponse<null>> => {
    return api.delete('/v1/users/batch', { data: { usernames } })
  },

  // 创建用户
  createUser: (userData: any): Promise<ApiResponse<User>> => {
    return api.post('/v1/users', userData)
  },

  // 获取当前用户信息
  getCurrentUser: (): Promise<ApiResponse<User>> => {
    return api.get('/v1/auth/user-info')
  },

  // 更新当前用户信息
  updateCurrentUser: (data: UpdateUserRequest): Promise<ApiResponse<any>> => {
    return api.put('/v1/users/profile', data)
  },

  // 获取我的资源
  getMyResources: (params?: {
    page?: number
    pageSize?: number
    status?: string
  }): Promise<ApiResponse<{
    list: any[]
    total: number
    page: number
    pageSize: number
  }>> => {
    return api.get('/v1/users/my-resources', { params })
  },

  // 获取我的评论
  getMyComments: (params?: {
    page?: number
    size?: number
  }): Promise<ApiResponse<{
    list: any[]
    total: number
    page: number
    size: number
  }>> => {
    return api.get('/v1/users/my-comments', { params })
  },
} 