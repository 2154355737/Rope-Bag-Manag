// 调试辅助工具
export const debugLog = (message: string, data?: any) => {
  if (import.meta.env.DEV) {
    console.log(`🐛 [DEBUG] ${message}`, data || '')
  }
}

export const debugError = (message: string, error?: any) => {
  if (import.meta.env.DEV) {
    console.error(`❌ [ERROR] ${message}`, error || '')
  }
}

export const debugWarn = (message: string, data?: any) => {
  if (import.meta.env.DEV) {
    console.warn(`⚠️ [WARN] ${message}`, data || '')
  }
}

export const debugInfo = (message: string, data?: any) => {
  if (import.meta.env.DEV) {
    console.info(`ℹ️ [INFO] ${message}`, data || '')
  }
}

// 检查环境信息
export const checkEnvironment = () => {
  if (import.meta.env.DEV) {
    console.group('🔧 环境信息')
    console.log('开发模式:', import.meta.env.DEV)
    console.log('生产模式:', import.meta.env.PROD)
    console.log('当前URL:', window.location.href)
    console.log('API基础路径:', import.meta.env.VITE_API_BASE || '使用默认配置')
    console.groupEnd()
  }
}

// 在页面加载时检查环境
if (typeof window !== 'undefined') {
  checkEnvironment()
} 