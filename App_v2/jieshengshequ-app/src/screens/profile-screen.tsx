import React, { useState, useRef, useEffect } from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '@/contexts/AuthContext'
import { Settings, Edit, LogOut, BookOpen, Heart, Bookmark, ChevronRight, Moon, Sun, Camera, Save, X, Share2, QrCode, Award, Copy, Download, FileText, MessageSquare, ChevronDown, RefreshCw } from 'lucide-react'
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
import { getCurrentUserProfile, User } from '@/api/auth'
import { 
  getMyStats, 
  getMyActivityStats, 
  getMyWeeklyReport, 
  getMyAchievements,
  getMyResources,
  getMyPosts,
  getMyComments,
  uploadAvatar,
  type UserStats,
  type UserActivityStats,
  type WeeklyReportData,
  type Achievement
} from '@/api/user'

const ProfileScreen: React.FC = () => {
  const navigate = useNavigate()
  const { theme, setTheme } = useTheme()
  const { logout } = useAuth()
  
  // 数据加载状态
  const [isLoading, setIsLoading] = useState(true)
  const [isRefreshing, setIsRefreshing] = useState(false)
  const [isUpdating, setIsUpdating] = useState(false) // 新增：头像上传状态
  
  // API数据状态
  const [currentUser, setCurrentUser] = useState<User | null>(null)
  const [myStats, setMyStats] = useState<UserStats | null>(null)
  const [activityStats, setActivityStats] = useState<UserActivityStats | null>(null)
  const [weeklyReport, setWeeklyReport] = useState<WeeklyReportData | null>(null)
  const [achievements, setAchievements] = useState<Achievement[]>([])
  
  // 内容数据状态
  const [myResources, setMyResources] = useState<any[]>([])
  const [myPosts, setMyPosts] = useState<any[]>([])
  const [myComments, setMyComments] = useState<any[]>([])
  
  // 分享和二维码状态管理
  const [isShareDialogOpen, setIsShareDialogOpen] = useState(false)
  const [isQRDialogOpen, setIsQRDialogOpen] = useState(false)
  const [qrCodeDataUrl, setQrCodeDataUrl] = useState('')
  const [isGeneratingQR, setIsGeneratingQR] = useState(false)
  const [qrContentType, setQrContentType] = useState<'url' | 'vcard'>('url')
  const qrCanvasRef = useRef<HTMLCanvasElement>(null)
  
  // 周报折叠状态
  const [isWeeklyReportExpanded, setIsWeeklyReportExpanded] = useState(false)
  
  
  // 格式化数字显示
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 获取状态显示信息
  const getStatusInfo = (status: string) => {
    // 统一转换为小写进行比较
    const normalizedStatus = status?.toLowerCase()
    
    switch (normalizedStatus) {
      case 'published':
      case 'active':
        return {
          text: '已发布',
          variant: 'default' as const,
          className: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'
        }
      case 'pending':
      case 'draft':
        return {
          text: '待审核',
          variant: 'secondary' as const,
          className: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400'
        }
      case 'rejected':
      case 'inactive':
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
    const profileId = (currentUser?.username || currentUser?.nickname || 'user').toLowerCase().replace(/\s+/g, '-')
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
FN:${currentUser?.nickname || '用户'}
ORG:结绳社区
TITLE:${activityStats?.level || '用户'}
EMAIL:${currentUser?.email || 'user@example.com'}
URL:${profileLink}
NOTE:${currentUser?.bio || ''}
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
      link.download = `${currentUser?.nickname || '用户'}-个人二维码.png`
      
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
      title: `${currentUser?.nickname || '用户'} - 结绳社区`,
      text: `来看看 ${currentUser?.nickname || '用户'} 在结绳社区的个人资料`,
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
  const handleAvatarUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file) {
      try {
        setIsUpdating(true)
        
        // 调用头像上传API
        const result = await uploadAvatar(file)
        
        // 更新当前用户数据
        setCurrentUser(prev => prev ? {
          ...prev,
          avatar_url: result.avatar_url
        } : null)
        
        toast({
          title: "头像上传成功",
          description: "您的头像已更新",
          duration: 3000,
        })
        
        // 重新加载用户数据以确保同步
        await loadUserData()
      } catch (error) {
        console.error('头像上传失败:', error)
        toast({
          title: "头像上传失败",
          description: "请稍后重试",
          variant: "destructive",
          duration: 3000,
        })
      } finally {
        setIsUpdating(false)
      }
    }
  }
  
  // 数据加载函数
  const loadUserData = async () => {
    try {
      setIsLoading(true)
      
      // 检查token
      const token = localStorage.getItem('token')
      console.log('当前token:', token ? '存在' : '不存在')
      
      // 并行加载所有用户数据
      console.log('开始加载用户数据...')
      const [user, stats, activity, weekly, achievementsData, resources, posts, comments] = await Promise.all([
        getCurrentUserProfile().catch((err) => { console.error('获取用户信息失败:', err); return null }),
        getMyStats().catch((err) => { console.error('获取用户统计失败:', err); return null }),
        getMyActivityStats().catch((err) => { console.error('获取活动统计失败:', err); return null }),
        getMyWeeklyReport().catch((err) => { console.error('获取周报失败:', err); return null }),
        getMyAchievements().catch((err) => { console.error('获取成就失败:', err); return { list: [], total: 0 } }),
        getMyResources().catch((err) => { console.error('获取资源失败:', err); return { list: [], total: 0 } }),
        getMyPosts().catch((err) => { console.error('获取帖子失败:', err); return { list: [], total: 0 } }),
        getMyComments().catch((err) => { console.error('获取评论失败:', err); return { list: [], total: 0 } })
      ])
      
      console.log('API调用结果:', { user, stats, activity, weekly, achievementsData, resources, posts, comments })
      
      setCurrentUser(user)
      setMyStats(stats)
      setActivityStats(activity)
      setWeeklyReport(weekly)
      setAchievements(achievementsData?.list || [])
      setMyResources(resources?.list || [])
      setMyPosts(posts?.list || [])
      setMyComments(comments?.list || [])
    } catch (error) {
      console.error('加载用户数据失败:', error)
      toast({
        title: "加载失败",
        description: "无法加载用户数据，请稍后重试",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }

  // 刷新数据
  const refreshUserData = async () => {
    setIsRefreshing(true)
    await loadUserData()
    setIsRefreshing(false)
  }

  // 退出登录
  const handleLogout = async () => {
    try {
      await logout()
      toast({
        title: "退出成功",
        description: "您已成功退出登录",
        variant: "default"
      })
      navigate('/login')
    } catch (error) {
      console.error('退出登录失败:', error)
      toast({
        title: "退出失败", 
        description: "退出登录时发生错误，请重试",
        variant: "destructive"
      })
    }
  }

  // 组件挂载时加载数据
  useEffect(() => {
    loadUserData()
  }, [])

  // 加载骨架屏组件
  const ProfileSkeleton = () => (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="个人中心"
        subtitle="加载中..."
        showSettingsButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="icon"
              className="h-9 w-9"
              disabled
            >
              <RefreshCw size={20} className="animate-spin" />
            </Button>
          </div>
        }
      />

      {/* 主要内容区域 */}
      <div className="pt-nav flex-1 overflow-y-auto">
        {/* 个人信息卡片骨架 */}
        <div className="p-4 border-b">
          <div className="flex items-start gap-4">
            <div className="flex flex-col items-center">
              <div className="relative flex-shrink-0">
                <div className="h-20 w-20 rounded-full bg-muted animate-pulse"></div>
              </div>
            </div>
            
            <div className="flex-1 min-w-0">
              <div className="flex items-center justify-between">
                <div className="h-6 bg-muted rounded animate-pulse w-32"></div>
                <div className="h-8 bg-muted rounded animate-pulse w-20"></div>
              </div>
              
              <div className="h-4 bg-muted rounded animate-pulse w-48 mt-3"></div>
              
              <div className="flex items-center mb-3 mt-3">
                <div className="h-6 bg-muted rounded animate-pulse w-24"></div>
              </div>
              
              {/* 技能标签骨架 */}
              <div className="flex flex-wrap gap-2">
                {[1,2,3,4].map((i) => (
                  <div key={i} className="h-6 bg-muted rounded animate-pulse w-16"></div>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* 统计数据骨架 */}
        <div className="grid grid-cols-4 gap-4 p-4 border-b">
          {[1,2,3,4].map((i) => (
            <div key={i} className="text-center">
              <div className="h-6 bg-muted rounded animate-pulse w-8 mx-auto mb-1"></div>
              <div className="h-4 bg-muted rounded animate-pulse w-12 mx-auto"></div>
            </div>
          ))}
        </div>

        {/* 周报卡片骨架 */}
        <div className="p-4 space-y-4">
          <div className="h-48 bg-muted rounded-lg animate-pulse"></div>
          
          {/* 内容预览骨架 */}
          <div className="h-64 bg-muted rounded-lg animate-pulse"></div>
        </div>
      </div>
    </div>
  )

  // 如果正在加载，显示骨架屏
  if (isLoading) {
    return <ProfileSkeleton />
  }

  return (
    <motion.div 
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.3 }}
      className="flex flex-col min-h-screen bg-background pb-16"
    >
      {/* 顶部导航栏 */}
      <TopNavigation
        title="个人中心"
        subtitle={activityStats?.level || '用户'}
        showSettingsButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="icon"
              className="h-9 w-9"
              onClick={refreshUserData}
              disabled={isRefreshing}
            >
              <RefreshCw size={20} className={isRefreshing ? 'animate-spin' : ''} />
            </Button>
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
                <AvatarImage src={currentUser?.avatar_url || 'https://via.placeholder.com/150'} />
                <AvatarFallback>{currentUser?.nickname || currentUser?.username || '用户'[0]}</AvatarFallback>
              </Avatar>
              <label className="absolute bottom-0 right-0 bg-primary text-primary-foreground rounded-full p-2 cursor-pointer hover:bg-primary/90 transition-all duration-200 shadow-lg hover:shadow-xl hover:scale-105 border-2 border-background disabled:opacity-50 disabled:cursor-not-allowed">
                {isUpdating ? (
                  <RefreshCw size={14} className="animate-spin" />
                ) : (
                  <Camera size={14} />
                )}
                <input
                  type="file"
                  accept="image/*"
                  onChange={handleAvatarUpload}
                  className="hidden"
                  disabled={isUpdating}
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
              <h2 className="text-xl font-bold">{currentUser?.nickname || currentUser?.username || '用户'}</h2>
              <Button 
                variant="outline" 
                size="sm" 
                className="flex items-center" 
                onClick={() => navigate('/edit-profile')}
              >
                <Edit size={14} className="mr-1" /> 编辑
              </Button>

            </div>
            <p className="text-muted-foreground text-sm mb-3">{currentUser?.bio || ''}</p>
            
            {/* 等级标签 */}
            <div className="flex items-center mb-3">
              <Badge variant="outline" className="bg-primary/10 text-primary border-primary/20 font-medium">
                {activityStats?.level || '用户'}
              </Badge>
            </div>
            
            {/* 技能标签 */}
            <div className="space-y-2">
              <div className="text-xs text-muted-foreground font-medium">技能专长</div>
              <div className="flex flex-wrap gap-2">
                {(() => {
                  // 处理技能数据：可能是字符串（逗号分隔）或数组
                  let skillsArray: string[] = []
                  if (currentUser?.skills) {
                    skillsArray = typeof currentUser.skills === 'string' 
                      ? currentUser.skills.split(',').map(s => s.trim()).filter(s => s.length > 0)
                      : Array.isArray(currentUser.skills) ? currentUser.skills : []
                  }
                  if (skillsArray.length === 0) {
                    skillsArray = ['技能1', '技能2', '技能3'] // 使用默认技能
                  }
                  
                  return skillsArray.slice(0, 6).map((skill, index) => (
                    <Badge 
                      key={index} 
                      variant="secondary" 
                      className="text-xs bg-muted hover:bg-muted/80 transition-colors cursor-default px-3 py-1 rounded-full"
                    >
                      {skill}
                    </Badge>
                  ))
                })()}
                {(() => {
                  let skillsArray: string[] = []
                  if (currentUser?.skills) {
                    skillsArray = typeof currentUser.skills === 'string' 
                      ? currentUser.skills.split(',').map(s => s.trim()).filter(s => s.length > 0)
                      : Array.isArray(currentUser.skills) ? currentUser.skills : []
                  }
                  if (skillsArray.length === 0) {
                    skillsArray = ['技能1', '技能2', '技能3']
                  }
                  
                  return skillsArray.length > 6 && (
                    <Badge 
                      variant="outline" 
                      className="text-xs text-muted-foreground border-dashed cursor-default px-3 py-1 rounded-full"
                    >
                      +{skillsArray.length - 6}
                    </Badge>
                  )
                })()}
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
              <span className="text-sm font-bold text-primary">
                {weeklyReport?.today_activity || 0}%
              </span>
              <span className="text-xs text-muted-foreground">+12%</span>
            </div>
          </div>
          <div className="mt-2 bg-background/50 rounded-full h-2">
            <div 
              className="bg-primary h-2 rounded-full transition-all duration-500" 
              style={{ width: `${weeklyReport?.today_activity || 0}%` }}
            ></div>
          </div>
          <div className="flex justify-between text-xs text-muted-foreground mt-1">
            <span>已发布 {myStats?.posts || 0} 篇内容</span>
            <span>获得 {myStats?.likes || 0} 个赞</span>
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
                <div className="text-2xl font-bold text-primary">{weeklyReport?.total_posts || 0}</div>
                <div className="text-xs text-muted-foreground">总发布</div>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="p-3 text-center">
                <div className="text-2xl font-bold text-primary">{weeklyReport?.completed_projects || 0}</div>
                <div className="text-xs text-muted-foreground">完成项目</div>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="p-3 text-center">
                <div className="text-2xl font-bold text-primary">{weeklyReport?.current_streak || 0}</div>
                <div className="text-xs text-muted-foreground">连续天数</div>
              </CardContent>
            </Card>
          </div>
          
          <Card className="mb-4">
            <CardContent className="p-3">
              <div className="flex items-center justify-between mb-3">
                <h4 className="text-sm font-medium">本周发布</h4>
                <div className="text-xs text-muted-foreground">
                  总计: {(weeklyReport?.weekly_posts || []).reduce((sum, posts) => sum + posts, 0)} 篇
                </div>
              </div>
              
                          {/* 图表容器 - 独立区域，不会影响上方标题 */}
              <div className="relative">
                {/* 纯柱状图区域 */}
                <div className="flex items-end h-16 gap-1 mb-2">
                  {(weeklyReport?.weekly_posts || [0,0,0,0,0,0,0]).map((posts, index) => {
                    // 限制最大高度，确保不会超出容器
                    const weeklyPosts = weeklyReport?.weekly_posts || [0,0,0,0,0,0,0]
                    const maxDisplayValue = Math.max(...weeklyPosts)
                    const height = maxDisplayValue > 0 ? (posts / maxDisplayValue) * 90 : 0
                    
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
                  {(weeklyReport?.weekly_posts || [0,0,0,0,0,0,0]).map((posts, index) => {
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
                    平均: {((weeklyReport?.weekly_posts || []).reduce((sum, posts) => sum + posts, 0) / 7).toFixed(1)} 篇/天
                  </motion.div>
                </div>
              </div>
            </CardContent>
          </Card>
          
          <h4 className="text-sm font-medium mb-2">成就徽章</h4>
          <div className="flex gap-3 overflow-x-auto pb-2">
            {(achievements.length > 0 ? achievements : weeklyReport?.achievements || []).map((achievement) => (
              <div key={achievement.id} className="flex flex-col items-center min-w-[60px]">
                <div className="flex items-center justify-center w-12 h-12 rounded-full bg-primary/10 mb-1">
                  <span className="text-2xl">{achievement.icon}</span>
                </div>
                <div className="text-xs text-center">{achievement.name}</div>
                <div className="text-xs text-muted-foreground text-center">{achievement.description}</div>
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
                <div className="text-lg font-bold">{myResources.length}</div>
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
                <div className="text-lg font-bold">{myPosts.length}</div>
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
                <div className="text-lg font-bold">{myComments.length}</div>
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
              {myResources.slice(0, 4).map((resource) => (
                <motion.div
                  key={resource.id}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                  className="cursor-pointer"
                  onClick={() => navigate(`/resource/${resource.id}`)}
                >
                  <Card className="overflow-hidden relative hover:shadow-lg transition-shadow">
                    <div className="absolute top-2 right-2 z-10">
                      <Badge className={`text-xs px-2 py-0.5 ${getStatusInfo(resource.status).className}`}>
                        {getStatusInfo(resource.status).text}
                      </Badge>
                    </div>
                    {/* 使用默认图片，因为后端资源数据没有图片字段 */}
                    <div className="w-full h-24 bg-gradient-to-br from-primary/10 to-primary/5 flex items-center justify-center">
                      <BookOpen size={32} className="text-primary/60" />
                    </div>
                    <CardContent className="p-2">
                      <h4 className="text-sm font-medium line-clamp-1">{resource.name}</h4>
                      <div className="flex items-center justify-between text-xs text-muted-foreground mt-1">
                        <div className="flex items-center">
                          <Heart size={12} className="mr-1" />
                          <span>{resource.like_count}</span>
                        </div>
                        <div className="flex items-center">
                          <BookOpen size={12} className="mr-1" />
                          <span>{resource.download_count}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
            
            {myResources.length > 4 && (
              <div className="text-center mt-4">
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={() => navigate('/my-content?tab=resources')}
                  className="w-full"
                >
                  查看全部 {myResources.length} 个资源
                  <ChevronRight size={14} className="ml-1" />
                </Button>
              </div>
            )}
          </TabsContent>
          
          <TabsContent value="posts" className="mt-0">
            <div className="grid grid-cols-2 gap-3">
              {myPosts.slice(0, 4).map((post) => (
                <motion.div
                  key={post.id}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                  className="cursor-pointer"
                  onClick={() => navigate(`/post/${post.id}`)}
                >
                  <Card className="overflow-hidden relative">
                    <div className="absolute top-2 right-2 z-10">
                      <Badge className={`text-xs px-2 py-0.5 ${getStatusInfo(post.status).className}`}>
                        {getStatusInfo(post.status).text}
                      </Badge>
                    </div>
                    {/* 使用默认图片，因为后端帖子数据没有图片字段 */}
                    <div className="w-full h-24 bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 flex items-center justify-center">
                      <FileText size={32} className="text-blue-500/60" />
                    </div>
                    <CardContent className="p-2">
                      <h4 className="text-sm font-medium line-clamp-1">{post.title}</h4>
                      <div className="flex items-center justify-between text-xs text-muted-foreground mt-1">
                        <span>{post.author_name || '匿名'}</span>
                        <div className="flex items-center">
                          <Heart size={12} className="mr-1" />
                          <span>{post.like_count}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
            
            {myPosts.length > 4 && (
              <div className="text-center mt-4">
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={() => navigate('/my-content?tab=posts')}
                  className="w-full"
                >
                  查看全部 {myPosts.length} 个帖子
                  <ChevronRight size={14} className="ml-1" />
                </Button>
              </div>
            )}
          </TabsContent>
          
          <TabsContent value="comments" className="mt-0">
            <div className="space-y-3">
              {myComments.slice(0, 4).map((comment) => (
                <motion.div
                  key={comment.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.3 }}
                  className="cursor-pointer"
                  onClick={() => {
                    // 根据评论的目标类型跳转到对应页面
                    if (comment.target_type === 'post') {
                      navigate(`/post/${comment.target_id}`)
                    } else if (comment.target_type === 'resource') {
                      navigate(`/resource/${comment.target_id}`)
                    }
                  }}
                >
                  <Card className="hover:shadow-lg transition-shadow">
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
            
            {myComments.length > 4 && (
              <div className="text-center mt-4">
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={() => navigate('/my-content?tab=comments')}
                  className="w-full"
                >
                  查看全部 {myComments.length} 条评论
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
        
        <Button 
          variant="outline" 
          className="w-full mt-4 text-destructive"
          onClick={handleLogout}
        >
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
              <h3 className="font-medium mb-2">{currentUser?.nickname || currentUser?.username || '用户'}</h3>
              <p className="text-sm text-muted-foreground mb-4">{currentUser?.bio || ''}</p>
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
                扫描二维码查看 {currentUser?.nickname || currentUser?.username || '用户'} 的个人资料
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
                    <div><span className="font-medium">姓名：</span>{currentUser?.nickname || currentUser?.username || '用户'}</div>
                    <div><span className="font-medium">等级：</span>{activityStats?.level || '用户'}</div>
                    <div><span className="font-medium">邮箱：</span>{currentUser?.email || 'user@example.com'}</div>
                    <div><span className="font-medium">链接：</span>{generateProfileLink()}</div>
                    <div><span className="font-medium">简介：</span>{currentUser?.bio || ''}</div>
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
    </div> {/* 结束 pt-nav div */}
    </motion.div>
  )
}

export default ProfileScreen