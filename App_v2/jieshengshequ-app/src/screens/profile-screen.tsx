import React, { useState, useRef, useEffect } from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Settings, Edit, LogOut, BookOpen, Heart, Bookmark, ChevronRight, Moon, Sun, Camera, Save, X, Share2, QrCode, Award, Copy, Download, FileText, MessageSquare, ChevronDown } from 'lucide-react'
import QRCodeLib from 'qrcode'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/dialog'
import { Separator } from '@/components/ui/separator'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { toast } from '@/hooks/use-toast'
import { useTheme } from '@/components/theme-provider'
import TopNavigation from '@/components/ui/top-navigation'

const ProfileScreen: React.FC = () => {
  const navigate = useNavigate()
  const { theme, setTheme } = useTheme()
  

  
  // 分享和二维码状态管理
  const [isShareDialogOpen, setIsShareDialogOpen] = useState(false)
  const [isQRDialogOpen, setIsQRDialogOpen] = useState(false)
  const [qrCodeDataUrl, setQrCodeDataUrl] = useState('')
  const [isGeneratingQR, setIsGeneratingQR] = useState(false)
  const [qrContentType, setQrContentType] = useState<'url' | 'vcard'>('url')
  const qrCanvasRef = useRef<HTMLCanvasElement>(null)
  
  // 周报折叠状态
  const [isWeeklyReportExpanded, setIsWeeklyReportExpanded] = useState(false)
  
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
  


  // 生成个人资料链接
  const generateProfileLink = () => {
    const baseUrl = window.location.origin
    const profileId = userProfile.name.toLowerCase().replace(/\s+/g, '-')
    return `${baseUrl}/profile/${profileId}`
  }

  // 处理分享功能
  const handleShare = () => {
    setIsShareDialogOpen(true)
  }

  // 复制链接到剪贴板
  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text)
      toast({
        title: "复制成功",
        description: "链接已复制到剪贴板",
        variant: "default"
      })
    } catch (err) {
      // 降级方案
      const textArea = document.createElement('textarea')
      textArea.value = text
      document.body.appendChild(textArea)
      textArea.select()
      document.execCommand('copy')
      document.body.removeChild(textArea)
      toast({
        title: "复制成功",
        description: "链接已复制到剪贴板",
        variant: "default"
      })
    }
  }

  // 打开二维码对话框
  const openQRDialog = () => {
    setIsQRDialogOpen(true)
    if (!qrCodeDataUrl) {
      generateQRCode()
    }
  }

  // 生成二维码
  const generateQRCode = async () => {
    setIsGeneratingQR(true)
    
    try {
      const profileLink = generateProfileLink()
      
      // 根据类型生成不同的二维码内容
      let qrContent = profileLink
      
      if (qrContentType === 'vcard') {
        qrContent = `BEGIN:VCARD
VERSION:3.0
FN:${userProfile.name}
ORG:结绳社区
TITLE:${userProfile.level}
EMAIL:${userProfile.email}
URL:${profileLink}
NOTE:${userProfile.bio}
END:VCARD`
      }

      // 生成二维码图片
      const qrOptions = {
        errorCorrectionLevel: 'M' as const,
        type: 'image/png' as const,
        quality: 0.95,
        margin: 1,
        color: {
          dark: '#1f2937',  // 深灰色，更现代
          light: '#ffffff',
        },
        width: 240,
        scale: 8,  // 高清晰度
      }

      const dataUrl = await QRCodeLib.toDataURL(qrContent, qrOptions)
      setQrCodeDataUrl(dataUrl)
      
      // 如果有canvas引用，也在canvas上绘制
      if (qrCanvasRef.current) {
        await QRCodeLib.toCanvas(qrCanvasRef.current, qrContent, qrOptions)
      }
      
    } catch (error) {
      console.error('生成二维码失败:', error)
      toast({
        title: "生成失败",
        description: "二维码生成失败，请重试",
        variant: "destructive"
      })
    } finally {
      setIsGeneratingQR(false)
    }
  }

  // 下载二维码
  const downloadQRCode = () => {
    if (!qrCodeDataUrl) {
      toast({
        title: "下载失败",
        description: "请先生成二维码",
        variant: "destructive"
      })
      return
    }

    try {
      // 创建下载链接
      const link = document.createElement('a')
      link.href = qrCodeDataUrl
      link.download = `${userProfile.name}-个人二维码.png`
      
      // 触发下载
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      
      toast({
        title: "下载成功",
        description: "二维码图片已保存",
        variant: "default"
      })
    } catch (error) {
      console.error('下载二维码失败:', error)
      toast({
        title: "下载失败",
        description: "保存图片时出现错误",
        variant: "destructive"
      })
    }
  }

  // 重新生成二维码
  const regenerateQRCode = () => {
    setQrCodeDataUrl('')
    generateQRCode()
  }

  // 关闭二维码对话框时清理状态
  const handleQRDialogClose = (open: boolean) => {
    setIsQRDialogOpen(open)
    if (!open) {
      // 可选：保留二维码数据以便下次快速显示
      // setQrCodeDataUrl('')
      setIsGeneratingQR(false)
    }
  }

  // 原生分享API
  const nativeShare = async () => {
    const profileLink = generateProfileLink()
    const shareData = {
      title: `${userProfile.name} - 结绳社区`,
      text: `来看看 ${userProfile.name} 在结绳社区的个人资料`,
      url: profileLink
    }

    try {
      if (navigator.share) {
        await navigator.share(shareData)
      } else {
        // 降级到复制链接
        await copyToClipboard(profileLink)
      }
    } catch (err) {
      console.log('分享取消或失败')
    }
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
      {
        id: 5,
        title: 'React Native组件集合',
        image: 'https://images.unsplash.com/photo-1517077304055-6e89abbf09b0?w=500&auto=format&fit=crop&q=60',
        likes: 0,
        downloads: 0,
        status: 'rejected' as const,
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
      <TopNavigation
        title="个人中心"
        subtitle={userProfile.level}
        showSettingsButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="icon"
              className="h-9 w-9"
              onClick={handleShare}
            >
              <Share2 size={20} />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-9 w-9"
              onClick={() => navigate('/settings')}
            >
              <Settings size={20} />
            </Button>
          </div>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
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
                onClick={handleShare}
              >
                <Share2 size={12} />
                分享
              </Button>
              <Button
                variant="outline"
                size="sm"
                className="flex items-center gap-1 text-xs px-3 py-1 h-7 w-16"
                onClick={openQRDialog}
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
              <Button 
                variant="outline" 
                size="sm" 
                className="flex items-center" 
                onClick={() => navigate('/edit-profile')}
              >
                <Edit size={14} className="mr-1" /> 编辑
              </Button>

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
        

      </div>

      {/* 我的周报 */}
      <div className="p-4 border-b">
        <div 
          className="flex items-center justify-between cursor-pointer mb-3"
          onClick={() => setIsWeeklyReportExpanded(!isWeeklyReportExpanded)}
        >
          <h3 className="text-lg font-medium">我的周报</h3>
          <motion.div
            animate={{ rotate: isWeeklyReportExpanded ? 180 : 0 }}
            transition={{ duration: 0.2 }}
          >
            <ChevronDown size={20} className="text-muted-foreground" />
          </motion.div>
        </div>
        
        {/* 今日活跃度 - 始终显示 */}
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
        
        {/* 可折叠的详细内容 */}
        <motion.div
          initial={false}
          animate={{ 
            height: isWeeklyReportExpanded ? 'auto' : 0,
            opacity: isWeeklyReportExpanded ? 1 : 0
          }}
          transition={{ duration: 0.3 }}
          className="overflow-hidden"
        >
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
                <div className="text-xs text-muted-foreground">内容浏览量</div>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="p-3 text-center">
                <div className="text-2xl font-bold text-primary">{weeklyReportData.currentStreak}</div>
                <div className="text-xs text-muted-foreground">连续签到</div>
              </CardContent>
            </Card>
          </div>
          
          <Card className="mb-4">
            <CardContent className="p-3">
              <div className="flex items-center justify-between mb-3">
                <h4 className="text-sm font-medium">本周发布</h4>
                <div className="text-xs text-muted-foreground">
                  总计: {weeklyReportData.weeklyPosts.reduce((sum, posts) => sum + posts, 0)} 篇
                </div>
              </div>
              
                          {/* 图表容器 - 独立区域，不会影响上方标题 */}
              <div className="relative">
                {/* 纯柱状图区域 */}
                <div className="flex items-end h-16 gap-1 mb-2">
                  {weeklyReportData.weeklyPosts.map((posts, index) => {
                    // 限制最大高度，确保不会超出容器
                    const maxDisplayValue = Math.max(...weeklyReportData.weeklyPosts)
                    const height = maxDisplayValue > 0 ? (posts / maxDisplayValue) * 90 : 0 // 恢复到90%
                    
                    return (
                      <motion.div
                        key={`bar-${index}`}
                        className="flex-1 relative"
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ 
                          delay: index * 0.1, 
                          duration: 0.6,
                          ease: "easeOut"
                        }}
                      >
                        {/* 柱子容器 */}
                        <div className="relative w-full h-16 flex items-end justify-center">
                          <div className="w-8 h-full relative flex items-end">
                          {/* 背景柱子 */}
                          <div className="absolute bottom-0 w-full h-full bg-muted/20 rounded-t" />
                          
                          {/* 数据柱子 */}
                          <motion.div
                            className="relative w-full rounded-t overflow-hidden"
                            style={{ 
                              backgroundColor: posts > 0 ? 'hsl(var(--primary))' : 'transparent'
                            }}
                            initial={{ height: 0 }}
                            animate={{ height: `${height}%` }}
                            transition={{ 
                              delay: 0.3 + index * 0.1, 
                              duration: 0.8,
                              ease: "easeOut"
                            }}
                          >
                            {/* 轻微的光泽效果 */}
                            {posts > 0 && (
                              <div className="absolute inset-0 bg-gradient-to-t from-transparent to-white/15" />
                            )}
                            
                            {/* 顶部高光 */}
                            {posts > 0 && (
                              <div className="absolute top-0 left-0 right-0 h-0.5 bg-white/40 rounded-t" />
                            )}
                          </motion.div>
                          </div>
                        </div>
                      </motion.div>
                    )
                  })}
                </div>
                
                {/* 日期和数值标签区域 - 独立在柱状图下方 */}
                <div className="flex gap-1 mb-3">
                  {weeklyReportData.weeklyPosts.map((posts, index) => {
                    const dayNames = ['一', '二', '三', '四', '五', '六', '日']
                    
                    return (
                      <motion.div
                        key={`label-${index}`}
                        className="flex-1 text-xs text-center"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        transition={{
                          delay: 0.8 + index * 0.05,
                          duration: 0.3
                        }}
                      >
                        <div className="text-muted-foreground font-medium mb-1">
                          {dayNames[index]}
                        </div>
                        <motion.div
                          className={posts > 0 ? "text-primary font-bold" : "text-muted-foreground/50"}
                          initial={{ opacity: 0, scale: 0.8 }}
                          animate={{ opacity: 1, scale: 1 }}
                          transition={{
                            delay: 1.0 + index * 0.1,
                            duration: 0.4
                          }}
                        >
                          ({posts})
                        </motion.div>
                      </motion.div>
                    )
                  })}
                </div>
              
                {/* 图例和统计信息 */}
                <div className="flex items-center justify-between pt-3 border-t border-border/50">
                  <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <div className="flex items-center gap-1">
                      <div className="w-2 h-2 rounded-full bg-primary"></div>
                      <span>发布量</span>
                    </div>
                  </div>
                  <motion.div
                    className="text-xs text-muted-foreground"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ delay: 1.5, duration: 0.5 }}
                  >
                    平均: {(weeklyReportData.weeklyPosts.reduce((sum, posts) => sum + posts, 0) / 7).toFixed(1)} 篇/天
                  </motion.div>
                </div>
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
        </motion.div>
      </div>

      {/* 内容管理 */}
      <div className="p-4 flex-1">
        {/* 内容管理快捷入口 */}
        <div className="grid grid-cols-3 gap-3 mb-6">
          <motion.div
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            <Card 
              className="cursor-pointer hover:shadow-lg transition-shadow"
              onClick={() => navigate('/my-content?tab=resources')}
            >
              <CardContent className="p-4 text-center">
                <BookOpen size={24} className="mx-auto mb-2 text-primary" />
                <div className="text-lg font-bold">{userContent.resources.length}</div>
                <div className="text-xs text-muted-foreground">我的资源</div>
              </CardContent>
            </Card>
          </motion.div>

          <motion.div
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            <Card 
              className="cursor-pointer hover:shadow-lg transition-shadow"
              onClick={() => navigate('/my-content?tab=posts')}
            >
              <CardContent className="p-4 text-center">
                <FileText size={24} className="mx-auto mb-2 text-primary" />
                <div className="text-lg font-bold">{userContent.posts.length}</div>
                <div className="text-xs text-muted-foreground">我的帖子</div>
              </CardContent>
            </Card>
          </motion.div>

          <motion.div
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            <Card 
              className="cursor-pointer hover:shadow-lg transition-shadow"
              onClick={() => navigate('/my-content?tab=comments')}
            >
              <CardContent className="p-4 text-center">
                <MessageSquare size={24} className="mx-auto mb-2 text-primary" />
                <div className="text-lg font-bold">{userContent.comments.length}</div>
                <div className="text-xs text-muted-foreground">我的评论</div>
              </CardContent>
            </Card>
          </motion.div>
        </div>

        <Tabs defaultValue="resources" className="w-full">
          <TabsList className="grid grid-cols-3 mb-4">
            <TabsTrigger value="resources">资源预览</TabsTrigger>
            <TabsTrigger value="posts">帖子预览</TabsTrigger>
            <TabsTrigger value="comments">评论预览</TabsTrigger>
          </TabsList>
          
          <TabsContent value="resources" className="mt-0">
            <div className="grid grid-cols-2 gap-3">
              {userContent.resources.slice(0, 4).map((resource) => (
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
            
            {userContent.resources.length > 4 && (
              <div className="text-center mt-4">
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={() => navigate('/my-content?tab=resources')}
                  className="w-full"
                >
                  查看全部 {userContent.resources.length} 个资源
                  <ChevronRight size={14} className="ml-1" />
                </Button>
              </div>
            )}
          </TabsContent>
          
          <TabsContent value="posts" className="mt-0">
            <div className="grid grid-cols-2 gap-3">
              {userContent.posts.slice(0, 4).map((post) => (
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
            
            {userContent.posts.length > 4 && (
              <div className="text-center mt-4">
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={() => navigate('/my-content?tab=posts')}
                  className="w-full"
                >
                  查看全部 {userContent.posts.length} 个帖子
                  <ChevronRight size={14} className="ml-1" />
                </Button>
              </div>
            )}
          </TabsContent>
          
          <TabsContent value="comments" className="mt-0">
            <div className="space-y-3">
              {userContent.comments.slice(0, 4).map((comment) => (
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
            
            {userContent.comments.length > 4 && (
              <div className="text-center mt-4">
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={() => navigate('/my-content?tab=comments')}
                  className="w-full"
                >
                  查看全部 {userContent.comments.length} 条评论
                  <ChevronRight size={14} className="ml-1" />
                </Button>
              </div>
            )}
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

      {/* 分享对话框 */}
      <Dialog open={isShareDialogOpen} onOpenChange={setIsShareDialogOpen}>
        <DialogContent className="max-w-sm w-[calc(100vw-3rem)] rounded-xl">
          <DialogHeader>
            <DialogTitle className="text-center">分享个人资料</DialogTitle>
            <DialogDescription className="text-center">
              通过链接或原生分享功能分享您的个人资料
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div className="text-center">
              <div className="w-16 h-16 mx-auto mb-3 bg-primary/10 rounded-full flex items-center justify-center">
                <Share2 size={24} className="text-primary" />
              </div>
              <h3 className="font-medium mb-2">{userProfile.name}</h3>
              <p className="text-sm text-muted-foreground mb-4">{userProfile.bio}</p>
            </div>
            
            <div className="space-y-3">
              <div className="p-3 bg-muted/30 rounded-lg">
                <div className="text-xs text-muted-foreground mb-1">个人资料链接</div>
                <div className="text-sm break-all">{generateProfileLink()}</div>
              </div>
              
              <div className="grid grid-cols-2 gap-3">
                <Button
                  variant="outline"
                  onClick={() => copyToClipboard(generateProfileLink())}
                  className="flex items-center gap-2"
                >
                  <Copy size={16} />
                  复制链接
                </Button>
                <Button
                  onClick={nativeShare}
                  className="flex items-center gap-2"
                >
                  <Share2 size={16} />
                  分享
                </Button>
              </div>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* 二维码对话框 */}
      <Dialog open={isQRDialogOpen} onOpenChange={handleQRDialogClose}>
        <DialogContent className="max-w-sm w-[calc(100vw-3rem)] rounded-xl">
          <DialogHeader>
            <DialogTitle className="text-center">个人二维码</DialogTitle>
            <DialogDescription className="text-center">
              生成包含个人信息的二维码，支持链接和名片两种格式
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div className="text-center">
              <div className="w-64 h-64 mx-auto mb-4 bg-white rounded-lg border-2 border-gray-200 flex items-center justify-center overflow-hidden">
                {isGeneratingQR ? (
                  <div className="text-center">
                    <div className="w-8 h-8 mx-auto mb-2 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
                    <div className="text-xs text-gray-500">生成中...</div>
                  </div>
                ) : qrCodeDataUrl ? (
                  <img 
                    src={qrCodeDataUrl} 
                    alt="个人二维码" 
                    className="w-full h-full object-contain p-4"
                  />
                ) : (
                  <div className="text-center">
                    <QrCode size={48} className="mx-auto mb-2 text-gray-400" />
                    <div className="text-xs text-gray-500">二维码</div>
                    <div className="text-xs text-gray-400 mt-1">扫描查看个人资料</div>
                  </div>
                )}
              </div>
              <p className="text-sm text-muted-foreground">
                扫描二维码查看 {userProfile.name} 的个人资料
              </p>
            </div>
            
            <Separator />
            
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <div className="text-xs text-muted-foreground">二维码类型：</div>
                <div className="flex gap-1">
                  <Button
                    variant={qrContentType === 'url' ? 'default' : 'outline'}
                    size="sm"
                    className="text-xs h-6 px-2"
                    onClick={() => {
                      setQrContentType('url')
                      setQrCodeDataUrl('')
                      generateQRCode()
                    }}
                  >
                    链接
                  </Button>
                  <Button
                    variant={qrContentType === 'vcard' ? 'default' : 'outline'}
                    size="sm"
                    className="text-xs h-6 px-2"
                    onClick={() => {
                      setQrContentType('vcard')
                      setQrCodeDataUrl('')
                      generateQRCode()
                    }}
                  >
                    名片
                  </Button>
                </div>
              </div>
              
              <div className="text-xs text-muted-foreground">
                {qrContentType === 'url' ? '链接二维码：' : 'vCard名片：'}
              </div>
              <div className="p-3 bg-muted/30 rounded-lg text-xs space-y-1">
                {qrContentType === 'url' ? (
                  <div>
                    <div className="font-medium mb-1">个人资料链接：</div>
                    <div className="break-all text-blue-600">{generateProfileLink()}</div>
                  </div>
                ) : (
                  <>
                    <div><span className="font-medium">姓名：</span>{userProfile.name}</div>
                    <div><span className="font-medium">等级：</span>{userProfile.level}</div>
                    <div><span className="font-medium">邮箱：</span>{userProfile.email}</div>
                    <div><span className="font-medium">链接：</span>{generateProfileLink()}</div>
                    <div><span className="font-medium">简介：</span>{userProfile.bio}</div>
                  </>
                )}
              </div>
              
              <div className="grid grid-cols-3 gap-2">
                <Button
                  variant="outline"
                  onClick={() => copyToClipboard(generateProfileLink())}
                  className="flex items-center gap-1 text-xs"
                >
                  <Copy size={14} />
                  复制链接
                </Button>
                <Button
                  variant="outline"
                  onClick={regenerateQRCode}
                  disabled={isGeneratingQR}
                  className="flex items-center gap-1 text-xs"
                >
                  <QrCode size={14} />
                  重新生成
                </Button>
                <Button
                  onClick={downloadQRCode}
                  disabled={!qrCodeDataUrl || isGeneratingQR}
                  className="flex items-center gap-1 text-xs"
                >
                  <Download size={14} />
                  保存图片
                </Button>
              </div>
            </div>
          </div>
          
          {/* 隐藏的canvas用于生成二维码 */}
          <canvas ref={qrCanvasRef} style={{ display: 'none' }} />
        </DialogContent>
      </Dialog>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default ProfileScreen