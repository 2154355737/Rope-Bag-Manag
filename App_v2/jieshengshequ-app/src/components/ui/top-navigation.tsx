import React from 'react'
import { ArrowLeft, Search, Bell, MoreVertical, Settings } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { useNavigate } from 'react-router-dom'

export interface TopNavigationProps {
  title?: string
  subtitle?: string
  showBackButton?: boolean
  showSearchButton?: boolean
  showNotificationButton?: boolean
  showSettingsButton?: boolean
  showMoreButton?: boolean
  notificationCount?: number
  leftAction?: React.ReactNode
  rightAction?: React.ReactNode
  className?: string
  variant?: 'default' | 'transparent' | 'primary' | 'secondary'
  onBackClick?: () => void
  onSearchClick?: () => void
  onNotificationClick?: () => void
  onSettingsClick?: () => void
  onMoreClick?: () => void
}

const TopNavigation: React.FC<TopNavigationProps> = ({
  title,
  subtitle,
  showBackButton = false,
  showSearchButton = false,
  showNotificationButton = false,
  showSettingsButton = false,
  showMoreButton = false,
  notificationCount = 0,
  leftAction,
  rightAction,
  className,
  variant = 'default',
  onBackClick,
  onSearchClick,
  onNotificationClick,
  onSettingsClick,
  onMoreClick,
}) => {
  const navigate = useNavigate()

  const handleBackClick = () => {
    if (onBackClick) {
      onBackClick()
    } else {
      navigate(-1)
    }
  }

  const handleSearchClick = () => {
    if (onSearchClick) {
      onSearchClick()
    } else {
      // 默认搜索行为
      console.log('Search clicked')
    }
  }

  const handleNotificationClick = () => {
    if (onNotificationClick) {
      onNotificationClick()
    } else {
      navigate('/messages')
    }
  }

  const handleSettingsClick = () => {
    if (onSettingsClick) {
      onSettingsClick()
    } else {
      navigate('/settings')
    }
  }

  const handleMoreClick = () => {
    if (onMoreClick) {
      onMoreClick()
    } else {
      console.log('More clicked')
    }
  }

  // 根据变体设置样式
  const getVariantStyles = () => {
    switch (variant) {
      case 'transparent':
        return 'bg-transparent backdrop-blur-sm border-transparent'
      case 'primary':
        return 'bg-primary text-primary-foreground border-primary/20'
      case 'secondary':
        return 'bg-secondary text-secondary-foreground border-secondary/20'
      default:
        return 'bg-background/95 backdrop-blur-sm border-border/50'
    }
  }

  return (
    <div
      className={cn(
        'fixed top-0 left-0 right-0 z-[9998] w-full border-b transition-all duration-200',
        'pt-safe', // 安全域适配
        getVariantStyles(),
        className
      )}
    >
      <div className="flex items-center justify-between px-4 py-3 min-h-[56px]">
        {/* 左侧区域 */}
        <div className="flex items-center gap-2 flex-1">
          {leftAction || (
            <>
              {showBackButton && (
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-9 w-9 shrink-0"
                  onClick={handleBackClick}
                >
                  <ArrowLeft size={20} />
                </Button>
              )}
            </>
          )}
          
          {/* 标题区域 */}
          {(title || subtitle) && (
            <div className="flex flex-col min-w-0 flex-1">
              {title && (
                <div className="flex items-center gap-2.5">
                  {/* 简洁装饰线 */}
                  <div className="w-0.5 h-4 bg-primary rounded-full flex-shrink-0"></div>
                  <h1 className="text-lg font-semibold truncate leading-tight">
                    {title}
                  </h1>
                </div>
              )}
              {subtitle && (
                <p className="text-sm text-muted-foreground/80 truncate leading-tight ml-3 mt-0.5">
                  {subtitle}
                </p>
              )}
            </div>
          )}
        </div>

        {/* 右侧区域 */}
        <div className="flex items-center gap-1 shrink-0">
          {rightAction || (
            <>
              {showSearchButton && (
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-9 w-9"
                  onClick={handleSearchClick}
                >
                  <Search size={20} />
                </Button>
              )}
              
              {showNotificationButton && (
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-9 w-9 relative"
                  onClick={handleNotificationClick}
                >
                  <Bell size={20} />
                  {notificationCount > 0 && (
                    <Badge 
                      className="absolute -top-1 -right-1 h-5 w-5 p-0 flex items-center justify-center text-xs bg-red-500 hover:bg-red-500"
                    >
                      {notificationCount > 99 ? '99+' : notificationCount}
                    </Badge>
                  )}
                </Button>
              )}
              
              {showSettingsButton && (
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-9 w-9"
                  onClick={handleSettingsClick}
                >
                  <Settings size={20} />
                </Button>
              )}
              
              {showMoreButton && (
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-9 w-9"
                  onClick={handleMoreClick}
                >
                  <MoreVertical size={20} />
                </Button>
              )}
            </>
          )}
        </div>
      </div>
    </div>
  )
}

export default TopNavigation 