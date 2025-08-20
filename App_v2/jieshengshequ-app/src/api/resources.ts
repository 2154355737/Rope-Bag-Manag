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