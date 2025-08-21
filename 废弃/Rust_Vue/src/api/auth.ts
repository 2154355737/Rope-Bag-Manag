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
  email: string
  nickname: string | null
  qq_number: string | null
  verification_code: string // 邮箱验证码
}

// 邮箱验证码登录请求类型
export interface EmailLoginRequest {
  email: string
  code: string
}

// 发送验证码请求类型
export interface SendCodeRequest {
  email: string
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
    return api.post('/v1/auth/login', data)
  },

  // 邮箱验证码登录
  loginByEmail: (data: EmailLoginRequest): Promise<ApiResponse<LoginResponse>> => {
    return api.post('/v1/auth/login-by-email', data)
  },

  // 用户注册
  register: (data: RegisterRequest): Promise<ApiResponse<LoginResponse>> => {
    return api.post('/v1/auth/register', data)
  },

  // 获取用户信息
  getUserInfo: (): Promise<ApiResponse<UserInfo>> => {
    return api.get('/v1/auth/user-info')
  },

  // 发送注册验证码
  sendRegisterCode: (data: SendCodeRequest): Promise<ApiResponse<null>> => {
    return api.post('/v1/auth/send-register-code', data)
  },

  // 发送登录验证码
  sendLoginCode: (data: SendCodeRequest): Promise<ApiResponse<null>> => {
    return api.post('/v1/auth/send-login-code', data)
  },

  // 请求重置密码邮件
  resetRequest: (email: string): Promise<ApiResponse<null>> => {
    return api.post('/v1/auth/reset-request', { email })
  },

  // 验证邮箱验证码
  verifyCode: (email: string, code: string): Promise<ApiResponse<null>> => {
    return api.post('/v1/auth/verify-code', { email, code })
  },

  // 重置密码
  resetPassword: (data: { email: string; token: string; new_password: string }): Promise<ApiResponse<null>> => {
    return api.post('/v1/auth/reset-password', data)
  },

  // 退出登录 (清除服务器端HttpOnly Cookie)
  logout: (): Promise<ApiResponse<null>> => {
    return api.post('/v1/auth/logout')
  },
} 