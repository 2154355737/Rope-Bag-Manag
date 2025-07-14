import { api, ApiResponse } from '../utils/apiClient'

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
  status: string
  created_at: string
  updated_at: string
}

// 创建绳包请求
export interface CreatePackageRequest {
  name: string
  author: string
  version?: string
  description?: string
  category_id?: number
}

// 更新绳包请求
export interface UpdatePackageRequest {
  name?: string
  version?: string
  description?: string
  category_id?: number
  status?: string
}

// 绳包列表查询参数
export interface PackageQueryParams {
  page?: number
  pageSize?: number
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
    if (params?.page) queryParams.append('page', params.page.toString())
    if (params?.pageSize) queryParams.append('page_size', params.pageSize.toString())
    if (params?.category_id) queryParams.append('category_id', params.category_id.toString())
    if (params?.status) queryParams.append('status', params.status)
    if (params?.search) queryParams.append('search', params.search)

    return api.get(`/api/v1/packages?${queryParams.toString()}`)
  },

  // 获取单个绳包
  getPackage: (id: number): Promise<ApiResponse<Package>> => {
    return api.get(`/api/v1/packages/${id}`)
  },

  // 创建绳包
  createPackage: (data: CreatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.post('/api/v1/packages', data)
  },

  // 更新绳包
  updatePackage: (id: number, data: UpdatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.put(`/api/v1/packages/${id}`, data)
  },

  // 删除绳包
  deletePackage: (id: number): Promise<ApiResponse<null>> => {
    return api.delete(`/api/v1/packages/${id}`)
  },

  // 下载绳包
  downloadPackage: (id: number): Promise<ApiResponse<string>> => {
    return api.get(`/api/v1/packages/${id}/download`)
  },

  // 上传绳包文件
  uploadPackageFile: (id: number, file: File): Promise<ApiResponse<Package>> => {
    const formData = new FormData()
    formData.append('file', file)
    return api.upload(`/api/v1/packages/${id}/upload`, formData)
  },
} 