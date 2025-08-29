import { defHttp } from '/@/utils/http/axios'

export interface PackageItem {
  id: number
  name: string
  author: string
  version?: string
  description?: string
  file_url: string
  file_size?: number
  download_count: number
  like_count: number
  favorite_count: number
  view_count?: number
  category_id?: number
  status: 'Pending' | 'Active' | 'Rejected' | 'Inactive' | 'Deleted'
  created_at: string
  updated_at: string
  reviewer_id?: number
  reviewed_at?: string
  review_comment?: string
  is_pinned: boolean
  is_featured: boolean
  tags?: string[]
}

export interface PackageListResponse {
  list: PackageItem[]
  total: number
  page?: number
  page_size?: number
}

export interface CreatePackagePayload {
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

export interface UpdatePackagePayload {
  name?: string
  version?: string
  description?: string
  category_id?: number
  tags?: string[]
  status?: 'Pending' | 'Active' | 'Rejected' | 'Inactive' | 'Deleted'
  is_pinned?: boolean
  is_featured?: boolean
  review_comment?: string
  author?: string
}

export interface ReviewPayload {
  status: 'approved' | 'rejected'
  comment?: string
}

const Api = {
  Packages: '/api/v1/packages',
  AdminCreate: '/api/v1/packages/admin-create',
  TopDownloads: '/api/v1/packages/top-downloads',
  TopLikes: '/api/v1/packages/top-likes',
}

const uploadUrl = (id: number) => `/api/v1/packages/${id}/upload`
const reviewUrl = (id: number) => `/api/v1/packages/${id}/review`

export function getPackages(params?: { page?: number; page_size?: number; category_id?: number; search?: string; status?: string }) {
  return defHttp.get<PackageListResponse>({ url: Api.Packages, params })
}

export function createPackage(data: CreatePackagePayload) {
  return defHttp.post<any>({ url: Api.AdminCreate, data })
}

export function updatePackage(id: number, data: UpdatePackagePayload) {
  return defHttp.put<any>({ url: `${Api.Packages}/${id}`, data })
}

export function deletePackage(id: number) {
  return defHttp.delete<any>({ url: `${Api.Packages}/${id}` })
}

export function uploadPackageFile(id: number, file: File) {
  const formData = new FormData()
  formData.append('file', file)
  return defHttp.post<any>({ url: uploadUrl(id), data: formData, headers: { 'Content-Type': 'multipart/form-data' } })
}

export function reviewPackage(id: number, data: ReviewPayload) {
  return defHttp.post<any>({ url: reviewUrl(id), data })
}

export function getTopDownloads(limit = 10) {
  return defHttp.get<{ list: PackageItem[] }>({ url: Api.TopDownloads, params: { limit } })
}

export function getTopLikes(limit = 10) {
  return defHttp.get<{ list: PackageItem[] }>({ url: Api.TopLikes, params: { limit } })
} 