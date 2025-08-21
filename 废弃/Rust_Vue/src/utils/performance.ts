// 性能监控工具
class PerformanceMonitor {
  private metrics: Map<string, number> = new Map()
  private observers: PerformanceObserver[] = []

  constructor() {
    this.initObservers()
  }

  // 初始化方法
  init() {
    console.log('性能监控已初始化')
    this.initObservers()
  }

  // 初始化性能观察器
  private initObservers() {
    // 观察页面加载性能
    if ('PerformanceObserver' in window) {
      const navigationObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.entryType === 'navigation') {
            this.recordNavigationTiming(entry as PerformanceNavigationTiming)
          }
        }
      })
      navigationObserver.observe({ entryTypes: ['navigation'] })

      // 观察资源加载性能
      const resourceObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.entryType === 'resource') {
            this.recordResourceTiming(entry as PerformanceResourceTiming)
          }
        }
      })
      resourceObserver.observe({ entryTypes: ['resource'] })

      this.observers.push(navigationObserver, resourceObserver)
    }
  }

  // 记录页面导航时间
  private recordNavigationTiming(entry: PerformanceNavigationTiming) {
    const metrics = {
      'DNS查询时间': entry.domainLookupEnd - entry.domainLookupStart,
      'TCP连接时间': entry.connectEnd - entry.connectStart,
      '请求响应时间': entry.responseEnd - entry.requestStart,
      'DOM解析时间': entry.domContentLoadedEventEnd - entry.domContentLoadedEventStart,
      '页面完全加载时间': entry.loadEventEnd - entry.loadEventStart,
      '总加载时间': entry.loadEventEnd - (entry as any).navigationStart
    }

    Object.entries(metrics).forEach(([key, value]) => {
      this.metrics.set(key, value)
    })

    console.log('页面加载性能指标:', metrics)
  }

  // 记录资源加载时间
  private recordResourceTiming(entry: PerformanceResourceTiming) {
    const duration = entry.duration
    const size = entry.transferSize || 0
    
    if (duration > 1000) { // 记录加载时间超过1秒的资源
      console.warn(`慢资源加载: ${entry.name}, 耗时: ${duration}ms, 大小: ${size}bytes`)
    }
  }

  // 记录自定义指标
  recordMetric(name: string, value: number) {
    this.metrics.set(name, value)
  }

  // 获取性能指标
  getMetrics() {
    return Object.fromEntries(this.metrics)
  }

  // 获取内存使用情况
  getMemoryInfo() {
    if ('memory' in performance) {
      const memory = (performance as any).memory
      return {
        已用堆内存: memory.usedJSHeapSize,
        总堆内存: memory.totalJSHeapSize,
        堆内存限制: memory.jsHeapSizeLimit
      }
    }
    return null
  }

  // 清理观察器
  destroy() {
    this.observers.forEach(observer => observer.disconnect())
    this.observers = []
  }
}

// 创建全局性能监控实例
export const performanceMonitor = new PerformanceMonitor()

// 页面可见性API监控
export const initVisibilityMonitor = () => {
  let hiddenTime = 0
  let visibleTime = Date.now()

  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      hiddenTime = Date.now()
    } else {
      visibleTime = Date.now()
      const hiddenDuration = visibleTime - hiddenTime
      if (hiddenDuration > 60000) { // 超过1分钟
        console.log(`页面隐藏时长: ${Math.round(hiddenDuration / 1000)}秒`)
      }
    }
  })
}

// 网络状态监控
export const initNetworkMonitor = () => {
  if ('connection' in navigator) {
    const connection = (navigator as any).connection
    console.log('网络信息:', {
      类型: connection.effectiveType,
      下行速度: connection.downlink,
      往返时间: connection.rtt,
      保存数据: connection.saveData
    })
  }
}