import { http } from './client'

// 用户统计数据接口
export interface UserStats {
  posts: number
  resources: number
  views: number
  likes: number
}

// 用户活动统计接口
export interface UserActivityStats {
  posts_count: number
  resources_count: number
  comments_count: number
  total_views: number
  total_likes: number
  total_downloads: number
  level: string
  experience: number
  next_level_exp: number
}

// 成就接口
export interface Achievement {
  id: number
  name: string
  icon: string
  description: string
  earned_at?: string
}

// 周报数据接口
export interface WeeklyReportData {
  total_posts: number
  completed_projects: number
  current_streak: number
  today_activity: number
  weekly_posts: number[]
  achievements: Achievement[]
}

// 获取我的统计数据
export async function getMyStats(): Promise<UserStats> {
  const response = await http.get<UserStats>('/me/stats')
  return response
}

// 获取我的活动统计
export async function getMyActivityStats(): Promise<UserActivityStats> {
  const response = await http.get<UserActivityStats>('/me/activity-stats')
  return response
}

// 获取我的周报
export async function getMyWeeklyReport(): Promise<WeeklyReportData> {
  const response = await http.get<WeeklyReportData>('/me/weekly-report')
  return response
}

// 获取我的成就
export async function getMyAchievements(): Promise<{ list: Achievement[]; total: number }> {
  const response = await http.get<{ list: Achievement[]; total: number }>('/me/achievements')
  return response
}

// 用户签到
export async function userCheckIn(): Promise<void> {
  await http.post<void>('/me/check-in')
}

// 上传头像
export async function uploadAvatar(file: File): Promise<{ avatar_url: string }> {
  const formData = new FormData()
  formData.append('avatar', file)
  
  const response = await http.post<{ avatar_url: string }>('/users/upload-avatar', formData)
  return response
}

// 获取我的资源
export async function getMyResources(params?: { page?: number; pageSize?: number }) {
  const response = await http.get<{ 
    list: any[]
    total: number
    page: number
    pageSize: number
    totalPages: number
  }>('/me/resources', params)
  return response
}

// 获取我的帖子
export async function getMyPosts(params?: { page?: number; pageSize?: number }) {
  const response = await http.get<{ 
    list: any[]
    total: number
    page: number
    pageSize: number
    totalPages: number
  }>('/me/posts', params)
  return response
}

// 获取我的评论
export async function getMyComments(params?: { page?: number; size?: number }) {
  const response = await http.get<{ 
    list: any[]
    total: number
    page: number
    size: number
    totalPages: number
  }>('/me/comments', params)
  return response
}

// 获取我的点赞
export async function getMyLikes(params?: { target?: string; page?: number; page_size?: number }) {
  const response = await http.get<{ 
    list: any[]
    total: number
    page: number
    page_size: number
  }>('/users/my-likes', params)
  return response
}

// 获取我的点赞统计
export async function getMyLikesStats() {
  const response = await http.get<{
    like_total: number
    like_by_type: { package: number; post: number }
    view_total: number
    view_by_type: { post: number }
  }>('/users/my-likes/stats')
  return response
} 