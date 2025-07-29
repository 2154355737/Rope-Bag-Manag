<template>
  <div class="elder-dashboard">
    <div class="welcome-section">
      <h2>元老后台</h2>
      <p>欢迎回来，{{ userInfo?.nickname || userInfo?.username || '元老' }}！您拥有资源审核等管理权限。</p>
    </div>

    <!-- 管理统计卡片 -->
    <el-row :gutter="20" class="stats-cards">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#E6A23C"><Clock /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.pendingResources }}</div>
              <div class="stat-label">待审核资源</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#409EFF"><Document /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.myResources }}</div>
              <div class="stat-label">我的资源</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#67C23A"><Select /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.reviewedResources }}</div>
              <div class="stat-label">已审核资源</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#F56C6C"><ChatDotRound /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.myComments }}</div>
              <div class="stat-label">我的评论</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="content-row">
      <!-- 快捷管理操作 -->
      <el-col :span="12">
        <el-card shadow="hover" class="action-card">
          <template #header>
            <div class="card-header">
              <span>管理操作</span>
            </div>
          </template>
          <div class="quick-actions">
            <el-button type="warning" icon="Clock" @click="$router.push('/elder/resources')">
              资源审核 
              <el-badge :value="stats.pendingResources" :hidden="stats.pendingResources === 0" class="item" />
            </el-button>
            <el-button type="primary" icon="Plus" @click="$router.push('/elder/my-resources')">
              上传资源
            </el-button>
            <el-button type="success" icon="Edit" @click="$router.push('/elder/profile')">
              编辑资料
            </el-button>
            <el-button type="info" icon="ChatDotRound" @click="$router.push('/elder/comments')">
              我的评论
            </el-button>
          </div>
        </el-card>
      </el-col>

      <!-- 待审核资源概览 -->
      <el-col :span="12">
        <el-card shadow="hover" class="pending-card">
          <template #header>
            <div class="card-header">
              <span>待审核资源</span>
              <el-button text type="primary" size="small" @click="loadPendingResources">
                刷新
              </el-button>
            </div>
          </template>
          <div class="pending-resources" v-loading="pendingLoading">
            <div v-if="pendingResources.length === 0" class="no-pending">
              <el-icon><Select /></el-icon>
              <span>暂无待审核资源</span>
            </div>
            <div v-else class="pending-list">
              <div 
                v-for="resource in pendingResources.slice(0, 5)" 
                :key="resource.id"
                class="pending-item"
                @click="handleReviewResource(resource)"
              >
                <div class="resource-info">
                  <div class="resource-name">{{ resource.name }}</div>
                  <div class="resource-meta">
                    <span class="resource-author">作者: {{ resource.author }}</span>
                    <span class="resource-time">{{ formatTime(resource.created_at) }}</span>
                  </div>
                </div>
                <div class="review-actions">
                  <el-button type="success" size="small" @click.stop="quickReview(resource, 'approved')">
                    通过
                  </el-button>
                  <el-button type="danger" size="small" @click.stop="quickReview(resource, 'rejected')">
                    拒绝
                  </el-button>
                </div>
              </div>
              <div v-if="pendingResources.length > 5" class="show-more">
                <el-button text type="primary" @click="$router.push('/elder/resources')">
                  查看全部 {{ stats.pendingResources }} 个待审核资源
                </el-button>
              </div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 最近审核记录 -->
    <el-card shadow="hover" class="review-history">
      <template #header>
        <div class="card-header">
          <span>最近审核记录</span>
          <el-button text type="primary" size="small" @click="loadReviewHistory">
            查看更多
          </el-button>
        </div>
      </template>
      <div v-loading="historyLoading">
        <div v-if="reviewHistory.length === 0" class="no-history">
          <el-empty description="暂无审核记录" />
        </div>
        <div v-else class="history-list">
          <div 
            v-for="record in reviewHistory.slice(0, 8)" 
            :key="record.id"
            class="history-item"
          >
            <div class="history-icon">
              <el-icon v-if="record.status === 'approved'" color="#67C23A"><Select /></el-icon>
              <el-icon v-else color="#F56C6C"><CloseBold /></el-icon>
            </div>
            <div class="history-content">
              <div class="history-text">
                {{ record.status === 'approved' ? '通过' : '拒绝' }}了资源 "{{ record.name }}"
              </div>
              <div class="history-time">{{ formatTime(record.reviewed_at) }}</div>
            </div>
            <div class="history-status">
              <el-tag :type="record.status === 'approved' ? 'success' : 'danger'" size="small">
                {{ record.status === 'approved' ? '已通过' : '已拒绝' }}
              </el-tag>
            </div>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 资源审核对话框 -->
    <el-dialog v-model="showReviewDialog" title="资源审核" width="600px" :close-on-click-modal="false">
      <div v-if="currentResource">
        <div class="resource-detail">
          <h4>{{ currentResource.name }}</h4>
          <div class="detail-item">
            <span class="label">作者:</span>
            <span class="value">{{ currentResource.author }}</span>
          </div>
          <div class="detail-item">
            <span class="label">版本:</span>
            <span class="value">{{ currentResource.version || '未设置' }}</span>
          </div>
          <div class="detail-item">
            <span class="label">文件链接:</span>
            <el-link :href="currentResource.file_url" target="_blank" type="primary">
              {{ currentResource.file_url }}
            </el-link>
          </div>
          <div class="detail-item">
            <span class="label">描述:</span>
            <div class="description">{{ currentResource.description || '暂无描述' }}</div>
          </div>
        </div>

        <el-form :model="reviewForm" :rules="reviewRules" ref="reviewFormRef" label-width="80px">
          <el-form-item label="审核结果" prop="status">
            <el-radio-group v-model="reviewForm.status">
              <el-radio label="approved">通过</el-radio>
              <el-radio label="rejected">拒绝</el-radio>
            </el-radio-group>
          </el-form-item>
          <el-form-item label="审核意见" prop="comment">
            <el-input 
              v-model="reviewForm.comment" 
              type="textarea" 
              :rows="4" 
              placeholder="请输入审核意见（可选）"
            />
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showReviewDialog = false">取消</el-button>
          <el-button type="primary" @click="handleSubmitReview" :loading="reviewing">
            提交审核
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox, ElForm } from 'element-plus'
import { 
  Clock, Document, Select, ChatDotRound, Plus, Edit, Refresh,
  CloseBold
} from '@element-plus/icons-vue'
import { getUserInfo } from '@/utils/auth'
import { packageApi, type Package, type ReviewResourceRequest } from '@/api/packages'
import { commentApi } from '@/api/comments'

const reviewFormRef = ref<InstanceType<typeof ElForm> | null>(null)

// 用户信息
const userInfo = ref(getUserInfo())

// 加载状态
const pendingLoading = ref(false)
const historyLoading = ref(false)
const reviewing = ref(false)

// 对话框状态
const showReviewDialog = ref(false)

// 数据
const pendingResources = ref<Package[]>([])
const reviewHistory = ref<any[]>([])
const currentResource = ref<Package | null>(null)

// 统计数据
const stats = reactive({
  pendingResources: 0,
  myResources: 0,
  reviewedResources: 0,
  myComments: 0
})

// 审核表单
const reviewForm = reactive<ReviewResourceRequest>({
  status: 'approved',
  comment: ''
})

const reviewRules = {
  status: [
    { required: true, message: '请选择审核结果', trigger: 'change' }
  ]
}

// 加载统计数据
const loadStats = async () => {
  try {
    // 获取待审核资源统计
    const pendingRes = await packageApi.getPendingResources({ page: 1, pageSize: 1 })
    if (pendingRes.code === 0) {
      stats.pendingResources = pendingRes.data?.total || 0
    }

    // 获取我的资源统计
    const resourcesRes = await packageApi.getPackages({ 
      page: 1, 
      pageSize: 1,
      search: userInfo.value?.username 
    })
    if (resourcesRes.code === 0) {
      stats.myResources = resourcesRes.data?.total || 0
    }

    // 获取我的评论统计
    if (userInfo.value?.id) {
      const commentsRes = await commentApi.getUserComments(userInfo.value.id, {
        page: 1,
        size: 1
      })
      if (commentsRes.code === 0) {
        stats.myComments = commentsRes.data?.total || 0
      }
    }

    // 模拟已审核资源统计
    stats.reviewedResources = Math.floor(Math.random() * 50) + 10
  } catch (error) {
    console.error('加载统计数据失败:', error)
  }
}

// 加载待审核资源
const loadPendingResources = async () => {
  pendingLoading.value = true
  try {
    const res = await packageApi.getPendingResources({ page: 1, pageSize: 10 })
    if (res.code === 0) {
      pendingResources.value = res.data?.list || []
      stats.pendingResources = res.data?.total || 0
    }
  } catch (error) {
    console.error('加载待审核资源失败:', error)
    ElMessage.error('加载待审核资源失败')
  } finally {
    pendingLoading.value = false
  }
}

// 加载审核历史
const loadReviewHistory = async () => {
  historyLoading.value = true
  try {
    // 这里应该调用获取审核历史的API，暂时模拟数据
    await new Promise(resolve => setTimeout(resolve, 500))
    reviewHistory.value = [
      {
        id: 1,
        name: '示例绳包1',
        status: 'approved',
        reviewed_at: new Date().toISOString()
      },
      {
        id: 2,
        name: '示例绳包2',
        status: 'rejected',
        reviewed_at: new Date(Date.now() - 86400000).toISOString()
      }
    ]
  } catch (error) {
    console.error('加载审核历史失败:', error)
  } finally {
    historyLoading.value = false
  }
}

// 处理审核资源
const handleReviewResource = (resource: Package) => {
  currentResource.value = resource
  reviewForm.status = 'approved'
  reviewForm.comment = ''
  showReviewDialog.value = true
}

// 快速审核
const quickReview = async (resource: Package, status: 'approved' | 'rejected') => {
  try {
    const res = await packageApi.reviewResource(resource.id, { status })
    if (res.code === 0) {
      ElMessage.success(status === 'approved' ? '审核通过' : '审核拒绝')
      loadPendingResources()
      loadStats()
    } else {
      ElMessage.error(res.message || '审核失败')
    }
  } catch (error) {
    console.error('快速审核失败:', error)
    ElMessage.error('审核失败')
  }
}

// 提交审核
const handleSubmitReview = async () => {
  if (!reviewFormRef.value || !currentResource.value) return
  
  try {
    await reviewFormRef.value.validate()
    reviewing.value = true
    
    const res = await packageApi.reviewResource(currentResource.value.id, reviewForm)
    if (res.code === 0) {
      ElMessage.success('审核完成')
      showReviewDialog.value = false
      loadPendingResources()
      loadStats()
      loadReviewHistory()
    } else {
      ElMessage.error(res.message || '审核失败')
    }
  } catch (error) {
    console.error('提交审核失败:', error)
    ElMessage.error('审核失败')
  } finally {
    reviewing.value = false
  }
}

// 格式化时间
const formatTime = (timeStr: string) => {
  const time = new Date(timeStr)
  const now = new Date()
  const diff = now.getTime() - time.getTime()
  
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}小时前`
  return `${Math.floor(diff / 86400000)}天前`
}

onMounted(() => {
  loadStats()
  loadPendingResources()
  loadReviewHistory()
})
</script>

<style scoped>
.elder-dashboard {
  padding: 24px;
  background-color: #f5f7fa;
  min-height: calc(100vh - 60px);
}

.welcome-section {
  margin-bottom: 24px;
}

.welcome-section h2 {
  margin: 0 0 8px 0;
  color: #303133;
  font-size: 28px;
  font-weight: 600;
}

.welcome-section p {
  margin: 0;
  color: #606266;
  font-size: 16px;
}

.stats-cards {
  margin-bottom: 24px;
}

.stat-card {
  border-radius: 8px;
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 8px;
  background-color: #f0f9ff;
}

.stat-icon .el-icon {
  font-size: 24px;
}

.stat-info {
  flex: 1;
}

.stat-number {
  font-size: 24px;
  font-weight: 600;
  color: #303133;
  line-height: 1;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: #909399;
}

.content-row {
  margin-bottom: 24px;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.quick-actions {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.quick-actions .el-button {
  justify-self: stretch;
}

.pending-resources {
  max-height: 300px;
  overflow-y: auto;
}

.no-pending {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 40px 0;
  color: #909399;
}

.no-pending .el-icon {
  font-size: 32px;
}

.pending-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.pending-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s;
}

.pending-item:hover {
  border-color: #409EFF;
  background-color: #f0f9ff;
}

.resource-info {
  flex: 1;
}

.resource-name {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  margin-bottom: 4px;
}

.resource-meta {
  font-size: 12px;
  color: #909399;
  display: flex;
  gap: 12px;
}

.review-actions {
  display: flex;
  gap: 8px;
}

.show-more {
  text-align: center;
  padding: 12px 0;
  border-top: 1px solid #f0f0f0;
}

.review-history {
  border-radius: 8px;
}

.no-history {
  padding: 20px 0;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.history-item:last-child {
  border-bottom: none;
}

.history-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: #f0f9ff;
  flex-shrink: 0;
}

.history-content {
  flex: 1;
}

.history-text {
  font-size: 14px;
  color: #303133;
  margin-bottom: 4px;
}

.history-time {
  font-size: 12px;
  color: #909399;
}

.resource-detail {
  margin-bottom: 20px;
  padding: 16px;
  background-color: #f8f9fa;
  border-radius: 6px;
}

.resource-detail h4 {
  margin: 0 0 12px 0;
  color: #303133;
}

.detail-item {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin-bottom: 8px;
}

.detail-item .label {
  font-weight: 500;
  color: #606266;
  min-width: 60px;
}

.detail-item .value {
  color: #303133;
  flex: 1;
}

.description {
  color: #303133;
  line-height: 1.6;
  word-break: break-word;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

@media screen and (max-width: 768px) {
  .quick-actions {
    grid-template-columns: 1fr;
  }
  
  .pending-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .review-actions {
    width: 100%;
    justify-content: flex-end;
  }
}
</style> 