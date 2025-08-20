import { http } from './client'

export interface ContentCard {
	id: number
	type: 'post' | 'resource' | 'announcement'
	title: string
	description?: string
	author?: { id?: number; name: string; avatar?: string }
	image?: string
	tags?: string[]
	stats?: { likes?: number; comments?: number; views?: number; downloads?: number; rating?: number }
	flags?: { isTop?: boolean; isHot?: boolean; isPinned?: boolean }
	publishedAt: string
}

export interface FeedResponse { list: any[]; total: number; page: number; page_size: number }

export async function fetchFeed(params?: { type?: 'post'|'resource'|'announcement'; page?: number; pageSize?: number }) {
	const data = await http.get<FeedResponse>('/feed', params)
	// 将后端 list 映射为 ContentCard[]（后端已统一 resource/post）
	const items: ContentCard[] = (data.list || []).map((i: any) => ({
		id: i.id,
		type: (i.item_type || i.type) as any,
		title: i.title,
		author: i.author ? { name: i.author } : undefined,
		tags: i.tags,
		publishedAt: i.created_at || i.publishedAt,
		// 添加置顶和精华字段
		is_pinned: i.is_pinned,
		is_featured: i.is_featured,
		// 添加统计信息
		stats: {
			likes: i.like_count || 0,
			comments: i.comment_count || 0,
			views: i.view_count || 0,
			downloads: i.download_count || 0,
			rating: i.rating || 0
		}
	}))
	return { items, total: data.total, page: data.page, pageSize: data.page_size }
} 