import { http } from './client'

export async function getResources(params?: { page?: number; pageSize?: number; category_id?: number; tag?: string; search?: string }) {
	const data = await http.get<{ list: any[]; total: number; page: number; page_size: number }>(`/resources`, params)
	return data
}

export async function getResource(id: number) {
	return http.get<any>(`/resources/${id}`)
}

export async function getResourceComments(id: number, page=1, pageSize=10) {
	return http.get<{ list: any[]; total: number; page: number; size: number }>(`/resources/${id}/comments`, { page, size: pageSize })
}

export async function createResourceComment(id: number, content: string) {
	return http.post<any>(`/resources/${id}/comments`, { content })
}

// 旧的 toggle（仅POST），保留兼容但不再使用
export async function toggleLikeResource(id: number) {
	return http.post<{ like_count?: number }>(`/resources/${id}/like`)
}

export async function toggleBookmarkResource(id: number) {
	try { return await http.post<{ favorite_count?: number }>(`/resources/${id}/bookmark`) } catch { return await http.delete<{ favorite_count?: number }>(`/resources/${id}/bookmark`) }
}

export async function reportResource(id: number) {
	return http.post(`/resources/${id}/report`, {})
}

export async function downloadResource(id: number) {
	const url = await http.get<string>(`/resources/${id}/download`)
	return url
}

// 状态查询
export async function getResourceLikeStatus(id: number) {
	return http.get<{ liked: boolean }>(`/resources/${id}/like-status`)
}
export async function getResourceBookmarkStatus(id: number) {
	return http.get<{ favorited: boolean }>(`/resources/${id}/bookmark-status`)
}

// 新增：显式点赞/取消点赞（POST/DELETE）
export async function likeResource(id: number) {
	return http.post<{ like_count: number }>(`/resources/${id}/like`)
}
export async function unlikeResource(id: number) {
	return http.delete<{ like_count: number }>(`/resources/${id}/like`)
}

// 删除资源
export async function deleteResource(id: number) {
	return http.delete(`/resources/${id}`)
}

// 更新资源状态
export async function updateResourceStatus(id: number, status: string) {
	return http.put(`/resources/${id}/status`, { status })
}

// 更新资源内容
export async function updateResource(id: number, data: {
	title?: string
	name?: string
	description?: string
	version?: string
	category_id?: number
	tags?: string[]
	requirements?: string[]
}) {
	return http.put(`/resources/${id}`, data)
} 