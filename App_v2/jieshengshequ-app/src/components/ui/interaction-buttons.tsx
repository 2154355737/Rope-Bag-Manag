import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Heart, Share2, Bookmark, Flag, ThumbsUp
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { cn } from '@/lib/utils'

export interface InteractionButtonProps {
  icon: React.ComponentType<any>
  label: string
  count?: number
  isActive?: boolean
  variant?: 'like' | 'bookmark' | 'share' | 'report'
  onClick?: () => void
}

interface InteractionButtonsProps {
  buttons: InteractionButtonProps[]
  className?: string
  compact?: boolean
}

const InteractionButtons: React.FC<InteractionButtonsProps> = ({
  buttons,
  className = "",
  compact = true
}) => {
  const [pressedButton, setPressedButton] = useState<string | null>(null)

  const formatCount = (count: number) => {
    if (count >= 10000) return `${(count / 10000).toFixed(1)}万`
    if (count >= 1000) return `${(count / 1000).toFixed(1)}k`
    return count.toString()
  }

  const handleButtonPress = (button: InteractionButtonProps, index: number) => {
    setPressedButton(button.label)
    setTimeout(() => setPressedButton(null), 150)
    
    try { console.debug('[interaction]', button.variant || 'custom', button.label) } catch {}
    button.onClick?.()
    
    // 触觉反馈（移动端）
    if ('vibrate' in navigator) {
      navigator.vibrate(5)
    }
  }

  const getButtonVariant = (button: InteractionButtonProps) => {
    if (button.isActive) {
      switch (button.variant) {
        case 'like': return 'like-active'
        case 'bookmark': return 'bookmark-active'
        default: return button.variant || 'minimal'
      }
    }
    return button.variant || 'minimal'
  }

  const renderButton = (button: InteractionButtonProps, index: number) => {
    const IconComponent = button.icon
    const isPressed = pressedButton === button.label
    
    return (
      <motion.div
        key={button.label}
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.2, delay: index * 0.05 }}
        className="flex-1"
      >
        <Button
          variant={getButtonVariant(button) as any}
          size={compact ? "xs" : "sm"}
          className={cn(
            "flex flex-col items-center h-auto w-full",
            compact ? "p-2" : "p-3"
          )}
          onClick={() => handleButtonPress(button, index)}
        >
          <motion.div
            className="flex items-center justify-center"
            animate={isPressed ? { scale: [1, 1.2, 1] } : {}}
            transition={{ duration: 0.15 }}
          >
            <IconComponent 
              size={compact ? 16 : 18} 
              className={cn(
                "transition-all duration-200",
                button.isActive && button.variant === 'like' ? "fill-current" : "",
                button.isActive && button.variant === 'bookmark' ? "fill-current" : ""
              )}
            />
          </motion.div>

          <div className="flex flex-col items-center mt-0.5">
            <span className={cn(
              "font-medium leading-none",
              compact ? "text-[10px]" : "text-xs"
            )}>
              {button.label}
            </span>
            
            {button.count !== undefined && (
              <motion.span
                className={cn(
                  "leading-none mt-0.5",
                  compact ? "text-[9px]" : "text-[10px]"
                )}
                animate={button.isActive ? { scale: [1, 1.1, 1] } : {}}
                transition={{ duration: 0.2 }}
              >
                {formatCount(button.count)}
              </motion.span>
            )}
          </div>
        </Button>
      </motion.div>
    )
  }

  return (
    <Card className={cn("border-border/50 bg-background/80 backdrop-blur-sm", className)}>
      <CardContent className={compact ? "p-3" : "p-4"}>
        <div className="flex items-center gap-1">
          {buttons.map((button, index) => renderButton(button, index))}
        </div>
      </CardContent>
    </Card>
  )
}

// 便捷创建函数
export const createLikeButton = (
  count: number = 0, 
  isLiked: boolean = false, 
  onClick?: () => void
): InteractionButtonProps => ({
  icon: Heart,
  label: '点赞',
  count,
  isActive: isLiked,
  variant: 'like',
  onClick
})

export const createBookmarkButton = (
  count?: number, 
  isBookmarked: boolean = false, 
  onClick?: () => void
): InteractionButtonProps => ({
  icon: Bookmark,
  label: '收藏',
  count,
  isActive: isBookmarked,
  variant: 'bookmark',
  onClick
})

export const createShareButton = (onClick?: () => void): InteractionButtonProps => ({
  icon: Share2,
  label: '分享',
  variant: 'share',
  onClick
})

export const createReportButton = (onClick?: () => void): InteractionButtonProps => ({
  icon: Flag,
  label: '举报',
  variant: 'report',
  onClick
})

export const createThumbsUpButton = (
  count: number = 0, 
  isLiked: boolean = false, 
  onClick?: () => void
): InteractionButtonProps => ({
  icon: ThumbsUp,
  label: '点赞',
  count,
  isActive: isLiked,
  variant: 'like',
  onClick
})

export default InteractionButtons 