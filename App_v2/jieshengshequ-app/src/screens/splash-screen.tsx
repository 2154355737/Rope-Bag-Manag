import React, { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { Progress } from '@/components/ui/progress'
import { Code } from 'lucide-react'

interface SplashScreenProps {}

const SplashScreen: React.FC<SplashScreenProps> = () => {
  const [progress, setProgress] = useState(0)

  useEffect(() => {
    const timer = setInterval(() => {
      setProgress((oldProgress) => {
        if (oldProgress === 100) {
          clearInterval(timer)
          return 100
        }
        const diff = Math.random() * 12
        return Math.min(oldProgress + diff, 100)
      })
    }, 120)

    return () => {
      clearInterval(timer)
    }
  }, [])

  return (
    <div className="fullscreen-content bg-gradient-to-br from-background via-background to-primary/5">
      <div className="content-area flex flex-col items-center justify-center text-foreground relative overflow-hidden">
        {/* 背景装饰 */}
        <div className="absolute inset-0 overflow-hidden">
          {/* 网格背景 */}
          <div className="absolute inset-0 opacity-30" style={{
            backgroundImage: `
              radial-gradient(circle at 25% 25%, hsl(var(--primary) / 0.05) 0%, transparent 50%),
              radial-gradient(circle at 75% 75%, hsl(var(--accent) / 0.05) 0%, transparent 50%)
            `
          }} />
          
          {/* 浮动粒子 */}
          {[...Array(12)].map((_, i) => (
            <motion.div
              key={i}
              className="absolute w-1 h-1 bg-primary/20 rounded-full"
              style={{
                left: `${Math.random() * 100}%`,
                top: `${Math.random() * 100}%`,
              }}
              animate={{
                y: [0, -100, 0],
                opacity: [0, 0.8, 0],
                scale: [0.5, 1, 0.5],
              }}
              transition={{
                duration: 4 + Math.random() * 2,
                repeat: Infinity,
                delay: Math.random() * 4,
                ease: "easeInOut"
              }}
            />
          ))}
        </div>

        {/* 主内容 */}
        <motion.div
          initial={{ scale: 0.9, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ duration: 1, ease: [0.16, 1, 0.3, 1] }}
          className="flex flex-col items-center relative z-10"
        >
          {/* Logo区域 */}
          <motion.div 
            className="relative flex items-center justify-center w-36 h-36 mb-8"
            initial={{ scale: 0.8 }}
            animate={{ scale: [0.8, 1.05, 0.8] }}
            transition={{ 
              duration: 4, 
              repeat: Infinity, 
              ease: "easeInOut" 
            }}
          >
            {/* 外层光环 */}
            <motion.div
              className="absolute inset-0 rounded-full opacity-60"
              style={{
                background: 'conic-gradient(from 0deg, hsl(var(--primary)) 0%, hsl(var(--accent)) 40%, hsl(var(--secondary)) 80%, hsl(var(--primary)) 100%)'
              }}
              animate={{
                rotate: [0, 360],
                scale: [1, 1.1, 1],
              }}
              transition={{
                rotate: {
                  duration: 8,
                  repeat: Infinity,
                  ease: "linear"
                },
                scale: {
                  duration: 3,
                  repeat: Infinity,
                  ease: "easeInOut"
                }
              }}
            />
            
            {/* 中层背景 */}
            <motion.div
              className="absolute inset-3 rounded-full bg-gradient-to-br from-primary/30 via-accent/20 to-secondary/30 backdrop-blur-sm"
              animate={{
                scale: [1, 1.05, 1],
                opacity: [0.7, 0.4, 0.7]
              }}
              transition={{
                duration: 2.5,
                repeat: Infinity,
                ease: "easeInOut",
                delay: 0.5
              }}
            />
            
            {/* Logo核心 */}
            <div className="relative z-10 flex items-center justify-center w-24 h-24 rounded-full bg-background/95 backdrop-blur-md shadow-xl border-2 border-primary/20">
              <motion.div
                animate={{
                  rotate: [0, 360],
                  scale: [1, 1.1, 1]
                }}
                transition={{
                  rotate: {
                    duration: 10,
                    repeat: Infinity,
                    ease: "linear"
                  },
                  scale: {
                    duration: 3,
                    repeat: Infinity,
                    ease: "easeInOut"
                  }
                }}
              >
                <Code size={40} className="text-primary" />
              </motion.div>
            </div>

            {/* 装饰光点 */}
            <motion.div
              className="absolute -top-1 -right-1 w-4 h-4 bg-accent rounded-full opacity-80"
              animate={{
                scale: [1, 1.3, 1],
                opacity: [0.8, 0.4, 0.8]
              }}
              transition={{
                duration: 2,
                repeat: Infinity,
                ease: "easeInOut",
                delay: 0.3
              }}
            />
            <motion.div
              className="absolute -bottom-1 -left-1 w-3 h-3 bg-secondary rounded-full opacity-70"
              animate={{
                scale: [1, 1.2, 1],
                y: [0, -3, 0]
              }}
              transition={{
                duration: 1.8,
                repeat: Infinity,
                ease: "easeInOut",
                delay: 0.8
              }}
            />
          </motion.div>
          
          {/* 标题 */}
          <motion.h1 
            initial={{ y: 40, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.5, duration: 1, ease: "easeOut" }}
            className="text-5xl font-bold mb-3 bg-gradient-to-r from-primary via-accent to-secondary bg-clip-text text-transparent"
          >
            结绳社区
          </motion.h1>
          
          {/* 副标题 */}
          <motion.p
            initial={{ y: 30, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.7, duration: 1, ease: "easeOut" }}
            className="text-muted-foreground mb-12 text-center text-lg font-medium tracking-wide"
          >
            连接每一位开发者的技术社区
          </motion.p>

          {/* 特性标签 */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.9, duration: 0.8 }}
            className="flex flex-wrap justify-center gap-3 mb-12 max-w-md"
          >
            {[
              { text: '技术分享', color: 'primary' },
              { text: '代码交流', color: 'accent' },
              { text: '项目合作', color: 'secondary' }
            ].map((tag, index) => (
              <motion.span
                key={tag.text}
                initial={{ scale: 0, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                transition={{ 
                  delay: 1.1 + index * 0.1, 
                  duration: 0.6,
                  type: "spring",
                  stiffness: 100
                }}
                className={`px-4 py-2 text-sm rounded-full border backdrop-blur-sm
                  ${tag.color === 'primary' ? 'bg-primary/10 text-primary border-primary/30' :
                    tag.color === 'accent' ? 'bg-accent/10 text-accent border-accent/30' :
                    'bg-secondary/10 text-secondary border-secondary/30'
                  }`}
              >
                {tag.text}
              </motion.span>
            ))}
          </motion.div>
        </motion.div>

        {/* 进度条区域 */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 1.4, duration: 0.8 }}
          className="w-full max-w-sm relative z-10"
        >
          <div className="relative mb-4">
            <Progress 
              value={progress} 
              className="h-3 bg-muted/30 rounded-full overflow-hidden border border-border/50"
            />
            {/* 进度条光效 */}
            <motion.div
              className="absolute inset-0 h-3 rounded-full opacity-30"
              style={{
                background: `linear-gradient(90deg, 
                  hsl(var(--primary)) 0%, 
                  hsl(var(--accent)) 50%, 
                  hsl(var(--secondary)) 100%)`,
                width: `${progress}%`
              }}
              animate={{
                opacity: [0.3, 0.6, 0.3]
              }}
              transition={{
                duration: 2,
                repeat: Infinity,
                ease: "easeInOut"
              }}
            />
          </div>
          
          <div className="flex justify-between text-sm text-muted-foreground">
            <motion.span
              animate={{ 
                opacity: [0.7, 1, 0.7] 
              }}
              transition={{ 
                duration: 2, 
                repeat: Infinity,
                ease: "easeInOut"
              }}
            >
              正在启动应用
            </motion.span>
            <span className="font-mono tabular-nums">
              {Math.round(progress)}%
            </span>
          </div>
        </motion.div>

        {/* 底部装饰 */}
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 2, duration: 1 }}
          className="absolute bottom-16 left-1/2 transform -translate-x-1/2"
        >
          <div className="flex space-x-2">
            {[...Array(3)].map((_, i) => (
              <motion.div
                key={i}
                className="w-2 h-2 rounded-full bg-primary/40"
                animate={{
                  scale: [1, 1.5, 1],
                  opacity: [0.4, 1, 0.4]
                }}
                transition={{
                  duration: 1.5,
                  repeat: Infinity,
                  delay: i * 0.2,
                  ease: "easeInOut"
                }}
              />
            ))}
          </div>
        </motion.div>
      </div>
    </div>
  )
}

export default SplashScreen 