import { isTokenExpired } from '@/utils/jwt'

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE'

// 自定义 API 错误，包含 HTTP 状态码与业务码
export class ApiError extends Error {
	status?: number
	code?: number
	data?: any
	constructor(message: string, status?: number, code?: number, data?: any) {
		super(message)
		this.name = 'ApiError'
		this.status = status
		this.code = code
		this.data = data
	}
}

// 根据环境配置API基础URL
const API_BASE = import.meta.env.PROD 
  ? 'http://39.105.113.219:15201/api/v1'  // 生产环境连接到云服务器
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
		const token = localStorage.getItem('token')
		if (!token) return null
		
		// 检查token是否过期
		if (isTokenExpired(token)) {
			console.warn('Token已过期，自动清除')
			clearToken()
			return null
		}
		
		return token
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

		// 先尝试解析JSON（即使是非200）
		const data = await resp.json().catch(() => ({} as any))

		// 处理401未授权错误
		if (resp.status === 401) {
			console.warn('收到401响应，token可能已过期')
			redirectToLogin()
			throw new ApiError('登录已过期，请重新登录', 401, 401, data)
		}

		// 兼容后端统一响应 { code, message, data }
		if (data && typeof data === 'object' && 'code' in data) {
			const bizCode = Number((data as any).code)
			if (bizCode === 0 || bizCode === 200) return (data as any).data as T
			// 其他业务错误（包含400类：用户不存在/密码错误）
			throw new ApiError((data as any).message || '请求失败', resp.status, bizCode, data)
		}

		// 非统一响应：根据HTTP状态判断
		if (!resp.ok) {
			throw new ApiError((data && (data as any).message) || resp.statusText || '请求失败', resp.status, undefined, data)
		}
		return data as T
	} catch (error) {
		// 网络错误或其他错误
		if (error instanceof TypeError && (error as any).message?.includes('fetch')) {
			throw new ApiError('网络连接失败，请检查网络或后端服务', undefined)
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