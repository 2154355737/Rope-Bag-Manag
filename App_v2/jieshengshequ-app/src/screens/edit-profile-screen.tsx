import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { ArrowLeft, Camera, Save, X, User, Mail, MapPin, Globe, Tag } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'

const EditProfileScreen: React.FC = () => {
  const navigate = useNavigate()
  const [isSaving, setIsSaving] = useState(false)
  
  // 用户资料状态
  const [userProfile, setUserProfile] = useState({
    name: '程序员小王',
    bio: '结绳语言爱好者，专注移动开发',
    avatar: 'https://i.pravatar.cc/150?img=5',
    email: 'xiaowang@example.com',
    location: '北京市',
    website: 'https://github.com/xiaowang',
    skills: 'React, TypeScript, 结绳语言, 移动开发, Tailwind CSS, Node.js'
  })
  
  // 表单验证错误
  const [errors, setErrors] = useState<Record<string, string>>({})
  
  // 处理输入变化
  const handleInputChange = (field: string, value: string) => {
    setUserProfile(prev => ({
      ...prev,
      [field]: value
    }))
    
    // 清除对应字段的错误
    if (errors[field]) {
      setErrors(prev => ({
        ...prev,
        [field]: ''
      }))
    }
  }
  
  // 处理头像上传
  const handleAvatarUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file) {
      // 验证文件类型
      if (!file.type.startsWith('image/')) {
        toast({
          title: "上传失败",
          description: "请选择图片文件",
          variant: "destructive",
        })
        return
      }
      
      // 验证文件大小（5MB）
      if (file.size > 5 * 1024 * 1024) {
        toast({
          title: "上传失败",
          description: "图片大小不能超过5MB",
          variant: "destructive",
        })
        return
      }
      
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
          })
        }
      }
      reader.readAsDataURL(file)
    }
  }
  
  // 表单验证
  const validateForm = () => {
    const newErrors: Record<string, string> = {}
    
    if (!userProfile.name.trim()) {
      newErrors.name = '用户名不能为空'
    } else if (userProfile.name.length > 20) {
      newErrors.name = '用户名不能超过20个字符'
    }
    
    if (userProfile.bio.length > 100) {
      newErrors.bio = '个人简介不能超过100个字符'
    }
    
    if (userProfile.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(userProfile.email)) {
      newErrors.email = '请输入有效的邮箱地址'
    }
    
    if (userProfile.website && !/^https?:\/\/.+/.test(userProfile.website)) {
      newErrors.website = '请输入有效的网址（以http://或https://开头）'
    }
    
    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }
  
  // 处理保存
  const handleSave = async () => {
    if (!validateForm()) {
      return
    }
    
    setIsSaving(true)
    
    try {
      // 模拟API调用
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      toast({
        title: "保存成功",
        description: "您的个人资料已更新",
      })
      
      // 返回上一页
      navigate(-1)
    } catch (error) {
      toast({
        title: "保存失败",
        description: "请稍后重试",
        variant: "destructive",
      })
    } finally {
      setIsSaving(false)
    }
  }
  
  return (
    <div className="flex flex-col min-h-screen bg-background">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="编辑资料"
        showBackButton
        onBackClick={() => navigate(-1)}
      />

      {/* 内容区域 */}
      <div className="flex-1 pt-nav pb-4">
        <div className="max-w-md mx-auto px-4 space-y-6">
          
          {/* 头像区域 */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
          >
            <Card>
              <CardContent className="p-6">
                <div className="flex flex-col items-center">
                  <div className="relative">
                    <Avatar className="h-24 w-24">
                      <AvatarImage src={userProfile.avatar} />
                      <AvatarFallback className="text-2xl">{userProfile.name[0]}</AvatarFallback>
                    </Avatar>
                    <label className="absolute bottom-0 right-0 bg-primary text-primary-foreground rounded-full p-3 cursor-pointer hover:bg-primary/90 transition-all duration-200 shadow-lg hover:shadow-xl hover:scale-105 border-2 border-background">
                      <Camera size={16} />
                      <input
                        type="file"
                        accept="image/*"
                        onChange={handleAvatarUpload}
                        className="hidden"
                      />
                    </label>
                  </div>
                  <p className="text-sm text-muted-foreground mt-3 text-center">
                    点击相机图标更换头像<br />
                    支持 JPG、PNG 格式，大小不超过 5MB
                  </p>
                </div>
              </CardContent>
            </Card>
          </motion.div>

          {/* 基本信息 */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.1 }}
          >
            <Card>
              <CardContent className="p-6 space-y-4">
                <h3 className="text-lg font-semibold flex items-center">
                  <User size={20} className="mr-2 text-primary" />
                  基本信息
                </h3>
                
                <div className="space-y-4">
                  {/* 用户名 */}
                  <div className="space-y-2">
                    <Label htmlFor="name" className="text-sm font-medium">
                      用户名 <span className="text-red-500">*</span>
                    </Label>
                    <Input
                      id="name"
                      value={userProfile.name}
                      onChange={(e) => handleInputChange('name', e.target.value)}
                      maxLength={20}
                      className={`${errors.name ? 'border-red-500' : ''}`}
                      placeholder="请输入用户名"
                    />
                    {errors.name && (
                      <p className="text-sm text-red-500">{errors.name}</p>
                    )}
                    <div className="text-xs text-muted-foreground text-right">
                      {userProfile.name.length}/20
                    </div>
                  </div>

                  {/* 个人简介 */}
                  <div className="space-y-2">
                    <Label htmlFor="bio" className="text-sm font-medium">个人简介</Label>
                    <Textarea
                      id="bio"
                      value={userProfile.bio}
                      onChange={(e) => handleInputChange('bio', e.target.value)}
                      maxLength={100}
                      rows={3}
                      className={`resize-none ${errors.bio ? 'border-red-500' : ''}`}
                      placeholder="介绍一下你自己..."
                    />
                    {errors.bio && (
                      <p className="text-sm text-red-500">{errors.bio}</p>
                    )}
                    <div className="text-xs text-muted-foreground text-right">
                      {userProfile.bio.length}/100
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          </motion.div>

          {/* 联系信息 */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.2 }}
          >
            <Card>
              <CardContent className="p-6 space-y-4">
                <h3 className="text-lg font-semibold flex items-center">
                  <Mail size={20} className="mr-2 text-primary" />
                  联系信息
                </h3>
                
                <div className="space-y-4">
                  {/* 邮箱 */}
                  <div className="space-y-2">
                    <Label htmlFor="email" className="text-sm font-medium">邮箱地址</Label>
                    <Input
                      id="email"
                      type="email"
                      value={userProfile.email}
                      onChange={(e) => handleInputChange('email', e.target.value)}
                      className={`${errors.email ? 'border-red-500' : ''}`}
                      placeholder="your@email.com"
                    />
                    {errors.email && (
                      <p className="text-sm text-red-500">{errors.email}</p>
                    )}
                  </div>

                  {/* 所在地 */}
                  <div className="space-y-2">
                    <Label htmlFor="location" className="text-sm font-medium flex items-center">
                      <MapPin size={16} className="mr-1" />
                      所在地
                    </Label>
                    <Input
                      id="location"
                      value={userProfile.location}
                      onChange={(e) => handleInputChange('location', e.target.value)}
                      placeholder="北京市"
                    />
                  </div>

                  {/* 个人网站 */}
                  <div className="space-y-2">
                    <Label htmlFor="website" className="text-sm font-medium flex items-center">
                      <Globe size={16} className="mr-1" />
                      个人网站
                    </Label>
                    <Input
                      id="website"
                      type="url"
                      value={userProfile.website}
                      onChange={(e) => handleInputChange('website', e.target.value)}
                      className={`${errors.website ? 'border-red-500' : ''}`}
                      placeholder="https://your-website.com"
                    />
                    {errors.website && (
                      <p className="text-sm text-red-500">{errors.website}</p>
                    )}
                  </div>
                </div>
              </CardContent>
            </Card>
          </motion.div>

          {/* 技能标签 */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.3 }}
          >
            <Card>
              <CardContent className="p-6 space-y-4">
                <h3 className="text-lg font-semibold flex items-center">
                  <Tag size={20} className="mr-2 text-primary" />
                  技能标签
                </h3>
                
                <div className="space-y-2">
                  <Label htmlFor="skills" className="text-sm font-medium">技能专长</Label>
                  <Textarea
                    id="skills"
                    value={userProfile.skills}
                    onChange={(e) => handleInputChange('skills', e.target.value)}
                    rows={3}
                    className="resize-none"
                    placeholder="React, TypeScript, 移动开发..."
                  />
                  <div className="text-xs text-muted-foreground">
                    💡 用逗号分隔多个技能，将显示为标签
                  </div>
                </div>
              </CardContent>
            </Card>
          </motion.div>

          {/* 保存按钮区域 */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.4 }}
            className="pb-4"
          >
            <div className="flex gap-3">
              <Button
                variant="outline"
                onClick={() => navigate(-1)}
                disabled={isSaving}
                className="flex-1"
              >
                <X size={16} className="mr-2" />
                取消
              </Button>
              <Button
                onClick={handleSave}
                disabled={isSaving}
                className="flex-1 bg-primary hover:bg-primary/90"
              >
                {isSaving ? (
                  <>
                    <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                    保存中...
                  </>
                ) : (
                  <>
                    <Save size={16} className="mr-2" />
                    保存更改
                  </>
                )}
              </Button>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  )
}

export default EditProfileScreen 