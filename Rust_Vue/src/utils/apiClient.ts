import axios, { type AxiosInstance, type AxiosResponse } from 'axios'
import type { ApiResponse } from '../types'

export class ApiError extends Error {
  constructor(
    message: string,
    public code: number,
    public statusCode?: number
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

export function createApiClient(): AxiosInstance {
  const client = axios.create({
    baseURL: '/api',
    timeout: 15000,
    headers: {
      'Content-Type': 'application/json',
    },
  })

  // 请求拦截器
  client.interceptors.request.use(
    (config) => {
      // 添加认证信息
      const token = localStorage.getItem('token')
      if (token) {
        config.headers.Authorization = `Bearer ${token}`
      }
      
      // 添加时间戳防止缓存
      if (config.method === 'get') {
        config.params = {
          ...config.params,
          _t: Date.now(),
        }
      }
      
      return config
    },
    (error) => {
      return Promise.reject(error)
    }
  )

  // 响应拦截器
  client.interceptors.response.use(
    (response: AxiosResponse<ApiResponse<any>>) => {
      const { code, msg, data } = response.data
      
      if (code === 0) {
        return data
      } else {
        throw new ApiError(msg || '请求失败', code, response.status)
      }
    },
    (error) => {
      if (error instanceof ApiError) {
        throw error
      }
      
      let message = '网络错误'
      let code = -1
      
      if (error.response) {
        const { status, data } = error.response
        code = status
        
        if (data?.msg) {
          message = data.msg
        } else {
          switch (status) {
            case 400:
              message = '请求参数错误'
              break
            case 401:
              message = '未授权，请重新登录'
              // 清除本地存储的认证信息
              localStorage.removeItem('token')
              localStorage.removeItem('isLoggedIn')
              break
            case 403:
              message = '拒绝访问'
              break
            case 404:
              message = '请求的资源不存在'
              break
            case 429:
              message = '请求过于频繁，请稍后再试'
              break
            case 500:
              message = '服务器内部错误'
              break
            default:
              message = `请求失败 (${status})`
          }
        }
      } else if (error.request) {
        message = '网络连接失败'
      } else {
        message = error.message || '未知错误'
      }
      
      throw new ApiError(message, code, error.response?.status)
    }
  )

  return client
}

// 创建默认的API客户端实例
export const apiClient = createApiClient()

// 缓存配置
export const CACHE_TIMES = {
  USERS: 5 * 60 * 1000,        // 用户数据5分钟
  PACKAGES: 2 * 60 * 1000,      // 资源数据2分钟
  STATS: 30 * 1000,             // 统计数据30秒
  LOGS: 10 * 1000,              // 日志数据10秒
  CONFIG: 10 * 60 * 1000,       // 配置数据10分钟
} as const

// 带缓存的API调用
const cache = new Map<string, { data: any; timestamp: number; ttl: number }>()

export function withCache<T>(
  apiCall: () => Promise<T>,
  key: string,
  ttl: number = 5 * 60 * 1000
): () => Promise<T> {
  return async () => {
    const cached = cache.get(key)
    if (cached && Date.now() - cached.timestamp < cached.ttl) {
      return cached.data
    }
    
    const data = await apiCall()
    cache.set(key, {
      data,
      timestamp: Date.now(),
      ttl
    })
    
    return data
  }
}

// 清除缓存
export function clearCache(key?: string) {
  if (key) {
    cache.delete(key)
  } else {
    cache.clear()
  }
}

// 重试机制
export async function withRetry<T>(
  fn: () => Promise<T>,
  maxRetries: number = 3,
  delay: number = 1000
): Promise<T> {
  let lastError: Error
  
  for (let i = 0; i <= maxRetries; i++) {
    try {
      return await fn()
    } catch (error) {
      lastError = error as Error
      
      if (i === maxRetries) {
        throw lastError
      }
      
      // 指数退避
      await new Promise(resolve => setTimeout(resolve, delay * Math.pow(2, i)))
    }
  }
  
  throw lastError!
}

// 批量请求
export async function batchRequest<T>(
  requests: (() => Promise<T>)[],
  concurrency: number = 5
): Promise<T[]> {
  const results: T[] = []
  const executing: Promise<void>[] = []
  
  for (let i = 0; i < requests.length; i++) {
    const request = requests[i]
    const promise = request()
      .then(result => {
        results[i] = result
      })
      .catch(error => {
        console.error(`批量请求第${i + 1}个失败:`, error)
        throw error
      })
    
    executing.push(promise)
    
    if (executing.length >= concurrency) {
      await Promise.race(executing)
      executing.splice(executing.findIndex(p => p === promise), 1)
    }
  }
  
  await Promise.all(executing)
  return results
}

// 防抖函数
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout
  
  return (...args: Parameters<T>) => {
    clearTimeout(timeout)
    timeout = setTimeout(() => func(...args), wait)
  }
}

// 节流函数
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean
  
  return (...args: Parameters<T>) => {
    if (!inThrottle) {
      func(...args)
      inThrottle = true
      setTimeout(() => inThrottle = false, limit)
    }
  }
} 