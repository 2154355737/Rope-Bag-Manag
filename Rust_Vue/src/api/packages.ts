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
  file_url?: string
}

// 更新绳包请求
export interface UpdatePackageRequest {
  name?: string
  version?: string
  description?: string
  category_id?: number
  status?: string | 'Active' | 'Inactive' | 'Deleted'
  file_url?: string
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
    
    // 添加分页参数
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

    console.log("请求参数:", queryParams.toString())
    return api.get(`/api/v1/packages?${queryParams.toString()}`)
  },

  // 获取单个绳包
  getPackage: (id: number): Promise<ApiResponse<Package>> => {
    return api.get(`/api/v1/packages/${id}`)
  },

  // 创建绳包
  createPackage: (data: CreatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.post('/api/v1/packages', data).then(response => {
      // 如果创建成功，记录资源操作
      if (response.code === 0 && response.data && response.data.id) {
        console.log('自动记录创建绳包操作:', response.data.id)
        resourceLogger.logCreate(response.data.id, 'Package', response.data)
          .catch(err => console.error('记录创建操作失败:', err))
      }
      return response
    })
  },

  // 更新绳包
  updatePackage: (id: number, data: UpdatePackageRequest): Promise<ApiResponse<Package>> => {
    return api.put(`/api/v1/packages/${id}`, data).then(response => {
      // 如果更新成功，记录资源操作
      if (response.code === 0) {
        console.log('自动记录更新绳包操作:', id)
        resourceLogger.logUpdate(id, 'Package', null, response.data)
          .catch(err => console.error('记录更新操作失败:', err))
      }
      return response
    })
  },

  // 删除绳包
  deletePackage: (id: number): Promise<ApiResponse<null>> => {
    return api.delete(`/api/v1/packages/${id}`).then(response => {
      // 如果删除成功，记录资源操作
      if (response.code === 0) {
        console.log('自动记录删除绳包操作:', id)
        resourceLogger.logDelete(id, 'Package')
          .catch(err => console.error('记录删除操作失败:', err))
      }
      return response
    })
  },

  // 下载绳包
  downloadPackage: (id: number): Promise<ApiResponse<string>> => {
    return api.get(`/api/v1/packages/${id}/download`).then(response => {
      // 如果下载成功，记录资源操作
      if (response.code === 0) {
        console.log('自动记录下载绳包操作:', id)
        resourceLogger.logDownload(id, 'Package')
          .catch(err => console.error('记录下载操作失败:', err))
      }
      return response
    })
  },

  // 上传绳包文件
  uploadPackageFile: (id: number, file: File): Promise<ApiResponse<Package>> => {
    const formData = new FormData()
    formData.append('file', file)
    return api.upload(`/api/v1/packages/${id}/upload`, formData).then(response => {
      // 如果上传成功，记录资源操作
      if (response.code === 0) {
        console.log('自动记录上传绳包操作:', id)
        resourceLogger.logUpload(id, 'Package', { filename: file.name, size: file.size })
          .catch(err => console.error('记录上传操作失败:', err))
      }
      return response
    })
  },
} 