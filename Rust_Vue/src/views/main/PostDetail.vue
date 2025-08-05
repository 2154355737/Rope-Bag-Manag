<template>
  <div class="post-detail-container">
    <!-- 统一背景层（与Home页面保持一致） -->
    <div class="background-layer">
      <div class="gradient-bg"></div>
      <div class="pattern-overlay"></div>
    </div>

    <!-- 顶部导航栏 -->
    <header class="top-header">
      <div class="header-wrapper">
        <div class="header-left">
          <div class="brand" @click="$router.push('/home')">
            <div class="brand-icon">🏘️</div>
            <div class="brand-text">
              <h1>智圆社区</h1>
              <span>分享、发现、学习</span>
            </div>
          </div>
        </div>
        
        <div class="header-center">
          <el-button @click="$router.push('/home')" type="primary" plain class="back-btn">
            <el-icon><ArrowLeft /></el-icon>
            返回首页
          </el-button>
        </div>
        
        <div class="header-right">
          <div class="header-actions">
            <el-button 
              v-if="!isLoggedIn" 
              type="primary" 
              size="default"
              @click="goToLogin"
            >
              <el-icon><User /></el-icon>
              登录
            </el-button>
            <div v-if="isLoggedIn" class="user-menu">
              <el-dropdown trigger="click" placement="bottom-end">
                <div class="user-trigger">
                  <el-avatar :size="32">
                    <el-icon><User /></el-icon>
                  </el-avatar>
                  <el-icon class="dropdown-arrow"><ArrowDown /></el-icon>
                </div>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="goToAdmin">
                      <el-icon><Setting /></el-icon>
                      管理后台
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- 主内容区域 -->
    <main class="main-content">
      <div class="content-container">
        <div v-if="loading" class="loading-state">
          <div class="glass-panel">
            <el-skeleton :rows="8" animated />
          </div>
        </div>
        
        <template v-else-if="post">
          <!-- 帖子主体卡片 -->
          <article class="post-main-card glass-panel">
            <!-- 帖子头部 -->
            <header class="post-header">
              <div class="post-title-section">
                <div class="title-row">
                  <h1 class="post-title">{{ post.title }}</h1>
                  <div class="status-badges">
                    <el-tag v-if="post.is_pinned" type="warning" size="small" effect="dark">
                      <el-icon><Star /></el-icon>
                      置顶
                    </el-tag>
                    <el-tag v-if="post.is_featured" type="success" size="small" effect="dark">
                      <el-icon><Trophy /></el-icon>
                      精选
                    </el-tag>
                    <el-tag v-if="post.status === 'Draft'" type="info" size="small" effect="dark">
                      草稿
                    </el-tag>
                  </div>
                </div>
                
                <div class="post-meta-section">
                  <div class="author-info">
                    <el-avatar :size="48" class="author-avatar">
                      <el-icon><User /></el-icon>
                    </el-avatar>
                    <div class="author-details">
                      <div class="author-name">{{ post.author_name }}</div>
                      <div class="post-time">{{ formatTime(post.created_at) }}</div>
                    </div>
                  </div>
                  
                  <div class="post-stats">
                    <div class="stat-item">
                      <el-icon class="stat-icon"><View /></el-icon>
                      <span class="stat-value">{{ post.view_count }}</span>
                      <span class="stat-label">浏览</span>
                    </div>
                    <div class="stat-item">
                      <el-icon class="stat-icon"><ChatDotRound /></el-icon>
                      <span class="stat-value">{{ post.comment_count }}</span>
                      <span class="stat-label">评论</span>
                    </div>
                    <div class="stat-item">
                      <el-icon class="stat-icon"><Star /></el-icon>
                      <span class="stat-value">{{ post.like_count }}</span>
                      <span class="stat-label">点赞</span>
                    </div>
                  </div>
                </div>
                
                <!-- 标签 -->
                <div class="post-tags" v-if="tags.length > 0">
                  <el-tag
                    v-for="tag in tags"
                    :key="tag.id"
                    :color="tag.color"
                    class="tag-item"
                    effect="light"
                  >
                    {{ tag.name }}
                  </el-tag>
                </div>
              </div>
            </header>

            <!-- 帖子内容 -->
            <div class="post-body">
              <div class="content-wrapper">
                <div class="post-content" v-html="formatContent(post.content)"></div>
              </div>
            </div>

            <!-- 操作按钮 -->
            <div class="post-actions" v-if="isLoggedIn">
              <div class="action-group primary-actions">
                <el-button 
                  type="primary" 
                  size="large"
                  @click="handleLike"
                  :loading="liking"
                  class="action-btn like-btn"
                  :class="{ liked: isLiked }"
                >
                  <el-icon><Star /></el-icon>
                  {{ isLiked ? '已点赞' : '点赞' }}
                  <span class="action-count">({{ post.like_count }})</span>
                </el-button>
                
                <el-button 
                  type="default" 
                  size="large"
                  @click="focusCommentInput"
                  class="action-btn comment-btn"
                >
                  <el-icon><ChatDotRound /></el-icon>
                  评论
                  <span class="action-count">({{ post.comment_count }})</span>
                </el-button>
              </div>
              
              <div class="action-group secondary-actions" v-if="canEdit || canDelete">
                <el-button 
                  v-if="canEdit"
                  type="warning" 
                  size="large"
                  @click="handleEdit"
                  class="action-btn edit-btn"
                >
                  <el-icon><Edit /></el-icon>
                  编辑
                </el-button>
                
                <el-button 
                  v-if="canDelete"
                  type="danger" 
                  size="large"
                  @click="handleDelete"
                  class="action-btn delete-btn"
                >
                  <el-icon><Delete /></el-icon>
                  删除
                </el-button>
              </div>
            </div>
          </article>

          <!-- 评论区域 -->
          <section class="comments-section glass-panel">
            <div class="section-header">
              <h2 class="section-title">
                <el-icon><ChatDotRound /></el-icon>
                评论讨论
                <span class="comment-count">({{ post?.comment_count || 0 }})</span>
              </h2>
            </div>
            
            <!-- 发表评论 -->
            <div class="comment-form-container">
              <div class="comment-form glass-inner" v-if="isLoggedIn">
                <h3 class="form-title">发表评论</h3>
                <div class="comment-input-wrapper">
                  <el-input
                    ref="commentInputRef"
                    v-model="commentContent"
                    type="textarea"
                    :rows="4"
                    placeholder="写下你的评论..."
                    @keydown.ctrl.enter="handleSubmitComment"
                    class="comment-textarea"
                  />
                </div>
                <div class="form-actions">
                  <div class="comment-tip">
                    <el-icon><InfoFilled /></el-icon>
                    Ctrl + Enter 快速发表
                  </div>
                  <el-button 
                    type="primary" 
                    @click="handleSubmitComment"
                    :loading="submittingComment"
                    :disabled="!commentContent.trim()"
                    class="submit-btn"
                  >
                    发表评论
                  </el-button>
                </div>
              </div>
              
              <div v-else class="login-prompt glass-inner">
                <div class="prompt-content">
                  <el-icon class="prompt-icon"><User /></el-icon>
                  <div class="prompt-text">
                    <h4>登录后参与讨论</h4>
                    <p>登录后即可发表评论，与作者和其他用户互动</p>
                  </div>
                  <el-button type="primary" @click="goToLogin" class="login-btn">
                    立即登录
                  </el-button>
                </div>
              </div>
            </div>

            <!-- 评论列表 -->
            <div class="comments-list" v-loading="commentsLoading">
              <div v-if="comments.length === 0 && !commentsLoading" class="empty-comments">
                <el-empty description="暂无评论，来发表第一个评论吧！" :image-size="100" />
              </div>
              
              <div v-else class="comment-items">
                <div
                  v-for="comment in comments"
                  :key="comment.id"
                  class="comment-item glass-inner"
                >
                  <div class="comment-header">
                    <div class="comment-author-section">
                      <el-avatar :size="40" class="comment-avatar">
                        <el-icon><User /></el-icon>
                      </el-avatar>
                      <div class="comment-meta">
                        <div class="author-info">
                          <span class="comment-author-name">{{ comment.username }}</span>
                          <span class="comment-time">{{ formatTime(comment.created_at) }}</span>
                        </div>
                      </div>
                    </div>
                    
                    <div class="comment-actions" v-if="canDeleteComment(comment)">
                      <el-button 
                        type="danger" 
                        size="small"
                        @click="deleteComment(comment.id)"
                        link
                        class="delete-comment-btn"
                      >
                        <el-icon><Delete /></el-icon>
                        删除
                      </el-button>
                    </div>
                  </div>
                  
                  <div class="comment-content">
                    {{ comment.content }}
                  </div>
                </div>
              </div>
            </div>
          </section>
        </template>
      </div>
    </main>

    <!-- 编辑对话框 -->
    <el-dialog
      v-model="showEditDialog"
      title="编辑帖子"
      width="70%"
      class="edit-dialog"
    >
      <el-form ref="editFormRef" :model="editForm" :rules="editRules" label-width="80px" class="edit-form">
        <el-form-item label="标题" prop="title">
          <el-input v-model="editForm.title" placeholder="请输入帖子标题" size="large" />
        </el-form-item>
        
        <el-form-item label="内容" prop="content">
          <el-input
            v-model="editForm.content"
            type="textarea"
            :rows="10"
            placeholder="请输入帖子内容"
          />
        </el-form-item>
        
        <el-form-item label="标签">
          <el-select
            v-model="editForm.tags"
            multiple
            filterable
            allow-create
            placeholder="选择或创建标签"
            style="width: 100%"
          >
            <el-option
              v-for="tag in allTags"
              :key="tag.id"
              :label="tag.name"
              :value="tag.name"
            />
          </el-select>
        </el-form-item>
      </el-form>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showEditDialog = false" size="large">取消</el-button>
          <el-button type="primary" @click="handleUpdatePost" :loading="updating" size="large">
            保存修改
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  User, 
  View, 
  ChatDotRound, 
  Star, 
  Edit, 
  Delete, 
  ArrowLeft,
  ArrowDown,
  Setting,
  Trophy,
  InfoFilled
} from '@element-plus/icons-vue'
import { postApi, tagApi, commentApi } from '@/api'
import type { Post, UpdatePostRequest, Tag, Comment } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { formatDate } from '@/utils/format'

// 格式化时间函数
const formatTime = (timeStr: string) => {
  return formatDate(timeStr, 'YYYY-MM-DD HH:mm')
}

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

// 响应式数据
const post = ref<Post>()
const tags = ref<Tag[]>([])
const comments = ref<Comment[]>([])
const allTags = ref<Tag[]>([])
const loading = ref(false)
const commentsLoading = ref(false)
const liking = ref(false)
const isLiked = ref(false)
const submittingComment = ref(false)
const updating = ref(false)

// 评论相关
const commentInputRef = ref()
const commentContent = ref('')

// 编辑相关
const showEditDialog = ref(false)
const editFormRef = ref()
const editForm = reactive<UpdatePostRequest>({
  title: '',
  content: '',
  tags: []
})

const editRules = {
  title: [
    { required: true, message: '请输入标题', trigger: 'blur' },
    { min: 1, max: 200, message: '标题长度在 1 到 200 个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入内容', trigger: 'blur' },
    { min: 10, message: '内容至少 10 个字符', trigger: 'blur' }
  ]
}

// 计算属性
const isLoggedIn = computed(() => authStore.isLoggedIn)
const currentUser = computed(() => authStore.currentUser)
const postId = computed(() => parseInt(route.params.id as string))

const canEdit = computed(() => {
  if (!post.value || !currentUser.value.value) return false
  return post.value.author_id === currentUser.value.value?.id ||
         currentUser.value.value?.role === 'admin' ||
         currentUser.value.value?.role === 'elder'
})

const canDelete = computed(() => {
  if (!post.value || !currentUser.value.value) return false
  return post.value.author_id === currentUser.value.value?.id ||
         currentUser.value.value?.role === 'admin'
})

// 方法
const loadPost = async () => {
  loading.value = true
  try {
    const response = await postApi.getPost(postId.value)
    if (response.code === 0 && response.data) {
      post.value = response.data
      loadTags()
      loadComments()
    } else {
      ElMessage.error('帖子不存在')
      router.push('/posts')
    }
  } catch (error) {
    console.error('加载帖子失败:', error)
    ElMessage.error('加载帖子失败')
  } finally {
    loading.value = false
  }
}

const loadTags = async () => {
  try {
    const response = await postApi.getPostTags(postId.value)
    if (response.code === 0 && response.data) {
      tags.value = response.data
    }
  } catch (error) {
    console.error('加载标签失败:', error)
  }
}

const loadComments = async () => {
  commentsLoading.value = true
  try {
    const response = await commentApi.getPostComments(postId.value)
    if (response.code === 0 && response.data) {
      comments.value = response.data.list
    }
  } catch (error) {
    console.error('加载评论失败:', error)
  } finally {
    commentsLoading.value = false
  }
}

const loadAllTags = async () => {
  try {
    const response = await tagApi.getAllTags()
    if (response.code === 0 && response.data) {
      allTags.value = response.data
    }
  } catch (error) {
    console.error('加载所有标签失败:', error)
  }
}

const formatContent = (content: string) => {
  // 简单的格式化，可以扩展为更复杂的markdown渲染
  return content.replace(/\n/g, '<br>')
}

const handleLike = async () => {
  if (!isLoggedIn.value) {
    ElMessage.warning('请先登录')
    return
  }
  
  liking.value = true
  try {
    // 这里需要实现点赞API
    isLiked.value = !isLiked.value
    if (post.value) {
      post.value.like_count += isLiked.value ? 1 : -1
    }
    ElMessage.success(isLiked.value ? '点赞成功' : '取消点赞成功')
  } catch (error) {
    console.error('点赞失败:', error)
    ElMessage.error('操作失败')
  } finally {
    liking.value = false
  }
}

const focusCommentInput = () => {
  if (commentInputRef.value) {
    commentInputRef.value.focus()
  }
}

const handleSubmitComment = async () => {
  if (!commentContent.value.trim()) return
  
  submittingComment.value = true
  try {
    const response = await commentApi.createComment({
      target_type: 'Post',
      target_id: postId.value,
      content: commentContent.value
    })
    
    if (response.code === 0) {
      ElMessage.success('评论发表成功')
      commentContent.value = ''
      loadComments()
      if (post.value) {
        post.value.comment_count++
      }
    } else {
      ElMessage.error(response.msg || response.message || '评论发表失败')
    }
  } catch (error) {
    console.error('发表评论失败:', error)
    ElMessage.error('发表评论失败')
  } finally {
    submittingComment.value = false
  }
}

const handleEdit = () => {
  if (!post.value) return
  
  editForm.title = post.value.title
  editForm.content = post.value.content
  editForm.tags = tags.value.map(tag => tag.name)
  showEditDialog.value = true
}

const handleUpdatePost = async () => {
  if (!editFormRef.value) return
  
  try {
    await editFormRef.value.validate()
    updating.value = true
    
    const response = await postApi.updatePost(postId.value, editForm)
    if (response.code === 0) {
      ElMessage.success('帖子更新成功')
      showEditDialog.value = false
      loadPost()
    } else {
      ElMessage.error(response.msg || '更新失败')
    }
  } catch (error) {
    console.error('更新帖子失败:', error)
    ElMessage.error('更新帖子失败')
  } finally {
    updating.value = false
  }
}

const handleDelete = () => {
  ElMessageBox.confirm('确定要删除这个帖子吗？删除后无法恢复。', '确认删除', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      const response = await postApi.deletePost(postId.value)
      if (response.code === 0) {
        ElMessage.success('帖子删除成功')
        router.push('/posts')
      } else {
        ElMessage.error(response.msg || '删除失败')
      }
    } catch (error) {
      console.error('删除帖子失败:', error)
      ElMessage.error('删除帖子失败')
    }
  })
}

const deleteComment = async (commentId: number) => {
  try {
    const response = await commentApi.deleteComment(commentId)
    if (response.code === 0) {
      ElMessage.success('评论删除成功')
      loadComments()
      if (post.value) {
        post.value.comment_count--
      }
    } else {
      ElMessage.error(response.msg || response.message || '删除失败')
    }
  } catch (error) {
    console.error('删除评论失败:', error)
    ElMessage.error('删除评论失败')
  }
}

const canDeleteComment = (comment: Comment) => {
  if (!currentUser.value.value) return false
  return comment.user_id === currentUser.value.value?.id ||
         currentUser.value.value?.role === 'admin' ||
         currentUser.value.value?.role === 'elder'
}

const goToLogin = () => {
  router.push('/login')
}

const goToAdmin = () => {
  const user = currentUser.value.value
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

// 生命周期
onMounted(() => {
  loadPost()
  loadAllTags()
})
</script>

<style scoped>
/* ===== 使用统一主题系统 ===== */
/* 所有主题变量现在由 theme-variables.css 统一管理 */

/* ===== 主容器样式 ===== */
.post-detail-container {
  min-height: 100vh;
  position: relative;
  overflow-x: hidden;
  background: linear-gradient(135deg, 
    hsl(220, 20%, 97%) 0%, 
    hsl(220, 20%, 95%) 100%);
}

:global(html.dark) .post-detail-container {
  background: var(--bg-primary);
}

/* ===== 背景层样式 ===== */
.background-layer {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: -10;
  pointer-events: none;
}

.gradient-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.1) 0%,
    rgba(168, 85, 247, 0.05) 25%,
    rgba(236, 72, 153, 0.05) 50%,
    rgba(245, 158, 11, 0.1) 100%
  );
}

.pattern-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: 
    radial-gradient(circle at 20% 20%, rgba(99, 102, 241, 0.1) 0%, transparent 50%),
    radial-gradient(circle at 80% 80%, rgba(168, 85, 247, 0.08) 0%, transparent 50%),
    radial-gradient(circle at 60% 40%, rgba(236, 72, 153, 0.06) 0%, transparent 50%);
  animation: patternFloat 20s ease-in-out infinite;
}

@keyframes patternFloat {
  0%, 100% { transform: translate(0, 0) rotate(0deg); }
  33% { transform: translate(-10px, -10px) rotate(1deg); }
  66% { transform: translate(10px, -5px) rotate(-1deg); }
}

/* ===== 顶部导航栏样式 ===== */
.top-header {
  position: sticky;
  top: 0;
  z-index: 100;
  background: var(--bg-glass-strong);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--border-color);
  transition: var(--transition-normal);
}

.header-wrapper {
  max-width: 1440px;
  margin: 0 auto;
  padding: var(--space-4) var(--space-6);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-8);
}

.header-left {
  flex-shrink: 0;
}

.brand {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  text-decoration: none;
  cursor: pointer;
}

.brand-icon {
  font-size: var(--font-size-2xl);
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
}

.brand-text h1 {
  font-size: var(--font-size-xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.2;
}

.brand-text span {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  line-height: 1;
}

.header-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.back-btn {
  border-radius: var(--radius-lg);
  transition: var(--transition-normal);
}

.header-right {
  flex-shrink: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.user-menu {
  position: relative;
}

.user-trigger {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: var(--transition-fast);
}

.user-trigger:hover {
  background: var(--bg-hover);
}

.dropdown-arrow {
  color: var(--text-muted);
  transition: var(--transition-fast);
}

/* ===== 主内容区域样式 ===== */
.main-content {
  position: relative;
  z-index: 1;
  padding: var(--space-6) 0;
}

.content-container {
  max-width: 1000px;
  margin: 0 auto;
  padding: 0 var(--space-6);
  display: flex;
  flex-direction: column;
  gap: var(--space-8);
}

/* ===== 毛玻璃面板样式 ===== */
.glass-panel {
  background: var(--bg-glass);
  backdrop-filter: var(--glass-backdrop);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-2xl);
  box-shadow: var(--shadow-glass);
  overflow: hidden;
  transition: var(--transition-normal);
}

.glass-inner {
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: var(--radius-xl);
  transition: var(--transition-normal);
}

:global(html.dark) .glass-panel {
  background: var(--bg-glass);
}

:global(html.dark) .glass-inner {
  background: var(--bg-glass-light);
  border-color: var(--border-color);
}

/* ===== 帖子主体卡片样式 ===== */
.post-main-card {
  padding: var(--space-8);
}

.post-header {
  padding-bottom: var(--space-8);
  border-bottom: 1px solid var(--border-light);
  margin-bottom: var(--space-8);
}

.post-title-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.title-row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
}

.post-title {
  font-size: var(--font-size-4xl);
  font-weight: 800;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.2;
  flex: 1;
}

.status-badges {
  display: flex;
  gap: var(--space-2);
  flex-shrink: 0;
}

.post-meta-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
}

.author-info {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.author-avatar {
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  color: white;
  font-weight: 600;
}

.author-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.author-name {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
}

.post-time {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}

.post-stats {
  display: flex;
  gap: var(--space-6);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
}

.stat-icon {
  color: var(--color-primary);
  font-size: var(--font-size-lg);
}

.stat-value {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
}

.stat-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.post-tags {
  display: flex;
  gap: var(--space-3);
  flex-wrap: wrap;
}

.tag-item {
  font-weight: 500;
  border-radius: var(--radius-lg);
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
  transition: var(--transition-fast);
}

.tag-item:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* ===== 帖子内容样式 ===== */
.post-body {
  margin-bottom: var(--space-8);
}

.content-wrapper {
  padding: var(--space-6);
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-light);
}

.post-content {
  font-size: var(--font-size-lg);
  line-height: 1.8;
  color: var(--text-primary);
  white-space: pre-wrap;
}

/* ===== 操作按钮样式 ===== */
.post-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  padding: var(--space-6);
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-light);
}

.action-group {
  display: flex;
  gap: var(--space-4);
  justify-content: center;
}

.action-btn {
  padding: var(--space-4) var(--space-6);
  border-radius: var(--radius-xl);
  font-size: var(--font-size-base);
  font-weight: 600;
  transition: var(--transition-normal);
  min-width: 120px;
  position: relative;
}

.action-btn:hover {
  transform: translateY(-2px);
}

.like-btn {
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border: none;
  color: white;
  box-shadow: var(--shadow-lg);
}

.like-btn.liked {
  background: linear-gradient(135deg, var(--color-warning), #f59e0b);
}

.comment-btn {
  background: var(--bg-elevated);
  border: 2px solid var(--border-color);
  color: var(--text-primary);
}

.comment-btn:hover {
  border-color: var(--color-primary);
  background: var(--bg-hover);
}

.edit-btn {
  background: linear-gradient(135deg, var(--color-warning), #f59e0b);
  border: none;
  color: white;
}

.delete-btn {
  background: linear-gradient(135deg, var(--color-error), #ef4444);
  border: none;
  color: white;
}

.action-count {
  margin-left: var(--space-2);
  opacity: 0.8;
  font-weight: 400;
}

/* ===== 评论区域样式 ===== */
.comments-section {
  padding: var(--space-8);
}

.section-header {
  margin-bottom: var(--space-8);
}

.section-title {
  font-size: var(--font-size-2xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.section-title .el-icon {
  color: var(--color-primary);
  font-size: var(--font-size-3xl);
}

.comment-count {
  color: var(--text-muted);
  font-weight: 400;
}

.comment-form-container {
  margin-bottom: var(--space-8);
}

.comment-form {
  padding: var(--space-6);
}

.form-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--space-4) 0;
}

.comment-input-wrapper {
  margin-bottom: var(--space-4);
}

.comment-textarea :deep(.el-textarea__inner) {
  border-radius: var(--radius-lg);
  border: 2px solid var(--border-light);
  background: var(--bg-elevated);
  transition: var(--transition-normal);
}

.comment-textarea :deep(.el-textarea__inner):focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.comment-tip {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}

.submit-btn {
  padding: var(--space-3) var(--space-6);
  border-radius: var(--radius-lg);
  font-weight: 600;
}

.login-prompt {
  padding: var(--space-8);
}

.prompt-content {
  display: flex;
  align-items: center;
  gap: var(--space-6);
  text-align: left;
}

.prompt-icon {
  font-size: var(--font-size-4xl);
  color: var(--color-primary);
  flex-shrink: 0;
}

.prompt-text {
  flex: 1;
}

.prompt-text h4 {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--space-2) 0;
}

.prompt-text p {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  margin: 0;
}

.login-btn {
  flex-shrink: 0;
  padding: var(--space-3) var(--space-6);
  border-radius: var(--radius-lg);
  font-weight: 600;
}

/* ===== 评论列表样式 ===== */
.comments-list {
  margin-bottom: var(--space-8);
}

.empty-comments {
  text-align: center;
  padding: var(--space-12);
}

.comment-items {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.comment-item {
  padding: var(--space-6);
  transition: var(--transition-normal);
}

.comment-item:hover {
  background: rgba(255, 255, 255, 0.7);
  transform: translateY(-1px);
}

:global(html.dark) .comment-item:hover {
  background: var(--bg-hover);
}

.comment-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

.comment-author-section {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  flex: 1;
}

.comment-avatar {
  flex-shrink: 0;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  color: white;
  font-weight: 600;
}

.comment-meta {
  flex: 1;
}

.comment-author-name {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  margin-right: var(--space-3);
}

.comment-time {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}

.comment-actions {
  flex-shrink: 0;
}

.delete-comment-btn {
  color: var(--color-error);
  transition: var(--transition-fast);
}

.delete-comment-btn:hover {
  background: rgba(220, 38, 38, 0.1);
}

.comment-content {
  font-size: var(--font-size-base);
  line-height: 1.6;
  color: var(--text-secondary);
  margin-left: calc(40px + var(--space-4));
}

/* ===== 编辑对话框样式 ===== */
.edit-dialog :deep(.el-dialog) {
  border-radius: var(--radius-2xl);
  overflow: hidden;
}

.edit-dialog :deep(.el-dialog__header) {
  background: var(--bg-glass);
  backdrop-filter: var(--glass-backdrop);
  border-bottom: 1px solid var(--border-light);
}

.edit-dialog :deep(.el-dialog__body) {
  background: var(--bg-elevated);
}

.edit-form {
  padding: var(--space-4);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-4);
  padding: var(--space-6);
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-light);
}

/* ===== 加载状态样式 ===== */
.loading-state {
  padding: var(--space-8);
}

/* ===== 响应式设计 ===== */
@media (max-width: 1024px) {
  .content-container {
    max-width: 100%;
    padding: 0 var(--space-4);
  }
  
  .post-main-card,
  .comments-section {
    padding: var(--space-6);
  }
  
  .post-meta-section {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-4);
  }
  
  .post-stats {
    gap: var(--space-4);
  }
}

@media (max-width: 768px) {
  .header-wrapper {
    flex-direction: column;
    gap: var(--space-4);
    align-items: stretch;
  }
  
  .header-center {
    order: 3;
  }
  
  .title-row {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
  }
  
  .post-title {
    font-size: var(--font-size-2xl);
  }
  
  .action-group {
    flex-direction: column;
  }
  
  .comment-content {
    margin-left: 0;
    margin-top: var(--space-3);
  }
  
  .comment-author-section {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-2);
  }
  
  .prompt-content {
    flex-direction: column;
    text-align: center;
    gap: var(--space-4);
  }
}

@media (max-width: 480px) {
  .content-container {
    padding: 0 var(--space-3);
  }
  
  .post-main-card,
  .comments-section {
    padding: var(--space-4);
  }
  
  .post-title {
    font-size: var(--font-size-xl);
  }
  
  .post-stats {
    flex-wrap: wrap;
    gap: var(--space-3);
  }
  
  .action-btn {
    padding: var(--space-3) var(--space-4);
    min-width: 100px;
  }
}
</style> 