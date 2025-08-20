import React, { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import { Code, ChevronRight } from 'lucide-react'

interface SplashScreenProps {
  onSkip: () => void
}

const SplashScreen: React.FC<SplashScreenProps> = ({ onSkip }) => {
  const [progress, setProgress] = useState(0)

  useEffect(() => {
    const timer = setInterval(() => {
      setProgress((oldProgress) => {
        if (oldProgress === 100) {
          clearInterval(timer)
          return 100
        }
        const diff = Math.random() * 10
        return Math.min(oldProgress + diff, 100)
      })
    }, 200)

    return () => {
      clearInterval(timer)
    }
  }, [])

  return (
    <div className="fullscreen-content bg-background">
      <div className="content-area flex flex-col items-center justify-center text-foreground relative overflow-hidden">
        <motion.div
          initial={{ scale: 0.8, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ duration: 0.8, ease: "easeOut" }}
          className="flex flex-col items-center"
        >
          {/* Logo with pulse gradient animation */}
          <motion.div 
            className="relative flex items-center justify-center w-32 h-32 mb-8"
            initial={{ scale: 0.9 }}
            animate={{ scale: [0.9, 1.1, 0.9] }}
            transition={{ 
              duration: 2.5, 
              repeat: Infinity, 
              ease: "easeInOut" 
            }}
          >
            {/* Gradient background with pulse */}
            <motion.div
              className="absolute inset-0 rounded-full opacity-80"
              style={{
                background: 'linear-gradient(135deg, hsl(var(--primary)) 0%, hsl(var(--accent)) 50%, hsl(var(--secondary)) 100%)'
              }}
              animate={{
                scale: [1, 1.2, 1],
                opacity: [0.8, 0.4, 0.8]
              }}
              transition={{
                duration: 2.5,
                repeat: Infinity,
                ease: "easeInOut"
              }}
            />
            
            {/* Inner glow effect */}
            <motion.div
              className="absolute inset-2 rounded-full bg-primary/20"
              animate={{
                scale: [1, 1.15, 1],
                opacity: [0.6, 0.2, 0.6]
              }}
              transition={{
                duration: 2.5,
                repeat: Infinity,
                ease: "easeInOut",
                delay: 0.3
              }}
            />
            
            {/* Logo icon */}
            <div className="relative z-10 flex items-center justify-center w-20 h-20 rounded-full bg-background/95 backdrop-blur-sm shadow-xl">
              <Code size={40} className="text-primary" />
            </div>
          </motion.div>
          
          <motion.h1 
            initial={{ y: 30, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.4, duration: 0.8, ease: "easeOut" }}
            className="text-4xl font-bold mb-3 bg-gradient-to-r from-primary via-accent to-secondary bg-clip-text text-transparent"
          >
            结绳社区
          </motion.h1>
          
          <motion.p
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.6, duration: 0.8, ease: "easeOut" }}
            className="text-muted-foreground mb-12 text-center text-lg font-medium"
          >
            程序员的移动端社区
          </motion.p>
        </motion.div>

        {/* Progress bar */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.8, duration: 0.6 }}
          className="w-full max-w-xs mb-8"
        >
          <Progress 
            value={progress} 
            className="h-2 bg-muted/50"
            style={{
              '--progress-background': 'linear-gradient(90deg, hsl(var(--primary)), hsl(var(--accent)))'
            } as React.CSSProperties}
          />
          <div className="flex justify-between text-xs text-muted-foreground mt-2">
            <span>正在加载</span>
            <span>{Math.round(progress)}%</span>
          </div>
        </motion.div>

        {/* Skip button */}
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 1.0, duration: 0.6 }}
          className="absolute bottom-safe-12"
        >
          <Button
            variant="ghost"
            size="sm"
            onClick={onSkip}
            className="text-muted-foreground hover:text-foreground transition-colors duration-300 group"
          >
            跳过 
            <motion.div
              animate={{ x: [0, 4, 0] }}
              transition={{ duration: 1.5, repeat: Infinity, ease: "easeInOut" }}
            >
              <ChevronRight size={16} className="ml-1 group-hover:translate-x-1 transition-transform duration-300" />
            </motion.div>
          </Button>
        </motion.div>
      </div>
    </div>
  )
}

export default SplashScreen 