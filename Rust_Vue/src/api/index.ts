import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 15000, // 增加超时时间到15秒
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

export async function login(username: string, password: string) {
  const res = await api.get('/login', { params: { username, password } })
  return res.data
}

export async function getUsers() {
  const res = await api.get('/get-users-db')
  return res.data
}

export async function getPackages() {
  // 从localStorage获取当前用户信息
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin' // 默认使用admin
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  const res = await api.get('/get-data-db', {
    params: { username }
  })
  return res.data
}

export async function getLogs() {
  const res = await api.get('/logs/entries')
  return res.data
}

export async function getStats() {
  // 从localStorage获取当前用户信息
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin' // 默认使用admin
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  const res = await api.get('/stats/api-counts', {
    params: { username }
  })
  const res2 = await api.get('/stats/downloads', {
    params: { username }
  })
  return { code: 0, data: { api_counts: res.data.data, downloads: res2.data.data } }
}

// 获取API调用统计
export async function getApiCallStats() {
  // 从localStorage获取当前用户信息
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin' // 默认使用admin
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  const res = await api.get('/stats/api-calls', {
    params: { username }
  })
  // 直接返回完整data，包含api_last_used
  return res.data
}

// 获取API性能详情
export async function getApiPerformance() {
  // 从localStorage获取当前用户信息
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin' // 默认使用admin
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  const res = await api.get('/stats/api-performance', {
    params: { username }
  })
  return res.data
}

// 获取最近API调用记录
export async function getRecentCalls() {
  // 从localStorage获取当前用户信息
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin' // 默认使用admin
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  const res = await api.get('/stats/recent-calls', {
    params: { username }
  })
  return res.data
}

export async function getDashboardData() {
  console.log('开始请求仪表盘数据...')
  console.log('请求URL:', '/api/dashboard')
  const res = await api.get('/dashboard')
  console.log('仪表盘数据请求成功:', res.data)
  return res.data
}

export async function banUser(username: string, banned: boolean) {
  // 管理员接口，需传 admin_username/admin_password
  const res = await api.get('/admin/ban-user', {
    params: { target: username, banned, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export async function setStar(username: string, star: number) {
  // 管理员接口，需传 admin_username/admin_password
  const res = await api.get('/admin/set-star', {
    params: { target: username, star, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export async function setNickname(username: string, nickname: string) {
  // 管理员接口，需传 admin_username/admin_password
  const res = await api.get('/admin/set-user', {
    params: { target: username, nickname, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export async function setPassword(username: string, password: string) {
  // 管理员接口，需传 admin_username/admin_password
  const res = await api.get('/admin/set-user', {
    params: { target: username, password, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export async function addPackage(data: any) {
  // 管理员接口，需传 admin_username/admin_password
  const res = await api.get('/admin/add-rope-package', {
    params: { ...data, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export async function updatePackage(data: any) {
  // 管理员接口，需传 admin_username/admin_password
  const res = await api.get('/admin/update-rope-package', {
    params: { ...data, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export async function deletePackage(id: number) {
  // 管理员接口，需传 admin_username/admin_password
  const res = await api.get('/admin/delete-rope-package', {
    params: { id, admin_username: 'admin', admin_password: 'admin123' }
  })
  return res.data
}

export async function getLogEntries(page: number = 1, pageSize: number = 20) {
  const res = await api.get('/logs/entries', {
    params: { page, page_size: pageSize }
  })
  return res.data
} 