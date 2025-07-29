// 统一的数据类型定义

// 资源相关类型
export interface Resource {
  id: number
  绳包名称: string
  作者: string
  版本: string
  简介: string
  项目直链: string
  下载次数: number
  上架时间: string
  likes?: number
  favorites?: number
  标签?: string[]
  category?: string
  status?: ResourceStatus
  cover?: string
}

export enum ResourceStatus {
  Published = 'published',
  Pending = 'pending',
  Rejected = 'rejected'
}

// 用户相关类型
export interface User {
  id: number
  username: string
  email: string
  nickname?: string
  password_hash?: string
  star: number
  ban_status: BanStatus
  ban_reason?: string | null
  qq_number?: string | null
  avatar_url?: string | null
  login_count: number
  upload_count: number
  download_count: number
  created_at: string
  last_login?: string | null
  is_admin: boolean
  role: UserRole
}

export enum UserRole {
  Admin = 'admin',
  Moderator = 'moderator',
  Elder = 'elder',
  User = 'user'
}

export enum BanStatus {
  Normal = 'normal',
  Suspended = 'suspended',
  Banned = 'banned'
}

export enum UserStatus {
  Online = 'online',
  Offline = 'offline'
}

// API响应类型
export interface ApiResponse<T = any> {
  code: number
  msg: string
  data?: T
}

// 分页相关类型
export interface PaginationParams {
  page?: number
  pageSize?: number
}

export interface PaginationResult<T> {
  data: T[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

// 统计相关类型
export interface StatsData {
  api_counts: Record<string, number>
  downloads: Record<string, number>
  api_calls?: ApiCallRecord[]
  api_performance?: Record<string, ApiPerformance>
}

export interface ApiCallRecord {
  timestamp: number
  api_name: string
  username: string
  ip_address: string
  user_agent: string
  response_time_ms: number
  status_code: number
  success: boolean
  error_message?: string
}

export interface ApiPerformance {
  total_calls: number
  success_calls: number
  failed_calls: number
  avg_response_time_ms: number
  min_response_time_ms: number
  max_response_time_ms: number
  last_called: number
  unique_users: Set<string>
}

// 日志相关类型
export interface LogEntry {
  timestamp: string
  level: string
  message: string
  module: string
  details?: any
}

// 分类相关类型
export interface Category {
  id: number
  name: string
  description: string
  icon: string
  color: string
  sort: number
  enabled: boolean
  tags: string[]
  count: number
}

// 上传相关类型
export interface UploadForm {
  title: string
  description: string
  category: string
  tags: string[]
  tagsInput?: string
  file_url?: string
  cover_url?: string
  /** 资源文件对象，可在上传组件中使用 */
  file?: File
  /** 封面文件对象，可在上传组件中使用 */
  cover?: File
  /** 资源状态，例如 active */
  status?: string
}

// 搜索和筛选类型
export interface SearchParams {
  search?: string
  category?: string
  status?: string
  role?: string
  sortBy?: string
  filterType?: string
}

// 路由元信息类型
export interface RouteMeta {
  title?: string
  requiresAuth?: boolean
  requiresAdmin?: boolean
  layout?: 'desktop' | 'mobile' | 'independent'
  preload?: boolean
  device?: 'desktop' | 'mobile' | 'all'
}

// 主题相关类型
export interface ThemeConfig {
  mode: 'light' | 'dark'
  primaryColor: string
  borderRadius: number
  fontSize: number
}

// 设备相关类型
export interface DeviceInfo {
  type: 'desktop' | 'mobile' | 'tablet'
  width: number
  height: number
  userAgent: string
}

// 缓存相关类型
export interface CacheConfig {
  key: string
  ttl: number
  maxSize?: number
}

// 错误相关类型
export interface ApiError {
  code: number
  message: string
  statusCode?: number
  details?: any
}

// 表单验证规则类型
export interface ValidationRule {
  required?: boolean
  min?: number
  max?: number
  pattern?: RegExp
  message?: string
  trigger?: 'blur' | 'change' | 'input'
}

// 表格列配置类型
export interface TableColumn {
  prop: string
  label: string
  width?: number | string
  minWidth?: number | string
  fixed?: boolean | 'left' | 'right'
  sortable?: boolean
  formatter?: (row: any, column: any, cellValue: any) => string
  render?: (row: any, column: any, cellValue: any) => any
}

// 菜单项类型
export interface MenuItem {
  id: string
  title: string
  icon?: string
  path?: string
  children?: MenuItem[]
  meta?: {
    requiresAuth?: boolean
    requiresAdmin?: boolean
    badge?: number | string
  }
}

// 通知消息类型
export interface Notification {
  id: string
  type: 'success' | 'warning' | 'error' | 'info'
  title: string
  message: string
  duration?: number
  showClose?: boolean
  timestamp: number
}

// 文件上传相关类型
export interface UploadConfig {
  accept?: string
  maxSize?: number
  multiple?: boolean
  autoUpload?: boolean
  showFileList?: boolean
  drag?: boolean
}

// 图表数据类型
export interface ChartData {
  labels: string[]
  datasets: {
    label: string
    data: number[]
    backgroundColor?: string | string[]
    borderColor?: string | string[]
    borderWidth?: number
  }[]
}

// 仪表盘数据类型
export interface DashboardData {
  userCount: number
  packageCount: number
  logCount: number
  activeUsers: number
  availablePackages: number
  todayLogs: number
  systemStatus: string
  uptime: string
  cpuUsage: number
  memoryUsage: number
  recentActivity: any[]
  charts: {
    userGrowth: ChartData
    packageDownloads: ChartData
    apiCalls: ChartData
  }
} 