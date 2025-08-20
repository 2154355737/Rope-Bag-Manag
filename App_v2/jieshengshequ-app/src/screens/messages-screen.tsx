import React, { useState, useEffect } from 'react'
import { motion, PanInfo } from 'framer-motion'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { MessageCircle, Search, MoreVertical, Trash2, Pin } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'

const MessagesScreen: React.FC = () => {
  const [activeSwipeId, setActiveSwipeId] = useState<number | null>(null)
  const [conversations, setConversations] = useState([
    {
      id: 1,
      name: '张三',
      avatar: '/api/placeholder/40/40',
      lastMessage: '你好，关于React的问题...',
      time: '2分钟前',
      unread: 2,
      online: true,
      pinned: false
    },
    {
      id: 2,
      name: '李四',
      avatar: '/api/placeholder/40/40',
      lastMessage: '项目进展如何？',
      time: '1小时前',
      unread: 0,
      online: false,
      pinned: false
    },
    {
      id: 3,
      name: 'JavaScript交流群',
      avatar: '/api/placeholder/40/40',
      lastMessage: '王五: 有人用过Next.js吗？',
      time: '3小时前',
      unread: 5,
      online: false,
      isGroup: true,
      pinned: true
    },
    {
      id: 4,
      name: '前端开发者社区',
      avatar: '/api/placeholder/40/40',
      lastMessage: '新的技术分享已发布',
      time: '昨天',
      unread: 1,
      online: false,
      isGroup: true,
      pinned: false
    }
  ])

  // 删除对话
  const deleteConversation = (id: number) => {
    const conversation = conversations.find(conv => conv.id === id)
    setConversations(prev => prev.filter(conv => conv.id !== id))
    
    toast({
      title: "对话已删除",
      description: `已删除与 ${conversation?.name} 的对话`,
      variant: "default"
    })
  }

  // 置顶/取消置顶对话
  const togglePinConversation = (id: number) => {
    const conversation = conversations.find(conv => conv.id === id)
    setConversations(prev => 
      prev.map(conv => 
        conv.id === id 
          ? { ...conv, pinned: !conv.pinned }
          : conv
      )
    )
    
    toast({
      title: conversation?.pinned ? "取消置顶" : "置顶成功",
      description: `${conversation?.pinned ? '已取消置顶' : '已置顶'} ${conversation?.name}`,
      variant: "default"
    })
  }

  // 对话排序：置顶的在前面
  const sortedConversations = [...conversations].sort((a, b) => {
    if (a.pinned && !b.pinned) return -1
    if (!a.pinned && b.pinned) return 1
    return 0
  })

  // 侧滑组件
  const SwipeableConversation: React.FC<{ 
    conversation: any, 
    onDelete: (id: number) => void,
    onTogglePin: (id: number) => void,
    isActive: boolean,
    onActivate: (id: number | null) => void
  }> = ({ conversation, onDelete, onTogglePin, isActive, onActivate }) => {
    const [dragX, setDragX] = useState(0)
    const [showDelete, setShowDelete] = useState(false)
    const [showPin, setShowPin] = useState(false)
    const [isDragging, setIsDragging] = useState(false)

    const handleDragStart = () => {
      setIsDragging(true)
    }

    const handleDrag = (event: any, info: PanInfo) => {
      // 增加拖拽阻力，减少灵敏度
      const resistance = 0.7
      const newX = Math.max(-100, Math.min(100, info.offset.x * resistance))
      setDragX(newX)
      
      // 提高显示按钮的阈值
      setShowDelete(newX < -60)
      setShowPin(newX > 60)
    }

    const handleDragEnd = (event: any, info: PanInfo) => {
      setIsDragging(false)
      const velocity = info.velocity.x
      const offset = info.offset.x
      
      // 提高触发阈值，并考虑滑动速度
      const deleteThreshold = velocity < -500 ? -50 : -80  // 快速滑动时降低阈值
      const pinThreshold = velocity > 500 ? 50 : 80
      
      if (offset < deleteThreshold) {
        // 左滑触发删除
        setDragX(-100)
        setShowDelete(true)
        setShowPin(false)
        onActivate(conversation.id)
      } else if (offset > pinThreshold) {
        // 右滑触发置顶
        setDragX(100)
        setShowPin(true)
        setShowDelete(false)
        onActivate(conversation.id)
      } else {
        // 回弹到原位
        setDragX(0)
        setShowDelete(false)
        setShowPin(false)
        onActivate(null)
      }
    }

    const handleDelete = () => {
      onDelete(conversation.id)
      setDragX(0)
      setShowDelete(false)
      onActivate(null)
    }

    const handleTogglePin = () => {
      onTogglePin(conversation.id)
      setDragX(0)
      setShowPin(false)
      onActivate(null)
    }

    // 监听外部状态变化，重置当前组件
    useEffect(() => {
      if (!isActive) {
        setDragX(0)
        setShowDelete(false)
        setShowPin(false)
      }
    }, [isActive])

    // 重置到原始状态
    const resetPosition = () => {
      setDragX(0)
      setShowDelete(false)
      setShowPin(false)
      onActivate(null)
    }

    return (
      <div className="relative overflow-hidden">
        {/* 置顶按钮背景 */}
        <div 
          className={`absolute inset-y-0 left-0 w-24 ${
            conversation.pinned ? 'bg-orange-500' : 'bg-blue-500'
          } flex items-center justify-center transition-all duration-300 ease-out ${
            showPin ? 'opacity-100' : 'opacity-0'
          }`}
        >
          <Button
            variant="ghost"
            size="icon"
            onClick={handleTogglePin}
            className="text-white hover:bg-opacity-80"
          >
            <Pin size={20} className={conversation.pinned ? 'rotate-45' : ''} />
          </Button>
        </div>

        {/* 删除按钮背景 */}
        <div 
          className={`absolute inset-y-0 right-0 w-24 bg-red-500 flex items-center justify-center transition-all duration-300 ease-out ${
            showDelete ? 'opacity-100' : 'opacity-0'
          }`}
        >
          <Button
            variant="ghost"
            size="icon"
            onClick={handleDelete}
            className="text-white hover:bg-red-600 hover:text-white"
          >
            <Trash2 size={20} />
          </Button>
        </div>

        {/* 可滑动的对话卡片 */}
        <motion.div
          drag="x"
          dragConstraints={{ left: -100, right: 100 }}
          dragElastic={0.05}  // 降低弹性，减少抖动
          dragMomentum={false}  // 禁用拖拽惯性，提高控制性
          dragDirectionLock={true}  // 锁定拖拽方向，避免意外滑动
          onDragStart={handleDragStart}
          onDrag={handleDrag}
          onDragEnd={handleDragEnd}
          animate={{ x: dragX }}
          transition={{ 
            type: "spring", 
            stiffness: 400,  // 提高刚度，减少弹跳
            damping: 40,     // 增加阻尼，减少振动
            mass: 0.8        // 降低质量，提高响应速度
          }}
          className="relative z-10"
        >
          <Card className="m-2 cursor-pointer hover:bg-muted/50 transition-colors border-none">
            <CardContent 
              className="p-4" 
              onClick={(e) => {
                // 防止拖拽时触发点击
                if (!isDragging) {
                  resetPosition()
                }
              }}
            >
              <div className="flex items-center space-x-3">
                <div className="relative">
                  <Avatar>
                    <AvatarImage src={conversation.avatar} alt={conversation.name} />
                    <AvatarFallback>
                      {conversation.isGroup ? (
                        <MessageCircle className="h-4 w-4" />
                      ) : (
                        conversation.name.charAt(0)
                      )}
                    </AvatarFallback>
                  </Avatar>
                  {conversation.online && !conversation.isGroup && (
                    <div className="absolute -bottom-1 -right-1 w-3 h-3 bg-green-500 rounded-full border-2 border-background"></div>
                  )}
                </div>
                
                <div className="flex-1 min-w-0">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center">
                      <h3 className="font-medium truncate">{conversation.name}</h3>
                      {conversation.pinned && (
                        <Pin size={14} className="ml-1 text-orange-500 rotate-45 flex-shrink-0" />
                      )}
                    </div>
                    <span className="text-xs text-muted-foreground">{conversation.time}</span>
                  </div>
                  <p className="text-sm text-muted-foreground truncate mt-1">
                    {conversation.lastMessage}
                  </p>
                </div>
                
                {conversation.unread > 0 && (
                  <Badge variant="destructive" className="ml-2">
                    {conversation.unread}
                  </Badge>
                )}
              </div>
            </CardContent>
          </Card>
        </motion.div>
      </div>
    )
  }

  return (
    <div className="flex flex-col h-full bg-background">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="消息"
        subtitle={`${conversations.length} 个对话`}
        showSearchButton
        showMoreButton
        onMoreClick={() => toast({ title: "更多选项", description: "功能开发中..." })}
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        {/* 搜索栏 */}
        <div className="p-4">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
          <Input
            id="messages-search"
            name="messageSearch"
            placeholder="搜索对话..."
            className="pl-10"
            autoComplete="search"
          />
        </div>
      </div>

      {/* 对话列表 */}
      <div 
        className="flex-1 overflow-y-auto"
        onClick={(e) => {
          // 点击空白区域重置所有滑动状态
          if (e.target === e.currentTarget) {
            setActiveSwipeId(null)
          }
        }}
      >
        {sortedConversations.map((conversation) => (
          <SwipeableConversation
            key={conversation.id}
            conversation={conversation}
            onDelete={deleteConversation}
            onTogglePin={togglePinConversation}
            isActive={activeSwipeId === conversation.id}
            onActivate={setActiveSwipeId}
          />
        ))}
      </div>

      {/* 空状态 */}
      {conversations.length === 0 && (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <MessageCircle className="h-16 w-16 text-muted-foreground mx-auto mb-4" />
            <h3 className="text-lg font-medium mb-2">暂无消息</h3>
            <p className="text-muted-foreground">开始与其他开发者交流吧！</p>
          </div>
        </div>
      )}
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default MessagesScreen 