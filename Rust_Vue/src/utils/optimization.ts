import { performanceMonitor, initVisibilityMonitor, initNetworkMonitor } from './performance'
import { apiCache } from '../api/cache'

// 性能优化管理器
class OptimizationManager {
  private isInitialized = false

  // 初始化所有优化
  init() {
    if (this.isInitialized) return
    
    console.log('初始化性能优化...')
    
    // 初始化性能监控
    initVisibilityMonitor()
    initNetworkMonitor()
    
    // 预加载关键资源
    this.preloadCriticalResources()
    
    // 优化图片加载
    this.optimizeImageLoading()
    
    // 优化字体加载
    this.optimizeFontLoading()
    
    this.isInitialized = true
    console.log('性能优化初始化完成')
  }

  // 预加载关键资源
  private preloadCriticalResources() {
    // 预加载关键CSS
    const criticalCSS = document.createElement('link')
    criticalCSS.rel = 'preload'
    criticalCSS.as = 'style'
    criticalCSS.href = '/src/assets/base.css'
    document.head.appendChild(criticalCSS)

    // 预加载关键字体
    const fontPreload = document.createElement('link')
    fontPreload.rel = 'preload'
    fontPreload.as = 'font'
    fontPreload.href = 'https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap'
    fontPreload.crossOrigin = 'anonymous'
    document.head.appendChild(fontPreload)
  }

  // 优化图片加载
  private optimizeImageLoading() {
    // 使用Intersection Observer实现图片懒加载
    if ('IntersectionObserver' in window) {
      const imageObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            const img = entry.target as HTMLImageElement
            if (img.dataset.src) {
              img.src = img.dataset.src
              img.removeAttribute('data-src')
              imageObserver.unobserve(img)
            }
          }
        })
      })

      // 观察所有懒加载图片
      document.querySelectorAll('img[data-src]').forEach(img => {
        imageObserver.observe(img)
      })
    }
  }

  // 优化字体加载
  private optimizeFontLoading() {
    // 使用Font Loading API
    if ('fonts' in document) {
      document.fonts.ready.then(() => {
        console.log('字体加载完成')
      })
    }
  }

  // 清理缓存
  clearCache() {
    apiCache.clear()
    console.log('API缓存已清理')
  }

  // 获取性能报告
  getPerformanceReport() {
    const metrics = performanceMonitor.getMetrics()
    const memory = performanceMonitor.getMemoryInfo()
    
    return {
      metrics,
      memory,
      cacheStats: apiCache.getStats()
    }
  }
}

// 创建全局优化管理器实例
export const optimizationManager = new OptimizationManager()

// 自动初始化
if (typeof window !== 'undefined') {
  window.addEventListener('load', () => {
    optimizationManager.init()
  })
} 