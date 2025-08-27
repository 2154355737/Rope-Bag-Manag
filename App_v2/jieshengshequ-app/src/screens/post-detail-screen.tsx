import React, { useState, useEffect } from 'react'
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
import { getPost, toggleLikePost, reportPost, getPostLikeStatus } from '../api/posts'
import { http } from '../api/client'
import { getComments as apiGetComments, createComment as apiCreateComment, replyComment as apiReplyComment, likeComment as apiLikeComment, getCommentReplies as apiGetCommentReplies, updateComment as apiUpdateComment, deleteComment as apiDeleteComment } from '../api/comments'
import { getLocalUser } from '../api/auth'
import { getMyComments } from '../api/user'

const PostDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [isLoadingComments, setIsLoadingComments] = useState(false)
  const [allComments, setAllComments] = useState<Comment[]>([])
  const [hasMoreComments, setHasMoreComments] = useState(true)
  const [recommendedItems, setRecommendedItems] = useState<any[]>([])
  const [isLiked, setIsLiked] = useState(false)
  const [isBookmarked, setIsBookmarked] = useState(false)
  const [commentTotal, setCommentTotal] = useState(0)

  // 占位数据
  const initialPost = {
    id: parseInt(id || '1'),
    title: '加载中...',
    author: {
      name: '加载中...',
      avatar: '',
      verified: false,
    },
    content: '正在加载帖子内容...',
    images: [],
    code: undefined,
    tags: [],
    likes: 0,
    comments: 156,
    views: 2340,
    time: '',
    publishDate: '',
    category: ''
  }

  const [post, setPost] = useState(initialPost)

  // 模拟评论数据生成函数
  const generateMockComments = (page: number): Comment[] => {
    const comments: Comment[] = []
    const startId = (page - 1) * 10 + 1
    
    for (let i = 0; i < 10; i++) {
      const commentId = startId + i
      const authors = ['技术专家', '前端开发者', '全栈工程师', '产品经理', '设计师', 'DevOps工程师']
      const contents = [
        '写得非常好，受益匪浅！',
        '这个解决方案很实用，感谢分享。',
        '有个问题想请教，在实际项目中遇到性能瓶颈怎么办？',
        '代码示例很清晰，学到了新技巧。',
        '建议补充一些错误处理的内容。',
        '期待更多这样的高质量内容！'
      ]
      
      comments.push({
        id: commentId,
        author: {
          name: authors[Math.floor(Math.random() * authors.length)],
          avatar: `https://i.pravatar.cc/150?img=${commentId}`,
          verified: Math.random() > 0.7
        },
        content: contents[Math.floor(Math.random() * contents.length)],
        time: `${Math.floor(Math.random() * 24)}小时前`,
        likes: Math.floor(Math.random() * 50),
        isLiked: Math.random() > 0.8,
        replies: []
      })
    }
    
    return comments
  }

  // 时间格式: 仅显示到时:分:秒
  const formatTimeOfDay = (t: any) => {
    try { const d = new Date(t); if (!isNaN(d.getTime())) return d.toLocaleTimeString('zh-CN', { hour12: false }); } catch {}
    const s = String(t || ''); return s.length >= 19 ? s.slice(11, 19) : s
  }

  useEffect(() => {
    // 加载帖子详情和相关数据
    const load = async () => {
      try {
        const p = await getPost(parseInt(id || '1'))
        // 初始化点赞状态
        try {
          const ls = await getPostLikeStatus(p.id)
          setIsLiked(!!ls?.liked)
        } catch {}
        const updatedPost = {
          id: p.id,
          title: p.title || '帖子标题',
          author: { name: p.author?.name || p.author_name || '用户', avatar: p.author?.avatar || '' , verified: false },
          content: p.content || p.title,
          images: Array.isArray(p.images) ? p.images : [],
          code: p.code_snippet || undefined,
          tags: p.tags || [],
          likes: p.like_count || 0,
          comments: p.comment_count || 0,
          views: p.view_count || 0,
          time: new Date(p.created_at || p.publishDate || Date.now()).toLocaleString('zh-CN'),
          publishDate: new Date(p.created_at || p.publishDate || Date.now()).toLocaleDateString('zh-CN'),
          category: p.category?.name || p.category_name || ''
        }
        setPost(updatedPost)
        
        // 加载相关推荐
        const recommendations = await getPostRecommendations(p.id, p.tags || [])
        setRecommendedItems(recommendations)
        
        // 加载第一页评论
        const cr = await apiGetComments('post', p.id, 1, 10, true)
        let mapped: Comment[] = []
        const me = getLocalUser()
        const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
        if ((cr.list || []).length === 0) {
          // 回退到平面列表接口 /posts/{id}/comments
          try {
            const flat = await http.get<{ list: any[]; total: number; page: number; size: number }>(`/posts/${p.id}/comments`, { page: 1, size: 10 })
            // 将平面列表按 parent_id 为空作为顶层，非空归为对应replies
            const parents = (flat.list || []).filter((c: any) => !c.parent_id)
            const replies = (flat.list || []).filter((c: any) => !!c.parent_id)
            mapped = parents.map((c: any) => ({
              id: c.id,
              author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
              content: c.content,
              time: formatTimeOfDay(c.created_at || ''),
              likes: c.likes || 0,
              isLiked: false,
              replies: replies.filter((r: any) => r.parent_id === c.id).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
              canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id),
            }))
            setHasMoreComments(((flat.total || 0) > (flat.page || 1) * (flat.size || 10)))
            setCommentTotal(flat.total || mapped.length)
          } catch {
            // 第三层回退：从我的评论中过滤出当前帖子的评论
            try {
              const mine = await getMyComments({ page: 1, size: 50 })
              const related = (mine.list || []).filter((c: any) => (c.target_type === 'Post' || c.target_type === 'post') && c.target_id === p.id)
              const parents = related.filter((c: any) => !c.parent_id)
              const replies = related.filter((c: any) => !!c.parent_id)
              mapped = parents.map((c: any) => ({
                id: c.id,
                author: { name: c.author_name || c.username || '我', avatar: c.author_avatar || '' },
                content: c.content,
                time: formatTimeOfDay(c.created_at || ''),
                likes: c.likes || 0,
                isLiked: false,
                replies: replies.filter((r: any) => r.parent_id === c.id).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: true })),
                canEdit: true,
              }))
              setHasMoreComments(false)
              setCommentTotal(related.length)
            } catch {}
          }
        } else {
          mapped = (cr.list || []).map((c: any) => ({
            id: c.id,
            author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
            content: c.content,
            time: formatTimeOfDay(c.created_at || ''),
            likes: c.likes || 0,
            isLiked: false,
            replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
            canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id),
          }))
          setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
          setCommentTotal(cr.total || mapped.length)
        }
        setAllComments(mapped)
      } catch (e) {
        console.warn(e)
      }
    }
    load()
  }, [id])

  // 加载更多评论 - 使用真实API
  const handleLoadMoreComments = async (page: number): Promise<Comment[]> => {
    setIsLoadingComments(true)
    
    try {
      const cr = await apiGetComments('post', post.id, page, 10, true)
      const me = getLocalUser(); const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
      const mapped = (cr.list || []).map((c: any) => ({
        id: c.id,
        author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
        content: c.content,
        time: formatTimeOfDay(c.created_at || ''),
        likes: c.likes || 0,
        isLiked: false,
        replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
        canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id),
      }))
      
      setHasMoreComments(((cr.total || 0) > page * (cr.size || 10)))
      setCommentTotal(cr.total || 0)
      setIsLoadingComments(false)
      return mapped
    } catch (error) {
      console.error('加载评论失败:', error)
      // 如果API失败，回退到模拟数据
      await new Promise(resolve => setTimeout(resolve, 1000))
      const newComments = generateMockComments(page)
      if (page >= 15) {
        setHasMoreComments(false)
      }
      setIsLoadingComments(false)
      return newComments
    }
  }

  // 评论区事件处理
  const handleSubmitComment = async (content: string) => {
    try {
      // 记录当前滚动位置
      const currentScrollY = window.scrollY
      
      await apiCreateComment('Post', post.id, content)
      const cr = await apiGetComments('post', post.id, 1, 10, true)
      const mapped = (cr.list || []).map((c: any) => ({
        id: c.id,
        author: { name: c.author_name || '用户', avatar: c.author_avatar || '' },
        content: c.content,
        time: formatTimeOfDay(c.created_at || ''),
        likes: c.likes || 0,
        isLiked: false,
        replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false })),
        canEdit: true,
      }))
      setAllComments(mapped)
      setCommentTotal(cr.total || mapped.length)
      
      // 恢复滚动位置
      setTimeout(() => {
        window.scrollTo(0, currentScrollY)
      }, 100)
      
      toast({ title: '评论发送成功', description: '您的评论已发布' })
    } catch (error) {
      console.error('评论提交失败:', error)
      toast({ title: '评论发送失败', description: '请稍后重试', variant: 'destructive' })
    }
  }

  const handleSubmitReply = async (commentId: number, content: string) => {
    try {
      // 记录当前滚动位置
      const currentScrollY = window.scrollY
      
      await apiReplyComment(commentId, content)
      // 刷新当前第一页以包含最新回复
      const cr = await apiGetComments('post', post.id, 1, 10)
      const base = (cr.list || []).map((c: any) => ({ id: c.id, author: { name: c.author_name || '用户', avatar: c.author_avatar || '' }, content: c.content, time: formatTimeOfDay(c.created_at || ''), likes: c.likes || 0, isLiked: false, replies: [] as any[] }))
      const repliesArr = await Promise.all(base.map(c => apiGetCommentReplies(c.id).catch(() => [])))
      const mapped = base.map((c, idx) => ({ ...c, replies: (repliesArr[idx] || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: true })) }))
      setAllComments(mapped)
      
      // 恢复滚动位置
      setTimeout(() => {
        window.scrollTo(0, currentScrollY)
      }, 100)
      
      toast({ title: '回复发送成功', description: '您的回复已发布' })
    } catch (error) {
      console.error('回复提交失败:', error)
      toast({ title: '回复发送失败', description: '请稍后重试', variant: 'destructive' })
    }
  }

  const handleEditComment = async (commentId: number, content: string) => {
    await apiUpdateComment(commentId, content)
    // 编辑后刷新第一页评论
    try {
      const cr = await apiGetComments('post', post.id, 1, 10, true)
      const me = getLocalUser(); const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
      const mapped = (cr.list || []).map((c: any) => ({
        id: c.id,
        author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
        content: c.content,
        time: formatTimeOfDay(c.created_at || ''),
        likes: c.likes || 0,
        isLiked: false,
        replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
        canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id),
      }))
      setAllComments(mapped)
      setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
      setCommentTotal(cr.total || mapped.length)
    } catch {}
  }
  const handleDeleteComment = async (commentId: number) => {
    await apiDeleteComment(commentId)
    // 重新拉取第一页评论，保持与后端一致
    try {
      const cr = await apiGetComments('post', post.id, 1, 10, true)
      const me = getLocalUser(); const isPrivileged = (role?: string) => (role === 'admin' || role === 'elder')
      let mapped: Comment[] = []
      if ((cr.list || []).length === 0) {
        try {
          const flat = await http.get<{ list: any[]; total: number; page: number; size: number }>(`/posts/${post.id}/comments`, { page: 1, size: 10 })
          const parents = (flat.list || []).filter((c: any) => !c.parent_id)
          const replies = (flat.list || []).filter((c: any) => !!c.parent_id)
          mapped = parents.map((c: any) => ({
            id: c.id,
            author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
            content: c.content,
            time: formatTimeOfDay(c.created_at || ''),
            likes: c.likes || 0,
            isLiked: false,
            replies: replies.filter((r: any) => r.parent_id === c.id).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
            canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id),
          }))
          setHasMoreComments(((flat.total || 0) > (flat.page || 1) * (flat.size || 10)))
          setCommentTotal(flat.total || mapped.length)
        } catch {
          try {
            const mine = await getMyComments({ page: 1, size: 50 })
            const related = (mine.list || []).filter((c: any) => (c.target_type === 'Post' || c.target_type === 'post') && c.target_id === post.id)
            const parents = related.filter((c: any) => !c.parent_id)
            const replies = related.filter((c: any) => !!c.parent_id)
            mapped = parents.map((c: any) => ({
              id: c.id,
              author: { name: c.author_name || c.username || '我', avatar: c.author_avatar || '' },
              content: c.content,
              time: formatTimeOfDay(c.created_at || ''),
              likes: c.likes || 0,
              isLiked: false,
              replies: replies.filter((r: any) => r.parent_id === c.id).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: true })),
              canEdit: true,
            }))
            setHasMoreComments(false)
            setCommentTotal(related.length)
          } catch {}
        }
      } else {
        mapped = (cr.list || []).map((c: any) => ({
          id: c.id,
          author: { name: c.author_name || c.username || '用户', avatar: c.author_avatar || '' },
          content: c.content,
          time: formatTimeOfDay(c.created_at || ''),
          likes: c.likes || 0,
          isLiked: false,
          replies: (c.replies || []).map((r: any) => ({ id: r.id, author: { name: r.author_name || r.username || '用户', avatar: r.author_avatar || '' }, content: r.content, time: formatTimeOfDay(r.created_at || ''), likes: r.likes || 0, isLiked: false, canEdit: !!me && (isPrivileged(me.role) || me.id === r.user_id) })),
          canEdit: !!me && (isPrivileged(me.role) || me.id === c.user_id),
        }))
        setHasMoreComments(((cr.total || 0) > (cr.page || 1) * (cr.size || 10)))
        setCommentTotal(cr.total || mapped.length)
      }
      setAllComments(mapped)
    } catch {}
  }

  const handleLikeComment = async (commentId: number) => {
    await apiLikeComment(commentId, true)
    toast({ title: '操作成功', description: '已点赞/取消点赞' })
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
  const handleLike = async () => {
    const res = await toggleLikePost(post.id)
    setIsLiked(!isLiked)
    // 同步计数
    setPost((prev: any) => ({ ...prev, likes: typeof res?.like_count === 'number' ? res.like_count : prev.likes + (isLiked ? -1 : 1) }))
    toast({ title: isLiked ? '已取消点赞' : '点赞成功', description: isLiked ? '已取消对此帖子的点赞' : '感谢您的支持', duration: 2000 })
  }

  // 收藏暂不开发
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
    await reportPost(post.id)
    toast({ title: '举报已提交', description: '我们会尽快处理您的举报', duration: 2000 })
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="帖子详情"
        showBackButton
        rightAction={
          <div className="flex items-center gap-1">
            {post.category && (
              <Badge variant="secondary" className="text-xs mr-2">
                {post.category}
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
                {post.content && post.content.trim() ? (
                  post.content.split('\n').map((line, idx) => {
                    const trimmedLine = line.trim()
                    if (!trimmedLine) return <div key={idx} className="h-2" />
                    
                    // 检查是否是URL
                    if (trimmedLine.startsWith('http://') || trimmedLine.startsWith('https://')) {
                      return (
                        <div key={idx} className="break-all">
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
                    
                    // 检查是否是列表项
                    if (trimmedLine.startsWith('•') || trimmedLine.startsWith('-') || trimmedLine.startsWith('*')) {
                      return (
                        <div key={idx} className="ml-4">
                          <span className="text-sm break-words">
                            • {trimmedLine.substring(1).trim()}
                          </span>
                        </div>
                      )
                    }
                    
                    // 普通文本段落
                    return (
                      <p key={idx} className="text-sm break-words whitespace-pre-wrap">
                        {trimmedLine}
                      </p>
                    )
                  })
                ) : (
                  <p className="text-sm text-muted-foreground">暂无内容</p>
                )}
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
            totalCount={commentTotal}
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
            onEditComment={handleEditComment}
            onDeleteComment={handleDeleteComment}
          />
        </div>
      </ScrollArea>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default PostDetailScreen