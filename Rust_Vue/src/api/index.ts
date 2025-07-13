import axios from 'axios'
import { apiCache, withCache } from './cache'

const api = axios.create({
  baseURL: 'http://127.0.0.1:15202',
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

// 获取当前用户名的辅助函数
function getCurrentUsername() {
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

// ====== 用户认证相关API ======
export const login = async (username: string, password: string) => {
  const res = await api.get('/api/login', { params: { username, password } })
  return res.data
}

export const register = async (username: string, password: string, nickname: string) => {
  const res = await api.get('/api/register', { params: { username, password, nickname } })
  return res.data
}

export const getUserInfo = async (username: string) => {
  const res = await api.get('/api/user-info', { params: { username } })
  return res.data
}

export const signIn = async (username: string) => {
  const res = await api.get('/api/sign-in', { params: { username } })
  return res.data
}

export const changePassword = async (username: string, oldPassword: string, newPassword: string) => {
  const res = await api.get('/api/change-password', { params: { username, old_password: oldPassword, new_password: newPassword } })
  return res.data
}

export const changeNickname = async (username: string, nickname: string) => {
  const res = await api.get('/api/change-nickname', { params: { username, nickname } })
  return res.data
}

export const getNicknames = withCache(async () => {
  const res = await api.get('/api/nicknames')
  return res.data
}, 'getNicknames', 60000) // 1分钟缓存

// ====== 资源管理相关API ======
export const getPackages = withCache(async () => {
  const username = getCurrentUsername()
  const res = await api.get('/api/get-data-db', { params: { username } })
  return res.data
}, 'getPackages', 30000) // 30秒缓存

export const addPackage = async (data: {
  name: string
  author: string
  version: string
  desc: string
  url: string
  username: string
  admin_password?: string
}) => {
  const res = await api.get('/api/add-rope-package', { params: data })
  return res.data
}

export const updatePackage = async (data: {
  id: number
  name: string
  author: string
  version: string
  desc: string
  url: string
  username: string
  admin_password?: string
}) => {
  const res = await api.get('/api/update-rope-package', { params: data })
  return res.data
}

export const deletePackage = async (id: number, username: string, admin_password?: string) => {
  const res = await api.get('/api/delete-rope-package', { params: { id, username, admin_password } })
  return res.data
}

export const downloadPackage = async (id: number) => {
  const res = await api.get('/api/download-rope-package', { params: { id } })
  return res.data
}

// ====== 管理员相关API ======
export const getUsers = withCache(async () => {
  const res = await api.get('/api/get-users-db')
  return res.data
}, 'getUsers', 60000) // 1分钟缓存

export const adminUserInfo = async (username: string, admin_username: string = 'muteduanxing', admin_password: string = 'ahk12378dx') => {
  const res = await api.get('/api/admin/user-info', { params: { username, admin_username, admin_password } })
  return res.data
}

export const adminSetUser = async (data: {
  target: string
  nickname?: string
  password?: string
  admin_username?: string
  admin_password?: string
}) => {
  const admin_username = data.admin_username || 'muteduanxing'
  const admin_password = data.admin_password || 'ahk12378dx'
  const res = await api.get('/api/admin/set-user', { params: { ...data, admin_username, admin_password } })
  
  // 清除用户缓存
  if (res.code === 0) {
    apiCache.delete('getUsers')
  }
  
  return res.data
}

export const adminSetStar = async (target: string, star: number, admin_username: string = 'muteduanxing', admin_password: string = 'ahk12378dx') => {
  const res = await api.get('/api/admin/set-star', { params: { target, star, admin_username, admin_password } })
  
  // 清除用户缓存
  if (res.code === 0) {
    apiCache.delete('getUsers')
  }
  
  return res.data
}

export const adminBanUser = async (target: string, banned: boolean, admin_username: string = 'muteduanxing', admin_password: string = 'ahk12378dx') => {
  const res = await api.get('/api/admin/ban-user', { params: { target, banned, admin_username, admin_password } })
  
  // 清除用户缓存
  if (res.code === 0) {
    apiCache.delete('getUsers')
  }
  
  return res.data
}

export const adminAddPackage = async (data: {
  name: string
  author: string
  version: string
  desc: string
  url: string
  category: string
  status: string
  admin_username?: string
  admin_password?: string
}) => {
  const admin_username = data.admin_username || 'muteduanxing'
  const admin_password = data.admin_password || 'ahk12378dx'
  const res = await api.get('/api/admin/add-rope-package', { params: { ...data, admin_username, admin_password } })
  
  // 清除绳包缓存
  if (res.code === 0) {
    apiCache.delete('getPackages')
  }
  
  return res.data
}

export const adminUpdatePackage = async (data: {
  id: number
  name: string
  author: string
  version: string
  desc: string
  url: string
  category: string
  status: string
  admin_username?: string
  admin_password?: string
}) => {
  const admin_username = data.admin_username || 'muteduanxing'
  const admin_password = data.admin_password || 'ahk12378dx'
  const res = await api.get('/api/admin/update-rope-package', { params: { ...data, admin_username, admin_password } })
  
  // 清除绳包缓存
  if (res.code === 0) {
    apiCache.delete('getPackages')
  }
  
  return res.data
}

export const adminDeletePackage = async (id: number, admin_username: string = 'muteduanxing', admin_password: string = 'ahk12378dx') => {
  const res = await api.get('/api/admin/delete-rope-package', { params: { id, admin_username, admin_password } })
  
  // 清除绳包缓存
  if (res.code === 0) {
    apiCache.delete('getPackages')
  }
  
  return res.data
}

export const setAdmin = async (target: string, is_admin: boolean, admin_username: string = 'muteduanxing', admin_password: string = 'ahk12378dx') => {
  const res = await api.get('/api/set-admin', { params: { target, is_admin, admin_username, admin_password } })
  return res.data
}

// ====== 分类管理相关API ======
export const getCategories = withCache(async () => {
  const admin_username = 'muteduanxing'
  const admin_password = 'ahk12378dx'
  const res = await api.get('/api/admin/categories', { params: { admin_username, admin_password } })
  return res.data
}, 'getCategories', 60000) // 1分钟缓存

export const addCategory = async (data: {
  name: string
  description: string
  enabled: boolean
  admin_username?: string
  admin_password?: string
}) => {
  const admin_username = data.admin_username || 'muteduanxing'
  const admin_password = data.admin_password || 'ahk12378dx'
  const res = await api.get('/api/admin/add-category', { params: { ...data, admin_username, admin_password } })
  
  // 清除分类缓存
  if (res.code === 0) {
    apiCache.delete('getCategories')
  }
  
  return res.data
}

export const updateCategory = async (data: {
  id: number
  name: string
  description: string
  enabled: boolean
  admin_username?: string
  admin_password?: string
}) => {
  const admin_username = data.admin_username || 'muteduanxing'
  const admin_password = data.admin_password || 'ahk12378dx'
  const res = await api.get('/api/admin/update-category', { params: { ...data, admin_username, admin_password } })
  
  // 清除分类缓存
  if (res.code === 0) {
    apiCache.delete('getCategories')
  }
  
  return res.data
}

export const deleteCategory = async (id: number, admin_username: string = 'muteduanxing', admin_password: string = 'ahk12378dx') => {
  const res = await api.get('/api/admin/delete-category', { params: { id, admin_username, admin_password } })
  
  // 清除分类缓存
  if (res.code === 0) {
    apiCache.delete('getCategories')
  }
  
  return res.data
}



// ====== 统计相关API ======
export const getStats = withCache(async () => {
  const username = getCurrentUsername()
  const [res1, res2] = await Promise.all([
    api.get('/api/stats/api-counts', { params: { username } }),
    api.get('/api/stats/downloads', { params: { username } })
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
  const username = getCurrentUsername()
  const res = await api.get('/api/stats/api-calls', { params: { username } })
  return res.data
}, 'getApiCallStats', 25000) // 25秒缓存

export const getApiPerformance = withCache(async () => {
  const username = getCurrentUsername()
  const res = await api.get('/api/stats/api-performance', { params: { username } })
  return res.data
}, 'getApiPerformance', 30000) // 30秒缓存

export const getRecentCalls = withCache(async () => {
  const username = getCurrentUsername()
  const res = await api.get('/api/stats/recent-calls', { params: { username } })
  return res.data
}, 'getRecentCalls', 10000) // 10秒缓存

export const getDashboardData = withCache(async () => {
  console.log('开始请求仪表盘数据...')
  const res = await api.get('/api/dashboard')
  console.log('仪表盘数据请求成功:', res.data)
  return res.data
}, 'getDashboardData', 15000) // 15秒缓存

// ====== 日志相关API ======
export const getLogs = withCache(async () => {
  const username = getCurrentUsername()
  const res = await api.get('/api/logs/entries', { params: { username } })
  return res.data
}, 'getLogs', 15000) // 15秒缓存

export const getLogStats = withCache(async () => {
  const username = getCurrentUsername()
  const res = await api.get('/api/logs/stats', { params: { username } })
  return res.data
}, 'getLogStats', 30000) // 30秒缓存

export const getLogEntries = withCache(async (page: number = 1, pageSize: number = 20) => {
  const username = getCurrentUsername()
  const res = await api.get('/api/logs/entries', {
    params: { username, page, page_size: pageSize }
  })
  return res.data
}, 'getLogEntries', 10000) // 10秒缓存

export const clearLogs = async () => {
  const username = getCurrentUsername()
  const res = await api.get('/api/logs/clear', { params: { username } })
  return res.data
}

// ====== 社区相关API（复用现有接口） ======
export const communityApi = {
  // 获取资源列表（复用getPackages）
  getResources: async (params: {
    page?: number
    pageSize?: number
    category?: string
    status?: string
    search?: string
  }) => {
    const res = await getPackages()
    // 这里可以根据参数进行过滤和分页
    return res
  },

  // 获取资源详情
  getResource: async (id: number) => {
    const res = await getPackages()
    if (res.code === 0 && res.data) {
      const resource = res.data.绳包列表.find((item: any) => item.id === id)
      return { code: 0, data: resource }
    }
    return { code: 1, msg: '资源不存在' }
  },

  // 下载资源
  downloadResource: async (id: number) => {
    return await downloadPackage(id)
  },

  // 创建资源（复用addPackage）
  createResource: async (data: {
    title: string
    description: string
    category: string
    tags: string[]
    status: string
    file?: File
    cover?: File
  }) => {
    const username = getCurrentUsername()
    return await addPackage({
      name: data.title,
      author: username,
      version: '1.0.0',
      desc: data.description,
      url: data.file ? URL.createObjectURL(data.file) : '',
      username
    })
  },

  // 更新资源（复用updatePackage）
  updateResource: async (id: number, data: {
    title?: string
    description?: string
    category?: string
    tags?: string[]
    status?: string
    file?: File
    cover?: File
  }) => {
    const username = getCurrentUsername()
    return await updatePackage({
      id,
      name: data.title || '',
      author: username,
      version: '1.0.0',
      desc: data.description || '',
      url: data.file ? URL.createObjectURL(data.file) : '',
      username
    })
  },

  // 删除资源（复用deletePackage）
  deleteResource: async (id: number) => {
    const username = getCurrentUsername()
    return await deletePackage(id, username)
  },

  // 获取资源统计
  getResourceStats: async () => {
    const res = await getStats()
    return res
  }
}

// 导出缓存管理器，供外部使用
export { apiCache } 