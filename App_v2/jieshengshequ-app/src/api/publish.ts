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

// 发布资源
export async function publishResource(data: PublishResourceRequest): Promise<PublishResponse> {
  return http.post<PublishResponse>('/publish/resource', data)
}

// 发布帖子  
export async function publishPost(data: PublishPostRequest): Promise<PublishResponse> {
  return http.post<PublishResponse>('/publish/post', data)
}

// 上传文件到指定资源
export async function uploadResourceFile(resourceId: number, file: File): Promise<{ file_path: string; file_name: string }> {
  const formData = new FormData()
  formData.append('file', file)
  
  return http.post<{ file_path: string; file_name: string }>(`/packages/${resourceId}/upload`, formData)
}

// 上传图片
export async function uploadImage(file: File, packageId?: number): Promise<{ file_path: string; download_url: string; file_size: number }> {
  const formData = new FormData()
  formData.append('file', file)
  if (packageId) {
    formData.append('package_id', packageId.toString())
  }
  
  return http.post<{ file_path: string; download_url: string; file_size: number }>('/storage/upload', formData)
}

// 上传帖子图片并绑定到帖子
export async function uploadPostImage(file: File, postId: number): Promise<{ file_path: string; download_url: string; file_size: number }> {
  const formData = new FormData()
  formData.append('file', file)
  formData.append('post_id', String(postId))
  return http.post<{ file_path: string; download_url: string; file_size: number }>('/storage/upload', formData)
} 

// 验证文件是否已成功上传到存储系统
export async function verifyFile(filePath: string): Promise<VerifyFileResponse> {
  return http.post<VerifyFileResponse>('/storage/verify', { file_path: filePath })
}
