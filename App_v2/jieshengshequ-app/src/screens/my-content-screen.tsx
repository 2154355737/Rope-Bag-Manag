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

const MyContentScreen: React.FC = () => {
  const navigate = useNavigate()
  const [searchParams] = useSearchParams()
  const [activeTab, setActiveTab] = useState('resources')
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedItems, setSelectedItems] = useState<number[]>([])
  const [showDeleteDialog, setShowDeleteDialog] = useState(false)
  const [deleteTarget, setDeleteTarget] = useState<{ type: string; id: number; title: string } | null>(null)

  // 根据URL参数设置初始tab
  useEffect(() => {
    const tab = searchParams.get('tab')
    if (tab && ['resources', 'posts', 'comments'].includes(tab)) {
      setActiveTab(tab)
    }
  }, [searchParams])

  // 模拟我的资源数据
  const myResources = [
    {
      id: 1,
      title: 'React Native 开发工具包',
      description: '一个功能强大的React Native开发工具包...',
      category: '开发工具',
      image: 'https://images.unsplash.com/photo-1512941937669-90a1b58e7e9c?w=300',
      status: 'published', // published, draft, reviewing, rejected
      downloads: 1250,
      likes: 89,
      comments: 45,
      createdAt: '2024-12-01',
      updatedAt: '2025-01-10',
      version: 'v2.1.0',
      tags: ['React Native', 'TypeScript', '开发工具']
    },
    {
      id: 2,
      title: 'Vue3 组件库',
      description: '基于Vue3和TypeScript的现代化组件库...',
      category: '组件库',
      image: 'https://images.unsplash.com/photo-1551650975-87deedd944c3?w=300',
      status: 'draft',
      downloads: 0,
      likes: 12,
      comments: 3,
      createdAt: '2025-01-15',
      updatedAt: '2025-01-15',
      version: 'v1.0.0',
      tags: ['Vue3', 'TypeScript', '组件库']
    },
    {
      id: 3,
      title: 'Python数据分析脚本',
      description: '用于数据分析和可视化的Python脚本集合...',
      category: '数据分析',
      image: 'https://images.unsplash.com/photo-1563013544-824ae1b704d3?w=300',
      status: 'reviewing',
      downloads: 45,
      likes: 23,
      comments: 8,
      createdAt: '2025-01-10',
      updatedAt: '2025-01-12',
      version: 'v1.2.0',
      tags: ['Python', '数据分析', '可视化']
    }
  ]

  // 模拟我的帖子数据
  const myPosts = [
    {
      id: 1,
      title: '结绳语言移动应用开发经验分享',
      content: '分享一些在结绳语言移动应用开发过程中的经验和踩过的坑...',
      image: 'https://images.unsplash.com/photo-1551033406-611cf9a28f67?w=300',
      status: 'published',
      likes: 234,
      comments: 45,
      views: 1250,
      createdAt: '2025-01-14',
      updatedAt: '2025-01-14',
      tags: ['移动开发', '项目分享', '经验总结']
    },
    {
      id: 2,
      title: 'TypeScript最佳实践总结',
      content: 'TypeScript在大型项目中的应用最佳实践...',
      image: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=300',
      status: 'draft',
      likes: 0,
      comments: 0,
      views: 0,
      createdAt: '2025-01-16',
      updatedAt: '2025-01-16',
      tags: ['TypeScript', '最佳实践', '开发经验']
    },
    {
      id: 3,
      title: 'React Hooks进阶使用技巧',
      content: 'React Hooks的高级使用技巧和性能优化...',
      image: 'https://images.unsplash.com/photo-1633356122544-f134324a6cee?w=300',
      status: 'reviewing',
      likes: 156,
      comments: 32,
      views: 890,
      createdAt: '2025-01-12',
      updatedAt: '2025-01-13',
      tags: ['React', 'Hooks', '性能优化']
    }
  ]

  // 模拟我的评论数据
  const myComments = [
    {
      id: 1,
      content: '这个工具包真的很实用！特别是TypeScript的支持，文档也很详细。',
      postTitle: 'React Native 最新开发指南',
      postAuthor: '技术大牛',
      likes: 12,
      replies: 3,
      createdAt: '2025-01-15 14:30',
      status: 'published'
    },
    {
      id: 2,
      content: '同意楼主的观点，我在实际项目中也遇到了类似的问题，这个解决方案确实有效。',
      postTitle: '前端性能优化最佳实践',
      postAuthor: '前端小王',
      likes: 8,
      replies: 1,
      createdAt: '2025-01-14 09:15',
      status: 'published'
    },
    {
      id: 3,
      content: '请问这个方法在Vue3中是否也适用？期待作者的回复。',
      postTitle: '状态管理方案对比分析',
      postAuthor: '架构师李四',
      likes: 5,
      replies: 0,
      createdAt: '2025-01-13 16:45',
      status: 'published'
    }
  ]

  // 获取状态信息
  const getStatusInfo = (status: string) => {
    switch (status) {
      case 'published':
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
        return { 
          text: '审核中', 
          className: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200',
          icon: Clock 
        }
      case 'rejected':
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
    toast({
      title: "跳转编辑页面",
      description: `正在编辑${type} ID: ${id}`
    })
    // 这里应该导航到对应的编辑页面
    // navigate(`/edit-${type}/${id}`)
  }

  // 处理删除
  const handleDelete = (type: string, id: number, title: string) => {
    setDeleteTarget({ type, id, title })
    setShowDeleteDialog(true)
  }

  // 确认删除
  const confirmDelete = () => {
    if (deleteTarget) {
      toast({
        title: "删除成功",
        description: `${deleteTarget.title} 已被删除`
      })
      setShowDeleteDialog(false)
      setDeleteTarget(null)
    }
  }

  // 处理状态变更
  const handleStatusChange = (type: string, id: number, newStatus: string) => {
    toast({
      title: "状态已更新",
      description: `内容状态已更改为: ${getStatusInfo(newStatus).text}`
    })
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
    return items.filter(item => 
      item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (item.content && item.content.toLowerCase().includes(searchQuery.toLowerCase())) ||
      (item.tags && item.tags.some((tag: string) => tag.toLowerCase().includes(searchQuery.toLowerCase())))
    )
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
      >
        <Card className="overflow-hidden hover:shadow-lg transition-shadow">
          <div className="relative">
            <img 
              src={resource.image} 
              alt={resource.title}
              className="w-full h-32 object-cover"
            />
            <div className="absolute top-2 right-2">
              <Badge className={`text-xs ${statusInfo.className}`}>
                <StatusIcon size={10} className="mr-1" />
                {statusInfo.text}
              </Badge>
            </div>
          </div>
          
          <CardContent className="p-4">
            <div className="flex items-start justify-between mb-2">
              <h3 className="font-medium text-sm line-clamp-1">{resource.title}</h3>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon" className="h-6 w-6">
                    <MoreVertical size={12} />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => handleEdit('resource', resource.id)}>
                    <Edit3 size={14} className="mr-2" />
                    编辑
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={() => navigate(`/resource/${resource.id}`)}>
                    <Eye size={14} className="mr-2" />
                    查看
                  </DropdownMenuItem>
                  {resource.status === 'published' && (
                    <DropdownMenuItem onClick={() => handleStatusChange('resource', resource.id, 'draft')}>
                      <EyeOff size={14} className="mr-2" />
                      下架
                    </DropdownMenuItem>
                  )}
                  {resource.status === 'draft' && (
                    <DropdownMenuItem onClick={() => handleStatusChange('resource', resource.id, 'reviewing')}>
                      <CheckCircle size={14} className="mr-2" />
                      提交审核
                    </DropdownMenuItem>
                  )}
                  <DropdownMenuSeparator />
                  <DropdownMenuItem 
                    onClick={() => handleDelete('resource', resource.id, resource.title)}
                    className="text-destructive"
                  >
                    <Trash2 size={14} className="mr-2" />
                    删除
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

            <p className="text-xs text-muted-foreground mb-3 line-clamp-2">
              {resource.description}
            </p>

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

            <div className="flex items-center justify-between text-xs text-muted-foreground">
              <span>{resource.version}</span>
              <div className="flex items-center gap-3">
                <div className="flex items-center">
                  <Download size={10} className="mr-1" />
                  {resource.downloads}
                </div>
                <div className="flex items-center">
                  <Heart size={10} className="mr-1" />
                  {resource.likes}
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
      >
        <Card className="hover:shadow-lg transition-shadow">
          <CardContent className="p-4">
            <div className="flex items-start gap-3">
              <img 
                src={post.image} 
                alt={post.title}
                className="w-16 h-16 rounded-lg object-cover shrink-0"
              />
              
              <div className="flex-1 min-w-0">
                <div className="flex items-start justify-between mb-2">
                  <h3 className="font-medium text-sm line-clamp-1">{post.title}</h3>
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" size="icon" className="h-6 w-6 shrink-0">
                        <MoreVertical size={12} />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem onClick={() => handleEdit('post', post.id)}>
                        <Edit3 size={14} className="mr-2" />
                        编辑
                      </DropdownMenuItem>
                      <DropdownMenuItem onClick={() => navigate(`/post/${post.id}`)}>
                        <Eye size={14} className="mr-2" />
                        查看
                      </DropdownMenuItem>
                      {post.status === 'published' && (
                        <DropdownMenuItem onClick={() => handleStatusChange('post', post.id, 'draft')}>
                          <EyeOff size={14} className="mr-2" />
                          下架
                        </DropdownMenuItem>
                      )}
                      {post.status === 'draft' && (
                        <DropdownMenuItem onClick={() => handleStatusChange('post', post.id, 'reviewing')}>
                          <CheckCircle size={14} className="mr-2" />
                          提交审核
                        </DropdownMenuItem>
                      )}
                      <DropdownMenuSeparator />
                      <DropdownMenuItem 
                        onClick={() => handleDelete('post', post.id, post.title)}
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

                <p className="text-xs text-muted-foreground mb-3 line-clamp-2">
                  {post.content}
                </p>

                <div className="flex items-center justify-between text-xs text-muted-foreground">
                  <span>{post.createdAt}</span>
                  <div className="flex items-center gap-3">
                    <div className="flex items-center">
                      <Eye size={10} className="mr-1" />
                      {post.views}
                    </div>
                    <div className="flex items-center">
                      <Heart size={10} className="mr-1" />
                      {post.likes}
                    </div>
                    <div className="flex items-center">
                      <MessageSquare size={10} className="mr-1" />
                      {post.comments}
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
      >
        <Card className="hover:shadow-lg transition-shadow">
          <CardContent className="p-4">
            <div className="flex items-start justify-between mb-2">
              <h4 className="text-sm font-medium text-primary line-clamp-1">
                {comment.postTitle}
              </h4>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon" className="h-6 w-6 shrink-0">
                    <MoreVertical size={12} />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => handleEdit('comment', comment.id)}>
                    <Edit3 size={14} className="mr-2" />
                    编辑
                  </DropdownMenuItem>
                  <DropdownMenuItem 
                    onClick={() => handleDelete('comment', comment.id, comment.content.slice(0, 20) + '...')}
                    className="text-destructive"
                  >
                    <Trash2 size={14} className="mr-2" />
                    删除
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

            <p className="text-sm text-muted-foreground mb-3 line-clamp-3">
              {comment.content}
            </p>

            <div className="flex items-center justify-between text-xs text-muted-foreground">
              <span>回复给 {comment.postAuthor}</span>
              <div className="flex items-center gap-3">
                <span>{comment.createdAt}</span>
                <div className="flex items-center">
                  <Heart size={10} className="mr-1" />
                  {comment.likes}
                </div>
                {comment.replies > 0 && (
                  <div className="flex items-center">
                    <MessageSquare size={10} className="mr-1" />
                    {comment.replies}
                  </div>
                )}
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
          <div className="p-4">
            {/* 搜索栏 */}
            <div className="flex items-center gap-2 mb-4">
              <div className="relative flex-1">
                <Search size={16} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                <Input
                  placeholder="搜索我的内容..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-10"
                />
              </div>
              <Button variant="outline" size="icon">
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

                <div className="space-y-4">
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