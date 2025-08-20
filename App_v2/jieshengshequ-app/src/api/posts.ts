import { http } from './client'

export async function getPosts(params?: { page?: number; pageSize?: number; tag?: string }) {
	const data = await http.get<{ list: any[]; total: number; page: number; page_size: number }>(`/posts`, params)
	return data
}

export async function getPost(id: number) {
	return http.get<any>(`/posts/${id}`)
}

export async function toggleLikePost(id: number) {
	// 后端是 POST /posts/{id}/like 并返回最新计数
	return http.post<{ like_count?: number }>(`/posts/${id}/like`)
}

export async function toggleBookmarkPost(id: number) {
	// 后端采用 POST/DELETE /posts/{id}/bookmark；做成幂等：先尝试 POST，失败再 DELETE
	try { return await http.post<{ favorite_count?: number }>(`/posts/${id}/bookmark`) } catch { return await http.delete<{ favorite_count?: number }>(`/posts/${id}/bookmark`) }
}

export async function reportPost(id: number) {
	return http.post(`/posts/${id}/report`, {})
} 