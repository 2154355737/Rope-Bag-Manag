import { api, ApiResponse } from '../utils/apiClient'

export interface ResourceRecord {
  id: number
  resource_id: number
  user_id: number
  action: string
  created_at: string
  timestamp: string
  ip_address: string
  resource_type: string
  resource_name?: string
  user_name?: string
  old_data?: any
  new_data?: any
}

export interface CreateResourceRecordRequest {
  resource_id: number
  resource_type: string
  action: string
  old_data?: string
  new_data?: string
}

export interface ResourceActionStats {
  create_count: number
  update_count: number
  delete_count: number
  download_count: number
  by_day: Array<{
    date: string
    count: number
  }>
}

export const resourceRecordApi = {
  // 获取资源记录
  getResourceRecords: (params?: { page?: number; pageSize?: number; resource_id?: number; user_id?: number; action?: string; resource_type?: string; start_date?: string; end_date?: string }): Promise<ApiResponse<{ list: ResourceRecord[]; total: number; page: number; size: number }>> => {
    return api.get('/v1/resource-records', { params })
  },
  
  // 记录资源操作
  logResourceAction: (resourceId: number, action: string, additionalData?: { resource_type?: string; old_data?: any; new_data?: any }): Promise<ApiResponse> => {
    const data: CreateResourceRecordRequest = {
      resource_id: resourceId,
      action,
      resource_type: additionalData?.resource_type || "Package" // 默认为Package类型
    }
    
    // 添加可选字段
    if (additionalData?.old_data) {
      data.old_data = typeof additionalData.old_data === 'string' 
        ? additionalData.old_data 
        : JSON.stringify(additionalData.old_data)
    }
    
    if (additionalData?.new_data) {
      data.new_data = typeof additionalData.new_data === 'string' 
        ? additionalData.new_data 
        : JSON.stringify(additionalData.new_data)
    }
    
    console.log('记录资源操作:', data);
    return api.post('/v1/resource-records', data)
  },
  
  // 获取资源操作统计
  getResourceActionStats: (params?: { resource_type?: string; user_id?: number; start_date?: string; end_date?: string }): Promise<ApiResponse<ResourceActionStats>> => {
    return api.get('/v1/resource-records/stats', { params })
  },
  
  // 删除资源记录
  deleteResourceRecord: (recordId: number): Promise<ApiResponse> => {
    return api.delete(`/v1/resource-records/${recordId}`)
  },
  
  // 批量删除资源记录
  batchDeleteResourceRecords: (recordIds: number[]): Promise<ApiResponse> => {
    return api.post('/api/v1/resource-records/batch-delete', { ids: recordIds })
  },
  
  // 清空资源记录
  clearResourceRecords: (params?: { resource_id?: number; user_id?: number; before_date?: string }): Promise<ApiResponse> => {
    return api.post('/api/v1/resource-records/clear', params)
  },
} 