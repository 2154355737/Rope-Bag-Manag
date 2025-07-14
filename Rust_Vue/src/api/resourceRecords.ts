import { api, ApiResponse } from '../utils/apiClient'

export interface ResourceRecord {
  id: number
  resource_id: number
  user_id: number
  action: string
  created_at: string
  user_name: string
  resource_title: string
}

export const resourceRecordApi = {
  // 获取资源记录
  getResourceRecords: (params?: { page?: number; pageSize?: number; resource_id?: number; user_id?: number; action?: string; start_date?: string; end_date?: string }): Promise<ApiResponse<{ list: ResourceRecord[]; total: number; page: number; size: number }>> => {
    return api.get('/api/v1/resource-records', { params })
  },
  // 记录资源操作
  logResourceAction: (resourceId: number, action: string): Promise<ApiResponse> => {
    return api.post('/api/v1/resource-records', { resource_id: resourceId, action })
  },
  // 删除资源记录
  deleteResourceRecord: (recordId: number): Promise<ApiResponse> => {
    return api.delete(`/api/v1/resource-records/${recordId}`)
  },
  // 批量删除资源记录
  batchDeleteResourceRecords: (recordIds: number[]): Promise<ApiResponse> => {
    return api.post('/api/v1/resource-records/batch-delete', { ids: recordIds })
  },
  // 清空资源记录
  clearResourceRecords: (params?: { resource_id?: number; user_id?: number; before_date?: string }): Promise<ApiResponse> => {
    return api.post('/api/v1/resource-records/clear', params)
  },
  // 获取资源操作统计
  getResourceActionStats: (params?: { resource_id?: number; user_id?: number; start_date?: string; end_date?: string }): Promise<ApiResponse<any>> => {
    return api.get('/api/v1/resource-records/stats', { params })
  }
} 