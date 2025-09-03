import React, { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { motion, AnimatePresence } from 'framer-motion'
import { Trophy, Star, Eye, Heart, Download, MessageSquare, TrendingUp, Award, Crown } from 'lucide-react'
import { cn } from '@/lib/utils'
import { getUserRanking, getResourceRanking, getPostRanking } from '../api/ranking'

// 排行榜数据接口
interface UserRanking {
  id: number
  username: string
  nickname: string
  avatar_url?: string
  score: number
  posts_count: number
  resources_count: number
  followers_count: number
  likes_count: number
}

interface ResourceRanking {
  id: number
  title: string
  author: {
    id: number
    username: string
    nickname: string
    avatar_url?: string
  }
  downloads: number
  likes: number
  rating: number
  category: string
  created_at: string
}

interface PostRanking {
  id: number
  title: string
  author: {
    id: number
    username: string
    nickname: string
    avatar_url?: string
  }
  views: number
  likes: number
  comments: number
  created_at: string
  tags: string[]
}

// 排行榜类型
type RankingType = 'users' | 'resources' | 'posts'

const RankingScreen: React.FC = () => {
  const navigate = useNavigate()
  const [activeTab, setActiveTab] = useState<RankingType>('users')
  const [userRankings, setUserRankings] = useState<UserRanking[]>([])
  const [resourceRankings, setResourceRankings] = useState<ResourceRanking[]>([])
  const [postRankings, setPostRankings] = useState<PostRanking[]>([])
  const [loading, setLoading] = useState(true)

  // 加载数据
  useEffect(() => {
    const loadData = async () => {
      setLoading(true)
      try {
        // 获取用户排行数据
        const userResponse = await getUserRanking({ page: 1, page_size: 20 })
        const userData = userResponse.items.map(user => ({
          id: user.id,
          username: user.username,
          nickname: user.nickname || user.username,
          avatar_url: user.avatar_url,
          score: user.star,
          posts_count: 0, // 暂时设为0，后续可以从API获取
          resources_count: user.upload_count,
          followers_count: 0, // 暂时设为0，后续可以从API获取
          likes_count: 0 // 暂时设为0，后续可以从API获取
        }))
        setUserRankings(userData)

        // 获取资源排行数据
        const resourceResponse = await getResourceRanking({ page: 1, page_size: 20 })
        const resourceData = resourceResponse.items.map(resource => ({
          id: resource.id,
          title: resource.title, // 修复：使用title而不是name
          author: {
            ...resource.author,
            nickname: resource.author.nickname || resource.author.username
          },
          downloads: resource.downloads, // 修复：使用downloads而不是download_count
          likes: resource.likes, // 修复：使用likes而不是like_count
          rating: resource.rating,
          category: resource.category,
          created_at: resource.created_at
        }))
        setResourceRankings(resourceData)

        // 获取帖子排行数据
        const postResponse = await getPostRanking({ page: 1, page_size: 20 })
        const postData = postResponse.items.map(post => ({
          id: post.id,
          title: post.title,
          author: {
            ...post.author,
            nickname: post.author.nickname || post.author.username
          },
          views: post.views,
          likes: post.likes,
          comments: post.comments,
          created_at: post.created_at,
          tags: post.tags
        }))
        setPostRankings(postData)

        setLoading(false)
      } catch (error) {
        console.error('加载排行榜数据失败:', error)
        setLoading(false)
      }
    }

    loadData()
  }, [activeTab])

  const tabs = [
    { id: 'users', label: '用户排行', icon: Trophy },
    { id: 'resources', label: '资源排行', icon: Download },
    { id: 'posts', label: '帖子排行', icon: TrendingUp }
  ]

  const getRankIcon = (rank: number) => {
    switch (rank) {
      case 1:
        return <Crown className="w-6 h-6 text-yellow-500" />
      case 2:
        return <Award className="w-6 h-6 text-gray-400" />
      case 3:
        return <Award className="w-6 h-6 text-amber-600" />
      default:
        return <span className="w-6 h-6 flex items-center justify-center text-lg font-bold text-muted-foreground">#{rank}</span>
    }
  }

  const renderUserRanking = () => (
    <div className="space-y-4">
      {userRankings.map((user, index) => (
        <motion.div
          key={user.id}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: index * 0.1 }}
          className={cn(
            "p-4 rounded-xl border backdrop-blur-sm transition-all duration-200",
            index < 3 
              ? "bg-gradient-to-r from-primary/5 via-accent/5 to-secondary/5 border-primary/20" 
              : "bg-card/50 border-border/50 hover:bg-card/80"
          )}
        >
          <div className="flex items-center gap-4">
            {/* 排名 */}
            <div className="flex-shrink-0">
              {getRankIcon(index + 1)}
            </div>

            {/* 头像 */}
            <div className="flex-shrink-0">
              <button
                onClick={() => navigate(`/user/${user.id}`)}
                className="rounded-full hover:ring-2 hover:ring-primary/30 transition-all duration-200"
              >
                <img
                  src={user.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=default'}
                  alt={user.nickname}
                  className="w-12 h-12 rounded-full border-2 border-primary/20"
                />
              </button>
            </div>

            {/* 用户信息 */}
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2">
                <h3 className="font-semibold text-foreground truncate">{user.nickname}</h3>
                <span className="text-xs text-muted-foreground">@{user.username}</span>
              </div>
              <div className="flex items-center gap-4 mt-1 text-sm text-muted-foreground">
                <span className="flex items-center gap-1">
                  <Star className="w-3 h-3" />
                  {user.score}
                </span>
                <span>{user.posts_count} 帖子</span>
                <span>{user.resources_count} 资源</span>
              </div>
            </div>

            {/* 统计信息 */}
            <div className="flex flex-col items-end gap-1">
              <div className="flex items-center gap-1 text-sm font-medium text-primary">
                <Heart className="w-3 h-3" />
                {user.likes_count}
              </div>
              <div className="text-xs text-muted-foreground">
                {user.followers_count} 关注者
              </div>
            </div>
          </div>
        </motion.div>
      ))}
    </div>
  )

  const renderResourceRanking = () => (
    <div className="space-y-4">
      {resourceRankings.map((resource, index) => (
        <motion.div
          key={resource.id}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: index * 0.1 }}
          className={cn(
            "p-4 rounded-xl border backdrop-blur-sm transition-all duration-200",
            index < 3 
              ? "bg-gradient-to-r from-primary/5 via-accent/5 to-secondary/5 border-primary/20" 
              : "bg-card/50 border-border/50 hover:bg-card/80"
          )}
        >
          <div className="flex items-start gap-4">
            {/* 排名 */}
            <div className="flex-shrink-0 mt-1">
              {getRankIcon(index + 1)}
            </div>

            <div className="flex-1 min-w-0">
              {/* 资源标题和分类 */}
              <div className="flex items-start justify-between gap-2">
                <button
                  onClick={() => navigate(`/resource/${resource.id}`)}
                  className="flex-1 text-left hover:text-primary transition-colors"
                >
                  <h3 className="font-semibold text-foreground line-clamp-2">{resource.title}</h3>
                </button>
                <span className="px-2 py-1 text-xs bg-primary/10 text-primary rounded-full whitespace-nowrap">
                  {resource.category}
                </span>
              </div>

              {/* 作者信息 */}
              <div className="flex items-center gap-2 mt-2">
                <button
                  onClick={() => navigate(`/user/${resource.author.id}`)}
                  className="rounded-full hover:ring-2 hover:ring-primary/30 transition-all duration-200"
                >
                  <img
                    src={resource.author.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=default'}
                    alt={resource.author.nickname}
                    className="w-6 h-6 rounded-full"
                  />
                </button>
                <button
                  onClick={() => navigate(`/user/${resource.author.id}`)}
                  className="text-sm text-muted-foreground hover:text-primary transition-colors"
                >
                  {resource.author.nickname}
                </button>
              </div>

              {/* 统计信息 */}
              <div className="flex items-center gap-4 mt-3 text-sm text-muted-foreground">
                <span className="flex items-center gap-1 font-medium text-primary">
                  <Download className="w-3 h-3" />
                  {resource.downloads}
                </span>
                <span className="flex items-center gap-1">
                  <Heart className="w-3 h-3" />
                  {resource.likes}
                </span>
                <span className="flex items-center gap-1">
                  <Star className="w-3 h-3" />
                  {resource.rating}
                </span>
              </div>
            </div>
          </div>
        </motion.div>
      ))}
    </div>
  )

  const renderPostRanking = () => (
    <div className="space-y-4">
      {postRankings.map((post, index) => (
        <motion.div
          key={post.id}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: index * 0.1 }}
          className={cn(
            "p-4 rounded-xl border backdrop-blur-sm transition-all duration-200",
            index < 3 
              ? "bg-gradient-to-r from-primary/5 via-accent/5 to-secondary/5 border-primary/20" 
              : "bg-card/50 border-border/50 hover:bg-card/80"
          )}
        >
          <div className="flex items-start gap-4">
            {/* 排名 */}
            <div className="flex-shrink-0 mt-1">
              {getRankIcon(index + 1)}
            </div>

            <div className="flex-1 min-w-0">
              {/* 帖子标题 */}
              <button
                onClick={() => navigate(`/post/${post.id}`)}
                className="text-left hover:text-primary transition-colors w-full mb-2"
              >
                <h3 className="font-semibold text-foreground line-clamp-2">{post.title}</h3>
              </button>

              {/* 标签 */}
              <div className="flex flex-wrap gap-1 mb-2">
                {post.tags.map((tag, tagIndex) => (
                  <span 
                    key={tagIndex}
                    className="px-2 py-1 text-xs bg-secondary/50 text-secondary-foreground rounded-full"
                  >
                    {tag}
                  </span>
                ))}
              </div>

              {/* 作者信息 */}
              <div className="flex items-center gap-2 mb-3">
                <button
                  onClick={() => navigate(`/user/${post.author.id}`)}
                  className="rounded-full hover:ring-2 hover:ring-primary/30 transition-all duration-200"
                >
                  <img
                    src={post.author.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=default'}
                    alt={post.author.nickname}
                    className="w-6 h-6 rounded-full"
                  />
                </button>
                <button
                  onClick={() => navigate(`/user/${post.author.id}`)}
                  className="text-sm text-muted-foreground hover:text-primary transition-colors"
                >
                  {post.author.nickname}
                </button>
              </div>

              {/* 统计信息 */}
              <div className="flex items-center gap-4 text-sm text-muted-foreground">
                <span className="flex items-center gap-1 font-medium text-primary">
                  <Eye className="w-3 h-3" />
                  {post.views}
                </span>
                <span className="flex items-center gap-1">
                  <Heart className="w-3 h-3" />
                  {post.likes}
                </span>
                <span className="flex items-center gap-1">
                  <MessageSquare className="w-3 h-3" />
                  {post.comments}
                </span>
              </div>
            </div>
          </div>
        </motion.div>
      ))}
    </div>
  )

  return (
    <div className="min-h-screen bg-gradient-to-br from-background via-background to-accent/5">
      {/* 头部 */}
      <div className="sticky top-0 z-10 bg-background/80 backdrop-blur-xl border-b border-border/50">
        <div className="px-6 py-4">
          <h1 className="text-2xl font-bold text-foreground flex items-center gap-2">
            <Trophy className="w-6 h-6 text-primary" />
            排行榜
          </h1>
          <p className="text-sm text-muted-foreground mt-1">社区活跃度排行</p>
        </div>

        {/* 标签切换 */}
        <div className="px-6 pb-4">
          <div className="flex bg-muted/30 rounded-xl p-1 backdrop-blur-sm">
            {tabs.map((tab) => {
              const Icon = tab.icon
              const isActive = activeTab === tab.id
              
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id as RankingType)}
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
        </div>
      </div>

      {/* 内容区域 */}
      <div className="px-6 py-4 pb-safe">
        <AnimatePresence mode="wait">
          {loading ? (
            <motion.div
              key="loading"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="flex items-center justify-center py-20"
            >
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
            </motion.div>
          ) : (
            <motion.div
              key={activeTab}
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.2 }}
            >
              {activeTab === 'users' && renderUserRanking()}
              {activeTab === 'resources' && renderResourceRanking()}
              {activeTab === 'posts' && renderPostRanking()}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  )
}

export default RankingScreen 