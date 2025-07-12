<template>
  <div class="user-manage-mobile">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="24"><User /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">用户管理</h1>
            <p class="page-subtitle">管理系统用户和权限</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button 
            type="primary" 
            size="small"
            @click="addUser"
            :icon="Plus"
          >
            添加用户
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="20"><User /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalUsers }}</div>
            <div class="stat-label">总用户</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="20"><Star /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ activeUsers }}</div>
            <div class="stat-label">活跃用户</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="20"><Close /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ bannedUsers }}</div>
            <div class="stat-label">封禁用户</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和筛选 -->
    <div class="search-section">
      <div class="search-box">
        <el-input
          v-model="searchQuery"
          placeholder="搜索用户..."
          prefix-icon="Search"
          clearable
          @input="handleSearch"
        />
      </div>
      <div class="filter-tabs">
        <div 
          v-for="tab in filterTabs" 
          :key="tab.key"
          class="filter-tab"
          :class="{ active: activeFilter === tab.key }"
          @click="setFilter(tab.key)"
        >
          <span class="tab-icon">{{ tab.icon }}</span>
          <span class="tab-label">{{ tab.label }}</span>
          <span class="tab-count">{{ tab.count }}</span>
        </div>
      </div>
    </div>

    <!-- 用户列表 -->
    <div class="users-list">
      <div v-for="user in filteredUsers" :key="user.id" class="user-card">
        <div class="user-avatar">
          <el-avatar :size="48" :src="user.avatar">
            {{ user.username.charAt(0).toUpperCase() }}
          </el-avatar>
          <div class="user-status" :class="user.status">
            <div class="status-dot"></div>
          </div>
        </div>
        
        <div class="user-info">
          <div class="user-header">
            <h3 class="user-name">{{ user.username }}</h3>
            <div class="user-badge" :class="user.role">
              {{ user.role === 'admin' ? '管理员' : '普通用户' }}
            </div>
          </div>
          
          <div class="user-details">
            <div class="detail-item">
              <el-icon><User /></el-icon>
              <span>{{ user.nickname || '未设置昵称' }}</span>
            </div>
            <div class="detail-item">
              <el-icon><Star /></el-icon>
              <span>星级: {{ user.star || 0 }}</span>
            </div>
            <div class="detail-item">
              <el-icon><Calendar /></el-icon>
              <span>注册: {{ formatDate(user.registerTime) }}</span>
            </div>
          </div>
          
          <div class="user-stats">
            <div class="stat-mini">
              <span class="stat-value">{{ user.loginCount }}</span>
              <span class="stat-label">登录次数</span>
            </div>
            <div class="stat-mini">
              <span class="stat-value">{{ user.lastLogin }}</span>
              <span class="stat-label">最后登录</span>
            </div>
          </div>
        </div>
        
        <div class="user-actions">
          <el-button 
            size="small" 
            type="primary" 
            @click="editUser(user)"
            :icon="Edit"
          >
            编辑
          </el-button>
          <el-button 
            size="small" 
            :type="user.banned ? 'success' : 'danger'"
            @click="toggleBan(user)"
            :icon="user.banned ? 'Check' : 'Close'"
          >
            {{ user.banned ? '解封' : '封禁' }}
          </el-button>
        </div>
      </div>
    </div>

    <!-- 加载更多 -->
    <div v-if="hasMore" class="load-more">
      <el-button 
        type="primary" 
        plain 
        @click="loadMore"
        :loading="loading"
      >
        加载更多
      </el-button>
    </div>

    <!-- 空状态 -->
    <div v-if="filteredUsers.length === 0" class="empty-state">
      <div class="empty-icon">
        <el-icon :size="48"><User /></el-icon>
      </div>
      <h3 class="empty-title">暂无用户</h3>
      <p class="empty-desc">当前筛选条件下没有找到用户</p>
    </div>

    <!-- 用户编辑对话框 -->
    <el-dialog
      v-model="editDialogVisible"
      title="编辑用户"
      width="90%"
      :close-on-click-modal="false"
    >
      <div class="edit-form">
        <el-form :model="editingUser" label-width="80px">
          <el-form-item label="用户名">
            <el-input v-model="editingUser.username" disabled />
          </el-form-item>
          <el-form-item label="昵称">
            <el-input v-model="editingUser.nickname" placeholder="请输入昵称" />
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
      </div>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveUser">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  User, 
  Star, 
  Calendar, 
  Edit, 
  Search,
  Check,
  Close,
  Plus
} from '@element-plus/icons-vue'
import { getUsers, banUser, setStar, setNickname } from '../../api'

// 响应式数据
const searchQuery = ref('')
const activeFilter = ref('all')
const loading = ref(false)
const editDialogVisible = ref(false)
const editingUser = ref<any>({})

// 用户数据
const users = ref<any[]>([])
const totalUsers = ref(0)
const activeUsers = ref(0)
const bannedUsers = ref(0)

// 筛选标签
const filterTabs = ref([
  { key: 'all', label: '全部', icon: '👥', count: 0 },
  { key: 'online', label: '在线', icon: '🟢', count: 0 },
  { key: 'banned', label: '封禁', icon: '🚫', count: 0 },
  { key: 'admin', label: '管理员', icon: '👑', count: 0 }
])

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
  switch (activeFilter.value) {
    case 'online':
      filtered = filtered.filter(user => !user.banned && user.status === 'online')
      break
    case 'banned':
      filtered = filtered.filter(user => user.banned)
      break
    case 'admin':
      filtered = filtered.filter(user => user.role === 'admin')
      break
  }
  
  return filtered
})

const hasMore = computed(() => {
  return filteredUsers.value.length < users.value.length
})

// 方法
function handleSearch() {
  // 搜索逻辑
}

function setFilter(filter: string) {
  activeFilter.value = filter
}

function editUser(user: any) {
  editingUser.value = { ...user }
  editDialogVisible.value = true
}

function saveUser() {
  // 保存用户信息
  ElMessage.success('用户信息已更新')
  editDialogVisible.value = false
}

function toggleBan(user: any) {
  const action = user.banned ? '解封' : '封禁'
  ElMessageBox.confirm(
    `确定要${action}用户 ${user.username} 吗？`,
    '确认操作',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    }
  ).then(() => {
    user.banned = !user.banned
    ElMessage.success(`用户已${action}`)
  }).catch(() => {
    // 用户取消
  })
}

function addUser() {
  ElMessage.info('添加用户功能开发中')
}

function loadMore() {
  // 加载更多用户
  loading.value = true
  setTimeout(() => {
    loading.value = false
    ElMessage.success('已加载更多用户')
  }, 1000)
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('zh-CN')
}

// 初始化数据
onMounted(() => {
  // 模拟数据
  users.value = [
    {
      id: 1,
      username: 'admin',
      nickname: '系统管理员',
      avatar: '',
      role: 'admin',
      status: 'online',
      star: 5,
      loginCount: 156,
      lastLogin: '2小时前',
      registerTime: '2024-01-01',
      banned: false
    },
    {
      id: 2,
      username: 'user001',
      nickname: '张三',
      avatar: '',
      role: 'user',
      status: 'online',
      star: 3,
      loginCount: 89,
      lastLogin: '1天前',
      registerTime: '2024-01-15',
      banned: false
    },
    {
      id: 3,
      username: 'user002',
      nickname: '李四',
      avatar: '',
      role: 'user',
      status: 'offline',
      star: 2,
      loginCount: 45,
      lastLogin: '3天前',
      registerTime: '2024-02-01',
      banned: true
    }
  ]
  
  totalUsers.value = users.value.length
  activeUsers.value = users.value.filter(u => !u.banned && u.status === 'online').length
  bannedUsers.value = users.value.filter(u => u.banned).length
  
  // 更新筛选标签数量
  filterTabs.value[0].count = totalUsers.value
  filterTabs.value[1].count = activeUsers.value
  filterTabs.value[2].count = bannedUsers.value
  filterTabs.value[3].count = users.value.filter(u => u.role === 'admin').length
})
</script>

<style scoped>
.user-manage-mobile {
  padding: 16px;
  min-height: 100vh;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

/* 页面头部 */
.page-header {
  margin-bottom: 24px;
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-icon {
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 6px 24px rgba(64, 158, 255, 0.3);
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 统计卡片 */
.stats-section {
  margin-bottom: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  gap: 12px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-medium);
}

.stat-icon {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 10px;
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
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}

/* 搜索和筛选 */
.search-section {
  margin-bottom: 24px;
}

.search-box {
  margin-bottom: 16px;
}

.search-box .el-input {
  border-radius: 12px;
}

.filter-tabs {
  display: flex;
  gap: 8px;
  overflow-x: auto;
  padding-bottom: 4px;
}

.filter-tab {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
  flex-shrink: 0;
}

.filter-tab:hover {
  background: var(--bg-primary);
  border-color: var(--brand-color);
}

.filter-tab.active {
  background: var(--brand-color);
  border-color: var(--brand-color);
  color: white;
}

.tab-icon {
  font-size: 14px;
}

.tab-label {
  font-size: 12px;
  font-weight: 500;
}

.tab-count {
  font-size: 10px;
  background: rgba(255, 255, 255, 0.2);
  padding: 2px 6px;
  border-radius: 10px;
  min-width: 16px;
  text-align: center;
}

/* 用户列表 */
.users-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 24px;
}

.user-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: flex-start;
  gap: 16px;
  transition: all 0.3s ease;
}

.user-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-medium);
}

.user-avatar {
  position: relative;
  flex-shrink: 0;
}

.user-status {
  position: absolute;
  bottom: 2px;
  right: 2px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 2px solid var(--bg-card);
}

.user-status.online .status-dot {
  background: #67c23a;
}

.user-status.offline .status-dot {
  background: var(--text-secondary);
}

.status-dot {
  width: 100%;
  height: 100%;
  border-radius: 50%;
}

.user-info {
  flex: 1;
  min-width: 0;
}

.user-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.user-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.user-badge {
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 10px;
  font-weight: 500;
}

.user-badge.admin {
  background: linear-gradient(135deg, #f56c6c 0%, #f78989 100%);
  color: white;
}

.user-badge.user {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  color: white;
}

.user-details {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.detail-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.detail-item .el-icon {
  font-size: 12px;
  color: var(--brand-color);
}

.user-stats {
  display: flex;
  gap: 16px;
}

.stat-mini {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.stat-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.stat-label {
  font-size: 10px;
  color: var(--text-secondary);
}

.user-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex-shrink: 0;
}

/* 加载更多 */
.load-more {
  display: flex;
  justify-content: center;
  margin-bottom: 24px;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.empty-icon {
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.empty-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

/* 编辑对话框 */
.edit-form {
  padding: 20px 0;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .user-card {
    flex-direction: column;
    align-items: center;
    text-align: center;
  }
  
  .user-actions {
    flex-direction: row;
    width: 100%;
    justify-content: center;
  }
}

/* 深色模式适配 */
.dark .user-manage-mobile {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .page-header,
.dark .stat-card,
.dark .user-card,
.dark .filter-tab {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

.dark .filter-tab.active {
  background: var(--brand-color);
  border-color: var(--brand-color);
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

.filter-tabs {
  animation: slide-up 0.6s ease-out 0.4s both;
}

.user-list {
  animation: slide-up 0.6s ease-out 0.6s both;
}

/* 卡片悬停动画 */
.stat-card,
.user-card {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.stat-card::before,
.user-card::before {
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

.stat-card::after,
.user-card::after {
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

.stat-card:hover,
.user-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-medium);
}

.stat-card:hover::before,
.user-card:hover::before {
  transform: scaleX(1);
}

.stat-card:hover::after,
.user-card:hover::after {
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

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .page-header,
  .stats-grid,
  .filter-tabs,
  .user-list {
    animation: none;
  }
  
  .stat-card,
  .user-card {
    transition: none;
  }
  
  .stat-card:hover,
  .user-card:hover {
    transform: none;
  }
  
  .stat-icon {
    transition: none;
  }
  
  .el-button::before {
    display: none;
  }
}
</style> 