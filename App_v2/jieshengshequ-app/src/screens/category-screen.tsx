import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { Filter, ChevronDown, Star, Clock, Zap, BookOpen } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from '@/components/ui/sheet'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Slider } from '@/components/ui/slider'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'

const CategoryScreen: React.FC = () => {
  const [activeCategory, setActiveCategory] = useState('all')
  
  const categories = [
    { id: 'all', name: '全部' },
    { id: 'basic', name: '基础语法' },
    { id: 'advanced', name: '高级特性' },
    { id: 'project', name: '实战项目' },
    { id: 'algorithm', name: '算法' },
    { id: 'interview', name: '面试题' },
  ]
  

  
  const resources = [
    {
      id: 1,
      title: '结绳语言基础语法详解',
      difficulty: '入门',
      duration: '2小时',
      tags: ['语法', '入门'],
      image: 'https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8Y29kaW5nfGVufDB8fDB8fHww',
      hot: 98,
    },
    {
      id: 2,
      title: '结绳异步编程实战',
      difficulty: '中级',
      duration: '3.5小时',
      tags: ['异步', '中级'],
      image: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8M3x8Y29kaW5nfGVufDB8fDB8fHww',
      hot: 76,
    },
    {
      id: 3,
      title: '结绳高性能应用开发',
      difficulty: '高级',
      duration: '5小时',
      tags: ['性能', '高级'],
      image: 'https://images.unsplash.com/photo-1542831371-29b0f74f9713?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8Y29kaW5nfGVufDB8fDB8fHww',
      hot: 120,
    },
    {
      id: 4,
      title: '结绳移动应用开发教程',
      difficulty: '中级',
      duration: '4小时',
      tags: ['移动', '实战'],
      image: 'https://images.unsplash.com/photo-1551033406-611cf9a28f67?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTJ8fGNvZGluZ3xlbnwwfHwwfHx8MA%3D%3D',
      hot: 85,
    },
  ]

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-bold">分类浏览</h1>
          
          <Sheet>
            <SheetTrigger asChild>
              <Button variant="outline" size="icon">
                <Filter size={18} />
              </Button>
            </SheetTrigger>
            <SheetContent>
              <SheetHeader>
                <SheetTitle>筛选条件</SheetTitle>
              </SheetHeader>
              <div className="py-6 space-y-6">
                <div className="space-y-2">
                  <h3 className="text-sm font-medium">难度</h3>
                  <div className="flex flex-wrap gap-2">
                    <Badge variant="outline" className="cursor-pointer hover:bg-primary hover:text-primary-foreground">入门</Badge>
                    <Badge variant="outline" className="cursor-pointer hover:bg-primary hover:text-primary-foreground">中级</Badge>
                    <Badge variant="outline" className="cursor-pointer hover:bg-primary hover:text-primary-foreground">高级</Badge>
                  </div>
                </div>
                
                <div className="space-y-2">
                  <h3 className="text-sm font-medium">热度</h3>
                  <Slider defaultValue={[50]} max={100} step={1} />
                </div>
                
                <div className="space-y-2">
                  <h3 className="text-sm font-medium">时长</h3>
                  <div className="flex items-center justify-between">
                    <span className="text-sm">1小时以内</span>
                    <span className="text-sm">5小时以上</span>
                  </div>
                  <Slider defaultValue={[0, 100]} max={100} step={1} />
                </div>
                
                <div className="flex items-center space-x-2">
                  <Switch id="only-favorite" />
                  <Label htmlFor="only-favorite">只看收藏</Label>
                </div>
                
                <div className="flex justify-end space-x-2 pt-4">
                  <Button variant="outline">重置</Button>
                  <Button>应用筛选</Button>
                </div>
              </div>
            </SheetContent>
          </Sheet>
        </div>
      </header>

      {/* 一级分类 */}
      <div className="border-b overflow-x-auto">
        <div className="flex p-2 min-w-max">
          {categories.map((category) => (
            <Button
              key={category.id}
              variant={activeCategory === category.id ? "default" : "ghost"}
              className="rounded-full text-sm px-4"
              onClick={() => setActiveCategory(category.id)}
            >
              {category.name}
            </Button>
          ))}
        </div>
      </div>



      {/* 内容标签页 */}
      <div className="p-4 flex-1">
        <Tabs defaultValue="grid" className="w-full">
          <div className="flex justify-between items-center mb-4">
            <span className="text-sm text-muted-foreground">共 {resources.length} 个资源</span>
            <TabsList>
              <TabsTrigger value="grid" className="text-xs px-2 py-1">
                网格
              </TabsTrigger>
              <TabsTrigger value="list" className="text-xs px-2 py-1">
                列表
              </TabsTrigger>
            </TabsList>
          </div>
          
          <TabsContent value="grid" className="mt-0">
            <div className="grid grid-cols-2 gap-4">
              {resources.map((resource) => (
                <motion.div
                  key={resource.id}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                >
                  <Card className="overflow-hidden h-full">
                    <div className="relative">
                      <img 
                        src={resource.image} 
                        alt={resource.title}
                        className="w-full h-32 object-cover"
                      />
                      <Button 
                        variant="ghost" 
                        size="icon" 
                        className="absolute top-2 right-2 bg-background/50 backdrop-blur-sm rounded-full h-8 w-8"
                      >
                        <Star size={16} className="text-yellow-500" />
                      </Button>
                      <div className="absolute bottom-2 right-2">
                        <Badge className="bg-background/50 backdrop-blur-sm text-foreground text-xs">
                          {resource.difficulty}
                        </Badge>
                      </div>
                    </div>
                    <CardContent className="p-3">
                      <h3 className="font-medium text-sm line-clamp-2 mb-2">{resource.title}</h3>
                      <div className="flex items-center justify-between text-xs text-muted-foreground">
                        <div className="flex items-center">
                          <Clock size={12} className="mr-1" />
                          <span>{resource.duration}</span>
                        </div>
                        <div className="flex items-center">
                          <Zap size={12} className="mr-1" />
                          <span>{resource.hot}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
          </TabsContent>
          
          <TabsContent value="list" className="mt-0 space-y-4">
            {resources.map((resource) => (
              <motion.div
                key={resource.id}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3 }}
              >
                <Card>
                  <div className="flex p-3">
                    <img 
                      src={resource.image} 
                      alt={resource.title}
                      className="w-20 h-20 object-cover rounded-md mr-3"
                    />
                    <div className="flex-1">
                      <h3 className="font-medium text-sm mb-1">{resource.title}</h3>
                      <div className="flex flex-wrap gap-1 mb-2">
                        {resource.tags.map((tag, idx) => (
                          <Badge key={idx} variant="outline" className="text-xs px-1">
                            {tag}
                          </Badge>
                        ))}
                      </div>
                      <div className="flex items-center justify-between text-xs text-muted-foreground">
                        <div className="flex items-center">
                          <BookOpen size={12} className="mr-1" />
                          <span>{resource.difficulty}</span>
                        </div>
                        <div className="flex items-center">
                          <Clock size={12} className="mr-1" />
                          <span>{resource.duration}</span>
                        </div>
                        <div className="flex items-center">
                          <Zap size={12} className="mr-1" />
                          <span>{resource.hot}</span>
                        </div>
                      </div>
                    </div>
                    <Button variant="ghost" size="icon" className="self-start ml-2">
                      <Star size={16} />
                    </Button>
                  </div>
                </Card>
              </motion.div>
            ))}
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}

export default CategoryScreen