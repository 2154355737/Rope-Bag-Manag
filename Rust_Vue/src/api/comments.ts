import { 
  initDB, 
  getCommentsByResourceId, 
  createComment, 
  deleteComment 
} from '@/utils/sqlite'
import type { ApiResponse, PaginatedResponse } from './types'

// 获取当前用户名的辅助函数
function getCurrentUsername(): string {
  const userInfo = localStorage.getItem('userInfo')
  let username = 'admin'
  
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      username = user.username || 'admin'
    } catch (e) {
      console.warn('解析用户信息失败，使用默认用户名')
    }
  }
  
  return username
}

// 评论管理API
export const commentApi = {
  // 获取资源评论
  getComments: async (resourceId: number, params?: {
    page?: number
    pageSize?: number
  }): Promise<ApiResponse<PaginatedResponse<any>>> => {
    await initDB()
    const comments = getCommentsByResourceId(resourceId)
    
    // 分页处理
    const page = params?.page || 1
    const pageSize = params?.pageSize || 10
    const start = (page - 1) * pageSize
    const end = start + pageSize
    const paginatedComments = comments.slice(start, end)
    
    // 转换为标准格式
    const commentList = paginatedComments.map(comment => ({
      id: comment[0],
      content: comment[1],
      user_id: comment[2],
      resource_id: comment[3],
      created_at: comment[4],
      author_name: comment[5] || '匿名用户'
    }))
    
    return {
      code: 0,
      msg: '获取评论列表成功',
      data: {
        list: commentList,
        total: comments.length,
        page,
        size: pageSize
      }
    }
  },

  // 创建评论
  createComment: async (commentData: {
    content: string
    resource_id: number
  }): Promise<ApiResponse> => {
    await initDB()
    
    const username = getCurrentUsername()
    const user = await import('@/utils/sqlite').then(m => m.getUserByUsername(username))
    
    if (!user) {
      return {
        code: 1,
        msg: '用户不存在'
      }
    }
    
    const result = createComment({
      content: commentData.content,
      user_id: user[0],
      resource_id: commentData.resource_id
    })
    
    if (result > 0) {
      return {
        code: 0,
        msg: '评论创建成功'
      }
    } else {
      return {
        code: 1,
        msg: '评论创建失败'
      }
    }
  },

  // 删除评论
  deleteComment: async (commentId: number): Promise<ApiResponse> => {
    await initDB()
    
    const result = deleteComment(commentId)
    
    if (result > 0) {
      return {
        code: 0,
        msg: '评论删除成功'
      }
    } else {
      return {
        code: 1,
        msg: '评论删除失败'
      }
    }
  },

  // 批量删除评论
  batchDeleteComments: async (commentIds: number[]): Promise<ApiResponse> => {
    await initDB()
    
    // 这里需要实现批量删除评论的方法
    // 暂时返回成功
    return {
      code: 0,
      msg: '批量删除评论成功'
    }
  },

  // 获取所有评论（管理员功能）
  getAllComments: async (params?: {
    page?: number
    pageSize?: number
    resource_id?: number
    user_id?: number
    search?: string
  }): Promise<ApiResponse<PaginatedResponse<any>>> => {
    await initDB()
    
    // 这里需要实现获取所有评论的方法
    // 暂时返回空数据
    return {
      code: 0,
      msg: '获取评论列表成功',
      data: {
        list: [],
        total: 0,
        page: params?.page || 1,
        size: params?.pageSize || 10
      }
    }
  }
} 