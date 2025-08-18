import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  ArrowLeft, Heart, MessageSquare, Share2, Bookmark, 
  MoreHorizontal, Send, Flag, Hash, ThumbsUp, ThumbsDown,
  Eye, Calendar, CheckCircle
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import { ScrollArea } from '@/components/ui/scroll-area'

const PostDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [commentText, setCommentText] = useState('')
  const [showReplyInput, setShowReplyInput] = useState<number | null>(null)
  const [replyText, setReplyText] = useState('')
  
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
  const comments = [
    {
      id: 1,
      author: {
        name: '张三',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      content: '非常感谢分享！我也在学习结绳语言，这些经验对我很有帮助。',
      time: '昨天 15:20',
      likes: 12,
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
        }
      ]
    }
  ]

  const handleSubmitComment = () => {
    if (commentText.trim()) {
      // 实际应用中应该调用API提交评论
      alert('评论已提交：' + commentText)
      setCommentText('')
    }
  }

  const handleSubmitReply = (commentId: number) => {
    if (replyText.trim()) {
      // 实际应用中应该调用API提交回复
      alert(`回复已提交 (评论ID: ${commentId}): ${replyText}`)
      setReplyText('')
      setShowReplyInput(null)
    }
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
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <Button variant="ghost" size="icon" onClick={() => navigate(-1)}>
            <ArrowLeft size={20} />
          </Button>
          
          <h1 className="text-lg font-medium">帖子详情</h1>
          
          <Button variant="ghost" size="icon">
            <MoreHorizontal size={20} />
          </Button>
        </div>
      </header>

      <ScrollArea className="flex-1">
        <div className="p-4">
          {/* 帖子内容 */}
          <Card>
            <CardContent className="p-4">
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
                <div className="bg-muted p-3 rounded-md my-4 overflow-x-auto">
                  <pre className="text-xs">
                    <code>{post.code}</code>
                  </pre>
                </div>
              )}
              
              {post.images && post.images.length > 0 && (
                <div className={`grid ${post.images.length > 1 ? 'grid-cols-2' : 'grid-cols-1'} gap-2 my-4`}>
                  {post.images.map((image, idx) => (
                    <img
                      key={idx}
                      src={image}
                      alt={`Post image ${idx + 1}`}
                      className="rounded-md w-full h-40 object-cover"
                    />
                  ))}
                </div>
              )}
              
              <div className="flex flex-wrap gap-1 mt-4">
                {post.tags.map((tag, idx) => (
                  <Badge key={idx} variant="outline" className="text-xs flex items-center">
                    <Hash size={10} className="mr-1" /> {tag}
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
          <div className="mt-4">
            <h2 className="text-lg font-medium mb-4">评论 ({comments.length})</h2>
            
            <div className="space-y-4">
              {comments.map((comment) => (
                <motion.div
                  key={comment.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.3 }}
                >
                  <Card>
                    <CardContent className="p-3">
                      <div className="flex items-start">
                        <Avatar className="h-8 w-8 mr-2">
                          <AvatarImage src={comment.author.avatar} />
                          <AvatarFallback>{comment.author.name[0]}</AvatarFallback>
                        </Avatar>
                        <div className="flex-1">
                          <div className="flex items-center justify-between">
                            <div className="font-medium text-sm">{comment.author.name}</div>
                            <div className="text-xs text-muted-foreground">{comment.time}</div>
                          </div>
                          <p className="text-sm mt-1">{comment.content}</p>
                          
                          <div className="flex items-center mt-2">
                            <Button variant="ghost" size="sm" className="h-6 text-xs text-muted-foreground">
                              <ThumbsUp size={12} className="mr-1" /> {comment.likes}
                            </Button>
                            <Button 
                              variant="ghost" 
                              size="sm" 
                              className="h-6 text-xs text-muted-foreground"
                              onClick={() => setShowReplyInput(comment.id)}
                            >
                              <MessageSquare size={12} className="mr-1" /> 回复
                            </Button>
                            <Button variant="ghost" size="sm" className="h-6 text-xs text-muted-foreground">
                              <Flag size={12} className="mr-1" /> 举报
                            </Button>
                          </div>
                          
                          {showReplyInput === comment.id && (
                            <div className="flex items-center mt-2">
                              <Input
                                id={`reply-input-${comment.id}`}
                                name={`reply-${comment.id}`}
                                placeholder="回复评论..."
                                className="text-xs h-8 mr-2"
                                value={replyText}
                                onChange={(e) => setReplyText(e.target.value)}
                                maxLength={200}
                                autoComplete="off"
                              />
                              <Button 
                                size="sm" 
                                className="h-8"
                                onClick={() => handleSubmitReply(comment.id)}
                              >
                                发送
                              </Button>
                            </div>
                          )}
                          
                          {/* 回复列表 */}
                          {comment.replies.length > 0 && (
                            <div className="mt-2 pl-4 border-l-2 border-muted space-y-2">
                              {comment.replies.map((reply) => (
                                <div key={reply.id} className="mt-2">
                                  <div className="flex items-start">
                                    <Avatar className="h-6 w-6 mr-2">
                                      <AvatarImage src={reply.author.avatar} />
                                      <AvatarFallback>{reply.author.name[0]}</AvatarFallback>
                                    </Avatar>
                                    <div className="flex-1">
                                      <div className="flex items-center justify-between">
                                        <div className="font-medium text-xs">{reply.author.name}</div>
                                        <div className="text-xs text-muted-foreground">{reply.time}</div>
                                      </div>
                                      <p className="text-xs mt-1">{reply.content}</p>
                                      
                                      <div className="flex items-center mt-1">
                                        <Button variant="ghost" size="sm" className="h-5 text-xs text-muted-foreground">
                                          <ThumbsUp size={10} className="mr-1" /> {reply.likes}
                                        </Button>
                                        <Button variant="ghost" size="sm" className="h-5 text-xs text-muted-foreground">
                                          <Flag size={10} className="mr-1" /> 举报
                                        </Button>
                                      </div>
                                    </div>
                                  </div>
                                </div>
                              ))}
                            </div>
                          )}
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
          </div>

          {/* 评论输入框 */}
          <Card className="mt-4">
            <CardContent className="p-4">
              <div className="flex items-center gap-3">
                <Input
                  id="comment-input"
                  name="comment"
                  placeholder="发表评论..."
                  className="flex-1"
                  value={commentText}
                  onChange={(e) => setCommentText(e.target.value)}
                  maxLength={200}
                  autoComplete="off"
                />
                <Button 
                  disabled={!commentText.trim()}
                  onClick={handleSubmitComment}
                >
                  <Send size={16} className="mr-1" />
                  发送
                </Button>
              </div>
              <div className="text-xs text-muted-foreground mt-2">
                {commentText.length}/200 字符
              </div>
            </CardContent>
          </Card>
        </div>
      </ScrollArea>
    </div>
  )
}

export default PostDetailScreen