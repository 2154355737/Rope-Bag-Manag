<template>
  <div class="user-dashboard">
    <div class="welcome-section">
      <h2>用户中心</h2>
      <p>欢迎回来，{{ userInfo?.nickname || userInfo?.username || '用户' }}！</p>
    </div>

    <!-- 统计卡片 -->
    <el-row :gutter="20" class="stats-cards">
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
              <el-icon color="#67C23A"><ChatDotRound /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.myComments }}</div>
              <div class="stat-label">我的评论</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#E6A23C"><Download /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.totalDownloads }}</div>
              <div class="stat-label">下载次数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#F56C6C"><Star /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ userInfo?.star || 0 }}</div>
              <div class="stat-label">积分</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="content-row">
      <!-- 快捷操作 -->
      <el-col :span="12">
        <el-card shadow="hover" class="action-card">
          <template #header>
            <div class="card-header">
              <span>快捷操作</span>
            </div>
          </template>
          <div class="quick-actions">
            <el-button type="primary" icon="Plus" @click="$router.push('/user/resources')">
              上传资源
            </el-button>
            <el-button type="success" icon="Edit" @click="$router.push('/user/profile')">
              编辑资料
            </el-button>
            <el-button type="info" icon="View" @click="$router.push('/home')">
              浏览资源
            </el-button>
            <el-button type="warning" icon="Bell" @click="handleSignIn" :loading="signingIn">
              {{ todaySignedIn ? '已签到' : '每日签到' }}
            </el-button>
          </div>
        </el-card>
      </el-col>

      <!-- 最近活动 -->
      <el-col :span="12">
        <el-card shadow="hover" class="activity-card">
          <template #header>
            <div class="card-header">
              <span>最近活动</span>
              <el-button text type="primary" size="small" @click="loadRecentActivity">
                刷新
              </el-button>
            </div>
          </template>
          <div class="recent-activity" v-loading="activityLoading">
            <div v-if="recentActivity.length === 0" class="no-activity">
              暂无最近活动
            </div>
            <div v-else class="activity-list">
              <div 
                v-for="item in recentActivity" 
                :key="item.id" 
                class="activity-item"
              >
                <div class="activity-icon">
                  <el-icon v-if="item.type === 'upload'" color="#409EFF"><Upload /></el-icon>
                  <el-icon v-else-if="item.type === 'comment'" color="#67C23A"><ChatDotRound /></el-icon>
                  <el-icon v-else-if="item.type === 'download'" color="#E6A23C"><Download /></el-icon>
                  <el-icon v-else color="#909399"><Operation /></el-icon>
                </div>
                <div class="activity-content">
                  <div class="activity-text">{{ item.description }}</div>
                  <div class="activity-time">{{ formatTime(item.created_at) }}</div>
                </div>
              </div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 我的资源概览 -->
    <el-card shadow="hover" class="resources-overview">
      <template #header>
        <div class="card-header">
          <span>我的资源</span>
          <el-button text type="primary" size="small" @click="$router.push('/user/resources')">
            查看全部
          </el-button>
        </div>
      </template>
      <div v-loading="resourcesLoading">
        <div v-if="myResources.length === 0" class="no-resources">
          <el-empty description="还没有上传任何资源">
            <el-button type="primary" @click="$router.push('/user/resources')">
              立即上传
            </el-button>
          </el-empty>
        </div>
        <div v-else class="resources-grid">
          <div 
            v-for="resource in myResources.slice(0, 6)" 
            :key="resource.id"
            class="resource-item"
            @click="$router.push(`/resource/${resource.id}`)"
          >
            <div class="resource-info">
              <div class="resource-name">{{ resource.name }}</div>
              <div class="resource-meta">
                <span class="resource-status" :class="getStatusClass(resource.status)">
                  {{ getStatusText(resource.status) }}
                </span>
                <span class="resource-downloads">
                  <el-icon><Download /></el-icon>
                  {{ resource.download_count }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { 
  Document, ChatDotRound, Download, Star, Plus, Edit, View, Bell, 
  Upload, Operation 
} from '@element-plus/icons-vue'
import { getUserInfo } from '@/utils/auth'
import { packageApi, type Package } from '@/api/packages'
import { commentApi } from '@/api/comments'
import { userApi } from '@/api/users'

// 用户信息
const userInfo = ref(getUserInfo())

// 统计数据
const stats = reactive({
  myResources: 0,
  myComments: 0,
  totalDownloads: 0
})

// 签到状态
const signingIn = ref(false)
const todaySignedIn = ref(false)

// 最近活动
const recentActivity = ref<any[]>([])
const activityLoading = ref(false)

// 我的资源
const myResources = ref<Package[]>([])
const resourcesLoading = ref(false)

// 加载统计数据
const loadStats = async () => {
  try {
    // 获取我的资源统计
    try {
      const resourcesRes = await userApi.getMyResources({
        page: 1, 
        pageSize: 1
      })
      if (resourcesRes.code === 0) {
        stats.myResources = resourcesRes.data?.total || 0
      }
    } catch (resourceError) {
      console.warn('获取资源统计失败，使用默认值:', resourceError)
      stats.myResources = 0
    }

    // 获取我的评论统计
    try {
      const commentsRes = await userApi.getMyComments({
        page: 1,
        size: 1
      })
      if (commentsRes.code === 0) {
        stats.myComments = commentsRes.data?.total || 0
      }
    } catch (commentError) {
      console.warn('获取评论统计失败，使用默认值:', commentError)
      stats.myComments = 0
    }
  } catch (error) {
    console.error('加载统计数据失败:', error)
    // 设置默认值，避免显示错误
    stats.myResources = 0
    stats.myComments = 0
  }
}

// 加载最近活动
const loadRecentActivity = async () => {
  activityLoading.value = true
  try {
    // 这里应该调用获取用户活动的API，暂时模拟数据
    await new Promise(resolve => setTimeout(resolve, 500))
    recentActivity.value = [
      {
        id: 1,
        type: 'upload',
        description: '上传了新资源: 示例绳包',
        created_at: new Date().toISOString()
      },
      {
        id: 2,
        type: 'comment',
        description: '评论了资源: 另一个绳包',
        created_at: new Date(Date.now() - 86400000).toISOString()
      }
    ]
  } catch (error) {
    console.error('加载最近活动失败:', error)
  } finally {
    activityLoading.value = false
  }
}

// 加载我的资源
const loadMyResources = async () => {
  resourcesLoading.value = true
  try {
    const res = await packageApi.getPackages({
      page: 1,
      pageSize: 6,
      search: userInfo.value?.username
    })
    if (res.code === 0) {
      myResources.value = res.data?.list || []
      // 计算总下载次数
      stats.totalDownloads = myResources.value.reduce((sum, resource) => 
        sum + (resource.download_count || 0), 0)
    }
  } catch (error) {
    console.error('加载我的资源失败:', error)
  } finally {
    resourcesLoading.value = false
  }
}

// 签到功能
const handleSignIn = async () => {
  if (todaySignedIn.value) return
  
  signingIn.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1000))
    todaySignedIn.value = true
    ElMessage.success('签到成功，获得积分+1')
    if (userInfo.value) {
      userInfo.value.star = (userInfo.value.star || 0) + 1
    }
  } catch (error) {
    ElMessage.error('签到失败')
  } finally {
    signingIn.value = false
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

// 获取资源状态样式
const getStatusClass = (status: string) => {
  const statusMap: Record<string, string> = {
    'Active': 'status-active',
    'Pending': 'status-pending',
    'Rejected': 'status-rejected',
    'Inactive': 'status-inactive'
  }
  return statusMap[status] || ''
}

// 获取资源状态文本
const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'Active': '已发布',
    'Pending': '审核中',
    'Rejected': '已拒绝',
    'Inactive': '已下架'
  }
  return statusMap[status] || status
}

onMounted(() => {
  loadStats()
  loadRecentActivity()
  loadMyResources()
})
</script>

<style scoped>
.user-dashboard {
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
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.quick-actions .el-button {
  flex: 1;
  min-width: 120px;
}

.recent-activity {
  max-height: 300px;
  overflow-y: auto;
}

.no-activity {
  text-align: center;
  color: #909399;
  padding: 40px 0;
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.activity-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.activity-item:last-child {
  border-bottom: none;
}

.activity-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: #f0f9ff;
  flex-shrink: 0;
}

.activity-content {
  flex: 1;
}

.activity-text {
  font-size: 14px;
  color: #303133;
  margin-bottom: 4px;
}

.activity-time {
  font-size: 12px;
  color: #909399;
}

.resources-overview {
  border-radius: 8px;
}

.no-resources {
  padding: 40px 0;
}

.resources-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.resource-item {
  padding: 16px;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.resource-item:hover {
  border-color: #409EFF;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.1);
}

.resource-name {
  font-size: 16px;
  font-weight: 500;
  color: #303133;
  margin-bottom: 8px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.resource-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 12px;
}

.resource-status {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
}

.status-active {
  background-color: #f0f9ff;
  color: #409EFF;
}

.status-pending {
  background-color: #fdf6ec;
  color: #e6a23c;
}

.status-rejected {
  background-color: #fef0f0;
  color: #f56c6c;
}

.status-inactive {
  background-color: #f4f4f5;
  color: #909399;
}

.resource-downloads {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #909399;
}
</style> 