<template>
  <div class="resource-detail-page">
    <!-- 顶部导航栏 -->
    <header class="page-header">
      <div class="header-container">
        <div class="nav-left">
          <el-button 
            @click="$router.go(-1)" 
            type="primary" 
            plain
            size="large"
            class="back-button"
          >
            <el-icon><ArrowLeft /></el-icon>
            返回
          </el-button>
        </div>
        
        <div class="nav-center">
          <div class="breadcrumb">
            <router-link to="/" class="breadcrumb-item home">
              <el-icon><HomeFilled /></el-icon>
            </router-link>
            <el-icon class="breadcrumb-separator"><ArrowRight /></el-icon>
            <router-link to="/home" class="breadcrumb-item">首页</router-link>
            <el-icon class="breadcrumb-separator"><ArrowRight /></el-icon>
            <span class="breadcrumb-item current">{{ resource?.name || '资源详情' }}</span>
          </div>
        </div>
        
        <div class="nav-right">
          <el-dropdown trigger="click" v-if="resource">
            <el-button type="primary" plain size="large" class="action-button">
              操作
              <el-icon><CaretBottom /></el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item @click="downloadResource">
                  <el-icon><Download /></el-icon>下载资源
                </el-dropdown-item>
                <el-dropdown-item @click="toggleLike">
                  <el-icon><Star /></el-icon>{{ hasLiked ? '取消点赞' : '点赞' }}
                </el-dropdown-item>
                <el-dropdown-item @click="shareResource">
                  <el-icon><Share /></el-icon>分享
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
          <ThemeSwitcher />
        </div>
      </div>
    </header>

    <!-- 主内容区域 -->
    <main class="main-content">
      <div class="content-container">
        <!-- 加载状态 -->
        <div v-if="loading" class="loading-container">
          <div class="loading-card">
            <el-skeleton :rows="8" animated />
          </div>
        </div>
        
        <!-- 资源不存在 -->
        <div v-else-if="!resource" class="error-container">
          <div class="error-card">
            <div class="error-icon">
              <el-icon><DocumentDelete /></el-icon>
            </div>
            <h3 class="error-title">资源不存在</h3>
            <p class="error-description">抱歉，您访问的资源不存在或已被删除</p>
            <el-button type="primary" @click="$router.push('/home')">
              返回首页
            </el-button>
          </div>
        </div>
        
        <!-- 资源内容 -->
        <template v-else>
          <!-- 资源主要信息卡片 -->
          <div class="resource-main-section">
            <div class="resource-hero-card">
              <!-- 资源头部信息 -->
            <div class="resource-header">
                <div class="resource-title-area">
                  <h1 class="resource-title">{{ resource.name }}</h1>
                  <div class="resource-badges">
                    <el-tag 
                      v-if="categoryName" 
                      :type="getCategoryTagType(resource.category_id)"
                      size="large" 
                      effect="light"
                      class="category-badge"
                    >
                      {{ categoryName }}
                    </el-tag>
                    <el-tag 
                      v-if="resource.status === 'featured'"
                      type="warning"
                      size="large"
                      effect="light"
                    >
                      精选
                    </el-tag>
                  </div>
                </div>
                
                <!-- 资源统计信息 -->
                <div class="resource-stats">
                  <div class="stat-item">
                    <div class="stat-icon">
                      <el-icon><View /></el-icon>
                    </div>
                    <div class="stat-content">
                      <span class="stat-value">{{ formatNumber(resource.view_count) }}</span>
                      <span class="stat-label">浏览</span>
                    </div>
                  </div>
                  
                  <div class="stat-item">
                    <div class="stat-icon">
                      <el-icon><Download /></el-icon>
                    </div>
                    <div class="stat-content">
                      <span class="stat-value">{{ formatNumber(resource.download_count) }}</span>
                      <span class="stat-label">下载</span>
                    </div>
                  </div>
                  
                  <div class="stat-item">
                    <div class="stat-icon">
                      <el-icon><Star /></el-icon>
                    </div>
                    <div class="stat-content">
                      <span class="stat-value">{{ formatNumber(resource.like_count) }}</span>
                      <span class="stat-label">点赞</span>
                    </div>
                  </div>
                </div>
              </div>
              
              <!-- 资源描述 -->
              <div class="resource-description">
                <div class="description-header">
                  <h3 class="section-title">
                    <el-icon><Document /></el-icon>
                    资源描述
                  </h3>
                </div>
                <div class="description-content">
                  <p v-if="resource.description" class="description-text">
                    {{ resource.description }}
                  </p>
                  <p v-else class="description-empty">
                    暂无描述信息
                  </p>
                </div>
              </div>

              <!-- 资源详细信息 -->
              <div class="resource-details">
                <div class="details-grid">
                  <div class="detail-item">
                    <div class="detail-icon">
                      <el-icon><User /></el-icon>
                    </div>
                    <div class="detail-content">
                      <span class="detail-label">上传者</span>
                      <span class="detail-value">{{ resource.uploader_name || '未知' }}</span>
                    </div>
                  </div>
                  
                  <div class="detail-item">
                    <div class="detail-icon">
                      <el-icon><Calendar /></el-icon>
                    </div>
                    <div class="detail-content">
                      <span class="detail-label">上传时间</span>
                      <span class="detail-value">{{ formatDate(resource.created_at) }}</span>
                    </div>
                  </div>
                  
                  <div class="detail-item" v-if="resource.file_size">
                    <div class="detail-icon">
                      <el-icon><Files /></el-icon>
                    </div>
                    <div class="detail-content">
                      <span class="detail-label">文件大小</span>
                      <span class="detail-value">{{ formatFileSize(resource.file_size) }}</span>
                    </div>
                  </div>
                  
                  <div class="detail-item">
                    <div class="detail-icon">
                      <el-icon><Refresh /></el-icon>
                    </div>
                    <div class="detail-content">
                      <span class="detail-label">更新时间</span>
                      <span class="detail-value">{{ formatDate(resource.updated_at) }}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 操作按钮区域 -->
              <div class="resource-actions">
                <el-button 
                  type="primary" 
                  size="large" 
                  @click="downloadResource"
                  :loading="downloadLoading"
                  class="primary-action"
                >
                  <el-icon><Download /></el-icon>
                  {{ downloadLoading ? '下载中...' : '立即下载' }}
                </el-button>
                
                <el-button
                  :type="hasLiked ? 'warning' : 'default'"
                  size="large"
                  @click="toggleLike"
                  :loading="likeLoading"
                  class="secondary-action"
                >
                  <el-icon><Star /></el-icon>
                  {{ hasLiked ? '已点赞' : '点赞' }}
                </el-button>
                
                <el-button
                  type="default"
                  size="large"
                  @click="shareResource"
                  class="secondary-action"
                >
                  <el-icon><Share /></el-icon>
                  分享
                </el-button>
              </div>
            </div>
          </div>
          
          <!-- 评论区域 -->
          <div class="comments-section">
            <div class="comments-card">
              <div class="comments-header">
                <h3 class="section-title">
                  <el-icon><ChatDotRound /></el-icon>
                用户评论
                <span class="comment-count">({{ comments.length }})</span>
                </h3>
            
                <!-- 发表评论 -->
                <div v-if="isLoggedIn" class="comment-form">
                    <el-input
                    v-model="newComment"
                      type="textarea"
                    :rows="3"
                    placeholder="写下你的评论..."
                    maxlength="500"
                      show-word-limit
                    class="comment-input"
                    />
                  <div class="comment-actions">
                    <el-button 
                      type="primary" 
                      @click="submitComment" 
                      :loading="commentLoading"
                      :disabled="!newComment.trim()"
                    >
                      发表评论
                    </el-button>
                  </div>
                </div>
                
                <div v-else class="login-prompt">
                  <p>请先 <router-link to="/login" class="login-link">登录</router-link> 后发表评论</p>
              </div>
            </div>
            
            <!-- 评论列表 -->
            <div class="comments-list">
              <div v-if="comments.length === 0" class="empty-comments">
                  <el-empty description="暂无评论" :image-size="80" />
              </div>
              
                <div v-else>
                  <div 
                    v-for="comment in comments" 
                    :key="comment.id"
                    class="comment-item"
                  >
                    <div class="comment-avatar">
                      <el-avatar :size="40">
                        {{ comment.user_name?.charAt(0) || 'U' }}
                      </el-avatar>
                    </div>
                    
                    <div class="comment-content">
                      <div class="comment-header">
                        <span class="comment-author">{{ comment.user_name }}</span>
                        <span class="comment-time">{{ formatDate(comment.created_at) }}</span>
                    </div>
                      <div class="comment-text">
                        {{ comment.content }}
                  </div>
                </div>
              </div>
            </div>
              </div>
            </div>
          </div>
        </template>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  ArrowLeft,
  ArrowRight,
  DocumentDelete,
  View,
  Download,
  Star,
  Share,
  Document,
  User,
  Calendar,
  Files,
  Refresh,
  ChatDotRound,
  HomeFilled,
  CaretBottom
} from '@element-plus/icons-vue'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import { packageApi, commentApi } from '@/api'
import { getUserInfo } from '@/utils/auth'
import { formatDate, formatFileSize } from '@/utils/format'

// 格式化数字显示
const formatNumber = (num: number): string => {
  if (num < 1000) return num.toString()
  if (num < 10000) return (num / 1000).toFixed(1) + 'K'
  if (num < 1000000) return (num / 10000).toFixed(1) + '万'
  return (num / 1000000).toFixed(1) + 'M'
}

const route = useRoute()
const router = useRouter()

// 响应式数据
const loading = ref(true)
const downloadLoading = ref(false)
const likeLoading = ref(false)
const commentLoading = ref(false)
const resource = ref<any>(null)
const comments = ref<any[]>([])
const newComment = ref('')
const hasLiked = ref(false)

// 计算属性
const isLoggedIn = computed(() => !!getUserInfo())
const categoryName = computed(() => {
  // 这里需要根据实际的分类数据结构调整
  return resource.value?.category_name || '未分类'
})

// 方法
const loadResource = async () => {
  const resourceId = route.params.id as string
  if (!resourceId) {
  router.push('/home')
    return
  }
  
  try {
    loading.value = true
    const response = await packageApi.getPackage(parseInt(resourceId))
    
    if (response.code === 0 && response.data) {
      resource.value = response.data
      await loadComments()
      await checkLikeStatus()
    } else {
      resource.value = null
    }
  } catch (error) {
    console.error('加载资源失败:', error)
    resource.value = null
  } finally {
    loading.value = false
  }
}

const loadComments = async () => {
  if (!resource.value) return
  
  try {
    const response = await commentApi.getComments(resource.value.id)
    if (response.code === 0 && response.data) {
      comments.value = response.data.list || []
    }
  } catch (error) {
    console.error('加载评论失败:', error)
  }
}

const checkLikeStatus = async () => {
  if (!resource.value || !isLoggedIn.value) return
  
  try {
    // 这里暂时设为false，实际项目中需要实现对应的API
    hasLiked.value = false
  } catch (error) {
    console.error('检查点赞状态失败:', error)
  }
}

const downloadResource = async () => {
  if (!resource.value) return
  
  try {
    downloadLoading.value = true
    const response = await packageApi.downloadPackage(resource.value.id)
    
    if (response.code === 0) {
      // 处理文件下载
      if (response.data && typeof response.data === 'string') {
        window.open(response.data, '_blank')
      }
      
      ElMessage.success('下载开始')
      // 重新加载资源以更新下载计数
      loadResource()
    } else {
      ElMessage.error(response.message || '下载失败')
    }
  } catch (error) {
    console.error('下载失败:', error)
    ElMessage.error('下载失败，请重试')
  } finally {
    downloadLoading.value = false
  }
}

const toggleLike = async () => {
  if (!resource.value) return
  
  if (!isLoggedIn.value) {
    ElMessage.warning('请先登录')
    router.push('/login')
    return
  }
  
  try {
    likeLoading.value = true
    // 这里暂时模拟点赞功能，实际项目中需要实现对应的API
    hasLiked.value = !hasLiked.value
    resource.value.like_count += hasLiked.value ? 1 : -1
    ElMessage.success(hasLiked.value ? '点赞成功' : '取消点赞')
  } catch (error) {
    console.error('点赞操作失败:', error)
    ElMessage.error('操作失败，请重试')
  } finally {
    likeLoading.value = false
  }
}

const shareResource = async () => {
  if (!resource.value) return
  
  const shareUrl = window.location.href
  const shareText = `${resource.value.name} - 智圆社区`
  
  if (navigator.share) {
    try {
      await navigator.share({
        title: shareText,
        url: shareUrl
      })
  } catch (error) {
      console.log('分享取消')
      }
    } else {
    // 复制到剪贴板
    try {
      await navigator.clipboard.writeText(shareUrl)
      ElMessage.success('链接已复制到剪贴板')
  } catch (error) {
      ElMessage.error('复制失败')
    }
  }
}

const submitComment = async () => {
  if (!resource.value || !newComment.value.trim()) return
  
  try {
    commentLoading.value = true
    const response = await commentApi.createComment({
      target_id: resource.value.id,
      target_type: 'Package',
      content: newComment.value.trim()
    })
    
    if (response.code === 0) {
      ElMessage.success('评论发表成功')
      newComment.value = ''
      await loadComments()
    } else {
      ElMessage.error(response.message || '评论发表失败')
    }
  } catch (error) {
    console.error('发表评论失败:', error)
    ElMessage.error('评论发表失败，请重试')
  } finally {
    commentLoading.value = false
  }
}

const getCategoryTagType = (categoryId: number) => {
  // 根据分类ID返回不同的标签类型
  const typeMap: Record<number, string> = {
    1: 'primary',
    2: 'success',
    3: 'warning',
    4: 'danger',
    5: 'info'
  }
  return typeMap[categoryId] || 'default'
}

// 生命周期
onMounted(() => {
  loadResource()
})
</script>

<style scoped>
.resource-detail-page {
  min-height: 100vh;
  background: var(--bg-primary);
}

/* ===== 页面头部 ===== */
.page-header {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 64px;
  background: var(--bg-elevated);
  box-shadow: var(--shadow-sm);
  z-index: 100;
  border-bottom: 1px solid var(--border-color-light);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: var(--transition-normal);
}

.page-header:hover {
  box-shadow: var(--shadow-md);
}

.header-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: var(--max-width-xl);
  height: 100%;
  margin: 0 auto;
  padding: 0 var(--space-6);
}

.nav-left {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.back-button {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-weight: 600;
  border-radius: var(--radius-lg);
  padding: var(--space-2) var(--space-4);
  transition: var(--transition-fast);
}

.back-button:hover {
  transform: translateX(-4px);
}

.nav-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--font-size-sm);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 600px;
}

.breadcrumb-item {
  color: var(--text-secondary);
  transition: var(--transition-fast);
  font-weight: 500;
  display: flex;
  align-items: center;
}

.breadcrumb-item:hover {
  color: var(--color-primary);
}

.breadcrumb-item.home {
  font-size: var(--font-size-lg);
}

.breadcrumb-item.current {
  color: var(--text-primary);
  font-weight: 600;
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.breadcrumb-separator {
  color: var(--text-tertiary);
  font-size: var(--font-size-xs);
}

.nav-right {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.action-button {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-weight: 600;
  border-radius: var(--radius-lg);
}

/* 调整主内容区域，避免被固定导航栏遮挡 */
.main-content {
  padding-top: 80px; /* 64px 导航栏 + 额外空间 */
}

.content-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 var(--space-6);
  display: flex;
  flex-direction: column;
  gap: var(--space-8);
}

/* ===== 加载和错误状态 ===== */
.loading-container,
.error-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

.loading-card,
.error-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-8);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
  text-align: center;
  max-width: 400px;
  width: 100%;
}

.error-icon {
  font-size: 48px;
  color: var(--color-danger);
  margin-bottom: var(--space-4);
}

.error-title {
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--space-2) 0;
}

.error-description {
  color: var(--text-secondary);
  margin: 0 0 var(--space-6) 0;
  line-height: 1.5;
}

/* ===== 资源主要信息卡片 ===== */
.resource-main-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.resource-hero-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-8);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: var(--space-8);
}

.resource-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.resource-title-area {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.resource-title {
  font-size: var(--font-size-3xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.2;
}

.resource-badges {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}

.category-badge {
  font-weight: 600;
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius-lg);
}

.resource-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: var(--space-4);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-4);
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color-light);
  transition: var(--transition-normal);
}

.stat-item:hover {
  background: var(--bg-hover);
  border-color: var(--color-primary);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.stat-icon {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: var(--font-size-lg);
}

.stat-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.stat-value {
  font-size: var(--font-size-lg);
  font-weight: 700;
  color: var(--text-primary);
}

.stat-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

/* ===== 资源描述 ===== */
.resource-description {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.description-header {
  border-bottom: 1px solid var(--border-color-light);
  padding-bottom: var(--space-3);
}

.section-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.section-title .el-icon {
  color: var(--color-primary);
}

.description-content {
  padding: var(--space-4);
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color-light);
}

.description-text {
  font-size: var(--font-size-base);
  color: var(--text-primary);
  line-height: 1.6;
  margin: 0;
}

.description-empty {
  font-size: var(--font-size-base);
  color: var(--text-tertiary);
  font-style: italic;
  margin: 0;
}

/* ===== 资源详细信息 ===== */
.resource-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.details-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--space-4);
}

.detail-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-4);
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color-light);
  transition: var(--transition-normal);
}

.detail-item:hover {
  background: var(--bg-hover);
  border-color: var(--color-primary);
}

.detail-icon {
  width: 36px;
  height: 36px;
  background: var(--color-primary);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: var(--font-size-base);
  flex-shrink: 0;
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  flex: 1;
}

.detail-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.detail-value {
  font-size: var(--font-size-base);
  font-weight: 500;
  color: var(--text-primary);
}

/* ===== 操作按钮区域 ===== */
.resource-actions {
  display: flex;
  gap: var(--space-4);
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-color-light);
}

.primary-action {
  min-width: 160px;
  height: 48px;
  font-size: var(--font-size-base);
  font-weight: 600;
  border-radius: var(--radius-lg);
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border: none;
  box-shadow: var(--shadow-md);
  transition: var(--transition-normal);
}

.primary-action:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.secondary-action {
  min-width: 120px;
  height: 48px;
  font-size: var(--font-size-base);
  font-weight: 600;
  border-radius: var(--radius-lg);
  transition: var(--transition-normal);
}

.secondary-action:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

/* ===== 评论区域 ===== */
.comments-section {
  margin-top: var(--space-8);
}

.comments-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-8);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.comments-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.comment-count {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  font-weight: 400;
}

.comment-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.comment-input :deep(.el-textarea__inner) {
  border-radius: var(--radius-lg);
  border: 2px solid var(--border-color);
  transition: var(--transition-normal);
  font-size: var(--font-size-base);
  line-height: 1.5;
}

.comment-input :deep(.el-textarea__inner):focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.comment-actions {
  display: flex;
  justify-content: flex-end;
}

.login-prompt {
  text-align: center;
  padding: var(--space-6);
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color-light);
}

.login-prompt p {
  margin: 0;
  color: var(--text-secondary);
}

.login-link {
  color: var(--color-primary);
  text-decoration: none;
  font-weight: 600;
}

.login-link:hover {
  text-decoration: underline;
}

.comments-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.empty-comments {
  text-align: center;
  padding: var(--space-8);
}

.comment-item {
  display: flex;
  gap: var(--space-4);
  padding: var(--space-4);
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color-light);
  transition: var(--transition-normal);
}

.comment-item:hover {
  background: var(--bg-hover);
  border-color: var(--color-primary);
}

.comment-avatar {
  flex-shrink: 0;
}

.comment-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.comment-header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.comment-author {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.comment-time {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.comment-text {
  font-size: var(--font-size-base);
  color: var(--text-primary);
  line-height: 1.5;
}

/* ===== 响应式设计 ===== */
@media (max-width: 768px) {
  .header-container {
    padding: 0 var(--space-4);
}

  .nav-center {
    display: none;
}

  .content-container {
    padding: 0 var(--space-4);
  }
  
  .resource-hero-card {
    padding: var(--space-6);
  }
  
  .resource-title {
    font-size: var(--font-size-2xl);
  }
  
  .resource-stats {
    grid-template-columns: 1fr;
  }
  
  .details-grid {
    grid-template-columns: 1fr;
  }
  
  .resource-actions {
    flex-direction: column;
    gap: var(--space-3);
  }
  
  .primary-action,
  .secondary-action {
    width: 100%;
  }
  
  .comments-card {
    padding: var(--space-6);
  }
}

@media (max-width: 480px) {
  .header-container {
    padding: 0 var(--space-3);
  }
  
  .content-container {
    padding: 0 var(--space-3);
  }
  
  .resource-hero-card,
  .comments-card {
    padding: var(--space-4);
  }
  
  .resource-title {
    font-size: var(--font-size-xl);
  }
}
</style>