// Fetch 拦截器：统一拦截 `/api/` 请求并在成功时记录资源操作。
// 该文件仅通过副作用生效，请在 apiClient.ts 中直接 `import './http/fetchInterceptor'`。

import { resourceLogger } from '../loggerService'

// 保留原生 fetch 引用
const originalFetch = window.fetch

// 重写全局 fetch
window.fetch = async function (input: RequestInfo | URL, init?: RequestInit) {
  const method = init?.method || 'GET'
  const urlString = typeof input === 'string' ? input : input instanceof URL ? input.toString() : (input as Request).url

  // 仅处理 /api/ 请求
  if (typeof urlString === 'string' && urlString.includes('/api/')) {
    console.log(`[Fetch拦截] ${method} ${urlString}`)
  }

  // 调用原始 fetch
  const response = await originalFetch(input, init)
  const clone = response.clone()

  // 仅处理 /api/ 请求
  if (typeof urlString === 'string' && urlString.includes('/api/')) {
    // 跳过包创建操作（已由后端记录）
    if (method === 'POST' && urlString.includes('/api/v1/packages') && !urlString.includes('/api/v1/packages/')) {
      console.log('[跳过前端记录] 包创建操作已在后端记录')
      return response
    }

    clone
      .json()
      .then((data) => {
        if (data && data.code === 0) {
          setTimeout(() => {
            try {
              let resourceId: number | null = null
              let action = 'Unknown'
              let resourceType = 'Unknown'

              // 提取资源类型与 ID
              const urlStr = urlString
              if (urlStr.includes('/api/v1/packages/')) {
                resourceType = 'Package'
                const m = urlStr.match(/\/api\/v1\/packages\/(\d+)/)
                if (m) resourceId = parseInt(m[1])
              } else if (urlStr.includes('/api/v1/users/')) {
                resourceType = 'User'
                const m = urlStr.match(/\/api\/v1\/users\/(\d+)/)
                if (m) resourceId = parseInt(m[1])
              } else if (urlStr.includes('/api/v1/comments/')) {
                resourceType = 'Comment'
                const m = urlStr.match(/\/api\/v1\/comments\/(\d+)/)
                if (m) resourceId = parseInt(m[1])
              } else if (urlStr.includes('/api/v1/categories/')) {
                resourceType = 'Category'
                const m = urlStr.match(/\/api\/v1\/categories\/(\d+)/)
                if (m) resourceId = parseInt(m[1])
              } else {
                // 默认提取数字 ID
                const idMatch = urlStr.match(/\/(\d+)(\/|$)/)
                if (idMatch) {
                  resourceId = parseInt(idMatch[1])
                  if (urlStr.includes('/api/v1/packages')) resourceType = 'Package'
                  else if (urlStr.includes('/api/v1/users')) resourceType = 'User'
                  else if (urlStr.includes('/api/v1/comments')) resourceType = 'Comment'
                  else if (urlStr.includes('/api/v1/categories')) resourceType = 'Category'
                }
              }

              // 操作类型
              if (method === 'POST') action = 'Create'
              else if (method === 'PUT') action = 'Update'
              else if (method === 'DELETE') action = 'Delete'
              else if (method === 'GET' && urlStr.includes('/download')) action = 'Download'

              // 记录
              if (resourceId && action !== 'Unknown' && resourceType !== 'Unknown') {
                const recordData: any = { resource_type: resourceType }
                if (data.data && action !== 'Delete') {
                  recordData.new_data = JSON.stringify(data.data)
                }
                console.log(`[Fetch记录] ${resourceType} ${action}：ID=${resourceId}`)
                resourceLogger
                  .logResourceOperation(resourceType, action, resourceId as number, null, data.data)
                  .catch((err) => console.warn('记录操作失败:', err))
              }
            } catch (err) {
              console.warn('处理响应数据失败:', err)
            }
          }, 0)
        }
      })
      .catch((err) => console.warn('解析响应JSON失败:', err))
  }

  return response
} 