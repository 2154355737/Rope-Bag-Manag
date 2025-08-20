import React, { useState, useEffect } from 'react'
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
import { getResource, getResourceComments, createResourceComment, toggleLikeResource, toggleBookmarkResource, reportResource, downloadResource } from '../api/resources'
import { replyComment as apiReplyComment, likeComment as apiLikeComment, getComments as apiGetComments } from '../api/comments'

const ResourceDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [isDownloading, setIsDownloading] = useState(false)
  const [downloadProgress, setDownloadProgress] = useState(0)
  const [isLiked, setIsLiked] = useState(false)
  const [isBookmarked, setIsBookmarked] = useState(false)
  
  // 资源数据（占位，挂载后加载）
  const [resource, setResource] = useState<any>({
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
  })

  const [comments, setComments] = useState<Comment[]>([])
  const [hasMoreComments, setHasMoreComments] = useState(true)
  const [recommendedItems, setRecommendedItems] = useState<any[]>([])

  useEffect(() => {
    const load = async () => {
      try {
        const r = await getResource(parseInt(id || '1'))
        setResource({
          id: r.id,
          title: r.name || r.title,
          author: { name: r.author || '开发者', avatar: '', verified: false },
          description: r.description || '',
          downloadUrl: r.file_url,
          fileSize: r.file_size?.toString() || '0',
          downloadCount: r.download_count || 0,
          likes: r.like_count || 0,
          views: r.view_count || 0,
          rating: r.rating || 4.5,
          reviewCount: r.review_count || 0,
          tags: r.tags || [],
          publishDate: new Date(r.created_at || Date.now()).toLocaleDateString('zh-CN'),
          lastUpdated: new Date(r.updated_at || r.created_at || Date.now()).toLocaleDateString('zh-CN'),
          version: r.version || '1.0.0',
          category: r.category?.name || '其他',
          screenshots: r.screenshots || [],
          files: r.files || [],
          requirements: r.requirements || [],
          safetyStatus: r.safety_status || 'unknown',
          authorStats: {
            totalResources: 12,
            totalDownloads: 8500,
            rating: 4.6
          }
        })
        
        // 加载相关推荐
        const recommendations = await getResourceRecommendations(r.id, r.tags || [])
        setRecommendedItems(recommendations)
        
        // 加载评论
        const cr = await apiGetComments('resource', r.id, 1, 10)
        const mapped = (cr.list || []).map((c: any) => ({
          id: c.id,
          author: { name: c.author_name || '用户', avatar: c.author_avatar || '' },
          content: c.content,
          time: c.created_at || '',
          likes: c.likes || 0,
          isLiked: false
        }))
        setComments(mapped)
        setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
      } catch (e) {
        console.warn(e)
      }
    }
    load()
  }, [id])
  
  // 获取相关推荐
  // const recommendedItems = getResourceRecommendations(resource.id, resource.tags)
  
  // 格式化文件大小
  const formatFileSize = (bytes: string) => {
    return bytes
  }

  // 格式化数字
  const formatNumber = (num: number | undefined | null) => {
    if (num == null || isNaN(num)) return '0'
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 处理下载
  const handleDownload = async () => {
    setIsDownloading(true)
    setDownloadProgress(20)
    try {
      const url = await downloadResource(resource.id)
      setDownloadProgress(80)
      window.location.href = url
      setDownloadProgress(100)
    } catch (e) {
      toast({ title: '下载失败', description: (e as any)?.message || '请稍后再试', variant: 'destructive' })
    } finally {
      setTimeout(() => setIsDownloading(false), 800)
    }
  }



  // 评论区事件处理
  const handleSubmitComment = async (content: string) => {
    await createResourceComment(resource.id, content)
    const cr = await getResourceComments(resource.id, 1, 10)
    const mapped = (cr.list || []).map((c: any) => ({ id: c.id, author: { name: c.author_name || '用户', avatar: c.author_avatar || '' }, content: c.content, time: c.created_at || '', likes: c.likes || 0, isLiked: false }))
    setComments(mapped)
    toast({ title: '评论发送成功', description: '您的评论已发布' })
  }

  const handleSubmitReply = async (commentId: number, content: string) => {
    await apiReplyComment(commentId, content)
    toast({ title: '回复发送成功', description: '您的回复已发布' })
  }

  const handleLikeComment = async (commentId: number) => {
    await apiLikeComment(commentId, true)
    toast({ title: '操作成功', description: '已点赞/取消点赞' })
  }

  const handleReportComment = (commentId: number) => {
    console.log('举报评论:', commentId)
  }

  // 处理点赞
  const handleLike = async () => {
    await toggleLikeResource(resource.id)
    setIsLiked(!isLiked)
    toast({ title: isLiked ? '已取消点赞' : '点赞成功', description: isLiked ? '已取消对此资源的点赞' : '感谢您的支持', duration: 2000 })
  }

  // 处理收藏
  const handleBookmark = async () => {
    await toggleBookmarkResource(resource.id)
    setIsBookmarked(!isBookmarked)
    toast({ title: isBookmarked ? '已取消收藏' : '收藏成功', description: isBookmarked ? '已从收藏夹中移除' : '已添加到您的收藏夹', duration: 2000 })
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
  const handleReport = async () => {
    await reportResource(resource.id)
    toast({ title: '举报已提交', description: '我们会尽快处理您的举报', duration: 2000 })
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
        <div className="p-4 space-y-4 content-container">
          {/* 资源基本信息 */}
          <Card>
            <CardContent className="p-4">
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center flex-1 min-w-0">
                  <Avatar className="h-12 w-12 mr-3 flex-shrink-0">
                    <AvatarImage src={resource.author.avatar} />
                    <AvatarFallback>{resource.author.name[0]}</AvatarFallback>
                  </Avatar>
                  <div className="min-w-0 flex-1">
                    <div className="flex items-center">
                      <span className="font-medium text-overflow-protection truncate">{resource.author.name}</span>
                      {resource.author.verified && (
                        <CheckCircle size={16} className="ml-1 text-blue-500 flex-shrink-0" />
                      )}
                    </div>
                    <div className="text-xs text-muted-foreground">
                      发布于 {resource.publishDate}
                    </div>
                  </div>
                </div>
                <Badge variant="secondary" className="text-xs flex-shrink-0 ml-2">
                  {resource.category}
                </Badge>
              </div>

              <h2 className="text-xl font-bold mb-2 text-overflow-protection">{resource.title}</h2>
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
            <Card>
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
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">详细介绍</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0 content-container">
              <div className="space-y-3 text-overflow-protection">
                {resource.description && resource.description.trim() ? (
                  resource.description.split('\n').map((line, idx) => {
                    const trimmedLine = line.trim()
                    if (!trimmedLine) return <div key={idx} className="h-2" />
                    
                    // 检查是否是URL
                    if (trimmedLine.startsWith('http://') || trimmedLine.startsWith('https://')) {
                      return (
                        <div key={idx} className="my-3">
                          <span className="text-sm text-muted-foreground block mb-1">预览图：</span>
                          <a 
                            href={trimmedLine} 
                            target="_blank" 
                            rel="noopener noreferrer"
                            className="text-sm text-blue-500 hover:text-blue-600 underline url-break block"
                          >
                            {trimmedLine}
                          </a>
                        </div>
                      )
                    }
                    
                    // 检查是否是代码路径（包含特殊字符的文件路径）
                    if (trimmedLine.includes('java') && (trimmedLine.includes('.') || trimmedLine.includes('/'))) {
                      return (
                        <div key={idx} className="my-3 p-3 bg-muted rounded-md">
                          <span className="text-sm font-mono text-overflow-protection break-all">
                            {trimmedLine}
                          </span>
                        </div>
                      )
                    }
                    
                    // 检查是否是特殊标记（★、@等）
                    if (trimmedLine.startsWith('★') || trimmedLine.startsWith('@')) {
                      return (
                        <div key={idx} className="my-2 p-2 bg-orange-50 border-l-4 border-orange-200 rounded-r">
                          <span className="text-sm font-medium text-orange-800 text-overflow-protection">
                            {trimmedLine}
                          </span>
                        </div>
                      )
                    }
                    
                    // 检查是否是方法标题（以"方法"开头）
                    if (trimmedLine.startsWith('方法') || trimmedLine.includes('方法')) {
                      return (
                        <div key={idx} className="my-3">
                          <h4 className="text-base font-semibold text-primary text-overflow-protection">
                            {trimmedLine}
                          </h4>
                        </div>
                      )
                    }
                    
                    // 检查是否是代码示例（包含code、class等关键词）
                    if (trimmedLine.toLowerCase().includes('code') || trimmedLine.includes('class') || trimmedLine.includes('()')) {
                      return (
                        <div key={idx} className="my-2 p-3 bg-slate-100 rounded border">
                          <code className="text-sm font-mono text-overflow-protection break-all">
                            {trimmedLine}
                          </code>
                        </div>
                      )
                    }
                    
                    // 检查是否是列表项
                    if (trimmedLine.startsWith('•') || trimmedLine.startsWith('-')) {
                      return (
                        <div key={idx} className="ml-4 my-1">
                          <span className="text-sm text-overflow-protection">
                            {trimmedLine.substring(1).trim()}
                          </span>
                        </div>
                      )
                    }
                    
                    // 普通文本段落
                    return (
                      <p key={idx} className="text-sm long-text text-overflow-protection leading-relaxed">
                        {trimmedLine}
                      </p>
                    )
                  })
                ) : (
                  <p className="text-sm text-muted-foreground">暂无详细介绍</p>
                )}
              </div>
            </CardContent>
          </Card>

          {/* 系统要求 */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">系统要求</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0 content-container">
              <ul className="space-y-2">
                {resource.requirements.map((req, idx) => (
                  <li key={idx} className="flex items-start text-sm">
                    <CheckCircle size={14} className="text-green-500 mr-2 mt-0.5 flex-shrink-0" />
                    <span className="text-overflow-protection">{req}</span>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          {/* 文件列表 */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">包含文件</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0 content-container">
              <div className="space-y-3">
                {resource.files.map((file, idx) => (
                  <div key={idx} className="flex items-center justify-between p-3 bg-muted rounded border">
                    <div className="flex items-center flex-1 min-w-0">
                      <FileText size={16} className="text-blue-500 mr-2 flex-shrink-0" />
                      <div className="min-w-0 flex-1">
                        <div className="font-medium text-sm text-overflow-protection truncate">{file.name}</div>
                        <div className="text-xs text-muted-foreground">{file.type} • {file.size}</div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>

          {/* 下载按钮 */}
          <Card>
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
            compact={true}
          />

          {/* 相关推荐 */}
          <RelatedRecommendations
            title="相关资源推荐"
            items={recommendedItems}
            currentItemId={resource.id}
            maxItems={6}
            onMoreClick={() => navigate('/category')}
          />

          {/* 评论区 */}
          <CommentSection
            comments={comments}
            totalCount={comments.length}
            onSubmitComment={handleSubmitComment}
            onSubmitReply={handleSubmitReply}
            onLikeComment={handleLikeComment}
            onReportComment={handleReportComment}
            hasMoreComments={hasMoreComments}
            isLoadingComments={false} // Set to false as comments are now loaded in useEffect
            placeholder="发表评论..."
            maxLength={200}
            initialCommentsToShow={5}
          />
        </div>
      </ScrollArea>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default ResourceDetailScreen 