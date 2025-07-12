// API缓存管理器
interface CacheItem<T> {
  data: T
  timestamp: number
  ttl: number // 缓存时间（毫秒）
}

interface CacheConfig {
  ttl: number // 默认缓存时间
  maxSize: number // 最大缓存条目数
}

class ApiCache {
  private cache = new Map<string, CacheItem<any>>()
  private config: CacheConfig

  constructor(config: CacheConfig = { ttl: 30000, maxSize: 100 }) {
    this.config = config
  }

  // 生成缓存键
  private generateKey(url: string, params?: any): string {
    const paramStr = params ? JSON.stringify(params) : ''
    return `${url}${paramStr}`
  }

  // 获取缓存数据
  get<T>(url: string, params?: any): T | null {
    const key = this.generateKey(url, params)
    const item = this.cache.get(key)
    
    if (!item) return null
    
    // 检查是否过期
    if (Date.now() - item.timestamp > item.ttl) {
      this.cache.delete(key)
      return null
    }
    
    return item.data
  }

  // 设置缓存数据
  set<T>(url: string, data: T, params?: any, ttl?: number): void {
    const key = this.generateKey(url, params)
    const item: CacheItem<T> = {
      data,
      timestamp: Date.now(),
      ttl: ttl || this.config.ttl
    }
    
    // 如果缓存已满，删除最旧的条目
    if (this.cache.size >= this.config.maxSize) {
      const oldestKey = this.cache.keys().next().value
      this.cache.delete(oldestKey)
    }
    
    this.cache.set(key, item)
  }

  // 清除缓存
  clear(): void {
    this.cache.clear()
  }

  // 删除特定缓存
  delete(url: string, params?: any): void {
    const key = this.generateKey(url, params)
    this.cache.delete(key)
  }

  // 获取缓存统计信息
  getStats() {
    return {
      size: this.cache.size,
      maxSize: this.config.maxSize,
      keys: Array.from(this.cache.keys())
    }
  }
}

// 创建全局缓存实例
export const apiCache = new ApiCache()

// 带缓存的API函数包装器
export function withCache<T>(
  apiFunction: () => Promise<T>,
  cacheKey: string,
  ttl: number = 30000
): () => Promise<T> {
  return async () => {
    // 尝试从缓存获取
    const cached = apiCache.get<T>(cacheKey)
    if (cached) {
      console.log(`从缓存获取数据: ${cacheKey}`)
      return cached
    }
    
    // 缓存未命中，调用API
    console.log(`缓存未命中，请求数据: ${cacheKey}`)
    try {
      const data = await apiFunction()
      apiCache.set(cacheKey, data, undefined, ttl)
      return data
    } catch (error) {
      console.error(`API请求失败: ${cacheKey}`, error)
      throw error
    }
  }
} 