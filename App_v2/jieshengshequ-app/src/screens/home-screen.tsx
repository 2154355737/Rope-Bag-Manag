import React, { useState, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Search, Bell, Code, BookOpen, Zap, Star, Clock, Bookmark, Pin, X, Eye, Download, Calendar } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopNavigation from '@/components/ui/top-navigation'
import { getUnreadCount } from '../api/notifications'
import { trendingKeywords, suggestKeywords } from '../api/search'
import { fetchFeed } from '../api/feed'
import { useNavigation } from '@/contexts/NavigationContext'
import { getCategories, Category } from '@/api/categories'

const HomeScreen: React.FC = () => {
  const navigate = useNavigate()
  const { getActiveTab, setActiveTab } = useNavigation()
  const [searchFocused, setSearchFocused] = useState(false)
  const [searchValue, setSearchValue] = useState('')
  const [isDropdownInteracting, setIsDropdownInteracting] = useState(false)
  const [hotKeywords, setHotKeywords] = useState<string[]>([])
  const [suggestions, setSuggestions] = useState<string[]>([])
  const [unread, setUnread] = useState(0)
  
  // 获取当前活跃的标签页
  const activeTab = getActiveTab('home', 'posts')

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
  const handleCardClick = (card: any) => {
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

  const [allContent, setAllContent] = useState<any[]>([])

  useEffect(() => {
    const loadFeed = async () => {
      const data = await fetchFeed({ page: 1, pageSize: 20 })
      const mapped = (data.items || []).map((i: any) => ({
        id: i.id,
        type: i.type || i.item_type,
        title: i.title,
        description: i.description,
        tags: i.tags || [],
        // 使用新的author_detail字段，如果没有则回退到原有逻辑
        author: i.author_detail ? {
          id: i.author_detail.id,
          name: i.author_detail.nickname || i.author_detail.name,
          avatar: i.author_detail.avatar || ''
        } : { 
          name: i.author_name || (typeof i.author === 'string' ? i.author : i.author?.name) || '用户', 
          avatar: i.author_avatar || i.author?.avatar || '' 
        },
        likes: i.stats?.likes || i.like_count || 0,
        comments: i.stats?.comments || i.comment_count || 0,
        views: i.stats?.views || i.view_count || 0,
        downloads: i.stats?.downloads || i.download_count || 0,
        date: i.created_at || i.publishedAt,
        isTop: i.is_pinned || false,
        isHot: i.is_featured || false,
        category: i.category, // 添加分类信息
      }))
      setAllContent(mapped)
    }
    loadFeed()
  }, [])

  useEffect(() => {
    // 加载热门搜索
    trendingKeywords().then(setHotKeywords).catch(() => setHotKeywords([]))
    getUnreadCount().then(data => setUnread(data.count)).catch(() => setUnread(0))
  }, [])

  useEffect(() => {
    const t = setTimeout(() => {
      if (searchValue.trim()) {
        suggestKeywords(searchValue.trim()).then(setSuggestions).catch(() => setSuggestions([]))
      } else {
        setSuggestions([])
      }
    }, 250)
    return () => clearTimeout(t)
  }, [searchValue])

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="结绳社区"
        subtitle="学习交流，共同进步"
        showNotificationButton
        notificationCount={unread}
        showSearchButton
        onSearchClick={() => setSearchFocused(true)}
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        {/* 搜索框 */}
        <div className="p-4">
        <div className="relative">
          <Input
            id="search-input"
            name="search"
            placeholder="搜索资源、话题、用户..."
            className="pl-10 pr-4 py-6 rounded-full"
            value={searchValue}
            onChange={(e) => setSearchValue(e.target.value)}
            onFocus={() => setSearchFocused(true)}
            onBlur={() => {
              // 延迟关闭，允许点击下拉框内容
              setTimeout(() => {
                if (!isDropdownInteracting) {
                  setSearchFocused(false)
                }
              }, 150)
            }}
            autoComplete="search"
          />
          <Search className="absolute left-3 top-3 text-muted-foreground" size={20} />
          {searchValue && (
            <Button
              variant="ghost"
              size="sm"
              className="absolute right-2 top-2 h-8 w-8 p-0 hover:bg-muted"
              onClick={() => {
                setSearchValue('')
                setSearchFocused(false)
                setIsDropdownInteracting(false)
              }}
            >
              <X size={16} />
            </Button>
          )}
          {searchFocused && (
            <div
              className="absolute left-0 right-0 top-full mt-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg p-4 z-[60] max-h-80 overflow-y-auto"
              onMouseEnter={() => setIsDropdownInteracting(true)}
              onMouseLeave={() => setIsDropdownInteracting(false)}
            >
            <div className="mb-4">
              <h3 className="text-sm font-medium mb-2">热门搜索</h3>
              <div className="flex flex-wrap gap-2">
                {(hotKeywords.length ? hotKeywords : ['结绳入门','数据结构','项目实战','性能优化','面试题']).map((tag) => (
                  <Badge 
                    key={tag}
                    variant="outline" 
                    className="cursor-pointer hover:bg-accent transition-colors"
                    onClick={() => {
                      setSearchValue(tag)
                      setSearchFocused(false)
                      setIsDropdownInteracting(false)
                    }}
                  >
                    {tag}
                  </Badge>
                ))}
              </div>
            </div>
            
            <div>
              <h3 className="text-sm font-medium mb-2">搜索历史</h3>
              <div className="space-y-2 text-sm text-muted-foreground">
                {(suggestions.length ? suggestions : ['结绳语言基础教程','如何优化结绳代码']).map((s) => (
                  <div
                    key={s}
                    className="flex items-center cursor-pointer hover:bg-accent rounded-md p-2 -m-2 transition-colors"
                    onClick={() => {
                      setSearchValue(s)
                      setSearchFocused(false)
                      setIsDropdownInteracting(false)
                    }}
                  >
                    <Clock size={14} className="mr-2" />
                    <span>{s}</span>
                  </div>
                ))}
              </div>
            </div>
            </div>
          )}
        </div>
      </div>

      {/* 分类快捷入口 */}
      <div className="p-4">
        <h2 className="text-lg font-medium mb-4">快速导航</h2>
        <div className="grid grid-cols-3 gap-4">
          {categories.map((category, index) => (
            <motion.div
              key={index}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.1 }}
              className="flex flex-col items-center"
            >
              <div className={`flex items-center justify-center w-14 h-14 rounded-full ${category.color} mb-2`}>
                <category.icon size={24} className="text-foreground" />
              </div>
                                <span className="text-xs text-center">{category.name}</span>
            </motion.div>
          ))}
        </div>
      </div>

      {/* 内容标签页 */}
      <div className="px-4 flex-1">
        <Tabs value={activeTab} onValueChange={(value) => setActiveTab('home', value)} className="w-full">
          <TabsList className="grid grid-cols-3 mb-4">
            <TabsTrigger value="posts">帖子</TabsTrigger>
            <TabsTrigger value="announcements">公告</TabsTrigger>
            <TabsTrigger value="resources">资源</TabsTrigger>
          </TabsList>
          
          <TabsContent value="posts" className="space-y-4">
            {allContent.filter(item => item.type === 'post')
              .sort((a, b) => {
                // 置顶的卡片排在前面
                if (a.isTop && !b.isTop) return -1
                if (!a.isTop && b.isTop) return 1
                return 0
              })
              .map((card) => (
              <motion.div
                key={card.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3 }}
              >
                <Card 
                  className={`overflow-hidden relative cursor-pointer hover:shadow-md transition-shadow ${card.isTop ? 'ring-2 ring-orange-200 dark:ring-orange-800 ring-opacity-50' : ''}`}
                  onClick={() => handleCardClick(card)}
                >
                  <CardContent className="p-4">
                    <div className="flex items-center mb-3">
                      <Avatar className="h-6 w-6 mr-2">
                        <AvatarImage src={card.author.avatar} />
                        <AvatarFallback>{card.author.name[0]}</AvatarFallback>
                      </Avatar>
                      <span className="text-sm">{card.author.name}</span>
                      <div className="ml-auto flex items-center gap-2">
                        {card.category && (
                          <Badge variant="outline" className="text-xs">
                            {card.category.name}
                          </Badge>
                        )}
                      {card.isTop && (
                        <Badge className="bg-gradient-to-r from-orange-500 to-red-500 text-white text-xs border-0">
                          <Pin size={10} className="mr-1" />
                          置顶
                        </Badge>
                      )}
                    </div>
                  </div>
                  
                  <h3 className="font-medium text-lg mb-2">{card.title}</h3>
                  <p className="text-muted-foreground text-sm mb-3">{card.description}</p>
                  
                  {/* 标签区域 - 只在有标签时显示 */}
                  {(card.tags && card.tags.length > 0) || card.isHot ? (
                    <div className="flex flex-wrap gap-2 mb-3">
                      {card.tags && card.tags.slice(0, 3).map((tag, idx) => (
                        <Badge key={idx} variant="outline" className="text-xs">
                          {tag}
                        </Badge>
                      ))}
                      {card.tags && card.tags.length > 3 && (
                        <Badge variant="outline" className="text-xs text-muted-foreground">
                          +{card.tags.length - 3}
                        </Badge>
                      )}
                      {card.isHot && (
                        <Badge className="bg-amber-100 text-amber-700 border-amber-200 text-xs">
                          <Star size={12} className="mr-1" /> 精华
                        </Badge>
                      )}
                      </div>
                    ) : null}
                  </CardContent>
                  
                  <CardFooter className="p-4 pt-3 border-t">
                    <div className="flex items-center text-muted-foreground text-xs space-x-4">
                      <div className="flex items-center">
                        <Calendar size={14} className="mr-1" />
                        {formatDate(card.date)}
                      </div>
                      <div className="flex items-center">
                        <Eye size={14} className="mr-1" />
                        {formatNumber(card.views)}
                      </div>
                      <div className="flex items-center">
                        <Star size={14} className="mr-1" />
                        {formatNumber(card.likes)}
                      </div>
                    </div>
                  </CardFooter>
                </Card>
              </motion.div>
            ))}
          </TabsContent>
          
          <TabsContent value="announcements" className="space-y-4">
            {allContent.filter(item => item.type === 'announcement')
              .sort((a, b) => {
                // 置顶的卡片排在前面
                if (a.isTop && !b.isTop) return -1
                if (!a.isTop && b.isTop) return 1
                return 0
              })
              .map((card) => (
              <motion.div
                key={card.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3 }}
              >
                <Card 
                  className={`overflow-hidden relative cursor-pointer hover:shadow-md transition-shadow ${card.isTop ? 'ring-2 ring-orange-200 dark:ring-orange-800 ring-opacity-50' : ''}`}
                  onClick={() => handleCardClick(card)}
                >
                  <CardContent className="p-4">
                    <div className="flex items-center mb-3">
                      <Avatar className="h-6 w-6 mr-2">
                        <AvatarImage src={card.author.avatar} />
                        <AvatarFallback>{card.author.name[0]}</AvatarFallback>
                      </Avatar>
                      <span className="text-sm">{card.author.name}</span>
                      <div className="ml-auto flex items-center gap-2">
                                              <Badge variant="outline" className="text-xs">
                        公告
                      </Badge>
                      {card.isTop && (
                        <Badge className="bg-gradient-to-r from-orange-500 to-red-500 text-white text-xs border-0">
                          <Pin size={10} className="mr-1" />
                          置顶
                        </Badge>
                      )}
                    </div>
                  </div>
                  
                  <h3 className="font-medium text-lg mb-2">{card.title}</h3>
                  <p className="text-muted-foreground text-sm mb-3">{card.description}</p>
                  
                  {/* 标签区域 - 只在有标签时显示 */}
                  {(card.tags && card.tags.length > 0) || card.isHot ? (
                    <div className="flex flex-wrap gap-2 mb-3">
                      {card.tags && card.tags.slice(0, 3).map((tag, idx) => (
                        <Badge key={idx} variant="outline" className="text-xs">
                          {tag}
                        </Badge>
                      ))}
                      {card.tags && card.tags.length > 3 && (
                        <Badge variant="outline" className="text-xs text-muted-foreground">
                          +{card.tags.length - 3}
                        </Badge>
                      )}
                      {card.isHot && (
                        <Badge className="bg-amber-100 text-amber-700 border-amber-200 text-xs">
                          <Star size={12} className="mr-1" /> 精华
                        </Badge>
                      )}
                      </div>
                    ) : null}
                  </CardContent>
                  
                  <CardFooter className="p-4 pt-3 border-t">
                    <div className="flex items-center text-muted-foreground text-xs space-x-4">
                      <div className="flex items-center">
                        <Calendar size={14} className="mr-1" />
                        {formatDate(card.date)}
                      </div>
                      <div className="flex items-center">
                        <Eye size={14} className="mr-1" />
                        {formatNumber(card.views)}
                      </div>
                      <div className="flex items-center">
                        <Star size={14} className="mr-1" />
                        {formatNumber(card.likes)}
                      </div>
                      {card.type === 'resource' && card.downloads && (
                        <div className="flex items-center">
                          <Download size={14} className="mr-1" />
                          {formatNumber(card.downloads)}
                        </div>
                      )}
                    </div>
                  </CardFooter>
                </Card>
              </motion.div>
            ))}
          </TabsContent>
          
          <TabsContent value="resources" className="space-y-4">
            {allContent.filter(item => item.type === 'resource')
              .sort((a, b) => {
                // 置顶的卡片排在前面
                if (a.isTop && !b.isTop) return -1
                if (!a.isTop && b.isTop) return 1
                return 0
              })
              .map((card) => (
              <motion.div
                key={card.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3 }}
              >
                <Card 
                  className={`overflow-hidden relative cursor-pointer hover:shadow-md transition-shadow ${card.isTop ? 'ring-2 ring-orange-200 dark:ring-orange-800 ring-opacity-50' : ''}`}
                  onClick={() => handleCardClick(card)}
                >
                  <CardContent className="p-4">
                    <div className="flex items-center mb-3">
                      <Avatar className="h-6 w-6 mr-2">
                        <AvatarImage src={card.author.avatar} />
                        <AvatarFallback>{card.author.name[0]}</AvatarFallback>
                      </Avatar>
                      <span className="text-sm">{card.author.name}</span>
                      <div className="ml-auto flex items-center gap-2">
                        {card.category && (
                          <Badge variant="secondary" className="text-xs">
                            {card.category.name}
                          </Badge>
                        )}
                      {card.isTop && (
                        <Badge className="bg-gradient-to-r from-orange-500 to-red-500 text-white text-xs border-0">
                          <Pin size={10} className="mr-1" />
                          置顶
                        </Badge>
                      )}
                    </div>
                  </div>
                  
                  <h3 className="font-medium text-lg mb-2">{card.title}</h3>
                  <p className="text-muted-foreground text-sm mb-3">{card.description}</p>
                  
                  {/* 标签区域 - 只在有标签时显示 */}
                  {(card.tags && card.tags.length > 0) || card.isHot ? (
                    <div className="flex flex-wrap gap-2 mb-3">
                      {card.tags && card.tags.slice(0, 3).map((tag, idx) => (
                        <Badge key={idx} variant="outline" className="text-xs">
                          {tag}
                        </Badge>
                      ))}
                      {card.tags && card.tags.length > 3 && (
                        <Badge variant="outline" className="text-xs text-muted-foreground">
                          +{card.tags.length - 3}
                        </Badge>
                      )}
                      {card.isHot && (
                        <Badge className="bg-amber-100 text-amber-700 border-amber-200 text-xs">
                          <Star size={12} className="mr-1" /> 精华
                        </Badge>
                      )}
                      </div>
                    ) : null}
                  </CardContent>
                  
                  <CardFooter className="p-4 pt-3 border-t">
                    <div className="flex items-center text-muted-foreground text-xs space-x-4">
                      <div className="flex items-center">
                        <Calendar size={14} className="mr-1" />
                        {formatDate(card.date)}
                      </div>
                      <div className="flex items-center">
                        <Eye size={14} className="mr-1" />
                        {formatNumber(card.views)}
                      </div>
                      <div className="flex items-center">
                        <Star size={14} className="mr-1" />
                        {formatNumber(card.likes)}
                      </div>
                      <div className="flex items-center">
                        <Download size={14} className="mr-1" />
                        {formatNumber(card.downloads || 0)}
                      </div>
                    </div>
                  </CardFooter>
                </Card>
              </motion.div>
            ))}
          </TabsContent>
        </Tabs>
      </div>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default HomeScreen