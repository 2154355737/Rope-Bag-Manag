<template>
  <div class="resource-detail-container">
    <!-- 统一动态背景（移植自 /home 页面） -->
    <div class="dynamic-background">
      <!-- 主背景渐变层 -->
      <div class="gradient-layer"></div>
      <!-- 几何形状 -->
      <div class="geometric-shapes">
        <div class="shape triangle-1"></div>
        <div class="shape triangle-2"></div>
        <div class="shape circle-1"></div>
        <div class="shape circle-2"></div>
        <div class="shape square-1"></div>
        <div class="shape circle-3"></div>
        <div class="shape triangle-3"></div>
        <div class="shape square-2"></div>
      </div>
      <!-- 浮动粒子 -->
      <div class="floating-particles">
        <div class="particle" v-for="n in 20" :key="n" :style="getParticleStyle(n)"></div>
      </div>
      <!-- 波纹效果 -->
      <div class="ripple-effects">
        <div class="ripple ripple-1"></div>
        <div class="ripple ripple-2"></div>
        <div class="ripple ripple-3"></div>
        <div class="ripple ripple-4"></div>
        <div class="ripple ripple-5"></div>
      </div>
    </div>

    <!-- 顶部导航栏 -->
    <header class="header">
      <div class="header-content">
        <div class="logo" @click="goToHome">
          <div class="logo-icon">📚</div>
          <div class="logo-text">
            <h1>资源社区</h1>
            <p>分享、发现、学习</p>
          </div>
        </div>
        
        <div class="actions">
          <ThemeSwitcher />
          <el-button 
            v-if="!isLoggedIn" 
            type="primary" 
            size="large"
            @click="goToLogin"
          >
            <el-icon><User /></el-icon>
            登录
          </el-button>
          <el-button 
            v-if="isLoggedIn" 
            type="success" 
            size="large"
            @click="goToAdmin"
          >
            <el-icon><Setting /></el-icon>
            管理后台
          </el-button>
        </div>
      </div>
    </header>

    <main class="main">
      <div class="main-content">
        <!-- 返回按钮 -->
        <el-button class="back-btn" @click="goToHome" type="default" plain>
          <el-icon><Back /></el-icon>
          返回列表
        </el-button>

        <div v-if="loading" class="loading-container">
          <el-skeleton :rows="10" animated />
        </div>
        
        <div v-else-if="!resource" class="not-found">
          <el-empty description="资源不存在或已被删除" />
          <el-button type="primary" @click="goToHome">返回首页</el-button>
        </div>
        
        <template v-else>
          <!-- 资源详情 -->
          <div class="resource-header">
            <div class="resource-title-area">
              <h1 class="resource-title">{{ resource.name }}</h1>
              <el-tag 
                v-if="categoryName" 
                class="resource-category" 
                size="large" 
                :color="getCategoryColor(resource.category_id)"
              >
                {{ categoryName }}
              </el-tag>
            </div>
            
            <div class="resource-meta">
              <span class="meta-item">
                <el-icon><User /></el-icon>
                作者: {{ resource.author }}
              </span>
              <span class="meta-item">
                <el-icon><Calendar /></el-icon>
                发布时间: {{ formatDate(resource.created_at) }}
              </span>
              <span class="meta-item">
                <el-icon><Download /></el-icon>
                下载次数: {{ resource.download_count }}
              </span>
              <span class="meta-item">
                <el-icon><Star /></el-icon>
                点赞数: {{ resource.like_count }}
              </span>
            </div>
          </div>
          
          <el-divider />
          
          <div class="resource-content">
            <div class="resource-description">
              <h2>资源描述</h2>
              <div class="description-content">
                {{ resource.description || '暂无描述信息' }}
              </div>
            </div>
            
            <div class="resource-actions">
              <el-button 
                type="primary" 
                size="large" 
                @click="downloadResource"
              >
                <el-icon><Download /></el-icon>
                下载资源
              </el-button>
              <el-button
                type="default"
                size="large"
                @click="likeResource"
                :loading="likeLoading"
              >
                <el-icon><Star /></el-icon>
                {{ hasLiked ? '已点赞' : '点赞' }}
              </el-button>
            </div>
          </div>
          
          <el-divider />
          
          <!-- 评论区 -->
          <div class="comments-section">
            <h2>用户评论 ({{ comments.length }})</h2>
            
            <!-- 评论列表 -->
            <div v-if="comments.length === 0" class="no-comments">
              <p>暂无评论，成为第一个评论的用户吧！</p>
            </div>
            
            <div v-else class="comment-list">
              <div v-for="comment in comments" :key="comment.id" class="comment-item">
                <div class="comment-header">
                  <el-avatar
                    :src="comment.author_avatar"
                    size="small"
                    class="comment-avatar"
                  >
                    {{ (comment.author_name || '匿').charAt(0) }}
                  </el-avatar>
                  <div class="comment-meta">
                    <span class="comment-author">
                      {{ comment.author_name || '匿名用户' }}
                      <el-tag v-if="isAuthor(comment)" size="small" type="warning" class="ml-1">作者</el-tag>
                      <el-tag v-if="comment.pinned" size="small" type="primary" class="ml-1">置顶</el-tag>
                    </span>
                    <el-tag v-if="comment.author_role" size="small" type="success" class="ml-1">{{ formatRole(comment.author_role) }}</el-tag>
                    <span v-if="comment.author_qq" class="comment-qq ml-1">QQ: {{ comment.author_qq }}</span>
                  </div>
                  <div class="comment-time">{{ formatDate(comment.created_at) }}</div>
                </div>
                <div class="comment-content">{{ comment.content }}</div>
                <div class="comment-actions">
                  <el-button 
                    link 
                    size="small" 
                    @click="likeComment(comment.id)"
                  >
                    <el-icon><Star /></el-icon>
                    点赞 ({{ comment.likes || 0 }})
                  </el-button>
                  <el-button 
                    v-if="isLoggedIn"
                    link 
                    size="small" 
                    @click="replyToComment(comment.id)"
                  >
                    <el-icon><ChatRound /></el-icon>
                    回复
                  </el-button>
                  <el-button 
                    v-if="canDeleteComment(comment)"
                    link 
                    size="small" 
                    @click="deleteComment(comment.id)"
                  >
                    <el-icon><Delete /></el-icon>
                    删除
                  </el-button>
                  <el-button 
                    v-if="canPinComment(comment)"
                    link 
                    size="small" 
                    @click="togglePinComment(comment)"
                  >
                    <el-icon><Top /></el-icon>
                    {{ comment.pinned ? '取消置顶' : '置顶' }}
                  </el-button>
                </div>
              </div>
            </div>
            
            <!-- 分页器 -->
            <div v-if="comments.length > 0" class="pagination">
              <el-pagination
                v-model:current-page="currentPage"
                v-model:page-size="pageSize"
                :total="totalComments"
                :page-sizes="[10, 20, 50]"
                layout="total, sizes, prev, pager, next, jumper"
                @size-change="handleSizeChange"
                @current-change="handleCurrentChange"
                background
              />
            </div>
            
            <!-- 评论输入框 -->
            <div class="comment-form">
              <h3>发表评论</h3>
              <el-alert
                v-if="!isLoggedIn"
                title="请先登录后再发表评论"
                type="warning"
                :closable="false"
                show-icon
              />
              <template v-else>
                <el-input
                  v-model="commentForm.content"
                  type="textarea"
                  :rows="4"
                  placeholder="请输入您的评论"
                  :maxlength="500"
                  show-word-limit
                />
                <div class="form-actions">
                  <el-button 
                    type="primary" 
                    @click="submitComment" 
                    :loading="commentSubmitting"
                    :disabled="!commentForm.content"
                  >
                    提交评论
                  </el-button>
                </div>
              </template>
            </div>
          </div>
        </template>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  User,
  Setting,
  Download,
  Star,
  Calendar,
  Back,
  ChatRound,
  Delete,
  Top
} from '@element-plus/icons-vue'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import { getUserInfo } from '@/utils/auth'
import { packageApi, type Package } from '@/api/packages'
import { commentApi, type Comment } from '@/api/comments'
import { categoryApi, type Category } from '@/api/categories'

const route = useRoute()
const router = useRouter()
const resourceId = computed(() => Number(route.params.id) || 0)

// 响应式状态
const loading = ref(true)
const resource = ref<Package | null>(null)
const comments = ref<Comment[]>([])
const categories = ref<Category[]>([])
const categoryName = ref('')
const totalComments = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)
const hasLiked = ref(false)
const likeLoading = ref(false)
const commentSubmitting = ref(false)

// 评论表单
const commentForm = reactive({
  content: '',
  parentId: null as number | null
})

// 计算属性
const isLoggedIn = computed(() => {
  return localStorage.getItem('isLoggedIn') === 'true'
})

// 获取当前用户ID
const currentUserId = computed(() => {
  const userInfo = getUserInfo()
  return userInfo ? userInfo.id : null
})

// 格式化日期
const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('zh-CN')
}

// 角色映射
const formatRole = (role?: string) => {
  if (!role) return ''
  const map: Record<string, string> = {
    admin: '管理员',
    moderator: '版主',
    elder: '长老',
    user: '用户'
  }
  return map[role] || role
}

// 方法
const goToHome = () => {
  router.push('/home')
}

const goToLogin = () => {
  router.push('/login')
}

const goToAdmin = () => {
  const user = getUserInfo()
  if (!user) {
    router.push('/login')
    return
  }
  
  if (user.role === 'admin' || user.role === 'moderator') {
    router.push('/admin')
  } else if (user.role === 'elder') {
    router.push('/elder')
  } else if (user.role === 'user') {
    router.push('/user')
  } else {
    router.push('/403')
  }
}

const loadResource = async () => {
  if (!resourceId.value) return
  
  try {
    loading.value = true
    const res = await packageApi.getPackage(resourceId.value)
    
    if (res.code === 0 && res.data) {
      resource.value = res.data
      
      // 加载分类名称
      if (resource.value.category_id) {
        await loadCategoryName(resource.value.category_id)
      }
    } else {
      ElMessage.error(res.message || '加载资源失败')
      resource.value = null
    }
  } catch (error) {
    console.error('加载资源出错:', error)
    ElMessage.error('加载资源时发生错误')
    resource.value = null
  } finally {
    loading.value = false
  }
}

const loadComments = async () => {
  if (!resourceId.value) return
  
  try {
    const res = await commentApi.getPackageComments(resourceId.value, {
      page: currentPage.value,
      size: pageSize.value
    })
    
    if (res.code === 0 && res.data) {
      comments.value = res.data.list || []
      totalComments.value = res.data.total || 0
    } else {
      console.warn('加载评论返回错误:', res.message)
      // 设置默认空评论状态
      comments.value = []
      totalComments.value = 0
    }
  } catch (error) {
    console.error('加载评论出错:', error)
    // 不显示错误信息给用户，只是显示空评论状态
    comments.value = []
    totalComments.value = 0
  }
}

const loadCategories = async () => {
  try {
    const res = await categoryApi.getCategories()
    if (res.code === 0 && res.data) {
      categories.value = res.data.list || []
    }
  } catch (error) {
    console.error('加载分类出错:', error)
  }
}

const loadCategoryName = async (categoryId: number) => {
  if (categories.value.length === 0) {
    await loadCategories()
  }
  
  const category = categories.value.find(c => c.id === categoryId)
  categoryName.value = category ? category.name : '未分类'
}

const getCategoryColor = (categoryId: any) => {
  if (!categoryId) return '#409EFF'
  
  const colorMap: { [key: string]: string } = {
    1: '#409EFF', // 蓝色
    2: '#67C23A', // 绿色
    3: '#E6A23C', // 黄色
    4: '#F56C6C', // 红色
    5: '#909399'  // 灰色
  }
  
  return colorMap[categoryId] || '#409EFF'
}

const downloadResource = async () => {
  if (!resourceId.value) return
  
  try {
    const res = await packageApi.downloadPackage(resourceId.value)
    if (res.code === 0) {
      // 处理下载链接
      if (res.data && typeof res.data === 'string') {
        window.open(res.data, '_blank')
      }
      
      ElMessage.success('下载开始')
      
      // 重新加载资源以更新下载计数
      loadResource()
    } else {
      ElMessage.error(res.message || '下载失败')
    }
  } catch (error) {
    console.error('下载失败:', error)
    ElMessage.error('下载资源时发生错误')
  }
}

const likeResource = async () => {
  if (!isLoggedIn.value) {
    ElMessage.warning('请先登录后再点赞')
    return
  }
  
  if (!resourceId.value) return
  
  try {
    likeLoading.value = true
    
    // 这里假设有点赞API，实际使用时需要替换为真实API
    // const res = await packageApi.likePackage(resourceId.value)
    const res = { code: 0, message: '点赞成功' } // 临时模拟
    
    if (res.code === 0) {
      ElMessage.success('点赞成功')
      hasLiked.value = true
      
      if (resource.value) {
        resource.value.like_count += 1
      }
    } else {
      ElMessage.error(res.message || '点赞失败')
    }
  } catch (error) {
    console.error('点赞失败:', error)
    ElMessage.error('点赞时发生错误')
  } finally {
    likeLoading.value = false
  }
}

const submitComment = async () => {
  if (!isLoggedIn.value) {
    ElMessage.warning('请先登录后再发表评论')
    return
  }
  
  if (!commentForm.content.trim()) {
    ElMessage.warning('评论内容不能为空')
    return
  }
  
  if (!resourceId.value) return
  
  try {
    commentSubmitting.value = true
    
    const res = await commentApi.createComment({
      content: commentForm.content.trim(),
      target_id: resourceId.value,
      target_type: 'Package',
      parent_id: commentForm.parentId ?? undefined
    })
    
    if (res.code === 0) {
      ElMessage.success('评论发布成功')
      commentForm.content = ''
      commentForm.parentId = null
      
      // 重新加载评论
      loadComments()
    } else {
      ElMessage.error(res.message || '发布评论失败')
    }
  } catch (error) {
    console.error('发布评论失败:', error)
    ElMessage.error('发布评论时发生错误')
  } finally {
    commentSubmitting.value = false
  }
}

const likeComment = async (commentId: number) => {
  try {
    const res = await commentApi.likeComment(commentId, true)
 
    if (res.code === 0) {
      ElMessage.success('点赞成功')
      
      // 更新评论的点赞数
      const comment = comments.value.find(c => c.id === commentId)
      if (comment) {
        comment.likes = res.data ?? (comment.likes + 1)
      }
    } else {
      ElMessage.error(res.message || '点赞失败')
    }
  } catch (error) {
    console.error('点赞失败:', error)
    ElMessage.error('点赞时发生错误')
  }
}

const replyToComment = (commentId: number) => {
  commentForm.parentId = commentId
  
  // 滚动到评论表单
  setTimeout(() => {
    document.querySelector('.comment-form')?.scrollIntoView({ behavior: 'smooth' })
  }, 100)
}

const deleteComment = async (commentId: number) => {
  try {
    await ElMessageBox.confirm('确定要删除此评论吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    const res = await commentApi.deleteComment(commentId)
    
    if (res.code === 0) {
      ElMessage.success('删除成功')
      
      // 重新加载评论
      loadComments()
    } else {
      ElMessage.error(res.message || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除评论失败:', error)
      ElMessage.error('删除评论时发生错误')
    }
  }
}

const canDeleteComment = (comment: Comment) => {
  if (!isLoggedIn.value) return false
  
  const userInfo = getUserInfo()
  if (!userInfo) return false
  
  // 管理员可以删除任何评论
  if (userInfo.role === 'admin' || userInfo.role === 'moderator') {
    return true
  }
  
  // 用户可以删除自己的评论
  return comment.user_id === userInfo.id
}

const canPinComment = (comment: Comment) => {
  if (!isLoggedIn.value) return false
  const userInfo = getUserInfo()
  if (!userInfo || !resource.value) return false
  // 管理员、元老或资源作者可以置顶评论
  return userInfo.role === 'Admin' || userInfo.role === 'Elder' || userInfo.username === resource.value.author
}

const togglePinComment = async (comment: Comment) => {
  try {
    const res = await commentApi.pinComment(comment.id, !comment.pinned)
    if (res.code === 0) {
      ElMessage.success(comment.pinned ? '取消置顶成功' : '置顶成功')
      comment.pinned = !comment.pinned
    } else {
      ElMessage.error(res.message || '操作失败')
    }
  } catch (error) {
    console.error('置顶失败:', error)
    ElMessage.error('置顶时发生错误')
  }
}

const isAuthor = (comment: Comment) => {
  if (!resource.value) return false
  console.log('检查作者身份:', {
    comment_author: comment.username,
    resource_author: resource.value.author,
    is_match: comment.username === resource.value.author
  })
  return comment.username === resource.value.author
}

const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  loadComments()
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
  loadComments()
}

// 计算粒子样式
const getParticleStyle = (index: number) => {
  const size = Math.random() * 4 + 2
  const left = Math.random() * 100
  const animationDelay = Math.random() * 20
  const animationDuration = Math.random() * 10 + 10
  
  return {
    width: `${size}px`,
    height: `${size}px`,
    left: `${left}%`,
    animationDelay: `${animationDelay}s`,
    animationDuration: `${animationDuration}s`
  }
}
</script>