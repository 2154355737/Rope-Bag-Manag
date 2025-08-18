import React from 'react'
import { motion } from 'framer-motion'
import { Settings, Edit, LogOut, BookOpen, Heart, Bookmark, ChevronRight, Moon, Sun } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { useTheme } from '@/components/theme-provider'

const ProfileScreen: React.FC = () => {
  const { theme, setTheme } = useTheme()
  
  const userStats = {
    posts: 12,
    followers: 156,
    following: 89,
    likes: 328,
  }
  
  const learningData = {
    totalHours: 86,
    completedCourses: 7,
    currentStreak: 12,
    achievements: [
      { id: 1, name: '初学者', icon: '🌱', description: '完成第一个课程' },
      { id: 2, name: '勤奋学习', icon: '📚', description: '连续学习7天' },
      { id: 3, name: '代码大师', icon: '💻', description: '完成5个项目' },
    ],
    weeklyProgress: [2, 1.5, 3, 0, 2.5, 4, 1],
  }
  
  const userContent = {
    posts: [
      {
        id: 1,
        title: '结绳语言异步编程实践',
        image: 'https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8Y29kaW5nfGVufDB8fDB8fHww',
        likes: 42,
        comments: 8,
      },
      {
        id: 2,
        title: '我的结绳学习心得',
        image: 'https://images.unsplash.com/photo-1542831371-29b0f74f9713?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8Y29kaW5nfGVufDB8fDB8fHww',
        likes: 36,
        comments: 15,
      },
    ],
    favorites: [
      {
        id: 3,
        title: '结绳高级特性详解',
        image: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8M3x8Y29kaW5nfGVufDB8fDB8fHww',
        author: '李教授',
        likes: 156,
      },
      {
        id: 4,
        title: '结绳性能优化指南',
        image: 'https://images.unsplash.com/photo-1551033406-611cf9a28f67?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTJ8fGNvZGluZ3xlbnwwfHwwfHx8MA%3D%3D',
        author: '王工程师',
        likes: 89,
      },
    ],
  }

  return (
    <div className="flex flex-col min-h-screen bg-background pb-16">
      {/* 顶部导航栏 */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-bold">个人中心</h1>
          
          <Button variant="ghost" size="icon">
            <Settings size={20} />
          </Button>
        </div>
      </header>

      {/* 用户信息 */}
      <div className="p-4 border-b">
        <div className="flex items-center">
          <Avatar className="h-20 w-20 mr-4">
            <AvatarImage src="https://i.pravatar.cc/150?img=5" />
            <AvatarFallback>用户</AvatarFallback>
          </Avatar>
          
          <div className="flex-1">
            <div className="flex items-center justify-between">
              <h2 className="text-xl font-bold">程序员小王</h2>
              <Button variant="outline" size="sm" className="flex items-center">
                <Edit size={14} className="mr-1" /> 编辑
              </Button>
            </div>
            <p className="text-muted-foreground text-sm mb-2">结绳语言爱好者，专注移动开发</p>
            <Badge variant="outline">Lv.3 进阶开发者</Badge>
          </div>
        </div>
        
        <div className="grid grid-cols-4 mt-4 text-center">
          <div>
            <div className="font-bold">{userStats.posts}</div>
            <div className="text-xs text-muted-foreground">发布</div>
          </div>
          <div>
            <div className="font-bold">{userStats.followers}</div>
            <div className="text-xs text-muted-foreground">粉丝</div>
          </div>
          <div>
            <div className="font-bold">{userStats.following}</div>
            <div className="text-xs text-muted-foreground">关注</div>
          </div>
          <div>
            <div className="font-bold">{userStats.likes}</div>
            <div className="text-xs text-muted-foreground">获赞</div>
          </div>
        </div>
      </div>

      {/* 学习数据 */}
      <div className="p-4 border-b">
        <h3 className="text-lg font-medium mb-3">学习数据</h3>
        
        <div className="grid grid-cols-3 gap-2 mb-4">
          <Card>
            <CardContent className="p-3 text-center">
              <div className="text-2xl font-bold text-primary">{learningData.totalHours}</div>
              <div className="text-xs text-muted-foreground">总学时</div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-3 text-center">
              <div className="text-2xl font-bold text-primary">{learningData.completedCourses}</div>
              <div className="text-xs text-muted-foreground">完成课程</div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-3 text-center">
              <div className="text-2xl font-bold text-primary">{learningData.currentStreak}</div>
              <div className="text-xs text-muted-foreground">连续学习</div>
            </CardContent>
          </Card>
        </div>
        
        <Card className="mb-4">
          <CardContent className="p-3">
            <h4 className="text-sm font-medium mb-2">本周学习时长</h4>
            <div className="flex items-end h-20 gap-1">
              {learningData.weeklyProgress.map((hours, index) => (
                <div 
                  key={index}
                  className="flex-1 bg-primary rounded-t"
                  style={{ 
                    height: `${(hours / 4) * 100}%`,
                    opacity: hours ? undefined : 0.3
                  }}
                />
              ))}
            </div>
            <div className="flex justify-between text-xs text-muted-foreground mt-1">
              <span>一</span>
              <span>二</span>
              <span>三</span>
              <span>四</span>
              <span>五</span>
              <span>六</span>
              <span>日</span>
            </div>
          </CardContent>
        </Card>
        
        <h4 className="text-sm font-medium mb-2">成就徽章</h4>
        <div className="flex gap-3 overflow-x-auto pb-2">
          {learningData.achievements.map((achievement) => (
            <div key={achievement.id} className="flex flex-col items-center min-w-[60px]">
              <div className="flex items-center justify-center w-12 h-12 rounded-full bg-primary/10 mb-1">
                <span className="text-2xl">{achievement.icon}</span>
              </div>
              <div className="text-xs text-center">{achievement.name}</div>
            </div>
          ))}
        </div>
      </div>

      {/* 内容管理 */}
      <div className="p-4 flex-1">
        <Tabs defaultValue="posts" className="w-full">
          <TabsList className="grid grid-cols-3 mb-4">
            <TabsTrigger value="posts">我的发布</TabsTrigger>
            <TabsTrigger value="favorites">我的收藏</TabsTrigger>
            <TabsTrigger value="likes">我的点赞</TabsTrigger>
          </TabsList>
          
          <TabsContent value="posts" className="mt-0">
            <div className="grid grid-cols-2 gap-3">
              {userContent.posts.map((post) => (
                <motion.div
                  key={post.id}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                >
                  <Card className="overflow-hidden">
                    <img 
                      src={post.image} 
                      alt={post.title}
                      className="w-full h-24 object-cover"
                    />
                    <CardContent className="p-2">
                      <h4 className="text-sm font-medium line-clamp-1">{post.title}</h4>
                      <div className="flex items-center justify-between text-xs text-muted-foreground mt-1">
                        <div className="flex items-center">
                          <Heart size={12} className="mr-1" />
                          <span>{post.likes}</span>
                        </div>
                        <div className="flex items-center">
                          <BookOpen size={12} className="mr-1" />
                          <span>{post.comments}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
          </TabsContent>
          
          <TabsContent value="favorites" className="mt-0">
            <div className="grid grid-cols-2 gap-3">
              {userContent.favorites.map((favorite) => (
                <motion.div
                  key={favorite.id}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                >
                  <Card className="overflow-hidden">
                    <img 
                      src={favorite.image} 
                      alt={favorite.title}
                      className="w-full h-24 object-cover"
                    />
                    <CardContent className="p-2">
                      <h4 className="text-sm font-medium line-clamp-1">{favorite.title}</h4>
                      <div className="flex items-center justify-between text-xs text-muted-foreground mt-1">
                        <span>{favorite.author}</span>
                        <div className="flex items-center">
                          <Heart size={12} className="mr-1" />
                          <span>{favorite.likes}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </motion.div>
              ))}
            </div>
          </TabsContent>
          
          <TabsContent value="likes" className="mt-0">
            <div className="flex items-center justify-center h-40 text-muted-foreground">
              暂无点赞内容
            </div>
          </TabsContent>
        </Tabs>
      </div>

      {/* 设置选项 */}
      <div className="p-4 border-t">
        <div className="flex items-center justify-between py-2">
          <div className="flex items-center">
            {theme === 'dark' ? <Moon size={18} className="mr-2" /> : <Sun size={18} className="mr-2" />}
            <span>深色模式</span>
          </div>
          <Switch 
            checked={theme === 'dark'}
            onCheckedChange={(checked) => setTheme(checked ? 'dark' : 'light')}
          />
        </div>
        
        <Button variant="outline" className="w-full mt-4 text-destructive">
          <LogOut size={16} className="mr-2" /> 退出登录
        </Button>
      </div>
    </div>
  )
}

export default ProfileScreen