import React from 'react'
import { motion } from 'framer-motion'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { User, Lock, Heart } from 'lucide-react'
import { useNavigate } from 'react-router-dom'

interface GuestPromptProps {
  title?: string
  description?: string
  actionText?: string
  onAction?: () => void
  showLoginButton?: boolean
  className?: string
}

const GuestPrompt: React.FC<GuestPromptProps> = ({
  title = "需要登录",
  description = "登录后可以享受更多功能",
  actionText = "立即登录", 
  onAction,
  showLoginButton = true,
  className = ""
}) => {
  const navigate = useNavigate()

  const handleLogin = () => {
    if (onAction) {
      onAction()
    } else {
      navigate('/login')
    }
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
      className={`flex items-center justify-center p-6 ${className}`}
    >
      <Card className="w-full max-w-md">
        <CardContent className="pt-6">
          <div className="text-center space-y-4">
            <motion.div
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              transition={{ delay: 0.1, type: "spring", stiffness: 200 }}
              className="flex justify-center"
            >
              <div className="relative">
                <div className="w-16 h-16 rounded-full bg-primary/10 flex items-center justify-center">
                  <Lock size={24} className="text-primary" />
                </div>
                <motion.div
                  animate={{ 
                    scale: [1, 1.2, 1],
                    opacity: [0.5, 1, 0.5] 
                  }}
                  transition={{ 
                    duration: 2, 
                    repeat: Infinity, 
                    ease: "easeInOut" 
                  }}
                  className="absolute -top-1 -right-1 w-6 h-6 rounded-full bg-accent/20 flex items-center justify-center"
                >
                  <Heart size={12} className="text-accent" />
                </motion.div>
              </div>
            </motion.div>

            <div className="space-y-2">
              <h3 className="text-lg font-semibold text-foreground">
                {title}
              </h3>
              <p className="text-muted-foreground text-sm">
                {description}
              </p>
            </div>

            <div className="space-y-3 pt-2">
              {showLoginButton && (
                <Button 
                  onClick={handleLogin}
                  className="w-full"
                  size="sm"
                >
                  <User size={16} className="mr-2" />
                  {actionText}
                </Button>
              )}
              
              <Button 
                variant="ghost"
                size="sm"
                onClick={() => navigate('/register')}
                className="w-full text-xs"
              >
                还没有账号？立即注册
              </Button>
            </div>

            <div className="pt-2 border-t">
              <p className="text-xs text-muted-foreground">
                ✨ 登录后可以发布内容、收藏资源、参与讨论
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  )
}

export default GuestPrompt 