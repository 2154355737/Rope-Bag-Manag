<template>
  <div class="comments-container">
    <div class="header">
      <el-button class="back-btn" @click="goBack" type="default" plain>
        <el-icon><Back /></el-icon>
        返回资源
      </el-button>
      <h1 class="title">资源评论</h1>
    </div>

    <div v-if="loading" class="loading">
      <el-skeleton :rows="10" animated />
    </div>

    <div v-else-if="!resource" class="not-found">
      <el-empty description="资源不存在或已被删除" />
      <el-button type="primary" @click="goToHome">返回首页</el-button>
    </div>

    <template v-else>
      <div class="resource-info">
        <h2>{{ resource.name }}</h2>
        <div class="meta">
          <span class="author">作者: {{ resource.author }}</span>
          <span class="date">发布时间: {{ formatDate(resource.created_at) }}</span>
        </div>
      </div>

      <div class="comments-section">
        <h3>评论 ({{ totalComments }})</h3>
        
        <!-- 评论列表 -->
        <div v-if="comments.length === 0" class="no-comments">
          <p>暂无评论，来发表第一条评论吧！</p>
        </div>
        
        <div v-else class="comment-list">
          <div v-for="comment in comments" :key="comment.id" class="comment-item">
            <div class="comment-header">
              <div class="comment-author">{{ comment.author_name || '匿名用户' }}</div>
              <div class="comment-time">{{ formatDate(comment.created_at) }}</div>
            </div>
            <div class="comment-content">{{ comment.content }}</div>
            <div class="comment-actions">
              <el-button 
                type="text" 
                size="small" 
                @click="likeComment(comment.id)"
              >
                <el-icon><Star /></el-icon>
                点赞 ({{ comment.likes || 0 }})
              </el-button>
              <el-button 
                v-if="isLoggedIn"
                type="text" 
                size="small" 
                @click="replyToComment(comment.id)"
              >
                <el-icon><ChatRound /></el-icon>
                回复
              </el-button>
              <el-button 
                v-if="canDeleteComment(comment)"
                type="text" 
                size="small" 
                @click="deleteComment(comment.id)"
              >
                <el-icon><Delete /></el-icon>
                删除
              </el-button>
            </div>
          </div>
        </div>
        
        <!-- 分页 -->
        <div v-if="totalComments > 0" class="pagination">
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
        
        <!-- 评论表单 -->
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
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Back,
  Star,
  ChatRound,
  Delete
} from '@element-plus/icons-vue'
import { getUserInfo } from '@/utils/auth'
import { packageApi, type Package } from '@/api/packages'
import { commentApi, type Comment } from '@/api/comments'

const route = useRoute()
const router = useRouter()
const resourceId = computed(() => Number(route.params.id) || 0)

// 响应式状态
const loading = ref(true)
const resource = ref<Package | null>(null)
const comments = ref<Comment[]>([])
const totalComments = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)
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

// 格式化日期
const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('zh-CN')
}

// 方法
const goBack = () => {
  router.back()
}

const goToHome = () => {
  router.push('/home')
}

const loadResource = async () => {
  if (!resourceId.value) return
  
  try {
    loading.value = true
    const res = await packageApi.getPackage(resourceId.value)
    
    if (res.code === 0 && res.data) {
      resource.value = res.data
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
      target_type: 'package',
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
    // 这里假设有评论点赞API，实际使用时需要替换为真实API
    // const res = await commentApi.likeComment(commentId)
    const res = { code: 0, message: '点赞成功' } // 临时模拟
    
    if (res.code === 0) {
      ElMessage.success('点赞成功')
      
      // 更新评论的点赞数
      const comment = comments.value.find(c => c.id === commentId)
      if (comment) {
        comment.likes = (comment.likes || 0) + 1
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

const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  loadComments()
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
  loadComments()
}

// 初始化
onMounted(async () => {
  await Promise.all([
    loadResource(),
    loadComments()
  ])
})
</script>

<style scoped>
.comments-container {
  max-width: 1000px;
  margin: 0 auto;
  padding: 32px 20px;
}

.header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}

.title {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
}

.loading {
  padding: 40px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

.not-found {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 60px 0;
  gap: 24px;
}

.resource-info {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 24px;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

.resource-info h2 {
  margin: 0 0 12px 0;
  font-size: 20px;
  font-weight: 600;
}

.meta {
  display: flex;
  gap: 20px;
  color: #6b7280;
  font-size: 14px;
}

.comments-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

.comments-section h3 {
  margin: 0 0 20px 0;
  font-size: 18px;
  font-weight: 600;
}

.no-comments {
  text-align: center;
  padding: 40px 0;
  color: #9ca3af;
}

.comment-list {
  margin-bottom: 24px;
}

.comment-item {
  padding: 16px;
  border-bottom: 1px solid #e5e7eb;
}

.comment-item:last-child {
  border-bottom: none;
}

.comment-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.comment-author {
  font-weight: 500;
  color: #1f2937;
}

.comment-time {
  font-size: 12px;
  color: #9ca3af;
}

.comment-content {
  font-size: 14px;
  line-height: 1.6;
  color: #4b5563;
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
  border-top: 1px solid #e5e7eb;
}

.comment-form h3 {
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
  margin: 0 0 16px 0;
}

.form-actions {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}

/* 深色模式适配 */
.dark .comments-container {
  background: #111827;
}

.dark .resource-info,
.dark .comments-section {
  background: #1f2937;
  border-color: #374151;
}

.dark .title,
.dark .resource-info h2,
.dark .comments-section h3,
.dark .comment-author {
  color: #f9fafb;
}

.dark .meta,
.dark .comment-content {
  color: #9ca3af;
}
</style> 