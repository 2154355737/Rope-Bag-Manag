import React from 'react'
import { Outlet, useLocation, useNavigate } from 'react-router-dom'
import { Home, Layers, PlusCircle, MessageSquare, User } from 'lucide-react'
import { cn } from '@/lib/utils'
import { useTheme } from '@/components/theme-provider'
import { motion, AnimatePresence } from 'framer-motion'

const Layout: React.FC = () => {
  const location = useLocation()
  const navigate = useNavigate()
  const { theme } = useTheme()
  
  const navItems = [
    { path: '/home', icon: Home, label: '首页' },
    { path: '/category', icon: Layers, label: '分类' },
    { path: '/publish', icon: PlusCircle, label: '发布' },
    { path: '/messages', icon: MessageSquare, label: '通知' },
    { path: '/profile', icon: User, label: '我的' },
  ]

  const handleNavClick = (path: string) => {
    navigate(path)
  }

  return (
    <div className="flex flex-col min-h-screen bg-background">
      {/* 主内容区域 - 添加安全区域适配 */}
      <div className="flex-1 overflow-hidden">
        <main className="h-full overflow-y-auto pb-20 scroll-container"> {/* 为底部导航留出空间 */}
          <Outlet />
        </main>
      </div>
      
      {/* 底部导航栏 - 现代化设计 */}
      <div className="fixed bottom-0 left-0 right-0 z-[9999] bottom-navigation">
        {/* 背景层 - 毛玻璃效果 */}
        <div className="absolute inset-0 bg-background/80 backdrop-blur-xl border-t border-border/50" />
        
        {/* 导航内容 */}
        <div className="relative px-2 pb-safe">
          <div className="flex items-center justify-around h-16">
            {navItems.map((item, index) => {
              const Icon = item.icon
              const isActive = location.pathname === item.path || 
                             (item.path === '/home' && location.pathname === '/')
              
              return (
                <motion.button
                  key={item.path}
                  onClick={() => handleNavClick(item.path)}
                  className="relative flex flex-col items-center justify-center flex-1 h-full py-2"
                  whileTap={{ scale: 0.95 }}
                  initial={false}
                >
                  {/* 活跃指示器背景 */}
                  <AnimatePresence>
                    {isActive && (
                      <motion.div
                        className="absolute inset-x-2 inset-y-1 rounded-2xl bg-primary/10"
                        initial={{ scale: 0.8, opacity: 0 }}
                        animate={{ scale: 1, opacity: 1 }}
                        exit={{ scale: 0.8, opacity: 0 }}
                        transition={{ duration: 0.2, ease: "easeInOut" }}
                        layoutId="nav-indicator"
                      />
                    )}
                  </AnimatePresence>
                  
                  {/* 图标容器 */}
                  <div className="relative flex flex-col items-center space-y-1 z-10">
                    <motion.div
                      className="relative"
                      animate={isActive ? { y: -2 } : { y: 0 }}
                      transition={{ duration: 0.2, ease: "easeInOut" }}
                    >
                      <Icon 
                        size={20} 
                        className={cn(
                          "transition-all duration-200",
                          isActive 
                            ? "text-primary stroke-[2.5]" 
                            : "text-muted-foreground hover:text-foreground stroke-[2]"
                        )}
                      />
                      
                      {/* 活跃状态的小点指示器 */}
                      <AnimatePresence>
                        {isActive && (
                          <motion.div
                            className="absolute -top-1 -right-1 w-2 h-2 bg-primary rounded-full"
                            initial={{ scale: 0 }}
                            animate={{ scale: 1 }}
                            exit={{ scale: 0 }}
                            transition={{ delay: 0.1, duration: 0.2 }}
                          />
                        )}
                      </AnimatePresence>
                    </motion.div>
                    
                    {/* 标签文字 */}
                    <motion.span
                      className={cn(
                        "text-xs font-medium transition-all duration-200",
                        isActive 
                          ? "text-primary opacity-100" 
                          : "text-muted-foreground opacity-80"
                      )}
                      animate={isActive ? { scale: 1.05 } : { scale: 1 }}
                      transition={{ duration: 0.2 }}
                    >
                      {item.label}
                    </motion.span>
                  </div>
                  
                  {/* 特殊处理发布按钮 */}
                  {item.path === '/publish' && (
                    <>
                      <motion.div
                        className="absolute inset-x-2 inset-y-1 rounded-2xl bg-gradient-to-br from-primary/20 via-accent/10 to-secondary/20"
                        animate={{ 
                          opacity: isActive ? 0.8 : 0.4,
                          scale: isActive ? 1.05 : 1
                        }}
                        transition={{ duration: 0.3 }}
                      />
                      {!isActive && (
                        <motion.div
                          className="absolute top-1 right-2 w-1 h-1 bg-accent rounded-full"
                          animate={{ 
                            scale: [1, 1.2, 1],
                            opacity: [0.6, 1, 0.6]
                          }}
                          transition={{ 
                            duration: 2, 
                            repeat: Infinity, 
                            ease: "easeInOut" 
                          }}
                        />
                      )}
                    </>
                  )}
                  
                  {/* 悬停效果 */}
                  <motion.div
                    className="absolute inset-x-3 inset-y-2 rounded-xl bg-foreground/5"
                    initial={{ scale: 0, opacity: 0 }}
                    whileHover={{ scale: 1, opacity: 1 }}
                    transition={{ duration: 0.15 }}
                  />
                </motion.button>
              )
            })}
          </div>
        </div>
        
        {/* 底部装饰线 - 移到导航栏内部 */}
        <motion.div 
          className="absolute bottom-safe left-1/2 transform -translate-x-1/2 w-12 h-0.5 bg-gradient-to-r from-primary via-accent to-secondary rounded-full opacity-40"
          animate={{ 
            opacity: [0.2, 0.6, 0.2],
            scaleX: [0.8, 1, 0.8]
          }}
          transition={{ 
            duration: 4, 
            repeat: Infinity, 
            ease: "easeInOut" 
          }}
        />
      </div>
    </div>
  )
}

export default Layout