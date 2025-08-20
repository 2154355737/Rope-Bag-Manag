import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { Search, Hash, Heart, MessageSquare, Share2, Bookmark, MoreHorizontal } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { ScrollArea } from '@/components/ui/scroll-area'
import TopNavigation from '@/components/ui/top-navigation'
import { useNavigation } from '@/contexts/NavigationContext'

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
  
  const posts = [
    {
      id: 1,
      author: {
        name: '张三',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      content: '分享一个我用结绳语言写的小工具，可以自动整理代码格式，提高可读性。',
      images: ['https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8Y29kaW5nfGVufDB8fDB8fHww'],
      tags: ['工具', '代码格式化', '效率'],
      likes: 42,
      comments: 8,
      time: '2小时前',
    },
    {
      id: 2,
      author: {
        name: '李四',
        avatar: 'https://i.pravatar.cc/150?img=2',
      },
      content: '结绳语言的异步处理机制真的很强大，分享一下我的学习心得和实践经验。',
      images: [],
      code: `async function fetchData() {
  try {
    const response = await api.get('/data');
    return response.data;
  } catch (error) {
    console.error('Error fetching data:', error);
    return null;
  }
}`,
      tags: ['异步编程', '学习笔记'],
      likes: 36,
      comments: 15,
      time: '5小时前',
    },
    {
      id: 3,
      author: {
        name: '王五',
        avatar: 'https://i.pravatar.cc/150?img=3',
      },
      content: '刚完成了一个结绳语言的移动应用项目，分享一些开发过程中的经验和踩过的坑。',
      images: [
        'https://images.unsplash.com/photo-1551033406-611cf9a28f67?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTJ8fGNvZGluZ3xlbnwwfHwwfHx8MA%3D%3D',
        'https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8M3x8Y29kaW5nfGVufDB8fDB8fHww'
      ],
      tags: ['移动开发', '项目分享', '经验总结', '踩坑记录', '结绳实战'],
      likes: 78,
      comments: 23,
      time: '昨天',
    },
    {
      id: 4,
      author: {
        name: '赵六',
        avatar: 'https://i.pravatar.cc/150?img=4',
      },
      content: '请教一个结绳语言中关于内存管理的问题，为什么我的程序在运行一段时间后会出现内存泄漏？',
      images: [],
      tags: ['问题求助', '内存管理', '性能优化'],
      likes: 12,
      comments: 31,
      time: '昨天',
    },
  ]

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
            {posts.map((post, index) => (
              <motion.div
                key={post.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3, delay: index * 0.1 }}
              >
                <Card className="overflow-hidden">
                  <CardContent className="p-4 min-w-0">
                    <div className="flex items-center justify-between mb-3">
                      <div className="flex items-center">
                        <Avatar className="h-8 w-8 mr-2">
                          <AvatarImage src={post.author.avatar} />
                          <AvatarFallback>{post.author.name[0]}</AvatarFallback>
                        </Avatar>
                        <div>
                          <div className="font-medium text-sm">{post.author.name}</div>
                          <div className="text-xs text-muted-foreground">{post.time}</div>
                        </div>
                      </div>
                      <Button variant="ghost" size="icon" className="h-8 w-8">
                        <MoreHorizontal size={16} />
                      </Button>
                    </div>
                    
                    <p className="text-sm mb-3">{post.content}</p>
                    
                    {post.code && (
                      <div className="bg-muted p-3 rounded-md mb-3 overflow-hidden">
                        <div className="overflow-x-auto">
                          <pre className="text-xs whitespace-pre-wrap break-words min-w-0">
                            <code className="block break-words">{post.code}</code>
                          </pre>
                        </div>
                      </div>
                    )}
                    
                    {post.images && post.images.length > 0 && (
                      <div className="mb-3 w-full overflow-hidden">
                        <div className={`grid ${post.images.length > 1 ? 'grid-cols-2' : 'grid-cols-1'} gap-2 w-full`}>
                          {post.images.map((image, idx) => (
                            <div key={idx} className="min-w-0 overflow-hidden">
                              <img
                                src={image}
                                alt={`Post image ${idx + 1}`}
                                className="rounded-md w-full h-40 object-cover"
                              />
                            </div>
                          ))}
                        </div>
                      </div>
                    )}
                    
                    <div className="flex flex-wrap gap-1 mb-1 w-full overflow-hidden">
                      {post.tags.map((tag, idx) => (
                        <Badge key={idx} variant="outline" className="text-xs flex items-center shrink-0">
                          <Hash size={10} className="mr-1" /> 
                          <span className="truncate max-w-16">{tag}</span>
                        </Badge>
                      ))}
                    </div>
                  </CardContent>
                  
                  <CardFooter className="p-2 pt-0 border-t flex justify-between">
                    <Button variant="ghost" size="sm" className="text-muted-foreground flex-1">
                      <Heart size={16} className="mr-1" /> {post.likes}
                    </Button>
                    <Button variant="ghost" size="sm" className="text-muted-foreground flex-1">
                      <MessageSquare size={16} className="mr-1" /> {post.comments}
                    </Button>
                    <Button variant="ghost" size="sm" className="text-muted-foreground flex-1">
                      <Share2 size={16} className="mr-1" />
                    </Button>
                    <Button variant="ghost" size="sm" className="text-muted-foreground flex-1">
                      <Bookmark size={16} className="mr-1" />
                    </Button>
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