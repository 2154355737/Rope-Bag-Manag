import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  ArrowLeft, Download, Heart, MessageSquare, Share2, Bookmark, 
  MoreHorizontal, Flag, Hash, ThumbsUp, ThumbsDown, 
  Star, FileText, Package, Shield, Calendar, Eye, AlertTriangle,
  ExternalLink, Copy, CheckCircle
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Progress } from '@/components/ui/progress'
import { Alert, AlertDescription } from '@/components/ui/alert'

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
import { getResourceRecommendations } from '@/utils/recommendations'

const ResourceDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [isDownloading, setIsDownloading] = useState(false)
  const [downloadProgress, setDownloadProgress] = useState(0)
  const [isLiked, setIsLiked] = useState(false)
  const [isBookmarked, setIsBookmarked] = useState(false)
  
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

  // 获取相关推荐
  const recommendedItems = getResourceRecommendations(resource.id, resource.tags)
  
  // 模拟评论数据（包含原评价内容）
  const comments: Comment[] = [
    {
      id: 1,
      author: {
        name: '张开发',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      content: '非常棒的工具包！组件质量很高，文档也很详细。已经在我的项目中使用了，效果很好。',
      time: '2025-01-12',
      likes: 23,
      isLiked: false,
    },
    {
      id: 2,
      author: {
        name: '李前端',
        avatar: 'https://i.pravatar.cc/150?img=2',
      },
      content: '整体不错，但是某些组件的API设计还可以优化。希望下个版本能改进。',
      time: '2025-01-08',
      likes: 12,
      isLiked: false,
    },
    {
      id: 3,
      author: {
        name: '王开发',
        avatar: 'https://i.pravatar.cc/150?img=3',
        verified: true,
      },
      content: '这个工具包真的很实用！TypeScript支持做得很好，文档也很全面。',
      time: '2天前',
      likes: 15,
      isLiked: true,
      replies: [
        {
          id: 101,
          author: {
            name: '李前端',
            avatar: 'https://i.pravatar.cc/150?img=2',
          },
          content: '同感！特别是组件库部分，节省了很多开发时间。',
          time: '1天前',
          likes: 8,
          isLiked: false,
        }
      ]
    },
    {
      id: 4,
      author: {
        name: '赵设计',
        avatar: 'https://i.pravatar.cc/150?img=4',
      },
      content: 'UI组件设计很不错，但是希望能增加更多的主题样式选项。',
      time: '3天前',
      likes: 12,
      isLiked: false,
    },
    {
      id: 5,
      author: {
        name: '程序员小张',
        avatar: 'https://i.pravatar.cc/150?img=5',
      },
      content: '这个组件库真的太好用了！集成简单，样式美观，强烈推荐给所有前端开发者。',
      time: '昨天',
      likes: 28,
      isLiked: true,
      rating: 5,
      helpful: 15
    },
    {
      id: 6,
      author: {
        name: 'React爱好者',
        avatar: 'https://i.pravatar.cc/150?img=6',
      },
      content: '文档写得很清楚，示例代码也很实用。唯一的建议是希望能提供更多的使用场景案例。',
      time: '昨天',
      likes: 18,
      isLiked: false,
      rating: 4,
      helpful: 12
    },
    {
      id: 7,
      author: {
        name: '全栈工程师',
        avatar: 'https://i.pravatar.cc/150?img=7',
      },
      content: '很好的资源包！TypeScript类型定义完善，对开发体验的提升很大。',
      time: '2天前',
      likes: 22,
      isLiked: true,
      rating: 5,
      helpful: 18,
      replies: [
        {
          id: 701,
          author: {
            name: '前端新手',
            avatar: 'https://i.pravatar.cc/150?img=8',
          },
          content: '作为新手，这个资源包帮了我很大忙！',
          time: '1天前',
          likes: 5,
          isLiked: false,
        }
      ]
    },
    {
      id: 8,
      author: {
        name: '移动端开发',
        avatar: 'https://i.pravatar.cc/150?img=9',
      },
      content: '移动端适配做得很好，响应式设计很赞。期待看到更多移动端专用组件。',
      time: '3天前',
      likes: 16,
      isLiked: false,
      rating: 4,
      helpful: 10
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

  // 处理点赞
  const handleLike = () => {
    setIsLiked(!isLiked)
    toast({
      title: isLiked ? "已取消点赞" : "点赞成功",
      description: isLiked ? "已取消对此资源的点赞" : "感谢您的支持",
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
                  <Star size={16} className="fill-yellow-400 text-yellow-400 mr-1" />
                  <span className="text-sm text-muted-foreground">
                    {resource.rating} ({resource.reviewCount} 评价)
                  </span>
                </div>
              </div>

              {/* 统计信息 */}
              <div className="grid grid-cols-3 gap-3 mb-4">
                <div className="text-center">
                  <div className="text-base font-bold">{formatNumber(resource.downloadCount)}</div>
                  <div className="text-[10px] text-muted-foreground">下载量</div>
                </div>
                <div className="text-center">
                  <div className="text-base font-bold">{formatNumber(resource.views)}</div>
                  <div className="text-[10px] text-muted-foreground">浏览量</div>
                </div>
                <div className="text-center">
                  <div className="text-base font-bold">{formatNumber(resource.likes)}</div>
                  <div className="text-[10px] text-muted-foreground">点赞数</div>
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



          {/* 操作按钮 */}
          <InteractionButtons
            buttons={[
              createLikeButton(resource.likes + (isLiked ? 1 : 0), isLiked, handleLike),
              createShareButton(handleShare),
              createBookmarkButton(undefined, isBookmarked, handleBookmark),
              createReportButton(handleReport)
            ]}
            className="mb-4"
            compact={true}
          />

          {/* 相关推荐 */}
          <RelatedRecommendations
            title="相关资源推荐"
            items={recommendedItems}
            currentItemId={resource.id}
            maxItems={6}
            className="mt-6"
            onMoreClick={() => navigate('/category')}
          />

          {/* 评论区 */}
          <CommentSection
            comments={comments}
            totalCount={156}
            onSubmitComment={handleSubmitComment}
            onSubmitReply={handleSubmitReply}
            onLikeComment={handleLikeComment}
            onReportComment={handleReportComment}
            placeholder="发表评论..."
            maxLength={200}
            initialCommentsToShow={5}
            className="mt-6"
          />
        </div>
      </ScrollArea>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default ResourceDetailScreen 