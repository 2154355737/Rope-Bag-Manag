import React, { useState, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useNavigate, useSearchParams } from 'react-router-dom'
import { 
  ArrowLeft, Search, Filter, MoreVertical, Edit3, Trash2, 
  Eye, EyeOff, Heart, MessageSquare, Download, Share2,
  Plus, Calendar, Tag, BookOpen, FileText, Clock,
  CheckCircle, XCircle, AlertCircle, Pause
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { 
  DropdownMenu, DropdownMenuContent, DropdownMenuItem, 
  DropdownMenuTrigger, DropdownMenuSeparator 
} from '@/components/ui/dropdown-menu'
import { 
  Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription,
  DialogFooter
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'
import { getMyResources, getMyPosts, getMyComments } from '@/api/user'
import { deletePost, updatePostStatus } from '@/api/posts'
import { deleteResource, updateResourceStatus } from '@/api/resources'
import { deleteComment } from '@/api/comments'

const MyContentScreen: React.FC = () => {
  const navigate = useNavigate()
  const [searchParams] = useSearchParams()
  const [activeTab, setActiveTab] = useState('resources')
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedItems, setSelectedItems] = useState<number[]>([])
  const [showDeleteDialog, setShowDeleteDialog] = useState(false)
  const [deleteTarget, setDeleteTarget] = useState<{ type: string; id: number; title: string } | null>(null)

  // 数据状态
  const [isLoading, setIsLoading] = useState(false)
  const [myResources, setMyResources] = useState<any[]>([])
  const [myPosts, setMyPosts] = useState<any[]>([])
  const [myComments, setMyComments] = useState<any[]>([])

  // 根据URL参数设置初始tab
  useEffect(() => {
    const tab = searchParams.get('tab')
    if (tab && ['resources', 'posts', 'comments'].includes(tab)) {
      setActiveTab(tab)
    }
  }, [searchParams])

  // 加载数据
  const loadData = async () => {
    try {
      setIsLoading(true)
      
      const [resources, posts, comments] = await Promise.all([
        getMyResources().catch((err) => { console.error('获取资源失败:', err); return { list: [] } }),
        getMyPosts().catch((err) => { console.error('获取帖子失败:', err); return { list: [] } }),
        getMyComments().catch((err) => { console.error('获取评论失败:', err); return { list: [] } })
      ])
      
      setMyResources(resources?.list || [])
      setMyPosts(posts?.list || [])
      setMyComments(comments?.list || [])
    } catch (error) {
      console.error('加载数据失败:', error)
      toast({
        title: "加载失败",
        description: "无法加载内容数据，请稍后重试",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }

  // 组件挂载时加载数据
  useEffect(() => {
    loadData()
  }, [])

  // 获取状态信息
  const getStatusInfo = (status: string) => {
    // 统一转换为小写进行比较
    const normalizedStatus = status?.toLowerCase()
    
    switch (normalizedStatus) {
      case 'published':
      case 'active':
        return { 
          text: '已发布', 
          		className: 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200',
          icon: CheckCircle 
        }
      case 'draft':
        return { 
          text: '草稿', 
          className: 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200',
          icon: FileText 
        }
      case 'reviewing':
      case 'pending':
        return { 
          text: '审核中', 
          className: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200',
          icon: Clock 
        }
      case 'rejected':
      case 'inactive':
        return { 
          text: '已拒绝', 
          className: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200',
          icon: XCircle 
        }
      default:
        return { 
          text: '未知', 
          className: 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200',
          icon: AlertCircle 
        }
    }
  }

  // 处理编辑
  const handleEdit = (type: string, id: number) => {
    if (type === 'post') {
      navigate(`/publish?type=post&id=${id}`)
    } else if (type === 'resource') {
      navigate(`/publish?type=resource&id=${id}`)
    } else if (type === 'comment') {
      toast({
        title: "暂不支持",
        description: "评论编辑功能正在开发中",
        variant: "destructive"
      })
    }
  }

  // 处理删除
  const handleDelete = (type: string, id: number, title: string) => {
    setDeleteTarget({ type, id, title })
    setShowDeleteDialog(true)
  }

  // 确认删除
  const confirmDelete = async () => {
    if (!deleteTarget) return
    
    try {
      const { type, id, title } = deleteTarget
      
      if (type === 'post') {
        await deletePost(id)
        setMyPosts(prev => prev.filter(item => item.id !== id))
      } else if (type === 'resource') {
        await deleteResource(id)
        setMyResources(prev => prev.filter(item => item.id !== id))
      } else if (type === 'comment') {
        await deleteComment(id)
        setMyComments(prev => prev.filter(item => item.id !== id))
      }
      
      toast({
        title: "删除成功",
        description: `${title} 已被删除`
      })
    } catch (error) {
      console.error('删除失败:', error)
      toast({
        title: "删除失败",
        description: "操作失败，请稍后重试",
        variant: "destructive"
      })
    } finally {
      setShowDeleteDialog(false)
      setDeleteTarget(null)
    }
  }

  // 处理状态变更
  const handleStatusChange = async (type: string, id: number, newStatus: string) => {
    try {
      if (type === 'post') {
        await updatePostStatus(id, newStatus)
        setMyPosts(prev => prev.map(item => 
          item.id === id ? { ...item, status: newStatus } : item
        ))
      } else if (type === 'resource') {
        await updateResourceStatus(id, newStatus)
        setMyResources(prev => prev.map(item => 
          item.id === id ? { ...item, status: newStatus } : item
        ))
      }
      
      toast({
        title: "状态已更新",
        description: `内容状态已更改为: ${getStatusInfo(newStatus).text}`
      })
    } catch (error) {
      console.error('状态更新失败:', error)
      toast({
        title: "更新失败",
        description: "状态更新失败，请稍后重试",
        variant: "destructive"
      })
    }
  }

  // 处理批量操作
  const handleBatchAction = (action: string) => {
    if (selectedItems.length === 0) {
      toast({
        title: "请选择内容",
        description: "请先选择要操作的内容",
        variant: "destructive"
      })
      return
    }

    toast({
      title: "批量操作成功",
      description: `已对 ${selectedItems.length} 项内容执行 ${action} 操作`
    })
    setSelectedItems([])
  }

  // 过滤内容
  const filterContent = (items: any[]) => {
    if (!searchQuery) return items
    return items.filter(item => {
      const title = item.title || item.name || ''
      const content = item.content || item.description || ''
      const tags = item.tags || []
      
      return title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        content.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (Array.isArray(tags) && tags.some((tag: string) => tag.toLowerCase().includes(searchQuery.toLowerCase())))
    })
  }

  // 渲染资源卡片
  const renderResourceCard = (resource: any) => {
    const statusInfo = getStatusInfo(resource.status)
    const StatusIcon = statusInfo.icon

    return (
      <motion.div
        key={resource.id}
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.3 }}
        className="cursor-pointer"
        onClick={() => navigate(`/resource/${resource.id}`)}
      >
        <Card className="overflow-hidden hover:shadow-lg transition-shadow">
          <div className="relative">
            {/* 使用默认图标，因为后端资源数据没有图片字段 */}
            <div className="w-full h-32 bg-gradient-to-br from-primary/10 to-primary/5 flex items-center justify-center">
              <BookOpen size={32} className="text-primary/60" />
            </div>
            <div className="absolute top-2 right-2">
              <Badge className={`text-xs ${statusInfo.className}`}>
                <StatusIcon size={10} className="mr-1" />
                {statusInfo.text}
              </Badge>
            </div>
          </div>
          
          <CardContent className="p-4">
            <div className="flex items-start justify-between mb-2">
              <h3 className="font-medium text-sm line-clamp-2 flex-1 min-w-0 pr-2">{resource.name}</h3>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon" className="h-6 w-6 shrink-0" onClick={(e) => e.stopPropagation()}>
                    <MoreVertical size={12} />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  {/* 只有草稿和已发布状态可以编辑 */}
                  {(resource.status === 'draft' || resource.status === 'published' || resource.status === 'Active') && (
                    <DropdownMenuItem onClick={(e) => { e.stopPropagation(); handleEdit('resource', resource.id) }}>
                      <Edit3 size={14} className="mr-2" />
                      编辑
                    </DropdownMenuItem>
                  )}
                  <DropdownMenuItem onClick={(e) => { e.stopPropagation(); navigate(`/resource/${resource.id}`) }}>
                    <Eye size={14} className="mr-2" />
                    查看
                  </DropdownMenuItem>
                  {(resource.status === 'published' || resource.status === 'Active') && (
                    <DropdownMenuItem onClick={(e) => { e.stopPropagation(); handleStatusChange('resource', resource.id, 'draft') }}>
                      <EyeOff size={14} className="mr-2" />
                      下架
                    </DropdownMenuItem>
                  )}
                  {resource.status === 'draft' && (
                    <DropdownMenuItem onClick={(e) => { e.stopPropagation(); handleStatusChange('resource', resource.id, 'reviewing') }}>
                      <CheckCircle size={14} className="mr-2" />
                      提交审核
                    </DropdownMenuItem>
                  )}
                  <DropdownMenuSeparator />
                  <DropdownMenuItem 
                    onClick={(e) => { e.stopPropagation(); handleDelete('resource', resource.id, resource.name) }}
                    className="text-destructive"
                  >
                    <Trash2 size={14} className="mr-2" />
                    删除
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

            <p className="text-xs text-muted-foreground mb-3 line-clamp-3 break-words">
              {resource.description}
            </p>

            {resource.tags && (
            <div className="flex flex-wrap gap-1 mb-3">
              {resource.tags.slice(0, 2).map((tag: string, index: number) => (
                <Badge key={index} variant="outline" className="text-xs">
                  {tag}
                </Badge>
              ))}
              {resource.tags.length > 2 && (
                <Badge variant="outline" className="text-xs">+{resource.tags.length - 2}</Badge>
              )}
            </div>
            )}

            <div className="flex items-center justify-between text-xs text-muted-foreground">
              <span className="truncate">{resource.version}</span>
              <div className="flex items-center gap-3 shrink-0">
                <div className="flex items-center">
                  <Download size={10} className="mr-1" />
                  {resource.download_count || 0}
                </div>
                <div className="flex items-center">
                  <Heart size={10} className="mr-1" />
                  {resource.like_count || 0}
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </motion.div>
    )
  }

  // 渲染帖子卡片
  const renderPostCard = (post: any) => {
    const statusInfo = getStatusInfo(post.status)
    const StatusIcon = statusInfo.icon

    return (
      <motion.div
        key={post.id}
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.3 }}
        className="cursor-pointer"
        onClick={() => navigate(`/post/${post.id}`)}
      >
        <Card className="hover:shadow-lg transition-shadow overflow-hidden">
          <CardContent className="p-4">
            <div className="flex items-start gap-3 w-full">
              {/* 使用默认图标，因为后端帖子数据没有图片字段 */}
              <div className="w-16 h-16 rounded-lg bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 flex items-center justify-center shrink-0">
                <FileText size={24} className="text-blue-500/60" />
              </div>
              
              <div className="flex-1 min-w-0 overflow-hidden w-0">
                <div className="flex items-start justify-between mb-2 gap-2">
                  <h3 className="font-medium text-sm line-clamp-2 flex-1 min-w-0 break-words">{post.title}</h3>
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" size="icon" className="h-6 w-6 shrink-0" onClick={(e) => e.stopPropagation()}>
                        <MoreVertical size={12} />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      {/* 只有草稿和已发布状态可以编辑 */}
                      {(post.status === 'draft' || post.status === 'Draft' || post.status === 'published' || post.status === 'Published') && (
                        <DropdownMenuItem onClick={(e) => { e.stopPropagation(); handleEdit('post', post.id) }}>
                          <Edit3 size={14} className="mr-2" />
                          编辑
                        </DropdownMenuItem>
                      )}
                      <DropdownMenuItem onClick={(e) => { e.stopPropagation(); navigate(`/post/${post.id}`) }}>
                        <Eye size={14} className="mr-2" />
                        查看
                      </DropdownMenuItem>
                      {(post.status === 'published' || post.status === 'Published') && (
                        <DropdownMenuItem onClick={(e) => { e.stopPropagation(); handleStatusChange('post', post.id, 'draft') }}>
                          <EyeOff size={14} className="mr-2" />
                          下架
                        </DropdownMenuItem>
                      )}
                      {(post.status === 'draft' || post.status === 'Draft') && (
                        <DropdownMenuItem onClick={(e) => { e.stopPropagation(); handleStatusChange('post', post.id, 'reviewing') }}>
                          <CheckCircle size={14} className="mr-2" />
                          提交审核
                        </DropdownMenuItem>
                      )}
                      <DropdownMenuSeparator />
                      <DropdownMenuItem 
                        onClick={(e) => { e.stopPropagation(); handleDelete('post', post.id, post.title) }}
                        className="text-destructive"
                      >
                        <Trash2 size={14} className="mr-2" />
                        删除
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>

                <div className="flex items-center gap-2 mb-2">
                  <Badge className={`text-xs ${statusInfo.className}`}>
                    <StatusIcon size={10} className="mr-1" />
                    {statusInfo.text}
                  </Badge>
                </div>

                <div className="mb-3 w-full overflow-hidden">
                  <p className="text-xs text-muted-foreground line-clamp-2 break-all leading-relaxed w-full max-w-full overflow-hidden text-ellipsis">
                    {post.content && post.content.length > 100 ? post.content.substring(0, 100) + '...' : post.content}
                  </p>
                </div>

                <div className="flex items-center justify-between text-xs text-muted-foreground gap-2 flex-wrap">
                  <span className="truncate text-xs">{new Date(post.created_at).toLocaleDateString()}</span>
                  <div className="flex items-center gap-2 shrink-0 text-xs">
                    <div className="flex items-center gap-1">
                      <Eye size={10} />
                      <span>{post.view_count || post.views || post.views_count || 0}</span>
                    </div>
                    <div className="flex items-center gap-1">
                      <Heart size={10} />
                      <span>{post.like_count || 0}</span>
                    </div>
                    <div className="flex items-center gap-1">
                      <MessageSquare size={10} />
                      <span>{post.comment_count || 0}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </motion.div>
    )
  }

  // 渲染评论卡片
  const renderCommentCard = (comment: any) => {
    return (
      <motion.div
        key={comment.id}
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.3 }}
        className="cursor-pointer"
        onClick={() => {
          // 根据评论的目标类型跳转到对应页面
          const targetId = comment.target_id || comment.post_id || comment.resource_id
          const targetType = comment.target_type || comment.type || 
                           (comment.post_id ? 'post' : comment.resource_id ? 'resource' : 'post')
          
          if (targetId) {
            if (targetType === 'post' || targetType === 'Post') {
              navigate(`/post/${targetId}`)
            } else if (targetType === 'resource' || targetType === 'Resource') {
              navigate(`/resource/${targetId}`)
            } else {
              // 默认按帖子处理
              navigate(`/post/${targetId}`)
            }
          } else {
            console.warn('评论数据缺少目标ID:', comment)
          }
        }}
      >
        <Card className="hover:shadow-lg transition-shadow">
          <CardContent className="p-4">
            <div className="flex items-start justify-between mb-2 gap-2">
              <h4 className="text-sm font-medium text-primary line-clamp-2 flex-1 min-w-0">
                {comment.target_title || '相关帖子'}
              </h4>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon" className="h-6 w-6 shrink-0" onClick={(e) => e.stopPropagation()}>
                    <MoreVertical size={12} />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={(e) => { e.stopPropagation(); handleEdit('comment', comment.id) }}>
                    <Edit3 size={14} className="mr-2" />
                    编辑
                  </DropdownMenuItem>
                  <DropdownMenuItem 
                    onClick={(e) => { e.stopPropagation(); handleDelete('comment', comment.id, comment.content.slice(0, 20) + '...') }}
                    className="text-destructive"
                  >
                    <Trash2 size={14} className="mr-2" />
                    删除
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

            <p className="text-sm text-muted-foreground mb-3 line-clamp-4 break-words">
              {comment.content}
            </p>

            <div className="flex items-center justify-between text-xs text-muted-foreground gap-2">
              <span className="truncate">我的评论</span>
              <div className="flex items-center gap-3 shrink-0">
                <span className="whitespace-nowrap">{new Date(comment.created_at).toLocaleDateString()}</span>
                <div className="flex items-center">
                  <Heart size={10} className="mr-1" />
                  {comment.likes || 0}
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </motion.div>
    )
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航 */}
      <TopNavigation
        title="我的内容"
        showBackButton
        onBackClick={() => navigate(-1)}
        rightAction={
          <Button variant="ghost" size="icon" onClick={() => navigate('/publish')}>
            <Plus size={20} />
          </Button>
        }
      />

      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        <ScrollArea className="flex-1">
          <div className="p-4 max-w-full overflow-hidden">
            {/* 搜索栏 */}
            <div className="flex items-center gap-2 mb-4">
              <div className="relative flex-1 min-w-0">
                <Search size={16} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                <Input
                  placeholder="搜索我的内容..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-10"
                />
              </div>
              <Button variant="outline" size="icon" className="shrink-0">
                <Filter size={16} />
              </Button>
            </div>

            {/* 标签页 */}
            <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-4">
              <TabsList className="grid w-full grid-cols-3">
                <TabsTrigger value="resources" className="flex items-center gap-2">
                  <BookOpen size={16} />
                  <span className="hidden sm:inline">我的资源</span>
                  <span className="sm:hidden">资源</span>
                </TabsTrigger>
                <TabsTrigger value="posts" className="flex items-center gap-2">
                  <FileText size={16} />
                  <span className="hidden sm:inline">我的帖子</span>
                  <span className="sm:hidden">帖子</span>
                </TabsTrigger>
                <TabsTrigger value="comments" className="flex items-center gap-2">
                  <MessageSquare size={16} />
                  <span className="hidden sm:inline">我的评论</span>
                  <span className="sm:hidden">评论</span>
                </TabsTrigger>
              </TabsList>

              {/* 我的资源 */}
              <TabsContent value="resources" className="space-y-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg font-medium">我的资源 ({myResources.length})</h3>
                  <Button size="sm" onClick={() => navigate('/publish?type=resource')}>
                    <Plus size={14} className="mr-1" />
                    发布资源
                  </Button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {filterContent(myResources).map(renderResourceCard)}
                </div>

                {filterContent(myResources).length === 0 && (
                  <div className="text-center py-12">
                    <BookOpen size={48} className="mx-auto text-muted-foreground mb-4" />
                    <p className="text-muted-foreground">
                      {searchQuery ? '没有找到匹配的资源' : '还没有发布过资源'}
                    </p>
                    <Button className="mt-4" onClick={() => navigate('/publish?type=resource')}>
                      <Plus size={16} className="mr-2" />
                      发布第一个资源
                    </Button>
                  </div>
                )}
              </TabsContent>

              {/* 我的帖子 */}
              <TabsContent value="posts" className="space-y-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg font-medium">我的帖子 ({myPosts.length})</h3>
                  <Button size="sm" onClick={() => navigate('/publish?type=post')}>
                    <Plus size={14} className="mr-1" />
                    发布帖子
                  </Button>
                </div>

                <div className="space-y-4 max-w-full overflow-hidden">
                  {filterContent(myPosts).map(renderPostCard)}
                </div>

                {filterContent(myPosts).length === 0 && (
                  <div className="text-center py-12">
                    <FileText size={48} className="mx-auto text-muted-foreground mb-4" />
                    <p className="text-muted-foreground">
                      {searchQuery ? '没有找到匹配的帖子' : '还没有发布过帖子'}
                    </p>
                    <Button className="mt-4" onClick={() => navigate('/publish?type=post')}>
                      <Plus size={16} className="mr-2" />
                      发布第一篇帖子
                    </Button>
                  </div>
                )}
              </TabsContent>

              {/* 我的评论 */}
              <TabsContent value="comments" className="space-y-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg font-medium">我的评论 ({myComments.length})</h3>
                </div>

                <div className="space-y-4">
                  {filterContent(myComments).map(renderCommentCard)}
                </div>

                {filterContent(myComments).length === 0 && (
                  <div className="text-center py-12">
                    <MessageSquare size={48} className="mx-auto text-muted-foreground mb-4" />
                    <p className="text-muted-foreground">
                      {searchQuery ? '没有找到匹配的评论' : '还没有发表过评论'}
                    </p>
                    <Button className="mt-4" onClick={() => navigate('/community')}>
                      <MessageSquare size={16} className="mr-2" />
                      去社区看看
                    </Button>
                  </div>
                )}
              </TabsContent>
            </Tabs>
          </div>
        </ScrollArea>
      </div>

      {/* 删除确认对话框 */}
      <Dialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>确认删除</DialogTitle>
            <DialogDescription>
              您确定要删除 "{deleteTarget?.title}" 吗？此操作无法撤销。
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowDeleteDialog(false)}>
              取消
            </Button>
            <Button variant="destructive" onClick={confirmDelete}>
              确认删除
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}

export default MyContentScreen 