import React, { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { getUserProfile } from '../api/user'
import { ApiError } from '../api/client'
import { motion, AnimatePresence } from 'framer-motion'
import { 
  ArrowLeft, 
  User, 
  MapPin, 
  Link as LinkIcon, 
  Calendar, 
  Heart, 
  Eye, 
  Download, 
  MessageSquare, 
  UserPlus, 
  UserMinus,
  Grid3X3,
  FileText,
  Package,
  Star,
  MoreHorizontal
} from 'lucide-react'
import { cn } from '@/lib/utils'
import { followUser, unfollowUser, checkFollowStatus } from '@/api/follow'

// 用户信息接口
interface UserProfile {
  id: number
  username: string
  nickname: string
  avatar_url?: string
  bio?: string
  location?: string
  website?: string
  skills?: string
  followers_count: number
  following_count: number
  posts_count: number
  resources_count: number
  total_likes: number
  total_views: number
  total_downloads: number
  created_at: string
  is_following: boolean
}

// 用户内容接口
interface UserContent {
  id: number
  type: 'post' | 'resource'
  title: string
  description?: string
  stats: {
    views?: number
    likes: number
    comments?: number
    downloads?: number
    rating?: number
  }
  created_at: string
  tags: string[]
}

type TabType = 'overview' | 'posts' | 'resources'

const UserProfileScreen: React.FC = () => {
  const { userId } = useParams<{ userId: string }>()
  const navigate = useNavigate()
  const [profile, setProfile] = useState<UserProfile | null>(null)
  const [contents, setContents] = useState<UserContent[]>([])
  const [activeTab, setActiveTab] = useState<TabType>('overview')
  const [loading, setLoading] = useState(true)
  const [following, setFollowing] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // 加载用户数据
  useEffect(() => {
    const loadUserProfile = async () => {
      if (!userId) return
      
      setLoading(true)
      setError(null) // 重置错误状态
      try {
        const userProfile = await getUserProfile(Number(userId))
        // API客户端已经处理了统一响应格式，直接返回data内容
        setProfile(userProfile)
        setError(null)

        // 暂时使用模拟的内容数据，后续可以调用用户的帖子和资源API
        setContents([
          {
            id: 1,
            type: 'post',
            title: '结绳语言最佳实践指南',
            description: '从零开始学习结绳编程语言，包含基础语法、进阶特性和实战项目。',
            stats: { views: 25640, likes: 1240, comments: 189 },
            created_at: '2024-01-10',
            tags: ['最佳实践', '教程', '经验分享']
          },
          {
            id: 2,
            type: 'resource',
            title: '结绳语言核心库 v2.5',
            description: '包含常用算法和数据结构的核心库，支持最新语言特性。',
            stats: { downloads: 15420, likes: 890, rating: 4.8 },
            created_at: '2024-01-15',
            tags: ['核心库', '算法', '数据结构']
          }
        ])
        
        setLoading(false)
      } catch (error) {
        console.error('加载用户资料失败:', error)
        // 如果是API错误，显示具体的错误信息
        if (error instanceof ApiError) {
          setError(error.message)
        } else if (error instanceof Error) {
          setError(error.message)
        } else {
          setError('加载用户资料失败，请稍后重试')
        }
        setLoading(false)
      }
    }

    loadUserProfile()
  }, [userId])

  // 关注/取消关注
  const handleFollowToggle = async () => {
    if (!profile || !userId) return
    
    try {
      setFollowing(true)
      
      const response = profile.is_following 
        ? await unfollowUser(Number(userId))
        : await followUser(Number(userId))
      
      setProfile(prev => prev ? {
        ...prev,
        is_following: response.is_following,
        followers_count: response.followers_count
      } : null)
    } catch (error) {
      console.error('关注操作失败:', error)
    } finally {
      setFollowing(false)
    }
  }

  const tabs = [
    { id: 'overview', label: '概览', icon: Grid3X3 },
    { id: 'posts', label: '帖子', icon: FileText },
    { id: 'resources', label: '资源', icon: Package }
  ]

  const renderOverview = () => (
    <div className="space-y-6">
      {/* 统计数据 */}
      <div className="grid grid-cols-2 gap-4">
        <div className="p-4 rounded-xl bg-card/50 border border-border/50">
          <div className="flex items-center gap-2 text-primary mb-2">
            <Eye className="w-4 h-4" />
            <span className="text-sm font-medium">总浏览量</span>
          </div>
          <p className="text-2xl font-bold">{profile?.total_views.toLocaleString()}</p>
        </div>
        <div className="p-4 rounded-xl bg-card/50 border border-border/50">
          <div className="flex items-center gap-2 text-primary mb-2">
            <Heart className="w-4 h-4" />
            <span className="text-sm font-medium">总点赞量</span>
          </div>
          <p className="text-2xl font-bold">{profile?.total_likes.toLocaleString()}</p>
        </div>
        <div className="p-4 rounded-xl bg-card/50 border border-border/50">
          <div className="flex items-center gap-2 text-primary mb-2">
            <Download className="w-4 h-4" />
            <span className="text-sm font-medium">总下载量</span>
          </div>
          <p className="text-2xl font-bold">{profile?.total_downloads.toLocaleString()}</p>
        </div>
        <div className="p-4 rounded-xl bg-card/50 border border-border/50">
          <div className="flex items-center gap-2 text-primary mb-2">
            <Star className="w-4 h-4" />
            <span className="text-sm font-medium">创作内容</span>
          </div>
          <p className="text-2xl font-bold">{(profile?.posts_count || 0) + (profile?.resources_count || 0)}</p>
        </div>
      </div>

      {/* 最新内容 */}
      <div>
        <h3 className="text-lg font-semibold mb-4">最新创作</h3>
        <div className="space-y-3">
          {contents.slice(0, 3).map((content) => (
            <motion.div
              key={content.id}
              className="p-4 rounded-xl bg-card/50 border border-border/50 hover:bg-card/80 transition-all duration-200"
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
            >
              <div className="flex items-start gap-3">
                <div className={cn(
                  "w-8 h-8 rounded-lg flex items-center justify-center",
                  content.type === 'post' ? "bg-blue-500/10 text-blue-500" : "bg-green-500/10 text-green-500"
                )}>
                  {content.type === 'post' ? <FileText className="w-4 h-4" /> : <Package className="w-4 h-4" />}
                </div>
                <div className="flex-1 min-w-0">
                  <h4 className="font-medium text-foreground line-clamp-1">{content.title}</h4>
                  {content.description && (
                    <p className="text-sm text-muted-foreground line-clamp-2 mt-1">{content.description}</p>
                  )}
                  <div className="flex items-center gap-3 mt-2 text-xs text-muted-foreground">
                    {content.stats.views && (
                      <span className="flex items-center gap-1">
                        <Eye className="w-3 h-3" />
                        {content.stats.views}
                      </span>
                    )}
                    <span className="flex items-center gap-1">
                      <Heart className="w-3 h-3" />
                      {content.stats.likes}
                    </span>
                    {content.stats.downloads && (
                      <span className="flex items-center gap-1">
                        <Download className="w-3 h-3" />
                        {content.stats.downloads}
                      </span>
                    )}
                  </div>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </div>
  )

  const renderContentList = (type: 'posts' | 'resources') => {
    const filteredContents = contents.filter(content => content.type === type.slice(0, -1) as 'post' | 'resource')
    
    return (
      <div className="space-y-3">
        {filteredContents.map((content) => (
          <motion.div
            key={content.id}
            className="p-4 rounded-xl bg-card/50 border border-border/50 hover:bg-card/80 transition-all duration-200"
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            <h4 className="font-medium text-foreground mb-2">{content.title}</h4>
            {content.description && (
              <p className="text-sm text-muted-foreground line-clamp-2 mb-3">{content.description}</p>
            )}
            <div className="flex flex-wrap gap-1 mb-3">
              {content.tags.map((tag, index) => (
                <span 
                  key={index}
                  className="px-2 py-1 text-xs bg-secondary/50 text-secondary-foreground rounded-full"
                >
                  {tag}
                </span>
              ))}
            </div>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3 text-xs text-muted-foreground">
                {content.stats.views && (
                  <span className="flex items-center gap-1">
                    <Eye className="w-3 h-3" />
                    {content.stats.views}
                  </span>
                )}
                <span className="flex items-center gap-1">
                  <Heart className="w-3 h-3" />
                  {content.stats.likes}
                </span>
                {content.stats.comments && (
                  <span className="flex items-center gap-1">
                    <MessageSquare className="w-3 h-3" />
                    {content.stats.comments}
                  </span>
                )}
                {content.stats.downloads && (
                  <span className="flex items-center gap-1">
                    <Download className="w-3 h-3" />
                    {content.stats.downloads}
                  </span>
                )}
              </div>
              <span className="text-xs text-muted-foreground">
                {new Date(content.created_at).toLocaleDateString()}
              </span>
            </div>
          </motion.div>
        ))}
      </div>
    )
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-background via-background to-accent/5 flex items-center justify-center">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-background via-background to-accent/5 flex items-center justify-center">
        <div className="text-center">
          <h2 className="text-xl font-semibold mb-2">
            {error.includes('不存在') ? '用户不存在' : '加载失败'}
          </h2>
          <p className="text-muted-foreground mb-4">{error}</p>
          <button
            onClick={() => navigate(-1)}
            className="px-4 py-2 bg-primary text-primary-foreground rounded-lg"
          >
            返回
          </button>
        </div>
      </div>
    )
  }

  if (!profile) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-background via-background to-accent/5 flex items-center justify-center">
        <div className="text-center">
          <div className="w-16 h-16 border-4 border-primary/20 border-t-primary rounded-full animate-spin mx-auto mb-4"></div>
          <p className="text-muted-foreground">加载用户资料中...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-background via-background to-accent/5">
      {/* 头部导航 */}
      <div className="sticky top-0 z-10 bg-background/80 backdrop-blur-xl border-b border-border/50">
        <div className="flex items-center justify-between px-6 py-4">
          <button
            onClick={() => navigate(-1)}
            className="p-2 -ml-2 rounded-full hover:bg-muted/50 transition-colors"
          >
            <ArrowLeft className="w-5 h-5" />
          </button>
          <h1 className="text-lg font-semibold">{profile.nickname}</h1>
          <button className="p-2 -mr-2 rounded-full hover:bg-muted/50 transition-colors">
            <MoreHorizontal className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* 用户信息卡片 */}
      <div className="px-6 py-6">
        <div className="bg-card/50 rounded-2xl border border-border/50 p-6 backdrop-blur-sm">
          {/* 头像和基本信息 */}
          <div className="flex items-start gap-4 mb-6">
            <img
              src={profile.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=default'}
              alt={profile.nickname}
              className="w-20 h-20 rounded-2xl border-2 border-primary/20"
            />
            <div className="flex-1 min-w-0">
              <h2 className="text-xl font-bold text-foreground">{profile.nickname}</h2>
              <p className="text-muted-foreground mb-2">@{profile.username}</p>
              {profile.bio && (
                <p className="text-sm text-foreground leading-relaxed">{profile.bio}</p>
              )}
            </div>
          </div>

          {/* 详细信息 */}
          <div className="space-y-2 mb-6">
            {profile.location && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <MapPin className="w-4 h-4" />
                <span>{profile.location}</span>
              </div>
            )}
            {profile.website && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <LinkIcon className="w-4 h-4" />
                <span className="text-primary">{profile.website}</span>
              </div>
            )}
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <Calendar className="w-4 h-4" />
              <span>加入于 {new Date(profile.created_at).toLocaleDateString()}</span>
            </div>
            {profile.skills && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <User className="w-4 h-4" />
                <span>{profile.skills}</span>
              </div>
            )}
          </div>

          {/* 统计信息 */}
          <div className="flex items-center gap-6 mb-6 text-sm">
            <span>
              <strong className="text-foreground">{profile.posts_count}</strong>
              <span className="text-muted-foreground ml-1">帖子</span>
            </span>
            <span>
              <strong className="text-foreground">{profile.resources_count}</strong>
              <span className="text-muted-foreground ml-1">资源</span>
            </span>
            <span>
              <strong className="text-foreground">{profile.followers_count}</strong>
              <span className="text-muted-foreground ml-1">关注者</span>
            </span>
            <span>
              <strong className="text-foreground">{profile.following_count}</strong>
              <span className="text-muted-foreground ml-1">关注中</span>
            </span>
          </div>

          {/* 关注按钮 */}
          <motion.button
            onClick={handleFollowToggle}
            disabled={following}
            className={cn(
              "w-full py-3 rounded-xl font-medium transition-all duration-200 flex items-center justify-center gap-2",
              profile.is_following
                ? "bg-muted/50 text-muted-foreground hover:bg-destructive/10 hover:text-destructive"
                : "bg-primary text-primary-foreground hover:bg-primary/90"
            )}
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            {following ? (
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-current"></div>
            ) : (
              <>
                {profile.is_following ? (
                  <>
                    <UserMinus className="w-4 h-4" />
                    取消关注
                  </>
                ) : (
                  <>
                    <UserPlus className="w-4 h-4" />
                    关注
                  </>
                )}
              </>
            )}
          </motion.button>
        </div>
      </div>

      {/* 标签页 */}
      <div className="px-6">
        <div className="flex bg-muted/30 rounded-xl p-1 backdrop-blur-sm mb-6">
          {tabs.map((tab) => {
            const Icon = tab.icon
            const isActive = activeTab === tab.id
            
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as TabType)}
                className={cn(
                  "flex-1 flex items-center justify-center gap-2 py-2 px-4 rounded-lg transition-all duration-200 text-sm font-medium",
                  isActive
                    ? "bg-primary text-primary-foreground shadow-lg"
                    : "text-muted-foreground hover:text-foreground hover:bg-muted/50"
                )}
              >
                <Icon className="w-4 h-4" />
                {tab.label}
              </button>
            )
          })}
        </div>

        {/* 内容区域 */}
        <div className="pb-safe">
          <AnimatePresence mode="wait">
            <motion.div
              key={activeTab}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              transition={{ duration: 0.2 }}
            >
              {activeTab === 'overview' && renderOverview()}
              {activeTab === 'posts' && renderContentList('posts')}
              {activeTab === 'resources' && renderContentList('resources')}
            </motion.div>
          </AnimatePresence>
        </div>
      </div>
    </div>
  )
}

export default UserProfileScreen 