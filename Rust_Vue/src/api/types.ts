// API响应类型定义
export interface ApiResponse<T = any> {
  code: number
  msg: string
  data: T
}

// 分页响应类型
export interface PaginatedResponse<T> {
  list: T[]
  total: number
  page: number
  size: number
}

// 用户相关类型
export interface User {
  id: number
  username: string
  email: string
  nickname?: string
  role: string
  star: number
  banned: boolean
  ban_status?: string
  ban_reason?: string | null
  qq_number?: string | null
  avatar_url?: string | null
  login_count: number
  upload_count: number
  download_count: number
  created_at: string
  last_login?: string | null
  is_admin: boolean
  create_time: string
  last_login_time?: string
}

export interface UserInfo {
  id: number
  username: string
  email: string
  nickname?: string
  role: string
  star: number
  banned: boolean
  ban_status?: string
  ban_reason?: string | null
  qq_number?: string | null
  avatar_url?: string | null
  login_count: number
  upload_count: number
  download_count: number
  created_at: string
  last_login?: string | null
  is_admin: boolean
  create_time: string
  last_login_time?: string
  qq_binding?: string
}

// 资源相关类型
export interface Package {
  id: number
  name: string
  author: string
  version: string
  desc: string
  url: string
  category: string
  status: string
  create_time: string
  update_time: string
}

// 分类相关类型
export interface Category {
  id: number
  name: string
  description: string
  enabled: boolean
  create_time: string
}

// 日志相关类型
export interface LogEntry {
  id: string
  level: string
  message: string
  timestamp: string
  source: string
  details?: Record<string, any>
}

// 统计相关类型
export interface Stats {
  total_users: number
  total_packages: number
  total_categories: number
  active_users: number
  new_users_today: number
  new_packages_today: number
  system_status: string
  uptime: number
}

// 设置相关类型
export interface SystemSettings {
  theme: {
    community_theme: string
    admin_theme: string
  }
  system_mode: string
  feature_flags: {
    enable_registration: boolean
    enable_community: boolean
    enable_upload: boolean
    enable_comments: boolean
    enable_qq_binding: boolean
  }
  backend_config: {
    proxy_address: string
    api_timeout: number
    max_upload_size: number
  }
  backup_settings: {
    enable_auto_backup: boolean
    backup_interval_hours: number
    backup_location: string
    max_backup_files: number
  }
  global_announcement: {
    enabled: boolean
    title: string
    content: string
    type_: string
    start_time: string
    end_time: string
    priority: number
  }
} 