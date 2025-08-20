import { http } from './client'

export async function searchAll(params: { query: string; type?: 'post'|'resource'|'announcement'; page?: number; pageSize?: number }) {
	return http.get<{ items: any[]; total: number; hasMore: boolean }>(`/search`, params as any)
}

export async function trendingKeywords() {
	const data = await http.get<{ keywords: string[] }>(`/search/trending`)
	return data.keywords || []
}

export async function suggestKeywords(query: string) {
	const data = await http.get<{ suggestions: string[] }>(`/search/suggest`, { query })
	return data.suggestions || []
} 