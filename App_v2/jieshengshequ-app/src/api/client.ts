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
const isDevelopment = import.meta.env.DEV || import.meta.env.MODE === 'development'

// 允许通过环境变量覆盖（构建时注入）：VITE_API_BASE / VITE_STORAGE_API_BASE
const API_BASE = import.meta.env.VITE_API_BASE
  || (isDevelopment ? '/api/v1' : 'http://39.105.113.219:15201/api/v1')

// Storage API（默认同 API_BASE，可单独覆盖）
export const STORAGE_API_BASE = import.meta.env.VITE_STORAGE_API_BASE
  || (isDevelopment ? '/api/v1' : 'http://39.105.113.219:15201/api/v1')

// 调试信息：输出当前环境和API_BASE
console.log('Environment Mode:', import.meta.env.MODE)
console.log('Environment DEV:', import.meta.env.DEV)
console.log('Environment PROD:', import.meta.env.PROD)
console.log('isDevelopment:', isDevelopment)
console.log('API_BASE:', API_BASE)
console.log('STORAGE_API_BASE:', STORAGE_API_BASE)

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
		'Cache-Control': 'no-cache',
	}
	if (body && !(body instanceof FormData)) {
		headers['Content-Type'] = 'application/json'
	}
	const token = getToken()
	if (token) {
		headers['Authorization'] = `Bearer ${token}`
		console.log('使用认证Token:', token.substring(0, 10) + '...')
	} else {
		console.warn('未找到认证Token，请确保已登录')
	}

	// 为文件上传设置更长的超时时间
	const isFileUpload = body instanceof FormData
	const timeout = isFileUpload ? 300000 : 30000 // 文件上传5分钟，普通请求30秒

	// 调试信息
	if (isFileUpload) {
		console.log('准备上传文件:', url)
		if (body instanceof FormData) {
			console.log('FormData内容:')
			for (const pair of (body as FormData).entries()) {
				if (pair[0] === 'file') {
					const file = pair[1] as File
					console.log('- 文件:', file.name, '类型:', file.type, '大小:', file.size)
				} else {
					console.log('- 字段:', pair[0], '值:', pair[1])
				}
			}
		}
	}

	try {
		const controller = new AbortController()
		const timeoutId = setTimeout(() => controller.abort(), timeout)

		// 对于storage相关的API使用不同的基础路径
		const baseUrl = url.startsWith('/storage') ? STORAGE_API_BASE : API_BASE
		const fullUrl = `${baseUrl}${url}`
		console.log(`发送${method}请求:`, fullUrl)
		
		// 调试信息：打印完整请求信息
		if (body instanceof FormData) {
			console.log('FormData详细内容:')
			for (const pair of body.entries()) {
				if (pair[1] instanceof File) {
					const file = pair[1] as File
					console.log(`- 字段名: ${pair[0]}, 文件名: ${file.name}, 类型: ${file.type}, 大小: ${file.size}字节`)
				} else {
					console.log(`- 字段名: ${pair[0]}, 值: ${pair[1]}`)
				}
			}
		}
		
		const resp = await fetch(fullUrl, {
			method,
			headers,
			body: body ? (body instanceof FormData ? body : JSON.stringify(body)) : undefined,
			credentials: 'include',
			signal: controller.signal,
			...init,
		})

		clearTimeout(timeoutId)
		console.log(`请求响应:`, resp.status, resp.statusText)

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
		// 处理不同类型的网络错误
		if (error instanceof ApiError) {
			throw error
		}
		
		if (error instanceof DOMException && error.name === 'AbortError') {
			const errorMessage = isFileUpload 
				? '文件上传超时，请检查网络连接或尝试上传较小的文件' 
				: '请求超时，请重试'
			throw new ApiError(errorMessage, 408)
		}
		
		// 网络连接错误
		if (error instanceof TypeError) {
			const message = error.message
			if (message.includes('fetch') || message.includes('Failed to fetch')) {
				const errorMessage = isFileUpload 
					? '文件上传失败，请检查网络连接或后端服务状态'
					: '网络连接失败，请检查网络或后端服务'
				throw new ApiError(errorMessage, undefined)
			}
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