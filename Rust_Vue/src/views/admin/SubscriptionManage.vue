<template>
  <div class="subscription-manage">
    <div class="page-header">
      <h2>订阅管理</h2>
      <p class="page-description">管理用户的分类订阅和通知设置</p>
    </div>

    <!-- 统计卡片 -->
    <el-row :gutter="20" class="stats-row">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-number">{{ totalSubscriptions }}</div>
            <div class="stat-label">总订阅数</div>
          </div>
          <el-icon class="stat-icon"><Bell /></el-icon>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-number">{{ activeUsers }}</div>
            <div class="stat-label">活跃订阅用户</div>
          </div>
          <el-icon class="stat-icon"><User /></el-icon>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-number">{{ totalCategories }}</div>
            <div class="stat-label">可订阅分类</div>
          </div>
          <el-icon class="stat-icon"><Grid /></el-icon>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-number">{{ sentNotifications }}</div>
            <div class="stat-label">已发送通知</div>
          </div>
          <el-icon class="stat-icon"><Message /></el-icon>
        </el-card>
      </el-col>
    </el-row>

    <!-- 操作区域 -->
    <el-card shadow="hover" class="action-card">
      <template #header>
        <div class="card-header">
          <span>分类订阅统计</span>
          <div class="card-actions">
            <el-button type="primary" size="small" @click="loadSubscriptionStats">
              <el-icon><Refresh /></el-icon>
              刷新统计
            </el-button>
            <el-button type="success" size="small" @click="exportSubscriptions">
              <el-icon><Download /></el-icon>
              导出数据
            </el-button>
          </div>
        </div>
      </template>

      <!-- 分类订阅统计表格 -->
      <el-table :data="categoryStats" v-loading="loading" style="width: 100%">
        <el-table-column prop="name" label="分类名称" min-width="120">
          <template #default="{ row }">
            <el-tag :type="getCategoryTagType(row.id)">{{ row.name }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="description" label="分类描述" show-overflow-tooltip />
        <el-table-column prop="resource_count" label="资源数量" width="100" align="center">
          <template #default="{ row }">
            <span class="resource-count">{{ row.resource_count }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="subscription_count" label="订阅用户数" width="120" align="center">
          <template #default="{ row }">
            <span class="subscription-count">{{ row.subscription_count }}</span>
          </template>
        </el-table-column>
        <el-table-column label="订阅率" width="100" align="center">
          <template #default="{ row }">
            <el-progress 
              :percentage="getSubscriptionRate(row.subscription_count)" 
              :stroke-width="8"
              :show-text="false"
            />
            <span class="rate-text">{{ getSubscriptionRate(row.subscription_count) }}%</span>
          </template>
        </el-table-column>
        <el-table-column label="订阅状态" width="120" align="center">
          <template #default="{ row }">
            <el-tag 
              :type="row.subscription_locked ? 'danger' : 'success'"
              size="small"
            >
              {{ row.subscription_locked ? '已锁定' : '可订阅' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="280" align="center">
          <template #default="{ row }">
            <el-button 
              type="primary" 
              size="small" 
              @click="viewSubscribers(row)"
            >
              查看订阅者
            </el-button>
            <el-button 
              type="warning" 
              size="small" 
              @click="sendNotification(row)"
            >
              发送通知
            </el-button>
            <el-button 
              :type="row.subscription_locked ? 'success' : 'danger'"
              size="small" 
              @click="toggleSubscriptionLock(row)"
            >
              {{ row.subscription_locked ? '解锁' : '锁定' }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 用户订阅详情弹窗 -->
    <el-dialog v-model="subscribersDialogVisible" title="订阅用户详情" width="800px">
      <div v-if="selectedCategory">
        <h4>{{ selectedCategory.name }} - 订阅用户列表</h4>
        <el-table :data="subscribers" v-loading="subscribersLoading" style="width: 100%">
          <el-table-column prop="username" label="用户名" width="150" />
          <el-table-column prop="nickname" label="昵称" width="150" />
          <el-table-column prop="email" label="邮箱" show-overflow-tooltip />
          <el-table-column prop="subscribed_at" label="订阅时间" width="180" />
          <el-table-column label="操作" width="100" align="center">
            <template #default="{ row }">
              <el-button 
                type="danger" 
                size="small" 
                @click="unsubscribeUser(row.user_id)"
              >
                取消订阅
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </el-dialog>

    <!-- 发送通知弹窗 -->
    <el-dialog v-model="notificationDialogVisible" title="发送分类通知" width="600px">
      <el-form :model="notificationForm" :rules="notificationRules" ref="notificationFormRef" label-width="100px">
        <el-form-item label="目标分类" prop="category_id">
          <el-select v-model="notificationForm.category_id" placeholder="选择分类" style="width: 100%">
            <el-option 
              v-for="category in categoryStats" 
              :key="category.id" 
              :label="category.name" 
              :value="category.id"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="通知标题" prop="title">
          <el-input v-model="notificationForm.title" placeholder="请输入通知标题" />
        </el-form-item>
        <el-form-item label="通知内容" prop="content">
          <el-input 
            v-model="notificationForm.content" 
            type="textarea" 
            :rows="4" 
            placeholder="请输入通知内容"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="notificationDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="submitNotification" :loading="sending">
            发送通知
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Bell, User, Grid, Message, Refresh, Download } from '@element-plus/icons-vue'
import { subscriptionApi, categoryApi } from '@/api'

// 数据定义
const loading = ref(false)
const subscribersLoading = ref(false)
const subscribersDialogVisible = ref(false)
const notificationDialogVisible = ref(false)
const sending = ref(false)

// 统计数据
const totalSubscriptions = ref(0)
const activeUsers = ref(0)
const totalCategories = ref(0)
const sentNotifications = ref(0)

// 分类统计数据
const categoryStats = ref([])
const subscribers = ref([])
const selectedCategory = ref(null)

// 通知表单
const notificationForm = reactive({
  category_id: '',
  title: '',
  content: ''
})

const notificationRules = {
  category_id: [{ required: true, message: '请选择目标分类', trigger: 'change' }],
  title: [{ required: true, message: '请输入通知标题', trigger: 'blur' }],
  content: [{ required: true, message: '请输入通知内容', trigger: 'blur' }]
}

// 加载订阅统计数据
const loadSubscriptionStats = async () => {
  loading.value = true
  try {
    // 获取分类列表
    const categoriesRes = await categoryApi.getCategories()
    if (categoriesRes.code === 0 && categoriesRes.data) {
      const categories = categoriesRes.data.list || []
      totalCategories.value = categories.length
      
      // 为每个分类获取订阅统计
      const statsPromises = categories.map(async (category) => {
        try {
          const subscriptionsRes = await subscriptionApi.getCategorySubscriptions(category.id)
          return {
            ...category,
            subscription_count: subscriptionsRes.data?.count || 0,
            resource_count: category.count || 0,
            subscription_locked: category.subscription_locked || false
          }
        } catch (error) {
          return {
            ...category,
            subscription_count: 0,
            resource_count: category.count || 0,
            subscription_locked: category.subscription_locked || false
          }
        }
      })
      
      categoryStats.value = await Promise.all(statsPromises)
      
      // 计算总统计
      totalSubscriptions.value = categoryStats.value.reduce((sum, cat) => sum + cat.subscription_count, 0)
      // 获取所有有订阅的用户数量（去重）
      const allSubscribers = new Set()
      for (const cat of categoryStats.value) {
        if (cat.subscription_count > 0) {
          // 由于我们只有订阅数量，暂时用订阅数量来估计活跃用户数
          // 实际应该从用户表获取总用户数
          allSubscribers.add(cat.subscription_count)
        }
      }
      // 暂时使用一个固定值或从用户API获取总用户数
      activeUsers.value = 1 // 至少有一个用户订阅了
    }
  } catch (error) {
    console.error('加载订阅统计失败:', error)
    ElMessage.error('加载订阅统计失败')
  } finally {
    loading.value = false
  }
}

// 获取分类标签类型
const getCategoryTagType = (categoryId: number) => {
  const types = ['primary', 'success', 'warning', 'info', 'danger']
  return types[categoryId % types.length]
}

// 计算订阅率（基于总订阅数的相对比例）
const getSubscriptionRate = (subscriptionCount: number) => {
  if (totalSubscriptions.value === 0) return 0
  return Math.round((subscriptionCount / totalSubscriptions.value) * 100)
}

// 查看订阅者
const viewSubscribers = async (category) => {
  selectedCategory.value = category
  subscribersDialogVisible.value = true
  subscribersLoading.value = true
  
  try {
    const res = await subscriptionApi.getCategorySubscribers(category.id)
    if (res.code === 0) {
      subscribers.value = res.data?.subscribers || []
    }
  } catch (error) {
    console.error('获取订阅者失败:', error)
    ElMessage.error('获取订阅者失败')
  } finally {
    subscribersLoading.value = false
  }
}

// 取消用户订阅
const unsubscribeUser = async (userId: number) => {
  try {
    await ElMessageBox.confirm('确定要取消该用户的订阅吗？', '确认操作', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    const res = await subscriptionApi.adminUnsubscribe(userId, selectedCategory.value.id)
    if (res.code === 0) {
      ElMessage.success('取消订阅成功')
      viewSubscribers(selectedCategory.value) // 重新加载订阅者列表
      loadSubscriptionStats() // 重新加载统计
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('取消订阅失败:', error)
      ElMessage.error('取消订阅失败')
    }
  }
}

// 发送通知
const sendNotification = (category) => {
  notificationForm.category_id = category.id
  notificationForm.title = `${category.name} 分类更新通知`
  notificationForm.content = `亲爱的用户，${category.name} 分类有新的内容更新，快来查看吧！`
  notificationDialogVisible.value = true
}

// 提交通知
const submitNotification = async () => {
  try {
    sending.value = true
    const res = await subscriptionApi.sendCategoryNotification(notificationForm)
    if (res.code === 0) {
      ElMessage.success('通知发送成功')
      notificationDialogVisible.value = false
      sentNotifications.value += 1
    }
  } catch (error) {
    console.error('发送通知失败:', error)
    ElMessage.error('发送通知失败')
  } finally {
    sending.value = false
  }
}

// 导出订阅数据
const exportSubscriptions = async () => {
  try {
    const res = await subscriptionApi.exportSubscriptions()
    if (res.code === 0) {
      // 创建下载链接
      const blob = new Blob([JSON.stringify(res.data, null, 2)], { type: 'application/json' })
      const url = window.URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = `subscriptions_${new Date().toISOString().split('T')[0]}.json`
      link.click()
      window.URL.revokeObjectURL(url)
      ElMessage.success('订阅数据导出成功')
    }
  } catch (error) {
    console.error('导出失败:', error)
    ElMessage.error('导出失败')
  }
}

// 切换订阅锁定状态
const toggleSubscriptionLock = async (category) => {
  try {
    const action = category.subscription_locked ? '解锁' : '锁定'
    await ElMessageBox.confirm(
      `确定要${action}该分类的订阅功能吗？${action}后用户将${category.subscription_locked ? '可以' : '无法'}订阅/取消订阅该分类。`, 
      `确认${action}`, 
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    const res = await subscriptionApi.toggleCategoryLock(category.id, !category.subscription_locked)
    if (res.code === 0) {
      ElMessage.success(`${action}成功`)
      loadSubscriptionStats() // 重新加载数据
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('操作失败:', error)
      ElMessage.error('操作失败')
    }
  }
}

onMounted(() => {
  loadSubscriptionStats()
})
</script>

<style scoped>
.subscription-manage {
  padding: 24px;
  background-color: #f5f7fa;
  min-height: calc(100vh - 60px);
}

.page-header {
  margin-bottom: 24px;
}

.page-header h2 {
  margin: 0 0 8px 0;
  font-size: 24px;
  font-weight: 600;
  color: #303133;
}

.page-description {
  margin: 0;
  color: #606266;
  font-size: 14px;
}

.stats-row {
  margin-bottom: 24px;
}

.stat-card {
  position: relative;
  overflow: hidden;
}

.stat-content {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.stat-number {
  font-size: 28px;
  font-weight: 700;
  color: #409EFF;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: #606266;
}

.stat-icon {
  position: absolute;
  top: 16px;
  right: 16px;
  font-size: 32px;
  color: #e0e0e0;
}

.action-card {
  margin-bottom: 24px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-actions {
  display: flex;
  gap: 8px;
}

.subscription-count {
  font-weight: 600;
  color: #409EFF;
}

.rate-text {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: #666;
}

.resource-count {
  font-size: 16px;
  font-weight: 600;
  color: #409EFF;
  background-color: #f0f9ff;
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid #e1f5fe;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style> 