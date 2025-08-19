import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  ArrowLeft, Download, Heart, MessageSquare, Share2, Bookmark, 
  MoreHorizontal, Send, Flag, Hash, ThumbsUp, ThumbsDown, 
  Star, FileText, Package, Shield, Calendar, Eye, AlertTriangle,
  ExternalLink, Copy, CheckCircle
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Progress } from '@/components/ui/progress'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Textarea } from '@/components/ui/textarea'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'

const ResourceDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [commentText, setCommentText] = useState('')
  const [showReviewForm, setShowReviewForm] = useState(false)
  const [reviewText, setReviewText] = useState('')
  const [reviewRating, setReviewRating] = useState(5)
  const [isDownloading, setIsDownloading] = useState(false)
  const [downloadProgress, setDownloadProgress] = useState(0)
  
  // 模拟资源数据
  const resource = {
    id: parseInt(id || '1'),
    title: 'React Native 开发工具包',
    version: 'v2.1.0',
    author: {
      name: '开发者社区',
      avatar: 'https://i.pravatar.cc/150?img=5',
      verified: true,
    },
    description: '一个功能强大的 React Native 开发工具包，包含常用组件、工具函数和最佳实践示例。帮助开发者快速构建高质量的移动应用。\n\n主要特性：\n• 丰富的UI组件库\n• 完整的导航解决方案\n• 网络请求封装\n• 状态管理工具\n• 性能优化工具\n• 完整的TypeScript支持',
    category: '开发工具',
    tags: ['React Native', 'TypeScript', '移动开发', '组件库', '工具包'],
    screenshots: [
      'https://images.unsplash.com/photo-1512941937669-90a1b58e7e9c?w=500&auto=format&fit=crop&q=60',
      'https://images.unsplash.com/photo-1551650975-87deedd944c3?w=500&auto=format&fit=crop&q=60',
      'https://images.unsplash.com/photo-1563013544-824ae1b704d3?w=500&auto=format&fit=crop&q=60'
    ],
    fileSize: '15.2 MB',
    downloadCount: 1250,
    likes: 89,
    views: 3420,
    rating: 4.8,
    reviewCount: 156,
    lastUpdated: '2025-01-10',
    publishDate: '2024-12-01',
    files: [
      { name: 'react-native-toolkit-v2.1.0.zip', size: '15.2 MB', type: 'ZIP' },
      { name: 'README.md', size: '12 KB', type: 'MD' },
      { name: 'CHANGELOG.md', size: '8 KB', type: 'MD' }
    ],
    requirements: [
      'React Native >= 0.70.0',
      'Node.js >= 16.0.0',
      'TypeScript >= 4.8.0'
    ],
    safetyStatus: 'verified'
  }
  
  // 模拟评价数据
  const reviews = [
    {
      id: 1,
      author: {
        name: '张开发',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      rating: 5,
      content: '非常棒的工具包！组件质量很高，文档也很详细。已经在我的项目中使用了，效果很好。',
      time: '2025-01-12',
      helpful: 23,
    },
    {
      id: 2,
      author: {
        name: '李前端',
        avatar: 'https://i.pravatar.cc/150?img=2',
      },
      rating: 4,
      content: '整体不错，但是某些组件的API设计还可以优化。希望下个版本能改进。',
      time: '2025-01-08',
      helpful: 12,
    }
  ]

  // 格式化文件大小
  const formatFileSize = (bytes: string) => {
    return bytes
  }

  // 格式化数字
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 处理下载
  const handleDownload = async () => {
    setIsDownloading(true)
    setDownloadProgress(0)
    
    // 模拟下载进度
    const interval = setInterval(() => {
      setDownloadProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval)
          setIsDownloading(false)
          toast({
            title: "下载完成",
            description: "资源已成功下载到您的设备",
            duration: 3000,
          })
          return 100
        }
        return prev + Math.random() * 15
      })
    }, 200)
  }

  // 提交评价
  const handleSubmitReview = () => {
    if (reviewText.trim()) {
      toast({
        title: "评价已提交",
        description: "感谢您的评价，审核通过后将显示",
        duration: 3000,
      })
      setReviewText('')
      setReviewRating(5)
      setShowReviewForm(false)
    }
  }

  // 提交评论
  const handleSubmitComment = () => {
    if (commentText.trim()) {
      toast({
        title: "评论已提交",
        description: "您的评论已提交成功",
        duration: 3000,
      })
      setCommentText('')
    }
  }

  // 渲染星级评分
  const renderStars = (rating: number, size = 16, interactive = false, onRate?: (rating: number) => void) => {
    return (
      <div className="flex items-center">
        {[1, 2, 3, 4, 5].map((star) => (
          <Star
            key={star}
            size={size}
            className={`${
              star <= rating 
                ? 'fill-yellow-400 text-yellow-400' 
                : 'text-gray-300'
            } ${interactive ? 'cursor-pointer hover:text-yellow-400' : ''}`}
            onClick={interactive && onRate ? () => onRate(star) : undefined}
          />
        ))}
      </div>
    )
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="资源详情"
        showBackButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button variant="ghost" size="icon" className="h-9 w-9">
              <Bookmark size={20} />
            </Button>
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
          {/* 资源基本信息 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center">
                  <Avatar className="h-12 w-12 mr-3">
                    <AvatarImage src={resource.author.avatar} />
                    <AvatarFallback>{resource.author.name[0]}</AvatarFallback>
                  </Avatar>
                  <div>
                    <div className="flex items-center">
                      <span className="font-medium">{resource.author.name}</span>
                      {resource.author.verified && (
                        <CheckCircle size={16} className="ml-1 text-blue-500" />
                      )}
                    </div>
                    <div className="text-xs text-muted-foreground">
                      发布于 {resource.publishDate}
                    </div>
                  </div>
                </div>
                <Badge variant="secondary" className="text-xs">
                  {resource.category}
                </Badge>
              </div>

              <h2 className="text-xl font-bold mb-2">{resource.title}</h2>
              <div className="flex items-center mb-3">
                <Badge variant="outline" className="text-xs mr-2">
                  {resource.version}
                </Badge>
                <div className="flex items-center mr-3">
                  {renderStars(Math.floor(resource.rating))}
                  <span className="text-sm text-muted-foreground ml-1">
                    {resource.rating} ({resource.reviewCount})
                  </span>
                </div>
              </div>

              {/* 统计信息 */}
              <div className="grid grid-cols-3 gap-4 mb-4">
                <div className="text-center">
                  <div className="text-lg font-bold">{formatNumber(resource.downloadCount)}</div>
                  <div className="text-xs text-muted-foreground">下载量</div>
                </div>
                <div className="text-center">
                  <div className="text-lg font-bold">{formatNumber(resource.views)}</div>
                  <div className="text-xs text-muted-foreground">浏览量</div>
                </div>
                <div className="text-center">
                  <div className="text-lg font-bold">{formatNumber(resource.likes)}</div>
                  <div className="text-xs text-muted-foreground">点赞数</div>
                </div>
              </div>

              {/* 安全状态 */}
              <Alert className="mb-4">
                <Shield size={16} />
                <AlertDescription>
                  此资源已通过安全检测，可放心下载使用
                </AlertDescription>
              </Alert>

              {/* 标签 */}
              <div className="flex flex-wrap gap-2 mb-4">
                {resource.tags.map((tag, idx) => (
                  <Badge key={idx} variant="outline" className="text-xs">
                    <Hash size={10} className="mr-1" /> {tag}
                  </Badge>
                ))}
              </div>
            </CardContent>
          </Card>

          {/* 截图展示 */}
          {resource.screenshots.length > 0 && (
            <Card className="mb-4">
              <CardHeader>
                <CardTitle className="text-lg">预览截图</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="grid grid-cols-1 gap-3">
                  {resource.screenshots.map((screenshot, idx) => (
                    <img
                      key={idx}
                      src={screenshot}
                      alt={`Screenshot ${idx + 1}`}
                      className="rounded-md w-full h-48 object-cover"
                    />
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 详细描述 */}
          <Card className="mb-4">
            <CardHeader>
              <CardTitle className="text-lg">详细介绍</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="space-y-3">
                {resource.description.split('\n\n').map((paragraph, idx) => (
                  <div key={idx}>
                    {paragraph.includes('•') ? (
                      <ul className="list-disc list-inside space-y-1 text-sm">
                        {paragraph.split('\n').map((line, lineIdx) => (
                          line.trim().startsWith('•') && (
                            <li key={lineIdx} className="ml-2">
                              {line.trim().substring(1).trim()}
                            </li>
                          )
                        ))}
                      </ul>
                    ) : (
                      <p className="text-sm">{paragraph}</p>
                    )}
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>

          {/* 系统要求 */}
          <Card className="mb-4">
            <CardHeader>
              <CardTitle className="text-lg">系统要求</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              <ul className="space-y-2">
                {resource.requirements.map((req, idx) => (
                  <li key={idx} className="flex items-center text-sm">
                    <CheckCircle size={14} className="text-green-500 mr-2" />
                    {req}
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          {/* 文件列表 */}
          <Card className="mb-4">
            <CardHeader>
              <CardTitle className="text-lg">包含文件</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="space-y-3">
                {resource.files.map((file, idx) => (
                  <div key={idx} className="flex items-center justify-between p-3 border rounded-md">
                    <div className="flex items-center">
                      <FileText size={16} className="text-muted-foreground mr-2" />
                      <div>
                        <div className="font-medium text-sm">{file.name}</div>
                        <div className="text-xs text-muted-foreground">{file.size}</div>
                      </div>
                    </div>
                    <Badge variant="secondary" className="text-xs">
                      {file.type}
                    </Badge>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>

          {/* 下载按钮 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              {isDownloading ? (
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-sm">下载中...</span>
                    <span className="text-sm">{Math.round(downloadProgress)}%</span>
                  </div>
                  <Progress value={downloadProgress} className="w-full" />
                </div>
              ) : (
                <Button 
                  className="w-full" 
                  size="lg"
                  onClick={handleDownload}
                >
                  <Download size={18} className="mr-2" />
                  免费下载 ({resource.fileSize})
                </Button>
              )}
            </CardContent>
          </Card>

          {/* 用户评价 */}
          <Card className="mb-4">
            <CardHeader className="flex flex-row items-center justify-between">
              <CardTitle className="text-lg">用户评价</CardTitle>
              <Button 
                variant="outline" 
                size="sm"
                onClick={() => setShowReviewForm(true)}
              >
                写评价
              </Button>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              {showReviewForm && (
                <div className="mb-4 p-4 border rounded-md">
                  <div className="mb-3">
                    <label className="text-sm font-medium mb-2 block">评分</label>
                    {renderStars(reviewRating, 20, true, setReviewRating)}
                  </div>
                  <Textarea
                    placeholder="分享您的使用体验..."
                    value={reviewText}
                    onChange={(e) => setReviewText(e.target.value)}
                    className="mb-3"
                    maxLength={500}
                  />
                  <div className="flex justify-end gap-2">
                    <Button 
                      variant="outline" 
                      size="sm"
                      onClick={() => setShowReviewForm(false)}
                    >
                      取消
                    </Button>
                    <Button 
                      size="sm"
                      onClick={handleSubmitReview}
                    >
                      提交评价
                    </Button>
                  </div>
                </div>
              )}

              <div className="space-y-4">
                {reviews.map((review) => (
                  <div key={review.id} className="border-b pb-4 last:border-b-0">
                    <div className="flex items-start">
                      <Avatar className="h-8 w-8 mr-2">
                        <AvatarImage src={review.author.avatar} />
                        <AvatarFallback>{review.author.name[0]}</AvatarFallback>
                      </Avatar>
                      <div className="flex-1">
                        <div className="flex items-center justify-between mb-1">
                          <div className="font-medium text-sm">{review.author.name}</div>
                          <div className="text-xs text-muted-foreground">{review.time}</div>
                        </div>
                        <div className="flex items-center mb-2">
                          {renderStars(review.rating, 14)}
                        </div>
                        <p className="text-sm mb-2">{review.content}</p>
                        <Button variant="ghost" size="sm" className="h-6 text-xs text-muted-foreground">
                          <ThumbsUp size={12} className="mr-1" /> 有用 ({review.helpful})
                        </Button>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
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

          {/* 评论输入框 */}
          <Card className="mt-4">
            <CardContent className="p-4">
              <div className="flex items-center gap-3">
                <Input
                  placeholder="发表评论..."
                  className="flex-1"
                  value={commentText}
                  onChange={(e) => setCommentText(e.target.value)}
                  maxLength={200}
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
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default ResourceDetailScreen 