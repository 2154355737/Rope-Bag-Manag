import React from 'react'
import { Outlet, useLocation, useNavigate } from 'react-router-dom'
import { Home, Layers, PlusCircle, MessageSquare, User } from 'lucide-react'
import { cn } from '@/lib/utils'
import { useTheme } from '@/components/theme-provider'
import { useModernKeyboard } from '@/hooks/use-modern-keyboard'

const Layout: React.FC = () => {
  const location = useLocation()
  const navigate = useNavigate()
  const { theme } = useTheme()
  
  // 使用现代化键盘处理
  const keyboard = useModernKeyboard({
    debug: import.meta.env.DEV, // 开发环境启用调试
    onStateChange: (state) => {
      console.log('🎹 键盘状态变化:', state)
    }
  })
  
  const navItems = [
    { icon: Home, label: '首页', path: '/home' },
    { icon: Layers, label: '分类', path: '/category' },
    { icon: PlusCircle, label: '发布', path: '/publish' },
    { icon: MessageSquare, label: '消息', path: '/messages' },
    { icon: User, label: '我的', path: '/profile' },
  ]

  return (
    <div className={cn("modern-app-container bg-background", keyboard.isOpen && "modern-keyboard-open")}>
      <main className="modern-main-content safe-area-container">
        <Outlet />
      </main>
      
      <nav className="modern-bottom-nav border-t bg-background">
        <div className="flex items-center justify-around h-16">
          {navItems.map((item, index) => {
            const isActive = location.pathname === item.path
            const isPublish = item.path === '/publish'
            
            return (
              <button
                key={index}
                onClick={() => navigate(item.path)}
                className={cn(
                  "flex flex-col items-center justify-center w-full h-full",
                  isActive && "text-primary"
                )}
              >
                {isPublish ? (
                  <div className="flex items-center justify-center w-12 h-12 rounded-full bg-primary text-primary-foreground -mt-6">
                    <item.icon size={24} />
                  </div>
                ) : (
                  <item.icon size={20} className={cn(isActive ? "text-primary" : "text-muted-foreground")} />
                )}
                <span className={cn(
                  "text-xs mt-1",
                  isActive ? "text-primary" : "text-muted-foreground",
                  isPublish && "mt-0"
                )}>
                  {item.label}
                </span>
              </button>
            )
          })}
        </div>
      </nav>
    </div>
  )
}

export default Layout