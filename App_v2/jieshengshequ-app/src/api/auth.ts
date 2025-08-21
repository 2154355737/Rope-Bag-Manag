import { http, setToken, clearToken, getToken } from './client'
import { isTokenExpired } from '@/utils/jwt'

export interface LoginRequest {
  username: string
  password: string
}

export interface RegisterRequest {
  username: string
  email: string
  password: string
}

export interface User {
  id: number
  username: string
  email: string
  avatar_url?: string
  nickname?: string
  bio?: string
  location?: string
  website?: string
  skills?: string
  role: string
  created_at: string
}

export interface LoginResponse {
  token: string
  user: User
}

// 登录
export async function login(data: LoginRequest): Promise<LoginResponse> {
  const response = await http.post<LoginResponse>('/auth/login', data)
  // 保存token和用户信息
  setToken(response.token)
  localStorage.setItem('user', JSON.stringify(response.user))
  return response
}

// 注册
export async function register(data: RegisterRequest): Promise<LoginResponse> {
  const response = await http.post<LoginResponse>('/auth/register', data)
  // 保存token和用户信息
  setToken(response.token)
  localStorage.setItem('user', JSON.stringify(response.user))
  return response
}

// 退出登录
export async function logout(): Promise<void> {
  try {
    await http.post('/auth/logout')
  } catch (error) {
    console.error('Logout API error:', error)
  } finally {
    // 无论API调用是否成功，都清除本地数据
    clearToken()
  }
}

// 获取当前用户信息
export async function getCurrentUser(): Promise<User> {
  return http.get<User>('/auth/user-info')
}

// 获取当前用户详细资料
export async function getCurrentUserProfile(): Promise<User> {
  return http.get<User>('/users/profile')
}

// 更新当前用户资料
export async function updateCurrentUserProfile(data: {
  nickname?: string
  bio?: string
  location?: string
  website?: string
  skills?: string
  avatar_url?: string
}): Promise<void> {
  return http.put<void>('/users/profile', data)
}

// 刷新token
export async function refreshToken(): Promise<{ token: string }> {
  const response = await http.post<{ token: string }>('/auth/refresh')
  setToken(response.token)
  return response
}

// 验证token是否有效
export async function verifyToken(): Promise<{ valid: boolean }> {
  return http.get<{ valid: boolean }>('/auth/verify')
}

// 获取本地用户信息
export function getLocalUser(): User | null {
  try {
    const userStr = localStorage.getItem('user')
    return userStr ? JSON.parse(userStr) : null
  } catch {
    return null
  }
}

// 检查是否已登录
export function isLoggedIn(): boolean {
  const token = getToken() // 使用getToken()，它会自动检查过期
  const user = getLocalUser()
  return !!(token && user)
}

// 检查token是否有效（包含过期检查）
export function isTokenValid(): boolean {
  const token = localStorage.getItem('token')
  if (!token) return false
  
  try {
    return !isTokenExpired(token)
  } catch {
    return false
  }
} 