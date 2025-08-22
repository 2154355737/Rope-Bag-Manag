import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { Eye, EyeOff, Mail, User, Lock, ArrowLeft, MessageSquare, CheckCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Separator } from '@/components/ui/separator'
import { Checkbox } from '@/components/ui/checkbox'
import { toast } from '@/hooks/use-toast'
import { register, RegisterRequest, sendRegisterCode } from '@/api/auth'

const RegisterScreen: React.FC = () => {
  const navigate = useNavigate()
  
  // 表单状态
  const [isLoading, setIsLoading] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  const [showConfirmPassword, setShowConfirmPassword] = useState(false)
  const [isRegistered, setIsRegistered] = useState(false)
  
  // 注册表单
  const [registerForm, setRegisterForm] = useState({
    username: '',
    email: '',
    qq: '',
    password: '',
    confirmPassword: '',
    verification_code: '',
    agreeTerms: false,
    agreePrivacy: false
  })
  
  // 验证码相关状态
  const [codeSent, setCodeSent] = useState(false)
  const [countdown, setCountdown] = useState(0)
  const [sendingCode, setSendingCode] = useState(false)
  
  // 表单验证
  const validateForm = () => {
    // 用户名验证
    if (!registerForm.username.trim()) {
      toast({
        title: "验证失败",
        description: "请输入用户名",
        variant: "destructive"
      })
      return false
    }
    if (registerForm.username.length < 3) {
      toast({
        title: "验证失败",
        description: "用户名长度不能少于3位",
        variant: "destructive"
      })
      return false
    }
    if (!/^[a-zA-Z0-9_\u4e00-\u9fa5]+$/.test(registerForm.username)) {
      toast({
        title: "验证失败",
        description: "用户名只能包含字母、数字、下划线和中文",
        variant: "destructive"
      })
      return false
    }
    
    // 邮箱验证
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!registerForm.email.trim()) {
      toast({
        title: "验证失败",
        description: "请输入邮箱地址",
        variant: "destructive"
      })
      return false
    }
    if (!emailRegex.test(registerForm.email)) {
      toast({
        title: "验证失败",
        description: "请输入有效的邮箱地址",
        variant: "destructive"
      })
      return false
    }
    
    // QQ号验证（可选）
    if (registerForm.qq && !/^\d{5,11}$/.test(registerForm.qq)) {
      toast({
        title: "验证失败",
        description: "请输入有效的QQ号码（5-11位数字）",
        variant: "destructive"
      })
      return false
    }
    
    // 密码验证
    if (!registerForm.password) {
      toast({
        title: "验证失败",
        description: "请输入密码",
        variant: "destructive"
      })
      return false
    }
    if (registerForm.password.length < 6) {
      toast({
        title: "验证失败",
        description: "密码长度不能少于6位",
        variant: "destructive"
      })
      return false
    }
    if (!/(?=.*[a-zA-Z])(?=.*\d)/.test(registerForm.password)) {
      toast({
        title: "验证失败",
        description: "密码必须包含字母和数字",
        variant: "destructive"
      })
      return false
    }
    
    // 确认密码验证
    if (registerForm.password !== registerForm.confirmPassword) {
      toast({
        title: "验证失败",
        description: "两次输入的密码不一致",
        variant: "destructive"
      })
      return false
    }
    
    // 协议同意验证
    if (!registerForm.agreeTerms) {
      toast({
        title: "验证失败",
        description: "请同意服务条款",
        variant: "destructive"
      })
      return false
    }
    if (!registerForm.agreePrivacy) {
      toast({
        title: "验证失败",
        description: "请同意隐私政策",
        variant: "destructive"
      })
      return false
    }
    
    // 验证码验证
    if (!registerForm.verification_code.trim()) {
      toast({
        title: "验证失败",
        description: "请输入邮箱验证码",
        variant: "destructive"
      })
      return false
    }
    
    if (registerForm.verification_code.length !== 6) {
      toast({
        title: "验证失败",
        description: "请输入6位验证码",
        variant: "destructive"
      })
      return false
    }
    
    return true
  }
  
  // 发送验证码
  const handleSendCode = async () => {
    if (!registerForm.email.trim()) {
      toast({
        title: "请先输入邮箱",
        description: "需要邮箱地址才能发送验证码",
        variant: "destructive"
      })
      return
    }
    
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!emailRegex.test(registerForm.email)) {
      toast({
        title: "邮箱格式错误",
        description: "请输入正确的邮箱地址",
        variant: "destructive"
      })
      return
    }
    
    setSendingCode(true)
    try {
      await sendRegisterCode(registerForm.email)
      setCodeSent(true)
      setCountdown(60)
      
      // 开始倒计时
      const timer = setInterval(() => {
        setCountdown(prev => {
          if (prev <= 1) {
            clearInterval(timer)
            return 0
          }
          return prev - 1
        })
      }, 1000)
      
      toast({
        title: "验证码已发送",
        description: "请查收邮箱中的验证码",
        variant: "default"
      })
    } catch (error: any) {
      console.error('发送验证码失败:', error)
      toast({
        title: "发送失败",
        description: error.message || "验证码发送失败，请重试",
        variant: "destructive"
      })
    } finally {
      setSendingCode(false)
    }
  }
  
  // 注册处理
  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!validateForm()) return
    
    setIsLoading(true)
    try {
      // 调用真实的注册API
      const registerData: RegisterRequest = {
        username: registerForm.username,
        email: registerForm.email,
        password: registerForm.password,
        verification_code: registerForm.verification_code,
        nickname: registerForm.username, // 默认昵称为用户名
        qq_number: registerForm.qq || undefined
      }
      
      const response = await register(registerData)
      
      setIsRegistered(true)
      toast({
        title: "注册成功",
        description: `欢迎加入结绳社区，${response.user.username}！`,
        variant: "default"
      })
      
      // 注册成功后，延迟跳转到首页
      setTimeout(() => {
        navigate('/')
      }, 2000)
    } catch (error: any) {
      console.error('注册失败:', error)
      toast({
        title: "注册失败",
        description: error.message || "用户名或邮箱已存在，请重试",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }
  
  // 获取密码强度
  const getPasswordStrength = (password: string) => {
    if (!password) return { level: 0, text: '', color: '' }
    
    let score = 0
    if (password.length >= 6) score += 1
    if (password.length >= 8) score += 1
    if (/(?=.*[a-z])/.test(password)) score += 1
    if (/(?=.*[A-Z])/.test(password)) score += 1
    if (/(?=.*\d)/.test(password)) score += 1
    if (/(?=.*[!@#$%^&*])/.test(password)) score += 1
    
    if (score <= 2) return { level: 1, text: '弱', color: 'text-red-500' }
    if (score <= 4) return { level: 2, text: '中', color: 'text-yellow-500' }
    return { level: 3, text: '强', color: 'text-green-500' }
  }
  
  const passwordStrength = getPasswordStrength(registerForm.password)
  
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
              {isRegistered ? (
                <CheckCircle size={32} className="text-green-600" />
              ) : (
                <span className="text-3xl">🪢</span>
              )}
            </div>
            <CardTitle className="text-2xl">
              {isRegistered ? "注册成功" : "创建账号"}
            </CardTitle>
            <CardDescription>
              {isRegistered 
                ? "欢迎加入结绳社区大家庭！"
                : "加入结绳社区，开启你的学习之旅"
              }
            </CardDescription>
          </CardHeader>
          
          <CardContent>
            {!isRegistered ? (
              <form onSubmit={handleRegister} className="space-y-4">
                {/* 用户名 */}
                <div className="space-y-2">
                  <Label htmlFor="username">用户名 *</Label>
                  <div className="relative">
                    <User size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                    <Input
                      id="username"
                      type="text"
                      placeholder="请输入用户名（3-20位）"
                      value={registerForm.username}
                      onChange={(e) => setRegisterForm(prev => ({ ...prev, username: e.target.value }))}
                      className="pl-10"
                      disabled={isLoading}
                      maxLength={20}
                    />
                  </div>
                </div>
                
                {/* 邮箱 */}
                <div className="space-y-2">
                  <Label htmlFor="email">邮箱地址 *</Label>
                  <div className="relative">
                    <Mail size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                    <Input
                      id="email"
                      type="email"
                      placeholder="请输入邮箱地址"
                      value={registerForm.email}
                      onChange={(e) => setRegisterForm(prev => ({ ...prev, email: e.target.value }))}
                      className="pl-10"
                      disabled={isLoading}
                    />
                  </div>
                </div>
                
                {/* 邮箱验证码 */}
                <div className="space-y-2">
                  <Label htmlFor="verification_code">邮箱验证码 *</Label>
                  <div className="flex gap-2">
                    <div className="relative flex-1">
                      <Input
                        id="verification_code"
                        type="text"
                        placeholder="请输入6位验证码"
                        value={registerForm.verification_code}
                        onChange={(e) => setRegisterForm(prev => ({ ...prev, verification_code: e.target.value }))}
                        disabled={isLoading}
                        maxLength={6}
                      />
                    </div>
                    <Button
                      type="button"
                      variant="outline"
                      onClick={handleSendCode}
                      disabled={sendingCode || countdown > 0 || isLoading}
                      className="whitespace-nowrap"
                    >
                      {sendingCode 
                        ? "发送中..." 
                        : countdown > 0 
                        ? `${countdown}s`
                        : codeSent 
                        ? "重新发送" 
                        : "发送验证码"
                      }
                    </Button>
                  </div>
                  {codeSent && (
                    <p className="text-sm text-muted-foreground">
                      验证码已发送至您的邮箱，请查收
                    </p>
                  )}
                </div>
                
                {/* QQ号（可选） */}
                <div className="space-y-2">
                  <Label htmlFor="qq">QQ账号（可选）</Label>
                  <div className="relative">
                    <MessageSquare size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                    <Input
                      id="qq"
                      type="text"
                      placeholder="请输入QQ号码"
                      value={registerForm.qq}
                      onChange={(e) => setRegisterForm(prev => ({ ...prev, qq: e.target.value }))}
                      className="pl-10"
                      disabled={isLoading}
                      maxLength={11}
                    />
                  </div>
                </div>
                
                {/* 密码 */}
                <div className="space-y-2">
                  <Label htmlFor="password">密码 *</Label>
                  <div className="relative">
                    <Lock size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                    <Input
                      id="password"
                      type={showPassword ? "text" : "password"}
                      placeholder="请输入密码（至少6位，包含字母和数字）"
                      value={registerForm.password}
                      onChange={(e) => setRegisterForm(prev => ({ ...prev, password: e.target.value }))}
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
                  {registerForm.password && (
                    <div className="flex items-center space-x-2 text-xs">
                      <span>密码强度：</span>
                      <span className={`font-medium ${passwordStrength.color}`}>
                        {passwordStrength.text}
                      </span>
                      <div className="flex space-x-1 ml-2">
                        {[1, 2, 3].map((level) => (
                          <div
                            key={level}
                            className={`h-1 w-6 rounded-full ${
                              level <= passwordStrength.level 
                                ? passwordStrength.level === 1 
                                  ? 'bg-red-500' 
                                  : passwordStrength.level === 2 
                                  ? 'bg-yellow-500' 
                                  : 'bg-green-500'
                                : 'bg-gray-200'
                            }`}
                          />
                        ))}
                      </div>
                    </div>
                  )}
                </div>
                
                {/* 确认密码 */}
                <div className="space-y-2">
                  <Label htmlFor="confirmPassword">确认密码 *</Label>
                  <div className="relative">
                    <Lock size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                    <Input
                      id="confirmPassword"
                      type={showConfirmPassword ? "text" : "password"}
                      placeholder="请再次输入密码"
                      value={registerForm.confirmPassword}
                      onChange={(e) => setRegisterForm(prev => ({ ...prev, confirmPassword: e.target.value }))}
                      className="pl-10 pr-10"
                      disabled={isLoading}
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                      className="absolute right-0 top-0 h-full px-3 hover:bg-transparent"
                      disabled={isLoading}
                    >
                      {showConfirmPassword ? <EyeOff size={18} /> : <Eye size={18} />}
                    </Button>
                  </div>
                  {registerForm.confirmPassword && registerForm.password !== registerForm.confirmPassword && (
                    <p className="text-xs text-red-500">两次输入的密码不一致</p>
                  )}
                </div>
                
                {/* 协议同意 */}
                <div className="space-y-3">
                  <div className="flex items-start space-x-2">
                    <Checkbox
                      id="agreeTerms"
                      checked={registerForm.agreeTerms}
                      onCheckedChange={(checked) => setRegisterForm(prev => ({ ...prev, agreeTerms: checked as boolean }))}
                      disabled={isLoading}
                    />
                    <div className="text-sm">
                      <label htmlFor="agreeTerms" className="cursor-pointer">
                        我已阅读并同意
                        <Button
                          type="button"
                          variant="link"
                          size="sm"
                          onClick={() => navigate('/terms')}
                          className="p-0 h-auto mx-1 text-primary"
                        >
                          《服务条款》
                        </Button>
                      </label>
                    </div>
                  </div>
                  
                  <div className="flex items-start space-x-2">
                    <Checkbox
                      id="agreePrivacy"
                      checked={registerForm.agreePrivacy}
                      onCheckedChange={(checked) => setRegisterForm(prev => ({ ...prev, agreePrivacy: checked as boolean }))}
                      disabled={isLoading}
                    />
                    <div className="text-sm">
                      <label htmlFor="agreePrivacy" className="cursor-pointer">
                        我已阅读并同意
                        <Button
                          type="button"
                          variant="link"
                          size="sm"
                          onClick={() => navigate('/privacy')}
                          className="p-0 h-auto mx-1 text-primary"
                        >
                          《隐私政策》
                        </Button>
                      </label>
                    </div>
                  </div>
                </div>
                
                <Button
                  type="submit"
                  className="w-full"
                  disabled={isLoading}
                >
                  {isLoading ? "注册中..." : "创建账号"}
                </Button>
              </form>
            ) : (
              <div className="space-y-4 text-center">
                <div className="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
                  <p className="text-green-800 dark:text-green-400 mb-2">
                    🎉 注册成功！
                  </p>
                  <p className="text-sm text-green-700 dark:text-green-300">
                    您的账号已创建完成，现在可以开始使用结绳社区了
                  </p>
                </div>
                
                <div className="flex flex-col space-y-2">
                  <Button
                    onClick={() => navigate('/login')}
                    className="w-full"
                  >
                    立即登录
                  </Button>
                  
                  <Button
                    onClick={() => navigate('/')}
                    variant="outline"
                  >
                    浏览社区
                  </Button>
                </div>
              </div>
            )}
            
            {!isRegistered && (
              <>
                <Separator className="my-6" />
                
                <div className="text-center">
                  <span className="text-sm text-muted-foreground">已有账号？</span>
                  <Button
                    variant="link"
                    size="sm"
                    onClick={() => navigate('/login')}
                    className="p-0 h-auto ml-1"
                    disabled={isLoading}
                  >
                    立即登录
                  </Button>
                </div>
              </>
            )}
          </CardContent>
        </Card>
        
        {!isRegistered && (
          <div className="mt-6 text-center text-xs text-muted-foreground">
            <p>注册即表示您同意我们的服务条款和隐私政策</p>
          </div>
        )}
      </motion.div>
    </div>
  )
}

export default RegisterScreen 