import axios from 'axios'
import { apiCache, withCache } from './cache'

const api = axios.create({
  baseURL: '/api',
  timeout: 15000,
})

// 添加响应拦截器，统一处理错误
api.interceptors.response.use(
  (response) => {
    return response
  },
  (error) => {
    console.log('API错误详情:', {
      code: error.code,
      message: error.message,
      response: error.response,
      config: error.config
    })
    
    // 网络错误或服务不可用
    if (error.code === 'ERR_NETWORK' || 
        error.code === 'ERR_CONNECTION_REFUSED' ||
        error.code === 'ECONNABORTED' ||
        !error.response) {
      console.error('网络连接错误:', error)
      throw new Error('Service unavailable')
    }
    
    // 其他HTTP错误
    console.error('HTTP错误:', error.response?.status, error.response?.data)
    throw error
  }
)

// 获取用户信息的辅助函数
function getUserInfo() {
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin'
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  return username
}

// 带缓存的API函数
export const login = async (username: string, password: string) => {
  const res = await api.get('/login', { params: { username, password } })
  return res.data
}

export const getUsers = withCache(async () => {
  const res = await api.get('/get-users-db')
  return res.data
}, 'getUsers', 60000) // 1分钟缓存

export const getPackages = withCache(async () => {
  const username = getUserInfo()
  const res = await api.get('/get-data-db', { params: { username } })
  return res.data
}, 'getPackages', 30000) // 30秒缓存

export const getLogs = withCache(async () => {
  const res = await api.get('/logs/entries')
  return res.data
}, 'getLogs', 15000) // 15秒缓存

export const getStats = withCache(async () => {
  const username = getUserInfo()
  const [res1, res2] = await Promise.all([
    api.get('/stats/api-counts', { params: { username } }),
    api.get('/stats/downloads', { params: { username } })
  ])
  return { 
    code: 0, 
    data: { 
      api_counts: res1.data.data, 
      downloads: res2.data.data 
    } 
  }
}, 'getStats', 20000) // 20秒缓存

export const getApiCallStats = withCache(async () => {
  const username = getUserInfo()
  const res = await api.get('/stats/api-calls', { params: { username } })
  return res.data
}, 'getApiCallStats', 25000) // 25秒缓存

export const getApiPerformance = withCache(async () => {
  const username = getUserInfo()
  const res = await api.get('/stats/api-performance', { params: { username } })
  return res.data
}, 'getApiPerformance', 30000) // 30秒缓存

export const getRecentCalls = withCache(async () => {
  const username = getUserInfo()
  const res = await api.get('/stats/recent-calls', { params: { username } })
  return res.data
}, 'getRecentCalls', 10000) // 10秒缓存

export const getDashboardData = withCache(async () => {
  console.log('开始请求仪表盘数据...')
  const res = await api.get('/dashboard')
  console.log('仪表盘数据请求成功:', res.data)
  return res.data
}, 'getDashboardData', 15000) // 15秒缓存

// 管理员操作函数（不缓存）
export const banUser = async (username: string, banned: boolean) => {
  const res = await api.get('/admin/ban-user', {
    params: { target: username, banned, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export const setStar = async (username: string, star: number) => {
  const res = await api.get('/admin/set-star', {
    params: { target: username, star, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export const setNickname = async (username: string, nickname: string) => {
  const res = await api.get('/admin/set-user', {
    params: { target: username, nickname, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export const setPassword = async (username: string, password: string) => {
  const res = await api.get('/admin/set-user', {
    params: { target: username, password, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export const addPackage = async (data: any) => {
  const res = await api.get('/admin/add-rope-package', {
    params: { ...data, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export const updatePackage = async (data: any) => {
  const res = await api.get('/admin/update-rope-package', {
    params: { ...data, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export const deletePackage = async (id: number) => {
  const res = await api.get('/admin/delete-rope-package', {
    params: { id, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export const getLogEntries = withCache(async (page: number = 1, pageSize: number = 20) => {
  const res = await api.get('/logs/entries', {
    params: { page, page_size: pageSize }
  })
  return res.data
}, 'getLogEntries', 10000) // 10秒缓存

// 导出缓存管理器，供外部使用
export { apiCache } 