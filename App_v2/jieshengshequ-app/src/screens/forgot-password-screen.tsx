import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { ArrowLeft, Mail, Send, CheckCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { toast } from '@/hooks/use-toast'

const ForgotPasswordScreen: React.FC = () => {
  const navigate = useNavigate()
  
  const [email, setEmail] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [isEmailSent, setIsEmailSent] = useState(false)
  
  // 验证邮箱格式
  const validateEmail = () => {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!email.trim()) {
      toast({
        title: "验证失败",
        description: "请输入邮箱地址",
        variant: "destructive"
      })
      return false
    }
    if (!emailRegex.test(email)) {
      toast({
        title: "验证失败",
        description: "请输入有效的邮箱地址",
        variant: "destructive"
      })
      return false
    }
    return true
  }
  
  // 发送重置邮件
  const handleSendResetEmail = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!validateEmail()) return
    
    setIsLoading(true)
    try {
      // 模拟发送邮件API调用
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      setIsEmailSent(true)
      toast({
        title: "邮件发送成功",
        description: "重置密码的邮件已发送到您的邮箱",
        variant: "default"
      })
    } catch (error) {
      toast({
        title: "发送失败",
        description: "邮件发送失败，请稍后重试",
        variant: "destructive"
      })
    } finally {
      setIsLoading(false)
    }
  }
  
  // 重新发送邮件
  const handleResendEmail = async () => {
    setIsLoading(true)
    try {
      await new Promise(resolve => setTimeout(resolve, 1500))
      toast({
        title: "邮件重新发送成功",
        description: "请检查您的邮箱",
        variant: "default"
      })
    } catch (error) {
      toast({
        title: "发送失败",
        description: "邮件发送失败，请稍后重试",
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
          返回登录
        </Button>
        
        <Card className="shadow-lg">
          <CardHeader className="text-center">
            <div className="mx-auto mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-primary/10">
              {isEmailSent ? (
                <CheckCircle size={32} className="text-green-600" />
              ) : (
                <Mail size={32} className="text-primary" />
              )}
            </div>
            <CardTitle className="text-2xl">
              {isEmailSent ? "邮件已发送" : "忘记密码"}
            </CardTitle>
            <CardDescription>
              {isEmailSent 
                ? "我们已向您的邮箱发送了重置密码的链接"
                : "输入您的邮箱地址，我们将发送重置密码的链接给您"
              }
            </CardDescription>
          </CardHeader>
          
          <CardContent>
            {!isEmailSent ? (
              <form onSubmit={handleSendResetEmail} className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="email">邮箱地址</Label>
                  <div className="relative">
                    <Mail size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground" />
                    <Input
                      id="email"
                      type="email"
                      placeholder="请输入您的邮箱地址"
                      value={email}
                      onChange={(e) => setEmail(e.target.value)}
                      className="pl-10"
                      disabled={isLoading}
                    />
                  </div>
                </div>
                
                <Button
                  type="submit"
                  className="w-full"
                  disabled={isLoading}
                >
                  {isLoading ? (
                    <>
                      <Send size={16} className="mr-2 animate-pulse" />
                      发送中...
                    </>
                  ) : (
                    <>
                      <Send size={16} className="mr-2" />
                      发送重置邮件
                    </>
                  )}
                </Button>
              </form>
            ) : (
              <div className="space-y-4">
                <div className="text-center p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
                  <p className="text-sm text-green-800 dark:text-green-400 mb-2">
                    重置邮件已发送至：
                  </p>
                  <p className="font-medium text-green-900 dark:text-green-300">
                    {email}
                  </p>
                </div>
                
                <div className="text-sm text-muted-foreground space-y-2">
                  <p>📧 请检查您的邮箱（包括垃圾邮件文件夹）</p>
                  <p>🔗 点击邮件中的链接来重置您的密码</p>
                  <p>⏰ 链接将在24小时后过期</p>
                </div>
                
                <div className="flex flex-col space-y-2">
                  <Button
                    onClick={handleResendEmail}
                    variant="outline"
                    disabled={isLoading}
                  >
                    {isLoading ? "重新发送中..." : "重新发送邮件"}
                  </Button>
                  
                  <Button
                    onClick={() => navigate('/login')}
                    variant="ghost"
                  >
                    返回登录页面
                  </Button>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
        
        <div className="mt-6 text-center text-xs text-muted-foreground">
          <p>遇到问题？</p>
          <div className="flex justify-center space-x-4 mt-1">
            <Button
              variant="link"
              size="sm"
              onClick={() => navigate('/help')}
              className="p-0 h-auto text-xs"
            >
              联系客服
            </Button>
            <span>或</span>
            <Button
              variant="link"
              size="sm"
              onClick={() => navigate('/register')}
              className="p-0 h-auto text-xs"
            >
              注册新账号
            </Button>
          </div>
        </div>
      </motion.div>
    </div>
  )
}

export default ForgotPasswordScreen 