import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import { resourceRecordApi } from '../api/resourceRecords'
import userActionService from './userActionService'
import { getToken, clearToken as clearAuthToken } from './auth'

// 为Vite环境变量声明类型
/// <reference types="vite/client" />

// API配置
// 生产环境建议通过 .env.production 设置 VITE_API_BASE_URL=/api
const API_BASE_URL: string = import.meta.env.VITE_API_BASE_URL || '/api'

// 创建axios实例
const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  withCredentials: true, // 支持Cookie发送
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器
apiClient.interceptors.request.use(
  (config) => {
    // 检查是否正在退出登录，如果是则阻止非登录相关的API调用
    if (typeof window !== 'undefined' && (window as any).isLoggingOut) {
      if (config.url && !config.url.includes('/auth/logout')) {
        console.log('🚫 正在退出登录，阻止API调用:', config.url)
        return Promise.reject(new Error('正在退出登录'))
      }
    }
    
    // 修正可能重复的 /api 前缀，避免出现 /api/api/*
    if (config.url && config.url.startsWith('/api/')) {
      config.url = config.url.replace(/^\/api/, '')
    }

    // 添加认证token (优先从Cookie获取)
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

// 资源操作映射
const resourceOperationMap: Record<string, { type: string, action: string }> = {
  // 包相关操作
  'POST /v1/packages': { type: 'Package', action: 'Create' },
  'PUT /v1/packages': { type: 'Package', action: 'Update' },
  'DELETE /v1/packages': { type: 'Package', action: 'Delete' },
  'GET /v1/packages/*/download': { type: 'Package', action: 'Download' },
  
  // 用户相关操作
  'POST /v1/users': { type: 'User', action: 'Create' },
  'PUT /v1/users': { type: 'User', action: 'Update' },
  'DELETE /v1/users': { type: 'User', action: 'Delete' },
  
  // 评论相关操作
  'POST /v1/comments': { type: 'Comment', action: 'Create' },
  'PUT /v1/comments': { type: 'Comment', action: 'Update' },
  'DELETE /v1/comments': { type: 'Comment', action: 'Delete' },
  
  // 分类相关操作
  'POST /v1/categories': { type: 'Category', action: 'Create' },
  'PUT /v1/categories': { type: 'Category', action: 'Update' },
  'DELETE /v1/categories': { type: 'Category', action: 'Delete' },
}

// 自动记录资源操作
const logResourceOperation = async (method: string, url: string, response: any) => {
  try {
    // 如果响应不成功，不记录操作
    if (response?.code !== 0) return
    
    // 检查URL，如果是包创建操作，不在前端记录（已在后端记录）
    if (method === 'POST' && url.includes('/v1/packages') && !url.includes('/v1/packages/')) {
      console.log(`[跳过前端记录] 包创建操作已在后端记录`)
      return
    }
    
    // 从URL和方法中推断资源类型和操作类型
    let resourceId: number | null = null
    let resourceType: string = 'Unknown'
    let action: string = 'Unknown'
    
    // 提取各种资源ID
    const packageMatch = url.match(/\/packages\/(\d+)/)
    const userMatch = url.match(/\/users\/(\d+)/)
    const commentMatch = url.match(/\/comments\/(\d+)/)
    const categoryMatch = url.match(/\/categories\/(\d+)/)
    
    if (packageMatch) {
      resourceId = parseInt(packageMatch[1])
      resourceType = 'Package'
    } else if (userMatch) {
      resourceId = parseInt(userMatch[1])
      resourceType = 'User'
    } else if (commentMatch) {
      resourceId = parseInt(commentMatch[1])
      resourceType = 'Comment'
    } else if (categoryMatch) {
      resourceId = parseInt(categoryMatch[1])
      resourceType = 'Category'
    } else {
      // 尝试从URL末尾提取ID
      const urlParts = url.split('/')
      const lastPart = urlParts[urlParts.length - 1]
      if (/^\d+$/.test(lastPart)) {
        resourceId = parseInt(lastPart)
        
        // 从URL推断资源类型
        if (url.includes('/packages')) resourceType = 'Package'
        else if (url.includes('/users')) resourceType = 'User'
        else if (url.includes('/comments')) resourceType = 'Comment'
        else if (url.includes('/categories')) resourceType = 'Category'
      }
    }
    
    // 确定操作类型
    if (method === 'POST') {
      // 创建操作
      if (url.includes('/resource-records')) {
        // 避免记录资源记录本身的创建操作
        return
      }
      action = 'Create'
      // 如果是创建操作，从响应中获取新资源ID
      if (response.data?.id) {
        resourceId = response.data.id
      }
    } else if (method === 'PUT') {
      action = 'Update'
    } else if (method === 'DELETE') {
      action = 'Delete'
    } else if (method === 'GET') {
      if (url.includes('/download')) {
        action = 'Download'
      } else {
        // 普通GET请求不记录
        return
      }
    } else {
      // 其他方法不记录
      return
    }
    
    // 如果找不到资源ID或操作类型未知，则不记录
    if (!resourceId || action === 'Unknown' || resourceType === 'Unknown') {
      console.log(`[未记录] 无法确定资源信息: ${method} ${url}`)
      return
    }
    
    // 准备记录数据
    const recordData: any = {
      resource_type: resourceType
    }
    
    // 如果有响应数据，添加为新数据
    if (response.data && action !== 'Delete') {
      recordData.new_data = typeof response.data === 'string' 
        ? response.data 
        : JSON.stringify(response.data)
    }
    
    console.log(`[自动记录] ${resourceType} ${action}操作：ID=${resourceId}`)
    
    // 使用资源记录服务记录操作
    import('../utils/loggerService').then(({ resourceLogger }) => {
      resourceLogger.logResourceOperation(resourceType, action, resourceId as number, null, response.data)
        .catch(err => console.warn('记录操作失败:', err))
    }).catch(err => console.error('导入记录服务失败:', err))
  } catch (err) {
    console.warn('自动记录资源操作失败:', err)
  }
}

// 直接请求拦截器
// 替换原生fetch方法，用于拦截直接的fetch请求
const originalFetch = window.fetch
window.fetch = async function(input: RequestInfo | URL, init?: RequestInit) {
  // 获取请求方法和URL
  const method = init?.method || 'GET'
  const urlString = typeof input === 'string' ? input : input instanceof URL ? input.toString() : (input as Request).url
  
  // 只拦截API请求
  if (typeof urlString === 'string' && urlString.includes('/api/')) {
    console.log(`[Fetch拦截] ${method} ${urlString}`)
  }
  
  // 调用原始fetch
  const response = await originalFetch(input, init)
  
  // 克隆响应以便读取内容
  const clone = response.clone()
  
  // 只拦截API请求
  if (typeof urlString === 'string' && urlString.includes('/api/')) {
    // 检查URL，如果是包创建操作，不在前端记录（已在后端记录）
    if (method === 'POST' && urlString.includes('/v1/packages') && !urlString.includes('/v1/packages/')) {
      console.log(`[跳过前端记录] 包创建操作已在后端记录`)
      return response
    }
    
    // 异步处理响应数据
    clone.json().then(data => {
      if (data && data.code === 0) {
        // 处理成功响应，尝试记录操作
        setTimeout(() => {
          try {
            // 提取操作类型和资源ID
            let resourceId: number | null = null
            let action: string = 'Unknown'
            let resourceType: string = 'Unknown'
            
            // 从URL中提取资源ID
            const urlStr = urlString
            
            // 识别资源类型
            if (urlStr.includes('/packages/')) {
              resourceType = 'Package'
              const packageMatch = urlStr.match(/\/packages\/(\d+)/)
              if (packageMatch) resourceId = parseInt(packageMatch[1])
            } else if (urlStr.includes('/users/')) {
              resourceType = 'User'
              const userMatch = urlStr.match(/\/users\/(\d+)/)
              if (userMatch) resourceId = parseInt(userMatch[1])
            } else if (urlStr.includes('/comments/')) {
              resourceType = 'Comment'
              const commentMatch = urlStr.match(/\/comments\/(\d+)/)
              if (commentMatch) resourceId = parseInt(commentMatch[1])
            } else if (urlStr.includes('/categories/')) {
              resourceType = 'Category'
              const categoryMatch = urlStr.match(/\/categories\/(\d+)/)
              if (categoryMatch) resourceId = parseInt(categoryMatch[1])
            } else {
              // 普通ID匹配
              const idMatch = urlStr.match(/\/(\d+)(\/|$)/)
              if (idMatch) {
                resourceId = parseInt(idMatch[1])
                // 从URL推断资源类型
                if (urlStr.includes('/packages')) resourceType = 'Package'
                else if (urlStr.includes('/users')) resourceType = 'User'
                else if (urlStr.includes('/comments')) resourceType = 'Comment'
                else if (urlStr.includes('/categories')) resourceType = 'Category'
              }
            }
            
            // 根据请求方法确定操作类型
            if (method === 'POST') action = 'Create'
            else if (method === 'PUT') action = 'Update'
            else if (method === 'DELETE') action = 'Delete'
            else if (method === 'GET' && urlStr.includes('/download')) action = 'Download'
            
            // 如果有资源ID和有效的操作类型，记录操作
            if (resourceId && action !== 'Unknown' && resourceType !== 'Unknown') {
              // 准备记录数据
              const recordData: any = {
                resource_type: resourceType
              }
              
              // 如果有响应数据，添加为新数据
              if (data.data && action !== 'Delete') {
                recordData.new_data = JSON.stringify(data.data)
              }
              
              console.log(`[Fetch记录] ${resourceType} ${action}操作：ID=${resourceId}`)
              
              // 使用资源记录服务记录操作
              import('../utils/loggerService').then(({ resourceLogger }) => {
                resourceLogger.logResourceOperation(resourceType, action, resourceId as number, null, data.data)
                  .catch(err => console.warn('记录操作失败:', err))
              }).catch(err => console.error('导入记录服务失败:', err))
            }
          } catch (err) {
            console.warn('处理响应数据失败:', err)
          }
        }, 0)
      }
    }).catch(err => {
      console.warn('解析响应JSON失败:', err)
    })
  }
  
  return response
}

// 响应拦截器
apiClient.interceptors.response.use(
  (response) => {
    const { config, data } = response
    const method = config.method?.toUpperCase() || 'GET'
    const url = config.url || ''
    const token = localStorage.getItem('token') || '';
    const userInfo = localStorage.getItem('userInfo');
    const isAuthenticated = !!token && !!userInfo;

    // 保留认证判断，但不做任何行为记录，防止语法错误
    if (data && data.code === 0 && isAuthenticated) {
      // 暂时不做任何行为记录
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
      } else {
        console.log('🚫 401错误，已在登录页面，不重复重定向')
      }
    }
    return Promise.reject(error)
  }
)

// API响应类型
export interface ApiResponse<T = any> {
  code: number;
  message?: string;
  msg?: string;
  data?: T;
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