// API 测试辅助工具
import { healthCheck, api } from './apiClient'
import { authApi } from '../api/auth'
import { packageApi } from '../api/packages'
import { userApi } from '../api/users'

export interface TestResult {
  name: string
  success: boolean
  message: string
  responseTime?: number
}

export class ApiTestHelper {
  static async testHealthCheck(): Promise<TestResult> {
    const startTime = Date.now()
    try {
      const response = await healthCheck()
      const responseTime = Date.now() - startTime
      
      if (response.code === 0) {
        return {
          name: '健康检查',
          success: true,
          message: '服务运行正常',
          responseTime
        }
      } else {
        return {
          name: '健康检查',
          success: false,
          message: response.msg || response.message || '服务异常',
          responseTime
        }
      }
    } catch (error) {
      const responseTime = Date.now() - startTime
      return {
        name: '健康检查',
        success: false,
        message: error instanceof Error ? error.message : '连接失败',
        responseTime
      }
    }
  }

  static async testPackagesList(): Promise<TestResult> {
    const startTime = Date.now()
    try {
      const response = await packageApi.getPackages({ page: 1, pageSize: 5 })
      const responseTime = Date.now() - startTime
      
      if (response.code === 0) {
        return {
          name: '包列表API',
          success: true,
          message: `获取到 ${response.data?.list?.length || 0} 个包`,
          responseTime
        }
      } else {
        return {
          name: '包列表API',
          success: false,
          message: response.msg || response.message || '获取失败',
          responseTime
        }
      }
    } catch (error) {
      const responseTime = Date.now() - startTime
      return {
        name: '包列表API',
        success: false,
        message: error instanceof Error ? error.message : '请求失败',
        responseTime
      }
    }
  }

  static async testUsersListWithAuth(): Promise<TestResult> {
    const startTime = Date.now()
    try {
      const response = await userApi.getUsers({ page: 1, pageSize: 5 })
      const responseTime = Date.now() - startTime
      
      if (response.code === 0) {
        return {
          name: '用户列表API (需认证)',
          success: true,
          message: `获取到 ${response.data?.list?.length || 0} 个用户`,
          responseTime
        }
      } else if (response.code === 401) {
        return {
          name: '用户列表API (需认证)',
          success: true, // 401是预期的，说明认证机制正常
          message: '需要认证 (正常)',
          responseTime
        }
      } else {
        return {
          name: '用户列表API (需认证)',
          success: false,
          message: response.msg || response.message || '请求失败',
          responseTime
        }
      }
    } catch (error: any) {
      const responseTime = Date.now() - startTime
      if (error.response?.status === 401) {
        return {
          name: '用户列表API (需认证)',
          success: true, // 401是预期的
          message: '需要认证 (正常)',
          responseTime
        }
      }
      return {
        name: '用户列表API (需认证)',
        success: false,
        message: error instanceof Error ? error.message : '请求失败',
        responseTime
      }
    }
  }

  static async runAllTests(): Promise<TestResult[]> {
    console.log('🧪 开始API对接测试...')
    
    const tests = [
      this.testHealthCheck,
      this.testPackagesList,
      this.testUsersListWithAuth
    ]

    const results: TestResult[] = []
    
    for (const test of tests) {
      try {
        const result = await test()
        results.push(result)
        console.log(`${result.success ? '✅' : '❌'} ${result.name}: ${result.message} (${result.responseTime}ms)`)
      } catch (error) {
        console.error(`❌ 测试执行失败:`, error)
        results.push({
          name: '测试执行',
          success: false,
          message: error instanceof Error ? error.message : '测试异常'
        })
      }
    }

    const successCount = results.filter(r => r.success).length
    console.log(`\n🎯 测试完成: ${successCount}/${results.length} 通过`)
    
    return results
  }

  // 快速诊断工具
  static async quickDiagnosis(): Promise<void> {
    console.log('🔍 快速诊断前后端连接状态...')
    
    try {
      // 1. 测试基础连接
      const health = await this.testHealthCheck()
      console.log(`连接状态: ${health.success ? '✅ 正常' : '❌ 异常'} - ${health.message}`)
      
      if (!health.success) {
        console.log('❗ 建议检查:')
        console.log('  1. 后端服务是否启动 (http://127.0.0.1:15201)')
        console.log('  2. 网络连接是否正常')
        console.log('  3. 防火墙设置')
        return
      }

      // 2. 测试API兼容性
      const packages = await this.testPackagesList()
      console.log(`API兼容性: ${packages.success ? '✅ 正常' : '❌ 异常'} - ${packages.message}`)
      
      if (!packages.success) {
        console.log('❗ 建议检查:')
        console.log('  1. 前后端API版本是否匹配')
        console.log('  2. 响应格式是否一致')
        console.log('  3. 路由配置是否正确')
      }

      // 3. 测试认证机制
      const users = await this.testUsersListWithAuth()
      console.log(`认证机制: ${users.success ? '✅ 正常' : '❌ 异常'} - ${users.message}`)

    } catch (error) {
      console.error('❌ 诊断过程中出现错误:', error)
    }
  }
} 