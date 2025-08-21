import { reactive } from 'vue'
import type { ApiResponse } from '../api/types'

export interface ApiState<T> {
  data: T | null
  loading: boolean
  error: string | null
}

export function useApiState<T>() {
  const state = reactive<ApiState<T>>({
    data: null,
    loading: false,
    error: null
  })

  const execute = async (apiCall: () => Promise<ApiResponse<T>>) => {
    state.loading = true
    state.error = null
    
    try {
      const response = await apiCall()
      if (response.code === 0) {
        state.data = (response.data || null) as any
      } else {
        // 兼容处理：优先使用msg，fallback到message
        state.error = response.msg || response.message || '请求失败'
      }
    } catch (error) {
      state.error = error instanceof Error ? error.message : '未知错误'
    } finally {
      state.loading = false
    }
  }

  const reset = () => {
    state.data = null
    state.loading = false
    state.error = null
  }

  return { state, execute, reset }
}

// 缓存管理
const cache = new Map<string, { data: any; timestamp: number; ttl: number }>()

export function useCache() {
  const get = (key: string) => {
    const item = cache.get(key)
    if (!item) return null
    
    if (Date.now() - item.timestamp > item.ttl) {
      cache.delete(key)
      return null
    }
    
    return item.data
  }

  const set = (key: string, data: any, ttl: number = 5 * 60 * 1000) => {
    cache.set(key, {
      data,
      timestamp: Date.now(),
      ttl
    })
  }

  const clear = () => {
    cache.clear()
  }

  return { get, set, clear }
}

// 统一的错误处理
export function useErrorHandler() {
  const handleError = (error: any, context: string = '操作') => {
    console.error(`${context}失败:`, error)
    
    let message = '未知错误'
    if (error instanceof Error) {
      message = error.message
    } else if (typeof error === 'string') {
      message = error
    } else if (error?.message) {
      message = error.message
    } else if (error?.msg) {
      message = error.msg
    }
    
    return message
  }

  return { handleError }
}

// 分页状态管理
export interface PaginationState {
  currentPage: number
  pageSize: number
  total: number
  totalPages: number
  loading: boolean
}

export function usePagination(initialPageSize: number = 10) {
  const state = reactive<PaginationState>({
    currentPage: 1,
    pageSize: initialPageSize,
    total: 0,
    totalPages: 0,
    loading: false
  })

  const setPage = (page: number) => {
    state.currentPage = page
  }

  const setPageSize = (size: number) => {
    state.pageSize = size
    state.currentPage = 1 // 重置到第一页
  }

  const setTotal = (total: number) => {
    state.total = total
  }

  const reset = () => {
    state.currentPage = 1
    state.total = 0
    state.loading = false
  }

  return {
    state,
    setPage,
    setPageSize,
    setTotal,
    reset
  }
}

// 搜索和筛选状态管理
export interface SearchState {
  query: string
  filters: Record<string, any>
  sortBy: string
  sortOrder: 'asc' | 'desc'
}

export function useSearch() {
  const state = reactive<SearchState>({
    query: '',
    filters: {},
    sortBy: '',
    sortOrder: 'desc'
  })

  const setQuery = (query: string) => {
    state.query = query
  }

  const setFilter = (key: string, value: any) => {
    state.filters[key] = value
  }

  const clearFilters = () => {
    state.filters = {}
  }

  const setSort = (field: string, order: 'asc' | 'desc' = 'desc') => {
    state.sortBy = field
    state.sortOrder = order
  }

  const reset = () => {
    state.query = ''
    state.filters = {}
    state.sortBy = ''
    state.sortOrder = 'desc'
  }

  return {
    state,
    setQuery,
    setFilter,
    clearFilters,
    setSort,
    reset
  }
} 