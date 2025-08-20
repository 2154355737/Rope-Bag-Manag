import { RecommendedItem } from '@/components/related-recommendations'

// 模拟推荐数据源
const mockRecommendationData: RecommendedItem[] = [
  // 帖子类型
  {
    id: 201,
    type: 'post',
    title: 'React性能优化最佳实践指南',
    description: '分享在大型项目中React性能优化的经验和技巧，包括代码分割、懒加载等',
    author: {
      name: '前端专家',
      avatar: 'https://i.pravatar.cc/150?img=10',
      verified: true
    },
    image: 'https://images.unsplash.com/photo-1633356122544-f134324a6cee?w=500&auto=format&fit=crop&q=60',
    tags: ['React', '性能优化', '前端'],
    stats: {
      likes: 245,
      comments: 38,
      views: 1250
    },
    time: '2天前',
    isHot: true
  },
  {
    id: 202,
    type: 'post', 
    title: 'TypeScript高级类型系统深度解析',
    description: '深入探讨TypeScript的高级类型特性，让你的代码更加类型安全',
    author: {
      name: 'TS开发者',
      avatar: 'https://i.pravatar.cc/150?img=11'
    },
    tags: ['TypeScript', '类型系统', '进阶'],
    stats: {
      likes: 189,
      comments: 24,
      views: 890
    },
    time: '3天前'
  },
  {
    id: 203,
    type: 'post',
    title: '微前端架构实战经验分享',
    description: '从零到一构建微前端架构，解决大型项目的工程化问题',
    author: {
      name: '架构师小王',
      avatar: 'https://i.pravatar.cc/150?img=12'
    },
    tags: ['微前端', '架构', '工程化'],
    stats: {
      likes: 156,
      comments: 31,
      views: 756
    },
    time: '5天前'
  },

  // 资源类型
  {
    id: 301,
    type: 'resource',
    title: 'React组件库开发工具包',
    description: '完整的React组件库开发解决方案，包含构建工具、文档生成、测试框架',
    author: {
      name: '开源贡献者',
      avatar: 'https://i.pravatar.cc/150?img=13',
      verified: true
    },
    image: 'https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=500&auto=format&fit=crop&q=60',
    tags: ['React', '组件库', '工具包'],
    stats: {
      downloads: 2450,
      rating: 4.8,
      views: 3200
    },
    time: '1周前',
    isHot: true
  },
  {
    id: 302,
    type: 'resource',
    title: 'Vue.js项目脚手架模板',
    description: '企业级Vue.js项目模板，集成路由、状态管理、UI框架等最佳实践',
    author: {
      name: 'Vue开发团队',
      avatar: 'https://i.pravatar.cc/150?img=14'
    },
    tags: ['Vue.js', '脚手架', '模板'],
    stats: {
      downloads: 1890,
      rating: 4.6,
      views: 2100
    },
    time: '2周前'
  },
  {
    id: 303,
    type: 'resource',
    title: 'Node.js微服务框架',
    description: '轻量级Node.js微服务开发框架，支持服务发现、负载均衡等功能',
    author: {
      name: '后端架构师',
      avatar: 'https://i.pravatar.cc/150?img=15'
    },
    tags: ['Node.js', '微服务', '框架'],
    stats: {
      downloads: 1256,
      rating: 4.5,
      views: 1800
    },
    time: '3周前'
  },

  // 公告类型
  {
    id: 401,
    type: 'announcement',
    title: '社区年度开发者大会即将举办',
    description: '结绳社区2025年度开发者大会将于下月举行，邀请行业专家分享最新技术趋势',
    image: 'https://images.unsplash.com/photo-1540575467063-178a50c2df87?w=500&auto=format&fit=crop&q=60',
    tags: ['大会', '技术分享', '社区活动'],
    stats: {
      views: 5600,
      comments: 89
    },
    time: '3天前',
    isPinned: true
  },
  {
    id: 402,
    type: 'announcement',
    title: '新版本发布：增强移动端体验',
    description: '社区移动端应用全新升级，新增深色模式、离线阅读等功能',
    tags: ['版本更新', '移动端', '新功能'],
    stats: {
      views: 4200,
      comments: 56
    },
    time: '1周前'
  },
  {
    id: 403,
    type: 'announcement',
    title: '开源项目贡献者招募',
    description: '社区开源项目正在招募贡献者，欢迎有兴趣的开发者加入我们',
    tags: ['开源', '招募', '贡献'],
    stats: {
      views: 3100,
      comments: 42
    },
    time: '2周前'
  },

  // 新增更多推荐内容
  {
    id: 204,
    type: 'post',
    title: 'JavaScript ES2024新特性完全指南',
    description: '全面介绍ES2024的新增特性，包括装饰器、管道操作符等',
    author: {
      name: 'JS专家',
      avatar: 'https://i.pravatar.cc/150?img=16'
    },
    image: 'https://images.unsplash.com/photo-1627398242454-45a1465c2479?w=500&auto=format&fit=crop&q=60',
    tags: ['JavaScript', 'ES2024', '新特性'],
    stats: {
      likes: 321,
      comments: 67,
      views: 2100
    },
    time: '1周前',
    isHot: true
  },
  {
    id: 304,
    type: 'resource',
    title: 'CSS动画效果库',
    description: '精美的CSS动画效果集合，包含各种流行的动画模式',
    author: {
      name: 'UI设计师',
      avatar: 'https://i.pravatar.cc/150?img=17'
    },
    tags: ['CSS', '动画', 'UI'],
    stats: {
      downloads: 3200,
      rating: 4.9,
      views: 4500
    },
    time: '4天前',
    isHot: true
  }
]

// 推荐算法接口
interface RecommendationOptions {
  currentItemId?: number
  currentItemType?: 'post' | 'resource' | 'announcement'
  currentItemTags?: string[]
  userId?: string
  limit?: number
  includeTypes?: ('post' | 'resource' | 'announcement')[]
}

// 基于内容的推荐算法
export const getContentBasedRecommendations = (options: RecommendationOptions): RecommendedItem[] => {
  const {
    currentItemId,
    currentItemType,
    currentItemTags = [],
    limit = 6,
    includeTypes = ['post', 'resource', 'announcement']
  } = options

  let recommendations = mockRecommendationData
    .filter(item => item.id !== currentItemId) // 过滤当前项目
    .filter(item => includeTypes.includes(item.type)) // 过滤类型

  // 基于标签相似度的评分推荐
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

  // 如果基于标签的推荐结果不足，补充热门内容
  if (recommendations.length < limit) {
    const hotItems = mockRecommendationData
      .filter(item => item.id !== currentItemId)
      .filter(item => includeTypes.includes(item.type))
      .filter(item => item.isHot || item.isPinned)
      .filter(item => !recommendations.find(rec => rec.id === item.id))

    recommendations = [...recommendations, ...hotItems]
  }

  // 如果还是不足，按统计数据排序补充
  if (recommendations.length < limit) {
    const popularItems = mockRecommendationData
      .filter(item => item.id !== currentItemId)
      .filter(item => includeTypes.includes(item.type))
      .filter(item => !recommendations.find(rec => rec.id === item.id))
      .sort((a, b) => {
        const aScore = (a.stats.likes || 0) + (a.stats.views || 0) * 0.1 + (a.stats.downloads || 0) * 2
        const bScore = (b.stats.likes || 0) + (b.stats.views || 0) * 0.1 + (b.stats.downloads || 0) * 2
        return bScore - aScore
      })

    recommendations = [...recommendations, ...popularItems]
  }

  return recommendations.slice(0, limit)
}

// 根据内容类型获取专门的推荐
export const getPostRecommendations = (currentPostId: number, currentTags: string[] = []): RecommendedItem[] => {
  return getContentBasedRecommendations({
    currentItemId: currentPostId,
    currentItemType: 'post',
    currentItemTags: currentTags,
    limit: 6,
    includeTypes: ['post', 'resource'] // 帖子页面推荐帖子和相关资源
  })
}

export const getResourceRecommendations = (currentResourceId: number, currentTags: string[] = []): RecommendedItem[] => {
  return getContentBasedRecommendations({
    currentItemId: currentResourceId,
    currentItemType: 'resource',
    currentItemTags: currentTags,
    limit: 6,
    includeTypes: ['resource', 'post'] // 资源页面推荐资源和相关帖子
  })
}

export const getAnnouncementRecommendations = (currentAnnouncementId: number, currentTags: string[] = []): RecommendedItem[] => {
  return getContentBasedRecommendations({
    currentItemId: currentAnnouncementId,
    currentItemType: 'announcement', 
    currentItemTags: currentTags,
    limit: 6,
    includeTypes: ['announcement', 'post'] // 公告页面推荐公告和相关帖子
  })
} 