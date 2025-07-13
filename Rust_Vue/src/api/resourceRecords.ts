import axios from 'axios'
import type { ApiResponse } from './types'

// 创建API客户端
const api = axios.create({
  baseURL: 'http://127.0.0.1:15202',
  timeout: 15000,
})

// 通用请求函数
const request = async (config: any) => {
  try {
    const response = await api(config)
    return response.data
  } catch (error) {
    console.error('API请求失败:', error)
    throw error
  }
}

// 资源记录相关接口类型
export interface ResourceRecord {
  id: string
  resource_id: string
  resource_name: string
  resource_type: string
  user_id: string
  username: string
  nickname: string
  action_type: string
  action_name: string
  file_size?: number
  file_path?: string
  download_count?: number
  view_count?: number
  create_time: string
  update_time: string
}

export interface ResourceRecordQuery {
  resource_id?: string
  user_id?: string
  action_type?: string
  resource_type?: string
  start_time?: string
  end_time?: string
  page?: number
  size?: number
}

// 获取资源记录列表
export function getResourceRecords(params?: ResourceRecordQuery): Promise<ApiResponse<{
  records: ResourceRecord[]
  total: number
  page: number
  size: number
}>> {
  return request({
    url: '/api/resource-records',
    method: 'GET',
    params
  })
}

// 记录资源操作
export function logResourceAction(data: {
  resource_id: string
  resource_name: string
  resource_type: string
  action_type: string
  action_name: string
  file_size?: number
  file_path?: string
}): Promise<ApiResponse<ResourceRecord>> {
  return request({
    url: '/api/resource-records',
    method: 'POST',
    data
  })
}

// 获取资源统计信息
export function getResourceStats(params?: {
  resource_id?: string
  user_id?: string
  action_type?: string
  start_time?: string
  end_time?: string
}): Promise<ApiResponse<{
  total_records: number
  action_types: Record<string, number>
  resource_types: Record<string, number>
  daily_stats: Array<{
    date: string
    count: number
  }>
}>> {
  return request({
    url: '/api/resource-records/stats',
    method: 'GET',
    params
  })
}

// 删除资源记录
export function deleteResourceRecord(id: string): Promise<ApiResponse<any>> {
  return request({
    url: `/api/resource-records/${id}`,
    method: 'DELETE'
  })
}

// 批量删除资源记录
export function batchDeleteResourceRecords(ids: string[]): Promise<ApiResponse<any>> {
  return request({
    url: '/api/resource-records/batch',
    method: 'DELETE',
    data: { ids }
  })
}

// 导出资源记录
export function exportResourceRecords(params?: ResourceRecordQuery): Promise<ApiResponse<{
  download_url: string
  filename: string
}>> {
  return request({
    url: '/api/resource-records/export',
    method: 'GET',
    params
  })
} 