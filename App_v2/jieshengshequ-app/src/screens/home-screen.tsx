import React, { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Search, Bell, Code, BookOpen, Zap, Star, Clock, Bookmark, Pin, X, Eye, Download, Calendar } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

const HomeScreen: React.FC = () => {
  const navigate = useNavigate()
  const [searchFocused, setSearchFocused] = useState(false)
  const [searchValue, setSearchValue] = useState('')
  const [isDropdownInteracting, setIsDropdownInteracting] = useState(false)

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
  
  const categories = [
    { icon: Code, label: '基础语法', color: 'bg-blue-100 dark:bg-blue-900' },
    { icon: BookOpen, label: '学习资源', color: 'bg-green-100 dark:bg-green-900' },
    { icon: Zap, label: '实战项目', color: 'bg-yellow-100 dark:bg-yellow-900' },
    { icon: Star, label: '精选内容', color: 'bg-purple-100 dark:bg-purple-900' },
    { icon: Clock, label: '最近更新', color: 'bg-red-100 dark:bg-red-900' },
    { icon: Bookmark, label: '我的收藏', color: 'bg-indigo-100 dark:bg-indigo-900' },
  ]

  const allContent = [
    // 公告
    {
      id: 1,
      type: 'announcement',
      title: '🎉 结绳社区2025年新春活动开始啦！',
      description: '参与社区活动，赢取丰厚奖品！分享你的学习心得，获得积分奖励。',
      tags: ['公告', '活动', '2025'],
      author: {
        name: '社区管理员',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      likes: 528,
      comments: 89,
      views: 1250,
      date: '2025-01-15',
      isTop: true,
      isHot: true,
    },
    // 帖子
    {
      id: 2,
      type: 'post',
      title: '结绳语言学习心得分享',
      description: '从零基础到熟练掌握，我的结绳语言学习之路总结',
      tags: ['学习心得', '经验分享'],
      author: {
        name: '张同学',
        avatar: 'https://i.pravatar.cc/150?img=2',
      },
      likes: 156,
      comments: 23,
      views: 890,
      date: '2025-01-14',
      isTop: false,
      isHot: true,
    },
    {
      id: 3,
      type: 'post',
      title: '如何优雅地处理结绳语言中的异步操作',
      description: '深入探讨异步编程的最佳实践和常见陷阱',
      tags: ['异步编程', '最佳实践'],
      author: {
        name: '李开发',
        avatar: 'https://i.pravatar.cc/150?img=3',
      },
      likes: 89,
      comments: 15,
      views: 456,
      date: '2025-01-13',
      isTop: false,
      isHot: false,
    },
    // 资源
    {
      id: 4,
      type: 'resource',
      title: '结绳语言完整开发工具包 v2.1',
      description: '包含编译器、调试器、代码格式化工具等完整开发环境',
      tags: ['开发工具', 'v2.1', '编译器'],
      author: {
        name: '工具开发组',
        avatar: 'https://i.pravatar.cc/150?img=4',
      },
      likes: 342,
      comments: 67,
      views: 2340,
      downloads: 856,
      date: '2025-01-12',
      isTop: false,
      isHot: true,
    },
    {
      id: 5,
      type: 'resource',
      title: '结绳语言标准库文档 PDF版',
      description: '官方标准库完整文档，支持离线阅读，包含所有API说明',
      tags: ['文档', 'PDF', '标准库'],
      author: {
        name: '文档团队',
        avatar: 'https://i.pravatar.cc/150?img=5',
      },
      likes: 198,
      comments: 34,
      views: 1120,
      downloads: 324,
      date: '2025-01-10',
      isTop: false,
      isHot: false,
    },
  ]

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Code size={24} className="text-primary mr-2" />
            <h1 className="text-xl font-bold">结绳社区</h1>
          </div>
          
          <div className="flex items-center">
            <Button variant="ghost" size="icon">
              <Bell size={20} />
            </Button>
          </div>
        </div>
      </header>

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
              className="absolute left-0 right-0 top-full mt-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg p-4 z-50 max-h-80 overflow-y-auto"
              onMouseEnter={() => setIsDropdownInteracting(true)}
              onMouseLeave={() => setIsDropdownInteracting(false)}
            >
            <div className="mb-4">
              <h3 className="text-sm font-medium mb-2">热门搜索</h3>
              <div className="flex flex-wrap gap-2">
                {['结绳入门', '数据结构', '项目实战', '性能优化', '面试题'].map((tag) => (
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
                {['结绳语言基础教程', '如何优化结绳代码'].map((history) => (
                  <div 
                    key={history}
                    className="flex items-center cursor-pointer hover:bg-accent rounded-md p-2 -m-2 transition-colors"
                    onClick={() => {
                      setSearchValue(history)
                      setSearchFocused(false)
                      setIsDropdownInteracting(false)
                    }}
                  >
                    <Clock size={14} className="mr-2" />
                    <span>{history}</span>
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
              <span className="text-xs text-center">{category.label}</span>
            </motion.div>
          ))}
        </div>
      </div>

      {/* 内容标签页 */}
      <div className="px-4 flex-1">
        <Tabs defaultValue="home" className="w-full">
          <TabsList className="grid grid-cols-3 mb-4">
            <TabsTrigger value="posts">帖子</TabsTrigger>
            <TabsTrigger value="home">首页</TabsTrigger>
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
                        <Badge variant="outline" className="text-xs">
                          帖子
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
                    
                    <div className="flex flex-wrap gap-2 mb-3">
                      {card.tags.map((tag, idx) => (
                        <Badge key={idx} variant="outline" className="text-xs">
                          {tag}
                        </Badge>
                      ))}
                      {card.isHot && (
                        <Badge variant="secondary" className="text-xs">
                          <Zap size={12} className="mr-1" /> 热门
                        </Badge>
                      )}
                    </div>
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
          
          <TabsContent value="home" className="space-y-4">
            {allContent
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
                          {card.type === 'announcement' ? '公告' : card.type === 'post' ? '帖子' : '资源'}
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
                    
                    <div className="flex flex-wrap gap-2 mb-3">
                      {card.tags.map((tag, idx) => (
                        <Badge key={idx} variant="outline" className="text-xs">
                          {tag}
                        </Badge>
                      ))}
                      {card.isHot && (
                        <Badge variant="secondary" className="text-xs">
                          <Zap size={12} className="mr-1" /> 热门
                        </Badge>
                      )}
                    </div>
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
                        <Badge variant="secondary" className="text-xs">
                          资源
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
                    
                    <div className="flex flex-wrap gap-2 mb-3">
                      {card.tags.map((tag, idx) => (
                        <Badge key={idx} variant="outline" className="text-xs">
                          {tag}
                        </Badge>
                      ))}
                      {card.isHot && (
                        <Badge variant="secondary" className="text-xs">
                          <Zap size={12} className="mr-1" /> 热门
                        </Badge>
                      )}
                    </div>
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
    </div>
  )
}

export default HomeScreen