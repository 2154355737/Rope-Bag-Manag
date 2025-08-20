import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  ArrowLeft, Share2, Bookmark, MoreHorizontal, Flag, 
  AlertTriangle, Info, CheckCircle, Clock, ExternalLink, 
  ThumbsUp, MessageSquare, Bell, Pin, Eye, Calendar,
  FileText, Download, Link as LinkIcon
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { toast } from '@/hooks/use-toast'
import TopNavigation from '@/components/ui/top-navigation'
import CommentSection, { Comment } from '@/components/comment-section'
import RelatedRecommendations from '@/components/related-recommendations'
import InteractionButtons, { 
  createThumbsUpButton, 
  createBookmarkButton, 
  createShareButton, 
  createReportButton 
} from '@/components/ui/interaction-buttons'
import { getAnnouncementRecommendations } from '@/utils/recommendations'

const AnnouncementDetailScreen: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [isLiked, setIsLiked] = useState(false)
  const [isBookmarked, setIsBookmarked] = useState(false)
  
  // 模拟公告数据
  const announcement = {
    id: parseInt(id || '1'),
    title: '重要通知：结绳社区服务升级维护公告',
    type: 'important', // important, info, warning, update
    priority: 'high', // high, medium, low
    author: {
      name: '结绳社区官方',
      avatar: 'https://i.pravatar.cc/150?img=6',
      role: '管理员',
      verified: true,
    },
    content: `各位用户：

为了给大家提供更好的服务体验，结绳社区将进行系统升级维护。现将相关事宜通知如下：

## 维护时间
**2025年1月20日 02:00 - 06:00 (UTC+8)**

## 维护内容
1. **服务器性能优化**
   - 提升响应速度
   - 增强系统稳定性
   - 优化数据库查询效率

2. **新功能上线**
   - 智能推荐系统
   - 实时消息通知
   - 个性化主题设置

3. **Bug修复**
   - 修复已知的界面显示问题
   - 解决文件上传偶发失败
   - 优化移动端适配

## 影响范围
维护期间，以下功能将暂时不可用：
- 用户登录/注册
- 内容发布与编辑
- 文件上传下载
- 实时消息推送

## 注意事项
- 请在维护开始前保存好您的工作内容
- 维护期间请勿尝试登录，以免造成数据异常
- 如遇紧急问题，请联系客服邮箱：support@jieshengshequ.com

## 补偿措施
为感谢大家的理解与支持，维护完成后所有用户将获得：
- 7天VIP体验权限
- 专属纪念徽章
- 社区积分奖励

感谢大家的理解与支持！

结绳社区运营团队  
2025年1月15日`,
    publishDate: '2025-01-15',
    effectiveDate: '2025-01-20',
    expiryDate: '2025-01-25',
    tags: ['系统维护', '重要通知', '服务升级'],
    views: 5680,
    likes: 234,
    comments: 45,
    isPinned: true,
    attachments: [
      {
        name: '维护详细说明.pdf',
        size: '2.1 MB',
        url: '#'
      }
    ],
    relatedLinks: [
      {
        title: '用户服务协议更新说明',
        url: '#',
        description: '查看最新的用户服务协议变更内容'
      },
      {
        title: '常见问题解答',
        url: '#',
        description: '维护期间可能遇到的问题及解决方案'
      }
    ]
  }

  // 获取相关推荐
  const recommendedItems = getAnnouncementRecommendations(announcement.id, announcement.tags)

  // 模拟评论数据
  const comments: Comment[] = [
    {
      id: 1,
      author: {
        name: '社区用户',
        avatar: 'https://i.pravatar.cc/150?img=1',
      },
      content: '感谢官方的及时通知，请问维护期间数据会丢失吗？',
      time: '1小时前',
      likes: 5,
      isLiked: false,
      replies: [
        {
          id: 101,
          author: {
            name: '结绳社区官方',
            avatar: 'https://i.pravatar.cc/150?img=6',
            verified: true,
          },
          content: '不会的，所有用户数据都会完全保留，请放心！',
          time: '30分钟前',
          likes: 12,
          isLiked: true,
        }
      ]
    },
    {
      id: 2,
      author: {
        name: '开发者小王',
        avatar: 'https://i.pravatar.cc/150?img=3',
      },
      content: '新功能很期待！希望智能推荐系统能更精准一些。',
      time: '3小时前',
      likes: 8,
      isLiked: false,
    },
    {
      id: 3,
      author: {
        name: '活跃用户',
        avatar: 'https://i.pravatar.cc/150?img=4',
      },
      content: '终于要更新了！期待已久的功能升级，社区越来越好了！',
      time: '4小时前',
      likes: 15,
      isLiked: true,
    },
    {
      id: 4,
      author: {
        name: '程序员李四',
        avatar: 'https://i.pravatar.cc/150?img=5',
      },
      content: '维护时间能不能安排在深夜进行，这样对白天使用的影响比较小。',
      time: '5小时前',
      likes: 12,
      isLiked: false,
      replies: [
        {
          id: 401,
          author: {
            name: '结绳社区官方',
            avatar: 'https://i.pravatar.cc/150?img=6',
            verified: true,
          },
          content: '感谢建议！我们会考虑在使用量较低的时段进行维护。',
          time: '2小时前',
          likes: 8,
          isLiked: false,
        }
      ]
    },
    {
      id: 5,
      author: {
        name: '新手小白',
        avatar: 'https://i.pravatar.cc/150?img=7',
      },
      content: '作为新用户，很喜欢这个社区的氛围！期待新功能上线。',
      time: '6小时前',
      likes: 6,
      isLiked: false,
    },
    {
      id: 6,
      author: {
        name: '资深开发者',
        avatar: 'https://i.pravatar.cc/150?img=8',
      },
      content: '希望新版本能解决一些性能问题，整体体验还是很不错的。',
      time: '昨天',
      likes: 20,
      isLiked: true,
    },
    {
      id: 7,
      author: {
        name: '产品经理小张',
        avatar: 'https://i.pravatar.cc/150?img=9',
      },
      content: '从产品角度来看，这次更新的功能都很实用，用户体验会有很大提升。',
      time: '昨天',
      likes: 14,
      isLiked: false,
    }
  ]

  // 格式化数字
  const formatNumber = (num: number) => {
    if (num >= 10000) return `${(num / 10000).toFixed(1)}万`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`
    return num.toString()
  }

  // 获取公告类型样式
  const getAnnouncementStyle = (type: string) => {
    switch (type) {
      case 'important':
        return {
          bgColor: 'bg-red-50 dark:bg-red-950',
          borderColor: 'border-red-200 dark:border-red-800',
          iconColor: 'text-red-500',
          icon: AlertTriangle
        }
      case 'warning':
        return {
          bgColor: 'bg-yellow-50 dark:bg-yellow-950',
          borderColor: 'border-yellow-200 dark:border-yellow-800',
          iconColor: 'text-yellow-500',
          icon: AlertTriangle
        }
      case 'info':
        return {
          bgColor: 'bg-blue-50 dark:bg-blue-950',
          borderColor: 'border-blue-200 dark:border-blue-800',
          iconColor: 'text-blue-500',
          icon: Info
        }
      default:
        return {
          bgColor: 'bg-green-50 dark:bg-green-950',
          borderColor: 'border-green-200 dark:border-green-800',
          iconColor: 'text-green-500',
          icon: CheckCircle
        }
    }
  }

  // 处理点赞
  const handleLike = () => {
    setIsLiked(!isLiked)
    toast({
      title: isLiked ? "已取消点赞" : "点赞成功",
      description: isLiked ? "已取消对此公告的点赞" : "感谢您的支持",
      duration: 2000,
    })
  }

  // 处理收藏
  const handleBookmark = () => {
    setIsBookmarked(!isBookmarked)
    toast({
      title: isBookmarked ? "已取消收藏" : "收藏成功",
      description: isBookmarked ? "已从收藏夹中移除" : "已添加到您的收藏夹",
      duration: 2000,
    })
  }

  // 评论区事件处理
  const handleSubmitComment = (content: string) => {
    console.log('新评论:', content)
    toast({
      title: "反馈已提交",
      description: "感谢您的反馈，我们会认真处理"
    })
  }

  const handleSubmitReply = (commentId: number, content: string) => {
    console.log('回复评论:', commentId, content)
    toast({
      title: "回复发送成功",
      description: "您的回复已发布"
    })
  }

  const handleLikeComment = (commentId: number) => {
    console.log('点赞评论:', commentId)
    toast({
      title: "操作成功",
      description: "已点赞/取消点赞"
    })
  }

  const handleReportComment = (commentId: number) => {
    console.log('举报评论:', commentId)
  }

  // 处理分享
  const handleShare = () => {
    toast({
      title: "分享链接已复制",
      description: "可以分享给更多朋友了",
      duration: 2000,
    })
  }

  // 处理举报
  const handleReport = () => {
    toast({
      title: "举报已提交",
      description: "我们会尽快处理您的举报",
      duration: 2000,
    })
  }

  const announcementStyle = getAnnouncementStyle(announcement.type)
  const IconComponent = announcementStyle.icon

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="公告详情"
        showBackButton
        rightAction={
          <div className="flex items-center gap-1">
            <Button 
              variant="ghost" 
              size="icon" 
              className="h-9 w-9"
              onClick={() => setIsBookmarked(!isBookmarked)}
            >
              <Bookmark size={20} className={isBookmarked ? "fill-current" : ""} />
            </Button>
            <Button variant="ghost" size="icon" className="h-9 w-9">
              <Share2 size={20} />
            </Button>
            <Button variant="ghost" size="icon" className="h-9 w-9">
              <MoreHorizontal size={20} />
            </Button>
          </div>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        <ScrollArea className="flex-1">
        <div className="p-4">
          {/* 公告头部 */}
          <Card className={`mb-4 ${announcementStyle.bgColor} ${announcementStyle.borderColor}`}>
            <CardContent className="p-4">
              <div className="flex items-start mb-4">
                <IconComponent size={24} className={`${announcementStyle.iconColor} mr-3 mt-1`} />
                <div className="flex-1">
                  <div className="flex items-center mb-2">
                    <Badge 
                      variant={announcement.priority === 'high' ? 'destructive' : 'secondary'} 
                      className="mr-2"
                    >
                      {announcement.priority === 'high' ? '重要' : '一般'}
                    </Badge>
                    {announcement.isPinned && (
                      <Badge variant="outline" className="mr-2">
                        <Pin size={10} className="mr-1" />
                        置顶
                      </Badge>
                    )}
                    <Badge variant="outline">
                      {announcement.type === 'important' ? '重要通知' : '系统公告'}
                    </Badge>
                  </div>
                  <h2 className="text-xl font-bold mb-3">{announcement.title}</h2>
                  
                  <div className="flex items-center justify-between">
                    <div className="flex items-center">
                      <Avatar className="h-8 w-8 mr-2">
                        <AvatarImage src={announcement.author.avatar} />
                        <AvatarFallback>{announcement.author.name[0]}</AvatarFallback>
                      </Avatar>
                      <div>
                        <div className="flex items-center">
                          <span className="font-medium text-sm">{announcement.author.name}</span>
                          {announcement.author.verified && (
                            <CheckCircle size={14} className="ml-1 text-blue-500" />
                          )}
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {announcement.author.role} • {announcement.publishDate}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* 时间信息 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              <div className="grid grid-cols-1 gap-3">
                <div className="flex items-center">
                  <Calendar size={16} className="text-muted-foreground mr-2" />
                  <span className="text-sm">发布时间：{announcement.publishDate}</span>
                </div>
                {announcement.effectiveDate && (
                  <div className="flex items-center">
                    <Clock size={16} className="text-muted-foreground mr-2" />
                    <span className="text-sm">生效时间：{announcement.effectiveDate}</span>
                  </div>
                )}
                {announcement.expiryDate && (
                  <div className="flex items-center">
                    <AlertTriangle size={16} className="text-orange-500 mr-2" />
                    <span className="text-sm">截止时间：{announcement.expiryDate}</span>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>

          {/* 公告内容 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              <div className="prose prose-sm max-w-none">
                {announcement.content.split('\n\n').map((section, idx) => {
                  if (section.startsWith('## ')) {
                    return (
                      <h3 key={idx} className="text-lg font-semibold mt-6 mb-3 first:mt-0">
                        {section.replace('## ', '')}
                      </h3>
                    )
                  } else if (section.startsWith('**') && section.endsWith('**')) {
                    return (
                      <div key={idx} className="bg-muted p-3 rounded-md my-3">
                        <p className="font-medium text-sm">
                          {section.replace(/\*\*/g, '')}
                        </p>
                      </div>
                    )
                  } else if (section.includes('- ')) {
                    return (
                      <ul key={idx} className="list-disc list-inside space-y-1 my-3">
                        {section.split('\n').map((line, lineIdx) => 
                          line.trim().startsWith('- ') && (
                            <li key={lineIdx} className="text-sm ml-2">
                              {line.trim().substring(2)}
                            </li>
                          )
                        )}
                      </ul>
                    )
                  } else if (section.includes('\n   - ')) {
                    const lines = section.split('\n')
                    return (
                      <div key={idx} className="my-3">
                        <p className="font-medium text-sm mb-2">{lines[0]}</p>
                        <ul className="list-disc list-inside space-y-1 ml-4">
                          {lines.slice(1).map((line, lineIdx) => 
                            line.trim().startsWith('- ') && (
                              <li key={lineIdx} className="text-sm">
                                {line.trim().substring(2)}
                              </li>
                            )
                          )}
                        </ul>
                      </div>
                    )
                  } else {
                    return (
                      <p key={idx} className="text-sm my-3">
                        {section}
                      </p>
                    )
                  }
                })}
              </div>
            </CardContent>
          </Card>

          {/* 附件下载 */}
          {announcement.attachments.length > 0 && (
            <Card className="mb-4">
              <CardHeader>
                <CardTitle className="text-lg">相关附件</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="space-y-3">
                  {announcement.attachments.map((attachment, idx) => (
                    <div key={idx} className="flex items-center justify-between p-3 border rounded-md">
                      <div className="flex items-center">
                        <FileText size={16} className="text-muted-foreground mr-2" />
                        <div>
                          <div className="font-medium text-sm">{attachment.name}</div>
                          <div className="text-xs text-muted-foreground">{attachment.size}</div>
                        </div>
                      </div>
                      <Button size="sm" variant="outline">
                        <Download size={14} className="mr-1" />
                        下载
                      </Button>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 相关链接 */}
          {announcement.relatedLinks.length > 0 && (
            <Card className="mb-4">
              <CardHeader>
                <CardTitle className="text-lg">相关链接</CardTitle>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="space-y-3">
                  {announcement.relatedLinks.map((link, idx) => (
                    <div key={idx} className="p-3 border rounded-md">
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <div className="flex items-center mb-1">
                            <LinkIcon size={14} className="text-muted-foreground mr-2" />
                            <span className="font-medium text-sm">{link.title}</span>
                          </div>
                          <p className="text-xs text-muted-foreground">{link.description}</p>
                        </div>
                        <Button size="sm" variant="ghost">
                          <ExternalLink size={14} />
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* 标签 */}
          <Card className="mb-4">
            <CardContent className="p-4">
              <div className="flex flex-wrap gap-2">
                {announcement.tags.map((tag, idx) => (
                  <Badge key={idx} variant="outline" className="text-xs">
                    {tag}
                  </Badge>
                ))}
              </div>
            </CardContent>
          </Card>

          {/* 统计信息 */}
          <Card className="mb-4">
            <CardContent className="p-3">
              <div className="grid grid-cols-3 gap-3 text-center">
                <div>
                  <div className="flex items-center justify-center mb-1">
                    <Eye size={12} className="text-muted-foreground mr-1" />
                  </div>
                  <div className="text-base font-bold">{formatNumber(announcement.views)}</div>
                  <div className="text-[10px] text-muted-foreground">浏览量</div>
                </div>
                <div>
                  <div className="flex items-center justify-center mb-1">
                    <ThumbsUp size={12} className="text-muted-foreground mr-1" />
                  </div>
                  <div className="text-base font-bold">{formatNumber(announcement.likes)}</div>
                  <div className="text-[10px] text-muted-foreground">点赞数</div>
                </div>
                <div>
                  <div className="flex items-center justify-center mb-1">
                    <MessageSquare size={12} className="text-muted-foreground mr-1" />
                  </div>
                  <div className="text-base font-bold">{formatNumber(announcement.comments)}</div>
                  <div className="text-[10px] text-muted-foreground">评论数</div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* 操作按钮 */}
          <InteractionButtons
            buttons={[
              createThumbsUpButton(announcement.likes + (isLiked ? 1 : 0), isLiked, handleLike),
              createShareButton(handleShare),
              createBookmarkButton(undefined, isBookmarked, handleBookmark),
              createReportButton(handleReport)
            ]}
            className="mb-4"
            compact={true}
          />

          {/* 重要提醒 */}
          <Alert className="mb-4">
            <Bell size={16} />
            <AlertDescription>
              此公告内容重要，建议收藏保存。如有疑问请及时联系客服。
            </AlertDescription>
          </Alert>
        </div>
      </ScrollArea>

      {/* 相关推荐 */}
      <div className="p-4">
        <RelatedRecommendations
          title="相关公告推荐"
          items={recommendedItems}
          currentItemId={announcement.id}
          maxItems={6}
          onMoreClick={() => navigate('/home')}
        />
      </div>

      {/* 评论区 */}
      <div className="p-4">
        <CommentSection
          comments={comments}
          totalCount={announcement.comments}
          onSubmitComment={handleSubmitComment}
          onSubmitReply={handleSubmitReply}
          onLikeComment={handleLikeComment}
          onReportComment={handleReportComment}
          placeholder="对此公告有疑问或建议..."
          maxLength={200}
          initialCommentsToShow={5}
        />
      </div>
      </div> {/* 结束内容区域 */}
    </div>
  )
}

export default AnnouncementDetailScreen 