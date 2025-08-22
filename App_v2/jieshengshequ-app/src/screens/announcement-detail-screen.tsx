import React, { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  ArrowLeft, Share2, Bookmark, MoreHorizontal, Flag, 
  AlertTriangle, Info, CheckCircle, Clock, ExternalLink, 
  ThumbsUp, MessageSquare, Bell, Pin, Eye, Calendar,
  FileText, Download, Link as LinkIcon
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'
import CommentSection, { Comment } from '@/components/comment-section'
import RelatedRecommendations from '@/components/related-recommendations'
import InteractionButtons, { 
  createThumbsUpButton, 
  createBookmarkButton, 
  createShareButton, 
  createReportButton 
} from '@/components/ui/interaction-buttons'
import { getAnnouncementRecommendations } from '@/utils/recommendations'
import { getAnnouncement } from '../api/announcements'
import { getComments as apiGetComments, createComment as apiCreateComment, replyComment as apiReplyComment, likeComment as apiLikeComment } from '../api/comments'

const AnnouncementDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [isLiked, setIsLiked] = useState(false)
  const [isBookmarked, setIsBookmarked] = useState(false)
  const [loading, setLoading] = useState(true)

  // 公告数据（初始为空，挂载后加载）
  const [announcement, setAnnouncement] = useState<any>(null)
  const [recommendedItems, setRecommendedItems] = useState<any[]>([])

  // 加载公告详情
  useEffect(() => {
    if (!id) {
      navigate('/home', { replace: true })
      return
    }
    const load = async () => {
      try {
        setLoading(true)
        const a = await getAnnouncement(parseInt(id || '1'))
        setAnnouncement(a)
        
        // 加载相关推荐
        const recommendations = await getAnnouncementRecommendations(a.id, a.tags || [])
        setRecommendedItems(recommendations)
        // 加载第一页评论（公告使用 target_type=post or announcement? 按后端公开接口仅支持 post/package，这里统一按 post 读取）
        try {
          const cr = await apiGetComments('post' as any, a.id, 1, 10)
          const mapped = (cr.list || []).map((c: any) => ({
            id: c.id,
            author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
            content: c.content,
            time: (c.created_at || '').slice(11, 19),
            likes: c.likes || 0,
            isLiked: false,
            replies: []
          }))
          setComments(mapped)
          setCommentTotal(cr.total || mapped.length)
          setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
        } catch {}
        
        setLoading(false)
      } catch (e) {
        console.warn(e)
        setLoading(false)
      }
    }
    load()
  }, [id])

  // 获取相关推荐
  // const recommendedItems = getAnnouncementRecommendations(announcement?.id || 0, announcement?.tags || [])

  const [comments, setComments] = useState<Comment[]>([])
  const [commentTotal, setCommentTotal] = useState(0)
  const [hasMoreComments, setHasMoreComments] = useState(false)
  const [isLoadingComments, setIsLoadingComments] = useState(false)

  // 格式化数字
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 获取公告类型样式
  const getAnnouncementStyle = (type: string) => {
    switch (type) {
      case 'important':
        return {
          bgColor: 'bg-red-50 dark:bg-red-950',
          borderColor: 'border-red-200 dark:border-red-800',
          iconColor: 'text-red-500',
          icon: AlertTriangle
        }
      case 'warning':
        return {
          bgColor: 'bg-yellow-50 dark:bg-yellow-950',
          borderColor: 'border-yellow-200 dark:border-yellow-800',
          iconColor: 'text-yellow-500',
          icon: AlertTriangle
        }
      case 'info':
        return {
          bgColor: 'bg-blue-50 dark:bg-blue-950',
          borderColor: 'border-blue-200 dark:border-blue-800',
          iconColor: 'text-blue-500',
          icon: Info
        }
      default:
        return {
          bgColor: 'bg-green-50 dark:bg-green-950',
          borderColor: 'border-green-200 dark:border-green-800',
          iconColor: 'text-green-500',
          icon: CheckCircle
        }
    }
  }

  // 处理点赞
  const handleLike = () => {
    setIsLiked(!isLiked)
    toast({
      title: isLiked ? "已取消点赞" : "点赞成功",
      description: isLiked ? "已取消对此公告的点赞" : "感谢您的支持",
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

  // 评论区事件处理
  const handleSubmitComment = async (content: string) => {
    if (!announcement) return
    await apiCreateComment('Post', announcement.id, content)
    // 重新加载第一页，刷新总数
    try {
      const cr = await apiGetComments('post' as any, announcement.id, 1, 10)
      const mapped = (cr.list || []).map((c: any) => ({ id: c.id, author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' }, content: c.content, time: (c.created_at || '').slice(11, 19), likes: c.likes || 0, isLiked: false, replies: [] }))
      setComments(mapped)
      setCommentTotal(cr.total || mapped.length)
      setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
    } catch {}
    toast({ title: '反馈已提交', description: '感谢您的反馈，我们会认真处理' })
  }

  const handleSubmitReply = async (commentId: number, content: string) => {
    await apiReplyComment(commentId, content)
    toast({ title: '回复发送成功', description: '您的回复已发布' })
  }

  // 加载更多评论
  const handleLoadMoreComments = async (page: number): Promise<Comment[]> => {
    if (!announcement) return []
    setIsLoadingComments(true)
    try {
      const cr = await apiGetComments('post' as any, announcement.id, page, 10)
      const mapped = (cr.list || []).map((c: any) => ({
        id: c.id,
        author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
        content: c.content,
        time: (c.created_at || '').slice(11, 19),
        likes: c.likes || 0,
        isLiked: false,
        replies: []
      }))
      setHasMoreComments(((cr.total || 0) > page * (cr.size || 10)))
      setCommentTotal(cr.total || 0)
      setIsLoadingComments(false)
      return mapped
    } catch (e) {
      setIsLoadingComments(false)
      return []
    }
  }

  const handleLikeComment = async (commentId: number) => {
    await apiLikeComment(commentId, true)
    toast({ title: '操作成功', description: '已点赞/取消点赞' })
  }

  const handleReportComment = (commentId: number) => {
    console.log('举报评论:', commentId)
  }

  const handleShare = () => {
    toast({ title: '分享链接已复制', description: '可以分享给更多朋友了', duration: 2000 })
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <span className="text-sm text-muted-foreground">正在加载公告...</span>
      </div>
    )
  }

  if (!announcement) {
    return (
      <div className="flex items-center justify-center h-screen">
        <span className="text-sm text-muted-foreground">未找到该公告</span>
      </div>
    )
  }

  const announcementStyle = getAnnouncementStyle(announcement.type)
  const IconComponent = announcementStyle.icon

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="公告详情"
        showBackButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button 
              variant="ghost" 
              size="icon" 
              className="h-9 w-9"
              onClick={() => setIsBookmarked(!isBookmarked)}
            >
              <Bookmark size={20} className={isBookmarked ? "fill-current" : ""} />
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
          {/* 公告头部 */}
          <Card className={`mb-4 ${announcementStyle.bgColor} ${announcementStyle.borderColor}`}>
            <CardContent className="p-4">
              <div className="flex items-start mb-4">
                <IconComponent size={24} className={`${announcementStyle.iconColor} mr-3 mt-1`} />
                <div className="flex-1">
                  <div className="flex items-center mb-2">
                    <Badge 
                      variant={announcement.priority === 'high' ? 'destructive' : 'secondary'} 
                      className="mr-2"
                    >
                      {announcement.priority === 'high' ? '重要' : '一般'}
                    </Badge>
                    {announcement.isPinned && (
                      <Badge variant="outline" className="mr-2">
                        <Pin size={10} className="mr-1" />
                        置顶
                      </Badge>
                    )}
                    <Badge variant="outline">
                      {announcement.type === 'important' ? '重要通知' : '系统公告'}
                    </Badge>
                  </div>
                  <h2 className="text-xl font-bold mb-3">{announcement.title}</h2>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center">
                      <Avatar className="h-8 w-8 mr-2">
                        <AvatarImage src={announcement.author?.avatar} />
                        <AvatarFallback>{(announcement.author?.name || '系')[0]}</AvatarFallback>
                      </Avatar>
                      <div>
                        <div className="flex items-center">
                          <span className="font-medium text-sm">{announcement.author?.name || '系统公告'}</span>
                          {announcement.author?.verified && (
                            <CheckCircle size={14} className="ml-1 text-blue-500" />
                          )}
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {(announcement.author?.role || '系统')} • {announcement.publishDate}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* 时间信息 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              <div className="grid grid-cols-1 gap-3">
                <div className="flex items-center">
                  <Calendar size={16} className="text-muted-foreground mr-2" />
                  <span className="text-sm">发布时间：{announcement.publishDate}</span>
                </div>
                {announcement.effectiveDate && (
                  <div className="flex items-center">
                    <Clock size={16} className="text-muted-foreground mr-2" />
                    <span className="text-sm">生效时间：{announcement.effectiveDate}</span>
                  </div>
                )}
                {announcement.expiryDate && (
                  <div className="flex items-center">
                    <AlertTriangle size={16} className="text-orange-500 mr-2" />
                    <span className="text-sm">截止时间：{announcement.expiryDate}</span>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>

          {/* 公告内容 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              <div className="prose prose-sm max-w-none">
                {announcement.content && announcement.content.trim() ? (
                  announcement.content.split('\n').map((line: string, idx: number) => {
                    const trimmedLine = line.trim()
                    if (!trimmedLine) return <div key={idx} className="h-2" />
                    
                    // 检查是否是URL
                    if (trimmedLine.startsWith('http://') || trimmedLine.startsWith('https://')) {
                      return (
                        <div key={idx} className="my-3 break-all">
                          <a 
                            href={trimmedLine} 
                            target="_blank" 
                            rel="noopener noreferrer"
                            className="text-sm text-blue-500 hover:text-blue-600 underline break-all"
                          >
                            {trimmedLine}
                          </a>
                        </div>
                      )
                    }
                    
                    // 检查是否是标题
                    if (trimmedLine.startsWith('## ')) {
                      return (
                        <h3 key={idx} className="text-lg font-semibold mt-6 mb-3 first:mt-0 break-words">
                          {trimmedLine.replace('## ', '')}
                        </h3>
                      )
                    }
                    
                    // 检查是否是强调文本
                    if (trimmedLine.startsWith('**') && trimmedLine.endsWith('**')) {
                      return (
                        <div key={idx} className="bg-muted p-3 rounded-md my-3">
                          <p className="font-medium text-sm break-words">
                            {trimmedLine.replace(/\*\*/g, '')}
                          </p>
                        </div>
                      )
                    }
                    
                    // 检查是否是列表项
                    if (trimmedLine.startsWith('- ') || trimmedLine.startsWith('• ')) {
                      return (
                        <div key={idx} className="ml-4 my-1">
                          <span className="text-sm break-words">
                            • {trimmedLine.substring(2).trim()}
                          </span>
                        </div>
                      )
                    }
                    
                    // 普通文本段落
                    return (
                      <p key={idx} className="text-sm my-3 break-words whitespace-pre-wrap">
                        {trimmedLine}
                      </p>
                    )
                  })
                ) : (
                  <p className="text-sm text-muted-foreground">暂无公告内容</p>
                )}
              </div>
            </CardContent>
          </Card>

          {/* 附件下载 - 后端暂未提供，保留结构 */}
          {Array.isArray(announcement.attachments) && announcement.attachments.length > 0 && (
            <Card className="mb-4">
              <CardHeader>
                <CardTitle className="text-lg">相关附件</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="space-y-3">
                  {announcement.attachments.map((attachment: any, idx: number) => (
                    <div key={idx} className="flex items-center justify-between p-3 border rounded-md">
                      <div className="flex items-center">
                        <FileText size={16} className="text-muted-foreground mr-2" />
                        <div>
                          <div className="font-medium text-sm">{attachment.name}</div>
                          <div className="text-xs text-muted-foreground">{attachment.size}</div>
                        </div>
                      </div>
                      <Button size="sm" variant="outline">
                        <Download size={14} className="mr-1" />
                        下载
                      </Button>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 相关链接 - 后端暂未提供，保留结构 */}
          {Array.isArray(announcement.relatedLinks) && announcement.relatedLinks.length > 0 && (
            <Card className="mb-4">
              <CardHeader>
                <CardTitle className="text-lg">相关链接</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="space-y-3">
                  {announcement.relatedLinks.map((link: any, idx: number) => (
                    <div key={idx} className="p-3 border rounded-md">
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <div className="flex items-center mb-1">
                            <LinkIcon size={14} className="text-muted-foreground mr-2" />
                            <span className="font-medium text-sm">{link.title}</span>
                          </div>
                          <p className="text-xs text-muted-foreground">{link.description}</p>
                        </div>
                        <Button size="sm" variant="ghost">
                          <ExternalLink size={14} />
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 标签 */}
          {Array.isArray(announcement.tags) && announcement.tags.length > 0 && (
            <Card className="mb-4">
              <CardContent className="p-4">
                <div className="flex flex-wrap gap-2">
                  {announcement.tags.map((tag: string, idx: number) => (
                    <Badge key={idx} variant="outline" className="text-xs">
                      {tag}
                    </Badge>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 统计信息 - 后端暂不提供，置 0 */}
          <Card className="mb-4">
            <CardContent className="p-3">
              <div className="grid grid-cols-3 gap-3 text-center">
                <div>
                  <div className="flex items-center justify-center mb-1">
                    <Eye size={12} className="text-muted-foreground mr-1" />
                  </div>
                  <div className="text-base font-bold">{formatNumber(announcement.views || 0)}</div>
                  <div className="text-[10px] text-muted-foreground">浏览量</div>
                </div>
                <div>
                  <div className="flex items-center justify-center mb-1">
                    <ThumbsUp size={12} className="text-muted-foreground mr-1" />
                  </div>
                  <div className="text-base font-bold">{formatNumber(announcement.likes || 0)}</div>
                  <div className="text-[10px] text-muted-foreground">点赞数</div>
                </div>
                <div>
                  <div className="flex items-center justify-center mb-1">
                    <MessageSquare size={12} className="text-muted-foreground mr-1" />
                  </div>
                  <div className="text-base font-bold">{formatNumber(announcement.comments || 0)}</div>
                  <div className="text-[10px] text-muted-foreground">评论数</div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* 操作按钮 */}
          <InteractionButtons
            buttons={[
              createThumbsUpButton((announcement.likes || 0) + (isLiked ? 1 : 0), isLiked, handleLike),
              createShareButton(handleShare),
              createBookmarkButton(undefined, isBookmarked, handleBookmark),
              createReportButton(() => {})
            ]}
            className="mb-4"
            compact={true}
          />

          {/* 重要提醒 */}
          <Alert className="mb-4">
            <Bell size={16} />
            <AlertDescription>
              此公告内容重要，建议收藏保存。如有疑问请及时联系客服。
            </AlertDescription>
          </Alert>
        </div>
      </ScrollArea>

      {/* 相关推荐 */}
      <div className="p-4">
        <RelatedRecommendations
          title="相关公告推荐"
          items={recommendedItems}
          currentItemId={announcement.id}
          maxItems={6}
          onMoreClick={() => navigate('/home')}
        />
      </div>

      {/* 评论区 */}
      <div className="p-4">
        <CommentSection
          comments={comments}
          totalCount={commentTotal}
          onSubmitComment={handleSubmitComment}
          onSubmitReply={handleSubmitReply}
          onLikeComment={handleLikeComment}
          onReportComment={handleReportComment}
          onLoadMoreComments={handleLoadMoreComments}
          hasMoreComments={hasMoreComments}
          isLoadingComments={isLoadingComments}
          placeholder="对此公告有疑问或建议..."
          maxLength={200}
          initialCommentsToShow={5}
        />
      </div>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default AnnouncementDetailScreen 