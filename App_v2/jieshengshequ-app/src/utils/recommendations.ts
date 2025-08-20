import { RecommendedItem } from '@/components/related-recommendations'
import { fetchFeed } from '../api/feed'

// 基于内容的推荐算法 - 使用真实API数据
export const getContentBasedRecommendations = async (options: RecommendationOptions): Promise<RecommendedItem[]> => {
  const {
    currentItemId,
    currentItemType,
    currentItemTags = [],
    limit = 4,
    includeTypes = ['post', 'resource', 'announcement']
  } = options

  try {
    // 获取feed数据
    const feedData = await fetchFeed({ page: 1, pageSize: 20 })
    
    let recommendations = (feedData.items || [])
      .filter((item: any) => item.id !== currentItemId) // 过滤当前项目
      .map((item: any) => ({
        id: item.id,
        type: item.type || item.item_type,
        title: item.title,
        description: item.description || '',
        author: {
          name: typeof item.author === 'string' ? item.author : (item.author?.name || item.author_name || '用户'),
          avatar: typeof item.author === 'object' ? (item.author?.avatar || '') : '',
          verified: typeof item.author === 'object' ? (item.author?.verified || false) : false
        },
        tags: item.tags || [],
        stats: {
          likes: item.stats?.likes || 0,
          comments: item.stats?.comments || 0,
          views: item.stats?.views || item.view_count || 0,
          downloads: item.stats?.downloads || item.download_count || 0,
          rating: item.stats?.rating || 0
        },
        time: formatTimeAgo(item.created_at || item.publishedAt || ''),
        isHot: item.flags?.isHot || false,
        isPinned: item.flags?.isTop || false
      }))
      .filter((item: RecommendedItem) => includeTypes.includes(item.type)) // 过滤类型

    // 基于标签相似度排序
    if (currentItemTags.length > 0) {
      recommendations = recommendations.map(item => {
        const itemTags = item.tags || []
        const commonTags = itemTags.filter(tag => 
          currentItemTags.some(currentTag => 
            currentTag.toLowerCase().includes(tag.toLowerCase()) ||
            tag.toLowerCase().includes(currentTag.toLowerCase())
          )
        )
        
        // 计算相似度分数
        const similarityScore = commonTags.length / Math.max(itemTags.length, currentItemTags.length, 1)
        
        return {
          ...item,
          _similarityScore: similarityScore
        }
      }).sort((a: any, b: any) => (b._similarityScore || 0) - (a._similarityScore || 0))
    }

    // 如果基于标签的推荐结果不足，按热门和统计数据排序
    if (recommendations.length < limit) {
      recommendations.sort((a, b) => {
        // 优先显示置顶和热门内容
        if (a.isPinned && !b.isPinned) return -1
        if (!a.isPinned && b.isPinned) return 1
        if (a.isHot && !b.isHot) return -1
        if (!a.isHot && b.isHot) return 1
        
        // 按统计数据排序
        const aScore = (a.stats.likes || 0) + (a.stats.views || 0) * 0.1 + (a.stats.downloads || 0) * 2
        const bScore = (b.stats.likes || 0) + (b.stats.views || 0) * 0.1 + (b.stats.downloads || 0) * 2
        return bScore - aScore
      })
    }

    return recommendations.slice(0, limit)
  } catch (error) {
    console.error('获取推荐内容失败:', error)
    return []
  }
}

// 推荐算法接口
interface RecommendationOptions {
  currentItemId?: number
  currentItemType?: 'post' | 'resource' | 'announcement'
  currentItemTags?: string[]
  userId?: string
  limit?: number
  includeTypes?: ('post' | 'resource' | 'announcement')[]
}

// 格式化时间显示
const formatTimeAgo = (dateString: string): string => {
  if (!dateString) return '未知时间'
  
  try {
    const date = new Date(dateString)
    const now = new Date()
    const diffTime = Math.abs(now.getTime() - date.getTime())
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
    
    if (diffDays === 1) return '今天'
    if (diffDays === 2) return '昨天'
    if (diffDays <= 7) return `${diffDays}天前`
    if (diffDays <= 30) return `${Math.ceil(diffDays / 7)}周前`
    if (diffDays <= 365) return `${Math.ceil(diffDays / 30)}个月前`
    
    return `${Math.ceil(diffDays / 365)}年前`
  } catch {
    return '未知时间'
  }
}

// 根据内容类型获取专门的推荐
export const getPostRecommendations = async (currentPostId: number, currentTags: string[] = []): Promise<RecommendedItem[]> => {
  return getContentBasedRecommendations({
    currentItemId: currentPostId,
    currentItemType: 'post',
    currentItemTags: currentTags,
    limit: 6,
    includeTypes: ['post', 'resource'] // 帖子页面推荐帖子和相关资源
  })
}

export const getResourceRecommendations = async (currentResourceId: number, currentTags: string[] = []): Promise<RecommendedItem[]> => {
  return getContentBasedRecommendations({
    currentItemId: currentResourceId,
    currentItemType: 'resource',
    currentItemTags: currentTags,
    limit: 6,
    includeTypes: ['resource', 'post'] // 资源页面推荐资源和相关帖子
  })
}

export const getAnnouncementRecommendations = async (currentAnnouncementId: number, currentTags: string[] = []): Promise<RecommendedItem[]> => {
  return getContentBasedRecommendations({
    currentItemId: currentAnnouncementId,
    currentItemType: 'announcement', 
    currentItemTags: currentTags,
    limit: 6,
    includeTypes: ['announcement', 'post'] // 公告页面推荐公告和相关帖子
  })
} 