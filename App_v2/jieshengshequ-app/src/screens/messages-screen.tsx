import React from 'react'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { MessageCircle, Search, MoreVertical } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

const MessagesScreen: React.FC = () => {
  const conversations = [
    {
      id: 1,
      name: '张三',
      avatar: '/api/placeholder/40/40',
      lastMessage: '你好，关于React的问题...',
      time: '2分钟前',
      unread: 2,
      online: true
    },
    {
      id: 2,
      name: '李四',
      avatar: '/api/placeholder/40/40',
      lastMessage: '项目进展如何？',
      time: '1小时前',
      unread: 0,
      online: false
    },
    {
      id: 3,
      name: 'JavaScript交流群',
      avatar: '/api/placeholder/40/40',
      lastMessage: '王五: 有人用过Next.js吗？',
      time: '3小时前',
      unread: 5,
      online: false,
      isGroup: true
    },
    {
      id: 4,
      name: '前端开发者社区',
      avatar: '/api/placeholder/40/40',
      lastMessage: '新的技术分享已发布',
      time: '昨天',
      unread: 1,
      online: false,
      isGroup: true
    }
  ]

  return (
    <div className="flex flex-col h-full bg-background">
      {/* 头部 */}
      <div className="flex items-center justify-between p-4 border-b">
        <h1 className="text-xl font-bold">消息</h1>
        <Button variant="ghost" size="icon">
          <MoreVertical className="h-5 w-5" />
        </Button>
      </div>

      {/* 搜索栏 */}
      <div className="p-4 border-b">
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
      <div className="flex-1 overflow-y-auto">
        {conversations.map((conversation) => (
          <Card key={conversation.id} className="m-2 cursor-pointer hover:bg-muted/50 transition-colors">
            <CardContent className="p-4">
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
                    <h3 className="font-medium truncate">{conversation.name}</h3>
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
    </div>
  )
}

export default MessagesScreen 