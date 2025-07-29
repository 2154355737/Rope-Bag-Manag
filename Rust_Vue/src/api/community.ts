import { api, ApiResponse } from '../utils/apiClient'
import { categoryApi } from './categories' // 导入categoryApi

export interface Resource {
  id: number
  title?: string // 兼容旧字段
  name?: string  // 新字段
  description?: string
  author: string
  version?: string
  category?: string
  tags?: string[]
  status?: string
  file_url?: string
  cover_url?: string
  created_at?: string
  updated_at?: string
}

export const communityApi = {
  // 获取资源列表
  getResources: (params?: { page?: number; pageSize?: number; category?: string; status?: string; search?: string }): Promise<ApiResponse<{ list: Resource[]; total: number; page: number; pageSize: number; totalPages?: number }>> => {
    return api.get('/api/v1/packages', { params })
  },
  // 获取资源详情
  getResource: (id: number): Promise<ApiResponse<Resource>> => {
    return api.get(`/api/v1/packages/${id}`)
  },
  // 下载资源
  downloadResource: (id: number): Promise<ApiResponse<{ url: string }>> => {
    return api.get(`/api/v1/packages/${id}/download`)
  },
  // 创建资源
  createResource: (data: {
    title: string
    description: string
    category: string
    tags?: string[]
    file_url: string
    cover_url?: string
  }): Promise<ApiResponse> => {
    return api.post('/api/v1/packages', data)
  },
  // 更新资源
  updateResource: (id: number, data: any): Promise<ApiResponse> => {
    return api.put(`/api/v1/packages/${id}`, data)
  },
  // 删除资源
  deleteResource: (id: number): Promise<ApiResponse> => {
    return api.delete(`/api/v1/packages/${id}`)
  },
  // 批量删除资源
  batchDeleteResources: (ids: number[]): Promise<ApiResponse> => {
    return api.post('/api/v1/packages/batch-delete', { ids })
  },
  // 更新资源状态
  updateResourceStatus: (id: number, status: string): Promise<ApiResponse> => {
    return api.put(`/api/v1/packages/${id}/status`, { status })
  },
  // 批量更新资源状态
  batchUpdateResourceStatus: (ids: number[], status: string): Promise<ApiResponse> => {
    return api.post('/api/v1/packages/batch-update-status', { ids, status })
  },
  // 获取资源统计
  getResourceStats: (): Promise<ApiResponse<{ total: number }>> => {
    return api.get('/api/v1/packages/stats')
  },
  
  // 分类管理API
  category: {
    // 获取分类列表
    getCategories: (params?: {
      enabled?: boolean
      search?: string
    }): Promise<ApiResponse<any>> => {
      return api.get('/api/v1/categories', { params })
    },

    // 获取分类详情
    getCategory: (id: number): Promise<ApiResponse<any>> => {
      return api.get(`/api/v1/categories/${id}`)
    },

    // 创建分类
    createCategory: (data: {
      name: string
      description: string
      icon?: string
      color?: string
      sort?: number
      enabled: boolean
      tags?: string[]
    }): Promise<ApiResponse<any>> => {
      return api.post('/api/v1/categories', data)
    },

    // 更新分类
    updateCategory: (id: number, data: {
      name?: string
      description?: string
      icon?: string
      color?: string
      sort?: number
      enabled?: boolean
      tags?: string[]
    }): Promise<ApiResponse<any>> => {
      return api.put(`/api/v1/categories/${id}`, data)
    },

    // 删除分类
    deleteCategory: (id: number): Promise<ApiResponse<any>> => {
      return api.delete(`/api/v1/categories/${id}`)
    },

    // 批量删除分类
    batchDeleteCategories: (ids: number[]): Promise<ApiResponse<any>> => {
      return api.post('/api/v1/categories/batch-delete', { ids })
    },

    // 更新分类状态
    updateCategoryStatus: (id: number, enabled: boolean): Promise<ApiResponse<any>> => {
      return api.put(`/api/v1/categories/${id}/status`, { enabled })
    }
  }
}

// 导出社区用户API
export const communityUserApi = {
  // 获取用户列表
  getUsers: (params?: {
    page?: number
    pageSize?: number
    role?: string
    status?: string
    search?: string
  }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/users', { params })
  },
  
  // 其他用户相关API...
}

// 社区统计API
export const communityStatsApi = {
  // 获取社区总览统计
  getOverviewStats: (): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/stats/overview')
  },

  // 获取资源统计
  getResourceStats: (params?: {
    period?: string
    category?: string
  }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/stats/resources', { params })
  },

  // 获取用户统计
  getUserStats: (params?: {
    period?: string
    role?: string
  }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/stats/users', { params })
  },

  // 获取分类统计
  getCategoryStats: (params?: {
    period?: string
  }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/stats/categories', { params })
  },

  // 获取热门资源
  getHotResources: (params?: {
    period?: string
    limit?: number
  }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/stats/hot-resources', { params })
  },

  // 获取活跃用户
  getActiveUsers: (params?: {
    period?: string
    limit?: number
  }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/stats/active-users', { params })
  }
}

// 社区内容审核API
export const communityModerationApi = {
  // 获取待审核内容
  getPendingContent: (params: {
    page?: number
    pageSize?: number
    type?: 'resource' | 'comment' | 'user'
  }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/moderation/pending', { params })
  },

  // 审核资源
  reviewResource: (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }): Promise<ApiResponse<any>> => {
    return api.post(`/api/v1/moderation/resources/${id}/review`, data)
  },

  // 审核评论
  reviewComment: (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }): Promise<ApiResponse<any>> => {
    return api.post(`/api/v1/moderation/comments/${id}/review`, data)
  },

  // 审核用户
  reviewUser: (id: number, data: {
    action: 'approve' | 'reject'
    reason?: string
  }): Promise<ApiResponse<any>> => {
    return api.post(`/api/v1/moderation/users/${id}/review`, data)
  },

  // 批量审核
  batchReview: (data: {
    ids: number[]
    type: 'resource' | 'comment' | 'user'
    action: 'approve' | 'reject'
    reason?: string
  }): Promise<ApiResponse<any>> => {
    return api.post('/api/v1/moderation/batch-review', data)
  },

  // 获取审核统计
  getModerationStats: (): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/moderation/stats')
  }
}

export default {
  communityApi,
  communityUserApi,
  communityStatsApi,
  communityModerationApi
} 