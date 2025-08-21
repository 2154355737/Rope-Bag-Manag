<template>
  <div class="admin-page post-review">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><Document /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">帖子审核</h1>
            <p class="page-subtitle">审核用户发布的帖子，决定是否上架到社区</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button @click="loadPendingPosts" :loading="loading">
            <el-icon><Refresh /></el-icon>
            刷新列表
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon pending">
            <el-icon :size="24"><Clock /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ pendingCount }}</div>
            <div class="stat-label">待审核</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon approved">
            <el-icon :size="24"><Select /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ approvedCount }}</div>
            <div class="stat-label">已通过</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon rejected">
            <el-icon :size="24"><Close /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ rejectedCount }}</div>
            <div class="stat-label">已拒绝</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon total">
            <el-icon :size="24"><Document /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalReviewed }}</div>
            <div class="stat-label">总审核数</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和过滤 -->
    <div class="search-section">
      <div class="search-left">
        <el-input
          v-model="searchQuery"
          placeholder="搜索帖子标题或作者"
          clearable
          style="width: 250px"
          @input="handleSearch"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        
        <el-select 
          v-model="categoryFilter" 
          placeholder="分类筛选" 
          clearable 
          style="width: 150px"
          @change="handleFilter"
        >
          <el-option 
            v-for="category in categories" 
            :key="category.id" 
            :label="category.name" 
            :value="category.id" 
          />
        </el-select>
      </div>
      
      <div class="search-right">
        <el-button type="success" @click="batchApprove" :disabled="selectedPosts.length === 0">
          <el-icon><Select /></el-icon>
          批量通过
        </el-button>
        <el-button type="danger" @click="batchReject" :disabled="selectedPosts.length === 0">
          <el-icon><Close /></el-icon>
          批量拒绝
        </el-button>
      </div>
    </div>

    <!-- 帖子列表 -->
    <div class="table-section">
      <el-table 
        :data="postList" 
        v-loading="loading"
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
        border
        stripe
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="title" label="帖子标题" min-width="200">
          <template #default="{ row }">
            <div class="post-title">
              <el-link @click="viewPost(row)" type="primary">
                {{ row.title }}
              </el-link>
              <div class="post-preview">{{ getContentPreview(row.content) }}</div>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="author_name" label="作者" width="120" />
        <el-table-column prop="category_id" label="分类" width="100">
          <template #default="{ row }">
            {{ getCategoryName(row.category_id) }}
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="发布时间" width="180">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column label="统计" width="120">
          <template #default="{ row }">
            <div class="post-stats">
              <div>阅读: {{ row.view_count }}</div>
              <div>点赞: {{ row.like_count }}</div>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" type="success" @click="approvePost(row)">
              <el-icon><Select /></el-icon>
              通过
            </el-button>
            <el-button size="small" type="danger" @click="rejectPost(row)">
              <el-icon><Close /></el-icon>
              拒绝
            </el-button>
            <el-button size="small" @click="viewPost(row)">
              <el-icon><View /></el-icon>
              详情
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-section">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </div>

    <!-- 审核详情弹窗 -->
    <el-dialog
      v-model="reviewDialogVisible"
      :title="currentAction === 'approve' ? '通过审核' : '拒绝审核'"
      width="600px"
      @close="handleReviewDialogClose"
    >
      <div v-if="currentPost">
        <div class="post-info">
          <h3>{{ currentPost.title }}</h3>
          <p><strong>作者：</strong>{{ currentPost.author_name }}</p>
          <p><strong>分类：</strong>{{ getCategoryName(currentPost.category_id) }}</p>
          <p><strong>发布时间：</strong>{{ formatTime(currentPost.created_at) }}</p>
          <div class="post-content">
            <strong>内容：</strong>
            <div class="content-preview">{{ currentPost.content }}</div>
          </div>
        </div>
        
        <el-form :model="reviewForm" label-width="80px" class="review-form">
          <el-form-item label="审核备注">
            <el-input
              v-model="reviewForm.comment"
              type="textarea"
              :rows="4"
              :placeholder="currentAction === 'approve' ? '可选：添加通过审核的备注' : '请说明拒绝的原因'"
            />
          </el-form-item>
        </el-form>
      </div>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="reviewDialogVisible = false">取消</el-button>
          <el-button 
            :type="currentAction === 'approve' ? 'success' : 'danger'" 
            @click="confirmReview" 
            :loading="reviewLoading"
          >
            确认{{ currentAction === 'approve' ? '通过' : '拒绝' }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 帖子详情查看弹窗 -->
    <el-dialog
      v-model="postDetailVisible"
      title="帖子详情"
      width="80%"
      @close="postDetailVisible = false"
    >
      <div v-if="viewingPost" class="post-detail">
        <h2>{{ viewingPost.title }}</h2>
        <div class="post-meta">
          <span>作者：{{ viewingPost.author_name }}</span>
          <span>分类：{{ getCategoryName(viewingPost.category_id) }}</span>
          <span>发布时间：{{ formatTime(viewingPost.created_at) }}</span>
        </div>
        <div class="post-content-full" v-html="viewingPost.content"></div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Document,
  Refresh, 
  Clock, 
  Select, 
  Close, 
  Search, 
  View 
} from '@element-plus/icons-vue'
import { getPendingPosts, reviewPost, type Post, type ReviewPostRequest } from '../../api/posts'
import { categoryApi, type Category } from '../../api/categories'
import { formatDate } from '../../utils/format'
import { getUserInfo } from '../../utils/auth'

// 响应式数据
const loading = ref(false)
const reviewLoading = ref(false)
const postList = ref<Post[]>([])
const selectedPosts = ref<Post[]>([])
const categories = ref<Category[]>([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const searchQuery = ref('')
const categoryFilter = ref<number | null>(null)

// 统计数据
const pendingCount = ref(0)
const approvedCount = ref(0)
const rejectedCount = ref(0)
const totalReviewed = computed(() => approvedCount.value + rejectedCount.value)

// 审核对话框相关
const reviewDialogVisible = ref(false)
const currentPost = ref<Post | null>(null)
const currentAction = ref<'approve' | 'reject'>('approve')
const reviewForm = reactive({
  comment: ''
})

// 帖子详情查看
const postDetailVisible = ref(false)
const viewingPost = ref<Post | null>(null)

// 方法
const loadPendingPosts = async () => {
  loading.value = true
  try {
    const params: any = {
      page: currentPage.value,
      page_size: pageSize.value
    }
    
    if (searchQuery.value) {
      params.search = searchQuery.value
    }
    
    if (categoryFilter.value) {
      params.category_id = categoryFilter.value
    }
    
    console.log('📤 发送待审核帖子请求:', params)
    const res = await getPendingPosts(params)
    
    if (res.code === 0 && res.data) {
      console.log('✅ 待审核帖子加载成功:', res.data)
      postList.value = res.data.list
      total.value = res.data.total
      pendingCount.value = res.data.total
    } else {
      console.warn('❌ 待审核帖子加载失败:', res)
      ElMessage.error(res.message || '加载待审核帖子失败')
    }
  } catch (error: any) {
    console.error('🚫 加载待审核帖子出错:', error)
    if (error.response?.status === 403) {
      ElMessage.error('权限不足：只有管理员和元老可以查看待审核帖子')
    } else if (error.response?.status === 404) {
      ElMessage.error('API接口不存在，请检查后端服务')
    } else {
      ElMessage.error(error.response?.data?.message || '加载待审核帖子时发生错误')
    }
  } finally {
    loading.value = false
  }
}

const loadCategories = async () => {
  try {
    const res = await categoryApi.getCategories()
    if (res.code === 0 && res.data) {
      categories.value = Array.isArray(res.data) ? res.data : res.data.list || []
    }
  } catch (error) {
    console.error('加载分类出错:', error)
  }
}

const getCategoryName = (categoryId: number | null | undefined): string => {
  if (!categoryId) return '未分类'
  const category = categories.value.find(c => c.id === categoryId)
  return category ? category.name : '未知分类'
}

const formatTime = (dateString: string): string => {
  return formatDate(dateString, 'YYYY-MM-DD HH:mm:ss')
}

const getContentPreview = (content: string): string => {
  // 移除HTML标签并截取前50个字符
  const plainText = content.replace(/<[^>]*>/g, '')
  return plainText.length > 50 ? plainText.substring(0, 50) + '...' : plainText
}

const handleSearch = () => {
  currentPage.value = 1
  loadPendingPosts()
}

const handleFilter = () => {
  currentPage.value = 1
  loadPendingPosts()
}

const handleSizeChange = () => {
  currentPage.value = 1
  loadPendingPosts()
}

const handleCurrentChange = () => {
  loadPendingPosts()
}

const handleSelectionChange = (selection: Post[]) => {
  selectedPosts.value = selection
}

const approvePost = (post: Post) => {
  currentPost.value = post
  currentAction.value = 'approve'
  reviewForm.comment = ''
  reviewDialogVisible.value = true
}

const rejectPost = (post: Post) => {
  currentPost.value = post
  currentAction.value = 'reject'
  reviewForm.comment = ''
  reviewDialogVisible.value = true
}

const viewPost = (post: Post) => {
  viewingPost.value = post
  postDetailVisible.value = true
}

const confirmReview = async () => {
  if (!currentPost.value) return
  
  // 如果是拒绝，必须填写原因
  if (currentAction.value === 'reject' && !reviewForm.comment.trim()) {
    ElMessage.warning('请说明拒绝的原因')
    return
  }
  
  reviewLoading.value = true
  try {
    const reviewData: ReviewPostRequest = {
      status: currentAction.value === 'approve' ? 'approved' : 'rejected',
      comment: reviewForm.comment.trim() || undefined
    }
    
    const res = await reviewPost(currentPost.value.id, reviewData)
    
    if (res.code === 0) {
      const actionText = currentAction.value === 'approve' ? '通过' : '拒绝'
      ElMessage.success(`帖子审核${actionText}成功`)
      reviewDialogVisible.value = false
      
      // 更新统计数据
      if (currentAction.value === 'approve') {
        approvedCount.value++
      } else {
        rejectedCount.value++
      }
      pendingCount.value--
      
      // 重新加载列表
      await loadPendingPosts()
    } else {
      ElMessage.error(res.message || '审核失败')
    }
  } catch (error) {
    console.error('审核帖子出错:', error)
    ElMessage.error('审核帖子时发生错误')
  } finally {
    reviewLoading.value = false
  }
}

const handleReviewDialogClose = () => {
  currentPost.value = null
  reviewForm.comment = ''
}

const batchApprove = async () => {
  if (selectedPosts.value.length === 0) {
    ElMessage.warning('请选择要批量通过的帖子')
    return
  }
  
  try {
    await ElMessageBox.confirm(
      `确定要批量通过选中的 ${selectedPosts.value.length} 个帖子吗？`,
      '批量通过',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'success'
      }
    )
    
    const promises = selectedPosts.value.map(post => 
      reviewPost(post.id, { status: 'approved' })
    )
    
    await Promise.all(promises)
    ElMessage.success('批量通过成功')
    
    // 更新统计
    approvedCount.value += selectedPosts.value.length
    pendingCount.value -= selectedPosts.value.length
    
    selectedPosts.value = []
    await loadPendingPosts()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('批量通过失败')
    }
  }
}

const batchReject = async () => {
  if (selectedPosts.value.length === 0) {
    ElMessage.warning('请选择要批量拒绝的帖子')
    return
  }
  
  try {
    const { value: reason } = await ElMessageBox.prompt(
      '请输入批量拒绝的原因：',
      '批量拒绝',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        inputPlaceholder: '请说明拒绝的原因',
        inputValidator: (value) => {
          if (!value || !value.trim()) {
            return '请输入拒绝原因'
          }
          return true
        }
      }
    )
    
    const promises = selectedPosts.value.map(post => 
      reviewPost(post.id, { 
        status: 'rejected',
        comment: reason.trim()
      })
    )
    
    await Promise.all(promises)
    ElMessage.success('批量拒绝成功')
    
    // 更新统计
    rejectedCount.value += selectedPosts.value.length
    pendingCount.value -= selectedPosts.value.length
    
    selectedPosts.value = []
    await loadPendingPosts()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('批量拒绝失败')
    }
  }
}

// 初始化
onMounted(async () => {
  // 检查用户权限
  const userInfo = getUserInfo()
  console.log('👤 当前用户信息:', userInfo)
  
  if (!userInfo) {
    ElMessage.error('用户未登录')
    return
  }
  
  if (userInfo.role !== 'admin' && userInfo.role !== 'elder') {
    ElMessage.error(`权限不足：当前角色 ${userInfo.role}，需要管理员或元老权限`)
    return
  }
  
  console.log('✅ 权限验证通过，加载帖子审核页面')
  await loadCategories()
  await loadPendingPosts()
})
</script>

<style scoped>
.admin-page {
  padding: 24px;
  background: var(--bg-primary);
  min-height: calc(100vh - 72px); /* 减去导航栏高度 */
}

/* 页面头部 */
.page-header {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon {
  width: 64px;
  height: 64px;
  background: linear-gradient(135deg, #409eff, #66b1ff);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.page-title {
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
}

.page-subtitle {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
}

/* 统计卡片 */
.stats-section {
  margin-bottom: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 24px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: var(--shadow-light);
  transition: transform 0.2s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.stat-icon.pending {
  background: linear-gradient(135deg, #e6a23c, #f0a020);
}

.stat-icon.approved {
  background: linear-gradient(135deg, #67c23a, #85ce61);
}

.stat-icon.rejected {
  background: linear-gradient(135deg, #f56c6c, #f78989);
}

.stat-icon.total {
  background: linear-gradient(135deg, #409eff, #66b1ff);
}

.stat-number {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 14px;
  margin-top: 4px;
}

/* 搜索和过滤 */
.search-section {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: var(--shadow-light);
}

.search-left {
  display: flex;
  gap: 16px;
  align-items: center;
}

.search-right {
  display: flex;
  gap: 12px;
}

/* 表格部分 */
.table-section {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 24px;
  box-shadow: var(--shadow-light);
}

.post-title {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.post-preview {
  font-size: 12px;
  color: var(--text-secondary);
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.post-stats {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 分页 */
.pagination-section {
  margin-top: 24px;
  display: flex;
  justify-content: center;
}

/* 帖子信息 */
.post-info {
  background: var(--bg-light);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
}

.post-info h3 {
  margin: 0 0 12px 0;
  color: var(--text-primary);
}

.post-info p {
  margin: 8px 0;
  color: var(--text-secondary);
}

.post-content {
  margin-top: 12px;
}

.content-preview {
  background: var(--bg-primary);
  border-radius: 4px;
  padding: 12px;
  margin-top: 8px;
  max-height: 200px;
  overflow-y: auto;
  border: 1px solid var(--border-color);
}

.review-form {
  margin-top: 16px;
}

/* 帖子详情 */
.post-detail {
  padding: 16px;
}

.post-detail h2 {
  margin: 0 0 16px 0;
  color: var(--text-primary);
}

.post-meta {
  display: flex;
  gap: 24px;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
  color: var(--text-secondary);
  font-size: 14px;
}

.post-content-full {
  line-height: 1.6;
  color: var(--text-primary);
}

/* 深色模式适配 */
.dark .stat-card {
  background: #1f2937;
}

.dark .search-section,
.dark .table-section {
  background: #1f2937;
}

.dark .post-info {
  background: #374151;
}

.dark .content-preview {
  background: #1f2937;
}
</style> 