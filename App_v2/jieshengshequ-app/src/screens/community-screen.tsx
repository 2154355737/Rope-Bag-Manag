import React, { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { Search, Hash, Heart, MessageSquare, Share2, Bookmark, MoreHorizontal, Star, Pin } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { ScrollArea } from '@/components/ui/scroll-area'
import TopNavigation from '@/components/ui/top-navigation'
import { useNavigation } from '@/contexts/NavigationContext'
import { fetchFeed } from '../api/feed'

const CommunityScreen: React.FC = () => {
  const { getActiveTab, setActiveTab } = useNavigation()
  const [activeTag, setActiveTag] = useState('all')
  
  // 获取当前活跃的标签页
  const activeTab = getActiveTab('community', 'all')
  
  const tags = [
    { id: 'all', name: '全部' },
    { id: 'trending', name: '热门' },
    { id: 'beginner', name: '新手问答' },
    { id: 'project', name: '项目展示' },
    { id: 'tips', name: '技巧分享' },
    { id: 'career', name: '职场交流' },
  ]
  
  const [posts, setPosts] = useState<any[]>([])
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    const load = async () => {
      try {
        setLoading(true)
        const res = await fetchFeed({ page: 1, pageSize: 10 })
        const mapped = (res.items || []).map((i: any) => ({
          id: i.id,
          author: { name: i.author?.name || '用户', avatar: i.author?.avatar || '' },
          content: i.title,
          images: [],
          tags: i.tags || [],
          likes: i.stats?.likes || 0,
          comments: i.stats?.comments || 0,
          time: new Date(i.created_at || i.publishedAt).toLocaleString('zh-CN'),
          isTop: i.is_pinned || false,
          isHot: i.is_featured || false,
        }))
        setPosts(mapped)
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [])

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="社区"
        subtitle="与开发者交流分享"
        showSearchButton
        showNotificationButton
        notificationCount={2}
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        {/* 标签导航 */}
        <div className="border-b">
        <ScrollArea className="whitespace-nowrap">
          <div className="flex p-2">
            {tags.map((tag) => (
              <Button
                key={tag.id}
                variant={activeTag === tag.id ? "default" : "ghost"}
                className="rounded-full text-sm px-4 mr-2"
                onClick={() => setActiveTag(tag.id)}
              >
                {tag.name}
              </Button>
            ))}
          </div>
        </ScrollArea>
      </div>

      {/* 内容标签页 */}
      <div className="p-4 flex-1">
        <Tabs value={activeTab} onValueChange={(value) => setActiveTab('community', value)} className="w-full">
          <TabsList className="grid grid-cols-3 mb-4">
            <TabsTrigger value="all">全部</TabsTrigger>
            <TabsTrigger value="following">关注</TabsTrigger>
            <TabsTrigger value="hot">热门</TabsTrigger>
          </TabsList>
          
          <TabsContent value="all" className="space-y-4 mt-0">
            {posts
              .sort((a, b) => {
                // 精华在最前面
                if (a.isHot && !b.isHot) return -1
                if (!a.isHot && b.isHot) return 1
                // 置顶在精华之后
                if (a.isTop && !b.isTop) return -1
                if (!a.isTop && b.isTop) return 1
                // 其他按时间降序
                return new Date(b.time).getTime() - new Date(a.time).getTime()
              })
              .map((post, index) => (
              <motion.div
                key={post.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3, delay: index * 0.1 }}
              >
                <Card className="overflow-hidden hover:shadow-md transition-all duration-200 border-l-4 border-l-transparent hover:border-l-primary">
                  <CardContent className="p-4 min-w-0">
                    {/* 头部信息 - 优化布局 */}
                    <div className="flex items-start justify-between mb-3">
                      <div className="flex items-center min-w-0 flex-1">
                        <Avatar className="h-8 w-8 mr-3 flex-shrink-0">
                          <AvatarImage src={post.author.avatar} />
                          <AvatarFallback>{post.author.name[0]}</AvatarFallback>
                        </Avatar>
                        <div className="min-w-0 flex-1">
                          <div className="flex items-center gap-2">
                            <span className="font-medium text-sm truncate">{post.author.name}</span>
                                                  {post.isHot && (
                        <Badge className="bg-amber-100 text-amber-700 border-amber-200 text-xs px-1.5 py-0.5">
                          <Star size={8} className="mr-1" />
                          精华
                        </Badge>
                      )}
                            {post.isTop && (
                              <Badge className="bg-gradient-to-r from-orange-500 to-red-500 text-white text-xs border-0 px-1.5 py-0.5">
                                <Pin size={8} className="mr-1" />
                                置顶
                              </Badge>
                            )}
                          </div>
                          <div className="text-xs text-muted-foreground">{post.time}</div>
                        </div>
                      </div>
                      <Button variant="ghost" size="icon" className="h-8 w-8 flex-shrink-0 ml-2">
                        <MoreHorizontal size={16} />
                      </Button>
                    </div>
                    
                    {/* 内容 - 改进排版 */}
                    <div className="space-y-3">
                      <p className="text-sm leading-relaxed line-clamp-3">{post.content}</p>
                      
                      {/* 代码块 */}
                      {post.code && (
                        <div className="bg-muted p-3 rounded-md overflow-hidden">
                          <div className="overflow-x-auto">
                            <pre className="text-xs whitespace-pre-wrap break-words min-w-0">
                              <code className="block break-words">{post.code}</code>
                            </pre>
                          </div>
                        </div>
                      )}
                      
                      {/* 图片 - 优化展示 */}
                      {post.images && post.images.length > 0 && (
                        <div className="w-full overflow-hidden">
                          <div className={`grid ${post.images.length > 1 ? 'grid-cols-2' : 'grid-cols-1'} gap-2 w-full`}>
                            {post.images.map((image, idx) => (
                              <div key={idx} className="min-w-0 overflow-hidden">
                                <img
                                  src={image}
                                  alt={`Post image ${idx + 1}`}
                                  className="rounded-md w-full h-32 object-cover"
                                />
                              </div>
                            ))}
                          </div>
                        </div>
                      )}
                      
                      {/* 标签 - 只在有标签时显示 */}
                      {post.tags && post.tags.length > 0 && (
                        <div className="flex flex-wrap gap-1.5">
                          {post.tags.slice(0, 4).map((tag, idx) => (
                            <Badge key={idx} variant="secondary" className="text-xs font-normal">
                              <Hash size={8} className="mr-1" /> 
                              <span className="truncate max-w-20">{tag}</span>
                            </Badge>
                          ))}
                          {post.tags.length > 4 && (
                            <Badge variant="outline" className="text-xs text-muted-foreground">
                              +{post.tags.length - 4}
                            </Badge>
                          )}
                        </div>
                      )}
                    </div>
                  </CardContent>
                  
                  {/* 底部交互区 - 紧凑设计 */}
                  <CardFooter className="px-4 py-2 border-t bg-muted/30 flex justify-between">
                    <div className="flex items-center gap-4 text-muted-foreground text-xs">
                      <div className="flex items-center">
                        <Heart size={14} className="mr-1" /> 
                        <span>{post.likes}</span>
                      </div>
                      <div className="flex items-center">
                        <MessageSquare size={14} className="mr-1" /> 
                        <span>{post.comments}</span>
                      </div>
                    </div>
                    <div className="flex items-center gap-1">
                      <Button variant="ghost" size="sm" className="h-7 px-2 text-muted-foreground hover:text-foreground">
                        <Share2 size={14} />
                      </Button>
                      <Button variant="ghost" size="sm" className="h-7 px-2 text-muted-foreground hover:text-foreground">
                        <Bookmark size={14} />
                      </Button>
                    </div>
                  </CardFooter>
                </Card>
              </motion.div>
            ))}
          </TabsContent>
          
          <TabsContent value="following">
            <div className="flex items-center justify-center h-40 text-muted-foreground">
              关注内容加载中...
            </div>
          </TabsContent>
          
          <TabsContent value="hot">
            <div className="flex items-center justify-center h-40 text-muted-foreground">
              热门内容加载中...
            </div>
          </TabsContent>
        </Tabs>
      </div>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default CommunityScreen