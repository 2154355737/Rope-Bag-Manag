<template>
  <div class="post-detail-container">
    <div class="post-content-wrapper">
      <!-- 帖子头部 -->
      <div class="post-header" v-if="post">
        <div class="post-meta">
          <div class="post-title-section">
            <h1 class="post-title">{{ post.title }}</h1>
            <div class="post-status-badges">
              <el-tag v-if="post.is_pinned" type="warning" size="small">置顶</el-tag>
              <el-tag v-if="post.is_featured" type="success" size="small">精选</el-tag>
              <el-tag v-if="post.status === 'Draft'" type="info" size="small">草稿</el-tag>
            </div>
          </div>
          
          <div class="post-info">
            <div class="author-info">
              <el-avatar :size="32" class="author-avatar">
                <el-icon><User /></el-icon>
              </el-avatar>
              <span class="author-name">{{ post.author_name }}</span>
              <span class="post-time">{{ formatTime(post.created_at) }}</span>
            </div>
            
            <div class="post-stats">
              <span class="stat-item">
                <el-icon><View /></el-icon>
                {{ post.view_count }} 浏览
              </span>
              <span class="stat-item">
                <el-icon><ChatDotRound /></el-icon>
                {{ post.comment_count }} 评论
              </span>
              <span class="stat-item">
                <el-icon><Star /></el-icon>
                {{ post.like_count }} 点赞
              </span>
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
          >
            {{ tag.name }}
          </el-tag>
        </div>
      </div>

      <!-- 帖子内容 -->
      <div class="post-body" v-if="post">
        <div class="content-wrapper">
          <div class="post-content" v-html="formatContent(post.content)"></div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="post-actions" v-if="post && isLoggedIn">
        <el-button-group>
          <el-button 
            type="primary" 
            :icon="isLiked ? 'Star' : 'StarFilled'"
            @click="handleLike"
            :loading="liking"
          >
            {{ isLiked ? '取消点赞' : '点赞' }}
          </el-button>
          <el-button 
            type="info" 
            icon="ChatDotRound"
            @click="focusCommentInput"
          >
            评论
          </el-button>
          <el-button 
            v-if="canEdit"
            type="warning" 
            icon="Edit"
            @click="handleEdit"
          >
            编辑
          </el-button>
          <el-button 
            v-if="canDelete"
            type="danger" 
            icon="Delete"
            @click="handleDelete"
          >
            删除
          </el-button>
        </el-button-group>
      </div>

      <!-- 评论区域 -->
      <div class="comments-section">
        <h3 class="comments-title">评论 ({{ post?.comment_count || 0 }})</h3>
        
        <!-- 发表评论 -->
        <div class="comment-form" v-if="isLoggedIn">
          <el-input
            ref="commentInputRef"
            v-model="commentContent"
            type="textarea"
            :rows="3"
            placeholder="写下你的评论..."
            @keydown.ctrl.enter="handleSubmitComment"
          />
          <div class="comment-actions">
            <el-button 
              type="primary" 
              @click="handleSubmitComment"
              :loading="submittingComment"
              :disabled="!commentContent.trim()"
            >
              发表评论
            </el-button>
            <span class="comment-tip">Ctrl + Enter 快速发表</span>
          </div>
        </div>
        
        <div v-else class="login-prompt">
          <el-button type="primary" @click="goToLogin">登录后发表评论</el-button>
        </div>

        <!-- 评论列表 -->
        <div class="comments-list" v-loading="commentsLoading">
          <div v-if="comments.length === 0 && !commentsLoading" class="empty-comments">
            <el-empty description="暂无评论" />
          </div>
          
          <div v-else class="comment-items">
            <div
              v-for="comment in comments"
              :key="comment.id"
              class="comment-item"
            >
              <div class="comment-header">
                <div class="comment-author">
                  <el-avatar :size="24" class="comment-avatar">
                    <el-icon><User /></el-icon>
                  </el-avatar>
                  <span class="comment-author-name">{{ comment.username }}</span>
                  <span class="comment-time">{{ formatTime(comment.created_at) }}</span>
                </div>
                <div class="comment-actions">
                  <el-button 
                    v-if="canDeleteComment(comment)"
                    type="danger" 
                    size="small"
                    @click="deleteComment(comment.id)"
                  >
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
      </div>
    </div>

    <!-- 编辑对话框 -->
    <el-dialog
      v-model="showEditDialog"
      title="编辑帖子"
      width="60%"
    >
      <el-form ref="editFormRef" :model="editForm" :rules="editRules" label-width="80px">
        <el-form-item label="标题" prop="title">
          <el-input v-model="editForm.title" placeholder="请输入帖子标题" />
        </el-form-item>
        
        <el-form-item label="内容" prop="content">
          <el-input
            v-model="editForm.content"
            type="textarea"
            :rows="8"
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
        <span class="dialog-footer">
          <el-button @click="showEditDialog = false">取消</el-button>
          <el-button type="primary" @click="handleUpdatePost" :loading="updating">
            保存
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { User, View, ChatDotRound, Star, StarFilled, Edit, Delete } from '@element-plus/icons-vue'
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

// 生命周期
onMounted(() => {
  loadPost()
  loadAllTags()
})
</script>

<style scoped>
.post-detail-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.post-content-wrapper {
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.post-header {
  padding: 30px;
  border-bottom: 1px solid #f0f0f0;
}

.post-title-section {
  margin-bottom: 20px;
}

.post-title {
  margin: 0 0 12px 0;
  font-size: 28px;
  font-weight: 600;
  color: #333;
  line-height: 1.4;
}

.post-status-badges {
  display: flex;
  gap: 8px;
}

.post-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.author-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.author-avatar {
  background: #f0f0f0;
}

.author-name {
  font-weight: 600;
  color: #333;
}

.post-time {
  color: #999;
  font-size: 14px;
}

.post-stats {
  display: flex;
  gap: 16px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #666;
  font-size: 14px;
}

.post-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.tag-item {
  cursor: pointer;
}

.post-body {
  padding: 30px;
}

.content-wrapper {
  line-height: 1.8;
  font-size: 16px;
  color: #333;
}

.post-content {
  white-space: pre-wrap;
}

.post-actions {
  padding: 20px 30px;
  border-top: 1px solid #f0f0f0;
  display: flex;
  justify-content: center;
}

.comments-section {
  padding: 30px;
  border-top: 1px solid #f0f0f0;
}

.comments-title {
  margin: 0 0 20px 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
}

.comment-form {
  margin-bottom: 30px;
}

.comment-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 12px;
}

.comment-tip {
  color: #999;
  font-size: 12px;
}

.login-prompt {
  text-align: center;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 8px;
  margin-bottom: 30px;
}

.comments-list {
  margin-top: 20px;
}

.empty-comments {
  text-align: center;
  padding: 40px 20px;
}

.comment-items {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.comment-item {
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.comment-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.comment-author {
  display: flex;
  align-items: center;
  gap: 8px;
}

.comment-avatar {
  background: #e0e0e0;
}

.comment-author-name {
  font-weight: 600;
  color: #333;
  font-size: 14px;
}

.comment-time {
  color: #999;
  font-size: 12px;
}

.comment-content {
  color: #333;
  line-height: 1.6;
  font-size: 14px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

@media (max-width: 768px) {
  .post-detail-container {
    padding: 10px;
  }
  
  .post-header,
  .post-body,
  .post-actions,
  .comments-section {
    padding: 20px;
  }
  
  .post-title {
    font-size: 24px;
  }
  
  .post-info {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .post-stats {
    gap: 12px;
  }
}
</style> 