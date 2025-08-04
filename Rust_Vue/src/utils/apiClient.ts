import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
// 引入 fetch 拦截器（副作用）
import './http/fetchInterceptor'
import { getToken, clearToken as clearAuthToken } from './auth'

// API 基础路径：统一使用相对路径，通过代理转发到后端
// 开发模式：Vite开发服务器代理 /api -> http://127.0.0.1:15201
// 生产模式：nginx代理 /api/ -> http://127.0.0.1:15201
const API_BASE_URL = '/api'

// 可选：通过环境变量覆盖（用于特殊部署场景）
const CUSTOM_API_URL = import.meta.env.VITE_API_BASE_URL
const FINAL_API_BASE_URL = CUSTOM_API_URL || API_BASE_URL

// 创建axios实例
const apiClient: AxiosInstance = axios.create({
  baseURL: FINAL_API_BASE_URL,
  timeout: 10000,
  withCredentials: true, // 支持Cookie发送
  headers: {
    'Content-Type': 'application/json',
  },
})

// 开发环境调试信息
if (import.meta.env.DEV) {
  console.log('🔧 API Client Configuration:', {
    baseURL: FINAL_API_BASE_URL,
    mode: import.meta.env.MODE
  })
}

// 请求拦截器
apiClient.interceptors.request.use(
  (config) => {
    // 检查是否正在退出登录，如果是则阻止非登录相关的API调用
    if (typeof window !== 'undefined' && (window as any).isLoggingOut) {
      if (config.url && !config.url.includes('/auth/logout')) {
        return Promise.reject(new Error('正在退出登录'))
      }
    }

    // 添加认证token
    const token = getToken()
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    
    // 过滤空字符串查询参数
    if (config.params) {
      const cleanParams = { ...config.params }
      Object.keys(cleanParams).forEach(key => {
        if (cleanParams[key] === '') {
          delete cleanParams[key]
        }
      })
      config.params = cleanParams
    }
    
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
apiClient.interceptors.response.use(
  (response) => {
    // 🔧 修复响应格式兼容性：将后端的 message 字段映射为前端期望的 msg 字段
    if (response.data && typeof response.data === 'object') {
      if ('message' in response.data && !('msg' in response.data)) {
        response.data.msg = response.data.message
      }
    }
    
    return response
  },
  (error) => {
    if (error.response?.status === 401) {
      // 清除token
      clearAuthToken()
      
      // 检查当前是否已经在登录页，避免重复重定向
      const currentPath = window.location.pathname
      if (!currentPath.startsWith('/login') && !currentPath.startsWith('/register') && !currentPath.startsWith('/forgot-password')) {
        console.log('🚫 401错误，重定向到登录页')
        window.location.href = '/login'
      }
    }
    return Promise.reject(error)
  }
)

// API响应类型 - 统一类型定义
export interface ApiResponse<T = any> {
  code: number
  message?: string
  msg?: string
  data?: T
}

// 通用API方法
export const api = {
  // GET请求
  get: <T = any>(url: string, config?: AxiosRequestConfig): Promise<ApiResponse<T>> => {
    return apiClient.get(url, config).then(response => response.data)
  },

  // POST请求
  post: <T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<ApiResponse<T>> => {
    return apiClient.post(url, data, config).then(response => response.data)
  },

  // PUT请求
  put: <T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<ApiResponse<T>> => {
    return apiClient.put(url, data, config).then(response => response.data)
  },

  // DELETE请求
  delete: <T = any>(url: string, config?: AxiosRequestConfig): Promise<ApiResponse<T>> => {
    return apiClient.delete(url, config).then(response => response.data)
  },

  // 文件上传
  upload: <T = any>(url: string, formData: FormData, config?: AxiosRequestConfig): Promise<ApiResponse<T>> => {
    return apiClient.post(url, formData, {
      ...config,
      headers: {
        'Content-Type': 'multipart/form-data',
        ...config?.headers,
      },
    }).then(response => response.data)
  },
}

export const uploadFile = <T = any>(url: string, formData: FormData, config?: AxiosRequestConfig): Promise<ApiResponse<T>> => {
  return apiClient.post(url, formData, {
    ...config,
    headers: {
      'Content-Type': 'multipart/form-data',
      ...(config?.headers || {})
    }
  }).then(res => res.data)
}

// 健康检查
export const healthCheck = () => {
  return api.get('/health')
}

// 设置token
export const setToken = (token: string) => {
  localStorage.setItem('token', token)
}

// 获取token (已移至auth.ts，这里重新导出以保持向后兼容)
export { getToken } from './auth'

// 清除token
export const clearToken = () => {
  localStorage.removeItem('token')
}

// 检查是否已登录
export const isLoggedIn = (): boolean => {
  return !!getToken()
}

export default apiClient 