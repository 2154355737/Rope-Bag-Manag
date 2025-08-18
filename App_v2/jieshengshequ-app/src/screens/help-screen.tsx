import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { ArrowLeft, Search, MessageSquare, Phone, Mail, ExternalLink, ChevronDown, ChevronRight, HelpCircle, BookOpen, Settings, Bug, Lightbulb, Users, Clock } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@/components/ui/accordion'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { toast } from '@/hooks/use-toast'

const HelpScreen: React.FC = () => {
  const navigate = useNavigate()
  const [searchQuery, setSearchQuery] = useState('')

  // 常见问题
  const faqs = [
    {
      id: '1',
      category: '基础使用',
      question: '如何注册和登录账户？',
      answer: '您可以通过手机号或邮箱注册账户。首次打开应用时，点击"立即注册"按钮，按照提示填写信息即可。已有账户的用户可直接点击"登录"进入。'
    },
    {
      id: '2',
      category: '基础使用',
      question: '忘记密码怎么办？',
      answer: '在登录页面点击"忘记密码"，输入您的手机号或邮箱，系统会发送重置密码的验证码。按照提示操作即可重新设置密码。'
    },
    {
      id: '3',
      category: '发布内容',
      question: '如何发布帖子和资源？',
      answer: '点击底部导航栏的"发布"按钮，选择要发布的内容类型（帖子或资源）。填写标题、内容、添加标签等信息后，点击"发布"即可。发布的内容需要经过审核。'
    },
    {
      id: '4',
      category: '发布内容',
      question: '为什么我的内容没有显示？',
      answer: '新发布的内容需要经过管理员审核，通常在1-24小时内完成。审核通过后会自动显示在相应页面。如果超过24小时仍未显示，请联系客服。'
    },
    {
      id: '5',
      category: '个人设置',
      question: '如何修改个人资料？',
      answer: '进入"个人中心"页面，点击头像旁边的"编辑"按钮，可以修改用户名、个人简介、头像等信息。修改后记得点击"保存"按钮。'
    },
    {
      id: '6',
      category: '个人设置',
      question: '如何调整界面显示？',
      answer: '在个人中心点击右上角设置图标，进入设置页面。您可以切换深色/浅色主题，调整安全区域边距等。这些设置会自动保存。'
    },
    {
      id: '7',
      category: '技术问题',
      question: '应用卡顿或崩溃怎么办？',
      answer: '首先尝试关闭应用重新打开。如果问题持续，可以尝试：1)清理应用缓存 2)重启设备 3)更新到最新版本。如仍有问题请联系技术支持。'
    },
    {
      id: '8',
      category: '技术问题',
      question: '网络连接问题如何解决？',
      answer: '请检查您的网络连接是否正常。如果网络正常但仍无法使用，可能是服务器维护，请稍后再试。持续问题请联系客服。'
    }
  ]

  // 快速操作
  const quickActions = [
    {
      icon: MessageSquare,
      title: '在线客服',
      description: '与客服人员实时对话',
      action: () => {
        toast({
          title: "客服功能",
          description: "在线客服功能开发中，敬请期待！",
          duration: 3000,
        })
      }
    },
    {
      icon: Bug,
      title: '反馈问题',
      description: '报告bug或提出建议',
      action: () => {
        toast({
          title: "问题反馈",
          description: "问题反馈功能开发中，敬请期待！",
          duration: 3000,
        })
      }
    },
    {
      icon: BookOpen,
      title: '使用教程',
      description: '查看详细使用指南',
      action: () => {
        toast({
          title: "使用教程",
          description: "使用教程功能开发中，敬请期待！",
          duration: 3000,
        })
      }
    },
    {
      icon: Users,
      title: '社区论坛',
      description: '与其他用户交流讨论',
      action: () => {
        toast({
          title: "社区论坛",
          description: "社区论坛功能开发中，敬请期待！",
          duration: 3000,
        })
      }
    }
  ]

  // 联系方式
  const contactMethods = [
    {
      icon: Mail,
      title: '邮箱支持',
      value: 'support@jieshengshequ.com',
      description: '工作日24小时内回复'
    },
    {
      icon: MessageSquare,
      title: '客服QQ',
      value: '2154355737',
      description: '在线客服咨询'
    },
    {
      icon: MessageSquare,
      title: 'QQ群',
      value: '616300355',
      description: '用户交流群'
    }
  ]

  // 筛选常见问题
  const filteredFaqs = faqs.filter(faq => 
    searchQuery === '' || 
    faq.question.toLowerCase().includes(searchQuery.toLowerCase()) ||
    faq.answer.toLowerCase().includes(searchQuery.toLowerCase()) ||
    faq.category.toLowerCase().includes(searchQuery.toLowerCase())
  )

  // 按分类分组
  const faqsByCategory = filteredFaqs.reduce((acc, faq) => {
    if (!acc[faq.category]) {
      acc[faq.category] = []
    }
    acc[faq.category].push(faq)
    return acc
  }, {} as Record<string, typeof faqs>)

  return (
    <div className="min-h-screen bg-background">
      {/* 头部导航 */}
      <div className="sticky top-0 z-50 bg-background/80 backdrop-blur-sm border-b">
        <div className="flex items-center justify-between p-4">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => navigate(-1)}
          >
            <ArrowLeft className="h-5 w-5" />
          </Button>
          <h1 className="text-lg font-semibold">帮助与支持</h1>
          <div className="w-10" />
        </div>
      </div>

      <div className="container max-w-2xl mx-auto p-4 space-y-6">
        {/* 搜索框 */}
        <Card>
          <CardContent className="p-4">
            <div className="relative">
              <Search className="absolute left-3 top-3 text-muted-foreground" size={18} />
              <Input
                placeholder="搜索帮助内容..."
                className="pl-10"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
              />
            </div>
          </CardContent>
        </Card>

        {/* 快速操作 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Lightbulb className="h-4 w-4 mr-2" />
              快速操作
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 gap-3">
              {quickActions.map((action, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: index * 0.1 }}
                >
                  <Button
                    variant="outline"
                    className="h-auto p-4 flex flex-col items-center space-y-2 w-full"
                    onClick={action.action}
                  >
                    <action.icon className="h-6 w-6 text-primary" />
                    <div className="text-center">
                      <div className="font-medium text-sm">{action.title}</div>
                      <div className="text-xs text-muted-foreground">{action.description}</div>
                    </div>
                  </Button>
                </motion.div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* 常见问题 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <HelpCircle className="h-4 w-4 mr-2" />
              常见问题
            </CardTitle>
          </CardHeader>
          <CardContent>
            {Object.keys(faqsByCategory).length > 0 ? (
              <div className="space-y-4">
                {Object.entries(faqsByCategory).map(([category, categoryFaqs]) => (
                  <div key={category}>
                    <div className="flex items-center mb-3">
                      <Badge variant="secondary" className="text-xs">
                        {category}
                      </Badge>
                      <span className="text-xs text-muted-foreground ml-2">
                        {categoryFaqs.length} 个问题
                      </span>
                    </div>
                    <Accordion type="single" collapsible className="w-full">
                      {categoryFaqs.map((faq) => (
                        <AccordionItem key={faq.id} value={faq.id}>
                          <AccordionTrigger className="text-left text-sm hover:no-underline">
                            {faq.question}
                          </AccordionTrigger>
                          <AccordionContent className="text-sm text-muted-foreground">
                            {faq.answer}
                          </AccordionContent>
                        </AccordionItem>
                      ))}
                    </Accordion>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-center py-8 text-muted-foreground">
                <Search className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <p>没有找到相关问题</p>
                <p className="text-sm">尝试使用其他关键词搜索</p>
              </div>
            )}
          </CardContent>
        </Card>

        {/* 联系我们 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Phone className="h-4 w-4 mr-2" />
              联系我们
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {contactMethods.map((method, index) => (
              <div key={index} className="flex items-center justify-between p-3 rounded-lg border">
                <div className="flex items-center space-x-3">
                  <method.icon className="h-5 w-5 text-primary" />
                  <div>
                    <div className="font-medium text-sm">{method.title}</div>
                    <div className="text-xs text-muted-foreground">{method.description}</div>
                  </div>
                </div>
                <div className="text-right">
                  <div className="font-mono text-sm">{method.value}</div>
                  <Button
                    variant="ghost"
                    size="sm"
                    className="h-auto p-0 text-xs text-primary"
                    onClick={() => {
                      navigator.clipboard.writeText(method.value)
                      toast({
                        title: "已复制",
                        description: `${method.title}已复制到剪贴板`,
                        duration: 2000,
                      })
                    }}
                  >
                    复制
                  </Button>
                </div>
              </div>
            ))}
          </CardContent>
        </Card>

        {/* 服务时间 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Clock className="h-4 w-4 mr-2" />
              服务时间
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex justify-between items-center">
              <span className="text-sm">在线客服</span>
              <span className="text-sm text-muted-foreground">24小时在线</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-sm">电话客服</span>
              <span className="text-sm text-muted-foreground">工作日 9:00-18:00</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-sm">邮箱回复</span>
              <span className="text-sm text-muted-foreground">工作日24小时内</span>
            </div>
            <Separator />
            <div className="text-xs text-muted-foreground text-center">
              我们致力于为您提供最好的服务体验
            </div>
          </CardContent>
        </Card>

        {/* 底部安全区域 */}
        <div className="h-8" />
      </div>
    </div>
  )
}

export default HelpScreen 