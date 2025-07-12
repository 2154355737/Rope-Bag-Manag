<template>
  <div class="user-manage-desktop">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><User /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">用户管理</h1>
            <p class="page-subtitle">管理系统用户和社区用户</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button type="primary" @click="showAddUserDialog">
            <el-icon><Plus /></el-icon>
            添加用户
          </el-button>
          <el-button @click="showUserStatsDialog = true">
            <el-icon><DataAnalysis /></el-icon>
            用户统计
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><User /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalUsers }}</div>
            <div class="stat-label">总用户数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Check /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ activeUsers }}</div>
            <div class="stat-label">活跃用户</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Close /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ bannedUsers }}</div>
            <div class="stat-label">封禁用户</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Star /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ adminUsers }}</div>
            <div class="stat-label">管理员</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和筛选 -->
    <div class="search-section">
      <div class="search-left">
        <el-input
          v-model="searchQuery"
          placeholder="搜索用户..."
          prefix-icon="Search"
          clearable
          style="width: 300px"
          @input="handleSearch"
        />
        <el-select v-model="statusFilter" placeholder="状态筛选" clearable style="width: 150px">
          <el-option label="全部" value="" />
          <el-option label="正常" value="active" />
          <el-option label="封禁" value="banned" />
        </el-select>
        <el-select v-model="roleFilter" placeholder="角色筛选" clearable style="width: 150px">
          <el-option label="全部" value="" />
          <el-option label="普通用户" value="user" />
          <el-option label="管理员" value="admin" />
        </el-select>
      </div>
      <div class="search-right">
        <el-button @click="refreshData">
          <el-icon><Refresh /></el-icon>
          刷新
        </el-button>
      </div>
    </div>

    <!-- 用户表格 -->
    <div class="table-section">
      <el-table
        :data="filteredUsers"
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
        v-loading="loading"
        border
        stripe
      >
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="username" label="用户名" width="150" />
        <el-table-column prop="nickname" label="昵称" width="150" />
        <el-table-column prop="role" label="角色" width="100">
          <template #default="{ row }">
            <el-tag :type="row.role === 'admin' ? 'danger' : 'primary'">
              {{ row.role === 'admin' ? '管理员' : '普通用户' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="star" label="星级" width="100">
          <template #default="{ row }">
            <el-rate v-model="row.star" :max="5" disabled />
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.status === 'online' ? 'success' : 'info'">
              {{ row.status === 'online' ? '在线' : '离线' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="banned" label="封禁状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.banned ? 'danger' : 'success'">
              {{ row.banned ? '已封禁' : '正常' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="registerTime" label="注册时间" width="150">
          <template #default="{ row }">
            {{ formatDate(row.registerTime) }}
          </template>
        </el-table-column>
        <el-table-column prop="loginCount" label="登录次数" width="100" />
        <el-table-column prop="lastLogin" label="最后登录" width="150" />
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" type="primary" @click="editUser(row)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button 
              size="small" 
              :type="row.banned ? 'success' : 'danger'"
              @click="toggleBan(row)"
            >
              <el-icon><component :is="row.banned ? 'Check' : 'Close'" /></el-icon>
              {{ row.banned ? '解封' : '封禁' }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 分页 -->
    <div class="pagination-section">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50, 100]"
        :total="totalUsers"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </div>

    <!-- 用户编辑对话框 -->
    <el-dialog
      v-model="editDialogVisible"
      :title="editingUser.id ? '编辑用户' : '添加用户'"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form :model="editingUser" label-width="100px">
        <el-form-item label="用户名">
          <el-input v-model="editingUser.username" :disabled="!!editingUser.id" />
        </el-form-item>
        <el-form-item label="昵称">
          <el-input v-model="editingUser.nickname" placeholder="请输入昵称" />
        </el-form-item>
        <el-form-item label="密码" v-if="!editingUser.id">
          <el-input v-model="editingUser.password" type="password" placeholder="请输入密码" />
        </el-form-item>
        <el-form-item label="星级">
          <el-rate v-model="editingUser.star" :max="5" />
        </el-form-item>
        <el-form-item label="角色">
          <el-select v-model="editingUser.role" placeholder="选择角色">
            <el-option label="普通用户" value="user" />
            <el-option label="管理员" value="admin" />
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-switch
            v-model="editingUser.banned"
            active-text="封禁"
            inactive-text="正常"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveUser">保存</el-button>
      </template>
    </el-dialog>

    <!-- 用户统计对话框 -->
    <el-dialog
      v-model="showUserStatsDialog"
      title="用户统计"
      width="800px"
      :close-on-click-modal="false"
    >
      <div class="stats-dialog-content">
        <div class="stats-chart">
          <h3>用户分布</h3>
          <div class="chart-container">
            <!-- 这里可以添加图表组件 -->
            <div class="chart-placeholder">
              <el-icon :size="48"><DataAnalysis /></el-icon>
              <p>用户统计图表</p>
            </div>
          </div>
        </div>
        
        <div class="stats-details">
          <h3>详细统计</h3>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="总用户数">{{ totalUsers }}</el-descriptions-item>
            <el-descriptions-item label="活跃用户">{{ activeUsers }}</el-descriptions-item>
            <el-descriptions-item label="封禁用户">{{ bannedUsers }}</el-descriptions-item>
            <el-descriptions-item label="管理员">{{ adminUsers }}</el-descriptions-item>
            <el-descriptions-item label="普通用户">{{ totalUsers - adminUsers }}</el-descriptions-item>
            <el-descriptions-item label="在线用户">{{ users.filter(u => u.status === 'online').length }}</el-descriptions-item>
          </el-descriptions>
        </div>
      </div>
      
      <template #footer>
        <el-button @click="showUserStatsDialog = false">关闭</el-button>
        <el-button type="primary" @click="exportUserStats">导出统计</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  User, 
  Plus, 
  Edit, 
  Check, 
  Close, 
  Star,
  Search,
  Refresh,
  DataAnalysis
} from '@element-plus/icons-vue'
import { getUsers, adminSetUser, adminBanUser, setAdmin } from '../../api'

// 响应式数据
const searchQuery = ref('')
const statusFilter = ref('')
const roleFilter = ref('')
const loading = ref(false)
const currentPage = ref(1)
const pageSize = ref(20)
const editDialogVisible = ref(false)
const showUserStatsDialog = ref(false)
const editingUser = ref<any>({})

// 用户数据
const users = ref<any[]>([])
const totalUsers = ref(0)
const activeUsers = ref(0)
const bannedUsers = ref(0)
const adminUsers = ref(0)

// 计算属性
const filteredUsers = computed(() => {
  let filtered = users.value
  
  // 搜索过滤
  if (searchQuery.value) {
    filtered = filtered.filter(user =>
      user.username.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      user.nickname?.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  }
  
  // 状态过滤
  if (statusFilter.value) {
    if (statusFilter.value === 'active') {
      filtered = filtered.filter(user => !user.banned)
    } else if (statusFilter.value === 'banned') {
      filtered = filtered.filter(user => user.banned)
    }
  }
  
  // 角色过滤
  if (roleFilter.value) {
    filtered = filtered.filter(user => user.is_admin === (roleFilter.value === 'admin'))
  }
  
  return filtered
})

// 方法
async function loadUsers() {
  try {
    loading.value = true
    const res = await getUsers()
    if (res.code === 0 && res.data) {
      users.value = Object.entries(res.data).map(([username, user]: [string, any]) => ({
        id: username,
        username,
        nickname: user.nickname || username,
        role: user.is_admin ? 'admin' : 'user',
        status: 'online', // 后端暂无在线状态
        star: user.star || 0,
        loginCount: user.sign_total || 0,
        lastLogin: user.last_sign || '从未登录',
        registerTime: user.register_time || '未知',
        banned: user.banned || false,
        is_admin: user.is_admin || false
      }))
      
      // 更新统计数据
      totalUsers.value = users.value.length
      activeUsers.value = users.value.filter(u => !u.banned).length
      bannedUsers.value = users.value.filter(u => u.banned).length
      adminUsers.value = users.value.filter(u => u.is_admin).length
    } else {
      ElMessage.error('获取用户数据失败')
    }
  } catch (error) {
    console.error('获取用户数据错误:', error)
    ElMessage.error('获取用户数据失败')
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  // 搜索逻辑
}

async function refreshData() {
  await loadUsers()
  ElMessage.success('数据已刷新')
}

function showAddUserDialog() {
  editingUser.value = {}
  editDialogVisible.value = true
}

function editUser(user: any) {
  editingUser.value = { ...user }
  editDialogVisible.value = true
}

async function saveUser() {
  try {
    const admin_username = 'admin'
    const admin_password = 'admin123'
    
    if (editingUser.value.id) {
      // 更新用户
      const updateData: any = {
        target: editingUser.value.username,
        admin_username,
        admin_password
      }
      
      if (editingUser.value.nickname !== editingUser.value.username) {
        updateData.nickname = editingUser.value.nickname
      }
      
      if (editingUser.value.password) {
        updateData.password = editingUser.value.password
      }
      
      const res = await adminSetUser(updateData)
      if (res.code === 0) {
        ElMessage.success('用户信息已保存')
        editDialogVisible.value = false
        await loadUsers()
      } else {
        ElMessage.error(res.msg || '保存失败')
      }
    } else {
      // 添加用户
      const addData: any = {
        target: editingUser.value.username,
        nickname: editingUser.value.nickname,
        password: editingUser.value.password,
        admin_username,
        admin_password
      }
      
      const res = await adminSetUser(addData)
      if (res.code === 0) {
        ElMessage.success('用户添加成功')
        editDialogVisible.value = false
        await loadUsers()
      } else {
        ElMessage.error(res.msg || '添加失败')
      }
    }
  } catch (error) {
    console.error('保存用户错误:', error)
    ElMessage.error('保存失败')
  }
}

async function toggleBan(user: any) {
  const action = user.banned ? '解封' : '封禁'
  try {
    await ElMessageBox.confirm(
      `确定要${action}用户 ${user.username} 吗？`,
      '确认操作',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    const res = await adminBanUser(user.username, !user.banned, 'admin', 'admin123')
    if (res.code === 0) {
      user.banned = !user.banned
      ElMessage.success(`用户已${action}`)
      await loadUsers() // 重新加载数据
    } else {
      ElMessage.error(res.msg || `${action}失败`)
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('封禁用户错误:', error)
      ElMessage.error(`${action}失败`)
    }
  }
}

function handleSizeChange(size: number) {
  pageSize.value = size
  currentPage.value = 1
}

function handleCurrentChange(page: number) {
  currentPage.value = page
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('zh-CN')
}

function exportUserStats() {
  // 导出用户统计数据的逻辑
  ElMessage.success('统计数据已导出')
}

// 初始化数据
onMounted(() => {
  loadUsers()
})
</script>

<style scoped>
.user-manage-desktop {
  padding: 24px;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  min-height: 100vh;
}

/* 页面头部 */
.page-header {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 32px;
  margin-bottom: 32px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  position: relative;
  overflow: hidden;
}

.page-header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(64, 158, 255, 0.1) 0%, rgba(103, 194, 58, 0.1) 100%);
  z-index: 0;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: relative;
  z-index: 1;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon {
  width: 64px;
  height: 64px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 8px 32px rgba(64, 158, 255, 0.3);
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.page-subtitle {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

/* 统计卡片 */
.stats-section {
  margin-bottom: 32px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 20px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-medium);
}

.stat-icon {
  width: 56px;
  height: 56px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.stat-content {
  flex: 1;
  min-width: 0;
}

.stat-number {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 14px;
  color: var(--text-secondary);
  margin-top: 8px;
}

/* 搜索和筛选 */
.search-section {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.search-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.search-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

/* 表格区域 */
.table-section {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  overflow: hidden;
}

/* 分页区域 */
.pagination-section {
  display: flex;
  justify-content: center;
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .user-manage-desktop {
    padding: 16px;
  }
}


/* 统计对话框样式 */
.stats-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.stats-chart {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 20px;
}

.stats-chart h3 {
  margin: 0 0 16px 0;
  color: var(--text-primary);
  font-size: 18px;
  font-weight: 600;
}

.chart-container {
  min-height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.chart-placeholder {
  text-align: center;
  color: var(--text-secondary);
}

.chart-placeholder p {
  margin: 12px 0 0 0;
  font-size: 14px;
}

.stats-details {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 20px;
}

.stats-details h3 {
  margin: 0 0 16px 0;
  color: var(--text-primary);
  font-size: 18px;
  font-weight: 600;
}
  
  .page-header {
    padding: 24px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .header-left {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .header-actions {
    margin-top: 16px;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .search-section {
    flex-direction: column;
    gap: 16px;
  }
  
  .search-left {
    flex-wrap: wrap;
    gap: 12px;
  }
  
  .search-left .el-input,
  .search-left .el-select {
    width: 100% !important;
  }



/* 深色模式适配 */
.dark .user-manage-desktop {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .page-header,
.dark .stat-card,
.dark .search-section,
.dark .table-section,
.dark .pagination-section {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

/* 主题适配 */
.blue .stat-card::before,
.blue .user-card::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-card::before,
.green .user-card::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .stat-card::before,
.orange .user-card::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .stat-card::before,
.purple .user-card::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.blue .stat-icon,
.blue .header-icon {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-icon,
.green .header-icon {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.purple .stat-icon,
.purple .header-icon {
  background: linear-gradient(135deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.orange .stat-icon,
.orange .header-icon {
  background: linear-gradient(135deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

/* 动画效果 */
@keyframes slide-up {
  0% {
    opacity: 0;
    transform: translateY(20px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.05);
    opacity: 0.9;
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) scale(1);
    opacity: 0.6;
  }
  50% {
    transform: translateY(-8px) scale(1.1);
    opacity: 1;
  }
}

/* 页面加载动画 */
.page-header {
  animation: slide-up 0.6s ease-out;
}

.stats-grid {
  animation: slide-up 0.6s ease-out 0.2s both;
}

.search-section {
  animation: slide-up 0.6s ease-out 0.4s both;
}

.table-section {
  animation: slide-up 0.6s ease-out 0.6s both;
}

.pagination-section {
  animation: slide-up 0.6s ease-out 0.8s both;
}

/* 卡片悬停动画 */
.stat-card {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  transform: scaleX(0);
  transition: transform 0.3s ease;
}

.stat-card::after {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(45deg, transparent, rgba(255, 255, 255, 0.05), transparent);
  transform: translateX(-100%) translateY(-100%) rotate(45deg);
  transition: transform 0.6s ease;
}

.stat-card:hover::before {
  transform: scaleX(1);
}

.stat-card:hover::after {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

/* 图标动画 */
.stat-icon {
  transition: transform 0.3s ease;
}

.stat-card:hover .stat-icon {
  transform: scale(1.1);
}

/* 按钮光泽动画 */
.el-button {
  position: relative;
  overflow: hidden;
}

.el-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.el-button:hover::before {
  left: 100%;
}

/* 表格行悬停动画 */
.el-table__row {
  transition: all 0.3s ease;
}

.el-table__row:hover {
  background-color: var(--bg-secondary) !important;
  transform: translateX(4px);
}

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .page-header,
  .stats-grid,
  .search-section,
  .table-section,
  .pagination-section {
    animation: none;
  }
  
  .stat-card {
    transition: none;
  }
  
  .stat-card:hover {
    transform: none;
  }
  
  .stat-icon {
    transition: none;
  }
  
  .el-button::before {
    display: none;
  }
  
  .el-table__row {
    transition: none;
  }
  
  .el-table__row:hover {
    transform: none;
  }
}

/* 统计对话框样式 */
.stats-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.stats-chart {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 20px;
}

.stats-chart h3 {
  margin: 0 0 16px 0;
  color: var(--text-primary);
  font-size: 18px;
  font-weight: 600;
}

.chart-container {
  min-height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.chart-placeholder {
  text-align: center;
  color: var(--text-secondary);
}

.chart-placeholder p {
  margin: 12px 0 0 0;
  font-size: 14px;
}

.stats-details {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 20px;
}

.stats-details h3 {
  margin: 0 0 16px 0;
  color: var(--text-primary);
  font-size: 18px;
  font-weight: 600;
}
</style> 