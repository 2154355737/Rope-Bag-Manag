import React, { useState, useRef } from 'react'
import { motion } from 'framer-motion'
import { Button } from '@/components/ui/button'
import { ChevronRight, Code, Users, BarChart } from 'lucide-react'

interface OnboardingScreenProps {
  onComplete: () => void
}

const OnboardingScreen: React.FC<OnboardingScreenProps> = ({ onComplete }) => {
  const [currentPage, setCurrentPage] = useState(0)
  const touchStartX = useRef(0)
  const touchEndX = useRef(0)

  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX
  }

  const handleTouchMove = (e: React.TouchEvent) => {
    touchEndX.current = e.touches[0].clientX
  }

  const handleTouchEnd = () => {
    if (touchStartX.current - touchEndX.current > 50) {
      // 向左滑动
      if (currentPage < 2) setCurrentPage(currentPage + 1)
    } else if (touchEndX.current - touchStartX.current > 50) {
      // 向右滑动
      if (currentPage > 0) setCurrentPage(currentPage - 1)
    }
  }

  const onboardingPages = [
    {
      icon: Code,
      title: '探索结绳资源',
      description: '浏览丰富的结绳编程资源，从入门到精通，一站式学习平台',
    },
    {
      icon: Users,
      title: '社区互动交流',
      description: '分享你的作品，与其他开发者交流，获取反馈和灵感',
    },
    {
      icon: BarChart,
      title: '追踪学习进度',
      description: '记录你的学习历程，可视化展示学习数据，持续进步',
    },
  ]

  const handleNext = () => {
    if (currentPage < 2) {
      setCurrentPage(currentPage + 1)
    } else {
      onComplete()
    }
  }

  return (
    <div 
      className="flex flex-col min-h-screen bg-background"
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
    >
      <div className="absolute top-4 right-4">
        <Button variant="ghost" size="sm" onClick={onComplete}>
          跳过
        </Button>
      </div>

      <div className="flex-1 flex flex-col items-center justify-center p-6">
        <motion.div
          key={currentPage}
          initial={{ x: 100, opacity: 0 }}
          animate={{ x: 0, opacity: 1 }}
          exit={{ x: -100, opacity: 0 }}
          transition={{ duration: 0.3 }}
          className="flex flex-col items-center"
        >
          <div className="flex items-center justify-center w-32 h-32 rounded-full bg-primary/10 mb-8">
            {React.createElement(onboardingPages[currentPage].icon, { size: 64, className: "text-primary" })}
          </div>
          
          <h2 className="text-2xl font-bold mb-4 text-center">
            {onboardingPages[currentPage].title}
          </h2>
          
          <p className="text-muted-foreground text-center max-w-xs">
            {onboardingPages[currentPage].description}
          </p>
        </motion.div>
      </div>

      <div className="p-6">
        <div className="flex justify-center mb-8">
          {[0, 1, 2].map((page) => (
            <div
              key={page}
              className={`w-2 h-2 rounded-full mx-1 ${
                page === currentPage ? 'bg-primary' : 'bg-muted'
              }`}
            />
          ))}
        </div>

        <Button onClick={handleNext} className="w-full">
          {currentPage === 2 ? '开始使用' : '下一步'} 
          <ChevronRight size={16} className="ml-1" />
        </Button>
      </div>
    </div>
  )
}

export default OnboardingScreen