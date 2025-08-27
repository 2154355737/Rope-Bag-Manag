import React, { useState, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Bell, Code, BookOpen, Zap, Star, Clock, Bookmark, Pin, Eye, Download, Calendar, Loader2, Package } from 'lucide-react'
import { SkeletonCard } from '@/components/ui/skeleton'
import { Button } from '@/components/ui/button'

import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopNavigation from '@/components/ui/top-navigation'
import { getUnreadCount } from '../api/notifications'
import { trendingKeywords } from '../api/search'
import { fetchFeed } from '../api/feed'
import { getPosts } from '../api/posts'
import { getAnnouncements } from '../api/announcements'
import { getResources } from '../api/resources'
import { useNavigation } from '@/contexts/NavigationContext'
import { getCategories, Category } from '@/api/categories'
import { App as CapacitorApp } from '@capacitor/app'

// 内容数据接口
interface ContentItem {
  id: number
  type: 'post' | 'resource' | 'announcement'
  title: string
  description?: string
  tags?: string[]
  author: {
    id?: number
    name: string
    avatar?: string
  }
  likes: number
  comments: number
  views: number
  downloads?: number
  date: string
  isTop: boolean
  isHot: boolean
  category?: any
}

// 标签页数据状态接口
interface TabDataState {
  data: ContentItem[]
  loading: boolean
  error: string | null
  page: number
  hasMore: boolean
  lastUpdate: number
}

const HomeScreen: React.FC = () => {
  const navigate = useNavigate()
  const { getActiveTab, setActiveTab } = useNavigation()

  const [hotKeywords, setHotKeywords] = useState<string[]>([])
  const [unread, setUnread] = useState(0)
  
  // 获取当前活跃的标签页
  const rawActiveTab = getActiveTab('home', 'posts')
  // 确保activeTab总是有效的值
  const activeTab = ['posts', 'announcements', 'resources'].includes(rawActiveTab) 
    ? rawActiveTab 
    : 'posts'

  // 为每个标签页维护独立的数据状态
  const [tabsData, setTabsData] = useState<Record<string, TabDataState>>({
    posts: {
      data: [],
      loading: false,
      error: null,
      page: 1,
      hasMore: true,
      lastUpdate: 0
    },
    announcements: {
      data: [],
      loading: false,
      error: null,
      page: 1,
      hasMore: true,
      lastUpdate: 0
    },
    resources: {
      data: [],
      loading: false,
      error: null,
      page: 1,
      hasMore: true,
      lastUpdate: 0
    }
  })

  // 调试日志 - 检查activeTab值
  React.useEffect(() => {
    console.log('HomeScreen activeTab:', { rawActiveTab, activeTab, availableTabs: Object.keys(tabsData) })
  }, [rawActiveTab, activeTab])

  // 格式化日期显示
  const formatDate = (dateString: string) => {
    const date = new Date(dateString)
    const now = new Date()
    const diffTime = Math.abs(now.getTime() - date.getTime())
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
    
    if (diffDays === 1) return '今天'
    if (diffDays === 2) return '昨天'
    if (diffDays <= 7) return `${diffDays}天前`
    
    return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
  }

  // 格式化数字显示
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 处理卡片点击
  const handleCardClick = (card: ContentItem) => {
    switch (card.type) {
      case 'post':
        navigate(`/post/${card.id}`)
        break
      case 'resource':
        navigate(`/resource/${card.id}`)
        break
      case 'announcement':
        navigate(`/announcement/${card.id}`)
        break
      default:
        navigate(`/post/${card.id}`)
    }
  }

  // 数据映射函数
  const mapApiDataToContentItem = (item: any, type: 'post' | 'resource' | 'announcement'): ContentItem => {
    // 处理tags字段，确保它是数组格式
    let tags: string[] = []
    if (item.tags) {
      if (Array.isArray(item.tags)) {
        tags = item.tags
      } else if (typeof item.tags === 'string') {
        try {
          // 尝试解析JSON字符串
          const parsed = JSON.parse(item.tags)
          tags = Array.isArray(parsed) ? parsed : []
        } catch {
          // 如果解析失败，当作单个标签处理
          tags = [item.tags]
        }
      }
    }
    
    return {
      id: item.id,
      type,
      title: item.title,
      description: item.description || item.summary || '',
      tags,
      author: item.author_detail ? {
        id: item.author_detail.id,
        name: item.author_detail.nickname || item.author_detail.name,
        avatar: item.author_detail.avatar || ''
      } : { 
        name: item.author_name || (typeof item.author === 'string' ? item.author : item.author?.name) || '用户', 
        avatar: item.author_avatar || item.author?.avatar || '' 
      },
      likes: item.stats?.likes || item.like_count || item.likes || 0,
      comments: item.stats?.comments || item.comment_count || item.comments || 0,
      views: item.stats?.views || item.view_count || item.views || item.views_count || 0,
      downloads: item.stats?.downloads || item.download_count || item.downloads || 0,
      date: item.created_at || item.publishedAt || item.publishDate,
      isTop: item.is_pinned || item.isPinned || false,
      isHot: item.is_featured || false,
      category: item.category,
    }
  }

  // 加载指定标签页数据
  const loadTabData = async (tabType: string, refresh = false) => {
    const currentState = tabsData[tabType]
    
    // 验证tabType是否有效
    if (!currentState) {
      console.warn(`Cannot load data for invalid tab type: ${tabType}`)
      return
    }
    
    const now = Date.now()
    
    // 如果数据较新且不是刷新操作，则跳过加载
    if (!refresh && currentState.data.length > 0 && (now - currentState.lastUpdate) < 30000) {
      return
    }

    // 设置加载状态
    setTabsData(prev => ({
      ...prev,
      [tabType]: {
        ...prev[tabType],
        loading: true,
        error: null
      }
    }))

    try {
      let response: any
      let mappedData: ContentItem[] = []

      switch (tabType) {
        case 'posts':
          response = await getPosts({ page: 1, pageSize: 20 })
          mappedData = (response.list || []).map((item: any) => 
            mapApiDataToContentItem(item, 'post')
          )
          break

        case 'announcements':
          const announcements = await getAnnouncements({ page: 1, pageSize: 20 })
          mappedData = announcements.map((item: any) => 
            mapApiDataToContentItem(item, 'announcement')
          )
          break

        case 'resources':
          response = await getResources({ page: 1, page_size: 20 })
          mappedData = (response.list || []).map((item: any) => 
            mapApiDataToContentItem(item, 'resource')
          )
          break

        default:
          console.warn(`未知的标签页类型: ${tabType}`)
          return
      }

      // 更新数据状态
      setTabsData(prev => ({
        ...prev,
        [tabType]: {
          ...prev[tabType],
          data: mappedData,
          loading: false,
          error: null,
          lastUpdate: now,
          hasMore: response?.total ? mappedData.length < response.total : false
        }
      }))

    } catch (error) {
      console.error(`加载${tabType}数据失败:`, error)
      setTabsData(prev => ({
        ...prev,
        [tabType]: {
          ...prev[tabType],
          loading: false,
          error: `加载失败，请稍后重试`
        }
      }))
    }
  }

  // 标签页切换处理
  const handleTabChange = (newTab: string) => {
    // 验证newTab是否为有效值
    if (!['posts', 'announcements', 'resources'].includes(newTab)) {
      console.warn(`Invalid tab type: ${newTab}, defaulting to 'posts'`)
      newTab = 'posts'
    }
    
    setActiveTab('home', newTab)
    // 立即加载新标签页的数据
    loadTabData(newTab)
  }
  
  // 分类数据
  const [categories, setCategories] = useState<(Category & { icon: any; color: string })[]>([])
  const [loadingCategories, setLoadingCategories] = useState(true)

  // 分类图标映射
  const categoryIcons = [Code, BookOpen, Zap, Star, Clock, Bookmark]
  const categoryColors = [
    'bg-blue-100 dark:bg-blue-900',
    				'bg-green-100 dark:bg-green-900',
    'bg-yellow-100 dark:bg-yellow-900',
    'bg-purple-100 dark:bg-purple-900',
    'bg-red-100 dark:bg-red-900',
    'bg-indigo-100 dark:bg-indigo-900'
  ]

  // 加载分类数据
  useEffect(() => {
    const loadCategories = async () => {
      try {
        setLoadingCategories(true)
        const categoryList = await getCategories()
        const categoriesWithStyle = categoryList.map((cat, index) => ({
          ...cat,
          icon: categoryIcons[index % categoryIcons.length] || Code,
          color: categoryColors[index % categoryColors.length]
        }))
        setCategories(categoriesWithStyle)
      } catch (error) {
        console.error('加载分类失败:', error)
        // 使用备用分类数据
        setCategories([
          { id: 1, name: '基础语法', icon: Code, color: 'bg-blue-100 dark:bg-blue-900' },
          		{ id: 2, name: '学习资源', icon: BookOpen, color: 'bg-green-100 dark:bg-green-900' },
          { id: 3, name: '实战项目', icon: Zap, color: 'bg-yellow-100 dark:bg-yellow-900' },
          { id: 4, name: '精选内容', icon: Star, color: 'bg-purple-100 dark:bg-purple-900' },
          { id: 5, name: '最近更新', icon: Clock, color: 'bg-red-100 dark:bg-red-900' },
          { id: 6, name: '我的收藏', icon: Bookmark, color: 'bg-indigo-100 dark:bg-indigo-900' },
        ])
      } finally {
        setLoadingCategories(false)
      }
    }
    
    loadCategories()
  }, [])

  // 初始化数据加载
  useEffect(() => {
    // 加载当前活跃标签页的数据
    loadTabData(activeTab)
    
    // 加载热门搜索和未读消息
    trendingKeywords().then(setHotKeywords).catch(() => setHotKeywords([]))
    getUnreadCount().then(data => setUnread(data.count)).catch(() => setUnread(0))
  }, [])

  // 首屏兜底：短延时后仍为空则强制刷新一次
  useEffect(() => {
    const t = setTimeout(() => {
      const tabState = tabsData[activeTab]
      if (tabState && !tabState.loading && tabState.data.length === 0) {
        loadTabData(activeTab, true)
      }
    }, 300)
    return () => clearTimeout(t)
  }, [activeTab, tabsData])

  // 窗口聚焦/页面可见/网络恢复/应用恢复时刷新当前标签
  useEffect(() => {
    const onFocus = () => loadTabData(activeTab, true)
    const onVisible = () => { if (document.visibilityState === 'visible') loadTabData(activeTab, true) }
    const onOnline = () => loadTabData(activeTab, true)
    window.addEventListener('focus', onFocus)
    document.addEventListener('visibilitychange', onVisible)
    window.addEventListener('online', onOnline)
    
    let listenerPromise: Promise<any> | null = null
    if (CapacitorApp?.addListener) {
      listenerPromise = CapacitorApp.addListener('resume', () => loadTabData(activeTab, true))
        .catch((error) => {
          console.warn('Failed to add Capacitor App resume listener:', error)
          return null
        })
    }
    
    return () => {
      window.removeEventListener('focus', onFocus)
      document.removeEventListener('visibilitychange', onVisible)
      window.removeEventListener('online', onOnline)
      if (listenerPromise) {
        listenerPromise.then((listener) => {
          if (listener && typeof listener.remove === 'function') {
            listener.remove().catch((error: any) => {
              console.warn('Failed to remove Capacitor App resume listener:', error)
            })
          }
        })
      }
    }
  }, [activeTab])



  // 渲染内容卡片
  const renderContentCard = (card: ContentItem) => (
    <motion.div
      key={card.id}
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
    >
      <Card 
        className={`relative cursor-pointer hover:shadow-lg hover:scale-[1.02] transition-all duration-200 border-l-2 group ${
          card.isTop 
            ? 'border-l-orange-400 bg-orange-50/30 dark:bg-orange-950/20 hover:border-l-orange-500' 
            : card.type === 'resource' 
              				? 'border-l-green-400 hover:border-l-green-500'
              : card.type === 'announcement'
                ? 'border-l-blue-400 hover:border-l-blue-500'
                : 'border-l-purple-400 hover:border-l-purple-500'
        }`}
        onClick={() => handleCardClick(card)}
      >
        <CardContent className="p-3 overflow-hidden">
          {/* 头部信息 - 更紧凑 */}
          <div className="flex items-center justify-between mb-2 overflow-hidden">
            <div className="flex items-center gap-1.5 min-w-0 flex-1 overflow-hidden">
              <Avatar className="h-5 w-5 flex-shrink-0 transition-transform duration-200 group-hover:scale-110">
                <AvatarImage src={card.author.avatar} />
                <AvatarFallback className="text-xs">{card.author.name[0]}</AvatarFallback>
              </Avatar>
              <span className="text-xs text-muted-foreground truncate max-w-[80px]">{card.author.name}</span>
              
              {/* 类型标识 */}
              <div className="flex items-center gap-1 flex-shrink-0">
                				{card.type === 'resource' && <Package size={12} className="text-green-500" />}
                {card.type === 'announcement' && <Bell size={12} className="text-blue-500" />}
                {card.type === 'post' && <BookOpen size={12} className="text-purple-500" />}
              </div>
            </div>
            
            {/* 状态标签 - 更小更紧凑 */}
            <div className="flex items-center gap-1 flex-shrink-0 z-10">
              {card.isTop && (
                <Badge className="bg-orange-500 text-white text-[10px] px-1 py-0 h-4 border-0 shadow-sm">
                  <Pin size={8} className="mr-0.5" />
                  置顶
                </Badge>
              )}
              {card.isHot && (
                <Badge className="bg-amber-500 text-white text-[10px] px-1 py-0 h-4 border-0 shadow-sm">
                  <Star size={8} className="mr-0.5 fill-current" />
                  精华
                </Badge>
              )}
            </div>
          </div>
          
          {/* 标题 - 限制行数防止溢出 */}
          <h3 className="font-medium text-base mb-1.5 line-clamp-2 leading-tight break-words hyphens-auto overflow-hidden">{card.title}</h3>
          
          {/* 描述 - 更小字体，限制行数 */}
          <p className="text-muted-foreground text-xs mb-2 line-clamp-2 leading-relaxed break-words overflow-hidden">{card.description}</p>
          

          
          {/* 底部统计信息 - 单行紧凑布局 */}
          <div className="flex items-center justify-between text-[10px] text-muted-foreground pt-1 border-t border-border/50 overflow-hidden">
            <div className="flex items-center gap-3 overflow-hidden">
              <div className="flex items-center gap-0.5">
                <Eye size={10} className="flex-shrink-0" />
                <span className="truncate">{formatNumber(card.views)}</span>
              </div>
              <div className="flex items-center gap-0.5">
                <Star size={10} className="flex-shrink-0" />
                <span className="truncate">{formatNumber(card.likes)}</span>
              </div>
              {card.type === 'resource' && (
                <div className="flex items-center gap-0.5">
                  <Download size={10} className="flex-shrink-0" />
                  <span className="truncate">{formatNumber(card.downloads || 0)}</span>
                </div>
              )}
            </div>
            <div className="flex items-center gap-0.5 text-[10px] flex-shrink-0">
              <Calendar size={10} />
              <span className="truncate max-w-[60px]">{formatDate(card.date)}</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  )

  // 渲染加载状态
  const renderLoadingState = () => (
    <div className="space-y-4">
      {[...Array(3)].map((_, index) => (
        <SkeletonCard key={index} />
      ))}
    </div>
  )

  // 渲染错误状态
  const renderErrorState = (error: string, tabType: string) => (
    <div className="flex flex-col items-center justify-center py-12 space-y-4">
      <div className="text-center">
        <p className="text-sm text-muted-foreground mb-4">{error}</p>
        <Button 
          variant="outline" 
          onClick={() => loadTabData(tabType, true)}
        >
          重试
        </Button>
      </div>
    </div>
  )

  // 渲染空状态
  const renderEmptyState = (tabType: string) => {
    const messages = {
      posts: '暂无帖子',
      announcements: '暂无公告',
      resources: '暂无资源'
    }
    
    return (
      <div className="flex flex-col items-center justify-center py-12 space-y-4">
        <p className="text-sm text-muted-foreground">{messages[tabType as keyof typeof messages]}</p>
        <Button 
          variant="outline" 
          onClick={() => loadTabData(tabType, true)}
        >
          刷新
        </Button>
      </div>
    )
  }

  // 渲染标签页内容
  const renderTabContent = (tabType: string) => {
    const tabState = tabsData[tabType]
    
    // 安全检查：如果tabState不存在，返回空状态
    if (!tabState) {
      console.warn(`Tab state for '${tabType}' is undefined. Available tabs:`, Object.keys(tabsData))
      return renderEmptyState(tabType)
    }
    
    if (tabState.loading) {
      return renderLoadingState()
    }
    
    if (tabState.error) {
      return renderErrorState(tabState.error, tabType)
    }
    
    if (tabState.data.length === 0) {
      return renderEmptyState(tabType)
    }

    // 排序数据（置顶优先）
    const sortedData = [...tabState.data].sort((a, b) => {
      if (a.isTop && !b.isTop) return -1
      if (!a.isTop && b.isTop) return 1
      return 0
    })

    return (
      <div className="space-y-4">
        {sortedData.map(renderContentCard)}
      </div>
    )
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="结绳社区"
        subtitle="学习交流，共同进步"
        showNotificationButton
        notificationCount={unread}
        showSearchButton
        onSearchClick={() => navigate('/search')}
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}

      {/* 分类快捷入口 */}
      <div className="p-4 max-w-4xl mx-auto">
        <div className="relative">
          {/* 装饰性标题 */}
          <div className="flex items-center mb-6">
            <div className="flex-shrink-0 w-1 h-6 bg-gradient-to-b from-primary to-primary/60 rounded-full mr-3"></div>
            <h2 className="text-lg font-medium">快速导航</h2>
            <div className="flex-1 ml-4 h-px bg-gradient-to-r from-border to-transparent"></div>
          </div>
          
          {/* 导航网格 */}
          <div className="grid grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
            {categories.map((category, index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: index * 0.1 }}
                whileHover={{ 
                  scale: 1.05,
                  transition: { duration: 0.2 }
                }}
                whileTap={{ scale: 0.95 }}
                className="flex flex-col items-center cursor-pointer group"
              >
                {/* 图标容器 - 添加悬停效果和阴影 */}
                <div className={`
                  relative flex items-center justify-center w-14 h-14 rounded-2xl mb-3
                  ${category.color} 
                  transition-all duration-300 
                  group-hover:shadow-lg group-hover:shadow-primary/20
                  group-hover:-translate-y-1
                  before:absolute before:inset-0 before:rounded-2xl 
                  before:bg-gradient-to-br before:from-white/20 before:to-transparent
                  before:opacity-0 group-hover:before:opacity-100 before:transition-opacity
                `}>
                  <category.icon size={24} className="text-foreground relative z-10" />
                  
                  {/* 装饰性光晕效果 */}
                  <div className="absolute -inset-1 rounded-2xl bg-gradient-to-br from-primary/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 -z-10"></div>
                </div>
                
                {/* 文字标签 - 添加悬停效果 */}
                <span className="text-xs text-center transition-colors duration-200 group-hover:text-primary font-medium">
                  {category.name}
                </span>
              </motion.div>
            ))}
          </div>
          
          {/* 底部装饰线 */}
          <div className="mt-6 h-px bg-gradient-to-r from-transparent via-border to-transparent"></div>
        </div>
      </div>

      {/* 内容标签页 */}
      <div className="px-4 flex-1 max-w-4xl mx-auto">
        <Tabs value={activeTab} onValueChange={handleTabChange} className="w-full mb-4">
          <TabsList className="grid grid-cols-3 max-w-md mx-auto md:max-w-lg">
            <TabsTrigger value="posts" className="text-xs md:text-sm">帖子</TabsTrigger>
            <TabsTrigger value="announcements" className="text-xs md:text-sm">公告</TabsTrigger>
            <TabsTrigger value="resources" className="text-xs md:text-sm">资源</TabsTrigger>
          </TabsList>
        </Tabs>
        
        <AnimatePresence mode="wait">
              <motion.div
            key={activeTab}
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            transition={{ duration: 0.2 }}
          >
            {renderTabContent(activeTab)}
              </motion.div>
        </AnimatePresence>
      </div>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default HomeScreen