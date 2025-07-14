import { api, ApiResponse } from '../utils/apiClient'

export interface Category {
  id: number
  name: string
  enabled: boolean
  created_at: string
  updated_at: string
}

export const categoryApi = {
  getCategories: (): Promise<ApiResponse<{ list: Category[] }>> => {
    return api.get('/api/v1/categories')
  },
  addCategory: (data: { name: string }): Promise<ApiResponse<Category>> => {
    return api.post('/api/v1/categories', data)
  },
  updateCategory: (id: number, data: { name?: string; enabled?: boolean }): Promise<ApiResponse<Category>> => {
    return api.put(`/api/v1/categories/${id}`, data)
  },
  deleteCategory: (id: number): Promise<ApiResponse<null>> => {
    return api.delete(`/api/v1/categories/${id}`)
  }
} 