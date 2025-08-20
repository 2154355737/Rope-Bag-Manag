import { http } from './client'

export async function getUnreadCount() {
	const data = await http.get<{ count: number }>(`/notifications/count`)
	return data.count ?? 0
} 