import { api, ApiResponse } from '../utils/apiClient'

export interface Category {
  id: number
  name: string
  description: string | null
  enabled: boolean
  created_at: string
}

export interface CreateCategoryRequest {
  name: string
  description?: string
  enabled?: boolean
}

export interface UpdateCategoryRequest {
  name?: string
  description?: string
  enabled?: boolean
}

export const categoryApi = {
  getCategories: (): Promise<ApiResponse<{ list: Category[] }>> => {
    return api.get('/api/v1/categories')
  },
  
  getCategory: (id: number): Promise<ApiResponse<Category>> => {
    return api.get(`/api/v1/categories/${id}`)
  },
  
  addCategory: (data: CreateCategoryRequest): Promise<ApiResponse<Category>> => {
    return api.post('/api/v1/categories', data)
  },
  
  updateCategory: (id: number, data: UpdateCategoryRequest): Promise<ApiResponse<Category>> => {
    return api.put(`/api/v1/categories/${id}`, data)
  },
  
  deleteCategory: (id: number): Promise<ApiResponse<null>> => {
    return api.delete(`/api/v1/categories/${id}`)
  }
} 