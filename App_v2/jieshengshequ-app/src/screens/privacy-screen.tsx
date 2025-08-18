import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { ArrowLeft, Shield, Eye, Lock, Database, UserCheck, Globe, AlertTriangle, Calendar, Mail, MessageSquare } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@/components/ui/accordion'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { Alert, AlertDescription } from '@/components/ui/alert'

const PrivacyScreen: React.FC = () => {
  const navigate = useNavigate()
  const [expandedSection, setExpandedSection] = useState<string>('')

  // 隐私政策章节
  const privacySections = [
    {
      id: 'overview',
      title: '隐私政策概述',
      icon: Shield,
      content: `
        结绳社区（以下简称"我们"）非常重视用户的隐私保护。本隐私政策说明了我们如何收集、使用、存储和保护您的个人信息。
        
        本政策适用于结绳社区移动应用及相关服务。使用我们的服务即表示您同意本隐私政策的条款。
        
                 最后更新日期：2025年8月19日
         生效日期：2025年8月19日
      `
    },
    {
      id: 'collection',
      title: '信息收集',
      icon: Database,
      content: `
        我们可能收集以下类型的信息：
        
        **个人身份信息：**
        • 用户名、邮箱地址
        • 头像图片（如果您选择上传）
        • 个人简介和偏好设置
        
        **使用信息：**
        • 应用使用统计数据
        • 设备信息（设备类型、操作系统版本）
        • 日志信息（访问时间、功能使用记录）
        
        **内容信息：**
        • 您发布的帖子、评论和资源
        • 上传的图片和文件
        • 互动记录（点赞、收藏、关注）
      `
    },
    {
      id: 'usage',
      title: '信息使用',
      icon: Eye,
      content: `
        我们使用收集的信息用于：
        
        **服务提供：**
        • 提供和维护应用功能
        • 个性化用户体验
        • 处理用户请求和支持
        
        **安全保障：**
        • 防止欺诈和滥用行为
        • 维护平台安全和稳定
        • 遵守法律法规要求
        
        **改进服务：**
        • 分析使用模式和趋势
        • 开发新功能和改进现有功能
        • 优化应用性能
        
        **通信联系：**
        • 发送重要通知和更新
        • 回复用户咨询和反馈
        • 提供客户支持服务
      `
    },
    {
      id: 'storage',
      title: '信息存储',
      icon: Lock,
      content: `
        **存储位置：**
        • 数据主要存储在中国境内的服务器
        • 使用业界标准的安全措施保护数据
        • 定期备份以确保数据安全
        
        **存储期限：**
        • 账户信息：账户存续期间
        • 发布内容：永久保存（除非用户删除）
        • 使用日志：最多保存12个月
        • 设备信息：最多保存6个月
        
        **数据安全：**
        • 采用加密技术保护数据传输
        • 实施访问控制和权限管理
        • 定期进行安全审计和漏洞检测
      `
    },
    {
      id: 'sharing',
      title: '信息共享',
      icon: Globe,
      content: `
        我们承诺不会出售、租赁或交易您的个人信息。在以下情况下，我们可能会共享您的信息：
        
        **公开内容：**
        • 您主动发布的帖子、评论等内容
        • 公开的个人资料信息
        • 社区互动记录
        
        **法律要求：**
        • 遵守适用的法律法规
        • 响应政府部门的合法请求
        • 保护我们的权利和财产
        
        **服务提供商：**
        • 云存储和服务器托管商
        • 数据分析和统计服务商
        • 技术支持和维护服务商
        
        **业务转让：**
        • 在合并、收购或资产转让情况下
        • 会提前通知用户并征得同意
      `
    },
    {
      id: 'rights',
      title: '用户权利',
      icon: UserCheck,
      content: `
        您对个人信息享有以下权利：
        
        **访问权：**
        • 查看我们收集的您的个人信息
        • 了解信息的使用目的和方式
        • 获取信息处理的详细记录
        
        **更正权：**
        • 更新和修改个人资料信息
        • 纠正不准确或过时的信息
        • 完善缺失的个人信息
        
        **删除权：**
        • 删除不再需要的个人信息
        • 撤销对信息处理的同意
        • 注销账户和删除相关数据
        
        **限制处理权：**
        • 限制特定信息的处理
        • 暂停自动化决策过程
        • 选择退出数据分析
        
        **数据可携权：**
        • 导出个人数据副本
        • 以结构化格式获取数据
        • 将数据转移到其他服务
      `
    },
    {
      id: 'cookies',
      title: 'Cookie和追踪技术',
      icon: Eye,
      content: `
        我们使用以下技术改善用户体验：
        
        **必要Cookie：**
        • 用户身份验证和会话管理
        • 安全防护和欺诈检测
        • 基本功能的正常运行
        
        **功能Cookie：**
        • 记住用户偏好和设置
        • 个性化内容推荐
        • 语言和主题选择
        
        **分析Cookie：**
        • 收集使用统计信息
        • 分析用户行为模式
        • 优化应用性能
        
        **管理选择：**
        • 您可以在设置中管理Cookie偏好
        • 禁用非必要Cookie不会影响基本功能
        • 清除浏览器Cookie可重置设置
      `
    },
    {
      id: 'minors',
      title: '未成年人保护',
      icon: Shield,
      content: `
        我们非常重视未成年人的隐私保护：
        
        **年龄限制：**
        • 本服务主要面向18岁以上用户
        • 13-18岁用户需要监护人同意
        • 13岁以下用户不得使用本服务
        
        **特殊保护：**
        • 限制未成年人信息的收集和使用
        • 不会向未成年人展示不适宜内容
        • 提供家长监护和控制功能
        
        **监护人权利：**
        • 查看未成年人的账户信息
        • 请求删除未成年人的个人信息
        • 撤销对信息处理的同意
        
        **举报机制：**
        • 发现未成年人不当使用可举报
        • 及时处理涉及未成年人的投诉
        • 配合相关部门的调查处理
      `
    },
    {
      id: 'updates',
      title: '政策更新',
      icon: Calendar,
      content: `
        **更新通知：**
        • 重大变更会提前30天通知用户
        • 通过应用内通知、邮件等方式告知
        • 在应用显著位置公布更新内容
        
        **用户选择：**
        • 不同意更新可以停止使用服务
        • 继续使用视为接受新政策
        • 可以联系我们了解变更详情
        
        **版本记录：**
        • 保留历史版本供用户查阅
        • 记录每次更新的主要变更
        • 提供变更对比和说明
      `
    }
  ]

  // 联系信息
  const contactInfo = [
    {
      icon: Mail,
      title: '隐私邮箱',
      value: '2154355737@qq.com',
      description: '隐私相关问题和投诉'
    },
    {
      icon: MessageSquare,
      title: '客服QQ',
      value: '2154355737',
      description: '在线客服咨询'
    }
  ]

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
          <h1 className="text-lg font-semibold">隐私政策</h1>
          <div className="w-10" />
        </div>
      </div>

      <div className="container max-w-2xl mx-auto p-4 space-y-6">
        {/* 政策概要 */}
        <Card>
          <CardContent className="p-6">
            <div className="text-center mb-4">
              <Shield className="h-12 w-12 text-primary mx-auto mb-3" />
              <h2 className="text-xl font-bold mb-2">隐私政策</h2>
                             <Badge variant="outline" className="mb-3">
                 最后更新：2025年8月19日
               </Badge>
            </div>
            <Alert>
              <AlertTriangle className="h-4 w-4" />
              <AlertDescription>
                <strong>重要提示：</strong>
                请仔细阅读本隐私政策。使用我们的服务即表示您同意我们按照本政策收集、使用和处理您的个人信息。
                如有疑问，请随时联系我们。
              </AlertDescription>
            </Alert>
          </CardContent>
        </Card>

        {/* 政策内容 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">政策详情</CardTitle>
          </CardHeader>
          <CardContent>
            <Accordion type="single" collapsible className="w-full">
              {privacySections.map((section, index) => (
                <AccordionItem key={section.id} value={section.id}>
                  <AccordionTrigger className="text-left hover:no-underline">
                    <div className="flex items-center">
                      <section.icon className="h-4 w-4 mr-2 text-primary" />
                      <span className="text-sm font-medium">{section.title}</span>
                    </div>
                  </AccordionTrigger>
                  <AccordionContent>
                    <motion.div
                      initial={{ opacity: 0, y: 10 }}
                      animate={{ opacity: 1, y: 0 }}
                      transition={{ duration: 0.3 }}
                      className="pl-6"
                    >
                      <div className="text-sm text-muted-foreground whitespace-pre-line leading-relaxed">
                        {section.content}
                      </div>
                    </motion.div>
                  </AccordionContent>
                </AccordionItem>
              ))}
            </Accordion>
          </CardContent>
        </Card>

        {/* 您的权利 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <UserCheck className="h-4 w-4 mr-2" />
              行使您的权利
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="text-sm text-muted-foreground">
              如需行使您的隐私权利，您可以：
            </div>
            <div className="grid grid-cols-1 gap-3">
              <div className="flex items-center p-3 rounded-lg border">
                <div className="flex-1">
                  <div className="font-medium text-sm">在应用中管理</div>
                  <div className="text-xs text-muted-foreground">通过设置页面管理个人信息和隐私选项</div>
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => navigate('/settings')}
                >
                  前往设置
                </Button>
              </div>
              <div className="flex items-center p-3 rounded-lg border">
                <div className="flex-1">
                  <div className="font-medium text-sm">联系我们</div>
                  <div className="text-xs text-muted-foreground">通过邮件或电话联系我们的隐私专员</div>
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setExpandedSection('contact')}
                >
                  查看联系方式
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 联系我们 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Mail className="h-4 w-4 mr-2" />
              隐私问题联系
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="text-sm text-muted-foreground">
              如果您对本隐私政策有任何疑问，或需要行使您的隐私权利，请通过以下方式联系我们：
            </div>
            {contactInfo.map((contact, index) => (
              <div key={index} className="flex items-center justify-between p-3 rounded-lg border">
                <div className="flex items-center space-x-3">
                  <contact.icon className="h-5 w-5 text-primary" />
                  <div>
                    <div className="font-medium text-sm">{contact.title}</div>
                    <div className="text-xs text-muted-foreground">{contact.description}</div>
                  </div>
                </div>
                <div className="text-right">
                  <div className="font-mono text-sm">{contact.value}</div>
                </div>
              </div>
            ))}
            
            <Separator />
            
            <div className="text-xs text-muted-foreground">
              <strong>响应时间：</strong>我们会在收到您的请求后30天内给予回复。对于复杂请求，我们可能需要更长时间，但会及时告知您处理进度。
            </div>
          </CardContent>
        </Card>

        {/* 法律依据 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Shield className="h-4 w-4 mr-2" />
              法律依据
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="text-sm text-muted-foreground">
              我们处理您的个人信息基于以下法律依据：
            </div>
            <div className="space-y-2">
              <div className="flex items-start p-2 rounded border">
                <div className="text-primary mr-2 mt-1">•</div>
                <div>
                  <div className="font-medium text-sm">用户同意</div>
                  <div className="text-xs text-muted-foreground">您明确同意我们处理您的个人信息</div>
                </div>
              </div>
              <div className="flex items-start p-2 rounded border">
                <div className="text-primary mr-2 mt-1">•</div>
                <div>
                  <div className="font-medium text-sm">履行合同</div>
                  <div className="text-xs text-muted-foreground">为您提供服务所必需的信息处理</div>
                </div>
              </div>
              <div className="flex items-start p-2 rounded border">
                <div className="text-primary mr-2 mt-1">•</div>
                <div>
                  <div className="font-medium text-sm">法律义务</div>
                  <div className="text-xs text-muted-foreground">遵守适用法律法规的要求</div>
                </div>
              </div>
              <div className="flex items-start p-2 rounded border">
                <div className="text-primary mr-2 mt-1">•</div>
                <div>
                  <div className="font-medium text-sm">合法权益</div>
                  <div className="text-xs text-muted-foreground">维护平台安全和改进服务质量</div>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 生效声明 */}
        <Card>
          <CardContent className="p-6 text-center">
            <Calendar className="h-8 w-8 text-primary mx-auto mb-3" />
            <h3 className="font-medium mb-2">政策生效</h3>
                         <p className="text-sm text-muted-foreground mb-4">
               本隐私政策自2025年8月19日起生效。我们保留随时修改本政策的权利，
               重大变更将提前通知用户。继续使用我们的服务即表示您接受更新后的政策。
             </p>
             <Separator className="my-4" />
             <div className="text-xs text-muted-foreground">
               结绳社区隐私政策 v2.1 | 最后更新：2025年8月19日
             </div>
          </CardContent>
        </Card>

        {/* 底部安全区域 */}
        <div className="h-8" />
      </div>
    </div>
  )
}

export default PrivacyScreen 