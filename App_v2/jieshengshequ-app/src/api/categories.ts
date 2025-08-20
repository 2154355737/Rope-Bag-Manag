import { http } from './client'

export interface Category { id: number|string; name: string }

export async function getCategories() {
	const data = await http.get<{ items?: Category[]; list?: Category[] }>(`/categories`)
	return (data.items || data.list || [])
} 