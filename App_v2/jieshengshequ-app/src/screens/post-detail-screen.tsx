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
import RelatedRecommendations from '@/components/related-recommendations'
import InteractionButtons, { 
  createLikeButton, 
  createBookmarkButton, 
  createShareButton, 
  createReportButton 
} from '@/components/ui/interaction-buttons'
import { getPostRecommendations } from '@/utils/recommendations'

const PostDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [isLoadingComments, setIsLoadingComments] = useState(false)
  const [allComments, setAllComments] = useState<Comment[]>([])
  const [hasMoreComments, setHasMoreComments] = useState(true)
  const [isLiked, setIsLiked] = useState(false)
  const [isBookmarked, setIsBookmarked] = useState(false)
  
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
    comments: 156, // 增加评论总数以演示分页
    views: 1250,
    time: '昨天 14:30',
    publishDate: '2025-01-14',
  }
  
  // 模拟初始评论数据（第一页）
  const initialComments: Comment[] = [
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
    },
    {
      id: 4,
      author: {
        name: '小明',
        avatar: 'https://i.pravatar.cc/150?img=5',
      },
      content: '这个教程写得很详细，我按照步骤实现了一个简单的应用，效果不错！',
      time: '今天 09:15',
      likes: 15,
      isLiked: false,
      replies: []
    },
    {
      id: 5,
      author: {
        name: '开发者小李',
        avatar: 'https://i.pravatar.cc/150?img=6',
      },
      content: '能否分享一下你在项目中使用的第三方库？我正在选择合适的组件库。',
      time: '今天 10:30',
      likes: 7,
      isLiked: true,
      replies: [
        {
          id: 501,
          author: {
            name: '王五',
            avatar: 'https://i.pravatar.cc/150?img=3',
          },
          content: '我主要使用了 UI Kit 和 Animation Library，都很好用。',
          time: '今天 11:00',
          likes: 4,
          isLiked: false,
        }
      ]
    },
    {
      id: 6,
      author: {
        name: '程序小白',
        avatar: 'https://i.pravatar.cc/150?img=7',
      },
      content: '刚开始学编程，这个教程对我来说有些深度，但还是学到了很多。希望有更多基础入门的内容。',
      time: '今天 12:15',
      likes: 9,
      isLiked: false,
      replies: []
    },
    {
      id: 7,
      author: {
        name: '资深开发者',
        avatar: 'https://i.pravatar.cc/150?img=8',
      },
      content: '作者提到的性能优化技巧很实用，特别是关于内存管理的部分。我在生产环境中也遇到过类似问题。',
      time: '今天 13:45',
      likes: 25,
      isLiked: true,
      replies: [
        {
          id: 701,
          author: {
            name: '王五',
            avatar: 'https://i.pravatar.cc/150?img=3',
          },
          content: '能分享一下你在生产环境中的具体解决方案吗？',
          time: '今天 14:00',
          likes: 6,
          isLiked: false,
        }
      ]
    },
    {
      id: 8,
      author: {
        name: '前端攻城狮',
        avatar: 'https://i.pravatar.cc/150?img=9',
      },
      content: '结绳语言在移动端的表现确实不错，我们团队也在考虑迁移到这个技术栈。请问有推荐的学习路径吗？',
      time: '今天 15:30',
      likes: 14,
      isLiked: false,
      replies: []
    },
    {
      id: 9,
      author: {
        name: '技术爱好者',
        avatar: 'https://i.pravatar.cc/150?img=10',
      },
      content: '感谢分享！特别是关于调试工具的介绍，之前一直不知道有这么好用的功能。',
      time: '今天 16:20',
      likes: 8,
      isLiked: true,
      replies: []
    },
    {
      id: 10,
      author: {
        name: '学习中的菜鸟',
        avatar: 'https://i.pravatar.cc/150?img=11',
      },
      content: '代码示例很清晰，我照着敲了一遍，确实帮助理解。希望能看到更多这样的实战案例。',
      time: '今天 17:10',
      likes: 11,
      isLiked: false,
      replies: [
        {
          id: 1001,
          author: {
            name: '张三',
            avatar: 'https://i.pravatar.cc/150?img=1',
          },
          content: '我也是这样学习的，实践出真知！',
          time: '今天 17:30',
          likes: 3,
          isLiked: false,
        }
      ]
    }
  ]

  // 模拟分页加载评论数据
  const generateMockComments = (page: number): Comment[] => {
    const comments: Comment[] = []
    const startId = (page - 1) * 10 + 100 // 从ID 100开始生成新评论
    
    for (let i = 0; i < 10; i++) {
      const commentId = startId + i
      const authorIndex = (commentId % 8) + 1
      
      comments.push({
        id: commentId,
        author: {
          name: `用户${commentId}`,
          avatar: `https://i.pravatar.cc/150?img=${authorIndex}`,
        },
        content: `这是第${page}页的第${i + 1}条评论。感谢作者的精彩分享，我从中学到了很多有用的知识和技巧！`,
        time: `${Math.floor(Math.random() * 24)}小时前`,
        likes: Math.floor(Math.random() * 20),
        isLiked: Math.random() > 0.7,
        replies: Math.random() > 0.8 ? [
          {
            id: commentId * 10 + 1,
            author: {
              name: `回复者${commentId}`,
              avatar: `https://i.pravatar.cc/150?img=${(authorIndex % 8) + 1}`,
            },
            content: `对评论${commentId}的回复，很有道理！`,
            time: `${Math.floor(Math.random() * 12)}小时前`,
            likes: Math.floor(Math.random() * 5),
            isLiked: Math.random() > 0.8,
          }
        ] : []
      })
    }
    
    return comments
  }

  // 获取相关推荐
  const recommendedItems = getPostRecommendations(post.id, post.tags)

  // 初始化评论数据
  React.useEffect(() => {
    console.log('初始化评论数据，数量:', initialComments.length)
    setAllComments(initialComments)
  }, [])

  // 模拟分页加载评论
  const handleLoadMoreComments = async (page: number): Promise<Comment[]> => {
    setIsLoadingComments(true)
    
    // 模拟网络延迟
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    const newComments = generateMockComments(page)
    
    // 模拟没有更多评论的情况（假设总共有15页）
    if (page >= 15) {
      setHasMoreComments(false)
    }
    
    setIsLoadingComments(false)
    return newComments
  }

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

  // 处理点赞
  const handleLike = () => {
    setIsLiked(!isLiked)
    toast({
      title: isLiked ? "已取消点赞" : "点赞成功",
      description: isLiked ? "已取消对此帖子的点赞" : "感谢您的支持",
      duration: 2000,
    })
  }

  // 处理收藏
  const handleBookmark = () => {
    setIsBookmarked(!isBookmarked)
    toast({
      title: isBookmarked ? "已取消收藏" : "收藏成功",
      description: isBookmarked ? "已从收藏夹中移除" : "已添加到您的收藏夹",
      duration: 2000,
    })
  }

  // 处理分享
  const handleShare = () => {
    toast({
      title: "分享链接已复制",
      description: "可以分享给更多朋友了",
      duration: 2000,
    })
  }

  // 处理举报
  const handleReport = () => {
    toast({
      title: "举报已提交",
      description: "我们会尽快处理您的举报",
      duration: 2000,
    })
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
            
            <CardFooter className="p-3 pt-2 border-t border-border/50">
              <div className="flex items-center text-muted-foreground text-[10px] space-x-3">
                <div className="flex items-center">
                  <Calendar size={10} className="mr-1" />
                  {post.publishDate}
                </div>
                <div className="flex items-center">
                  <Eye size={10} className="mr-1" />
                  {formatNumber(post.views)}
                </div>
                <div className="flex items-center">
                  <Heart size={10} className="mr-1" />
                  {formatNumber(post.likes)}
                </div>
                <div className="flex items-center">
                  <MessageSquare size={10} className="mr-1" />
                  {formatNumber(post.comments)}
                </div>
              </div>
            </CardFooter>
          </Card>

          {/* 操作按钮 */}
          <InteractionButtons
            buttons={[
              createLikeButton(post.likes + (isLiked ? 1 : 0), isLiked, handleLike),
              createShareButton(handleShare),
              createBookmarkButton(undefined, isBookmarked, handleBookmark),
              createReportButton(handleReport)
            ]}
            className="mb-4"
            compact={true}
          />

          {/* 相关推荐 */}
          <RelatedRecommendations
            title="相关推荐"
            items={recommendedItems}
            currentItemId={post.id}
            maxItems={6}
            className="mt-6"
            onMoreClick={() => navigate('/community')}
          />

          {/* 评论区 */}
          <CommentSection
            comments={allComments}
            totalCount={post.comments}
            onSubmitComment={handleSubmitComment}
            onSubmitReply={handleSubmitReply}
            onLikeComment={handleLikeComment}
            onReportComment={handleReportComment}
            onLoadMoreComments={handleLoadMoreComments}
            hasMoreComments={hasMoreComments}
            isLoadingComments={isLoadingComments}
            placeholder="发表评论..."
            maxLength={200}
            initialCommentsToShow={5}
            pageSize={10}
            className="mt-6"
          />
        </div>
      </ScrollArea>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default PostDetailScreen