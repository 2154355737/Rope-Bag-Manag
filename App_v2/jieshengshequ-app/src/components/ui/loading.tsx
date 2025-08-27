import React from 'react'
import { motion } from 'framer-motion'
import { Loader2, RefreshCw, Zap } from 'lucide-react'
import { cn } from '@/lib/utils'

interface LoadingProps {
  size?: 'sm' | 'md' | 'lg' | 'xl'
  variant?: 'spinner' | 'dots' | 'pulse' | 'wave'
  text?: string
  className?: string
  fullScreen?: boolean
}

const Loading: React.FC<LoadingProps> = ({
  size = 'md',
  variant = 'spinner',
  text,
  className,
  fullScreen = false
}) => {
  const sizeClasses = {
    sm: 'h-4 w-4',
    md: 'h-6 w-6', 
    lg: 'h-8 w-8',
    xl: 'h-12 w-12'
  }

  const textSizeClasses = {
    sm: 'text-xs',
    md: 'text-sm',
    lg: 'text-base',
    xl: 'text-lg'
  }

  const LoadingIcon = () => {
    switch (variant) {
      case 'spinner':
        return <Loader2 className={cn('animate-spin', sizeClasses[size])} />
      case 'dots':
        return (
          <div className="flex space-x-1">
            {[0, 1, 2].map((i) => (
              <motion.div
                key={i}
                className={cn(
                  'rounded-full bg-current',
                  size === 'sm' ? 'h-1 w-1' : 
                  size === 'md' ? 'h-1.5 w-1.5' :
                  size === 'lg' ? 'h-2 w-2' : 'h-3 w-3'
                )}
                animate={{
                  scale: [1, 1.2, 1],
                  opacity: [0.7, 1, 0.7]
                }}
                transition={{
                  duration: 0.6,
                  repeat: Infinity,
                  delay: i * 0.2
                }}
              />
            ))}
          </div>
        )
      case 'pulse':
        return (
          <motion.div
            className={cn('rounded-full bg-current', sizeClasses[size])}
            animate={{
              scale: [1, 1.2, 1],
              opacity: [0.5, 1, 0.5]
            }}
            transition={{
              duration: 1,
              repeat: Infinity
            }}
          />
        )
      case 'wave':
        return (
          <div className="flex items-center space-x-0.5">
            {[0, 1, 2, 3, 4].map((i) => (
              <motion.div
                key={i}
                className={cn(
                  'bg-current rounded-sm',
                  size === 'sm' ? 'h-3 w-0.5' :
                  size === 'md' ? 'h-4 w-1' :
                  size === 'lg' ? 'h-6 w-1' : 'h-8 w-1.5'
                )}
                animate={{
                  scaleY: [1, 2, 1]
                }}
                transition={{
                  duration: 0.8,
                  repeat: Infinity,
                  delay: i * 0.1
                }}
              />
            ))}
          </div>
        )
      default:
        return <Loader2 className={cn('animate-spin', sizeClasses[size])} />
    }
  }

  const content = (
    <div className={cn(
      'flex flex-col items-center justify-center space-y-2 text-muted-foreground',
      className
    )}>
      <LoadingIcon />
      {text && (
        <motion.p 
          className={cn('font-medium', textSizeClasses[size])}
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.2 }}
        >
          {text}
        </motion.p>
      )}
    </div>
  )

  if (fullScreen) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm">
        {content}
      </div>
    )
  }

  return content
}

// 预定义的常用loading样式
export const LoadingSpinner = ({ className, ...props }: Omit<LoadingProps, 'variant'>) => (
  <Loading variant="spinner" className={className} {...props} />
)

export const LoadingDots = ({ className, ...props }: Omit<LoadingProps, 'variant'>) => (
  <Loading variant="dots" className={className} {...props} />
)

export const LoadingPulse = ({ className, ...props }: Omit<LoadingProps, 'variant'>) => (
  <Loading variant="pulse" className={className} {...props} />
)

export const LoadingWave = ({ className, ...props }: Omit<LoadingProps, 'variant'>) => (
  <Loading variant="wave" className={className} {...props} />
)

// 内联loading组件
export const InlineLoading = ({ text = "加载中...", size = 'sm' }: Pick<LoadingProps, 'text' | 'size'>) => {
  const sizeClasses = {
    sm: 'h-4 w-4',
    md: 'h-6 w-6', 
    lg: 'h-8 w-8',
    xl: 'h-12 w-12'
  }

  const textSizeClasses = {
    sm: 'text-xs',
    md: 'text-sm',
    lg: 'text-base',
    xl: 'text-lg'
  }

  return (
    <div className="flex items-center space-x-2 text-muted-foreground">
      <Loader2 className={cn('animate-spin', sizeClasses[size])} />
      <span className={textSizeClasses[size]}>{text}</span>
    </div>
  )
}

// 页面加载组件
export const PageLoading = ({ text = "页面加载中..." }: Pick<LoadingProps, 'text'>) => (
  <div className="min-h-[50vh] flex items-center justify-center">
    <Loading size="lg" text={text} />
  </div>
)

export default Loading 