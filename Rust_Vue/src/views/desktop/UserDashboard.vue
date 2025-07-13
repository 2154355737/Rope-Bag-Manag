<template>
  <div class="user-dashboard">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">用户后台</h1>
      <p class="page-subtitle">欢迎回来，{{ userInfo.nickname || userInfo.username }}</p>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon">
          <el-icon><Box /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ stats.myPackages }}</div>
          <div class="stat-label">我的资源</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">
          <el-icon><Download /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ stats.downloadCount }}</div>
          <div class="stat-label">下载次数</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">
          <el-icon><View /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ stats.viewCount }}</div>
          <div class="stat-label">查看次数</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">
          <el-icon><Star /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ stats.star }}</div>
          <div class="stat-label">我的星级</div>
        </div>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="main-content">
      <!-- 我的资源 -->
      <div class="content-section">
        <div class="section-header">
          <h2 class="section-title">我的资源</h2>
          <el-button type="primary" @click="showAddPackageDialog">
            <el-icon><Plus /></el-icon>
            添加资源
          </el-button>
        </div>

        <el-table :data="myPackages" v-loading="loading" style="width: 100%">
          <el-table-column prop="name" label="资源名称" min-width="200">
            <template #default="{ row }">
              <div class="package-name">
                <span class="name">{{ row.name }}</span>
                <el-tag size="small" :type="getStatusType(row.status)">
                  {{ getStatusLabel(row.status) }}
                </el-tag>
              </div>
            </template>
          </el-table-column>
          <el-table-column prop="version" label="版本" width="100" />
          <el-table-column prop="category" label="分类" width="120" />
          <el-table-column prop="download_count" label="下载次数" width="100" />
          <el-table-column prop="create_time" label="创建时间" width="180">
            <template #default="{ row }">
              {{ formatTime(row.create_time) }}
            </template>
          </el-table-column>
          <el-table-column label="操作" width="200" fixed="right">
            <template #default="{ row }">
              <el-button size="small" @click="editPackage(row)">编辑</el-button>
              <el-button size="small" type="danger" @click="deletePackage(row)">删除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- 最近活动 -->
      <div class="content-section">
        <div class="section-header">
          <h2 class="section-title">最近活动</h2>
        </div>

        <div class="activity-list">
          <div v-for="activity in recentActivities" :key="activity.id" class="activity-item">
            <div class="activity-icon">
              <el-icon><Clock /></el-icon>
            </div>
            <div class="activity-content">
              <div class="activity-title">{{ activity.title }}</div>
              <div class="activity-time">{{ formatTime(activity.time) }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加资源对话框 -->
    <el-dialog v-model="addPackageDialogVisible" title="添加资源" width="600px">
      <el-form :model="newPackage" label-width="100px">
        <el-form-item label="资源名称" required>
          <el-input v-model="newPackage.name" placeholder="请输入资源名称" />
        </el-form-item>
        <el-form-item label="作者" required>
          <el-input v-model="newPackage.author" placeholder="请输入作者名称" />
        </el-form-item>
        <el-form-item label="版本" required>
          <el-input v-model="newPackage.version" placeholder="请输入版本号" />
        </el-form-item>
        <el-form-item label="分类" required>
          <el-select v-model="newPackage.category" placeholder="请选择分类">
            <el-option
              v-for="category in categories"
              :key="category.id"
              :label="category.name"
              :value="category.name"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="描述" required>
          <el-input
            v-model="newPackage.desc"
            type="textarea"
            :rows="3"
            placeholder="请输入资源描述"
          />
        </el-form-item>
        <el-form-item label="下载链接" required>
          <el-input v-model="newPackage.url" placeholder="请输入下载链接" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addPackageDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="addPackage" :loading="submitting">
          添加
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Box,
  Download,
  View,
  Star,
  Plus,
  Clock
} from '@element-plus/icons-vue'
import { getPackages, addPackage, deletePackage as deletePackageApi } from '@/api'
import { getCategories } from '@/api'

// 响应式数据
const loading = ref(false)
const submitting = ref(false)
const addPackageDialogVisible = ref(false)
const myPackages = ref([])
const categories = ref([])
const recentActivities = ref([])

// 用户信息
const userInfo = ref({
  username: 'admin',
  nickname: '管理员',
  star: 5
})

// 统计数据
const stats = ref({
  myPackages: 0,
  downloadCount: 0,
  viewCount: 0,
  star: 5
})

// 新资源表单
const newPackage = ref({
  name: '',
  author: '',
  version: '',
  category: '',
  desc: '',
  url: ''
})

// 计算属性
const userPackages = computed(() => {
  return myPackages.value.filter(pkg => pkg.author === userInfo.value.username)
})

// 方法
const loadData = async () => {
  loading.value = true
  try {
    // 加载资源列表
    const packagesRes = await getPackages()
    if (packagesRes.code === 0) {
      myPackages.value = packagesRes.data.filter(pkg => 
        pkg.author === userInfo.value.username
      )
      stats.value.myPackages = myPackages.value.length
      
      // 计算统计数据
      stats.value.downloadCount = myPackages.value.reduce((sum, pkg) => 
        sum + (pkg.download_count || 0), 0
      )
      stats.value.viewCount = myPackages.value.reduce((sum, pkg) => 
        sum + (pkg.view_count || 0), 0
      )
    }

    // 加载分类列表
    const categoriesRes = await getCategories()
    if (categoriesRes.code === 0) {
      categories.value = categoriesRes.data
    }

    // 模拟最近活动
    recentActivities.value = [
      {
        id: '1',
        title: '添加了资源 "AutoHotkey 脚本管理器"',
        time: new Date().toISOString()
      },
      {
        id: '2',
        title: '更新了资源 "键盘快捷键工具"',
        time: new Date(Date.now() - 86400000).toISOString()
      },
      {
        id: '3',
        title: '下载了资源 "文本处理工具"',
        time: new Date(Date.now() - 172800000).toISOString()
      }
    ]
  } catch (error) {
    console.error('加载数据失败:', error)
    ElMessage.error('加载数据失败')
  } finally {
    loading.value = false
  }
}

const showAddPackageDialog = () => {
  addPackageDialogVisible.value = true
  newPackage.value = {
    name: '',
    author: userInfo.value.username,
    version: '',
    category: '',
    desc: '',
    url: ''
  }
}

const addPackage = async () => {
  if (!newPackage.value.name || !newPackage.value.author || 
      !newPackage.value.version || !newPackage.value.category || 
      !newPackage.value.desc || !newPackage.value.url) {
    ElMessage.warning('请填写完整信息')
    return
  }

  submitting.value = true
  try {
    const res = await addPackage({
      ...newPackage.value,
      username: userInfo.value.username
    })
    
    if (res.code === 0) {
      ElMessage.success('添加成功')
      addPackageDialogVisible.value = false
      loadData()
    } else {
      ElMessage.error(res.msg || '添加失败')
    }
  } catch (error) {
    console.error('添加资源失败:', error)
    ElMessage.error('添加失败')
  } finally {
    submitting.value = false
  }
}

const editPackage = (package: any) => {
  ElMessage.info('编辑功能开发中...')
}

const deletePackage = async (package: any) => {
  try {
    await ElMessageBox.confirm(
      `确定要删除资源 "${package.name}" 吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    const res = await deletePackageApi(package.id, userInfo.value.username)
    if (res.code === 0) {
      ElMessage.success('删除成功')
      loadData()
    } else {
      ElMessage.error(res.msg || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除资源失败:', error)
      ElMessage.error('删除失败')
    }
  }
}

const getStatusType = (status: string) => {
  switch (status) {
    case 'active': return 'success'
    case 'pending': return 'warning'
    case 'rejected': return 'danger'
    default: return 'info'
  }
}

const getStatusLabel = (status: string) => {
  switch (status) {
    case 'active': return '已发布'
    case 'pending': return '审核中'
    case 'rejected': return '已拒绝'
    default: return '未知'
  }
}

const formatTime = (time: string) => {
  return new Date(time).toLocaleString('zh-CN')
}

// 生命周期
onMounted(() => {
  loadData()
})
</script>

<style scoped>
.user-dashboard {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.page-subtitle {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 24px;
  margin-bottom: 32px;
}

.stat-card {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 24px;
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: var(--brand-color);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 24px;
}

.stat-content {
  flex: 1;
}

.stat-value {
  font-size: 32px;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: var(--text-secondary);
}

.main-content {
  display: grid;
  gap: 32px;
}

.content-section {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.package-name {
  display: flex;
  align-items: center;
  gap: 8px;
}

.package-name .name {
  font-weight: 500;
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 8px;
  background: var(--bg-secondary);
  transition: all 0.3s ease;
}

.activity-item:hover {
  background: var(--bg-primary);
}

.activity-icon {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: var(--brand-color);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 16px;
}

.activity-content {
  flex: 1;
}

.activity-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.activity-time {
  font-size: 12px;
  color: var(--text-secondary);
}

@media (max-width: 768px) {
  .user-dashboard {
    padding: 16px;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
  }
  
  .stat-card {
    padding: 16px;
  }
  
  .stat-value {
    font-size: 24px;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
}
</style> 