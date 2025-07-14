import { api, ApiResponse } from '../utils/apiClient'

// 用户登录请求类型
export interface LoginRequest {
  username: string
  password: string
}

// 用户注册请求类型
export interface RegisterRequest {
  username: string
  password: string
  nickname: string
  qq_number: string
}

// 用户信息类型
export interface UserInfo {
  id: number
  username: string
  email: string
  role: string
  status: number
  created_at: string
  updated_at: string
}

// 登录响应类型
export interface LoginResponse {
  user: UserInfo
  token: string
}

// 认证API
export const authApi = {
  // 用户登录
  login: (data: LoginRequest): Promise<ApiResponse<LoginResponse>> => {
    return api.post('/api/v1/auth/login', data)
  },

  // 用户注册
  register: (data: RegisterRequest): Promise<ApiResponse<LoginResponse>> => {
    return api.post('/api/v1/auth/register', data)
  },

  // 获取用户信息
  getUserInfo: (): Promise<ApiResponse<UserInfo>> => {
    return api.get('/api/v1/auth/user-info')
  },

  // 用户登出
  logout: (): Promise<ApiResponse<null>> => {
    return api.post('/api/v1/auth/logout')
  },
} 