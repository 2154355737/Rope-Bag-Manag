import React, { useState, useEffect, useMemo } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { motion } from 'framer-motion'
import { 
  ArrowLeft, Share2, MoreHorizontal, Download, 
  Heart, MessageSquare, Eye, Star, CheckCircle, Shield, Hash,
  FileText, ChevronDown, Loader2, Calendar, User, Tag, X, ZoomIn
} from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Progress } from '@/components/ui/progress'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { ScrollArea } from '@/components/ui/scroll-area'
import { toast } from '@/hooks/use-toast'

import TopNavigation from '@/components/ui/top-navigation'
import CommentSection, { Comment } from '@/components/comment-section'
import RelatedRecommendations from '@/components/related-recommendations'
import InteractionButtons, { 
  createLikeButton, 
  createShareButton, 
  createReportButton 
} from '@/components/ui/interaction-buttons'

// API imports
import { getPost, toggleLikePost, reportPost, getPostLikeStatus } from '../api/posts'
import { getResource, downloadResource, toggleLikeResource, getResourceLikeStatus, reportResource } from '../api/resources'
import { getAnnouncement } from '../api/announcements'
import { getComments as apiGetComments, createComment as apiCreateComment, replyComment as apiReplyComment, likeComment as apiLikeComment } from '../api/comments'
import { getLocalUser } from '../api/auth'
import { getPostRecommendations, getResourceRecommendations, getAnnouncementRecommendations } from '@/utils/recommendations'

// 通用详情项目类型
type ItemType = 'post' | 'resource' | 'announcement'

// 通用详情数据接口
interface UniversalDetailItem {
  id: number
  type: ItemType
  title: string
  author: {
    name: string
    avatar: string
    verified?: boolean
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
  code?: string
  
  // 公告特有字段
  priority?: 'low' | 'medium' | 'high'
  validUntil?: string
}

const UniversalDetailScreen: React.FC = () => {
  const navigate = useNavigate()
  const { id } = useParams<{ id: string }>()
  
  // 从当前路径推断类型
  const getCurrentType = (): ItemType => {
    const path = window.location.pathname
    if (path.includes('/post/')) return 'post'
    if (path.includes('/resource/')) return 'resource'
    if (path.includes('/announcement/')) return 'announcement'
    return 'post' // 默认值
  }
  
  const type = getCurrentType()
  
  const [item, setItem] = useState<UniversalDetailItem | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<{ type: 'forbidden' | 'not-found' | 'general', message: string } | null>(null)
  const [comments, setComments] = useState<Comment[]>([])
  const [hasMoreComments, setHasMoreComments] = useState(false)
  const [recommendedItems, setRecommendedItems] = useState<any[]>([])
  
  // 图片查看器状态
  const [imageViewerOpen, setImageViewerOpen] = useState(false)
  const [currentImageIndex, setCurrentImageIndex] = useState(0)
  const [imageList, setImageList] = useState<string[]>([])
  
  // 交互状态
  const [isLiked, setIsLiked] = useState(false)
  const [isBookmarked, setIsBookmarked] = useState(false)
  const [isDownloading, setIsDownloading] = useState(false)
  const [downloadProgress, setDownloadProgress] = useState(0)
  const [commentTotal, setCommentTotal] = useState(0)
  const [isLoadingComments, setIsLoadingComments] = useState(false)

  // 格式化数字
  const formatNumber = (num: number | undefined | null) => {
    if (num == null || isNaN(num)) return '0'
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }
  const humanFileSize = (size?: string) => {
    if (!size) return ''
    // 如果是纯数字字符串，转成 KB/MB
    const n = parseInt(size)
    if (!isNaN(n) && n > 0) {
      return n >= 1024 * 1024 ? `${(n / (1024 * 1024)).toFixed(1)} MB` : `${Math.max(1, Math.round(n / 1024))} KB`
    }
    return size
  }

  // 时间格式: 仅显示到时:分:秒
  const formatTimeOfDay = (t: any) => {
    try { const d = new Date(t); if (!isNaN(d.getTime())) return d.toLocaleTimeString('zh-CN', { hour12: false }); } catch {}
    const s = String(t || ''); return s.length >= 19 ? s.slice(11, 19) : s
  }

  // 获取页面标题
  const getPageTitle = () => {
    switch (type) {
      case 'post': return '帖子详情'
      case 'resource': return '资源详情'
      case 'announcement': return '公告详情'
      default: return '详情'
    }
  }

  // 获取数据
  useEffect(() => {
    const loadData = async () => {
      if (!type || !id) return
      
      try {
        setLoading(true)
        let data: any
        let recommendations: any[] = []
        
        // 统一处理：可展示文件名生成
        const extractNameFromUrl = (url?: string) => {
          if (!url) return ''
          const clean = url.split('?')[0].split('#')[0]
          const last = clean.split('/').pop() || ''
          try { return decodeURIComponent(last) } catch { return last }
        }
        const stripHashedPrefix = (name: string) => name
          .replace(/^([0-9a-fA-F]{8,})_([0-9a-fA-F]{8,})_/, '')
          .replace(/^\d{10,}_/, '')
          .replace(/^[0-9A-Za-z]{8,}_/, '')
          .replace(/^[0-9A-Za-z]{6,}_(.+)$/,(m, p1)=>m) // 占位，不改变值
          // 若还有 <hash>_原名 的形式，保留“_”右边
          .replace(/^([0-9A-Za-z]{6,})_(.+)$/,'$2')
        const getDisplayName = (given?: string, fallbackUrl?: string) => {
          const base = (given && given.trim()) ? given : extractNameFromUrl(fallbackUrl)
          return stripHashedPrefix(base)
        }

        switch (type) {
          case 'post':
            data = await getPost(parseInt(id))
            recommendations = await getPostRecommendations(data.id, data.tags || [])
            setItem({
              id: data.id,
              type: 'post',
              title: data.title || data.content?.substring(0, 50) + '...',
              author: { 
                name: data.author_detail?.name || data.author_name || (typeof data.author === 'string' ? data.author : data.author?.name) || '用户', 
                avatar: data.author_detail?.avatar || data.author_avatar || (typeof data.author === 'object' ? data.author?.avatar : '') || '', 
                verified: (typeof data.author === 'object' ? data.author?.verified : false) || false 
              },
              content: data.content,
              tags: data.tags || [],
              stats: {
                likes: data.like_count || 0,
                comments: data.comment_count || 0,
                views: data.view_count || 0
              },
              publishDate: new Date(data.created_at || Date.now()).toLocaleDateString('zh-CN'),
              images: data.images || [],
              code: data.code
            })
            // 初始化点赞状态
            try { const ls = await getPostLikeStatus(data.id); setIsLiked(!!ls?.liked) } catch {}
            break
            
          case 'resource':
            data = await getResource(parseInt(id))
            recommendations = await getResourceRecommendations(data.id, data.tags || [])
            // 兼容后端：将 included_files 正常化为 files
            const normalizedFiles = Array.isArray(data.files) && data.files.length
              ? data.files
              : (Array.isArray(data.included_files) ? data.included_files : [])
            const mapped = normalizedFiles.map((f: any) => ({
              name: getDisplayName(f?.name ?? String(f ?? ''), data.file_url),
              type: f?.type ?? f?.file_type ?? '未知',
              size: typeof f?.size === 'number' ? `${f.size}` : (f?.size || '')
            }))
            // 回退主文件
            const fallback: Array<{ name: string; type: string; size: string }> = []
            if ((!mapped || mapped.length === 0) && data.file_url) {
              const fileName = getDisplayName(undefined, data.file_url)
              const ext = fileName.split('.').pop()?.toLowerCase() || ''
              let fileType = '未知'
              if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) fileType = '压缩包'
              else if (['exe', 'msi', 'dmg', 'pkg'].includes(ext)) fileType = '安装程序'
              else if (['apk', 'ipa'].includes(ext)) fileType = '移动应用'
              else if (['pdf', 'doc', 'docx', 'txt', 'md'].includes(ext)) fileType = '文档'
              const humanSize = typeof data.file_size === 'number' && data.file_size > 0
                ? (data.file_size >= 1024 * 1024
                    ? `${(data.file_size / (1024 * 1024)).toFixed(1)} MB`
                    : `${Math.max(1, Math.round(data.file_size / 1024))} KB`)
                : ''
              fallback.push({ name: fileName, type: fileType, size: humanSize })
            }
            const nameSet = new Set(mapped.map(f => f.name))
            const files = mapped.concat(fallback.filter(f => !nameSet.has(f.name)))
            setItem({
              id: data.id,
              type: 'resource',
              title: data.name || data.title,
              author: { 
                name: data.author_detail?.name || data.author_name || (typeof data.author === 'string' ? data.author : data.author?.name) || '开发者', 
                avatar: data.author_detail?.avatar || data.author_avatar || (typeof data.author === 'object' ? data.author?.avatar : '') || '', 
                verified: (typeof data.author === 'object' ? data.author?.verified : false) || false 
              },
              description: data.description || '',
              tags: data.tags || [],
              stats: {
                likes: data.like_count || 0,
                comments: data.comment_count || 0,
                views: data.view_count || 0,
                downloads: data.download_count || 0,
                rating: data.rating || 4.5
              },
              publishDate: new Date(data.created_at || Date.now()).toLocaleDateString('zh-CN'),
              category: data.category?.name || '其他',
              version: data.version || '1.0.0',
              fileSize: data.file_size?.toString() || '0',
              downloadUrl: data.file_url,
              files,
              requirements: data.requirements || [],
              screenshots: data.screenshots || [],
              safetyStatus: data.safety_status || 'unknown'
            })
            // 初始化点赞状态
            try { const ls = await getResourceLikeStatus(data.id); setIsLiked(!!ls?.liked) } catch {}
            break
            
          case 'announcement':
            data = await getAnnouncement(parseInt(id))
            recommendations = await getAnnouncementRecommendations(data.id, data.tags || [])
            setItem({
              id: data.id,
              type: 'announcement',
              title: data.title,
              author: { 
                name: (typeof data.author === 'string' ? data.author : data.author?.name) || '系统公告', 
                avatar: (typeof data.author === 'object' ? data.author?.avatar : '') || '', 
                verified: (typeof data.author === 'object' ? data.author?.verified : true) || true 
              },
              content: data.content,
              tags: data.tags || [],
              stats: {
                likes: 0,
                comments: data.comment_count || 0,
                views: data.view_count || 0
              },
              publishDate: new Date(data.created_at || Date.now()).toLocaleDateString('zh-CN'),
              priority: data.priority || 'medium',
              validUntil: data.valid_until
            })
            break
        }
        
        setRecommendedItems(recommendations)
        
        // 加载评论
        const targetType = type === 'resource' ? 'package' : type
        const cr = await apiGetComments(targetType as any, parseInt(id), 1, 10, true)
        const me = getLocalUser(); const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
        const mapped = (cr.list || []).map((c: any) => ({
          id: c.id,
          author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
          content: c.content,
          time: formatTimeOfDay(c.created_at || ''),
          likes: c.likes || 0,
          isLiked: false,
          replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
          canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id)
        }))
        setComments(mapped)
        setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
        setCommentTotal(cr.total || mapped.length)
        
      } catch (error: any) {
        console.error('加载详情失败:', error)
        
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

  // 交互事件处理
  const handleLike = async () => {
    if (!item) return
    try {
      if (item.type === 'resource') {
        const res = isLiked ? await (await import('../api/resources')).unlikeResource(item.id) : await (await import('../api/resources')).likeResource(item.id)
        setIsLiked(!isLiked)
        setItem(prev => prev ? ({ ...prev, stats: { ...prev.stats, likes: typeof (res as any)?.like_count === 'number' ? (res as any).like_count : prev.stats.likes + (isLiked ? -1 : 1) } }) : prev)
      } else if (item.type === 'post') {
        const res = isLiked ? await (await import('../api/posts')).unlikePost(item.id) : await toggleLikePost(item.id)
        setIsLiked(!isLiked)
        setItem(prev => prev ? ({ ...prev, stats: { ...prev.stats, likes: typeof (res as any)?.like_count === 'number' ? (res as any).like_count : prev.stats.likes + (isLiked ? -1 : 1) } }) : prev)
      } else {
        // 公告暂不支持点赞，做前端提示
        toast({ title: '暂不支持', description: '公告暂不支持点赞', duration: 2000 })
        return
      }
      toast({ title: isLiked ? '已取消点赞' : '点赞成功', description: isLiked ? '已取消点赞' : '感谢您的支持', duration: 2000 })
    } catch (e) {
      toast({ title: '操作失败', description: '请稍后重试', variant: 'destructive' })
    }
  }

  // 收藏暂不开发，去掉按钮

  const handleShare = () => {
    try {
      const url = window.location.href
      if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(url)
      }
      toast({ title: '分享链接已复制', description: '可以分享给更多朋友了', duration: 2000 })
    } catch {
      toast({ title: '分享', description: '请手动复制地址栏链接', duration: 2000 })
    }
  }

  const handleReport = async () => {
    if (!item) return
    try {
      if (item.type === 'resource') {
        await reportResource(item.id)
      } else if (item.type === 'post') {
        await reportPost(item.id)
      } else {
        toast({ title: '已收到', description: '公告的反馈已记录', duration: 2000 })
        return
      }
      toast({ title: '举报已提交', description: '我们会尽快处理您的举报', duration: 2000 })
    } catch (e) {
      toast({ title: '举报失败', description: '请稍后重试', variant: 'destructive' })
    }
  }

  const handleDownload = async () => {
    if (item?.type !== 'resource') return
    setIsDownloading(true)
    setDownloadProgress(15)
    try {
      // 调用后端获取真实下载地址
      const url = await downloadResource(item.id)
      setDownloadProgress(60)
      const finalUrl = typeof url === 'string' ? url : (url as any)?.url || item.downloadUrl
      if (!finalUrl) throw new Error('下载链接不可用')

      // 触发浏览器下载
      const a = document.createElement('a')
      a.href = finalUrl
      a.target = '_blank'
      a.rel = 'noopener noreferrer'
      const fileName = (finalUrl.split('/').pop() || item.title || 'download')
      a.download = fileName
      document.body.appendChild(a)
      setDownloadProgress(85)
      a.click()
      document.body.removeChild(a)
      setDownloadProgress(100)
      toast({ title: '下载开始', description: '文件已开始下载', duration: 2500 })
    } catch (e) {
      // 回退：若API失败，尝试直接用原始URL
      try {
        if (item.downloadUrl) {
          const a = document.createElement('a')
          a.href = item.downloadUrl
          a.target = '_blank'
          a.rel = 'noopener noreferrer'
          document.body.appendChild(a)
          a.click()
          document.body.removeChild(a)
          toast({ title: '使用备用下载', description: '正在使用直链下载', duration: 2500 })
        } else {
          throw new Error('无可用下载链接')
        }
      } catch (fallbackError) {
        toast({ title: '下载失败', description: '请检查网络或稍后重试', variant: 'destructive' })
      }
    } finally {
      setTimeout(() => { setIsDownloading(false); setDownloadProgress(0) }, 800)
    }
  }

  // 图片点击处理
  const handleImageClick = (images: string[], index: number) => {
    setImageList(images)
    setCurrentImageIndex(index)
    setImageViewerOpen(true)
  }

  // 图片查看器导航
  const navigateImage = (direction: 'prev' | 'next') => {
    if (direction === 'prev') {
      setCurrentImageIndex(prev => prev > 0 ? prev - 1 : imageList.length - 1)
    } else {
      setCurrentImageIndex(prev => prev < imageList.length - 1 ? prev + 1 : 0)
    }
  }

  const handleSubmitComment = async (content: string) => {
    if (!item) return
    
    try {
      const targetType = item.type === 'resource' ? 'Package' : 
                        item.type === 'post' ? 'Post' : 'Announcement'
      await apiCreateComment(targetType as any, item.id, content)
      
      // 重新加载评论
      const cr = await apiGetComments(item.type === 'resource' ? 'package' : item.type as any, item.id, 1, 10, true)
      const me = getLocalUser(); const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
      const mapped = (cr.list || []).map((c: any) => ({
        id: c.id,
        author: { name: c.author_name || '用户', avatar: c.author_avatar || '' },
        content: c.content,
        time: formatTimeOfDay(c.created_at || ''),
        likes: c.likes || 0,
        isLiked: false,
        replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
        canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id)
      }))
      setComments(mapped)
      setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
      setCommentTotal(cr.total || mapped.length)
      
      toast({ title: '评论发送成功', description: '您的评论已发布' })
    } catch (error) {
      toast({ title: '评论发送失败', description: '请稍后重试', variant: 'destructive' })
    }
  }

  const handleSubmitReply = async (commentId: number, content: string) => {
    try {
      await apiReplyComment(commentId, content)
      // 重新加载评论
      const targetType = type === 'resource' ? 'package' : type
      const cr = await apiGetComments(targetType as any, parseInt(id!), 1, 10, true)
      const me2 = getLocalUser(); const isPriv2 = (role?: string) => (role === 'admin' || role === 'elder')
      const updated = (cr.list || []).map((c: any) => ({ id: c.id, author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' }, content: c.content, time: formatTimeOfDay(c.created_at || ''), likes: c.likes || 0, isLiked: false, replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me2 && (isPriv2(me2.role) || me2.id === r.user_id) })), canEdit: !!me2 && (isPriv2(me2.role) || me2.id === c.user_id) }))
      setComments(updated)
      setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
      setCommentTotal(cr.total || updated.length)
      toast({ title: '回复发送成功', description: '您的回复已发布' })
    } catch (error) {
      toast({ title: '回复发送失败', description: '请稍后重试', variant: 'destructive' })
    }
  }

  // 加载更多评论
  const handleLoadMoreComments = async (page: number): Promise<Comment[]> => {
    setIsLoadingComments(true)
    try {
      const targetType = type === 'resource' ? 'package' : type
      const cr = await apiGetComments(targetType as any, parseInt(id!), page, 10, true)
      const me = getLocalUser(); const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
      const mapped = (cr.list || []).map((c: any) => ({
        id: c.id,
        author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
        content: c.content,
        time: formatTimeOfDay(c.created_at || ''),
        likes: c.likes || 0,
        isLiked: false,
        replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
        canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id)
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
    try {
      await apiLikeComment(commentId, true)
      toast({ title: '点赞成功' })
    } catch (error) {
      toast({ title: '点赞失败', description: '请稍后重试', variant: 'destructive' })
    }
  }

  const editableComments = useMemo(() => comments, [comments])

  if (loading) {
    return (
      <div className="flex flex-col min-h-screen bg-background">
        <TopNavigation title={getPageTitle()} showBackButton />
        <div className="flex-1 flex items-center justify-center">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      </div>
    )
  }

  if (!item) {
    return (
      <div className="flex flex-col min-h-screen bg-background">
        <TopNavigation title={getPageTitle()} showBackButton />
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <h2 className="text-lg font-medium mb-2">内容不存在</h2>
            <p className="text-muted-foreground mb-4">该内容可能已被删除或不存在</p>
            <Button onClick={() => navigate(-1)}>返回上一页</Button>
          </div>
        </div>
      </div>
    )
  }

  // 调试检查：确保author是正确的对象结构
  if (item.author && typeof item.author !== 'object') {
    console.error('Author is not an object:', item.author)
    return (
      <div className="flex flex-col min-h-screen bg-background">
        <TopNavigation title={getPageTitle()} showBackButton />
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <h2 className="text-lg font-medium mb-2">数据格式错误</h2>
            <p className="text-muted-foreground mb-4">作者信息格式不正确</p>
            <Button onClick={() => navigate(-1)}>返回上一页</Button>
          </div>
        </div>
      </div>
    )
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

      {/* 内容区域 */}
      <div className="pt-nav">
        <ScrollArea className="flex-1">
          <div className="p-4 space-y-4 content-container">
            
            {/* 基本信息 */}
            <Card>
              <CardContent className="p-4">
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-center flex-1 min-w-0">
                    <Avatar className="h-12 w-12 mr-3 flex-shrink-0">
                                              <AvatarImage src={item.author?.avatar || ''} />
                      <AvatarFallback>{item.author?.name?.[0] || 'U'}</AvatarFallback>
                    </Avatar>
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center">
                        <span className="font-medium text-overflow-protection truncate">{item.author?.name || '用户'}</span>
                        {item.author?.verified && (
                          <CheckCircle size={16} className="ml-1 text-blue-500 flex-shrink-0" />
                        )}
                      </div>
                      <div className="text-xs text-muted-foreground">
                        发布于 {item.publishDate}
                      </div>
                    </div>
                  </div>
                  {item.category && (
                    <Badge variant="secondary" className="text-xs flex-shrink-0 ml-2">
                      {item.category}
                    </Badge>
                  )}
                  {item.priority && (
                    <Badge 
                      variant={item.priority === 'high' ? 'destructive' : item.priority === 'medium' ? 'default' : 'secondary'} 
                      className="text-xs flex-shrink-0 ml-2"
                    >
                      {item.priority === 'high' ? '重要' : item.priority === 'medium' ? '普通' : '一般'}
                    </Badge>
                  )}
                </div>

                <h2 className="text-xl font-bold mb-2 text-overflow-protection">{item.title}</h2>
                
                {/* 版本信息（仅资源） */}
                {item.type === 'resource' && item.version && (
                  <div className="flex items-center mb-3">
                    <Badge variant="outline" className="text-xs mr-2">
                      {item.version}
                    </Badge>
                    <div className="flex items-center">
                      <Star size={14} className="text-yellow-500 mr-1" />
                      <span className="text-sm font-medium">{item.stats.rating}</span>
                      <span className="text-xs text-muted-foreground ml-1">(0 评价)</span>
                    </div>
                  </div>
                )}

                {/* 安全状态（仅资源） */}
                {item.type === 'resource' && (
                  <Alert className="mb-4">
                    <Shield size={16} />
                    <AlertDescription>
                      此资源已通过安全检测，可放心下载使用
                    </AlertDescription>
                  </Alert>
                )}

                {/* 标签 */}
                {item.tags.length > 0 && (
                  <div className="flex flex-wrap gap-2">
                    {item.tags.map((tag, idx) => (
                      <Badge key={idx} variant="outline" className="text-xs">
                        <Hash size={10} className="mr-1" /> {tag}
                      </Badge>
                    ))}
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

            {/* 内容详情渲染 */}
            <ContentRenderer item={item} onImageClick={handleImageClick} />

            {/* 下载按钮（仅资源） */}
            {item.type === 'resource' && (
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
                      免费下载{humanFileSize(item.fileSize) ? ` (${humanFileSize(item.fileSize)})` : ''}
                    </Button>
                  )}
                </CardContent>
              </Card>
            )}

            {/* 操作按钮 */}
            <InteractionButtons
              buttons={[
                createLikeButton(item.stats.likes + (isLiked ? 1 : 0), isLiked, handleLike),
                createShareButton(handleShare),
                createReportButton(handleReport)
              ]}
              compact={true}
            />

            {/* 评论区 */}
            <CommentSection
              comments={editableComments}
              totalCount={commentTotal || item.stats.comments}
              onSubmitComment={handleSubmitComment}
              onSubmitReply={handleSubmitReply}
              onLikeComment={handleLikeComment}
              onReportComment={(commentId) => console.log('举报评论:', commentId)}
              onLoadMoreComments={handleLoadMoreComments}
              hasMoreComments={hasMoreComments}
              isLoadingComments={isLoadingComments}
              placeholder="发表评论..."
              maxLength={200}
              initialCommentsToShow={5}
              onEditComment={async (commentId, content) => {
                const m = await import('../api/comments'); await m.updateComment(commentId, content)
              }}
              onDeleteComment={async (commentId) => {
                const m = await import('../api/comments'); await m.deleteComment(commentId)
              }}
            />

            {/* 相关推荐 */}
            <RelatedRecommendations
              title={`相关${item.type === 'post' ? '帖子' : item.type === 'resource' ? '资源' : '公告'}推荐`}
              items={recommendedItems}
              currentItemId={item.id}
              maxItems={3}
              showMoreButton={true}
              compact={true}
              onMoreClick={() => navigate('/category')}
            />
          </div>
        </ScrollArea>
      </div>

      {/* 图片查看器模态框 */}
      {imageViewerOpen && (
        <div className="fixed inset-0 bg-black/90 z-50 flex items-center justify-center">
          <div className="relative max-w-full max-h-full p-4">
            {/* 关闭按钮 */}
            <Button
              variant="ghost"
              size="icon"
              className="absolute top-4 right-4 z-10 bg-black/50 hover:bg-black/70 text-white"
              onClick={() => setImageViewerOpen(false)}
            >
              <X size={24} />
            </Button>

            {/* 图片计数 */}
            {imageList.length > 1 && (
              <div className="absolute top-4 left-4 z-10 bg-black/50 text-white px-3 py-1 rounded-md text-sm">
                {currentImageIndex + 1} / {imageList.length}
              </div>
            )}

            {/* 主图片 */}
            <img
              src={imageList[currentImageIndex]}
              alt={`图片 ${currentImageIndex + 1}`}
              className="max-w-full max-h-full object-contain"
              onClick={(e) => e.stopPropagation()}
            />

            {/* 导航按钮 */}
            {imageList.length > 1 && (
              <>
                <Button
                  variant="ghost"
                  size="icon"
                  className="absolute left-4 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 text-white"
                  onClick={() => navigateImage('prev')}
                >
                  <ArrowLeft size={24} />
                </Button>
                <Button
                  variant="ghost"
                  size="icon"
                  className="absolute right-4 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 text-white"
                  onClick={() => navigateImage('next')}
                >
                  <ArrowLeft size={24} className="rotate-180" />
                </Button>
              </>
            )}
          </div>

          {/* 点击背景关闭 */}
          <div 
            className="absolute inset-0 -z-10" 
            onClick={() => setImageViewerOpen(false)}
          />
        </div>
      )}
    </div>
  )
}

// 内容渲染组件
const ContentRenderer: React.FC<{ 
  item: UniversalDetailItem
  onImageClick?: (images: string[], index: number) => void 
}> = ({ item, onImageClick }) => {
  const renderContent = (content: string) => {
    // 检查是否是HTML内容
    const isHtmlContent = content.includes('<') && content.includes('>')
    
    if (isHtmlContent) {
      // 对于HTML内容，使用dangerouslySetInnerHTML但需要清理
      const sanitizedHtml = content
        .replace(/<script[^>]*>.*?<\/script>/gi, '') // 移除script标签
        .replace(/javascript:/gi, '') // 移除javascript: 协议
        .replace(/on\w+\s*=/gi, '') // 移除事件处理器
      
      return (
        <div 
          className="prose prose-sm max-w-none dark:prose-invert 
                     prose-headings:text-foreground prose-p:text-foreground 
                     prose-strong:text-foreground prose-em:text-foreground
                     prose-code:text-foreground prose-code:bg-muted prose-code:px-1 prose-code:rounded
                     prose-pre:bg-muted prose-pre:border prose-pre:rounded-lg
                     prose-blockquote:border-l-primary prose-blockquote:text-muted-foreground
                     prose-a:text-primary hover:prose-a:text-primary/80
                     prose-ul:text-foreground prose-ol:text-foreground prose-li:text-foreground"
          dangerouslySetInnerHTML={{ __html: sanitizedHtml }}
        />
      )
    }

    // 对于纯文本内容，解析简单的Markdown语法
    return content.split('\n').map((line, idx) => {
      const trimmedLine = line.trim()
      if (!trimmedLine) return <div key={idx} className="h-3" />
      
      // Markdown标题 (### ## #)
      if (trimmedLine.startsWith('###')) {
        return (
          <h3 key={idx} className="text-lg font-semibold text-foreground mt-6 mb-3">
            {trimmedLine.substring(3).trim()}
          </h3>
        )
      }
      if (trimmedLine.startsWith('##')) {
        return (
          <h2 key={idx} className="text-xl font-semibold text-foreground mt-6 mb-4">
            {trimmedLine.substring(2).trim()}
          </h2>
        )
      }
      if (trimmedLine.startsWith('#')) {
        return (
          <h1 key={idx} className="text-2xl font-bold text-foreground mt-6 mb-4">
            {trimmedLine.substring(1).trim()}
          </h1>
        )
      }
      
      // Markdown代码块 (```)
      if (trimmedLine.startsWith('```')) {
        const language = trimmedLine.substring(3).trim()
        return (
          <div key={idx} className="my-4 p-4 bg-muted rounded-lg border">
            {language && (
              <div className="text-xs text-muted-foreground mb-2 font-mono">{language}</div>
            )}
            <code className="text-sm font-mono text-foreground block">
              {/* 这里应该收集后续行直到遇到结束的``` */}
              代码块开始...
            </code>
          </div>
        )
      }
      
      // Markdown引用 (>)
      if (trimmedLine.startsWith('>')) {
        return (
          <blockquote key={idx} className="border-l-4 border-primary pl-4 my-3 text-muted-foreground italic">
            {trimmedLine.substring(1).trim()}
          </blockquote>
        )
      }
      
      // Markdown列表 (- 或 *)
      if (trimmedLine.startsWith('- ') || trimmedLine.startsWith('* ')) {
        return (
          <div key={idx} className="flex items-start my-1">
            <span className="text-primary mr-2">•</span>
            <span className="text-sm text-foreground">
              {renderInlineMarkdown(trimmedLine.substring(2).trim())}
            </span>
          </div>
        )
      }
      
      // Markdown数字列表 (1. 2. 等)
      if (/^\d+\.\s/.test(trimmedLine)) {
        const match = trimmedLine.match(/^(\d+)\.\s(.*)/)
        if (match) {
          return (
            <div key={idx} className="flex items-start my-1">
              <span className="text-primary mr-2 font-mono text-sm">{match[1]}.</span>
              <span className="text-sm text-foreground">
                {renderInlineMarkdown(match[2])}
              </span>
            </div>
          )
        }
      }
      
      // 内联代码 (`code`)
      if (trimmedLine.includes('`')) {
        return (
          <p key={idx} className="text-sm text-foreground leading-relaxed my-2">
            {renderInlineMarkdown(trimmedLine)}
          </p>
        )
      }
      
      // URL检测
      if (trimmedLine.startsWith('http://') || trimmedLine.startsWith('https://')) {
        return (
          <div key={idx} className="my-3">
            <a 
              href={trimmedLine} 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-sm text-primary hover:text-primary/80 underline break-all"
            >
              {trimmedLine}
            </a>
          </div>
        )
      }
      
      // 普通段落
      return (
        <p key={idx} className="text-sm text-foreground leading-relaxed my-2">
          {renderInlineMarkdown(trimmedLine)}
        </p>
      )
    })
  }

  // 渲染内联Markdown语法
  const renderInlineMarkdown = (text: string): React.ReactNode => {
    // **粗体**
    text = text.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    // *斜体*
    text = text.replace(/\*(.*?)\*/g, '<em>$1</em>')
    // `代码`
    text = text.replace(/`(.*?)`/g, '<code class="bg-muted px-1 rounded text-sm font-mono">$1</code>')
    // [链接](url)
    text = text.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener noreferrer" class="text-primary hover:text-primary/80 underline">$1</a>')
    
    return <span dangerouslySetInnerHTML={{ __html: text }} />
  }

  switch (item.type) {
    case 'post':
      return (
        <>
          {/* 帖子内容 */}
          {item.content && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">帖子内容</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0 content-container">
                <div className="space-y-3 text-overflow-protection">
                  {renderContent(item.content)}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 代码展示 */}
          {item.code && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">代码示例</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <pre className="bg-muted p-4 rounded-md overflow-x-auto text-sm">
                  <code>{item.code}</code>
                </pre>
              </CardContent>
            </Card>
          )}

          {/* 图片展示 */}
          {item.images && item.images.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">图片</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="grid grid-cols-1 gap-3">
                  {item.images.map((image, idx) => (
                    <div 
                      key={idx} 
                      className="relative group cursor-pointer"
                      onClick={() => onImageClick?.(item.images!, idx)}
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
          {/* 截图展示 */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">预览截图</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              {item.screenshots && item.screenshots.length > 0 ? (
                <div className="grid grid-cols-1 gap-3">
                  {item.screenshots.map((screenshot, idx) => (
                    <div 
                      key={idx} 
                      className="relative group cursor-pointer"
                      onClick={() => onImageClick?.(item.screenshots!, idx)}
                    >
                      <img
                        src={screenshot}
                        alt={`Screenshot ${idx + 1}`}
                        className="rounded-md w-full h-48 object-cover transition-transform group-hover:scale-105"
                      />
                      <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors rounded-md flex items-center justify-center">
                        <ZoomIn className="text-white opacity-0 group-hover:opacity-100 transition-opacity" size={24} />
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8 text-muted-foreground">
                  <span className="text-sm">暂无预览截图</span>
                </div>
              )}
            </CardContent>
          </Card>

          {/* 详细描述 */}
          {item.description && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">详细介绍</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0 content-container">
                <div className="space-y-3 text-overflow-protection">
                  {renderContent(item.description)}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 系统要求 */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">系统要求</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0 content-container">
              {item.requirements && item.requirements.length > 0 ? (
                <ul className="space-y-2">
                  {item.requirements.map((req, idx) => (
                    <li key={idx} className="flex items-start text-sm">
                      <CheckCircle size={14} className="text-green-500 mr-2 mt-0.5 flex-shrink-0" />
                      <span className="text-overflow-protection">{req}</span>
                    </li>
                  ))}
                </ul>
              ) : (
                <div className="text-center py-8 text-muted-foreground">
                  <span className="text-sm">暂无系统要求</span>
                </div>
              )}
            </CardContent>
          </Card>

          {/* 文件列表 */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">包含文件</CardTitle>
            </CardHeader>
            <CardContent className="p-4 pt-0 content-container">
              {item.files && item.files.length > 0 ? (
                <div className="space-y-3">
                  {item.files.map((file, idx) => (
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
              ) : (
                <div className="text-center py-8 text-muted-foreground">
                  <span className="text-sm">暂无文件信息</span>
                </div>
              )}
            </CardContent>
          </Card>
        </>
      )

    case 'announcement':
      return (
        <>
          {/* 公告内容 */}
          {item.content && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">公告内容</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0 content-container">
                <div className="space-y-3 text-overflow-protection">
                  {renderContent(item.content)}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 有效期 */}
          {item.validUntil && (
            <Card>
              <CardContent className="p-4">
                <div className="flex items-center text-sm text-muted-foreground">
                  <Calendar size={16} className="mr-2" />
                  有效期至：{new Date(item.validUntil).toLocaleDateString('zh-CN')}
                </div>
              </CardContent>
            </Card>
          )}
        </>
      )

    default:
      return null
  }
}

export default UniversalDetailScreen 