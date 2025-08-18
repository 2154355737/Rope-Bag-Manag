import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { Search, Bell, Code, BookOpen, Zap, Star, Clock, Bookmark, Pin, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

const HomeScreen: React.FC = () => {
  const [searchFocused, setSearchFocused] = useState(false)
  const [searchValue, setSearchValue] = useState('')
  
  const categories = [
    { icon: Code, label: '基础语法', color: 'bg-blue-100 dark:bg-blue-900' },
    { icon: BookOpen, label: '学习资源', color: 'bg-green-100 dark:bg-green-900' },
    { icon: Zap, label: '实战项目', color: 'bg-yellow-100 dark:bg-yellow-900' },
    { icon: Star, label: '精选内容', color: 'bg-purple-100 dark:bg-purple-900' },
    { icon: Clock, label: '最近更新', color: 'bg-red-100 dark:bg-red-900' },
    { icon: Bookmark, label: '我的收藏', color: 'bg-indigo-100 dark:bg-indigo-900' },
  ]

  const resourceCards = [
    {
      id: 1,
      title: '结绳语言入门指南 2025版',
      description: '从零开始学习结绳语言，掌握核心语法和基本概念',
      tags: ['入门', '教程', '2025'],
      author: {
        name: '张教授',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      likes: 328,
      comments: 42,
      isTop: true,
      isHot: true,
    },
    {
      id: 2,
      title: '结绳高级特性详解',
      description: '深入探讨结绳语言的高级特性和优化技巧',
      tags: ['高级', '性能优化'],
      author: {
        name: '李开发',
        avatar: 'https://i.pravatar.cc/150?img=2',
      },
      likes: 156,
      comments: 23,
      isTop: false,
      isHot: true,
    },
    {
      id: 3,
      title: '结绳实战：构建移动应用',
      description: '使用结绳语言开发跨平台移动应用的完整教程',
      tags: ['实战', '移动开发'],
      author: {
        name: '王工程师',
        avatar: 'https://i.pravatar.cc/150?img=3',
      },
      likes: 89,
      comments: 15,
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
            placeholder="搜索资源、话题、用户..."
            className="pl-10 pr-4 py-6 rounded-full"
            value={searchValue}
            onChange={(e) => setSearchValue(e.target.value)}
            onFocus={() => setSearchFocused(true)}
            onBlur={(e) => {
              // 延迟关闭，允许点击下拉框内容
              setTimeout(() => {
                if (!e.currentTarget.contains(document.activeElement)) {
                  setSearchFocused(false)
                }
              }, 150)
            }}
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
              }}
            >
              <X size={16} />
            </Button>
          )}
        </div>
        
        {searchFocused && (
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="absolute left-4 right-4 top-full mt-2 bg-background border rounded-lg shadow-lg p-4 z-20 max-h-80 overflow-y-auto"
            onMouseDown={(e) => e.preventDefault()} // 防止点击时输入框失焦
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
                    }}
                  >
                    <Clock size={14} className="mr-2" />
                    <span>{history}</span>
                  </div>
                ))}
              </div>
            </div>
          </motion.div>
        )}
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
        <Tabs defaultValue="recommend" className="w-full">
          <TabsList className="grid grid-cols-3 mb-4">
            <TabsTrigger value="recommend">推荐</TabsTrigger>
            <TabsTrigger value="latest">最新</TabsTrigger>
            <TabsTrigger value="following">关注</TabsTrigger>
          </TabsList>
          
          <TabsContent value="recommend" className="space-y-4">
            {resourceCards
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
                <Card className={`overflow-hidden relative ${card.isTop ? 'ring-2 ring-orange-200 dark:ring-orange-800 ring-opacity-50' : ''}`}>
                  {card.isTop && (
                    <div className="bg-gradient-to-r from-orange-500 to-red-500 text-white text-xs py-1.5 px-3 absolute top-0 right-0 z-10 rounded-bl-md shadow-md flex items-center gap-1">
                      <Pin size={12} />
                      置顶
                    </div>
                  )}
                  
                  <CardContent className="p-4">
                    <div className="flex items-center mb-3">
                      <Avatar className="h-6 w-6 mr-2">
                        <AvatarImage src={card.author.avatar} />
                        <AvatarFallback>{card.author.name[0]}</AvatarFallback>
                      </Avatar>
                      <span className="text-sm">{card.author.name}</span>
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
                  
                  <CardFooter className="p-4 pt-0 border-t flex justify-between">
                    <div className="flex items-center text-muted-foreground text-sm">
                      <Button variant="ghost" size="sm" className="text-muted-foreground">
                        <Star size={16} className="mr-1" /> {card.likes}
                      </Button>
                      <Button variant="ghost" size="sm" className="text-muted-foreground">
                        <Bell size={16} className="mr-1" /> {card.comments}
                      </Button>
                    </div>
                    <Button variant="ghost" size="sm" className="text-muted-foreground">
                      <Bookmark size={16} />
                    </Button>
                  </CardFooter>
                </Card>
              </motion.div>
            ))}
          </TabsContent>
          
          <TabsContent value="latest">
            <div className="flex items-center justify-center h-40 text-muted-foreground">
              最新内容加载中...
            </div>
          </TabsContent>
          
          <TabsContent value="following">
            <div className="flex items-center justify-center h-40 text-muted-foreground">
              关注内容加载中...
            </div>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}

export default HomeScreen