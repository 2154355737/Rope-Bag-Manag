import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'

// API配置
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:15201'

// 创建axios实例
const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器
apiClient.interceptors.request.use(
  (config) => {
    // 添加认证token
    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
apiClient.interceptors.response.use(
  (response: AxiosResponse) => {
    return response
  },
  (error) => {
    if (error.response?.status === 401) {
      // 清除token并跳转到登录页
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

// API响应类型
export interface ApiResponse<T = any> {
  code: number
  message: string
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

// 健康检查
export const healthCheck = () => {
  return api.get('/health')
}

// 设置token
export const setToken = (token: string) => {
  localStorage.setItem('token', token)
}

// 获取token
export const getToken = (): string | null => {
  return localStorage.getItem('token')
}

// 清除token
export const clearToken = () => {
  localStorage.removeItem('token')
}

// 检查是否已登录
export const isLoggedIn = (): boolean => {
  return !!getToken()
}

export default apiClient 