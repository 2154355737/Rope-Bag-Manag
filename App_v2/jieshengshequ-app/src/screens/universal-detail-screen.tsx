import React, { useState, useEffect } from 'react'
import { useNavigate, useParams, useLocation } from 'react-router-dom'
import { motion } from 'framer-motion'
import { 
  ArrowLeft, Share2, MoreHorizontal, Download,
  Heart, MessageSquare, Eye, CheckCircle, Shield, Hash,
  FileText, Loader2, Calendar, X, ZoomIn, Settings
} from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Progress } from '@/components/ui/progress'
import { toast } from '@/hooks/use-toast'

import TopNavigation from '@/components/ui/top-navigation'
import MarkdownRenderer from '@/components/ui/markdown-renderer'
import CommentSection, { Comment } from '@/components/comment-section'
import RelatedRecommendations from '@/components/related-recommendations'
import InteractionButtons, { 
  createLikeButton, 
  createShareButton, 
  createReportButton 
} from '@/components/ui/interaction-buttons'

// API 导入
import { getPost, toggleLikePost, getPostLikeStatus } from '@/api/posts'
import { getResource, toggleLikeResource, getResourceLikeStatus, downloadResource } from '@/api/resources'
import { getAnnouncement } from '@/api/announcements'
import { createComment as apiCreateComment, replyComment as apiReplyComment, getComments as apiGetComments } from '@/api/comments'
import { getLocalUser } from '@/api/auth'
import { getContentBasedRecommendations } from '@/utils/recommendations'

// 通用数据类型
interface UniversalDetailItem {
  id: number
  type: 'post' | 'resource' | 'announcement'
  title: string
  author: {
    id: number
    name: string
    avatar: string
    verified?: boolean
    role?: string
  }
  content?: string
  description?: string
  tags: string[]
  stats: {
    likes: number
    comments: number
    views: number
    downloads?: number
    rating?: number
  }
  publishDate: string
  category?: string
  
  // 资源特有字段
  version?: string
  fileSize?: string
  downloadUrl?: string
  files?: Array<{ name: string; size: string; type: string }>
  requirements?: string[]
  screenshots?: string[]
  safetyStatus?: string
  
  // 帖子特有字段
  images?: string[]
  codeSnippet?: string
  
  // 公告特有字段
  type_detail?: string
  priority?: string
  isPinned?: boolean
  effectiveDate?: string
  expiryDate?: string
  attachments?: Array<{ name: string; size: string; url: string }>
  relatedLinks?: Array<{ title: string; url: string; description?: string }>
}

const UniversalDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const location = useLocation()
  const navigate = useNavigate()
  
  // 从路径中提取类型
  const getTypeFromPath = (): string | null => {
    const pathname = location.pathname
    if (pathname.startsWith('/post/')) return 'post'
    if (pathname.startsWith('/resource/')) return 'resource'  
    if (pathname.startsWith('/announcement/')) return 'announcement'
    return null
  }
  
  const type = getTypeFromPath()
  
  const [item, setItem] = useState<UniversalDetailItem | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<{ type: 'forbidden' | 'not-found' | 'general', message: string } | null>(null)
  
  // 图片查看器状态
  const [imageViewerOpen, setImageViewerOpen] = useState(false)
  const [currentImageIndex, setCurrentImageIndex] = useState(0)
  const [imageList, setImageList] = useState<string[]>([])
  
  // 文件大小格式化函数
  const formatFileSize = (size: string | number): string => {
    if (!size) return ''
    
    // 转换为字符串处理
    const sizeStr = String(size)
    
    // 移除已有的单位，只保留数字和小数点
    const cleanSize = sizeStr.replace(/[^\d.]/g, '')
    const sizeNumber = parseFloat(cleanSize)
    
    if (isNaN(sizeNumber)) return sizeStr
    
    // 如果数字很小，假设是字节
    if (sizeNumber < 1024) {
      return `${sizeNumber}B`
    }
    // 如果在1024-1048576之间，假设是KB
    else if (sizeNumber < 1048576) {
      return `${(sizeNumber / 1024).toFixed(1)}KB`
    }
    // 如果更大，转换为MB
    else {
      return `${(sizeNumber / 1048576).toFixed(1)}MB`
    }
  }
  
  // 交互状态
  const [isLiked, setIsLiked] = useState(false)
  const [isDownloading, setIsDownloading] = useState(false)
  const [downloadProgress, setDownloadProgress] = useState(0)
  const [comments, setComments] = useState<Comment[]>([])
  const [commentTotal, setCommentTotal] = useState(0)
  const [hasMoreComments, setHasMoreComments] = useState(false)
  const [isLoadingComments, setIsLoadingComments] = useState(false)
  const [recommendedItems, setRecommendedItems] = useState<any[]>([])

  // 加载数据
  useEffect(() => {
    const loadData = async () => {
      if (!type || !id) return
      
      try {
        setLoading(true)
        setError(null)
        let data: any

        switch (type) {
          case 'post':
            data = await getPost(parseInt(id))
            break
            
          case 'resource':
            data = await getResource(parseInt(id))
            // 兼容后端：将 included_files 正常化为 files
            const normalizedFiles = Array.isArray(data.files) && data.files.length
              ? data.files
              : (Array.isArray(data.included_files) ? data.included_files : [])
            const mapped = normalizedFiles.map((f: any) => ({
              name: f?.name || '未知文件',
              type: f?.type || f?.file_type || '未知',
              size: typeof f?.size === 'number' ? `${f.size}` : (f?.size || '')
            }))
            // 回退主文件
            const fallback: Array<{ name: string; type: string; size: string }> = []
            if ((!mapped || mapped.length === 0) && data.file_url) {
              const fileName = data.file_url.split('/').pop() || '下载文件'
              const ext = fileName.split('.').pop()?.toLowerCase() || ''
              const typeMap: Record<string, string> = { zip: 'ZIP压缩包', apk: 'Android应用', exe: 'Windows程序', dmg: 'macOS应用', pdf: 'PDF文档' }
              fallback.push({ name: fileName, type: typeMap[ext] || '文件', size: data.file_size || '' })
            }
            data.files = mapped.length > 0 ? mapped : fallback
            break
            
          case 'announcement':
            data = await getAnnouncement(parseInt(id))
            break
            
          default:
            throw new Error('不支持的内容类型')
        }
        
        // 处理tags字段，确保它是数组格式
        let tags: string[] = []
        if (data.tags) {
          if (Array.isArray(data.tags)) {
            tags = data.tags
          } else if (typeof data.tags === 'string') {
            try {
              // 尝试解析JSON字符串
              const parsed = JSON.parse(data.tags)
              tags = Array.isArray(parsed) ? parsed : []
            } catch {
              // 如果解析失败，当作单个标签处理
              tags = [data.tags]
            }
          }
        }

        // 处理requirements字段，确保它是数组格式
        let requirements: string[] = []
        if (data.requirements) {
          if (Array.isArray(data.requirements)) {
            requirements = data.requirements
          } else if (typeof data.requirements === 'string') {
            try {
              // 尝试解析JSON字符串
              const parsed = JSON.parse(data.requirements)
              requirements = Array.isArray(parsed) ? parsed : []
            } catch {
              // 如果解析失败，当作单个需求处理
              requirements = [data.requirements]
            }
          }
        }

        // 处理files字段，确保它是数组格式
        let files: any[] = []
        if (data.files) {
          if (Array.isArray(data.files)) {
            files = data.files
          } else if (typeof data.files === 'string') {
            try {
              // 尝试解析JSON字符串
              const parsed = JSON.parse(data.files)
              files = Array.isArray(parsed) ? parsed : []
            } catch {
              // 如果解析失败，返回空数组
              files = []
            }
          }
        }

        // 处理screenshots字段，确保它是数组格式
        let screenshots: string[] = []
        if (data.screenshots) {
          if (Array.isArray(data.screenshots)) {
            screenshots = data.screenshots
          } else if (typeof data.screenshots === 'string') {
            try {
              // 尝试解析JSON字符串
              const parsed = JSON.parse(data.screenshots)
              screenshots = Array.isArray(parsed) ? parsed : []
            } catch {
              // 如果解析失败，返回空数组
              screenshots = []
            }
          }
        }

        const unifiedItem: UniversalDetailItem = {
              id: data.id,
          type: type as any,
          title: data.title || data.name,
              author: { 
            id: data.author?.id || data.user_id || 1,
            name: data.author?.name || data.author_name || data.username || '系统',
            avatar: data.author?.avatar || data.author_avatar || '',
            verified: data.author?.verified,
            role: data.author?.role
              },
              content: data.content,
          description: data.description,
              tags,
              stats: {
            likes: data.like_count || data.likes || 0,
            comments: data.comment_count || data.comments || 0,
            views: data.view_count || data.views || data.views_count || 0,
            downloads: data.download_count || data.downloads,
            rating: data.rating
          },
          publishDate: data.created_at || data.publishedAt || data.published_at || data.publishDate || new Date().toISOString(),
          category: data.category || data.category_name,
          
          // 资源特有
          version: data.version,
          fileSize: data.file_size,
          downloadUrl: data.download_url || data.file_url,
          files: files,
          requirements: requirements,
          screenshots: screenshots,
          safetyStatus: data.safety_status,
          
          // 帖子特有
          images: data.images,
          codeSnippet: data.code_snippet,
          
          // 公告特有
          type_detail: data.type,
          priority: data.priority,
          isPinned: data.is_pinned,
          effectiveDate: data.effective_date,
          expiryDate: data.expiry_date,
          attachments: data.attachments,
          relatedLinks: data.related_links
        }
        
        setItem(unifiedItem)
        
        // 加载点赞状态
        try {
          if (type === 'post') {
            const likeStatus = await getPostLikeStatus(parseInt(id))
            setIsLiked(!!likeStatus?.liked)
          } else if (type === 'resource') {
            const likeStatus = await getResourceLikeStatus(parseInt(id))
            setIsLiked(!!likeStatus?.liked)
          }
        } catch (error) {
          console.log('获取点赞状态失败:', error)
        }
        
        // 加载评论
        try {
        const targetType = type === 'resource' ? 'package' : type
          const commentResponse = await apiGetComments(targetType as any, parseInt(id), 1, 10, true)
          const user = getLocalUser()
          const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
          
          const mappedComments = (commentResponse.list || []).map((c: any) => ({
          id: c.id,
            author: { 
              name: c.author_name || '用户', 
              avatar: c.author_avatar || '' 
            },
          content: c.content,
          time: formatTimeOfDay(c.created_at || ''),
          likes: c.likes || 0,
          isLiked: false,
            replies: (c.replies || []).map((r: any) => ({
              id: r.id,
              author: { 
                name: r.author_name || '用户', 
                avatar: r.author_avatar || '' 
              },
              content: r.content,
              time: formatTimeOfDay(r.created_at || ''),
              likes: r.likes || 0,
              isLiked: false,
              canEdit: !!user && (isPrivileged(user.role) || user.id === r.user_id)
            })),
            canEdit: !!user && (isPrivileged(user.role) || user.id === c.user_id)
          }))
          
          setComments(mappedComments)
          setHasMoreComments((commentResponse.total || 0) > (commentResponse.page || 1) * (commentResponse.size || 10))
          setCommentTotal(commentResponse.total || mappedComments.length)
      } catch (error) {
          console.log('加载评论失败:', error)
        }
        
        // 获取真实推荐数据
        try {
          const includeTypes: ('post' | 'resource' | 'announcement')[] = type === 'resource' 
            ? ['resource', 'post'] 
            : type === 'post' 
            ? ['post', 'resource'] 
            : ['announcement', 'post']
            
          const recommendations = await getContentBasedRecommendations({
            currentItemId: parseInt(id),
            currentItemType: type as 'post' | 'resource' | 'announcement',
            currentItemTags: unifiedItem.tags,
            limit: 6,
            includeTypes
          })
          setRecommendedItems(recommendations)
        } catch (error) {
          console.log('获取推荐数据失败:', error)
          setRecommendedItems([])
        }
        
      } catch (error: any) {
        console.error('Universal Detail: 加载详情失败:', error)
        
        // 根据错误类型设置不同的错误状态
        if (error?.message?.includes('未审核通过') || error?.message?.includes('Forbidden')) {
          setError({
            type: 'forbidden',
            message: '该内容正在审核中或未通过审核，暂时无法查看'
          })
          toast({
            title: "访问受限",
            description: "该内容正在审核中或未通过审核，暂时无法查看",
            variant: "destructive"
          })
        } else if (error?.message?.includes('Not Found') || error?.status === 404) {
          setError({
            type: 'not-found',
            message: '该内容可能已被删除或不存在'
          })
          toast({
            title: "内容不存在",
            description: "该内容可能已被删除或不存在",
            variant: "destructive"
          })
        } else {
          setError({
            type: 'general',
            message: '无法加载详情信息，请稍后重试'
          })
          toast({
            title: "加载失败",
            description: "无法加载详情信息，请稍后重试",
            variant: "destructive"
          })
        }
      } finally {
        setLoading(false)
      }
    }
    
    loadData()
  }, [type, id])

  // 图片查看器功能
  const handleImageClick = (images: string[], index: number) => {
    setImageList(images)
    setCurrentImageIndex(index)
    setImageViewerOpen(true)
  }

  const navigateImage = (direction: 'prev' | 'next') => {
    if (direction === 'prev') {
      setCurrentImageIndex(prev => prev > 0 ? prev - 1 : imageList.length - 1)
      } else {
      setCurrentImageIndex(prev => prev < imageList.length - 1 ? prev + 1 : 0)
    }
  }

  // 工具函数
  const formatNumber = (num: number): string => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  const getPageTitle = (): string => {
    if (!item) return '加载中...'
    switch (item.type) {
      case 'post': return '帖子详情'
      case 'resource': return '资源详情'
      case 'announcement': return '公告详情'
      default: return '详情'
    }
  }

  const handleShare = () => {
    if (navigator.share) {
      navigator.share({
        title: item?.title,
        url: window.location.href
      })
    } else {
      navigator.clipboard.writeText(window.location.href)
      toast({ title: '链接已复制', description: '分享链接已复制到剪贴板' })
    }
  }

  const formatDate = (dateString: string): string => {
    try {
      const date = new Date(dateString)
      return date.toLocaleDateString('zh-CN', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      })
    } catch {
      return '未知时间'
    }
  }

  const formatTimeOfDay = (dateString: string): string => {
    try {
      const date = new Date(dateString)
      const now = new Date()
      const diffMs = now.getTime() - date.getTime()
      const diffMins = Math.floor(diffMs / (1000 * 60))
      const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
      const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

      if (diffMins < 1) return '刚刚'
      if (diffMins < 60) return `${diffMins}分钟前`
      if (diffHours < 24) return `${diffHours}小时前`
      if (diffDays < 7) return `${diffDays}天前`
      return formatDate(dateString)
    } catch {
      return '未知时间'
    }
  }

  // 交互处理函数
  const handleLike = async () => {
    if (!item) return
    try {
      if (item.type === 'post') {
        await toggleLikePost(item.id)
      } else if (item.type === 'resource') {
        await toggleLikeResource(item.id)
      }
      setIsLiked(!isLiked)
      // 更新本地计数
      setItem(prev => prev ? {
        ...prev,
        stats: {
          ...prev.stats,
          likes: prev.stats.likes + (isLiked ? -1 : 1)
        }
      } : null)
    } catch (error) {
      console.error('点赞失败:', error)
      toast({
        title: "操作失败",
        description: "点赞操作失败，请稍后重试",
        variant: "destructive"
      })
    }
  }

  const handleReport = () => {
    toast({
      title: "举报已提交",
      description: "我们会尽快处理您的举报"
    })
  }

  const handleSubmitComment = async (content: string) => {
    if (!item) return
    try {
      const targetType = item.type === 'resource' ? 'package' : item.type
      await apiCreateComment(targetType as any, item.id, content)
      // 重新加载评论
      // 这里可以优化为直接添加到本地状态
      toast({
        title: "评论成功",
        description: "您的评论已发布"
      })
    } catch (error) {
      console.error('评论失败:', error)
      toast({
        title: "评论失败",
        description: "发布评论失败，请稍后重试",
        variant: "destructive"
      })
    }
  }

  const handleReplyComment = async (commentId: number, content: string) => {
    if (!item) return
    try {
      await apiReplyComment(commentId, content)
      toast({
        title: "回复成功",
        description: "您的回复已发布"
      })
    } catch (error) {
      console.error('回复失败:', error)
      toast({
        title: "回复失败",
        description: "发布回复失败，请稍后重试",
        variant: "destructive"
      })
    }
  }

  // 处理下载
  const handleDownload = async () => {
    if (!item || item.type !== 'resource' || !item.downloadUrl) {
      toast({ 
        title: '下载失败', 
        description: '下载链接不可用', 
        variant: 'destructive' 
      })
      return
    }

    setIsDownloading(true)
    setDownloadProgress(20)
    
    try {
      // 先尝试获取下载URL
      const url = await downloadResource(item.id)
      setDownloadProgress(60)
      
      if (url) {
        // 创建一个临时链接来触发下载
        const link = document.createElement('a')
        link.href = url
        link.target = '_blank'
        link.rel = 'noopener noreferrer'
        
        // 尝试从URL中提取文件名
        const fileName = url.split('/').pop() || item.title || 'download'
        link.download = fileName
        
        document.body.appendChild(link)
        setDownloadProgress(80)
        
        link.click()
        document.body.removeChild(link)
        
        setDownloadProgress(100)
        
        // 更新下载计数
        setItem(prev => prev ? {
          ...prev,
          stats: {
            ...prev.stats,
            downloads: (prev.stats.downloads || 0) + 1
          }
        } : null)
        
        toast({
          title: '下载成功',
          description: '文件下载已开始'
        })
      } else {
        throw new Error('无法获取下载链接')
      }
    } catch (error: any) {
      console.error('下载失败:', error)
      toast({
        title: '下载失败',
        description: error.message || '下载过程中出现错误，请稍后重试',
        variant: 'destructive'
      })
    } finally {
      setTimeout(() => {
        setIsDownloading(false)
        setDownloadProgress(0)
      }, 1000)
    }
  }

  const handleLoadMoreComments = async (page: number): Promise<Comment[]> => {
    // 实现加载更多评论
    setIsLoadingComments(true)
    try {
      if (!item) return []
      const targetType = item.type === 'resource' ? 'package' : item.type
      const commentResponse = await apiGetComments(targetType as any, item.id, page, 10, true)
      const user = getLocalUser()
      const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
      
      const newComments = (commentResponse.list || []).map((c: any) => ({
        id: c.id,
        author: { 
          name: c.author_name || '用户', 
          avatar: c.author_avatar || '' 
        },
        content: c.content,
        time: formatTimeOfDay(c.created_at || ''),
        likes: c.likes || 0,
        isLiked: false,
        replies: (c.replies || []).map((r: any) => ({
          id: r.id,
          author: { 
            name: r.author_name || '用户', 
            avatar: r.author_avatar || '' 
          },
          content: r.content,
          time: formatTimeOfDay(r.created_at || ''),
          likes: r.likes || 0,
          isLiked: false,
          canEdit: !!user && (isPrivileged(user.role) || user.id === r.user_id)
        })),
        canEdit: !!user && (isPrivileged(user.role) || user.id === c.user_id)
      }))
      
      // 更新本地状态
      setComments(prev => [...prev, ...newComments])
      setHasMoreComments((commentResponse.total || 0) > page * (commentResponse.size || 10))
      
      return newComments
    } catch (error) {
      console.error('加载更多评论失败:', error)
      return []
    } finally {
      setIsLoadingComments(false)
    }
  }

  // 错误状态显示
  if (error) {
    return (
      <div className="flex flex-col min-h-screen bg-background pb-16">
        <TopNavigation
          title="加载失败"
          showBackButton
        />
        
        <div className="flex-1 flex items-center justify-center p-4">
          <Card className="w-full max-w-md">
            <CardContent className="p-6 text-center">
              <div className="mb-4">
                {error.type === 'forbidden' ? (
                  <Shield className="h-16 w-16 text-amber-500 mx-auto" />
                ) : error.type === 'not-found' ? (
                  <FileText className="h-16 w-16 text-gray-400 mx-auto" />
                ) : (
                  <X className="h-16 w-16 text-red-500 mx-auto" />
                )}
              </div>
              
              <h3 className="text-lg font-semibold mb-2">
                {error.type === 'forbidden' ? '访问受限' : 
                 error.type === 'not-found' ? '内容不存在' : '加载失败'}
              </h3>
              
              <p className="text-muted-foreground mb-4">{error.message}</p>
              
              <div className="flex gap-2 justify-center">
                <Button variant="outline" onClick={() => navigate('/')}>
                  返回首页
                </Button>
                {error.type === 'general' && (
                  <Button onClick={() => window.location.reload()}>
                    重新加载
                  </Button>
                )}
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    )
  }

  // 加载状态
  if (loading) {
    return (
      <div className="flex flex-col min-h-screen bg-background pb-16">
        <TopNavigation
          title="加载中..."
          showBackButton
        />
        <div className="flex-1 flex items-center justify-center">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      </div>
    )
  }

  if (!item) {
    return (
      <div className="flex flex-col min-h-screen bg-background pb-16">
        <TopNavigation
          title="未找到内容"
          showBackButton
        />
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <FileText className="h-16 w-16 text-muted-foreground mx-auto mb-4" />
            <p className="text-muted-foreground">内容不存在或已被删除</p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title={getPageTitle()}
        showBackButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button variant="ghost" size="icon" className="h-9 w-9" onClick={handleShare}>
              <Share2 size={20} />
            </Button>
            <Button variant="ghost" size="icon" className="h-9 w-9">
              <MoreHorizontal size={20} />
            </Button>
          </div>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="flex-1 pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        <ScrollArea className="h-full">
          <motion.div 
            className="container max-w-2xl mx-auto px-4 py-3 space-y-3 pb-safe-bottom"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3 }}
          >
          
          {/* 标题和作者信息 */}
            <Card>
              <CardContent className="p-4">
              <div className="flex items-start gap-3 mb-4">
                <Avatar className="h-10 w-10">
                  <AvatarImage src={item.author.avatar} />
                  <AvatarFallback>{item.author.name[0]}</AvatarFallback>
                    </Avatar>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="font-medium text-sm">{item.author.name}</span>
                    {item.author.verified && <CheckCircle size={14} className="text-blue-500" />}
                      </div>
                  <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <Calendar size={12} />
                    <span>{formatDate(item.publishDate)}</span>
                      </div>
                    </div>
                </div>

              <h1 className="text-xl font-bold mb-3 leading-tight">{item.title}</h1>

                {/* 标签 */}
                {item.tags.length > 0 && (
                <div className="flex items-center gap-2 mb-4">
                  <Hash size={14} className="text-muted-foreground" />
                  <div className="flex flex-wrap gap-1">
                    {item.tags.map((tag, idx) => (
                      <Badge key={idx} variant="secondary" className="text-xs">
                        {tag}
                      </Badge>
                    ))}
                  </div>
                  </div>
                )}
              </CardContent>
            </Card>

            {/* 统计信息卡片 */}
            <Card>
              <CardContent className="p-4">
                <div className="flex items-center justify-around">
                  {item.stats.downloads !== undefined && (
                    <div className="text-center">
                      <div className="text-lg font-bold text-primary">{formatNumber(item.stats.downloads)}</div>
                      <div className="text-xs text-muted-foreground">下载量</div>
                    </div>
                  )}
                  {item.stats.downloads !== undefined && <div className="h-8 w-px bg-border"></div>}
                  <div className="text-center">
                    <div className="text-lg font-bold text-primary">{formatNumber(item.stats.views)}</div>
                    <div className="text-xs text-muted-foreground">浏览量</div>
                  </div>
                  <div className="h-8 w-px bg-border"></div>
                  <div className="text-center">
                    <div className="text-lg font-bold text-primary">{formatNumber(item.stats.likes)}</div>
                    <div className="text-xs text-muted-foreground">点赞数</div>
                  </div>
                  {item.stats.comments !== undefined && (
                    <>
                      <div className="h-8 w-px bg-border"></div>
                      <div className="text-center">
                        <div className="text-lg font-bold text-primary">{formatNumber(item.stats.comments)}</div>
                        <div className="text-xs text-muted-foreground">评论数</div>
                      </div>
                    </>
                  )}
                </div>
              </CardContent>
            </Card>

          {/* 内容渲染区域 */}
          <ContentRenderer 
            item={item} 
            onImageClick={handleImageClick} 
            isDownloading={isDownloading}
            downloadProgress={downloadProgress}
            onDownload={handleDownload}
            formatFileSize={formatFileSize}
          />

          {/* 互动按钮 */}
            <InteractionButtons
              buttons={[
              createLikeButton(item.stats.likes, isLiked, handleLike),
                createShareButton(handleShare),
                createReportButton(handleReport)
              ]}
              compact={true}
            />

          {/* 相关推荐 */}
          {recommendedItems.length > 0 && (
            <RelatedRecommendations
              title={`相关${item.type === 'resource' ? '资源' : item.type === 'post' ? '帖子' : '公告'}推荐`}
              items={recommendedItems}
              currentItemId={item.id}
              maxItems={3}
              compact={true}
              onMoreClick={() => navigate('/category')}
            />
          )}

            {/* 评论区 */}
            <CommentSection
            comments={comments}
            totalCount={commentTotal}
              onSubmitComment={handleSubmitComment}
            onSubmitReply={handleReplyComment}
              onLoadMoreComments={handleLoadMoreComments}
              hasMoreComments={hasMoreComments}
              isLoadingComments={isLoadingComments}
            placeholder="写下你的看法..."
              maxLength={200}
              initialCommentsToShow={5}
          />
          
          </motion.div>
        </ScrollArea>
      </div> {/* 结束内容区域 */}

      {/* 图片查看器 */}
      {imageViewerOpen && (
        <div className="fixed inset-0 bg-black/90 z-50 flex items-center justify-center">
          <Button
            variant="ghost"
            size="icon"
            className="absolute top-4 right-4 text-white hover:bg-white/20"
            onClick={() => setImageViewerOpen(false)}
          >
            <X size={24} />
          </Button>
          
          <div className="relative w-full h-full flex items-center justify-center p-4">
            <img
              src={imageList[currentImageIndex]}
              alt={`图片 ${currentImageIndex + 1}`}
              className="max-w-full max-h-full object-contain"
            />
            
            {imageList.length > 1 && (
              <>
                <Button
                  variant="ghost"
                  size="icon"
                  className="absolute left-4 top-1/2 -translate-y-1/2 text-white hover:bg-white/20"
                  onClick={() => navigateImage('prev')}
                >
                  <ArrowLeft size={24} />
                </Button>
                <Button
                  variant="ghost"
                  size="icon"
                  className="absolute right-4 top-1/2 -translate-y-1/2 text-white hover:bg-white/20"
                  onClick={() => navigateImage('next')}
                >
                  <ArrowLeft size={24} className="rotate-180" />
                </Button>
                
                <div className="absolute bottom-4 left-1/2 -translate-x-1/2 bg-black/50 text-white px-3 py-1 rounded">
                  {currentImageIndex + 1} / {imageList.length}
          </div>
              </>
            )}
          </div>
          </div>
      )}
          </div>
        )
      }
      
// 内容渲染组件
const ContentRenderer: React.FC<{ 
  item: UniversalDetailItem
  onImageClick?: (images: string[], index: number) => void
  isDownloading?: boolean
  downloadProgress?: number
  onDownload?: () => void
  formatFileSize?: (size: string | number) => string
}> = ({ item, onImageClick, isDownloading = false, downloadProgress = 0, onDownload, formatFileSize }) => {
  switch (item.type) {
    case 'post':
      return (
        <>
          {/* 帖子内容 */}
          {item.content && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">详细说明</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0 overflow-hidden">
                <MarkdownRenderer 
                  content={item.content} 
                  onImageClick={onImageClick}
                  className="text-foreground"
                />
              </CardContent>
            </Card>
          )}

          {/* 帖子图片 */}
          {item.images && item.images.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">相关图片</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="grid grid-cols-2 gap-3">
                  {item.images.map((image, idx) => (
                    <div 
                      key={idx}
                      className="relative group cursor-pointer"
                      onClick={() => onImageClick && onImageClick(item.images!, idx)}
                    >
                      <img 
                      src={image}
                      alt={`图片 ${idx + 1}`}
                        className="rounded-md w-full h-48 object-cover transition-transform group-hover:scale-105"
                    />
                      <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors rounded-md flex items-center justify-center">
                        <ZoomIn className="text-white opacity-0 group-hover:opacity-100 transition-opacity" size={24} />
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}
        </>
      )

    case 'resource':
      return (
        <>
          {/* 资源内容 */}
          {item.description && (
          <Card>
            <CardHeader>
                <CardTitle className="text-lg">详细说明</CardTitle>
            </CardHeader>
              <CardContent className="p-4 pt-0 overflow-hidden">
                <MarkdownRenderer 
                  content={item.description} 
                  onImageClick={onImageClick}
                  className="text-foreground"
                />
            </CardContent>
          </Card>
          )}

          {/* 资源截图 */}
          {item.screenshots && item.screenshots.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">应用截图</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="grid grid-cols-2 gap-3">
                  {item.screenshots.map((screenshot, idx) => (
                    <div 
                      key={idx} 
                      className="relative group cursor-pointer"
                      onClick={() => onImageClick && onImageClick(item.screenshots!, idx)}
                    >
                      <img 
                        src={screenshot} 
                        alt={`截图 ${idx + 1}`} 
                        className="rounded-md w-full h-48 object-cover transition-transform group-hover:scale-105"
                      />
                      <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors rounded-md flex items-center justify-center">
                        <ZoomIn className="text-white opacity-0 group-hover:opacity-100 transition-opacity" size={24} />
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 系统要求 */}
          <Card>
            <CardHeader>
              <CardTitle className="text-base flex items-center">
                <Settings className="h-4 w-4 mr-2" />
                系统要求
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {item.requirements && item.requirements.length > 0 ? (
                  <div className="space-y-2">
                    {item.requirements.map((req, index) => (
                      <div key={index} className="flex items-center gap-2 p-2 bg-muted rounded-lg">
                        <CheckCircle className="h-4 w-4 text-green-500" />
                        <span className="text-sm">{req}</span>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-6 text-muted-foreground">
                    <span className="text-sm">暂无系统要求</span>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>

          {/* 文件列表 */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">包含文件</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              {item.files && item.files.length > 0 ? (
                <div className="space-y-3">
                  {item.files.map((file, idx) => (
                    <div key={idx} className="flex items-center justify-between p-3 bg-muted/50 rounded-lg">
                      <div className="flex items-center min-w-0 flex-1">
                        <FileText size={16} className="text-primary mr-3 flex-shrink-0" />
                        <div className="min-w-0 flex-1">
                          <div className="font-medium text-sm">{file.name}</div>
                          <div className="text-xs text-muted-foreground">
                            {file.type && <span className="mr-2">{file.type}</span>}
                            {file.size && <span>{formatFileSize ? formatFileSize(file.size) : file.size}</span>}
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8 text-muted-foreground">
                  <FileText size={32} className="mx-auto mb-2 opacity-50" />
                  <span className="text-sm">暂无文件信息</span>
                </div>
              )}
            </CardContent>
          </Card>

          {/* 下载按钮 */}
            <Card>
              <CardContent className="p-4">
              {isDownloading ? (
                <div className="space-y-2">
                  <div className="flex items-center justify-center text-sm text-muted-foreground">
                    <Download size={16} className="mr-2 animate-pulse" />
                    下载中... {downloadProgress}%
                </div>
                  <Progress value={downloadProgress} className="w-full" />
                </div>
              ) : (
                <Button 
                  className="w-full" 
                  size="lg"
                  onClick={onDownload}
                >
                  <Download size={18} className="mr-2" />
                  免费下载{item.fileSize ? ` (${formatFileSize ? formatFileSize(item.fileSize) : item.fileSize})` : ''}
                </Button>
              )}
              </CardContent>
            </Card>
        </>
      )

    default:
      return (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">详细说明</CardTitle>
          </CardHeader>
          <CardContent className="p-4 pt-0">
            <MarkdownRenderer 
              content={item.description || item.content || '暂无详细说明'} 
              onImageClick={onImageClick}
              className="text-foreground"
            />
          </CardContent>
        </Card>
      )
  }
}

export default UniversalDetailScreen 