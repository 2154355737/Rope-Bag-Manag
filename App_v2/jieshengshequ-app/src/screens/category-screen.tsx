import React, { useState, useEffect } from 'react'
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
import TopNavigation from '@/components/ui/top-navigation'
import { useNavigation } from '@/contexts/NavigationContext'
import { getCategories } from '../api/categories'
import { getResources } from '../api/resources'

const CategoryScreen: React.FC = () => {
  const { getActiveTab, setActiveTab } = useNavigation()
  const [activeCategory, setActiveCategory] = useState<string|number>('all')
  const [categories, setCategories] = useState<{id:string|number; name:string}[]>([{ id: 'all', name: '全部' }])
  const [resources, setResources] = useState<any[]>([])
  
  // 获取当前活跃的显示模式
  const activeDisplayMode = getActiveTab('category', 'grid')
  
  useEffect(() => {
    // 加载分类
    getCategories().then((list) => setCategories([{ id: 'all', name: '全部' }, ...list])).catch(() => setCategories([{ id: 'all', name: '全部' }]))
    // 初始加载资源
    loadResources('all')
  }, [])

  const loadResources = async (cat: string|number) => {
    const params: any = { page: 1, pageSize: 20 }
    if (cat !== 'all') params.category_id = cat
    const data = await getResources(params)
    const list = (data.list || data.items || []).map((r: any) => ({
      id: r.id,
      title: r.name || r.title,
      difficulty: '',
      duration: '',
      tags: r.tags || [],
      image: (r.screenshots && r.screenshots[0]) || '',
      hot: r.download_count || r.like_count || 0,
    }))
    setResources(list)
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="分类"
        subtitle="发现更多精彩内容"
        showSearchButton
        rightAction={
          <Sheet>
            <SheetTrigger asChild>
              <Button variant="ghost" size="icon" className="h-9 w-9">
                <Filter size={20} />
              </Button>
            </SheetTrigger>
            <SheetContent side="right" className="w-80">
              <SheetHeader>
                <SheetTitle>筛选条件</SheetTitle>
              </SheetHeader>
              <div className="mt-6 space-y-6">
                <div>
                  <Label className="text-base font-medium">难度等级</Label>
                  <div className="mt-2 space-y-2">
                    <div className="flex items-center space-x-2">
                      <Switch id="beginner" />
                      <Label htmlFor="beginner">入门</Label>
                    </div>
                    <div className="flex items-center space-x-2">
                      <Switch id="intermediate" />
                      <Label htmlFor="intermediate">中级</Label>
                    </div>
                    <div className="flex items-center space-x-2">
                      <Switch id="advanced" />
                      <Label htmlFor="advanced">高级</Label>
                    </div>
                  </div>
                </div>
                
                <div>
                  <Label className="text-base font-medium">时长范围</Label>
                  <div className="mt-4">
                    <Slider
                      defaultValue={[0, 10]}
                      max={10}
                      step={0.5}
                      className="w-full"
                    />
                    <div className="flex justify-between text-sm text-muted-foreground mt-2">
                      <span>0小时</span>
                      <span>10小时+</span>
                    </div>
                  </div>
                </div>
                
                <div>
                  <Label className="text-base font-medium">热度筛选</Label>
                  <div className="mt-2 space-y-2">
                    <div className="flex items-center space-x-2">
                      <Switch id="hot" />
                      <Label htmlFor="hot">热门内容</Label>
                    </div>
                    <div className="flex items-center space-x-2">
                      <Switch id="new" />
                      <Label htmlFor="new">最新发布</Label>
                    </div>
                  </div>
                </div>
              </div>
            </SheetContent>
          </Sheet>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        {/* 一级分类 */}
        <div className="border-b overflow-x-auto">
        <div className="flex p-2 min-w-max">
          {categories.map((category) => (
            <Button
              key={category.id}
              variant={activeCategory === category.id ? "default" : "ghost"}
              className="rounded-full text-sm px-4"
              onClick={() => {
                setActiveCategory(category.id)
                loadResources(category.id)
              }}
            >
              {category.name}
            </Button>
          ))}
        </div>
      </div>



      {/* 内容标签页 */}
      <div className="p-4 flex-1">
        <Tabs value={activeDisplayMode} onValueChange={(value) => setActiveTab('category', value)} className="w-full">
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
                        className="absolute top-2 right-2 bg-background/50 backdrop-blur-sm rounded-full h-8 w-8 z-10"
                      >
                        <Star size={16} className="text-yellow-500" />
                      </Button>
                      <div className="absolute bottom-2 right-2 z-10">
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
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default CategoryScreen