import React, { useState, useEffect, useMemo } from 'react'
import { motion } from 'framer-motion'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  ArrowLeft, Download, Heart, MessageSquare, Share2, Bookmark, 
  MoreHorizontal, Flag, Hash, ThumbsUp, ThumbsDown, 
  Star, FileText, Package, Shield, Calendar, Eye, AlertTriangle,
  ExternalLink, Copy, CheckCircle, Smartphone
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
import { getResource, getResourceComments, createResourceComment, toggleLikeResource, toggleBookmarkResource, reportResource, downloadResource, getResourceLikeStatus, getResourceBookmarkStatus } from '../api/resources'
import { replyComment as apiReplyComment, likeComment as apiLikeComment, getComments as apiGetComments, getCommentReplies as apiGetCommentReplies, updateComment as apiUpdateComment, deleteComment as apiDeleteComment } from '../api/comments'

const ResourceDetailScreen = (): JSX.Element => {
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
  const [commentTotal, setCommentTotal] = useState(0)
  const [isLoadingComments, setIsLoadingComments] = useState(false)

  useEffect(() => {
    const load = async () => {
      try {
        const r = await getResource(parseInt(id || '1'))
        // 初始化点赞/收藏状态
        try {
          const [ls, bs] = await Promise.allSettled([
            getResourceLikeStatus(r.id),
            getResourceBookmarkStatus(r.id)
          ])
          if (ls.status === 'fulfilled') setIsLiked(!!ls.value?.liked)
          if (bs.status === 'fulfilled') setIsBookmarked(!!bs.value?.favorited)
        } catch {}
        
        // 统一处理：获取可展示的文件名
        const extractNameFromUrl = (url?: string) => {
          if (!url) return ''
          const clean = url.split('?')[0].split('#')[0]
          const last = clean.split('/').pop() || ''
          try { return decodeURIComponent(last) } catch { return last }
        }
        const stripHashedPrefix = (name: string) => {
          // 先处理常见前缀：<hash>_<hash>_原名 或 13位时间戳_原名
          let s = name
            .replace(/^([0-9a-fA-F]{8,})_([0-9a-fA-F]{8,})_/, '')
            .replace(/^\d{10,}_/, '')
            .replace(/^[0-9A-Za-z]{8,}_/, '') // 单段哈希前缀
          // 若仍存在类似 <hash>_原名 的结构，按“_”右侧显示
          if (s.includes('_')) {
            const idx = s.indexOf('_')
            const left = s.slice(0, idx)
            if (/^[0-9A-Za-z]{6,}$/.test(left)) s = s.slice(idx + 1)
          }
          return s
        }
        const getDisplayName = (given?: string, fallbackUrl?: string) => {
          const base = (given && given.trim()) ? given : extractNameFromUrl(fallbackUrl)
          return stripHashedPrefix(base)
        }

        // 优先使用服务端提供的包含文件（files 或 included_files），并做字段兼容
        const serverFilesRaw = (r.files && r.files.length ? r.files : (r.included_files || [])) as any[]
        const serverFiles = Array.isArray(serverFilesRaw)
          ? serverFilesRaw.map((f: any) => ({
              name: getDisplayName(f?.name ?? String(f ?? ''), r.file_url),
              type: f?.type ?? f?.file_type ?? '未知',
              size: typeof f?.size === 'number' ? formatFileSize(f.size) : (f?.size || '')
            }))
          : []

        // 基于file_url和描述补充files数组（去重按名称）
        const files: any[] = [...serverFiles]
        if (r.file_url) {
          const fileName = getDisplayName(undefined, r.file_url)
          const fileExtension = (fileName.split('.').pop() || '').toLowerCase()
          let fileType = 'unknown'
          
          // 根据文件扩展名确定类型
          if (['zip', 'rar', '7z', 'tar', 'gz'].includes(fileExtension)) {
            fileType = '压缩包'
          } else if (['exe', 'msi', 'dmg', 'pkg'].includes(fileExtension)) {
            fileType = '安装程序'
          } else if (['apk', 'ipa'].includes(fileExtension)) {
            fileType = '移动应用'
          } else if (['pdf', 'doc', 'docx', 'txt', 'md'].includes(fileExtension)) {
            fileType = '文档'
          }
          
          if (!files.some(f => f.name === fileName)) {
            files.push({
              name: fileName,
              type: fileType,
              size: formatFileSize(r.file_size || 0)
            })
          }
        }

        // 从描述中提取可能的文件信息
        if (r.description) {
          const description = r.description.toLowerCase()
          const commonFiles = [
            { pattern: /readme\.md|说明文档/, name: 'README.md', type: '说明文档', size: '2-5 KB' },
            { pattern: /changelog\.md|更新日志/, name: 'CHANGELOG.md', type: '更新日志', size: '1-3 KB' },
            { pattern: /license|许可证/, name: 'LICENSE', type: '许可证', size: '1 KB' },
            { pattern: /\.java|java文件/, name: 'Java源码', type: 'Java代码', size: '若干文件' },
            { pattern: /\.js|javascript/, name: 'JavaScript文件', type: 'JS代码', size: '若干文件' },
            { pattern: /\.py|python/, name: 'Python文件', type: 'Python代码', size: '若干文件' },
            { pattern: /\.cpp|\.c\+\+|c\+\+/, name: 'C++源码', type: 'C++代码', size: '若干文件' },
            { pattern: /\.xml|配置文件/, name: '配置文件', type: 'XML配置', size: '若干文件' },
            { pattern: /\.json|json文件/, name: 'JSON文件', type: '配置文件', size: '若干文件' },
            { pattern: /\.png|\.jpg|\.jpeg|图片|截图/, name: '图片文件', type: '图片资源', size: '若干文件' },
            { pattern: /\.apk|安卓|android/, name: 'Android APK', type: '安卓应用', size: '若干MB' },
            { pattern: /\.exe|可执行文件/, name: '可执行文件', type: '程序文件', size: '若干MB' }
          ]
          
          commonFiles.forEach(fileInfo => {
            if (fileInfo.pattern.test(description)
                && !files.some(f => f.type === fileInfo.type || f.name === fileInfo.name)) {
              files.push({ name: fileInfo.name, type: fileInfo.type, size: fileInfo.size })
            }
          })
        }

        setResource({
          id: r.id,
          title: r.name || r.title,
          author: { name: r.author_name || r.author || '开发者', avatar: r.author_avatar || '', verified: false },
          description: r.description || '',
          downloadUrl: r.file_url,
          fileSize: (typeof r.file_size === 'number' && r.file_size > 0) ? formatFileSize(r.file_size) : '',
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
          files: files,
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
        const cr = await apiGetComments('resource', r.id, 1, 10, true)
        const mapped = (cr.list || []).map((c: any) => ({
          id: c.id,
          author: { name: c.author_name || '用户', avatar: c.author_avatar || '' },
          content: c.content,
          time: formatTimeOfDay(c.created_at || ''),
          likes: c.likes || 0,
          isLiked: false,
          replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: true })),
          canEdit: true,
        }))
        setComments(mapped)
        setCommentTotal(cr.total || mapped.length)
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
  const formatFileSize = (bytes: string | number) => {
    const size = typeof bytes === 'string' ? parseInt(bytes) : bytes
    if (isNaN(size) || size === 0) return '未知大小'
    
    const units = ['B', 'KB', 'MB', 'GB']
    let unitIndex = 0
    let fileSize = size
    
    while (fileSize >= 1024 && unitIndex < units.length - 1) {
      fileSize /= 1024
      unitIndex++
    }
    
    return `${fileSize.toFixed(unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`
  }

  // 格式化数字
  const formatNumber = (num: number | undefined | null) => {
    if (num == null || isNaN(num)) return '0'
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 时间格式: 仅显示到时:分:秒
  const formatTimeOfDay = (t: any) => {
    try { const d = new Date(t); if (!isNaN(d.getTime())) return d.toLocaleTimeString('zh-CN', { hour12: false }); } catch {}
    const s = String(t || ''); return s.length >= 19 ? s.slice(11, 19) : s
  }

  // 处理下载
  const handleDownload = async () => {
    if (!resource.downloadUrl) {
      toast({ title: '下载失败', description: '下载链接不可用', variant: 'destructive' })
      return
    }

    setIsDownloading(true)
    setDownloadProgress(20)
    
    try {
      // 先尝试获取下载URL
      const url = await downloadResource(resource.id)
      setDownloadProgress(60)
      
      if (url) {
        // 创建一个临时链接来触发下载
        const link = document.createElement('a')
        link.href = url
        link.target = '_blank'
        link.rel = 'noopener noreferrer'
        
        // 尝试从URL中提取文件名
        const fileName = url.split('/').pop() || resource.title || 'download'
        link.download = fileName
        
        document.body.appendChild(link)
        setDownloadProgress(80)
        
        link.click()
        document.body.removeChild(link)
        
        setDownloadProgress(100)
        toast({ title: '下载开始', description: '文件开始下载，请查看浏览器下载管理器' })
      } else {
        throw new Error('无法获取下载链接')
      }
    } catch (e) {
      console.error('Download error:', e)
      const errorMessage = (e as any)?.message || '下载失败，请稍后再试'
      toast({ title: '下载失败', description: errorMessage, variant: 'destructive' })
      
      // 如果API失败，尝试直接使用原始URL
      if (resource.downloadUrl) {
        try {
          const link = document.createElement('a')
          link.href = resource.downloadUrl
          link.target = '_blank'
          link.rel = 'noopener noreferrer'
          document.body.appendChild(link)
          link.click()
          document.body.removeChild(link)
          toast({ title: '使用备用下载', description: '正在使用直链下载' })
        } catch (fallbackError) {
          console.error('Fallback download error:', fallbackError)
        }
      }
    } finally {
      setTimeout(() => {
        setIsDownloading(false)
        setDownloadProgress(0)
      }, 1000)
    }
  }



  // 评论区事件处理
  const handleSubmitComment = async (content: string) => {
    await createResourceComment(resource.id, content)
    const cr = await getResourceComments(resource.id, 1, 10)
    const base = (cr.list || []).map((c: any) => ({ id: c.id, author: { name: c.author_name || '用户', avatar: c.author_avatar || '' }, content: c.content, time: formatTimeOfDay(c.created_at || ''), likes: c.likes || 0, isLiked: false, replies: [] as any[] }))
    const repliesArr = await Promise.all(base.map(c => apiGetCommentReplies(c.id).catch(() => [])))
    const mapped = base.map((c, idx) => ({ ...c, replies: (repliesArr[idx] || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false })) }))
    setComments(mapped)
    setCommentTotal(cr.total || mapped.length)
    toast({ title: '评论发送成功', description: '您的评论已发布' })
  }

  const handleSubmitReply = async (commentId: number, content: string) => {
    await apiReplyComment(commentId, content)
    toast({ title: '回复发送成功', description: '您的回复已发布' })
  }

  // 加载更多评论
  const handleLoadMoreComments = async (page: number): Promise<Comment[]> => {
    setIsLoadingComments(true)
    try {
      const cr = await getResourceComments(resource.id, page, 10)
      const mapped = (cr.list || []).map((c: any) => ({
        id: c.id,
        author: { name: c.author_name || '用户', avatar: c.author_avatar || '' },
        content: c.content,
        time: formatTimeOfDay(c.created_at || ''),
        likes: c.likes || 0,
        isLiked: false,
        replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false }))
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

  const handleEditComment = async (commentId: number, content: string) => {
    await apiUpdateComment(commentId, content)
  }
  const handleDeleteComment = async (commentId: number) => {
    await apiDeleteComment(commentId)
  }

  const handleLikeComment = async (commentId: number) => {
    await apiLikeComment(commentId, true)
    toast({ title: '操作成功', description: '已点赞/取消点赞' })
  }

  const handleReportComment = (commentId: number) => {
    console.log('举报评论:', commentId)
  }

  const editableComments = useMemo(() => comments.map(c => ({ ...c, canEdit: true })), [comments])

  // 处理点赞
  const handleLike = async () => {
    const res = await toggleLikeResource(resource.id)
    setIsLiked(!isLiked)
    // 同步最新计数
    setResource((prev: any) => ({ ...prev, likes: typeof res?.like_count === 'number' ? res.like_count : prev.likes + (isLiked ? -1 : 1) }))
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
            {resource.category && (
              <Badge variant="secondary" className="text-xs mr-2">
                {resource.category}
              </Badge>
            )}
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
          {resource.files.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg flex items-center">
                  <Package size={20} className="mr-2" />
                  包含文件 ({resource.files.length})
                </CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0 content-container">
                <div className="space-y-3">
                  {resource.files.map((file, idx) => (
                    <div key={idx} className="flex items-center justify-between p-3 bg-muted/50 rounded-lg border border-border/50 hover:bg-muted/70 transition-colors">
                      <div className="flex items-center flex-1 min-w-0">
                        {/* 文件类型图标 */}
                        {file.type === '压缩包' && <Package size={18} className="text-orange-500 mr-3 flex-shrink-0" />}
                        {file.type === '安装程序' && <Download size={18} className="text-green-500 mr-3 flex-shrink-0" />}
                        {file.type === '移动应用' && <Smartphone size={18} className="text-blue-500 mr-3 flex-shrink-0" />}
                        {file.type === '文档' && <FileText size={18} className="text-gray-500 mr-3 flex-shrink-0" />}
                        {['Java代码', 'JS代码', 'Python代码', 'C++代码'].includes(file.type) && <FileText size={18} className="text-purple-500 mr-3 flex-shrink-0" />}
                        {['说明文档', '更新日志', '许可证'].includes(file.type) && <FileText size={18} className="text-blue-500 mr-3 flex-shrink-0" />}
                        {['配置文件', 'XML配置'].includes(file.type) && <FileText size={18} className="text-yellow-500 mr-3 flex-shrink-0" />}
                        {file.type === '图片资源' && <FileText size={18} className="text-pink-500 mr-3 flex-shrink-0" />}
                        {!['压缩包', '安装程序', '移动应用', '文档', 'Java代码', 'JS代码', 'Python代码', 'C++代码', '说明文档', '更新日志', '许可证', '配置文件', 'XML配置', '图片资源'].includes(file.type) && 
                          <FileText size={18} className="text-gray-400 mr-3 flex-shrink-0" />}
                        
                        <div className="min-w-0 flex-1">
                          <div className="font-medium text-sm text-overflow-protection truncate">{file.name}</div>
                          <div className="text-xs text-muted-foreground flex items-center">
                            <span className="mr-2">{file.type}</span>
                            {file.size && (
                              <>
                                <span className="w-1 h-1 bg-muted-foreground rounded-full mr-2"></span>
                                <span>{file.size}</span>
                              </>
                            )}
                          </div>
                        </div>
                      </div>
                      {idx === 0 && resource.downloadUrl && (
                        <Badge variant="secondary" className="text-xs ml-2 flex-shrink-0">
                          主文件
                        </Badge>
                      )}
                    </div>
                  ))}
                </div>
                
                {resource.files.length > 1 && (
                  <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-950/20 rounded-lg border border-blue-200 dark:border-blue-800">
                    <div className="flex items-center text-sm text-blue-700 dark:text-blue-300">
                      <AlertTriangle size={16} className="mr-2 flex-shrink-0" />
                      <span>此资源包含多个文件，下载后请解压查看完整内容</span>
                    </div>
                  </div>
                )}
              </CardContent>
            </Card>
          )}

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
                  免费下载{resource.fileSize ? ` (${resource.fileSize})` : ''}
                </Button>
              )}
            </CardContent>
          </Card>



          {/* 操作按钮 */}
          <InteractionButtons
            buttons={[
              createLikeButton(resource.likes + (isLiked ? 1 : 0), isLiked, handleLike),
              createShareButton(handleShare),
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
            comments={editableComments}
            totalCount={commentTotal}
            onSubmitComment={handleSubmitComment}
            onSubmitReply={handleSubmitReply}
            onLikeComment={handleLikeComment}
            onReportComment={handleReportComment}
            onLoadMoreComments={handleLoadMoreComments}
            hasMoreComments={hasMoreComments}
            isLoadingComments={isLoadingComments}
            placeholder="写下你的看法..."
            maxLength={200}
            initialCommentsToShow={5}
            onEditComment={handleEditComment}
            onDeleteComment={handleDeleteComment}
          />
        </div>
      </ScrollArea>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default ResourceDetailScreen 