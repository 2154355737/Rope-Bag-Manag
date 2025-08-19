import React from 'react'
import { motion } from 'framer-motion'
import { ArrowLeft, Code, Heart, Users, Zap, Shield, Globe, Star, ExternalLink, Github, Mail, Award, Calendar, Download, Smartphone } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { toast } from '@/hooks/use-toast'

const AboutScreen: React.FC = () => {
  const navigate = useNavigate()

  // 应用信息
  const appInfo = {
    name: '结绳社区',
    version: '2.1.0',
    buildNumber: '20250819',
    releaseDate: '2025年8月19日',
    description: '一个基于 React + TypeScript + Capacitor 构建的跨平台移动社区应用，专注于提供现代化的用户体验和原生性能。',
    logo: '🪢'
  }

  // 技术栈
  const techStack = [
    { name: 'React', version: '18.2.0', description: '用户界面构建' },
    { name: 'TypeScript', version: '5.2.2', description: '类型安全的 JavaScript' },
    { name: 'Capacitor', version: '7.4.2', description: '跨平台移动应用框架' },
    { name: 'Tailwind CSS', version: '3.4.1', description: '实用优先的 CSS 框架' },
    { name: 'Vite', version: '5.0.8', description: '快速构建工具' },
    { name: 'Radix UI', version: 'latest', description: '无障碍的 UI 组件基础' }
  ]

  // 功能特性
  const features = [
    {
      icon: Smartphone,
      title: '跨平台支持',
      description: '一套代码，支持 Android 和 Web 平台'
    },
    {
      icon: Code,
      title: '现代化技术',
      description: '基于最新的 React 18 和 TypeScript'
    },
    {
      icon: Zap,
      title: '高性能',
      description: 'Vite 构建工具提供快速的开发体验'
    },
    {
      icon: Shield,
      title: '安全可靠',
      description: '完整的类型安全支持和安全域适配'
    },
    {
      icon: Heart,
      title: '用户体验',
      description: '精美的界面设计和流畅的交互动画'
    },
    {
      icon: Globe,
      title: '国际化',
      description: '支持多语言和本地化适配'
    }
  ]

  // 开发团队
  const team = [
    {
      name: 'Ki.',
      role: '全栈开发工程师',
      avatar: 'http://q2.qlogo.cn/headimg_dl?dst_uin=2154355737&spec=100',
      description: '负责项目全栈架构设计和核心功能开发'
    }
  ]

  // 更新日志
  const changelog = [
    {
      version: '2.1.0',
      date: '2025-08-19',
      changes: [
        '新增安全域配置功能',
        '优化导航栏检测算法',
        '添加预览模式',
        '完善设置页面功能',
        '修复已知问题'
      ]
    },
    {
      version: '2.0.0',
      date: '2025-08-17',
      changes: [
        '全新的界面设计',
        '重构代码架构',
        '添加主题切换功能',
        '优化性能表现',
        '增强移动端适配'
      ]
    }
  ]

  // 统计数据
  const stats = [
    { label: '代码行数', value: '15,000+' },
    { label: 'UI组件', value: '30+' },
    { label: '页面数量', value: '10+' },
    { label: '支持平台', value: '2' }
  ]

  return (
    <div className="min-h-screen bg-background">
      {/* 头部导航 */}
              <div className="sticky top-0 z-40 bg-background/80 backdrop-blur-sm border-b">
        <div className="flex items-center justify-between p-4">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => navigate(-1)}
          >
            <ArrowLeft className="h-5 w-5" />
          </Button>
          <h1 className="text-lg font-semibold">关于应用</h1>
          <div className="w-10" />
        </div>
      </div>

      <div className="container max-w-2xl mx-auto p-4 space-y-6">
        {/* 应用信息 */}
        <Card>
          <CardContent className="p-6 text-center">
            <motion.div
              initial={{ scale: 0.8, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ duration: 0.5 }}
            >
              <div className="text-6xl mb-4">{appInfo.logo}</div>
              <h2 className="text-2xl font-bold mb-2">{appInfo.name}</h2>
              <div className="flex items-center justify-center gap-2 mb-4">
                <Badge variant="outline">v{appInfo.version}</Badge>
                <Badge variant="secondary">Build {appInfo.buildNumber}</Badge>
              </div>
              <p className="text-muted-foreground text-sm leading-relaxed">
                {appInfo.description}
              </p>
              <div className="mt-4 text-xs text-muted-foreground">
                发布日期：{appInfo.releaseDate}
              </div>
            </motion.div>
          </CardContent>
        </Card>

        {/* 项目统计 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Award className="h-4 w-4 mr-2" />
              项目统计
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 gap-4">
              {stats.map((stat, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: index * 0.1 }}
                  className="text-center p-4 rounded-lg border"
                >
                  <div className="text-2xl font-bold text-primary">{stat.value}</div>
                  <div className="text-sm text-muted-foreground">{stat.label}</div>
                </motion.div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* 核心特性 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Star className="h-4 w-4 mr-2" />
              核心特性
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 gap-4">
              {features.map((feature, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: index * 0.1 }}
                  className="flex items-start space-x-3 p-3 rounded-lg border"
                >
                  <feature.icon className="h-5 w-5 text-primary mt-0.5" />
                  <div>
                    <h3 className="font-medium text-sm">{feature.title}</h3>
                    <p className="text-xs text-muted-foreground mt-1">{feature.description}</p>
                  </div>
                </motion.div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* 技术栈 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Code className="h-4 w-4 mr-2" />
              技术栈
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {techStack.map((tech, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: index * 0.05 }}
                  className="flex items-center justify-between p-3 rounded-lg border"
                >
                  <div>
                    <div className="font-medium text-sm">{tech.name}</div>
                    <div className="text-xs text-muted-foreground">{tech.description}</div>
                  </div>
                  <Badge variant="outline" className="text-xs">
                    {tech.version}
                  </Badge>
                </motion.div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* 开发团队 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Users className="h-4 w-4 mr-2" />
              开发团队
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {team.map((member, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: index * 0.1 }}
                  className="flex items-center space-x-3 p-3 rounded-lg border"
                >
                  <Avatar className="h-10 w-10">
                    <AvatarImage src={member.avatar} />
                    <AvatarFallback>{member.name[0]}</AvatarFallback>
                  </Avatar>
                  <div>
                    <div className="font-medium text-sm">{member.name}</div>
                    <div className="text-xs text-primary">{member.role}</div>
                    <div className="text-xs text-muted-foreground mt-1">{member.description}</div>
                  </div>
                </motion.div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* 更新日志 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Calendar className="h-4 w-4 mr-2" />
              更新日志
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {changelog.map((log, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: index * 0.1 }}
                  className="border-l-2 border-primary pl-4"
                >
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="font-medium text-sm">v{log.version}</h3>
                    <span className="text-xs text-muted-foreground">{log.date}</span>
                  </div>
                  <ul className="space-y-1">
                    {log.changes.map((change, changeIndex) => (
                      <li key={changeIndex} className="text-xs text-muted-foreground flex items-start">
                        <span className="text-primary mr-2">•</span>
                        {change}
                      </li>
                    ))}
                  </ul>
                </motion.div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* 开源信息 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Github className="h-4 w-4 mr-2" />
              开源信息
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="text-sm text-muted-foreground">
              本项目采用现代化的技术栈构建，展现了跨平台移动应用开发的最佳实践。
            </div>
            <div className="flex flex-wrap gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  toast({
                    title: "GitHub",
                    description: "项目源码功能开发中，敬请期待！",
                    duration: 3000,
                  })
                }}
              >
                <Github className="h-4 w-4 mr-1" />
                查看源码
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  toast({
                    title: "文档",
                    description: "技术文档功能开发中，敬请期待！",
                    duration: 3000,
                  })
                }}
              >
                <ExternalLink className="h-4 w-4 mr-1" />
                技术文档
              </Button>
            </div>
          </CardContent>
        </Card>

        {/* 联系我们 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Mail className="h-4 w-4 mr-2" />
              联系我们
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="text-sm text-muted-foreground">
              如果您有任何问题、建议或合作意向，欢迎联系我们：
            </div>
            <div className="space-y-2">
              <div className="flex items-center justify-between p-2 rounded border">
                <span className="text-sm">项目邮箱</span>
                <span className="text-sm font-mono">2154355737@qq.com</span>
              </div>
              <div className="flex items-center justify-between p-2 rounded border">
                <span className="text-sm">技术支持</span>
                <span className="text-sm font-mono">QQ:2154355737</span>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 致谢 */}
        <Card>
          <CardContent className="p-6 text-center">
            <Heart className="h-8 w-8 text-red-500 mx-auto mb-3" />
            <h3 className="font-medium mb-2">特别致谢</h3>
            <p className="text-sm text-muted-foreground">
              感谢所有开源项目的贡献者，以及每一位使用和支持结绳社区的用户。
              正是因为有了大家的支持，我们才能不断改进和完善这个应用。
            </p>
            <Separator className="my-4" />
            <div className="text-xs text-muted-foreground">
              Made with ❤️ by 结绳社区开发团队 | 更新日期：2025年8月19日
            </div>
          </CardContent>
        </Card>

        {/* 底部安全区域 */}
        <div className="h-8" />
      </div>
    </div>
  )
}

export default AboutScreen 