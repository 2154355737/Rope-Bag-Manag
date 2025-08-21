import { http } from '@/api/client'

// 测试后端连接
export const testBackendConnection = async (): Promise<boolean> => {
  try {
    // 尝试访问一个公开的端点，比如获取分类列表
    await http.get('/categories')
    console.log('✅ 后端连接成功')
    return true
  } catch (error) {
    console.error('❌ 后端连接失败:', error)
    return false
  }
}

// 测试认证端点
export const testAuthEndpoint = async (): Promise<boolean> => {
  try {
    // 尝试访问认证验证端点
    await http.get('/auth/verify')
    console.log('✅ 认证端点可访问')
    return true
  } catch (error) {
    console.error('❌ 认证端点测试失败:', error)
    return false
  }
}

// 测试消息端点（需要登录）
export const testMessagesEndpoint = async (): Promise<boolean> => {
  try {
    await http.get('/messages/conversations')
    console.log('✅ 消息端点可访问')
    return true
  } catch (error) {
    console.error('❌ 消息端点测试失败:', error)
    return false
  }
}

// 运行所有测试
export const runAPITests = async () => {
  console.log('🧪 开始API连接测试...')
  
  const backendTest = await testBackendConnection()
  const authTest = await testAuthEndpoint()
  
  console.log('📊 测试结果:')
  console.log(`  - 后端连接: ${backendTest ? '✅' : '❌'}`)
  console.log(`  - 认证端点: ${authTest ? '✅' : '❌'}`)
  
  return {
    backend: backendTest,
    auth: authTest
  }
} 