import React from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { 
  Eye, Heart, MessageSquare, Clock, User, Star, Download, 
  ChevronRight, BookOpen, FileText, Megaphone 
} from 'lucide-react'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'

export interface RecommendedItem {
  id: number
  type: 'post' | 'resource' | 'announcement'
  title: string
  description?: string
  author?: {
    name: string
    avatar: string
    verified?: boolean
  }
  image?: string
  tags?: string[]
  stats: {
    likes?: number
    comments?: number
    views?: number
    downloads?: number
    rating?: number
  }
  time: string
  isHot?: boolean
  isPinned?: boolean
}

interface RelatedRecommendationsProps {
  title?: string
  items: RecommendedItem[]
  currentItemId?: number
  className?: string
  maxItems?: number
  showMoreButton?: boolean
  onMoreClick?: () => void
  compact?: boolean
}

const RelatedRecommendations: React.FC<RelatedRecommendationsProps> = ({
  title = "相关推荐",
  items,
  currentItemId,
  className = "",
  maxItems = 6,
  showMoreButton = true,
  onMoreClick,
  compact = false
}) => {
  const navigate = useNavigate()

  // 过滤掉当前正在查看的内容
  const filteredItems = items.filter(item => item.id !== currentItemId).slice(0, maxItems)

  // 格式化数字显示
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 获取内容类型图标
  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'post': return FileText
      case 'resource': return BookOpen
      case 'announcement': return Megaphone
      default: return FileText
    }
  }

  // 获取内容类型颜色
  const getTypeColor = (type: string) => {
    switch (type) {
      case 'post': return 'text-blue-600 bg-blue-50'
      case 'resource': return 'text-green-600 bg-green-50'
      case 'announcement': return 'text-orange-600 bg-orange-50'
      default: return 'text-gray-600 bg-gray-50'
    }
  }

  // 处理点击事件
  const handleItemClick = (item: RecommendedItem) => {
    switch (item.type) {
      case 'post':
        navigate(`/post/${item.id}`)
        break
      case 'resource':
        navigate(`/resource/${item.id}`)
        break
      case 'announcement':
        navigate(`/announcement/${item.id}`)
        break
      default:
        break
    }
  }

  if (filteredItems.length === 0) {
    return null
  }

  return (
    <div className={`space-y-4 ${className}`}>
      {/* 标题栏 */}
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-medium flex items-center">
          <div className="w-1 h-5 bg-primary rounded-full mr-2" />
          {title}
        </h3>
        {showMoreButton && onMoreClick && (
          <Button
            variant="ghost"
            size="sm"
            onClick={onMoreClick}
            className="text-muted-foreground hover:text-foreground"
          >
            查看更多
            <ChevronRight size={14} className="ml-1" />
          </Button>
        )}
      </div>

      {/* 推荐列表 */}
      <div className={compact ? "space-y-2" : "grid grid-cols-1 md:grid-cols-2 gap-4"}>
        {filteredItems.map((item, index) => {
          const TypeIcon = getTypeIcon(item.type)
          
          return (
            <motion.div
              key={item.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.1, duration: 0.3 }}
            >
              <Card 
                className="overflow-hidden cursor-pointer hover:shadow-md transition-all duration-200 group"
                onClick={() => handleItemClick(item)}
              >
                <CardContent className={compact ? "p-3" : "p-4"}>
                  {compact ? (
                    // 紧凑模式：单行布局
                    <div className="flex items-center gap-3">
                      <div className={`p-1.5 rounded-full ${getTypeColor(item.type)} flex-shrink-0`}>
                        <TypeIcon size={12} />
                      </div>
                      <div className="flex-1 min-w-0">
                        <h4 className="font-medium text-sm line-clamp-1 group-hover:text-primary transition-colors">
                          {item.title}
                        </h4>
                        <div className="flex items-center gap-2 mt-1">
                          <span className="text-xs text-muted-foreground">
                            {typeof item.author?.name === 'string' ? item.author.name : '用户'}
                          </span>
                          <span className="text-xs text-muted-foreground">•</span>
                          <span className="text-xs text-muted-foreground">{item.time}</span>
                        </div>
                      </div>
                      <div className="flex items-center gap-2 text-xs text-muted-foreground flex-shrink-0">
                        {item.stats.views !== undefined && (
                          <div className="flex items-center">
                            <Eye size={12} className="mr-1" />
                            <span>{item.stats.views}</span>
                          </div>
                        )}
                        {item.stats.likes !== undefined && (
                          <div className="flex items-center">
                            <Heart size={12} className="mr-1" />
                            <span>{item.stats.likes}</span>
                          </div>
                        )}
                      </div>
                    </div>
                  ) : (
                    <>
                      {/* 内容头部 */}
                      <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-2">
                      <div className={`p-1.5 rounded-full ${getTypeColor(item.type)}`}>
                        <TypeIcon size={12} />
                      </div>
                      <Badge variant="outline" className="text-xs">
                        {item.type === 'post' && '帖子'}
                        {item.type === 'resource' && '资源'}
                        {item.type === 'announcement' && '公告'}
                      </Badge>
                      {item.isHot && (
                        <Badge className="bg-red-500 text-white text-xs">热门</Badge>
                      )}
                      {item.isPinned && (
                        <Badge className="bg-orange-500 text-white text-xs">置顶</Badge>
                      )}
                    </div>
                    <span className="text-xs text-muted-foreground">{item.time}</span>
                  </div>

                  {/* 标题和描述 */}
                  <h4 className="font-medium text-sm line-clamp-2 mb-2 group-hover:text-primary transition-colors">
                    {item.title}
                  </h4>
                  
                  {item.description && (
                    <p className="text-xs text-muted-foreground line-clamp-2 mb-3">
                      {item.description}
                    </p>
                  )}

                  {/* 图片 */}
                  {item.image && (
                    <div className="mb-3">
                      <img 
                        src={item.image} 
                        alt={item.title}
                        className="w-full h-32 object-cover rounded-md"
                      />
                    </div>
                  )}

                  {/* 作者信息 */}
                  {item.author && item.author.name && (
                    <div className="flex items-center mb-3">
                      <Avatar className="h-5 w-5 mr-2">
                        <AvatarImage src={item.author.avatar} />
                        <AvatarFallback className="text-xs">
                          {typeof item.author.name === 'string' ? item.author.name[0] : 'U'}
                        </AvatarFallback>
                      </Avatar>
                      <span className="text-xs text-muted-foreground">
                        {typeof item.author.name === 'string' ? item.author.name : '用户'}
                      </span>
                      {item.author.verified && (
                        <div className="w-3 h-3 bg-blue-500 rounded-full flex items-center justify-center ml-1">
                          <div className="w-1.5 h-1.5 bg-white rounded-full" />
                        </div>
                      )}
                    </div>
                  )}

                  {/* 标签 */}
                  {item.tags && item.tags.length > 0 && (
                    <div className="flex flex-wrap gap-1 mb-3">
                      {item.tags.slice(0, 3).map((tag, idx) => (
                        <Badge key={idx} variant="secondary" className="text-xs">
                          {tag}
                        </Badge>
                      ))}
                      {item.tags.length > 3 && (
                        <span className="text-xs text-muted-foreground">+{item.tags.length - 3}</span>
                      )}
                    </div>
                  )}

                  {/* 统计信息 */}
                  <div className="flex items-center gap-3 text-xs text-muted-foreground">
                    {item.stats.views && (
                      <div className="flex items-center">
                        <Eye size={12} className="mr-1" />
                        {formatNumber(item.stats.views)}
                      </div>
                    )}
                    {item.stats.likes && (
                      <div className="flex items-center">
                        <Heart size={12} className="mr-1" />
                        {formatNumber(item.stats.likes)}
                      </div>
                    )}
                    {item.stats.comments && (
                      <div className="flex items-center">
                        <MessageSquare size={12} className="mr-1" />
                        {formatNumber(item.stats.comments)}
                      </div>
                    )}
                    {item.stats.downloads && (
                      <div className="flex items-center">
                        <Download size={12} className="mr-1" />
                        {formatNumber(item.stats.downloads)}
                      </div>
                    )}
                    {item.stats.rating && (
                      <div className="flex items-center">
                        <Star size={12} className="mr-1 fill-yellow-400 text-yellow-400" />
                        {item.stats.rating.toFixed(1)}
                      </div>
                    )}
                  </div>
                      </>
                    )}
                </CardContent>
              </Card>
            </motion.div>
          )
        })}
      </div>
    </div>
  )
}

export default RelatedRecommendations 