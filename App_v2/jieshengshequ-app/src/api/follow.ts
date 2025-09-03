import { http } from './client'

export interface FollowRequest {
  user_id: number
}

export interface FollowResponse {
  success: boolean
  is_following: boolean
  followers_count: number
}

export interface Follower {
  id: number
  username: string
  nickname: string
  avatar_url?: string
  bio?: string
  followed_at: string
}

export interface Following {
  id: number
  username: string
  nickname: string
  avatar_url?: string
  bio?: string
  followed_at: string
}

export interface FollowListResponse {
  items: Follower[] | Following[]
  total: number
  hasMore: boolean
}

// 关注用户
export async function followUser(userId: number): Promise<FollowResponse> {
  return http.post<FollowResponse>(`/users/${userId}/follow`)
}

// 取消关注用户
export async function unfollowUser(userId: number): Promise<FollowResponse> {
  return http.delete<FollowResponse>(`/users/${userId}/follow`)
}

// 检查是否关注某用户
export async function checkFollowStatus(userId: number): Promise<{ is_following: boolean }> {
  return http.get<{ is_following: boolean }>(`/users/${userId}/follow-status`)
}

// 获取用户的关注者列表
export async function getUserFollowers(userId: number, params?: {
  page?: number
  pageSize?: number
}): Promise<FollowListResponse> {
  const query = new URLSearchParams()
  if (params?.page) query.append('page', params.page.toString())
  if (params?.pageSize) query.append('pageSize', params.pageSize.toString())
  
  const queryString = query.toString()
  return http.get<FollowListResponse>(`/users/${userId}/followers${queryString ? `?${queryString}` : ''}`)
}

// 获取用户的关注列表
export async function getUserFollowing(userId: number, params?: {
  page?: number
  pageSize?: number
}): Promise<FollowListResponse> {
  const query = new URLSearchParams()
  if (params?.page) query.append('page', params.page.toString())
  if (params?.pageSize) query.append('pageSize', params.pageSize.toString())
  
  const queryString = query.toString()
  return http.get<FollowListResponse>(`/users/${userId}/following${queryString ? `?${queryString}` : ''}`)
}

// 获取我的关注者列表
export async function getMyFollowers(params?: {
  page?: number
  pageSize?: number
}): Promise<FollowListResponse> {
  const query = new URLSearchParams()
  if (params?.page) query.append('page', params.page.toString())
  if (params?.pageSize) query.append('pageSize', params.pageSize.toString())
  
  const queryString = query.toString()
  return http.get<FollowListResponse>(`/me/followers${queryString ? `?${queryString}` : ''}`)
}

// 获取我的关注列表
export async function getMyFollowing(params?: {
  page?: number
  pageSize?: number
}): Promise<FollowListResponse> {
  const query = new URLSearchParams()
  if (params?.page) query.append('page', params.page.toString())
  if (params?.pageSize) query.append('pageSize', params.pageSize.toString())
  
  const queryString = query.toString()
  return http.get<FollowListResponse>(`/me/following${queryString ? `?${queryString}` : ''}`)
}

// 获取关注动态（关注的用户发布的内容）
export async function getFollowingFeed(params?: {
  page?: number
  pageSize?: number
  type?: 'post' | 'resource' | 'all'
}): Promise<{
  items: any[]
  total: number
  hasMore: boolean
}> {
  const query = new URLSearchParams()
  if (params?.page) query.append('page', params.page.toString())
  if (params?.pageSize) query.append('pageSize', params.pageSize.toString())
  if (params?.type && params.type !== 'all') query.append('type', params.type)
  
  const queryString = query.toString()
  return http.get(`/me/following-feed${queryString ? `?${queryString}` : ''}`)
} 