import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { Eye, EyeOff, User, Mail, Lock, Loader2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { toast } from '@/hooks/use-toast'
import { login, register, LoginRequest, RegisterRequest } from '@/api/auth'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '@/contexts/AuthContext'

const LoginScreen: React.FC = () => {
  const navigate = useNavigate()
  const { login: authLogin } = useAuth()
  const [isLoading, setIsLoading] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  const [showConfirmPassword, setShowConfirmPassword] = useState(false)
  
  // 登录表单状态
  const [loginForm, setLoginForm] = useState<LoginRequest>({
    username: '',
    password: ''
  })
  
  // 注册表单状态
  const [registerForm, setRegisterForm] = useState<RegisterRequest & { confirmPassword: string }>({
    username: '',
    email: '',
    password: '',
    confirmPassword: ''
  })

  // 处理登录
  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!loginForm.username || !loginForm.password) {
      toast({
        title: "请填写完整信息",
        description: "用户名和密码不能为空",
        variant: "destructive"
      })
      return
    }

    setIsLoading(true)
    try {
      const response = await login(loginForm)
      
      // 更新认证上下文
      authLogin(response.user, response.token)
      
      toast({
        title: "登录成功",
        description: `欢迎回来，${response.user.nickname || response.user.username}！`,
      })
      
      // 跳转到首页或之前访问的页面
      const redirectUrl = new URLSearchParams(window.location.search).get('redirect') || '/'
      navigate(redirectUrl)
    } catch (error: any) {
      toast({
        title: "登录失败",
        description: error.message || "用户名或密码错误",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }

  // 处理注册
  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!registerForm.username || !registerForm.email || !registerForm.password) {
      toast({
        title: "请填写完整信息",
        description: "所有字段都不能为空",
        variant: "destructive"
      })
      return
    }

    if (registerForm.password !== registerForm.confirmPassword) {
      toast({
        title: "密码不匹配",
        description: "两次输入的密码不一致",
        variant: "destructive"
      })
      return
    }

    if (registerForm.password.length < 6) {
      toast({
        title: "密码太短",
        description: "密码长度至少6位",
        variant: "destructive"
      })
      return
    }

    setIsLoading(true)
    try {
      const { confirmPassword, ...registerData } = registerForm
      const response = await register(registerData)
      
      // 更新认证上下文
      authLogin(response.user, response.token)
      
      toast({
        title: "注册成功",
        description: `欢迎加入，${response.user.nickname || response.user.username}！`,
      })
      
      // 跳转到首页
      navigate('/')
    } catch (error: any) {
      toast({
        title: "注册失败",
        description: error.message || "注册失败，请重试",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-background to-muted/20 p-4">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
        className="w-full max-w-md"
      >
        <Card className="shadow-lg">
          <CardHeader className="text-center space-y-2">
            <CardTitle className="text-2xl font-bold">结绳社区</CardTitle>
            <CardDescription>
              连接开发者，分享知识与资源
            </CardDescription>
          </CardHeader>
          
          <CardContent>
            <Tabs defaultValue="login" className="w-full">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="login">登录</TabsTrigger>
                <TabsTrigger value="register">注册</TabsTrigger>
              </TabsList>
              
              {/* 登录表单 */}
              <TabsContent value="login" className="space-y-4 mt-6">
                <form onSubmit={handleLogin} className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="login-username">用户名</Label>
                    <div className="relative">
                      <User className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                      <Input
                        id="login-username"
                        type="text"
                        placeholder="请输入用户名"
                        className="pl-10"
                        value={loginForm.username}
                        onChange={(e) => setLoginForm(prev => ({ ...prev, username: e.target.value }))}
                        disabled={isLoading}
                      />
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <Label htmlFor="login-password">密码</Label>
                    <div className="relative">
                      <Lock className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                      <Input
                        id="login-password"
                        type={showPassword ? "text" : "password"}
                        placeholder="请输入密码"
                        className="pl-10 pr-10"
                        value={loginForm.password}
                        onChange={(e) => setLoginForm(prev => ({ ...prev, password: e.target.value }))}
                        disabled={isLoading}
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="icon"
                        className="absolute right-0 top-0 h-full px-3 hover:bg-transparent"
                        onClick={() => setShowPassword(!showPassword)}
                        disabled={isLoading}
                      >
                        {showPassword ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                      </Button>
                    </div>
                  </div>
                  
                  <Button type="submit" className="w-full" disabled={isLoading}>
                    {isLoading ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        登录中...
                      </>
                    ) : (
                      '登录'
                    )}
                  </Button>
                </form>
              </TabsContent>
              
              {/* 注册表单 */}
              <TabsContent value="register" className="space-y-4 mt-6">
                <form onSubmit={handleRegister} className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="register-username">用户名</Label>
                    <div className="relative">
                      <User className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                      <Input
                        id="register-username"
                        type="text"
                        placeholder="请输入用户名"
                        className="pl-10"
                        value={registerForm.username}
                        onChange={(e) => setRegisterForm(prev => ({ ...prev, username: e.target.value }))}
                        disabled={isLoading}
                      />
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <Label htmlFor="register-email">邮箱</Label>
                    <div className="relative">
                      <Mail className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                      <Input
                        id="register-email"
                        type="email"
                        placeholder="请输入邮箱"
                        className="pl-10"
                        value={registerForm.email}
                        onChange={(e) => setRegisterForm(prev => ({ ...prev, email: e.target.value }))}
                        disabled={isLoading}
                      />
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <Label htmlFor="register-password">密码</Label>
                    <div className="relative">
                      <Lock className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                      <Input
                        id="register-password"
                        type={showPassword ? "text" : "password"}
                        placeholder="请输入密码（至少6位）"
                        className="pl-10 pr-10"
                        value={registerForm.password}
                        onChange={(e) => setRegisterForm(prev => ({ ...prev, password: e.target.value }))}
                        disabled={isLoading}
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="icon"
                        className="absolute right-0 top-0 h-full px-3 hover:bg-transparent"
                        onClick={() => setShowPassword(!showPassword)}
                        disabled={isLoading}
                      >
                        {showPassword ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                      </Button>
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <Label htmlFor="register-confirm-password">确认密码</Label>
                    <div className="relative">
                      <Lock className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                      <Input
                        id="register-confirm-password"
                        type={showConfirmPassword ? "text" : "password"}
                        placeholder="请再次输入密码"
                        className="pl-10 pr-10"
                        value={registerForm.confirmPassword}
                        onChange={(e) => setRegisterForm(prev => ({ ...prev, confirmPassword: e.target.value }))}
                        disabled={isLoading}
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="icon"
                        className="absolute right-0 top-0 h-full px-3 hover:bg-transparent"
                        onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                        disabled={isLoading}
                      >
                        {showConfirmPassword ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                      </Button>
                    </div>
                  </div>
                  
                  <Button type="submit" className="w-full" disabled={isLoading}>
                    {isLoading ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        注册中...
                      </>
                    ) : (
                      '注册'
                    )}
                  </Button>
                </form>
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>
      </motion.div>
    </div>
  )
}

export default LoginScreen 