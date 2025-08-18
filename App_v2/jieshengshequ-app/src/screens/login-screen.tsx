import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Eye, EyeOff, Mail, User, Lock, ArrowLeft } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Separator } from '@/components/ui/separator'
import { toast } from '@/hooks/use-toast'

const LoginScreen: React.FC = () => {
  const navigate = useNavigate()
  
  // 表单状态
  const [isLoading, setIsLoading] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  
  // 账号登录表单
  const [accountForm, setAccountForm] = useState({
    username: '',
    password: '',
    remember: false
  })
  
  // 邮箱登录表单
  const [emailForm, setEmailForm] = useState({
    email: '',
    password: '',
    remember: false
  })
  
  // 表单验证
  const validateAccountForm = () => {
    if (!accountForm.username.trim()) {
      toast({
        title: "验证失败",
        description: "请输入用户名",
        variant: "destructive"
      })
      return false
    }
    if (!accountForm.password) {
      toast({
        title: "验证失败",
        description: "请输入密码",
        variant: "destructive"
      })
      return false
    }
    if (accountForm.password.length < 6) {
      toast({
        title: "验证失败",
        description: "密码长度不能少于6位",
        variant: "destructive"
      })
      return false
    }
    return true
  }
  
  const validateEmailForm = () => {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!emailForm.email.trim()) {
      toast({
        title: "验证失败",
        description: "请输入邮箱地址",
        variant: "destructive"
      })
      return false
    }
    if (!emailRegex.test(emailForm.email)) {
      toast({
        title: "验证失败",
        description: "请输入有效的邮箱地址",
        variant: "destructive"
      })
      return false
    }
    if (!emailForm.password) {
      toast({
        title: "验证失败",
        description: "请输入密码",
        variant: "destructive"
      })
      return false
    }
    if (emailForm.password.length < 6) {
      toast({
        title: "验证失败",
        description: "密码长度不能少于6位",
        variant: "destructive"
      })
      return false
    }
    return true
  }
  
  // 账号登录处理
  const handleAccountLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!validateAccountForm()) return
    
    setIsLoading(true)
    try {
      // 模拟登录API调用
      await new Promise(resolve => setTimeout(resolve, 1500))
      
      toast({
        title: "登录成功",
        description: `欢迎回来，${accountForm.username}！`,
        variant: "default"
      })
      
      // 登录成功后跳转到主页
      navigate('/', { replace: true })
    } catch (error) {
      toast({
        title: "登录失败",
        description: "用户名或密码错误，请重试",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }
  
  // 邮箱登录处理
  const handleEmailLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!validateEmailForm()) return
    
    setIsLoading(true)
    try {
      // 模拟登录API调用
      await new Promise(resolve => setTimeout(resolve, 1500))
      
      toast({
        title: "登录成功",
        description: `欢迎回来！`,
        variant: "default"
      })
      
      // 登录成功后跳转到主页
      navigate('/', { replace: true })
    } catch (error) {
      toast({
        title: "登录失败",
        description: "邮箱或密码错误，请重试",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }
  
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
        className="w-full max-w-md"
      >
        {/* 返回按钮 */}
        <Button
          variant="ghost"
          size="sm"
          onClick={() => navigate(-1)}
          className="mb-6"
        >
          <ArrowLeft size={16} className="mr-2" />
          返回
        </Button>
        
        <Card className="shadow-lg">
          <CardHeader className="text-center">
            <div className="mx-auto mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-primary/10">
              <span className="text-3xl">🪢</span>
            </div>
            <CardTitle className="text-2xl">欢迎回来</CardTitle>
            <CardDescription>
              登录您的结绳社区账号
            </CardDescription>
          </CardHeader>
          
          <CardContent>
            <Tabs defaultValue="account" className="w-full">
              <TabsList className="grid w-full grid-cols-2 mb-6">
                <TabsTrigger value="account" className="flex items-center">
                  <User size={16} className="mr-2" />
                  账号登录
                </TabsTrigger>
                <TabsTrigger value="email" className="flex items-center">
                  <Mail size={16} className="mr-2" />
                  邮箱登录
                </TabsTrigger>
              </TabsList>
              
              {/* 账号登录 */}
              <TabsContent value="account">
                <form onSubmit={handleAccountLogin} className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="username">用户名</Label>
                    <div className="relative">
                      <User size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                      <Input
                        id="username"
                        type="text"
                        placeholder="请输入用户名"
                        value={accountForm.username}
                        onChange={(e) => setAccountForm(prev => ({ ...prev, username: e.target.value }))}
                        className="pl-10"
                        disabled={isLoading}
                      />
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <Label htmlFor="account-password">密码</Label>
                    <div className="relative">
                      <Lock size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                      <Input
                        id="account-password"
                        type={showPassword ? "text" : "password"}
                        placeholder="请输入密码"
                        value={accountForm.password}
                        onChange={(e) => setAccountForm(prev => ({ ...prev, password: e.target.value }))}
                        className="pl-10 pr-10"
                        disabled={isLoading}
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        onClick={() => setShowPassword(!showPassword)}
                        className="absolute right-0 top-0 h-full px-3 hover:bg-transparent"
                        disabled={isLoading}
                      >
                        {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
                      </Button>
                    </div>
                  </div>
                  
                  <div className="flex items-center justify-between">
                    <label className="flex items-center space-x-2 text-sm">
                      <input
                        type="checkbox"
                        checked={accountForm.remember}
                        onChange={(e) => setAccountForm(prev => ({ ...prev, remember: e.target.checked }))}
                        className="rounded border-gray-300"
                        disabled={isLoading}
                      />
                      <span>记住我</span>
                    </label>
                    <Button
                      type="button"
                      variant="link"
                      size="sm"
                      onClick={() => navigate('/forgot-password')}
                      className="p-0 h-auto"
                      disabled={isLoading}
                    >
                      忘记密码？
                    </Button>
                  </div>
                  
                  <Button
                    type="submit"
                    className="w-full"
                    disabled={isLoading}
                  >
                    {isLoading ? "登录中..." : "登录"}
                  </Button>
                </form>
              </TabsContent>
              
              {/* 邮箱登录 */}
              <TabsContent value="email">
                <form onSubmit={handleEmailLogin} className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="email">邮箱地址</Label>
                    <div className="relative">
                      <Mail size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                      <Input
                        id="email"
                        type="email"
                        placeholder="请输入邮箱地址"
                        value={emailForm.email}
                        onChange={(e) => setEmailForm(prev => ({ ...prev, email: e.target.value }))}
                        className="pl-10"
                        disabled={isLoading}
                      />
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <Label htmlFor="email-password">密码</Label>
                    <div className="relative">
                      <Lock size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                      <Input
                        id="email-password"
                        type={showPassword ? "text" : "password"}
                        placeholder="请输入密码"
                        value={emailForm.password}
                        onChange={(e) => setEmailForm(prev => ({ ...prev, password: e.target.value }))}
                        className="pl-10 pr-10"
                        disabled={isLoading}
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        onClick={() => setShowPassword(!showPassword)}
                        className="absolute right-0 top-0 h-full px-3 hover:bg-transparent"
                        disabled={isLoading}
                      >
                        {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
                      </Button>
                    </div>
                  </div>
                  
                  <div className="flex items-center justify-between">
                    <label className="flex items-center space-x-2 text-sm">
                      <input
                        type="checkbox"
                        checked={emailForm.remember}
                        onChange={(e) => setEmailForm(prev => ({ ...prev, remember: e.target.checked }))}
                        className="rounded border-gray-300"
                        disabled={isLoading}
                      />
                      <span>记住我</span>
                    </label>
                    <Button
                      type="button"
                      variant="link"
                      size="sm"
                      onClick={() => navigate('/forgot-password')}
                      className="p-0 h-auto"
                      disabled={isLoading}
                    >
                      忘记密码？
                    </Button>
                  </div>
                  
                  <Button
                    type="submit"
                    className="w-full"
                    disabled={isLoading}
                  >
                    {isLoading ? "登录中..." : "登录"}
                  </Button>
                </form>
              </TabsContent>
            </Tabs>
            
            <Separator className="my-6" />
            
            <div className="text-center">
              <span className="text-sm text-muted-foreground">还没有账号？</span>
              <Button
                variant="link"
                size="sm"
                onClick={() => navigate('/register')}
                className="p-0 h-auto ml-1"
                disabled={isLoading}
              >
                立即注册
              </Button>
            </div>
          </CardContent>
        </Card>
        
        <div className="mt-6 text-center text-xs text-muted-foreground">
          <p>登录即表示您同意我们的</p>
          <div className="flex justify-center space-x-4 mt-1">
            <Button
              variant="link"
              size="sm"
              onClick={() => navigate('/privacy')}
              className="p-0 h-auto text-xs"
            >
              隐私政策
            </Button>
            <span>和</span>
            <Button
              variant="link"
              size="sm"
              onClick={() => navigate('/terms')}
              className="p-0 h-auto text-xs"
            >
              服务条款
            </Button>
          </div>
        </div>
      </motion.div>
    </div>
  )
}

export default LoginScreen 