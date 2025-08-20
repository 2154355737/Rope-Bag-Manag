import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  ArrowLeft, Heart, MessageSquare, Share2, Bookmark, 
  MoreHorizontal, Flag, Hash, Eye, Calendar, CheckCircle
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'
import CommentSection, { Comment } from '@/components/comment-section'

const PostDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  
  // 模拟帖子数据
  const post = {
    id: parseInt(id || '1'),
    author: {
      name: '王五',
      avatar: 'https://i.pravatar.cc/150?img=3',
      verified: true,
    },
    content: '刚完成了一个结绳语言的移动应用项目，分享一些开发过程中的经验和踩过的坑。\n\n首先，结绳语言的异步处理机制非常强大，但需要注意内存管理问题。在开发过程中，我发现如果不正确处理异步任务的取消，很容易导致内存泄漏。\n\n其次，结绳语言的UI渲染性能优化有几个关键点：\n1. 减少不必要的重渲染\n2. 使用虚拟列表处理大量数据\n3. 图片懒加载和缓存\n\n最后，结绳语言的调试工具非常好用，强烈推荐大家尝试！',
    images: [
      'https://images.unsplash.com/photo-1551033406-611cf9a28f67?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTJ8fGNvZGluZ3xlbnwwfHwwfHx8MA%3D%3D',
      'https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8M3x8Y29kaW5nfGVufDB8fDB8fHww'
    ],
    code: `// 结绳语言异步任务示例
async function fetchData() {
  try {
    const response = await api.get('/data');
    return response.data;
  } catch (error) {
    console.error('Error:', error);
    return null;
  }
}`,
    tags: ['移动开发', '项目分享', '经验总结', '踩坑记录', '结绳实战'],
    likes: 78,
    comments: 23,
    views: 1250,
    time: '昨天 14:30',
    publishDate: '2025-01-14',
  }
  
  // 模拟评论数据
  const comments: Comment[] = [
    {
      id: 1,
      author: {
        name: '张三',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      content: '非常感谢分享！我也在学习结绳语言，这些经验对我很有帮助。',
      time: '昨天 15:20',
      likes: 12,
      isLiked: false,
      replies: [
        {
          id: 101,
          author: {
            name: '王五',
            avatar: 'https://i.pravatar.cc/150?img=3',
          },
          content: '不客气，希望对你有所帮助！',
          time: '昨天 15:45',
          likes: 3,
          isLiked: true,
        }
      ]
    },
    {
      id: 2,
      author: {
        name: '李四',
        avatar: 'https://i.pravatar.cc/150?img=2',
      },
      content: '关于内存泄漏的问题，你有没有更具体的解决方案？我的项目中也遇到了类似问题。',
      time: '昨天 16:10',
      likes: 8,
      isLiked: true,
      replies: []
    },
    {
      id: 3,
      author: {
        name: '赵六',
        avatar: 'https://i.pravatar.cc/150?img=4',
      },
      content: '结绳语言的调试工具确实很强大，特别是性能分析功能，帮我解决了很多优化问题。',
      time: '昨天 18:05',
      likes: 5,
      isLiked: false,
      replies: [
        {
          id: 301,
          author: {
            name: '王五',
            avatar: 'https://i.pravatar.cc/150?img=3',
          },
          content: '是的，性能分析工具非常好用，尤其是内存分析功能。',
          time: '昨天 18:30',
          likes: 2,
          isLiked: false,
        },
        {
          id: 302,
          author: {
            name: '张三',
            avatar: 'https://i.pravatar.cc/150?img=1',
          },
          content: '请问这个工具在哪里可以下载到？',
          time: '昨天 19:15',
          likes: 1,
          isLiked: false,
        }
      ]
    }
  ]

  // 评论区事件处理
  const handleSubmitComment = (content: string) => {
    console.log('新评论:', content)
    toast({
      title: "评论发送成功",
      description: "您的评论已发布"
    })
  }

  const handleSubmitReply = (commentId: number, content: string) => {
    console.log('回复评论:', commentId, content)
    toast({
      title: "回复发送成功",
      description: "您的回复已发布"
    })
  }

  const handleLikeComment = (commentId: number) => {
    console.log('点赞评论:', commentId)
    toast({
      title: "操作成功",
      description: "已点赞/取消点赞"
    })
  }

  const handleReportComment = (commentId: number) => {
    console.log('举报评论:', commentId)
  }

  // 格式化数字
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="帖子详情"
        showBackButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button variant="ghost" size="icon" className="h-9 w-9">
              <Share2 size={20} />
            </Button>
            <Button variant="ghost" size="icon" className="h-9 w-9">
              <MoreHorizontal size={20} />
            </Button>
          </div>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        <ScrollArea className="flex-1">
        <div className="p-4">
          {/* 帖子内容 */}
          <Card className="overflow-hidden">
            <CardContent className="p-4 min-w-0">
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <Avatar className="h-10 w-10 mr-2">
                    <AvatarImage src={post.author.avatar} />
                    <AvatarFallback>{post.author.name[0]}</AvatarFallback>
                  </Avatar>
                  <div>
                    <div className="flex items-center">
                      <span className="font-medium">{post.author.name}</span>
                      {post.author.verified && (
                        <CheckCircle size={14} className="ml-1 text-blue-500" />
                      )}
                    </div>
                    <div className="text-xs text-muted-foreground">{post.time}</div>
                  </div>
                </div>
                <Button variant="ghost" size="icon" className="h-8 w-8">
                  <MoreHorizontal size={16} />
                </Button>
              </div>
              
              <div className="space-y-4">
                {post.content.split('\n\n').map((paragraph, idx) => (
                  <p key={idx} className="text-sm">
                    {paragraph}
                  </p>
                ))}
              </div>
              
              {post.code && (
                <div className="bg-muted p-3 rounded-md my-4 overflow-hidden">
                  <div className="overflow-x-auto">
                    <pre className="text-xs whitespace-pre-wrap break-words min-w-0">
                      <code className="block break-words">{post.code}</code>
                    </pre>
                  </div>
                </div>
              )}
              
              {post.images && post.images.length > 0 && (
                <div className="my-4 w-full overflow-hidden">
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
              
              <div className="flex flex-wrap gap-1 mt-4 w-full overflow-hidden">
                {post.tags.map((tag, idx) => (
                  <Badge key={idx} variant="outline" className="text-xs flex items-center shrink-0">
                    <Hash size={10} className="mr-1" /> 
                    <span className="truncate max-w-20">{tag}</span>
                  </Badge>
                ))}
              </div>
            </CardContent>
            
            <CardFooter className="p-4 pt-3 border-t">
              <div className="flex items-center text-muted-foreground text-xs space-x-4">
                <div className="flex items-center">
                  <Calendar size={14} className="mr-1" />
                  {post.publishDate}
                </div>
                <div className="flex items-center">
                  <Eye size={14} className="mr-1" />
                  {formatNumber(post.views)}
                </div>
                <div className="flex items-center">
                  <Heart size={14} className="mr-1" />
                  {formatNumber(post.likes)}
                </div>
                <div className="flex items-center">
                  <MessageSquare size={14} className="mr-1" />
                  {formatNumber(post.comments)}
                </div>
              </div>
            </CardFooter>
          </Card>

          {/* 操作按钮 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              <div className="grid grid-cols-4 gap-2">
                <Button variant="ghost" size="sm" className="flex flex-col items-center p-2">
                  <Heart size={18} className="mb-1" />
                  <span className="text-xs">点赞</span>
                </Button>
                <Button variant="ghost" size="sm" className="flex flex-col items-center p-2">
                  <Share2 size={18} className="mb-1" />
                  <span className="text-xs">分享</span>
                </Button>
                <Button variant="ghost" size="sm" className="flex flex-col items-center p-2">
                  <Bookmark size={18} className="mb-1" />
                  <span className="text-xs">收藏</span>
                </Button>
                <Button variant="ghost" size="sm" className="flex flex-col items-center p-2">
                  <Flag size={18} className="mb-1" />
                  <span className="text-xs">举报</span>
                </Button>
              </div>
            </CardContent>
          </Card>

          {/* 评论区 */}
          <CommentSection
            comments={comments}
            totalCount={post.comments}
            onSubmitComment={handleSubmitComment}
            onSubmitReply={handleSubmitReply}
            onLikeComment={handleLikeComment}
            onReportComment={handleReportComment}
            placeholder="发表评论..."
            maxLength={200}
            className="mt-4"
          />
        </div>
      </ScrollArea>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default PostDetailScreen