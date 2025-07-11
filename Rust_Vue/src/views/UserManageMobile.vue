<template>
  <div class="user-manage-mobile">
    <AutoSwitchNotice />
    <!-- 顶部统计卡片 -->
    <div class="stats-cards">
      <div class="stat-card total">
        <div class="stat-number">{{ totalUsers }}</div>
        <div class="stat-label">总用户</div>
      </div>
      <div class="stat-card normal">
        <div class="stat-number">{{ normalUsers }}</div>
        <div class="stat-label">正常</div>
      </div>
      <div class="stat-card banned">
        <div class="stat-number">{{ bannedUsers }}</div>
        <div class="stat-label">封禁</div>
      </div>
    </div>

    <!-- 搜索和筛选区域 -->
    <div class="search-section">
      <div class="search-input">
        <el-input
          v-model="searchKeyword"
          placeholder="搜索用户名或昵称"
          clearable
          @input="handleSearch"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
      </div>
      
      <div class="filter-row">
        <el-select v-model="statusFilter" placeholder="状态" clearable @change="handleSearch" class="filter-select">
          <el-option label="全部" value="" />
          <el-option label="正常" value="normal" />
          <el-option label="封禁" value="banned" />
        </el-select>
        
        <el-select v-model="starFilter" placeholder="星级" clearable @change="handleSearch" class="filter-select">
          <el-option label="全部" value="" />
          <el-option label="1星" value="1" />
          <el-option label="2星" value="2" />
          <el-option label="3星" value="3" />
          <el-option label="4星" value="4" />
          <el-option label="5星" value="5" />
        </el-select>
      </div>
      
      <div class="action-buttons">
        <el-button type="primary" @click="handleSearch" size="small">搜索</el-button>
        <el-button @click="resetSearch" size="small">重置</el-button>
        <el-button type="success" @click="exportUsers" size="small">导出</el-button>
      </div>
    </div>

    <!-- 用户卡片列表 -->
    <div class="user-cards" v-loading="loading">
      <div 
        v-for="user in paginatedUsers" 
        :key="user.username" 
        class="user-card"
        :class="{ 'banned': user.banned }"
      >
        <div class="user-header">
          <div class="user-info">
            <div class="username">{{ user.username }}</div>
            <div class="nickname">{{ user.nickname }}</div>
          </div>
          <div class="user-status">
            <el-tag :type="user.banned ? 'danger' : 'success'" size="small">
              {{ user.banned ? '封禁' : '正常' }}
            </el-tag>
            <el-tag v-if="user.is_admin" type="warning" size="small">管理员</el-tag>
          </div>
        </div>
        
        <div class="user-details">
          <div class="detail-item">
            <span class="label">星级:</span>
            <el-rate v-model="user.star" disabled show-score />
          </div>
          <div class="detail-item">
            <span class="label">签到天数:</span>
            <span>{{ user.sign_days }}</span>
          </div>
          <div class="detail-item">
            <span class="label">总签到:</span>
            <span>{{ user.sign_total }}</span>
          </div>
          <div class="detail-item">
            <span class="label">最后签到:</span>
            <span>{{ user.last_sign || '未签到' }}</span>
          </div>
        </div>
        
        <div class="user-actions">
          <el-dropdown @command="(command: string) => handleAction(command, user)" trigger="click">
            <el-button type="primary" size="small">
              操作<el-icon class="el-icon--right"><ArrowDown /></el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item :command="user.banned ? 'unban' : 'ban'">
                  {{ user.banned ? '解封' : '封禁' }}
                </el-dropdown-item>
                <el-dropdown-item command="addStar">加星</el-dropdown-item>
                <el-dropdown-item command="removeStar">减星</el-dropdown-item>
                <el-dropdown-item command="editNickname">改昵称</el-dropdown-item>
                <el-dropdown-item command="editPassword">改密码</el-dropdown-item>
                <el-dropdown-item command="viewDetail">查看详情</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </div>
    </div>

    <!-- 分页 -->
    <div class="pagination">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50]"
        :total="filteredUsers.length"
        layout="prev, pager, next, sizes"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
        size="small"
      />
    </div>

    <!-- 修改昵称弹窗 -->
    <el-dialog v-model="showNicknameDialog" title="修改昵称" width="90%" :close-on-click-modal="false">
      <el-form :model="editForm" :rules="nicknameRules" ref="nicknameFormRef">
        <el-form-item label="用户名" prop="username">
          <el-input v-model="editForm.username" disabled />
        </el-form-item>
        <el-form-item label="新昵称" prop="nickname">
          <el-input v-model="editForm.nickname" placeholder="请输入新昵称" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showNicknameDialog = false">取消</el-button>
        <el-button type="primary" @click="submitEditNickname" :loading="submitting">确定</el-button>
      </template>
    </el-dialog>

    <!-- 修改密码弹窗 -->
    <el-dialog v-model="showPasswordDialog" title="修改密码" width="90%" :close-on-click-modal="false">
      <el-form :model="editForm" :rules="passwordRules" ref="passwordFormRef">
        <el-form-item label="用户名" prop="username">
          <el-input v-model="editForm.username" disabled />
        </el-form-item>
        <el-form-item label="新密码" prop="password">
          <el-input v-model="editForm.password" type="password" placeholder="请输入新密码" show-password />
        </el-form-item>
        <el-form-item label="确认密码" prop="confirmPassword">
          <el-input v-model="editForm.confirmPassword" type="password" placeholder="请再次输入密码" show-password />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showPasswordDialog = false">取消</el-button>
        <el-button type="primary" @click="submitEditPassword" :loading="submitting">确定</el-button>
      </template>
    </el-dialog>

    <!-- 用户详情弹窗 -->
    <el-dialog v-model="showDetailDialog" title="用户详情" width="95%" :close-on-click-modal="false">
      <div v-if="selectedUser" class="user-detail-mobile">
        <div class="detail-section">
          <h4>基本信息</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="label">用户名:</span>
              <span>{{ selectedUser.username }}</span>
            </div>
            <div class="detail-item">
              <span class="label">昵称:</span>
              <span>{{ selectedUser.nickname }}</span>
            </div>
            <div class="detail-item">
              <span class="label">状态:</span>
              <el-tag :type="selectedUser.banned ? 'danger' : 'success'">
                {{ selectedUser.banned ? '封禁' : '正常' }}
              </el-tag>
            </div>
            <div class="detail-item">
              <span class="label">管理员:</span>
              <el-tag v-if="selectedUser.is_admin" type="warning">是</el-tag>
              <span v-else>否</span>
            </div>
          </div>
        </div>
        
        <div class="detail-section">
          <h4>星级信息</h4>
          <div class="star-section">
            <el-rate v-model="selectedUser.star" disabled show-score />
          </div>
        </div>
        
        <div class="detail-section">
          <h4>签到信息</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="label">签到天数:</span>
              <span>{{ selectedUser.sign_days }}</span>
            </div>
            <div class="detail-item">
              <span class="label">总签到次数:</span>
              <span>{{ selectedUser.sign_total }}</span>
            </div>
            <div class="detail-item">
              <span class="label">最后签到:</span>
              <span>{{ selectedUser.last_sign || '未签到' }}</span>
            </div>
          </div>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, ArrowDown } from '@element-plus/icons-vue'
import { getUsers, banUser, setStar, setNickname, setPassword } from '../api'
import { useRouter } from 'vue-router'
import AutoSwitchNotice from '../components/AutoSwitchNotice.vue'

// 响应式数据
const users = ref<any[]>([])
const filteredUsers = ref<any[]>([])
const loading = ref(false)
const submitting = ref(false)

// 搜索和筛选
const searchKeyword = ref('')
const statusFilter = ref('')
const starFilter = ref('')

// 分页
const currentPage = ref(1)
const pageSize = ref(10)

// 弹窗控制
const showNicknameDialog = ref(false)
const showPasswordDialog = ref(false)
const showDetailDialog = ref(false)

// 编辑表单
const editForm = ref({
  username: '',
  nickname: '',
  password: '',
  confirmPassword: ''
})

// 选中的用户
const selectedUser = ref<any>(null)

// 表单引用
const nicknameFormRef = ref()
const passwordFormRef = ref()

const router = useRouter()

// 表单验证规则
const nicknameRules = {
  nickname: [
    { required: true, message: '请输入昵称', trigger: 'blur' },
    { min: 1, max: 20, message: '昵称长度在 1 到 20 个字符', trigger: 'blur' }
  ]
}

const passwordRules = {
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于6位', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认密码', trigger: 'blur' },
    {
      validator: (rule: any, value: string, callback: any) => {
        if (value !== editForm.value.password) {
          callback(new Error('两次输入密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

// 计算属性
const totalUsers = computed(() => users.value.length)
const normalUsers = computed(() => users.value.filter(u => !u.banned).length)
const bannedUsers = computed(() => users.value.filter(u => u.banned).length)

// 分页后的用户列表
const paginatedUsers = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredUsers.value.slice(start, end)
})

onMounted(async () => {
  await loadUsers()
})

async function loadUsers() {
  loading.value = true
  try {
    const res = await getUsers()
    if (res.code === 0 && res.data) {
      users.value = Object.values(res.data)
      handleSearch()
    }
  } catch (error) {
    console.error('加载用户数据失败:', error)
    handleNetworkError(error)
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  let filtered = [...users.value]
  
  // 关键词搜索
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    filtered = filtered.filter(user => 
      user.username.toLowerCase().includes(keyword) ||
      user.nickname.toLowerCase().includes(keyword)
    )
  }
  
  // 状态筛选
  if (statusFilter.value) {
    if (statusFilter.value === 'normal') {
      filtered = filtered.filter(user => !user.banned)
    } else if (statusFilter.value === 'banned') {
      filtered = filtered.filter(user => user.banned)
    }
  }
  
  // 星级筛选
  if (starFilter.value) {
    const star = parseInt(starFilter.value)
    filtered = filtered.filter(user => user.star === star)
  }
  
  filteredUsers.value = filtered
  currentPage.value = 1 // 重置到第一页
}

function resetSearch() {
  searchKeyword.value = ''
  statusFilter.value = ''
  starFilter.value = ''
  handleSearch()
}

function handleSizeChange(size: number) {
  pageSize.value = size
  currentPage.value = 1
}

function handleCurrentChange(page: number) {
  currentPage.value = page
}

async function handleAction(command: string, user: any) {
  switch (command) {
    case 'ban':
    case 'unban':
      await toggleBan(user)
      break
    case 'addStar':
      await changeStar(user, 1)
      break
    case 'removeStar':
      await changeStar(user, -1)
      break
    case 'editNickname':
      openEditNickname(user)
      break
    case 'editPassword':
      openEditPassword(user)
      break
    case 'viewDetail':
      viewUserDetail(user)
      break
  }
}

async function toggleBan(user: any) {
  try {
    await ElMessageBox.confirm(
      `确定要${user.banned ? '解封' : '封禁'}用户 "${user.username}" 吗？`,
      '确认操作',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    const res = await banUser(user.username, !user.banned)
    if (res.code === 0) {
      await loadUsers()
      ElMessage.success((user.banned ? '解封' : '封禁') + '成功')
    } else {
      ElMessage.error(res.msg || '操作失败')
    }
  } catch (error) {
    if (error === 'cancel') return
    console.error('封禁/解封用户失败:', error)
    handleNetworkError(error)
  }
}

async function changeStar(user: any, delta: number) {
  try {
    const newStar = Math.max(0, Math.min(5, user.star + delta))
    const res = await setStar(user.username, newStar)
    if (res.code === 0) {
      await loadUsers()
      ElMessage.success((delta > 0 ? '加星' : '减星') + '成功')
    } else {
      ElMessage.error(res.msg || '操作失败')
    }
  } catch (error) {
    console.error('修改星级失败:', error)
    handleNetworkError(error)
  }
}

function openEditNickname(user: any) {
  editForm.value = {
    username: user.username,
    nickname: user.nickname,
    password: '',
    confirmPassword: ''
  }
  showNicknameDialog.value = true
}

function openEditPassword(user: any) {
  editForm.value = {
    username: user.username,
    nickname: '',
    password: '',
    confirmPassword: ''
  }
  showPasswordDialog.value = true
}

async function submitEditNickname() {
  try {
    await nicknameFormRef.value.validate()
    submitting.value = true
    
    const res = await setNickname(editForm.value.username, editForm.value.nickname)
    if (res.code === 0) {
      await loadUsers()
      ElMessage.success('修改昵称成功')
      showNicknameDialog.value = false
    } else {
      ElMessage.error(res.msg || '操作失败')
    }
  } catch (error) {
    if (error === false) return // 表单验证失败
    console.error('修改昵称失败:', error)
    handleNetworkError(error)
  } finally {
    submitting.value = false
  }
}

async function submitEditPassword() {
  try {
    await passwordFormRef.value.validate()
    submitting.value = true
    
    const res = await setPassword(editForm.value.username, editForm.value.password)
    if (res.code === 0) {
      ElMessage.success('修改密码成功')
      showPasswordDialog.value = false
    } else {
      ElMessage.error(res.msg || '操作失败')
    }
  } catch (error) {
    if (error === false) return // 表单验证失败
    console.error('修改密码失败:', error)
    handleNetworkError(error)
  } finally {
    submitting.value = false
  }
}

function viewUserDetail(user: any) {
  selectedUser.value = user
  showDetailDialog.value = true
}

function exportUsers() {
  const data = users.value.map(user => ({
    用户名: user.username,
    昵称: user.nickname,
    星级: user.star,
    状态: user.banned ? '封禁' : '正常',
    签到天数: user.sign_days,
    总签到: user.sign_total,
    最后签到: user.last_sign || '未签到',
    管理员: user.is_admin ? '是' : '否'
  }))
  
  const csvContent = [
    Object.keys(data[0]).join(','),
    ...data.map(row => Object.values(row).join(','))
  ].join('\n')
  
  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = `用户数据_${new Date().toISOString().split('T')[0]}.csv`
  link.click()
  
  ElMessage.success('用户数据导出成功')
}

function handleNetworkError(error: any) {
  const errorMessage = error instanceof Error ? error.message : String(error)
  if (errorMessage.includes('fetch') || 
      errorMessage.includes('network') || 
      errorMessage.includes('Failed to fetch') ||
      errorMessage.includes('ERR_NETWORK') ||
      errorMessage.includes('ERR_CONNECTION_REFUSED') ||
      errorMessage.includes('Service unavailable')) {
    
    ElMessage.error('服务异常已安全退出！')
    setTimeout(() => {
      router.push('/login')
    }, 2000)
    return
  }
  
  ElMessage.error('操作失败，请稍后重试')
}
</script>

<style scoped>
.user-manage-mobile {
  padding: 16px;
  background-color: var(--vt-c-white-soft);
  min-height: 100vh;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 统计卡片 */
.stats-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin-bottom: 20px;
}

.stat-card {
  background: var(--vt-c-white);
  border-radius: 12px;
  padding: 16px;
  text-align: center;
  border: 1px solid transparent;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.stat-card.total {
  border-left: 4px solid #409eff;
}

.stat-card.normal {
  border-left: 4px solid #67c23a;
}

.stat-card.banned {
  border-left: 4px solid #f56c6c;
}

.stat-number {
  font-size: 24px;
  font-weight: bold;
  color: var(--vt-c-text-light-1);
  margin-bottom: 4px;
}

.stat-label {
  font-size: 12px;
  color: var(--vt-c-text-light-2);
  font-weight: 500;
}

/* 搜索区域 */
.search-section {
  background: var(--vt-c-white);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 20px;
  border: 1px solid transparent;
}

.search-input {
  margin-bottom: 12px;
}

.filter-row {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.filter-select {
  flex: 1;
}

.action-buttons {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.action-buttons .el-button {
  flex: 1;
  min-width: 80px;
}

/* 用户卡片 */
.user-cards {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
}

.user-card {
  background: var(--vt-c-white);
  border-radius: 12px;
  padding: 16px;
  border: 1px solid transparent;
  border-left: 4px solid #67c23a;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.user-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.user-card.banned {
  border-left-color: #f56c6c;
  opacity: 0.8;
}

.user-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}

.user-info {
  flex: 1;
}

.username {
  font-size: 16px;
  font-weight: bold;
  color: var(--vt-c-text-light-1);
  margin-bottom: 4px;
}

.nickname {
  font-size: 14px;
  color: var(--vt-c-text-light-2);
}

.user-status {
  display: flex;
  flex-direction: column;
  gap: 4px;
  align-items: flex-end;
}

.user-details {
  margin-bottom: 12px;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
  font-size: 14px;
  color: var(--vt-c-text-light-1);
}

.detail-item .label {
  color: var(--vt-c-text-light-2);
  font-weight: 500;
}

.user-actions {
  display: flex;
  justify-content: flex-end;
}

/* 分页 */
.pagination {
  display: flex;
  justify-content: center;
  margin-top: 20px;
  padding: 16px;
  background: var(--vt-c-white);
  border-radius: 12px;

  border: 1px solid transparent;
}

/* 详情弹窗 */
.user-detail-mobile {
  padding: 16px 0;
}

.detail-section {
  margin-bottom: 24px;
}

.detail-section h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: bold;
  color: var(--vt-c-text-light-1);
}

.detail-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.detail-grid .detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--vt-c-divider-light-1);
}

.detail-grid .detail-item:last-child {
  border-bottom: none;
}

.detail-grid .label {
  color: var(--vt-c-text-light-2);
  font-weight: 500;
}

.star-section {
  display: flex;
  justify-content: center;
  padding: 16px 0;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .user-manage-mobile {
    padding: 12px;
  }
  
  .stats-cards {
    gap: 8px;
  }
  
  .stat-card {
    padding: 12px;
  }
  
  .stat-number {
    font-size: 20px;
  }
  
  .user-card {
    padding: 12px;
  }
  
  .username {
    font-size: 14px;
  }
  
  .action-buttons .el-button {
    min-width: 70px;
    font-size: 12px;
  }
  
  .filter-row {
    flex-direction: column;
    gap: 8px;
  }
  
  .filter-select {
    width: 100%;
  }
}

/* 移动端优化 */
@media (max-width: 768px) {
  .user-manage-mobile {
    padding: 8px;
  }
  
  .stats-cards {
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  
  .stat-card {
    padding: 12px 8px;
  }
  
  .stat-number {
    font-size: 18px;
  }
  
  .stat-label {
    font-size: 10px;
  }
  
  .search-section {
    padding: 12px;
  }
  
  .user-card {
    margin-bottom: 8px;
  }
  
  .user-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .user-status {
    flex-direction: row;
    align-items: center;
    gap: 4px;
  }
  
  .detail-item {
    font-size: 12px;
  }
  
  .action-buttons {
    flex-wrap: wrap;
    gap: 4px;
  }
  
  .action-buttons .el-button {
    flex: 1;
    min-width: 60px;
    font-size: 11px;
    padding: 4px 8px;
  }
}



/* Element Plus 主题切换支持 */
:deep(.dark) .user-manage-mobile {
  background-color: var(--vt-c-black) !important;
  color: var(--vt-c-text-dark-1) !important;
}

:deep(.dark) .stat-card,
:deep(.dark) .search-section,
:deep(.dark) .user-card,
:deep(.dark) .pagination {
  background: #1a1a1a !important;
  color: var(--vt-c-text-dark-1) !important;
  border: 1px solid #333333 !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5) !important;
}

:deep(.dark) .stat-card:hover,
:deep(.dark) .user-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.7) !important;
  background: #222222 !important;
}

:deep(.dark) .user-card.banned {
  background: #0f0f0f !important;
  color: var(--vt-c-text-dark-1) !important;
  border: 1px solid #444444 !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.6) !important;
  opacity: 0.95 !important;
}

:deep(.dark) .user-card.banned .username {
  color: var(--el-color-danger) !important;
}

:deep(.dark) .user-card.banned .nickname {
  color: var(--el-color-danger-light-3) !important;
}

:deep(.dark) .username {
  color: var(--vt-c-text-light-1) !important;
  font-weight: 700;
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

:deep(.dark) .nickname {
  color: var(--vt-c-text-light-2) !important;
  font-weight: 500;
  opacity: 0.95;
}

:deep(.dark) .detail-item {
  color: var(--vt-c-text-light-1) !important;
  font-weight: 500;
}

:deep(.dark) .detail-item .label {
  color: var(--vt-c-text-light-2) !important;
  font-weight: 600;
  opacity: 1;
}

:deep(.dark) .stat-number {
  color: var(--vt-c-text-light-1) !important;
  font-weight: 700;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
  font-size: 26px;
}

:deep(.dark) .stat-label {
  color: var(--vt-c-text-light-2) !important;
  font-weight: 600;
  opacity: 1;
}

:deep(.dark) .detail-section h4 {
  color: var(--vt-c-text-light-1) !important;
  font-weight: 700;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
  font-size: 18px;
}

:deep(.dark) .detail-grid .detail-item {
  border-bottom-color: var(--vt-c-divider-dark-1);
}

/* 浅色模式优化 */
:deep(.light) .user-manage-mobile {
  background-color: var(--vt-c-white-soft) !important;
  color: var(--vt-c-text-light-1) !important;
}

:deep(.light) .stat-card,
:deep(.light) .search-section,
:deep(.light) .user-card,
:deep(.light) .pagination {
  background: var(--vt-c-white) !important;
  color: var(--vt-c-text-light-1) !important;
  border: 1px solid var(--vt-c-divider-light-1) !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1) !important;
}

:deep(.light) .user-card.banned {
  background: var(--vt-c-white-mute) !important;
  color: var(--vt-c-text-light-1) !important;
  border: 1px solid var(--vt-c-divider-light-2) !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1) !important;
  opacity: 0.9 !important;
}

:deep(.light) .user-card.banned .username {
  color: var(--el-color-danger) !important;
}

:deep(.light) .user-card.banned .nickname {
  color: var(--el-color-danger-light-3) !important;
}

:deep(.light) .username {
  color: var(--vt-c-text-light-1) !important;
  font-weight: 600;
}

:deep(.light) .nickname {
  color: var(--vt-c-text-light-2) !important;
  opacity: 0.9;
}

:deep(.light) .detail-item .label {
  color: var(--vt-c-text-light-2) !important;
  font-weight: 500;
}

:deep(.light) .stat-number {
  color: var(--vt-c-text-light-1) !important;
  font-weight: 600;
}

:deep(.light) .stat-label {
  color: var(--vt-c-text-light-2) !important;
  font-weight: 500;
}

:deep(.light) .detail-section h4 {
  color: var(--vt-c-text-light-1) !important;
  font-weight: 600;
}

:deep(.light) .detail-grid .detail-item {
  border-bottom-color: var(--vt-c-divider-light-1);
}

/* 主题适配自动化修复 */
:deep(.dark) .user-manage-mobile {
  background-color: var(--vt-c-black) !important;
  color: var(--vt-c-text-dark-1) !important;
}
:deep(.dark) .stats-cards,
:deep(.dark) .stat-card,
:deep(.dark) .search-section,
:deep(.dark) .user-cards,
:deep(.dark) .user-card,
:deep(.dark) .pagination {
  background: var(--vt-c-black-soft) !important;
  color: var(--vt-c-text-dark-1) !important;
  border: 1px solid var(--vt-c-divider-dark-1) !important;
  box-shadow: 0 2px 8px rgba(0,0,0,0.3) !important;
}
:deep(.dark) .user-card.banned {
  background: var(--vt-c-black-mute) !important;
  color: var(--vt-c-text-dark-1) !important;
  border: 1px solid var(--vt-c-divider-dark-2) !important;
}
:deep(.dark) .user-card.banned .username {
  color: var(--el-color-danger) !important;
}
:deep(.dark) .user-card.banned .nickname {
  color: var(--el-color-danger-light-3) !important;
}
:deep(.dark) .username {
  color: var(--vt-c-text-dark-1) !important;
}
:deep(.dark) .nickname {
  color: var(--vt-c-text-dark-2) !important;
}
:deep(.dark) .detail-item .label {
  color: var(--vt-c-text-dark-2) !important;
}
:deep(.dark) .stat-number {
  color: var(--vt-c-text-dark-1) !important;
}
:deep(.dark) .stat-label {
  color: var(--vt-c-text-dark-2) !important;
}
:deep(.dark) .detail-section h4 {
  color: var(--vt-c-text-dark-1) !important;
}
:deep(.dark) .detail-grid .detail-item {
  border-bottom-color: var(--vt-c-divider-dark-1) !important;
}

:deep(.light) .user-manage-mobile {
  background-color: var(--vt-c-white-soft) !important;
  color: var(--vt-c-text-light-1) !important;
}
:deep(.light) .stats-cards,
:deep(.light) .stat-card,
:deep(.light) .search-section,
:deep(.light) .user-cards,
:deep(.light) .user-card,
:deep(.light) .pagination {
  background: var(--vt-c-white) !important;
  color: var(--vt-c-text-light-1) !important;
  border: 1px solid var(--vt-c-divider-light-1) !important;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1) !important;
}
:deep(.light) .user-card.banned {
  background: var(--vt-c-white-mute) !important;
  color: var(--vt-c-text-light-1) !important;
  border: 1px solid var(--vt-c-divider-light-2) !important;
}
:deep(.light) .user-card.banned .username {
  color: var(--el-color-danger) !important;
}
:deep(.light) .user-card.banned .nickname {
  color: var(--el-color-danger-light-3) !important;
}
:deep(.light) .username {
  color: var(--vt-c-text-light-1) !important;
}
:deep(.light) .nickname {
  color: var(--vt-c-text-light-2) !important;
}
:deep(.light) .detail-item .label {
  color: var(--vt-c-text-light-2) !important;
}
:deep(.light) .stat-number {
  color: var(--vt-c-text-light-1) !important;
}
:deep(.light) .stat-label {
  color: var(--vt-c-text-light-2) !important;
}
:deep(.light) .detail-section h4 {
  color: var(--vt-c-text-light-1) !important;
}
:deep(.light) .detail-grid .detail-item {
  border-bottom-color: var(--vt-c-divider-light-1) !important;
}
</style> 