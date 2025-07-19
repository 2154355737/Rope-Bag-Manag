import { api, ApiResponse } from '../utils/apiClient'

// 缓存数据类型
export interface CacheData {
  key: string
  value: any
  ttl: number
  created_at: string
}

// 缓存API
export const apiCache = {
  // 获取缓存
  get: (key: string): Promise<ApiResponse<any>> => {
    return api.get(`/api/v1/cache/${key}`)
  },

  // 设置缓存
  set: (key: string, value: any, ttl?: number): Promise<ApiResponse<null>> => {
    return api.post('/api/v1/cache', { key, value, ttl })
  },

  // 删除缓存
  delete: (key: string): Promise<ApiResponse<null>> => {
    return api.delete(`/api/v1/cache/${key}`)
  },

  // 清空所有缓存
  clear: (): Promise<ApiResponse<null>> => {
    return api.delete('/api/v1/cache')
  },

  // 获取缓存统计
  getStats: (): Promise<ApiResponse<{
    total_keys: number
    total_size: number
    hit_rate: number
  }>> => {
    return api.get('/api/v1/cache/stats')
  }
} 