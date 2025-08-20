import { http } from './client'

export interface AnnouncementSummary { id: number; title: string; type?: string; priority?: string; author?: { name: string; avatar?: string; role?: string; verified?: boolean }; tags?: string[]; views?: number; likes?: number; comments?: number; isPinned?: boolean; publishDate?: string }
export interface AnnouncementDetail extends AnnouncementSummary { content: string; effectiveDate?: string; expiryDate?: string; attachments?: Array<{ id?: number; name: string; size?: string; url: string }>; relatedLinks?: Array<{ title: string; url: string; description?: string }> }

const mapType = (t?: string) => ({ Error: 'important', Warning: 'warning', Info: 'info', Success: 'update' } as any)[t || ''] || 'info'
const mapPriority = (p?: number) => (p && p >= 5 ? 'high' : 'low')

export async function getAnnouncements(params?: { page?: number; pageSize?: number; tag?: string }) {
	// 优先使用顶层别名 /announcements；失败时回退到 /public/announcements
	let data: { list: any[] }
	try {
		data = await http.get<{ list: any[] }>(`/announcements`, params)
	} catch {
		data = await http.get<{ list: any[] }>(`/public/announcements`, params)
	}
	return (data.list || []).map(a => ({
		id: a.id,
		title: a.title,
		type: mapType(a.type_),
		priority: mapPriority(a.priority),
		author: { name: '系统公告', role: '系统', verified: true },
		publishDate: a.start_time,
		isPinned: (a.priority || 0) >= 10,
	})) as AnnouncementSummary[]
}

export async function getAnnouncement(id: number) {
	// 优先 /announcements/{id}；失败时回退到 /public/announcements/{id}
	let raw: any
	try {
		raw = await http.get<any>(`/announcements/${id}`)
	} catch {
		raw = await http.get<any>(`/public/announcements/${id}`)
	}
	const detail: AnnouncementDetail = {
		id: raw.id,
		title: raw.title,
		content: raw.content || '',
		type: mapType(raw.type_),
		priority: mapPriority(raw.priority),
		author: { name: '结绳社区公告', avatar: '', role: '系统', verified: true },
		tags: [],
		views: 0,
		likes: 0,
		comments: 0,
		isPinned: (raw.priority || 0) >= 10,
		publishDate: raw.start_time,
		effectiveDate: raw.start_time,
		expiryDate: raw.end_time || undefined,
		attachments: [],
		relatedLinks: [],
	}
	return detail
} 