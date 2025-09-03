import React from 'react'
import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'
import { ArrowLeft, Shield, Scale, AlertTriangle, FileText, Users, Globe, Zap } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@/components/ui/accordion'

const TermsScreen: React.FC = () => {
  const navigate = useNavigate()

  // 服务条款内容
  const termsData = [
    {
      id: 'service-overview',
      title: '服务概述',
      icon: Globe,
      content: `
        结绳社区是一个专注于技术交流与学习的在线平台。我们致力于为用户提供：
        
        • 技术文章发布与分享
        • 开源项目展示与协作
        • 在线学习资源与教程
        • 技术问答与讨论社区
        • 开发工具与资源下载
        
        通过使用我们的服务，您同意遵守本服务条款的所有规定。
      `
    },
    {
      id: 'user-accounts',
      title: '用户账户',
      icon: Users,
      content: `
        账户注册：
        • 您必须提供准确、完整的注册信息
        • 一个邮箱地址只能注册一个账户
        • 您有责任保护账户密码的安全
        • 禁止创建虚假账户或冒充他人
        
        账户使用：
        • 您对账户下的所有活动负责
        • 不得与他人共享账户信息
        • 发现账户被盗用应立即联系我们
        • 我们保留暂停或终止违规账户的权利
      `
    },
    {
      id: 'content-policy',
      title: '内容政策',
      icon: FileText,
      content: `
        允许的内容：
        • 原创技术文章和教程
        • 开源项目和代码分享
        • 建设性的技术讨论
        • 学习心得和经验分享
        
        禁止的内容：
        • 侵犯他人版权的内容
        • 恶意代码或病毒
        • 垃圾信息和广告
        • 违法违规内容
        • 人身攻击和仇恨言论
        • 虚假信息和谣言
      `
    },
    {
      id: 'intellectual-property',
      title: '知识产权',
      icon: Shield,
      content: `
        用户内容：
        • 您保留对上传内容的所有权
        • 授予我们展示和分发内容的许可
        • 承诺不侵犯他人知识产权
        • 对内容的合法性承担责任
        
        平台内容：
        • 平台界面和功能受版权保护
        • 未经许可不得复制平台内容
        • 商标和Logo属于结绳社区所有
        • 尊重第三方知识产权
      `
    },
    {
      id: 'user-conduct',
      title: '用户行为规范',
      icon: Scale,
      content: `
        行为准则：
        • 尊重其他用户，友善交流
        • 遵守社区讨论规则
        • 不得进行恶意攻击或骚扰
        • 保护他人隐私信息
        
        禁止行为：
        • 发布垃圾信息或广告
        • 恶意刷屏或灌水
        • 传播病毒或恶意代码
        • 尝试入侵系统或获取他人信息
        • 使用自动化工具进行非法操作
      `
    },
    {
      id: 'service-availability',
      title: '服务可用性',
      icon: Zap,
      content: `
        服务承诺：
        • 我们努力保证服务的稳定运行
        • 定期进行系统维护和更新
        • 提供技术支持和用户帮助
        • 不断改进用户体验
        
        服务限制：
        • 服务可能因维护而暂时中断
        • 不保证服务100%无故障运行
        • 可能对某些功能进行限制
        • 保留修改或终止服务的权利
      `
    },
    {
      id: 'liability',
      title: '责任限制',
      icon: AlertTriangle,
      content: `
        平台责任：
        • 我们尽力保护用户数据安全
        • 对故意违法行为不承担责任
        • 不对用户内容的准确性负责
        • 不承担间接或特殊损失
        
        用户责任：
        • 对自己的行为和内容负责
        • 承担违法使用的后果
        • 配合平台的管理工作
        • 及时报告发现的问题
      `
    },
    {
      id: 'modifications',
      title: '条款修改',
      icon: FileText,
      content: `
        修改权利：
        • 我们保留修改本条款的权利
        • 重大修改将提前通知用户
        • 继续使用服务视为接受修改
        • 不同意修改可以停止使用服务
        
        通知方式：
        • 网站公告或邮件通知
        • 应用内消息推送
        • 官方社交媒体发布
        • 用户主动查看最新条款
      `
    }
  ]

  return (
    <div className="min-h-screen bg-background">
      <div className="container max-w-4xl mx-auto p-4">
        {/* 头部导航 */}
        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="mb-6"
        >
          <Button
            variant="ghost"
            size="sm"
            onClick={() => navigate(-1)}
            className="mb-4"
          >
            <ArrowLeft size={16} className="mr-2" />
            返回
          </Button>
        </motion.div>

        {/* 标题部分 */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.1 }}
        >
          <Card className="mb-6">
            <CardHeader className="text-center">
              <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
                <Scale size={32} className="text-primary" />
              </div>
              <div className="space-y-2">
                <CardTitle className="text-2xl font-bold">服务条款</CardTitle>
                <Badge variant="outline" className="mb-3">
                  最后更新：2025年8月19日
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="text-center">
              <p className="text-muted-foreground">
                欢迎使用结绳社区！请仔细阅读以下服务条款，这些条款规定了您使用我们服务的权利和义务。
                使用我们的服务即表示您同意遵守这些条款。
              </p>
              <Separator className="my-4" />
              <div className="text-xs text-muted-foreground">
                生效日期：2025年8月19日 | 适用范围：结绳社区所有用户
              </div>
            </CardContent>
          </Card>
        </motion.div>

        {/* 条款内容 */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.2 }}
        >
          <Card>
            <CardContent className="p-6">
              <Accordion type="single" collapsible className="w-full">
                {termsData.map((section, index) => (
                  <AccordionItem key={section.id} value={section.id}>
                    <AccordionTrigger className="text-left">
                      <div className="flex items-center">
                        <section.icon size={20} className="mr-3 text-primary" />
                        <span className="font-medium">{section.title}</span>
                      </div>
                    </AccordionTrigger>
                    <AccordionContent>
                      <div className="pl-8 pt-2">
                        <div className="prose prose-sm dark:prose-invert max-w-none">
                          <div className="whitespace-pre-line text-sm text-muted-foreground leading-relaxed">
                            {section.content}
                          </div>
                        </div>
                      </div>
                    </AccordionContent>
                  </AccordionItem>
                ))}
              </Accordion>
            </CardContent>
          </Card>
        </motion.div>

        {/* 底部信息 */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.3 }}
          className="mt-6"
        >
          <Card>
            <CardContent className="p-6">
              <div className="text-center space-y-4">
                <h3 className="font-medium">联系我们</h3>
                <p className="text-sm text-muted-foreground">
                  如果您对本服务条款有任何疑问，请通过以下方式联系我们：
                </p>
                <div className="flex justify-center space-x-6 text-sm">
                  <div className="flex items-center">
                    <span className="text-muted-foreground">邮箱：</span>
                    <span className="ml-1">2154355737@qq.com</span>
                  </div>
                  <div className="flex items-center">
                    <span className="text-muted-foreground">QQ：</span>
                    <span className="ml-1">2154355737</span>
                  </div>
                </div>
                <Separator className="my-4" />
                <div className="text-xs text-muted-foreground">
                  结绳社区服务条款 v2.1.8 | 最后更新：2025年9月4日
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>
      </div>
    </div>
  )
}

export default TermsScreen 