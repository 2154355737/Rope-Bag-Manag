import React from 'react'
import { Outlet, useLocation, useNavigate } from 'react-router-dom'
import { Home, Layers, PlusCircle, MessageSquare, User } from 'lucide-react'
import { cn } from '@/lib/utils'
import { useTheme } from '@/components/theme-provider'

const Layout: React.FC = () => {
  const location = useLocation()
  const navigate = useNavigate()
  const { theme } = useTheme()
  
  const navItems = [
    { path: '/home', icon: Home, label: '首页' },
    { path: '/category', icon: Layers, label: '分类' },
    { path: '/publish', icon: PlusCircle, label: '发布' },
    { path: '/messages', icon: MessageSquare, label: '消息' },
    { path: '/profile', icon: User, label: '我的' },
  ]

  const handleNavClick = (path: string) => {
    navigate(path)
  }

  return (
    <div className="flex flex-col min-h-screen bg-background">
      {/* 主内容区域 - 添加安全区域适配 */}
      <div className="flex-1 overflow-hidden">
        <main className="h-full overflow-y-auto pb-16 scroll-container"> {/* 为底部导航留出空间 */}
          <Outlet />
        </main>
      </div>
      
      {/* 底部导航栏 - 固定在底部，包含安全区域 */}
      <div className="fixed bottom-0 left-0 right-0 bg-background border-t border-border pb-safe z-[9999] bottom-navigation">
        <div className="flex items-center justify-around h-16">
          {navItems.map((item) => {
            const Icon = item.icon
            const isActive = location.pathname === item.path || 
                           (item.path === '/home' && location.pathname === '/')
            
            return (
              <button
                key={item.path}
                onClick={() => handleNavClick(item.path)}
                className={cn(
                  "flex flex-col items-center justify-center flex-1 h-full space-y-1 text-xs transition-colors",
                  isActive 
                    ? "text-primary" 
                    : "text-muted-foreground hover:text-foreground"
                )}
              >
                <Icon size={20} />
                <span>{item.label}</span>
              </button>
            )
          })}
        </div>
      </div>
    </div>
  )
}

export default Layout