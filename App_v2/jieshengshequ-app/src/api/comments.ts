import { http } from './client'

export async function getComments(targetType: 'post'|'resource'|'package', targetId: number, page=1, pageSize=10, includeReplies: boolean = true) {
	// 后端要求非管理员读取时 target_type 仅允许 post/package（resource 对应 package）
	const t = (targetType === 'resource' || targetType === 'package') ? 'package' : 'post'
	return http.get<{ list: any[]; total: number; page: number; size: number }>(`/comments`, { target_type: t, target_id: targetId, page, size: pageSize, include_replies: includeReplies })
}

// 平面列表（不包含replies）
export async function getCommentsFlat(targetType: 'post'|'resource'|'package', targetId: number, page=1, pageSize=10) {
  return getComments(targetType, targetId, page, pageSize, false)
}

export async function getCommentReplies(commentId: number) {
	return http.get<any[]>(`/comments/${commentId}/replies`)
}

export async function createComment(targetType: 'Post'|'Package', targetId: number, content: string, parentId?: number) {
	return http.post(`/comments`, { target_type: targetType, target_id: targetId, content, parent_id: parentId })
}

export async function updateComment(commentId: number, content?: string, status?: string) {
	return http.put(`/comments/${commentId}`, { content, status })
}

export async function deleteComment(commentId: number) {
	return http.delete(`/comments/${commentId}`)
}

export async function likeComment(commentId: number, like: boolean) {
	return http.post<number>(`/comments/${commentId}/like`, { like })
}

export async function helpfulComment(commentId: number) {
	return http.post<{ active: boolean }>(`/comments/${commentId}/helpful`)
}

export async function replyComment(commentId: number, content: string) {
	return http.post(`/comments/${commentId}/reply`, { content })
}

export async function reportComment(commentId: number) {
	return http.post(`/comments/${commentId}/report`, {})
} 