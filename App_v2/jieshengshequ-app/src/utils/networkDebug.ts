// 网络调试工具
export class NetworkDebugger {
  private static apiBase = 'http://39.105.113.219:15201/api/v1'
  
  // 测试基本连接
  static async testConnection(): Promise<void> {
    console.log('🔍 开始网络连接诊断...')
    
    // 1. 测试基础网络连接
    await this.testBasicConnectivity()
    
    // 2. 测试API服务器
    await this.testApiServer()
    
    // 3. 测试具体端点
    await this.testApiEndpoints()
    
    console.log('✅ 网络连接诊断完成')
  }
  
  // 测试基础网络连接
  private static async testBasicConnectivity(): Promise<void> {
    console.log('📡 测试基础网络连接...')
    
    try {
      // 测试互联网连接
      const response = await fetch('https://www.baidu.com', {
        method: 'HEAD',
        mode: 'no-cors',
        cache: 'no-cache'
      })
      console.log('✅ 互联网连接正常')
    } catch (error) {
      console.error('❌ 互联网连接失败:', error)
    }
  }
  
  // 测试API服务器
  private static async testApiServer(): Promise<void> {
    console.log('🔌 测试API服务器连接...')
    
    try {
      // 测试服务器根路径
      const serverUrl = 'http://39.105.113.219:15201'
      const response = await fetch(serverUrl, {
        method: 'HEAD',
        mode: 'cors',
        cache: 'no-cache',
        headers: {
          'Accept': 'application/json'
        }
      })
      
      console.log(`✅ 服务器响应: ${response.status} ${response.statusText}`)
      console.log('📋 响应头:', Object.fromEntries(response.headers.entries()))
      
    } catch (error) {
      console.error('❌ 服务器连接失败:', error)
      
      if (error instanceof TypeError && error.message.includes('fetch')) {
        console.error('🚨 可能的原因:')
        console.error('   1. 服务器未启动或不可访问')
        console.error('   2. 网络防火墙阻止了连接')
        console.error('   3. CORS配置问题')
        console.error('   4. Android网络安全策略限制')
      }
    }
  }
  
  // 测试API端点
  private static async testApiEndpoints(): Promise<void> {
    console.log('🎯 测试API端点...')
    
    const endpoints = [
      '/health',
      '/categories',
      '/posts'
    ]
    
    for (const endpoint of endpoints) {
      try {
        const url = `${this.apiBase}${endpoint}`
        console.log(`📍 测试端点: ${url}`)
        
        const response = await fetch(url, {
          method: 'GET',
          headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
          },
          cache: 'no-cache'
        })
        
        console.log(`✅ ${endpoint}: ${response.status} ${response.statusText}`)
        
        if (response.ok) {
          const data = await response.text()
          console.log(`📦 ${endpoint} 响应长度: ${data.length} 字符`)
        }
        
      } catch (error) {
        console.error(`❌ ${endpoint} 失败:`, error)
      }
    }
  }
  
  // 获取设备网络信息
  static getNetworkInfo(): void {
    console.log('📱 设备网络信息:')
    console.log('   User Agent:', navigator.userAgent)
    console.log('   Platform:', navigator.platform)
    console.log('   Online:', navigator.onLine)
    
    // 检查是否在Capacitor环境中
    if ((window as any).Capacitor) {
      console.log('📱 Capacitor环境:', (window as any).Capacitor.getPlatform())
      console.log('📱 原生平台:', (window as any).Capacitor.isNativePlatform())
    } else {
      console.log('🌐 Web环境')
    }
  }
  
  // 测试CORS预检请求
  static async testCORS(): Promise<void> {
    console.log('🔒 测试CORS配置...')
    
    try {
      const response = await fetch(`${this.apiBase}/health`, {
        method: 'OPTIONS',
        headers: {
          'Origin': window.location.origin,
          'Access-Control-Request-Method': 'GET',
          'Access-Control-Request-Headers': 'Content-Type, Authorization'
        }
      })
      
      console.log('✅ CORS预检成功:', response.status)
      console.log('📋 CORS头:', {
        'Access-Control-Allow-Origin': response.headers.get('Access-Control-Allow-Origin'),
        'Access-Control-Allow-Methods': response.headers.get('Access-Control-Allow-Methods'),
        'Access-Control-Allow-Headers': response.headers.get('Access-Control-Allow-Headers')
      })
      
    } catch (error) {
      console.error('❌ CORS预检失败:', error)
    }
  }
}

// 自动运行诊断
if (typeof window !== 'undefined') {
  // 延迟运行，确保应用已加载
  setTimeout(() => {
    NetworkDebugger.getNetworkInfo()
    NetworkDebugger.testConnection()
    NetworkDebugger.testCORS()
  }, 2000)
} 