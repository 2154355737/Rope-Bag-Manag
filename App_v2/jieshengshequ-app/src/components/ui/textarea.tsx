import * as React from "react"

import { cn } from "@/lib/utils"

export interface TextareaProps extends React.ComponentProps<"textarea"> {
  /**
   * 是否启用键盘遮挡自动滚动
   * @default true
   */
  autoScrollOnFocus?: boolean
}

const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, autoScrollOnFocus = true, ...props }, ref) => {
    const textareaRef = React.useRef<HTMLTextAreaElement>(null)
    
    // 合并 refs
    React.useImperativeHandle(ref, () => textareaRef.current!, [])
    
    // 处理焦点事件
    const handleFocus = React.useCallback((event: React.FocusEvent<HTMLTextAreaElement>) => {
      // 在原生平台上，让平台级别的键盘处理逻辑来处理滚动
      // 在Web平台上，使用 scrollIntoView 作为备选方案
      if (autoScrollOnFocus && textareaRef.current) {
        // 检查是否在原生平台
        const isNative = (window as any).Capacitor?.isNativePlatform?.() || false
        
        if (!isNative) {
          // 仅在Web平台使用 scrollIntoView
          setTimeout(() => {
            if (textareaRef.current) {
              textareaRef.current.scrollIntoView({
                behavior: 'smooth',
                block: 'center',
                inline: 'nearest'
              })
            }
          }, 300)
        }
        // 原生平台的滚动由 platform.ts 中的键盘事件处理
      }
      
      // 调用原始的 onFocus 处理器
      if (props.onFocus) {
        props.onFocus(event)
      }
    }, [autoScrollOnFocus, props.onFocus])

    return (
      <textarea
        className={cn(
          "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
          className
        )}
        ref={textareaRef}
        onFocus={handleFocus}
        {...props}
      />
    )
  }
)
Textarea.displayName = "Textarea"

export { Textarea }
