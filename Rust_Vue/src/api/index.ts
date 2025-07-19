// 导出所有API模块
export { authApi } from './auth'
export { userApi } from './users'
export { packageApi } from './packages'
export { adminApi } from './admin'
export { categoryApi } from './categories'
export { commentApi } from './comments'
export { settingsApi } from './settings'
export { apiCache } from './cache'

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