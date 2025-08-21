import React, { useState, useEffect, useCallback } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Bell, Search, MoreVertical, Check, CheckCheck, Loader2, ExternalLink, Trash2, Settings, AlertTriangle, RefreshCw, Filter, ChevronDown } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { 
  getNotifications, 
  markAsRead,
  markAllAsRead,
  getUnreadCount,
  deleteReadNotifications,
  Notification,
  NotificationQuery
} from '@/api/notifications'

// 通知类型配置 - 基于后端实际使用的类型
const NOTIFICATION_TYPES = [
  { value: 'all', label: '全部通知', icon: '📢' },
  { value: 'ResourceApproved', label: '资源审核', icon: '✅' },
  { value: 'CommentReceived', label: '评论回复', icon: '💬' },
  { value: 'PostFlagChanged', label: '帖子状态', icon: '📝' },
  { value: 'CategoryUpdate', label: '分类更新', icon: '🏷️' },
  { value: 'unknown', label: '公告通知', icon: '📢' },
]

const MessagesScreen: React.FC = () => {
  const [notifications, setNotifications] = useState<Notification[]>([])
  const [loading, setLoading] = useState(true)
  const [refreshing, setRefreshing] = useState(false)
  const [loadingMore, setLoadingMore] = useState(false)
  const [searchQuery, setSearchQuery] = useState('')
  const [unreadCount, setUnreadCount] = useState(0)
  const [bulkActionLoading, setBulkActionLoading] = useState(false)
  const [showClearDialog, setShowClearDialog] = useState(false)
  const [selectedType, setSelectedType] = useState('all')
  const [currentPage, setCurrentPage] = useState(1)
  const [hasMore, setHasMore] = useState(true)
  const [selectedNotifications, setSelectedNotifications] = useState<Set<number>>(new Set())
  const [isSelectionMode, setIsSelectionMode] = useState(false)

  // 加载通知列表
  useEffect(() => {
    loadNotifications(true)
    loadUnreadCount()
  }, [])

  const loadNotifications = async (reset = false, showLoading = true) => {
    try {
      if (showLoading) setLoading(true)
      const page = reset ? 1 : currentPage
      const params: NotificationQuery = { 
        page, 
        size: 20
      }
      
      const data = await getNotifications(params)
      const newNotifications = data.list || []
      
      if (reset) {
        setNotifications(newNotifications)
        setCurrentPage(1)
      } else {
        setNotifications(prev => [...prev, ...newNotifications])
      }
      
      setHasMore(newNotifications.length === 20)
      if (!reset) setCurrentPage(prev => prev + 1)
    } catch (error: any) {
      console.error('Failed to load notifications:', error)
      if (error.message !== '未授权，请重新登录') {
        toast({
          title: "加载失败",
          description: "无法加载通知列表",
          variant: "destructive"
        })
      }
    } finally {
      if (showLoading) setLoading(false)
    }
  }

  const loadUnreadCount = async () => {
    try {
      const data = await getUnreadCount()
      setUnreadCount(data.count || 0)
    } catch (error) {
      console.error('Failed to load unread count:', error)
    }
  }

  // 下拉刷新
  const handleRefresh = async () => {
    setRefreshing(true)
    await Promise.all([
      loadNotifications(true, false),
      loadUnreadCount()
    ])
    setRefreshing(false)
    toast({
      title: "刷新完成",
      description: "通知列表已更新",
    })
  }

  // 加载更多
  const handleLoadMore = async () => {
    if (loadingMore || !hasMore) return
    setLoadingMore(true)
    await loadNotifications(false, false)
    setLoadingMore(false)
  }

  // 标记单个通知为已读
  const handleMarkAsRead = async (id: number) => {
    try {
      await markAsRead(id)
      setNotifications(prev => 
        prev.map(notification => 
          notification.id === id 
            ? { ...notification, is_read: true }
            : notification
        )
      )
      setUnreadCount(prev => Math.max(0, prev - 1))
    } catch (error: any) {
      console.error('Mark as read failed:', error)
      toast({
        title: "操作失败",
        description: error.message || "标记已读失败",
        variant: "destructive"
      })
    }
  }

  // 标记所有通知为已读
  const handleMarkAllAsRead = async () => {
    if (unreadCount === 0) {
      toast({
        title: "提示",
        description: "没有未读通知",
      })
      return
    }

    try {
      setBulkActionLoading(true)
      await markAllAsRead()
      setNotifications(prev => 
        prev.map(notification => ({ ...notification, is_read: true }))
      )
      setUnreadCount(0)
      
      toast({
        title: "全部已读",
        description: "所有通知已标记为已读",
      })
    } catch (error: any) {
      console.error('Mark all as read failed:', error)
      toast({
        title: "操作失败",
        description: error.message || "标记全部已读失败",
        variant: "destructive"
      })
    } finally {
      setBulkActionLoading(false)
    }
  }

  // 清理已读通知
  const handleClearRead = async () => {
    try {
      setBulkActionLoading(true)
      const result = await deleteReadNotifications()
      
      // 从前端状态中移除已读通知
      setNotifications(prev => prev.filter(n => !n.is_read))
      
      toast({
        title: "清理完成",
        description: `已清理 ${result.deleted_count} 条已读通知`,
      })
    } catch (error: any) {
      console.error('Clear read failed:', error)
      toast({
        title: "操作失败",
        description: error.message || "清理失败",
        variant: "destructive"
      })
    } finally {
      setBulkActionLoading(false)
      setShowClearDialog(false)
    }
  }

  // 批量选择相关
  const toggleSelection = (id: number) => {
    const newSelected = new Set(selectedNotifications)
    if (newSelected.has(id)) {
      newSelected.delete(id)
    } else {
      newSelected.add(id)
    }
    setSelectedNotifications(newSelected)
  }

  const selectAll = () => {
    const visibleIds = filteredNotifications.map(n => n.id)
    setSelectedNotifications(new Set(visibleIds))
  }

  const clearSelection = () => {
    setSelectedNotifications(new Set())
    setIsSelectionMode(false)
  }

  // 批量操作选中的通知
  const handleBulkMarkAsRead = async () => {
    const selectedIds = Array.from(selectedNotifications)
    const unreadSelected = selectedIds.filter(id => {
      const notification = notifications.find(n => n.id === id)
      return notification && !notification.is_read
    })

    if (unreadSelected.length === 0) {
      toast({
        title: "提示",
        description: "所选通知都已是已读状态",
      })
      return
    }

    try {
      setBulkActionLoading(true)
      // 这里需要批量标记API，暂时逐个调用
      await Promise.all(unreadSelected.map(id => markAsRead(id)))
      
      setNotifications(prev => 
        prev.map(notification => 
          selectedIds.includes(notification.id) 
            ? { ...notification, is_read: true }
            : notification
        )
      )
      
      setUnreadCount(prev => Math.max(0, prev - unreadSelected.length))
      clearSelection()
      
      toast({
        title: "批量操作完成",
        description: `已标记 ${unreadSelected.length} 条通知为已读`,
      })
    } catch (error: any) {
      console.error('Bulk mark as read failed:', error)
      toast({
        title: "操作失败",
        description: error.message || "批量标记失败",
        variant: "destructive"
      })
    } finally {
      setBulkActionLoading(false)
    }
  }

  // 检查是否有已读通知可清理
  const readNotificationsCount = notifications.filter(n => n.is_read).length

  // 格式化时间
  const formatTime = (timeStr: string) => {
    const time = new Date(timeStr)
    const now = new Date()
    const diffInMs = now.getTime() - time.getTime()
    const diffInMinutes = Math.floor(diffInMs / (1000 * 60))
    const diffInHours = Math.floor(diffInMinutes / 60)
    const diffInDays = Math.floor(diffInHours / 24)

    if (diffInMinutes < 1) return '刚刚'
    if (diffInMinutes < 60) return `${diffInMinutes}分钟前`
    if (diffInHours < 24) return `${diffInHours}小时前`
    if (diffInDays === 1) return '昨天'
    if (diffInDays < 7) return `${diffInDays}天前`
    
    return time.toLocaleDateString()
  }

  // 过滤通知（前端筛选）
  const filteredNotifications = notifications.filter(notification => {
    // 类型筛选
    let typeMatch = false
    if (selectedType === 'all') {
      typeMatch = true
    } else if (selectedType === 'unknown') {
      // "公告通知" 匹配所有未知类型或空类型（通常是系统公告）
      typeMatch = !notification.notif_type || !NOTIFICATION_TYPES.some(t => t.value === notification.notif_type && t.value !== 'all' && t.value !== 'unknown')
    } else {
      typeMatch = notification.notif_type === selectedType
    }
    
    // 搜索筛选
    const searchMatch = !searchQuery || 
      notification.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      notification.content.toLowerCase().includes(searchQuery.toLowerCase())
    
    return typeMatch && searchMatch
  })

  // 获取实际存在的通知类型
  const actualTypes = React.useMemo(() => {
    if (notifications.length === 0) return []
    const uniqueTypes = [...new Set(notifications.map(n => n.notif_type).filter(Boolean))]
    return uniqueTypes
  }, [notifications])

  // 调试信息：显示实际的通知类型
  React.useEffect(() => {
    if (notifications.length > 0) {
      console.log('实际通知类型:', actualTypes)
      console.log('当前筛选类型:', selectedType)
      console.log('筛选后数量:', filteredNotifications.length)
    }
  }, [actualTypes, selectedType, filteredNotifications.length])

  // 获取通知类型图标
  const getNotificationIcon = (type?: string) => {
    if (!type) return '📢' // 未知类型使用公告图标（通常是系统公告）
    const typeConfig = NOTIFICATION_TYPES.find(t => t.value === type)
    return typeConfig?.icon || '📢' // 未匹配的类型也使用公告图标
  }

  // 处理通知点击
  const handleNotificationClick = (notification: Notification) => {
    const navigate = useNavigate()
    
    if (isSelectionMode) {
      toggleSelection(notification.id)
      return
    }

    // 如果未读，标记为已读
    if (!notification.is_read) {
      handleMarkAsRead(notification.id)
    }
    
    // 如果有链接，跳转
    if (notification.link) {
      // 解析链接并跳转到相应页面
      try {
        const url = new URL(notification.link, window.location.origin)
        const pathname = url.pathname
        
        // 内部路由跳转
        if (pathname.startsWith('/')) {
          navigate(pathname)
        } else {
          // 外部链接
          window.open(notification.link, '_blank')
        }
      } catch (error) {
        console.error('Invalid link:', notification.link)
        // 如果链接格式不正确，尝试作为内部路由处理
        if (notification.link.startsWith('/')) {
          navigate(notification.link)
        }
      }
    }
  }

  return (
    <div className="flex flex-col h-full bg-background">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="通知"
        subtitle={loading ? "加载中..." : `${notifications.length} 条通知${unreadCount > 0 ? `，${unreadCount} 条未读` : ''}`}
        showSearchButton
        rightAction={
          <div className="flex items-center space-x-2">
            {/* 刷新按钮 */}
            <Button 
              variant="ghost" 
              size="icon" 
              className="h-9 w-9"
              onClick={handleRefresh}
              disabled={refreshing}
            >
              <RefreshCw className={`h-4 w-4 ${refreshing ? 'animate-spin' : ''}`} />
            </Button>

            {/* 选择模式切换 */}
            {notifications.length > 0 && (
              <Button 
                variant="ghost" 
                size="icon" 
                className="h-9 w-9"
                onClick={() => setIsSelectionMode(!isSelectionMode)}
              >
                <Check className={`h-4 w-4 ${isSelectionMode ? 'text-primary' : ''}`} />
              </Button>
            )}

            {/* 更多操作菜单 */}
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button 
                  variant="ghost" 
                  size="icon" 
                  className="h-9 w-9"
                  disabled={bulkActionLoading}
                >
                  {bulkActionLoading ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    <MoreVertical size={20} />
                  )}
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" sideOffset={20} className="w-48 z-[80]">
                <DropdownMenuItem 
                  onClick={handleMarkAllAsRead}
                  disabled={unreadCount === 0 || bulkActionLoading}
                  className="flex items-center"
                >
                  <CheckCheck className="mr-2 h-4 w-4" />
                  一键已读 {unreadCount > 0 && `(${unreadCount})`}
                </DropdownMenuItem>
                <DropdownMenuItem 
                  onClick={() => setShowClearDialog(true)}
                  disabled={readNotificationsCount === 0 || bulkActionLoading}
                  className="flex items-center"
                >
                  <Trash2 className="mr-2 h-4 w-4" />
                  清理已读 {readNotificationsCount > 0 && `(${readNotificationsCount})`}
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem 
                  className="flex items-center"
                  onClick={() => toast({ title: "通知设置", description: "功能开发中..." })}
                >
                  <Settings className="mr-2 h-4 w-4" />
                  通知设置
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        {/* 筛选和搜索栏 */}
        <div className="px-4 py-3 space-y-3">
          {/* 通知类型筛选 */}
          <div className="flex items-center space-x-3">
            <Select value={selectedType} onValueChange={setSelectedType}>
              <SelectTrigger className="w-40 h-9">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {NOTIFICATION_TYPES.map((type) => {
                  // 计算该类型的通知数量
                  let count = 0
                  if (type.value === 'all') {
                    count = notifications.length
                  } else if (type.value === 'unknown') {
                    count = notifications.filter(n => !n.notif_type || !NOTIFICATION_TYPES.some(t => t.value === n.notif_type && t.value !== 'all' && t.value !== 'unknown')).length
                  } else {
                    count = notifications.filter(n => n.notif_type === type.value).length
                  }

                  // 如果不是"全部通知"且数量为0，则不显示该选项
                  if (type.value !== 'all' && count === 0) {
                    return null
                  }

                  return (
                    <SelectItem key={type.value} value={type.value}>
                      <div className="flex items-center justify-between w-full">
                        <div className="flex items-center space-x-2">
                          <span>{type.icon}</span>
                          <span>{type.label}</span>
                        </div>
                        <span className="text-xs text-muted-foreground ml-2">
                          ({count})
                        </span>
                      </div>
                    </SelectItem>
                  )
                })}
              </SelectContent>
            </Select>
            
            {selectedType !== 'all' && (
              <Button 
                variant="outline" 
                size="sm" 
                onClick={() => setSelectedType('all')}
                className="h-9"
              >
                清除筛选
              </Button>
            )}
            

          </div>

          {/* 搜索框 */}
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
          <Input
              id="notifications-search"
              name="notificationSearch"
              placeholder="搜索通知..."
              className="pl-10 h-9"
            autoComplete="search"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
          </div>

          {/* 批量选择工具栏 */}
          <AnimatePresence>
            {isSelectionMode && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                exit={{ opacity: 0, height: 0 }}
                className="flex items-center justify-between p-3 bg-muted/30 rounded-lg"
              >
                <div className="flex items-center space-x-3">
                  <span className="text-sm text-muted-foreground">
                    已选择 {selectedNotifications.size} 项
                  </span>
                  <Button size="sm" variant="outline" onClick={selectAll}>
                    全选
                  </Button>
                  <Button size="sm" variant="outline" onClick={clearSelection}>
                    取消
                  </Button>
                </div>
                
                {selectedNotifications.size > 0 && (
                  <div className="flex items-center space-x-2">
                    <Button size="sm" onClick={handleBulkMarkAsRead} disabled={bulkActionLoading}>
                      <Check className="mr-1 h-3 w-3" />
                      标记已读
                    </Button>
                  </div>
                )}
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* 加载状态 */}
        {loading && (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-8 w-8 animate-spin" />
            <span className="ml-2">加载通知中...</span>
          </div>
        )}

        {/* 通知列表 */}
        {!loading && (
          <div className="flex-1 overflow-y-auto pb-4">
            <div className="space-y-2 px-4">
              {filteredNotifications.map((notification) => (
                <motion.div
                  key={notification.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.3 }}
                  onClick={() => handleNotificationClick(notification)}
                >
                  <Card className={`cursor-pointer transition-all duration-200 border-l-4 hover:shadow-md ${
                    notification.is_read 
                      ? 'border-l-muted hover:bg-muted/20' 
                      : 'border-l-primary bg-primary/5 hover:bg-primary/10 shadow-sm'
                  } ${
                    selectedNotifications.has(notification.id) ? 'ring-2 ring-primary bg-primary/10' : ''
                  }`}>
                    <CardContent className="p-3">
                      <div className="flex items-start space-x-3">
                        {/* 选择框（选择模式下） */}
                        {isSelectionMode && (
                          <div className="flex-shrink-0 mt-0.5">
                            <div className={`w-4 h-4 rounded border-2 flex items-center justify-center ${
                              selectedNotifications.has(notification.id)
                                ? 'bg-primary border-primary'
                                : 'border-muted-foreground'
                            }`}>
                              {selectedNotifications.has(notification.id) && (
                                <Check className="w-3 h-3 text-primary-foreground" />
                              )}
                            </div>
                          </div>
                        )}

                        {/* 通知图标 */}
                        <div className="flex-shrink-0 text-lg mt-0.5">
                          {getNotificationIcon(notification.notif_type)}
                        </div>
                        
                        <div className="flex-1 min-w-0">
                          <div className="flex items-start justify-between mb-1">
                            <h3 className={`font-medium text-sm leading-tight ${
                              notification.is_read ? 'text-muted-foreground' : 'text-foreground'
                            }`}>
                              {notification.title}
                            </h3>
                            <div className="flex items-center space-x-2 ml-2 flex-shrink-0">
                              <span className="text-xs text-muted-foreground whitespace-nowrap">
                                {formatTime(notification.created_at)}
                              </span>
                              {!notification.is_read && (
                                <div className="w-2 h-2 bg-primary rounded-full"></div>
                              )}
                            </div>
                          </div>
                          
                          <p className={`text-sm mb-2 leading-relaxed ${
                            notification.is_read ? 'text-muted-foreground' : 'text-foreground/80'
                          }`}>
                            {notification.content}
                          </p>
                          
                          <div className="flex items-center justify-between">
                            <div className="flex items-center space-x-2">
                              {notification.notif_type && (
                                <Badge variant="outline" className="text-xs px-2 py-0.5">
                                  {NOTIFICATION_TYPES.find(t => t.value === notification.notif_type)?.label || notification.notif_type}
                                </Badge>
                              )}
                              {notification.link && (
                                <ExternalLink size={12} className="text-muted-foreground" />
                              )}
      </div>

                            {!isSelectionMode && !notification.is_read && (
                              <Button
                                variant="ghost"
                                size="sm"
                                className="h-6 px-2 text-xs hover:bg-primary/20"
        onClick={(e) => {
                                  e.stopPropagation()
                                  handleMarkAsRead(notification.id)
                                }}
                              >
                                <Check size={12} className="mr-1" />
                                已读
                              </Button>
                            )}
                          </div>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}

              {/* 加载更多按钮 */}
              {!loading && hasMore && notifications.length > 0 && (
                <div className="flex justify-center py-4">
                  <Button 
                    variant="outline" 
                    onClick={handleLoadMore}
                    disabled={loadingMore}
                    className="w-full max-w-xs"
                  >
                    {loadingMore ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        加载中...
                      </>
                    ) : (
                      <>
                        <ChevronDown className="mr-2 h-4 w-4" />
                        加载更多
                      </>
                    )}
                  </Button>
                </div>
              )}
            </div>
      </div>
        )}

      {/* 空状态 */}
        {!loading && filteredNotifications.length === 0 && (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
              <Bell className="h-16 w-16 text-muted-foreground mx-auto mb-4" />
              <h3 className="text-lg font-medium mb-2">
                {searchQuery || selectedType !== 'all' ? '未找到相关通知' : notifications.length === 0 ? '暂无通知' : '没有匹配的通知'}
              </h3>
              <p className="text-muted-foreground">
                {searchQuery || selectedType !== 'all'
                  ? '尝试调整筛选条件或搜索关键词' 
                  : notifications.length === 0 
                    ? '当有新的通知时，会在这里显示' 
                    : '尝试调整搜索条件'
                }
              </p>
            </div>
        </div>
      )}
      </div> {/* 结束内容区域 */}

      {/* 清理已读通知确认对话框 */}
      <AlertDialog open={showClearDialog} onOpenChange={setShowClearDialog}>
        <AlertDialogContent className="mx-4 rounded-2xl max-w-md">
          <AlertDialogHeader className="pb-4">
            <AlertDialogTitle className="flex items-center text-lg">
              <AlertTriangle className="mr-2 h-5 w-5 text-orange-500" />
              确认清理已读通知
            </AlertDialogTitle>
            <AlertDialogDescription className="text-sm leading-relaxed pt-2">
              您即将永久清理 <span className="font-medium text-foreground">{readNotificationsCount}</span> 条已读通知。
              <br /><br />
              <span className="text-orange-600 font-medium">注意：此操作不可撤销，清理后的通知将无法恢复。</span>
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter className="pt-4 gap-3">
            <AlertDialogCancel className="rounded-xl">取消</AlertDialogCancel>
            <AlertDialogAction 
              onClick={handleClearRead}
              disabled={bulkActionLoading}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-xl"
            >
              {bulkActionLoading ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  清理中...
                </>
              ) : (
                <>
                  <Trash2 className="mr-2 h-4 w-4" />
                  确认清理
                </>
              )}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}

export default MessagesScreen 