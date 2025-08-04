import { api, ApiResponse } from '../utils/apiClient'
import { resourceLogger } from '../utils/loggerService'

// 绳包信息类型
export interface Package {
  id: number
  name: string
  author: string
  version: string | null
  description: string | null
  file_url: string
  file_size: number | null
  download_count: number
  like_count: number
  favorite_count: number
  category_id: number | null
  status: 'Pending' | 'Active' | 'Rejected' | 'Inactive' | 'Deleted'
  created_at: string
  updated_at: string
  // 审核相关字段
  reviewer_id?: number | null
  reviewed_at?: string | null
  review_comment?: string | null
  // 前端展示需要的额外字段
  tags?: string[]
  category?: string
  cover?: string
  is_featured?: boolean
  is_pinned?: boolean
}

// 创建绳包请求
export interface CreatePackageRequest {
  name: string
  author: string
  version?: string
  description?: string
  category_id?: number
  file_url?: string
  tags?: string[]
  is_pinned?: boolean
  is_featured?: boolean
}

// 更新绳包请求
export interface UpdatePackageRequest {
  name?: string
  version?: string
  description?: string
  category_id?: number
  status?: 'Pending' | 'Active' | 'Rejected' | 'Inactive' | 'Deleted'
  file_url?: string
  tags?: string[]
  is_pinned?: boolean
  is_featured?: boolean
  // 审核相关字段
  reviewer_id?: number
  reviewed_at?: string
  review_comment?: string
}

// 审核资源请求
export interface ReviewResourceRequest {
  status: 'approved' | 'rejected'
  comment?: string
}

// 绳包列表查询参数
export interface PackageQueryParams {
  page?: number
  pageSize?: number  // 前端使用pageSize，但会转换为page_size发送给后端
  category_id?: number
  status?: string
  search?: string
}

// 绳包列表响应
export interface PackageListResponse {
  list: Package[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

// 绳包API
export const packageApi = {
  // 获取绳包列表
  getPackages: (params?: PackageQueryParams): Promise<ApiResponse<PackageListResponse>> => {
    const queryParams = new URLSearchParams()
    
    // 添加分页参数 - 注意：后端期望page_size，前端传入pageSize
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())
    
    // 添加分类过滤
    if (params?.category_id !== undefined) {
      console.log("添加分类过滤:", params.category_id)
      queryParams.append('category_id', params.category_id.toString())
    }
    
    // 添加状态过滤
    if (params?.status) queryParams.append('status', params.status)
    
    // 添加搜索过滤
    if (params?.search) queryParams.append('search', params.search)
    
    const queryString = queryParams.toString()
    const url = queryString ? `/v1/packages?${queryString}` : '/v1/packages'
    
    return api.get(url)
  },

  // 获取单个绳包
  getPackage: (id: number): Promise<ApiResponse<Package>> => {
    return api.get(`/v1/packages/${id}`)
  },

  // 创建绳包
  createPackage: (data: CreatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.post('/v1/packages', data)
  },

  // 用户提交资源（普通用户使用）
  userSubmitResource: (data: CreatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.post('/v1/packages/user-submit', data)
  },

  // 管理员创建资源（管理员/元老使用）
  adminCreatePackage: (data: CreatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.post('/v1/packages/admin-create', data)
  },

  // 更新绳包
  updatePackage: (id: number, data: UpdatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.put(`/v1/packages/${id}`, data)
  },

  // 删除绳包
  deletePackage: (id: number): Promise<ApiResponse<null>> => {
    return api.delete(`/v1/packages/${id}`)
  },

  // 批量删除绳包
  batchDeletePackages: (ids: number[]): Promise<ApiResponse<null>> => {
    return api.post('/v1/packages/batch-delete', { ids })
  },

  // 下载绳包
  downloadPackage: (id: number): Promise<ApiResponse<{ url: string }>> => {
    return api.get(`/v1/packages/${id}/download`)
  },

  // 上传绳包文件
  uploadPackageFile: (id: number, formData: FormData): Promise<ApiResponse<{ file_url: string }>> => {
    return api.upload(`/v1/packages/${id}/upload`, formData)
  },

  // 获取绳包评论
  getPackageComments: (id: number): Promise<ApiResponse<any[]>> => {
    return api.get(`/v1/packages/${id}/comments`)
  },

  // 审核资源
  reviewResource: (id: number, data: ReviewResourceRequest): Promise<ApiResponse<Package>> => {
    return api.post(`/v1/packages/${id}/review`, data)
  },

  // 获取待审核资源列表
  getPendingResources: (params?: PackageQueryParams): Promise<ApiResponse<PackageListResponse>> => {
    const queryParams = new URLSearchParams()
    
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())
    if (params?.search) queryParams.append('search', params.search)
    
    const queryString = queryParams.toString()
    const url = queryString ? `/v1/packages/pending?${queryString}` : '/v1/packages/pending'
    
    return api.get(url)
  },

  // 获取包分类
  getPackageCategories: (): Promise<ApiResponse<any[]>> => {
    return api.get('/v1/packages/categories')
  },

  // 批量更新状态
  batchUpdateStatus: (ids: number[], status: string): Promise<ApiResponse<null>> => {
    return api.post('/v1/packages/batch-update-status', { ids, status })
  },
} 