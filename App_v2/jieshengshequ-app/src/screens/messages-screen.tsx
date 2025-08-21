import React, { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Bell, Search, MoreVertical, Check, CheckCheck, Loader2, ExternalLink, Trash2, Settings, AlertTriangle } from 'lucide-react'
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
  getNotifications, 
  markAsRead,
  markAllAsRead,
  getUnreadCount,
  Notification,
  NotificationQuery
} from '@/api/notifications'

const MessagesScreen: React.FC = () => {
  const [notifications, setNotifications] = useState<Notification[]>([])
  const [loading, setLoading] = useState(true)
  const [searchQuery, setSearchQuery] = useState('')
  const [unreadCount, setUnreadCount] = useState(0)
  const [bulkActionLoading, setBulkActionLoading] = useState(false)
  const [showClearDialog, setShowClearDialog] = useState(false)

  // 加载通知列表
  useEffect(() => {
    loadNotifications()
    loadUnreadCount()
  }, [])

  const loadNotifications = async () => {
    try {
      setLoading(true)
      const params: NotificationQuery = { page: 1, size: 50 }
      const data = await getNotifications(params)
      setNotifications(data.list || [])
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
      setLoading(false)
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
      
      toast({
        title: "已标记为已读",
        description: "通知已标记为已读",
      })
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
    const readNotifications = notifications.filter(n => n.is_read)
    
    try {
      setBulkActionLoading(true)
      // 这里暂时只在前端移除，后端可能需要添加批量删除API
      setNotifications(prev => prev.filter(n => !n.is_read))
      
      toast({
        title: "清理完成",
        description: `已清理 ${readNotifications.length} 条已读通知`,
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

  // 过滤通知
  const filteredNotifications = notifications.filter(notification =>
    notification.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    notification.content.toLowerCase().includes(searchQuery.toLowerCase())
  )

  // 获取通知类型图标
  const getNotificationIcon = (type?: string) => {
    switch (type) {
      case 'ResourceApproved':
        return '✅'
      case 'CommentReceived':
        return '💬'
      case 'SystemNotification':
        return '🔔'
      default:
        return '📢'
    }
  }

  // 处理通知点击
  const handleNotificationClick = (notification: Notification) => {
    // 如果未读，标记为已读
    if (!notification.is_read) {
      handleMarkAsRead(notification.id)
    }
    
    // 如果有链接，跳转
    if (notification.link) {
      // 这里可以根据需要处理内部路由或外部链接
      console.log('Navigate to:', notification.link)
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
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        {/* 搜索栏 */}
        <div className="px-4 py-3">
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
                  }`}>
                    <CardContent className="p-3">
                      <div className="flex items-start space-x-3">
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
                                  {notification.notif_type}
                                </Badge>
                              )}
                              {notification.link && (
                                <ExternalLink size={12} className="text-muted-foreground" />
                              )}
                            </div>
                            
                            {!notification.is_read && (
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
            </div>
          </div>
        )}

        {/* 空状态 */}
        {!loading && filteredNotifications.length === 0 && (
          <div className="flex-1 flex items-center justify-center">
            <div className="text-center">
              <Bell className="h-16 w-16 text-muted-foreground mx-auto mb-4" />
              <h3 className="text-lg font-medium mb-2">
                {searchQuery ? '未找到相关通知' : notifications.length === 0 ? '暂无通知' : '没有匹配的通知'}
              </h3>
              <p className="text-muted-foreground">
                {searchQuery 
                  ? '尝试使用其他关键词搜索' 
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
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle className="flex items-center">
              <AlertTriangle className="mr-2 h-5 w-5 text-orange-500" />
              确认清理已读通知
            </AlertDialogTitle>
            <AlertDialogDescription>
              您即将清理 <span className="font-medium text-foreground">{readNotificationsCount}</span> 条已读通知。
              此操作无法撤销，确定要继续吗？
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>取消</AlertDialogCancel>
            <AlertDialogAction 
              onClick={handleClearRead}
              disabled={bulkActionLoading}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
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