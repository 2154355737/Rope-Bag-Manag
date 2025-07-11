<template>
  <div class="user-manage">
    <el-card>
      <div class="header">
        <h2>用户管理</h2>
        <div class="stats">
          <el-tag type="success">总用户: {{ totalUsers }}</el-tag>
          <el-tag type="warning">正常用户: {{ normalUsers }}</el-tag>
          <el-tag type="danger">封禁用户: {{ bannedUsers }}</el-tag>
        </div>
      </div>

      <!-- 搜索和筛选区域 -->
      <div class="search-section">
        <el-row :gutter="20">
          <el-col :span="6">
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
          </el-col>
          <el-col :span="4">
            <el-select v-model="statusFilter" placeholder="状态筛选" clearable @change="handleSearch">
              <el-option label="全部" value="" />
              <el-option label="正常" value="normal" />
              <el-option label="封禁" value="banned" />
            </el-select>
          </el-col>
          <el-col :span="4">
            <el-select v-model="starFilter" placeholder="星级筛选" clearable @change="handleSearch">
              <el-option label="全部" value="" />
              <el-option label="1星" value="1" />
              <el-option label="2星" value="2" />
              <el-option label="3星" value="3" />
              <el-option label="4星" value="4" />
              <el-option label="5星" value="5" />
            </el-select>
          </el-col>
          <el-col :span="4">
            <el-button type="primary" @click="handleSearch">搜索</el-button>
            <el-button @click="resetSearch">重置</el-button>
          </el-col>
          <el-col :span="6">
            <el-button type="success" @click="exportUsers">导出用户</el-button>
            <el-button type="warning" @click="batchBan" :disabled="!selectedUsers.length">批量封禁</el-button>
          </el-col>
        </el-row>
      </div>

      <!-- 用户表格 -->
      <el-table 
        :data="paginatedUsers" 
        style="width: 100%; margin-top: 18px;"
        @selection-change="handleSelectionChange"
        v-loading="loading"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="username" label="用户名" width="140" />
        <el-table-column prop="nickname" label="昵称" width="140" />
        <el-table-column prop="star" label="星级" width="120">
          <template #default="scope">
            <el-rate v-model="scope.row.star" disabled show-score />
          </template>
        </el-table-column>
        <el-table-column prop="banned" label="状态" width="100">
          <template #default="scope">
            <el-tag :type="scope.row.banned ? 'danger' : 'success'">
              {{ scope.row.banned ? '封禁' : '正常' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="sign_days" label="签到天数" width="100" />
        <el-table-column prop="sign_total" label="总签到" width="100" />
        <el-table-column prop="last_sign" label="最后签到" width="120" />
        <el-table-column prop="is_admin" label="管理员" width="80">
          <template #default="scope">
            <el-tag v-if="scope.row.is_admin" type="warning">是</el-tag>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="400" fixed="right">
          <template #default="scope">
            <el-button size="small" type="warning" @click="toggleBan(scope.row)">
              {{ scope.row.banned ? '解封' : '封禁' }}
            </el-button>
            <el-button size="small" @click="changeStar(scope.row, 1)">加星</el-button>
            <el-button size="small" @click="changeStar(scope.row, -1)">减星</el-button>
            <el-button size="small" @click="openEditNickname(scope.row)">改昵称</el-button>
            <el-button size="small" @click="openEditPassword(scope.row)">改密码</el-button>
            <el-button size="small" type="info" @click="viewUserDetail(scope.row)">详情</el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="filteredUsers.length"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </el-card>

    <!-- 修改昵称弹窗 -->
    <el-dialog v-model="showNicknameDialog" title="修改昵称" width="400px">
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
    <el-dialog v-model="showPasswordDialog" title="修改密码" width="400px">
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
    <el-dialog v-model="showDetailDialog" title="用户详情" width="600px">
      <div v-if="selectedUser" class="user-detail">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="用户名">{{ selectedUser.username }}</el-descriptions-item>
          <el-descriptions-item label="昵称">{{ selectedUser.nickname }}</el-descriptions-item>
          <el-descriptions-item label="星级">
            <el-rate v-model="selectedUser.star" disabled show-score />
          </el-descriptions-item>
          <el-descriptions-item label="状态">
            <el-tag :type="selectedUser.banned ? 'danger' : 'success'">
              {{ selectedUser.banned ? '封禁' : '正常' }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="管理员">
            <el-tag v-if="selectedUser.is_admin" type="warning">是</el-tag>
            <span v-else>否</span>
          </el-descriptions-item>
          <el-descriptions-item label="签到天数">{{ selectedUser.sign_days }}</el-descriptions-item>
          <el-descriptions-item label="总签到次数">{{ selectedUser.sign_total }}</el-descriptions-item>
          <el-descriptions-item label="最后签到">{{ selectedUser.last_sign || '未签到' }}</el-descriptions-item>
        </el-descriptions>
      </div>
    </el-dialog>

    <!-- 批量操作确认弹窗 -->
    <el-dialog v-model="showBatchDialog" title="批量操作确认" width="400px">
      <p>确定要封禁选中的 {{ selectedUsers.length }} 个用户吗？</p>
      <template #footer>
        <el-button @click="showBatchDialog = false">取消</el-button>
        <el-button type="danger" @click="confirmBatchBan" :loading="submitting">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search } from '@element-plus/icons-vue'
import { getUsers, banUser, setStar, setNickname, setPassword } from '../api'
import { useRouter } from 'vue-router'

// 响应式数据
const users = ref<any[]>([])
const filteredUsers = ref<any[]>([])
const selectedUsers = ref<any[]>([])
const loading = ref(false)
const submitting = ref(false)

// 搜索和筛选
const searchKeyword = ref('')
const statusFilter = ref('')
const starFilter = ref('')

// 分页
const currentPage = ref(1)
const pageSize = ref(20)

// 弹窗控制
const showNicknameDialog = ref(false)
const showPasswordDialog = ref(false)
const showDetailDialog = ref(false)
const showBatchDialog = ref(false)

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

function handleSelectionChange(selection: any[]) {
  selectedUsers.value = selection
}

function handleSizeChange(size: number) {
  pageSize.value = size
  currentPage.value = 1
}

function handleCurrentChange(page: number) {
  currentPage.value = page
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

function batchBan() {
  if (selectedUsers.value.length === 0) {
    ElMessage.warning('请先选择要封禁的用户')
    return
  }
  showBatchDialog.value = true
}

async function confirmBatchBan() {
  try {
    submitting.value = true
    let successCount = 0
    let failCount = 0
    
    for (const user of selectedUsers.value) {
      try {
        const res = await banUser(user.username, true)
        if (res.code === 0) {
          successCount++
        } else {
          failCount++
        }
      } catch (error) {
        failCount++
      }
    }
    
    await loadUsers()
    ElMessage.success(`批量封禁完成：成功 ${successCount} 个，失败 ${failCount} 个`)
    showBatchDialog.value = false
  } catch (error) {
    console.error('批量封禁失败:', error)
    handleNetworkError(error)
  } finally {
    submitting.value = false
  }
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
.user-manage {
  padding: 32px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.stats {
  display: flex;
  gap: 10px;
}

.search-section {
  margin-bottom: 20px;
  padding: 20px;
  background-color: #f5f7fa;
  border-radius: 8px;
}

.pagination {
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

.user-detail {
  padding: 20px 0;
}

:deep(.el-table) {
  border-radius: 8px;
  overflow: hidden;
}

:deep(.el-card) {
  border-radius: 12px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}
</style> 