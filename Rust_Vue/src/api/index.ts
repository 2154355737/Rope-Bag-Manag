// 统一导出各模块的API
import { commentApi } from './comments'
import { userApi } from './users'
import { packageApi } from './packages'
import { categoryApi } from './categories'
import { apiCache } from './cache'
import { adminApi } from './admin'
import { authApi } from './auth'
import { settingsApi } from './settings'
import { resourceRecordApi } from './resourceRecords'
import { communityApi } from './community'

export {
  commentApi,
  userApi,
  packageApi,
  categoryApi,
  apiCache,
  adminApi,
  authApi,
  settingsApi,
  resourceRecordApi,
  communityApi
}

// 导出一个默认包含所有API的对象
const api = {
  comment: commentApi,
  user: userApi,
  package: packageApi,
  category: categoryApi,
  cache: apiCache,
  admin: adminApi,
  auth: authApi,
  settings: settingsApi,
  resourceRecord: resourceRecordApi,
  community: communityApi
}

export default api

// 重新导出类型
export type { LoginRequest, RegisterRequest, UserInfo, LoginResponse } from './auth'
export type { User, UserQueryParams, UserListResponse, UpdateUserRequest } from './users'
export type { Package, CreatePackageRequest, UpdatePackageRequest, PackageQueryParams, PackageListResponse } from './packages'
export type { Stats, SystemLog, UserAction } from './admin'
export type { Category } from './categories'
export type { Comment } from './comments'
export type { SystemSettings } from './settings'
export type { CacheData } from './cache'

// 导出API客户端工具
export { api, healthCheck, setToken, getToken, clearToken, isLoggedIn } from '../utils/apiClient'
export type { ApiResponse } from '../utils/apiClient' 