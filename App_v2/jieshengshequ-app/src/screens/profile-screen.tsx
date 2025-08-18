import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Settings, Edit, LogOut, BookOpen, Heart, Bookmark, ChevronRight, Moon, Sun, Camera, Save, X, Share2, QrCode, Award, Copy, Download } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog'
import { Separator } from '@/components/ui/separator'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { toast } from '@/hooks/use-toast'
import { useTheme } from '@/components/theme-provider'

const ProfileScreen: React.FC = () => {
  const navigate = useNavigate()
  const { theme, setTheme } = useTheme()
  
  // 编辑状态管理
  const [isEditDialogOpen, setIsEditDialogOpen] = useState(false)
  const [isSaving, setIsSaving] = useState(false)
  
  // 用户资料状态
  const [userProfile, setUserProfile] = useState({
    name: '程序员小王',
    bio: '结绳语言爱好者，专注移动开发',
    avatar: 'https://i.pravatar.cc/150?img=5',
    level: 'Lv.3 进阶开发者',
    email: 'xiaowang@example.com',
    location: '北京市',
    website: 'https://github.com/xiaowang',
    skills: ['结绳语言', 'React', 'TypeScript', '移动开发', 'Tailwind CSS', 'Node.js', 'Python', 'UI设计']
  })
  
  // 编辑表单状态
  const [editForm, setEditForm] = useState({
    name: userProfile.name,
    bio: userProfile.bio,
    email: userProfile.email,
    location: userProfile.location,
    website: userProfile.website,
    skills: userProfile.skills.join(', ')
  })
  
  // 格式化数字显示
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 获取状态显示信息
  const getStatusInfo = (status: 'published' | 'pending' | 'rejected') => {
    switch (status) {
      case 'published':
        return {
          text: '已发布',
          variant: 'default' as const,
          className: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'
        }
      case 'pending':
        return {
          text: '待审核',
          variant: 'secondary' as const,
          className: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400'
        }
      case 'rejected':
        return {
          text: '已拒绝',
          variant: 'destructive' as const,
          className: 'bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400'
        }
      default:
        return {
          text: '未知',
          variant: 'outline' as const,
          className: ''
        }
    }
  }
  
  // 处理编辑对话框打开
  const handleEditClick = () => {
    setEditForm({
      name: userProfile.name,
      bio: userProfile.bio,
      email: userProfile.email,
      location: userProfile.location,
      website: userProfile.website,
      skills: userProfile.skills.join(', ')
    })
    setIsEditDialogOpen(true)
  }
  
  // 处理表单输入变化
  const handleFormChange = (field: string, value: string) => {
    setEditForm(prev => ({
      ...prev,
      [field]: value
    }))
  }
  
  // 处理头像上传
  const handleAvatarUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file) {
      // 这里应该上传到服务器，现在模拟
      const reader = new FileReader()
      reader.onload = (e) => {
        if (e.target?.result) {
          setUserProfile(prev => ({
            ...prev,
            avatar: e.target!.result as string
          }))
          toast({
            title: "头像上传成功",
            description: "您的头像已更新",
            duration: 3000,
          })
        }
      }
      reader.readAsDataURL(file)
    }
  }
  
  // 处理保存
  const handleSave = async () => {
    if (!editForm.name.trim()) {
      toast({
        title: "保存失败",
        description: "用户名不能为空",
        variant: "destructive",
        duration: 3000,
      })
      return
    }
    
    setIsSaving(true)
    
    try {
      // 模拟API调用
      await new Promise(resolve => setTimeout(resolve, 1500))
      
      // 更新用户资料
      setUserProfile(prev => ({
        ...prev,
        name: editForm.name,
        bio: editForm.bio,
        email: editForm.email,
        location: editForm.location,
        website: editForm.website,
        skills: editForm.skills.split(',').map(s => s.trim()).filter(s => s)
      }))
      
      setIsEditDialogOpen(false)
      toast({
        title: "保存成功",
        description: "您的个人资料已更新",
        duration: 3000,
      })
    } catch (error) {
      toast({
        title: "保存失败",
        description: "请稍后重试",
        variant: "destructive",
        duration: 3000,
      })
    } finally {
      setIsSaving(false)
    }
  }
  
  const userStats = {
    posts: 12,
    resources: 8,
    views: 2560,
    likes: 328,
  }
  
  const weeklyReportData = {
    totalPosts: 86,
    completedProjects: 7,
    currentStreak: 12,
    achievements: [
      { id: 1, name: '初学者', icon: '🌱', description: '完成第一个课程' },
      { id: 2, name: '勤奋学习', icon: '📚', description: '连续学习7天' },
      { id: 3, name: '代码大师', icon: '💻', description: '完成5个项目' },
    ],
    weeklyPosts: [2, 1, 3, 0, 2, 4, 1],
  }
  
  const userContent = {
    resources: [
      {
        id: 1,
        title: '结绳语言开发工具包',
        image: 'https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8Y29kaW5nfGVufDB8fDB8fHww',
        likes: 42,
        downloads: 128,
        status: 'published' as const,
      },
      {
        id: 2,
        title: '移动端UI组件库',
        image: 'https://images.unsplash.com/photo-1542831371-29b0f74f9713?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8Y29kaW5nfGVufDB8fDB8fHww',
        likes: 36,
        downloads: 89,
        status: 'pending' as const,
      },
      {
        id: 5,
        title: 'React Native组件集合',
        image: 'https://images.unsplash.com/photo-1517077304055-6e89abbf09b0?w=500&auto=format&fit=crop&q=60',
        likes: 0,
        downloads: 0,
        status: 'rejected' as const,
      },
    ],
    posts: [
      {
        id: 3,
        title: '结绳高级特性详解',
        image: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8M3x8Y29kaW5nfGVufDB8fDB8fHww',
        author: '程序员小王',
        likes: 156,
        comments: 23,
        status: 'published' as const,
      },
      {
        id: 4,
        title: '结绳性能优化指南',
        image: 'https://images.unsplash.com/photo-1551033406-611cf9a28f67?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTJ8fGNvZGluZ3xlbnwwfHwwfHx8MA%3D%3D',
        author: '程序员小王',
        likes: 89,
        comments: 12,
        status: 'pending' as const,
      },
      {
        id: 6,
        title: '结绳语言最佳实践分享',
        image: 'https://images.unsplash.com/photo-1516116216624-53e697fedbea?w=500&auto=format&fit=crop&q=60',
        author: '程序员小王',
        likes: 0,
        comments: 0,
        status: 'rejected' as const,
      },
    ],
    comments: [
      {
        id: 5,
        postTitle: '结绳语言新手入门指南',
        content: '这个教程写得很详细，对新手很友好！',
        author: '张三',
        likes: 15,
        time: '2小时前',
      },
      {
        id: 6,
        postTitle: 'Capacitor跨平台开发实践',
        content: '感谢分享，解决了我的问题',
        author: '李四',
        likes: 8,
        time: '5小时前',
      },
    ],
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-bold">个人中心</h1>
          
          <Button variant="ghost" size="icon" onClick={() => navigate('/settings')}>
            <Settings size={20} />
          </Button>
        </div>
      </header>

      {/* 用户信息 */}
      <div className="p-4 border-b">
        <div className="flex items-start gap-4">
          <div className="flex flex-col items-center">
            <div className="relative flex-shrink-0">
              <Avatar className="h-20 w-20">
                <AvatarImage src={userProfile.avatar} />
                <AvatarFallback>{userProfile.name[0]}</AvatarFallback>
              </Avatar>
              <label className="absolute bottom-0 right-0 bg-primary text-primary-foreground rounded-full p-2 cursor-pointer hover:bg-primary/90 transition-all duration-200 shadow-lg hover:shadow-xl hover:scale-105 border-2 border-background">
                <Camera size={14} />
                <input
                  type="file"
                  accept="image/*"
                  onChange={handleAvatarUpload}
                  className="hidden"
                />
              </label>
            </div>
            
            {/* 快捷操作按钮 */}
            <div className="flex flex-col gap-2 mt-3">
              <Button
                variant="outline"
                size="sm"
                className="flex items-center gap-1 text-xs px-3 py-1 h-7 w-16"
                onClick={() => {
                  toast({
                    title: "分享个人资料",
                    description: "个人资料链接已复制到剪贴板",
                    variant: "default"
                  })
                }}
              >
                <Share2 size={12} />
                分享
              </Button>
              <Button
                variant="outline"
                size="sm"
                className="flex items-center gap-1 text-xs px-3 py-1 h-7 w-16"
                onClick={() => {
                  toast({
                    title: "个人二维码",
                    description: "正在生成个人二维码",
                    variant: "default"
                  })
                }}
              >
                <QrCode size={12} />
                二维码
              </Button>
            </div>
            
            {/* 在线状态指示器 */}
            <div className="flex items-center gap-1 mt-2">
              <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
              <span className="text-xs text-muted-foreground">在线</span>
            </div>
          </div>
          
          <div className="flex-1 min-w-0">
            <div className="flex items-center justify-between">
              <h2 className="text-xl font-bold">{userProfile.name}</h2>
              <Dialog open={isEditDialogOpen} onOpenChange={setIsEditDialogOpen}>
                <DialogTrigger asChild>
                  <Button variant="outline" size="sm" className="flex items-center" onClick={handleEditClick}>
                    <Edit size={14} className="mr-1" /> 编辑
                  </Button>
                </DialogTrigger>
                <DialogContent className="max-w-sm w-[calc(100vw-3rem)] rounded-xl p-0 overflow-hidden shadow-xl left-[50%] top-[50%] translate-x-[-50%] translate-y-[-50%]">
                  <div className="p-6">
                  <DialogHeader className="pb-2">
                    <DialogTitle className="text-center text-lg">编辑个人资料</DialogTitle>
                  </DialogHeader>
                  <div className="space-y-5 py-2">
                    <div className="bg-muted/30 rounded-lg p-4 space-y-3">
                      <div className="space-y-2">
                        <Label htmlFor="name" className="text-sm font-medium text-foreground">用户名 <span className="text-red-500">*</span></Label>
                        <Input
                          id="name"
                          value={editForm.name}
                          onChange={(e) => handleFormChange('name', e.target.value)}
                          maxLength={20}
                          className="rounded-lg border-2 focus:border-primary focus:outline-none focus-visible:ring-0 focus-visible:ring-offset-0 transition-colors"
                          placeholder="请输入用户名"
                        />
                        <div className="text-xs text-muted-foreground text-right">
                          {editForm.name.length}/20
                        </div>
                      </div>
                      
                      <div className="space-y-2">
                        <Label htmlFor="bio" className="text-sm font-medium text-foreground">个人简介</Label>
                        <Textarea
                          id="bio"
                          value={editForm.bio}
                          onChange={(e) => handleFormChange('bio', e.target.value)}
                          maxLength={100}
                          rows={3}
                          className="rounded-lg border-2 focus:border-primary focus:outline-none focus-visible:ring-0 focus-visible:ring-offset-0 transition-colors resize-none"
                          placeholder="介绍一下你自己..."
                        />
                        <div className="text-xs text-muted-foreground text-right">
                          {editForm.bio.length}/100
                        </div>
                      </div>
                    </div>

                    <div className="bg-muted/30 rounded-lg p-4 space-y-3">
                      <div className="space-y-2">
                        <Label htmlFor="email" className="text-sm font-medium text-foreground">邮箱地址</Label>
                        <Input
                          id="email"
                          type="email"
                          value={editForm.email}
                          onChange={(e) => handleFormChange('email', e.target.value)}
                          className="rounded-lg border-2 focus:border-primary focus:outline-none focus-visible:ring-0 focus-visible:ring-offset-0 transition-colors"
                          placeholder="your@email.com"
                        />
                      </div>
                      
                      <div className="space-y-2">
                        <Label htmlFor="location" className="text-sm font-medium text-foreground">所在地</Label>
                        <Input
                          id="location"
                          value={editForm.location}
                          onChange={(e) => handleFormChange('location', e.target.value)}
                          className="rounded-lg border-2 focus:border-primary focus:outline-none focus-visible:ring-0 focus-visible:ring-offset-0 transition-colors"
                          placeholder="北京市"
                        />
                      </div>
                      
                      <div className="space-y-2">
                        <Label htmlFor="website" className="text-sm font-medium text-foreground">个人网站</Label>
                        <Input
                          id="website"
                          type="url"
                          value={editForm.website}
                          onChange={(e) => handleFormChange('website', e.target.value)}
                          className="rounded-lg border-2 focus:border-primary focus:outline-none focus-visible:ring-0 focus-visible:ring-offset-0 transition-colors"
                          placeholder="https://your-website.com"
                        />
                      </div>
                    </div>

                    <div className="bg-muted/30 rounded-lg p-4 space-y-3">
                      <div className="space-y-2">
                        <Label htmlFor="skills" className="text-sm font-medium text-foreground">技能标签</Label>
                        <Input
                          id="skills"
                          value={editForm.skills}
                          onChange={(e) => handleFormChange('skills', e.target.value)}
                          className="rounded-lg border-2 focus:border-primary focus:outline-none focus-visible:ring-0 focus-visible:ring-offset-0 transition-colors"
                          placeholder="React, TypeScript, 移动开发"
                        />
                        <div className="text-xs text-muted-foreground">
                          💡 用逗号分隔多个技能，将显示为标签
                        </div>
                      </div>
                    </div>
                  </div>
                  <div className="flex justify-end gap-3 pt-4 border-t border-border/50">
                    <Button
                      variant="outline"
                      onClick={() => setIsEditDialogOpen(false)}
                      disabled={isSaving}
                      className="rounded-lg px-6 hover:bg-muted/80 transition-colors"
                    >
                      取消
                    </Button>
                    <Button 
                      onClick={handleSave} 
                      disabled={isSaving}
                      className="rounded-lg px-6 bg-primary hover:bg-primary/90 transition-all duration-200 shadow-sm hover:shadow-md"
                    >
                      {isSaving ? (
                        <>
                          <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                          保存中...
                        </>
                      ) : (
                        <>
                          <Save size={14} className="mr-2" />
                          保存更改
                        </>
                      )}
                    </Button>
                  </div>
                  </div>
                </DialogContent>
              </Dialog>
            </div>
            <p className="text-muted-foreground text-sm mb-3">{userProfile.bio}</p>
            
            {/* 等级标签 */}
            <div className="flex items-center mb-3">
              <Badge variant="outline" className="bg-primary/10 text-primary border-primary/20 font-medium">
                {userProfile.level}
              </Badge>
            </div>
            
            {/* 技能标签 */}
            <div className="space-y-2">
              <div className="text-xs text-muted-foreground font-medium">技能专长</div>
              <div className="flex flex-wrap gap-2">
                {userProfile.skills.slice(0, 6).map((skill, index) => (
                  <Badge 
                    key={index} 
                    variant="secondary" 
                    className="text-xs bg-muted hover:bg-muted/80 transition-colors cursor-default px-3 py-1 rounded-full"
                  >
                    {skill}
                  </Badge>
                ))}
                {userProfile.skills.length > 6 && (
                  <Badge 
                    variant="outline" 
                    className="text-xs text-muted-foreground border-dashed cursor-default px-3 py-1 rounded-full"
                  >
                    +{userProfile.skills.length - 6}
                  </Badge>
                )}
              </div>
            </div>
          </div>
        </div>
        
        <div className="grid grid-cols-4 mt-4 text-center">
          <div>
            <div className="font-bold">{formatNumber(userStats.posts)}</div>
            <div className="text-xs text-muted-foreground">帖子</div>
          </div>
          <div>
            <div className="font-bold">{formatNumber(userStats.resources)}</div>
            <div className="text-xs text-muted-foreground">资源</div>
          </div>
          <div>
            <div className="font-bold">{formatNumber(userStats.views)}</div>
            <div className="text-xs text-muted-foreground">浏览</div>
          </div>
          <div>
            <div className="font-bold">{formatNumber(userStats.likes)}</div>
            <div className="text-xs text-muted-foreground">点赞</div>
          </div>
        </div>
      </div>

      {/* 我的周报 */}
      <div className="p-4 border-b">
        <h3 className="text-lg font-medium mb-3">我的周报</h3>
        
        {/* 今日活跃度 */}
        <div className="mb-4 p-3 bg-gradient-to-r from-primary/10 to-primary/5 rounded-lg border border-primary/20">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Award size={16} className="text-primary" />
              <span className="text-sm font-medium">今日活跃度</span>
            </div>
            <div className="flex items-center gap-1">
              <span className="text-sm font-bold text-primary">85%</span>
              <span className="text-xs text-muted-foreground">+12%</span>
            </div>
          </div>
          <div className="mt-2 bg-background/50 rounded-full h-2">
            <div className="bg-primary h-2 rounded-full transition-all duration-500" style={{ width: '85%' }}></div>
          </div>
          <div className="flex justify-between text-xs text-muted-foreground mt-1">
            <span>已发布 3 篇内容</span>
            <span>获得 12 个赞</span>
          </div>
        </div>
        
        <div className="grid grid-cols-3 gap-2 mb-4">
          <Card>
            <CardContent className="p-3 text-center">
              <div className="text-2xl font-bold text-primary">{weeklyReportData.totalPosts}</div>
              <div className="text-xs text-muted-foreground">总发布</div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-3 text-center">
              <div className="text-2xl font-bold text-primary">{weeklyReportData.completedProjects}</div>
              <div className="text-xs text-muted-foreground">完成项目</div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-3 text-center">
              <div className="text-2xl font-bold text-primary">{weeklyReportData.currentStreak}</div>
              <div className="text-xs text-muted-foreground">连续活跃</div>
            </CardContent>
          </Card>
        </div>
        
        <Card className="mb-4">
          <CardContent className="p-3">
            <h4 className="text-sm font-medium mb-2">本周发布</h4>
            <div className="flex items-end h-20 gap-1">
              {weeklyReportData.weeklyPosts.map((posts, index) => (
                <div 
                  key={index}
                  className="flex-1 bg-primary rounded-t"
                  style={{ 
                    height: `${(posts / 4) * 100}%`,
                    opacity: posts ? undefined : 0.3
                  }}
                />
              ))}
            </div>
            <div className="flex justify-between text-xs text-muted-foreground mt-1">
              <span>一</span>
              <span>二</span>
              <span>三</span>
              <span>四</span>
              <span>五</span>
              <span>六</span>
              <span>日</span>
            </div>
          </CardContent>
        </Card>
        
        <h4 className="text-sm font-medium mb-2">成就徽章</h4>
        <div className="flex gap-3 overflow-x-auto pb-2">
          {weeklyReportData.achievements.map((achievement) => (
            <div key={achievement.id} className="flex flex-col items-center min-w-[60px]">
              <div className="flex items-center justify-center w-12 h-12 rounded-full bg-primary/10 mb-1">
                <span className="text-2xl">{achievement.icon}</span>
              </div>
              <div className="text-xs text-center">{achievement.name}</div>
            </div>
          ))}
        </div>
      </div>

      {/* 内容管理 */}
      <div className="p-4 flex-1">
        <Tabs defaultValue="resources" className="w-full">
          <TabsList className="grid grid-cols-3 mb-4">
            <TabsTrigger value="resources">我的资源</TabsTrigger>
            <TabsTrigger value="posts">帖子</TabsTrigger>
            <TabsTrigger value="comments">评论</TabsTrigger>
          </TabsList>
          
          <TabsContent value="resources" className="mt-0">
            <div className="grid grid-cols-2 gap-3">
              {userContent.resources.map((resource) => (
                <motion.div
                  key={resource.id}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                >
                  <Card className="overflow-hidden relative">
                    <div className="absolute top-2 right-2 z-10">
                      <Badge className={`text-xs px-2 py-0.5 ${getStatusInfo(resource.status).className}`}>
                        {getStatusInfo(resource.status).text}
                      </Badge>
                    </div>
                    <img 
                      src={resource.image} 
                      alt={resource.title}
                      className="w-full h-24 object-cover"
                    />
                    <CardContent className="p-2">
                      <h4 className="text-sm font-medium line-clamp-1">{resource.title}</h4>
                      <div className="flex items-center justify-between text-xs text-muted-foreground mt-1">
                        <div className="flex items-center">
                          <Heart size={12} className="mr-1" />
                          <span>{resource.likes}</span>
                        </div>
                        <div className="flex items-center">
                          <BookOpen size={12} className="mr-1" />
                          <span>{resource.downloads}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
          </TabsContent>
          
          <TabsContent value="posts" className="mt-0">
            <div className="grid grid-cols-2 gap-3">
              {userContent.posts.map((post) => (
                <motion.div
                  key={post.id}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                >
                  <Card className="overflow-hidden relative">
                    <div className="absolute top-2 right-2 z-10">
                      <Badge className={`text-xs px-2 py-0.5 ${getStatusInfo(post.status).className}`}>
                        {getStatusInfo(post.status).text}
                      </Badge>
                    </div>
                    <img 
                      src={post.image} 
                      alt={post.title}
                      className="w-full h-24 object-cover"
                    />
                    <CardContent className="p-2">
                      <h4 className="text-sm font-medium line-clamp-1">{post.title}</h4>
                      <div className="flex items-center justify-between text-xs text-muted-foreground mt-1">
                        <span>{post.author}</span>
                        <div className="flex items-center">
                          <Heart size={12} className="mr-1" />
                          <span>{post.likes}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
          </TabsContent>
          
          <TabsContent value="comments" className="mt-0">
            <div className="space-y-3">
              {userContent.comments.map((comment) => (
                <motion.div
                  key={comment.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.3 }}
                >
                  <Card>
                    <CardContent className="p-3">
                      <div className="flex items-start justify-between mb-2">
                        <h4 className="text-sm font-medium text-primary line-clamp-1">{comment.postTitle}</h4>
                        <span className="text-xs text-muted-foreground whitespace-nowrap ml-2">{comment.time}</span>
                      </div>
                      <p className="text-sm text-muted-foreground mb-2 line-clamp-2">{comment.content}</p>
                      <div className="flex items-center justify-between">
                        <span className="text-xs text-muted-foreground">回复给 {comment.author}</span>
                        <div className="flex items-center">
                          <Heart size={12} className="mr-1" />
                          <span className="text-xs">{comment.likes}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
          </TabsContent>
        </Tabs>
      </div>

      {/* 设置选项 */}
      <div className="p-4 border-t">
        <div className="flex items-center justify-between py-2">
          <div className="flex items-center">
            {theme === 'dark' ? <Moon size={18} className="mr-2" /> : <Sun size={18} className="mr-2" />}
            <span>深色模式</span>
          </div>
          <Switch 
            checked={theme === 'dark'}
            onCheckedChange={(checked) => setTheme(checked ? 'dark' : 'light')}
          />
        </div>
        
        <Button variant="outline" className="w-full mt-4 text-destructive">
          <LogOut size={16} className="mr-2" /> 退出登录
        </Button>
      </div>
    </div>
  )
}

export default ProfileScreen