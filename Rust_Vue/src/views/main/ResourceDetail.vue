<template>
  <div class="resource-detail-container">
    <!-- 动态背景 -->
    <div class="animated-background">
      <!-- 大型装饰性渐变球 -->
      <div class="gradient-orbs">
        <div class="orb orb-1"></div>
        <div class="orb orb-2"></div>
        <div class="orb orb-3"></div>
      </div>
      
      <!-- 脉冲波纹效果 -->
      <div class="pulse-waves">
        <div class="wave wave-1"></div>
        <div class="wave wave-2"></div>
      </div>
      
      <div class="background-shapes">
        <div class="shape shape-1"></div>
        <div class="shape shape-2"></div>
        <div class="shape shape-3"></div>
        <div class="shape shape-4"></div>
      </div>
      <div class="floating-particles">
        <div class="particle" v-for="n in 12" :key="n" :style="getParticleStyle(n)"></div>
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

// 生成粒子样式
const getParticleStyle = (index: number) => {
  const size = Math.random() * 6 + 3 // 3-9px
  const left = Math.random() * 100 // 0-100%
  const animationDelay = Math.random() * 12 // 0-12s
  const animationDuration = Math.random() * 8 + 12 // 12-20s
  
  return {
    width: `${size}px`,
    height: `${size}px`,
    left: `${left}%`,
    animationDelay: `${animationDelay}s`,
    animationDuration: `${animationDuration}s`
  }
}

// 初始化
onMounted(async () => {
  await Promise.all([
    loadResource(),
    loadComments(),
    loadCategories()
  ])
})
</script>

<style scoped>
.resource-detail-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #0f1419 0%, #1a202c 50%, #2d3748 100%);
  transition: background-color var(--transition-base);
  position: relative;
}

/* 为浅色主题添加不同的背景 */
:root:not(.dark) .resource-detail-container {
  background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 50%, #e2e8f0 100%);
}

/* 动态背景 */
.animated-background {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: -1;
  overflow: hidden;
}

.gradient-orbs {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  display: flex;
  justify-content: center;
  align-items: center;
}

.orb {
  position: absolute;
  border-radius: 50%;
  opacity: 0.1;
  animation: pulse 10s infinite ease-in-out;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(5px);
}

.orb-1 {
  width: 200px;
  height: 200px;
  background: radial-gradient(circle, rgba(64, 158, 255, 0.1), rgba(59, 130, 246, 0.05));
  animation-delay: -2s;
}

.orb-2 {
  width: 150px;
  height: 150px;
  background: radial-gradient(circle, rgba(103, 194, 58, 0.1), rgba(76, 175, 80, 0.05));
  animation-delay: -4s;
}

.orb-3 {
  width: 180px;
  height: 180px;
  background: radial-gradient(circle, rgba(230, 162, 60, 0.1), rgba(217, 119, 6, 0.05));
  animation-delay: -6s;
}

.pulse-waves {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  overflow: hidden;
}

.wave {
  position: absolute;
  border-radius: 50%;
  opacity: 0.1;
  animation: pulse-wave 15s infinite ease-in-out;
  border: 1px solid rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(5px);
}

.wave-1 {
  width: 300px;
  height: 300px;
  background: radial-gradient(circle, rgba(64, 158, 255, 0.05), rgba(59, 130, 246, 0.02));
  animation-delay: -1s;
}

.wave-2 {
  width: 250px;
  height: 250px;
  background: radial-gradient(circle, rgba(103, 194, 58, 0.05), rgba(76, 175, 80, 0.02));
  animation-delay: -5s;
}

.background-shapes {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.shape {
  position: absolute;
  background: linear-gradient(135deg, rgba(64, 158, 255, 0.08), rgba(59, 130, 246, 0.12));
  border-radius: 50%;
  animation: float 12s infinite ease-in-out;
  backdrop-filter: blur(8px);
  border: 1px solid rgba(64, 158, 255, 0.15);
}

.shape-1 {
  width: 80px;
  height: 80px;
  top: 15%;
  left: 15%;
  animation-delay: -1s;
}

.shape-2 {
  width: 120px;
  height: 120px;
  top: 60%;
  right: 15%;
  animation-delay: -3s;
}

.shape-3 {
  width: 100px;
  height: 100px;
  bottom: 25%;
  left: 40%;
  animation-delay: -5s;
}

.shape-4 {
  width: 90px;
  height: 90px;
  top: 35%;
  right: 30%;
  animation-delay: -7s;
}

.floating-particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  overflow: hidden;
}

.particle {
  position: absolute;
  background: radial-gradient(circle, rgba(64, 158, 255, 0.25), rgba(59, 130, 246, 0.08));
  border-radius: 50%;
  animation: float 18s infinite ease-in-out;
  box-shadow: 0 0 8px rgba(64, 158, 255, 0.2);
}

@keyframes float {
  0% {
    transform: translateY(0) translateX(0) scale(1) rotate(0deg);
    opacity: 0;
  }
  20% {
    opacity: 0.4;
  }
  50% {
    transform: translateY(-15px) translateX(8px) scale(1.05) rotate(180deg);
    opacity: 0.7;
  }
  80% {
    opacity: 0.4;
  }
  100% {
    transform: translateY(0) translateX(0) scale(1) rotate(360deg);
    opacity: 0;
  }
}

@keyframes pulse {
  0% {
    transform: scale(0.9);
    opacity: 0.5;
  }
  50% {
    transform: scale(1.1);
    opacity: 0.8;
  }
  100% {
    transform: scale(0.9);
    opacity: 0.5;
  }
}

@keyframes pulse-wave {
  0% {
    transform: scale(1);
    opacity: 0.1;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.2;
  }
  100% {
    transform: scale(1);
    opacity: 0.1;
  }
}

.header {
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
  padding: 16px 0;
  position: sticky;
  top: 0;
  z-index: 100;
  transition: all var(--transition-base);
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
}

.logo-icon {
  font-size: 32px;
}

.logo-text h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.logo-text p {
  margin: 4px 0 0 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.actions {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 0 8px;
}

.main {
  padding: 32px 0;
}

.main-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
}

.back-btn {
  margin-bottom: 24px;
}

.loading-container {
  padding: 40px;
  background: var(--bg-primary);
  border-radius: 12px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
}

.not-found {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 60px 0;
  gap: 24px;
}

.resource-header {
  margin-bottom: 24px;
}

.resource-title-area {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

.resource-title {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
}

.resource-category {
  padding: 8px 16px !important;
  font-size: 14px !important;
  font-weight: 500 !important;
  border-radius: 20px !important;
  border: none !important;
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light)) !important;
  color: #fff !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1) !important;
  transition: all 0.3s ease !important;
}

.resource-category:hover {
  transform: translateY(-1px) !important;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15) !important;
}

.resource-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  color: var(--text-secondary);
}

.resource-content {
  background: var(--bg-primary);
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 32px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  transition: all var(--transition-base);
  position: relative;
  overflow: hidden;
}

.resource-content::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, 
    transparent, 
    rgba(64, 158, 255, 0.08), 
    rgba(103, 194, 58, 0.08),
    transparent
  );
  transition: left 3s ease;
  animation: shimmer 8s infinite;
}

@keyframes shimmer {
  0% {
    left: -100%;
  }
  50% {
    left: 100%;
  }
  100% {
    left: 100%;
  }
}

.resource-description h2 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.description-content {
  font-size: 16px;
  line-height: 1.6;
  color: var(--text-secondary);
  white-space: pre-line;
}

.resource-actions {
  margin-top: 24px;
  display: flex;
  gap: 12px;
}

.comments-section {
  background: var(--bg-primary);
  border-radius: 12px;
  padding: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  transition: all var(--transition-base);
  position: relative;
}

.comments-section::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  border-radius: 12px;
  border: 2px solid transparent;
  background: linear-gradient(45deg, 
    rgba(64, 158, 255, 0.2), 
    rgba(103, 194, 58, 0.2), 
    rgba(230, 162, 60, 0.2),
    rgba(64, 158, 255, 0.2)
  ) border-box;
  mask: linear-gradient(#fff 0 0) padding-box, linear-gradient(#fff 0 0);
  mask-composite: exclude;
  opacity: 0;
  transition: opacity 0.3s ease;
  animation: commentPulse 6s infinite ease-in-out;
}

.comments-section:hover::after {
  opacity: 0.5;
}

@keyframes commentPulse {
  0% {
    opacity: 0.1;
  }
  50% {
    opacity: 0.3;
  }
  100% {
    opacity: 0.1;
  }
}

.comments-section h2 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 24px 0;
}

.no-comments {
  text-align: center;
  padding: 40px 0;
  color: var(--text-muted);
}

.comment-list {
  margin-bottom: 24px;
}

.comment-item {
  padding: 16px;
  border-bottom: 1px solid var(--border-color-light);
  transition: all var(--transition-base);
}

.comment-item:last-child {
  border-bottom: none;
}

.comment-header {
  display: flex;
  align-items: center; /* Added for avatar alignment */
  justify-content: space-between;
  margin-bottom: 8px;
}

.comment-avatar {
  margin-right: 12px;
}

.comment-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-grow: 1; /* Allow meta to grow and take available space */
}

.comment-author {
  font-weight: 500;
  color: var(--text-primary);
}

.comment-qq {
  font-size: 12px;
  color: var(--text-secondary);
}

.comment-time {
  font-size: 12px;
  color: var(--text-muted);
}

.comment-content {
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-secondary);
  margin-bottom: 12px;
  white-space: pre-line;
}

.comment-actions {
  display: flex;
  gap: 16px;
}

.pagination {
  display: flex;
  justify-content: center;
  margin: 24px 0;
}

.comment-form {
  padding-top: 24px;
  border-top: 1px solid var(--border-color);
}

.comment-form h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.form-actions {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .resource-title-area {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .resource-meta {
    flex-direction: column;
    gap: 8px;
  }
  
  .resource-actions {
    flex-direction: column;
  }
}
</style> 