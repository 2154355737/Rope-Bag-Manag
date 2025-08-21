// 后端连接测试工具
import { http } from '@/api/client'

export const testBackendConnection = async () => {
  console.log('🧪 开始测试后端连接...')
  
  try {
    // 测试健康检查端点
    const healthResponse = await fetch('/health')
    const healthData = await healthResponse.json()
    console.log('✅ 健康检查通过:', healthData)
    
    // 测试分类API（公开接口）
    const categories = await http.get('/categories')
    console.log('✅ 分类API测试通过:', categories)
    
    // 测试通知API（需要登录）
    try {
      const notifications = await http.get('/notifications')
      console.log('✅ 通知API测试通过:', notifications)
    } catch (error: any) {
      if (error.message.includes('未授权')) {
        console.log('ℹ️ 通知API需要登录（正常）')
      } else {
        console.log('❌ 通知API测试失败:', error.message)
      }
    }
    
    console.log('🎉 后端连接测试完成')
    return true
  } catch (error: any) {
    console.error('❌ 后端连接测试失败:', error)
    return false
  }
}

// 在开发环境下自动运行测试
if (import.meta.env.DEV) {
  // 延迟一点时间让应用初始化完成
  setTimeout(() => {
    testBackendConnection()
  }, 1000)
} 