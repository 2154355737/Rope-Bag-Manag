import React, { useState, useEffect, useMemo, useCallback } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { 
  Search, 
  Filter, 
  Grid3X3, 
  List, 
  Star, 
  Download, 
  Eye, 
  Heart, 
  Clock, 
  ChevronDown,
  Loader2,
  RefreshCw,
  TrendingUp,
  Calendar
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Separator } from '@/components/ui/separator'
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from '@/components/ui/sheet'
import TopNavigation from '@/components/ui/top-navigation'
import { useNavigation } from '@/contexts/NavigationContext'
import { getCategories } from '../api/categories'
import { getResources } from '../api/resources'

// 资源数据接口
interface Resource {
  id: number
  title: string
  author?: string
  description?: string
  image?: string
  tags: string[]
  downloads: number
  views: number
  likes: number
  rating: number
  createdAt: string
  isPinned: boolean
  categoryId?: number
}

// 分类数据接口
interface Category {
  id: number | string
  name: string
  description?: string
  count?: number
}

// 排序选项
type SortOption = 'latest' | 'popular' | 'downloads' | 'rating'

// 显示模式
type ViewMode = 'grid' | 'list'

const CategoryScreen: React.FC = () => {
  const { getActiveTab, setActiveTab } = useNavigation()
  
  // 状态管理
  const [categories, setCategories] = useState<Category[]>([])
  const [resources, setResources] = useState<Resource[]>([])
  const [loading, setLoading] = useState(true)
  const [refreshing, setRefreshing] = useState(false)
  const [selectedCategory, setSelectedCategory] = useState<string | number>('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [sortBy, setSortBy] = useState<SortOption>('latest')
  const [showFilters, setShowFilters] = useState(false)
  
  // 获取当前视图模式
  const viewMode = getActiveTab('category', 'grid') as ViewMode

  // 加载分类数据
  const loadCategories = useCallback(async () => {
    try {
      const categoryList = await getCategories()
      const categoriesWithAll = [
        { id: 'all', name: '全部', count: 0 },
        ...categoryList.map(cat => ({ ...cat, count: 0 }))
      ]
      setCategories(categoriesWithAll)
    } catch (error) {
      console.error('加载分类失败:', error)
      setCategories([{ id: 'all', name: '全部', count: 0 }])
    }
  }, [])

  // 加载资源数据
  const loadResources = useCallback(async (categoryId: string | number = 'all', refresh = false) => {
    if (refresh) {
      setRefreshing(true)
    } else {
      setLoading(true)
    }
    
    try {
      const params: any = { page: 1, page_size: 50 }
      if (categoryId !== 'all') {
        params.category_id = categoryId
      }
      
      const response = await getResources(params)
      const resourceList = (response.list || []).map((item: any): Resource => ({
        id: item.id,
        title: item.name || item.title || '未命名资源',
        author: item.author || '匿名用户',
        description: item.description || '',
        image: item.screenshots?.[0] || item.image || '',
        tags: Array.isArray(item.tags) ? item.tags : [],
        downloads: item.download_count || 0,
        views: item.view_count || 0,
        likes: item.like_count || 0,
        rating: item.rating || 0,
        createdAt: item.created_at || new Date().toISOString(),
        isPinned: item.is_pinned || false,
        categoryId: item.category_id
      }))
      
      setResources(resourceList)
    } catch (error) {
      console.error('加载资源失败:', error)
      setResources([])
    } finally {
      setLoading(false)
      setRefreshing(false)
  }
  }, [])

  // 筛选和排序资源
  const filteredAndSortedResources = useMemo(() => {
    let filtered = [...resources]

    // 搜索筛选
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase().trim()
      filtered = filtered.filter(resource =>
        resource.title.toLowerCase().includes(query) ||
        resource.author?.toLowerCase().includes(query) ||
        resource.description?.toLowerCase().includes(query) ||
        resource.tags.some(tag => tag.toLowerCase().includes(query))
      )
    }

    // 排序
    filtered.sort((a, b) => {
      switch (sortBy) {
      case 'latest':
          return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
      case 'popular':
          return (b.views + b.likes) - (a.views + a.likes)
      case 'downloads':
          return b.downloads - a.downloads
      case 'rating':
          return b.rating - a.rating
        default:
          return 0
      }
    })

    // 置顶资源排在前面
    return filtered.sort((a, b) => (b.isPinned ? 1 : 0) - (a.isPinned ? 1 : 0))
  }, [resources, searchQuery, sortBy])

  // 初始化数据
  useEffect(() => {
    const initData = async () => {
      await Promise.all([
        loadCategories(),
        loadResources('all')
      ])
    }
    initData()
  }, [loadCategories, loadResources])

  // 处理分类切换
  const handleCategoryChange = useCallback((categoryId: string | number) => {
    setSelectedCategory(categoryId)
    setSearchQuery('') // 清空搜索
    loadResources(categoryId)
  }, [loadResources])

  // 处理刷新
  const handleRefresh = useCallback(() => {
    loadResources(selectedCategory, true)
  }, [loadResources, selectedCategory])

  // 获取排序选项的显示文本
  const getSortText = (sort: SortOption) => {
    const sortTexts = {
      latest: '最新发布',
      popular: '最受欢迎',
      downloads: '下载最多',
      rating: '评分最高'
    }
    return sortTexts[sort]
  }

  // 格式化数字显示
  const formatNumber = (num: number) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`
    return num.toString()
  }

  // 格式化时间显示
  const formatTime = (dateString: string) => {
    const date = new Date(dateString)
    const now = new Date()
    const diffTime = now.getTime() - date.getTime()
    const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24))
    
    if (diffDays === 0) return '今天'
    if (diffDays === 1) return '昨天'
    if (diffDays < 7) return `${diffDays}天前`
    if (diffDays < 30) return `${Math.floor(diffDays / 7)}周前`
    if (diffDays < 365) return `${Math.floor(diffDays / 30)}个月前`
    return `${Math.floor(diffDays / 365)}年前`
  }

  // 资源卡片组件 - 网格模式
  const ResourceGridCard = ({ resource }: { resource: Resource }) => {
    const navigate = useNavigate()
    
    const handleCardClick = () => {
      navigate(`/resource/${resource.id}`)
    }
    
    return (
      <motion.div
        layout
        initial={{ opacity: 0, scale: 0.9 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.9 }}
        transition={{ duration: 0.2 }}
        className="group cursor-pointer"
        onClick={handleCardClick}
        onMouseLeave={() => {
          // 确保鼠标离开时重置所有hover状态
        }}
      >
        <Card className="overflow-hidden h-full hover:shadow-lg transition-all duration-200 border border-border/60 bg-card shadow-sm">
          {/* 图片区域 */}
          <div className="relative aspect-[4/3] overflow-hidden">
            {resource.image ? (
              <img 
                src={resource.image} 
                alt={resource.title}
                className="w-full h-full object-cover transition-transform duration-200"
                loading="lazy"
              />
            ) : (
              <div className="w-full h-full bg-gradient-to-br from-primary/15 to-accent/15 flex items-center justify-center">
                <div className="text-center text-muted-foreground">
                  <Grid3X3 size={24} className="mx-auto mb-1" />
                  <span className="text-xs">暂无封面</span>
                </div>
              </div>
            )}
            
            {/* 置顶标记 */}
            {resource.isPinned && (
              <div className="absolute top-2 left-2">
                <Badge variant="secondary" className="text-xs px-2 py-0.5 bg-primary/90 text-primary-foreground">
                  置顶
                </Badge>
              </div>
            )}
            
            {/* 评分 */}
            {resource.rating > 0 && (
              <div className="absolute top-2 right-2">
                <Badge variant="secondary" className="text-xs px-2 py-0.5 bg-background/90 text-foreground flex items-center gap-1">
                  <Star size={10} className="fill-yellow-400 text-yellow-400" />
                  {resource.rating.toFixed(1)}
                </Badge>
              </div>
            )}
          </div>
          
          {/* 内容区域 */}
          <CardContent className="p-3 space-y-2">
            <div>
              <h3 className="font-medium text-sm line-clamp-2 text-foreground">
                {resource.title}
              </h3>
              {resource.author && (
                <p className="text-xs text-muted-foreground mt-1">
                  by {resource.author}
                </p>
              )}
            </div>
            
            {/* 标签 */}
            {resource.tags.length > 0 && (
              <div className="flex flex-wrap gap-1">
                {resource.tags.slice(0, 2).map((tag, index) => (
                  <Badge key={index} variant="outline" className="text-xs px-1.5 py-0">
                    {tag}
                  </Badge>
                ))}
                {resource.tags.length > 2 && (
                  <Badge variant="outline" className="text-xs px-1.5 py-0">
                    +{resource.tags.length - 2}
                  </Badge>
                )}
              </div>
            )}
            
            {/* 统计信息 */}
            <div className="flex items-center justify-between text-xs text-foreground/70 pt-1">
              <div className="flex items-center gap-3">
                <span className="flex items-center gap-1">
                  <Download size={10} />
                  {formatNumber(resource.downloads)}
                </span>
                <span className="flex items-center gap-1">
                  <Heart size={10} />
                  {formatNumber(resource.likes)}
                </span>
              </div>
              <span className="flex items-center gap-1">
                <Clock size={10} />
                {formatTime(resource.createdAt)}
              </span>
            </div>
          </CardContent>
        </Card>
      </motion.div>
    )
  }

  // 资源卡片组件 - 列表模式
  const ResourceListCard = ({ resource }: { resource: Resource }) => {
    const navigate = useNavigate()
    
    const handleCardClick = () => {
      navigate(`/resource/${resource.id}`)
    }
    
    return (
      <motion.div
        layout
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -10 }}
        transition={{ duration: 0.2 }}
        className="group cursor-pointer"
        onClick={handleCardClick}
        onMouseLeave={() => {
          // 确保鼠标离开时重置所有hover状态
        }}
      >
        <Card className="overflow-hidden hover:shadow-md transition-all duration-200 border border-border/60 bg-card shadow-sm">
          <div className="flex p-3 gap-3">
            {/* 缩略图 */}
            <div className="w-16 h-16 rounded-lg overflow-hidden flex-shrink-0">
              {resource.image ? (
                <img 
                  src={resource.image} 
                  alt={resource.title}
                  className="w-full h-full object-cover transition-transform duration-200"
                  loading="lazy"
                />
              ) : (
                <div className="w-full h-full bg-gradient-to-br from-primary/15 to-accent/15 flex items-center justify-center">
                  <Grid3X3 size={14} className="text-muted-foreground" />
                </div>
              )}
            </div>
            
            {/* 内容 */}
            <div className="flex-1 min-w-0 space-y-1">
              <div className="flex items-start justify-between gap-2">
                <div className="min-w-0 flex-1">
                  <h3 className="font-medium text-sm line-clamp-1 text-foreground">
                    {resource.title}
                  </h3>
                  {resource.author && (
                    <p className="text-xs text-muted-foreground">
                      by {resource.author}
                    </p>
                  )}
                </div>
                
                {/* 置顶和评分 */}
                <div className="flex items-center gap-1 flex-shrink-0">
                  {resource.isPinned && (
                    <Badge variant="secondary" className="text-xs px-1.5 py-0 bg-primary/90 text-primary-foreground">
                      置顶
                    </Badge>
                  )}
                  {resource.rating > 0 && (
                    <Badge variant="outline" className="text-xs px-1.5 py-0 flex items-center gap-1">
                      <Star size={8} className="fill-yellow-400 text-yellow-400" />
                      {resource.rating.toFixed(1)}
                    </Badge>
                  )}
                </div>
              </div>
              
              {/* 标签 */}
              {resource.tags.length > 0 && (
                <div className="flex flex-wrap gap-1">
                  {resource.tags.slice(0, 3).map((tag, index) => (
                    <Badge key={index} variant="outline" className="text-xs px-1.5 py-0">
                      {tag}
                    </Badge>
                  ))}
                  {resource.tags.length > 3 && (
                    <Badge variant="outline" className="text-xs px-1.5 py-0">
                      +{resource.tags.length - 3}
                    </Badge>
                  )}
                </div>
              )}
              
              {/* 统计信息 */}
              <div className="flex items-center justify-between text-xs text-foreground/70">
                <div className="flex items-center gap-3">
                  <span className="flex items-center gap-1">
                    <Download size={10} />
                    {formatNumber(resource.downloads)}
                  </span>
                  <span className="flex items-center gap-1">
                    <Eye size={10} />
                    {formatNumber(resource.views)}
                  </span>
                  <span className="flex items-center gap-1">
                    <Heart size={10} />
                    {formatNumber(resource.likes)}
                  </span>
                </div>
                <span className="flex items-center gap-1">
                  <Clock size={10} />
                  {formatTime(resource.createdAt)}
                </span>
              </div>
            </div>
          </div>
        </Card>
      </motion.div>
    )
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航 */}
      <TopNavigation
        title="分类"
        subtitle={`发现 ${filteredAndSortedResources.length} 个精彩资源`}
        rightAction={
          <div className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="icon"
              onClick={handleRefresh}
              disabled={refreshing}
              className="h-9 w-9"
            >
              <RefreshCw size={16} className={refreshing ? 'animate-spin' : ''} />
              </Button>
          </div>
        }
      />

      {/* 主内容区域 */}
      <div className="pt-nav flex-1 flex flex-col">
        {/* 分类选择和搜索 */}
        <div className="sticky top-nav bg-background/80 backdrop-blur-sm border-b z-10">
          <div className="p-4 space-y-3">
            {/* 分类选择 */}
            <div className="flex items-center gap-2 overflow-x-auto pb-1">
                {categories.map((category) => (
                  <Button
                    key={category.id}
                  variant={selectedCategory === category.id ? "default" : "outline"}
                  size="sm"
                  onClick={() => handleCategoryChange(category.id)}
                  className="flex-shrink-0 h-8 px-3 text-xs"
                  >
                    {category.name}
                  </Button>
                ))}
              </div>
            
            {/* 搜索和筛选 */}
            <div className="flex items-center gap-2">
              <div className="relative flex-1">
                <Search size={16} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                      <Input
                  placeholder="搜索资源..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-9 h-9 text-sm"
                      />
                    </div>

              <Select value={sortBy} onValueChange={(value) => setSortBy(value as SortOption)}>
                <SelectTrigger className="w-32 h-9 text-sm">
                          <SelectValue />
                        </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="latest">
                    <div className="flex items-center gap-2">
                      <Calendar size={12} />
                      最新
                          </div>
                        </SelectItem>
                        <SelectItem value="popular">
                    <div className="flex items-center gap-2">
                      <TrendingUp size={12} />
                      热门
                          </div>
                        </SelectItem>
                        <SelectItem value="downloads">
                    <div className="flex items-center gap-2">
                      <Download size={12} />
                      下载
                    </div>
                        </SelectItem>
                        <SelectItem value="rating">
                    <div className="flex items-center gap-2">
                      <Star size={12} />
                      评分
                    </div>
                        </SelectItem>
                      </SelectContent>
                    </Select>
              
              <Tabs value={viewMode} onValueChange={(value) => setActiveTab('category', value)}>
                <TabsList className="h-9">
                  <TabsTrigger value="grid" className="px-2">
                    <Grid3X3 size={14} />
                  </TabsTrigger>
                  <TabsTrigger value="list" className="px-2">
                    <List size={14} />
                  </TabsTrigger>
                </TabsList>
              </Tabs>
                      </div>
                    </div>
                  </div>

        {/* 资源列表 */}
        <div className="flex-1 p-4">
          {loading ? (
            <div className="flex items-center justify-center py-12">
              <div className="text-center space-y-2">
                <Loader2 size={24} className="animate-spin mx-auto text-primary" />
                <p className="text-sm text-muted-foreground">加载中...</p>
                      </div>
                    </div>
          ) : filteredAndSortedResources.length === 0 ? (
            <div className="flex items-center justify-center py-12">
              <div className="text-center space-y-2">
                <Search size={24} className="mx-auto text-muted-foreground" />
                <p className="text-sm text-muted-foreground">
                  {searchQuery ? '没有找到匹配的资源' : '暂无资源'}
                </p>
                {searchQuery && (
                  <Button variant="outline" size="sm" onClick={() => setSearchQuery('')}>
                    清空搜索
                  </Button>
                )}
              </div>
          </div>
          ) : (
            <Tabs value={viewMode} className="w-full">
          <TabsContent value="grid" className="mt-0">
                <motion.div
                  className="grid grid-cols-2 gap-4"
                  layout
                >
                  <AnimatePresence>
                    {filteredAndSortedResources.map((resource) => (
                      <ResourceGridCard key={resource.id} resource={resource} />
                    ))}
                  </AnimatePresence>
                </motion.div>
          </TabsContent>
          
              <TabsContent value="list" className="mt-0">
              <motion.div
                  className="space-y-3"
                  layout
                >
                  <AnimatePresence>
                    {filteredAndSortedResources.map((resource) => (
                      <ResourceListCard key={resource.id} resource={resource} />
                    ))}
                  </AnimatePresence>
              </motion.div>
          </TabsContent>
        </Tabs>
          )}
      </div>
      </div>
    </div>
  )
}

export default CategoryScreen