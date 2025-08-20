export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE'

const API_BASE = '/api/v1'

function buildQuery(params?: Record<string, any>): string {
	if (!params) return ''
	const q = Object.entries(params)
		.filter(([, v]) => v !== undefined && v !== null && v !== '')
		.map(([k, v]) => `${encodeURIComponent(k)}=${encodeURIComponent(String(v))}`)
		.join('&')
	return q ? `?${q}` : ''
}

function getToken(): string | null {
	try {
		return localStorage.getItem('token')
	} catch {
		return null
	}
}

async function request<T>(method: HttpMethod, url: string, body?: any, init?: RequestInit): Promise<T> {
	const headers: HeadersInit = {
		'Accept': 'application/json',
	}
	if (body && !(body instanceof FormData)) {
		headers['Content-Type'] = 'application/json'
	}
	const token = getToken()
	if (token) headers['Authorization'] = `Bearer ${token}`

	const resp = await fetch(`${API_BASE}${url}`, {
		method,
		headers,
		body: body ? (body instanceof FormData ? body : JSON.stringify(body)) : undefined,
		credentials: 'include',
		...init,
	})

	const data = await resp.json().catch(() => ({}))
	// 兼容后端统一响应 { code, message, data }
	if ('code' in data) {
		if (data.code === 0 || data.code === 200) return data.data as T
		throw new Error(data.message || '请求失败')
	}
	if (!resp.ok) throw new Error((data && data.message) || resp.statusText)
	return data as T
}

export const http = {
	get<T>(url: string, params?: Record<string, any>) {
		return request<T>('GET', `${url}${buildQuery(params)}`)
	},
	post<T>(url: string, body?: any) {
		return request<T>('POST', url, body)
	},
	put<T>(url: string, body?: any) {
		return request<T>('PUT', url, body)
	},
	delete<T>(url: string) {
		return request<T>('DELETE', url)
	},
} 