export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE'

// 根据环境配置API基础URL
const API_BASE = import.meta.env.PROD 
  ? 'http://127.0.0.1:15201/api/v1'  // 生产环境直接连接后端
  : '/api/v1'  // 开发环境使用代理

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

function setToken(token: string): void {
	try {
		localStorage.setItem('token', token)
	} catch {
		console.error('Failed to save token')
	}
}

function clearToken(): void {
	try {
		localStorage.removeItem('token')
		localStorage.removeItem('user')
	} catch {
		console.error('Failed to clear token')
	}
}

function redirectToLogin(): void {
	// 清除认证信息
	clearToken()
	// 跳转到登录页面
	if (typeof window !== 'undefined') {
		window.location.href = '/login'
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

	try {
		const resp = await fetch(`${API_BASE}${url}`, {
			method,
			headers,
			body: body ? (body instanceof FormData ? body : JSON.stringify(body)) : undefined,
			credentials: 'include',
			...init,
		})

		// 处理401未授权错误
		if (resp.status === 401) {
			redirectToLogin()
			throw new Error('未授权，请重新登录')
		}

		const data = await resp.json().catch(() => ({}))
		// 兼容后端统一响应 { code, message, data }
		if ('code' in data) {
			if (data.code === 0 || data.code === 200) return data.data as T
			// 特殊处理401错误码
			if (data.code === 401) {
				redirectToLogin()
				throw new Error('未授权，请重新登录')
			}
			throw new Error(data.message || '请求失败')
		}
		if (!resp.ok) throw new Error((data && data.message) || resp.statusText)
		return data as T
	} catch (error) {
		// 网络错误或其他错误
		if (error instanceof TypeError && error.message.includes('fetch')) {
			throw new Error('网络连接失败，请检查网络或后端服务')
		}
		throw error
	}
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

// 导出认证相关工具函数
export { getToken, setToken, clearToken } 