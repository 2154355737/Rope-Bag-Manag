import { http } from './client'

// 文件验证接口
export interface VerifyFileRequest {
  file_path: string
}

export interface VerifyFileResponse {
  exists: boolean
  file_size?: number
  download_url?: string
}

// 发布数据类型定义
export interface PublishFileInfo {
  name: string
  size: number
  type?: string
}

export interface PublishResourceRequest {
  title: string
  content: string
  version?: string
  category?: string
  tags?: string[]
  requirements?: string[]
  files?: PublishFileInfo[]
  screenshots?: PublishFileInfo[]
}

export interface PublishPostRequest {
  title: string
  content: string
  tags?: string[]
  images?: PublishFileInfo[]
  code_snippet?: string
}

export interface PublishResponse {
  id: number
  title: string
  status: string
  created_at: string
}

// 发布资源（REST）
export async function publishResource(data: PublishResourceRequest): Promise<PublishResponse> {
  return http.post<PublishResponse>('/publish/resource', data)
}

// 发布帖子（REST）
export async function publishPost(data: PublishPostRequest): Promise<PublishResponse> {
  return http.post<PublishResponse>('/publish/post', data)
}

// 统一：上传资源文件，经由通用存储端点并绑定 package_id
export async function uploadResourceFile(resourceId: number, file: File): Promise<{ file_path: string; download_url: string; file_size: number }> {
  const formData = new FormData()
  // 使用"file"作为字段名
  formData.append('file', file)
  formData.append('package_id', String(resourceId))
  console.log('开始上传资源文件:', file.name, '大小:', file.size, '资源ID:', resourceId)
  
  // 注意：API_BASE已经包含了/api/v1前缀，所以这里不需要再加
  // 直接使用不带前缀的路径
  const url = '/storage/upload';
  console.log('完整上传URL:', url);
  return http.post<{ file_path: string; download_url: string; file_size: number }>(url, formData)
}

// 上传图片
export async function uploadImage(file: File, packageId?: number): Promise<{ file_path: string; download_url: string; file_size: number }> {
  const formData = new FormData()
  // 使用"file"作为字段名
  formData.append('file', file)
  if (packageId) {
    formData.append('package_id', packageId.toString())
  }
  
  console.log('开始上传图片:', file.name, '大小:', file.size, 'packageId:', packageId)
  // 注意：API_BASE已经包含了/api/v1前缀，所以这里不需要再加
  const url = '/storage/upload';
  console.log('完整上传URL:', url);
  const result = await http.post<{ file_path: string; download_url: string; file_size: number }>(url, formData)
  console.log('上传响应:', result)
  return result
}

// 上传帖子图片并绑定到帖子
export async function uploadPostImage(file: File, postId: number): Promise<{ file_path: string; download_url: string; file_size: number }> {
  const formData = new FormData()
  // 使用"file"作为字段名
  formData.append('file', file)
  formData.append('post_id', String(postId))
  console.log('开始上传帖子图片:', file.name, '大小:', file.size, '帖子ID:', postId)
  // 注意：API_BASE已经包含了/api/v1前缀，所以这里不需要再加
  const url = '/storage/upload';
  console.log('完整上传URL:', url);
  return http.post<{ file_path: string; download_url: string; file_size: number }>(url, formData)
} 

// 验证文件是否已成功上传到存储系统
export async function verifyFile(filePath: string): Promise<VerifyFileResponse> {
  console.log('开始验证文件:', filePath)
  const result = await http.post<VerifyFileResponse>('/storage/verify', { file_path: filePath })
  console.log('验证响应:', result)
  return result
}