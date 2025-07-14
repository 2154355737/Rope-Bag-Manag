<template>
  <div class="user-manage">
    <el-card class="manage-card">
      <template #header>
        <div class="manage-header">
          <h2>用户管理</h2>
          <p>管理系统用户和社区用户，支持用户信息编辑、删除等操作</p>
        </div>
      </template>

      <div class="manage-content">
        <!-- 搜索和过滤 -->
        <div class="search-section">
          <el-row :gutter="20">
            <el-col :span="6">
              <el-input
                v-model="searchQuery"
                placeholder="搜索用户名或QQ号"
                clearable
                @input="handleSearch"
              >
                <template #prefix>
                  <el-icon><Search /></el-icon>
                </template>
              </el-input>
            </el-col>
            <el-col :span="4">
              <el-select v-model="roleFilter" placeholder="角色筛选" clearable @change="handleFilter">
                <el-option label="普通用户" value="Normal" />
                <el-option label="开发者" value="Developer" />
                <el-option label="元老" value="Elder" />
              </el-select>
            </el-col>
            <el-col :span="4">
              <el-select v-model="statusFilter" placeholder="状态筛选" clearable @change="handleFilter">
                <el-option label="正常" value="Normal" />
                <el-option label="封禁" value="Banned" />
              </el-select>
            </el-col>
            <el-col :span="10">
              <el-button type="primary" @click="showAddUserDialog = true">添加用户</el-button>
              <el-button type="primary" @click="refreshUsers">刷新</el-button>
              <el-button type="success" @click="exportUsers">导出用户</el-button>
              <el-button type="danger" @click="batchDelete" :disabled="selectedUsers.length === 0">
                批量删除
              </el-button>
            </el-col>
          </el-row>
        </div>

        <!-- 用户列表 -->
        <div class="user-list">
          <el-table 
            :data="userList" 
            v-loading="loading"
            style="width: 100%"
            @selection-change="handleSelectionChange"
          >
            <el-table-column type="selection" width="55" />
            <el-table-column prop="username" label="用户名" min-width="120" />
            <el-table-column prop="star" label="星级" width="80">
              <template #default="{ row }">
                <el-rate v-model="row.star" disabled show-score />
              </template>
            </el-table-column>
            <el-table-column prop="role" label="角色" width="100">
              <template #default="{ row }">
                <el-tag :type="getRoleTag(row.role)">
                  {{ getRoleLabel(row.role) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="ban_status" label="状态" width="100">
              <template #default="{ row }">
                <el-tag :type="getStatusTag(row.ban_status)">
                  {{ getStatusLabel(row.ban_status) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="qq_number" label="QQ号" width="120" />
            <el-table-column prop="register_time" label="注册时间" width="180">
              <template #default="{ row }">
                {{ formatTime(row.register_time) }}
              </template>
            </el-table-column>
            <el-table-column prop="last_login" label="最后登录" width="180">
              <template #default="{ row }">
                {{ formatTime(row.last_login) }}
              </template>
            </el-table-column>
            <el-table-column prop="login_count" label="登录次数" width="100" />
            <el-table-column prop="upload_count" label="上传数" width="80" />
            <el-table-column prop="download_count" label="下载数" width="80" />
            <el-table-column label="操作" width="200" fixed="right">
              <template #default="{ row }">
                <el-button size="small" @click="viewUser(row)">查看</el-button>
                <el-button size="small" type="primary" @click="editUser(row)">编辑</el-button>
                <el-button 
                  size="small" 
                  type="danger" 
                  @click="deleteUser(row)"
                  :disabled="row.is_admin"
                >
                  删除
                </el-button>
              </template>
            </el-table-column>
          </el-table>

          <!-- 分页 -->
          <div class="pagination-wrapper">
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
      </div>
    </el-card>

    <!-- 用户详情对话框 -->
    <el-dialog 
      v-model="userDialogVisible" 
      title="用户详情" 
      width="600px"
    >
      <div class="user-detail" v-if="currentUser">
        <div class="detail-item">
          <label>用户ID:</label>
          <span>{{ currentUser.id }}</span>
        </div>
        <div class="detail-item">
          <label>用户名:</label>
          <span>{{ currentUser.username }}</span>
        </div>
        <div class="detail-item">
          <label>星级:</label>
          <el-rate v-model="currentUser.star" disabled show-score />
        </div>
        <div class="detail-item">
          <label>角色:</label>
          <el-tag :type="getRoleTag(currentUser.role)">
            {{ getRoleLabel(currentUser.role) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>状态:</label>
          <el-tag :type="getStatusTag(currentUser.ban_status)">
            {{ getStatusLabel(currentUser.ban_status) }}
          </el-tag>
        </div>
        <div class="detail-item" v-if="currentUser.ban_reason">
          <label>封禁原因:</label>
          <span>{{ currentUser.ban_reason }}</span>
        </div>
        <div class="detail-item">
          <label>QQ号:</label>
          <span>{{ currentUser.qq_number || '-' }}</span>
        </div>
        <div class="detail-item">
          <label>注册时间:</label>
          <span>{{ formatTime(currentUser.register_time) }}</span>
        </div>
        <div class="detail-item">
          <label>最后登录:</label>
          <span>{{ formatTime(currentUser.last_login) }}</span>
        </div>
        <div class="detail-item">
          <label>登录次数:</label>
          <span>{{ currentUser.login_count }}</span>
        </div>
        <div class="detail-item">
          <label>上传数量:</label>
          <span>{{ currentUser.upload_count }}</span>
        </div>
        <div class="detail-item">
          <label>下载数量:</label>
          <span>{{ currentUser.download_count }}</span>
        </div>
        <div class="detail-item">
          <label>管理员:</label>
          <el-tag :type="currentUser.is_admin ? 'danger' : 'info'">
            {{ currentUser.is_admin ? '是' : '否' }}
          </el-tag>
        </div>
      </div>
      <template #footer>
        <el-button @click="userDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- 编辑用户对话框 -->
    <el-dialog 
      v-model="editDialogVisible" 
      title="编辑用户信息" 
      width="500px"
    >
      <el-form :model="editForm" label-width="100px" v-if="currentUser">
        <el-form-item label="用户名">
          <el-input v-model="editForm.username" />
        </el-form-item>
        <el-form-item label="星级">
          <el-rate v-model="editForm.star" />
        </el-form-item>
        <el-form-item label="角色">
          <el-select v-model="editForm.role">
            <el-option label="普通用户" value="Normal" />
            <el-option label="开发者" value="Developer" />
            <el-option label="元老" value="Elder" />
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="editForm.ban_status">
            <el-option label="正常" value="Normal" />
            <el-option label="封禁" value="Banned" />
          </el-select>
        </el-form-item>
        <el-form-item label="封禁原因" v-if="editForm.ban_status === 'Banned'">
          <el-input v-model="editForm.ban_reason" type="textarea" />
        </el-form-item>
        <el-form-item label="QQ号">
          <el-input v-model="editForm.qq_number" />
        </el-form-item>
        <el-form-item label="管理员">
          <el-switch v-model="editForm.is_admin" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveUserEdit">保存</el-button>
      </template>
    </el-dialog>

    <!-- 添加用户对话框 -->
    <el-dialog 
      v-model="showAddUserDialog" 
      title="添加用户" 
      width="500px"
    >
      <el-form :model="addUserForm" :rules="addUserRules" ref="addUserFormRef" label-width="100px">
        <el-form-item label="用户名" prop="username">
          <el-input v-model="addUserForm.username" placeholder="请输入用户名" />
        </el-form-item>
        <el-form-item label="密码" prop="password">
          <el-input v-model="addUserForm.password" type="password" placeholder="请输入密码" />
        </el-form-item>
        <el-form-item label="角色" prop="role">
          <el-select v-model="addUserForm.role" placeholder="请选择角色">
            <el-option label="普通用户" value="Normal" />
            <el-option label="开发者" value="Developer" />
            <el-option label="元老" value="Elder" />
          </el-select>
        </el-form-item>
        <el-form-item label="星级" prop="star">
          <el-rate v-model="addUserForm.star" />
        </el-form-item>
        <el-form-item label="QQ号" prop="qq_number">
          <el-input v-model="addUserForm.qq_number" placeholder="请输入QQ号" />
        </el-form-item>
        <el-form-item label="头像URL" prop="avatar_url">
          <el-input v-model="addUserForm.avatar_url" placeholder="请输入头像URL" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddUserDialog = false">取消</el-button>
        <el-button type="primary" @click="handleAddUser">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search } from '@element-plus/icons-vue'

// 响应式数据
const loading = ref(false)
const userList = ref<any[]>([])
const selectedUsers = ref<any[]>([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const searchQuery = ref('')
const roleFilter = ref('')
const statusFilter = ref('')
const userDialogVisible = ref(false)
const editDialogVisible = ref(false)
const showAddUserDialog = ref(false)
const currentUser = ref<any>(null)
const addUserFormRef = ref()

const editForm = reactive({
  username: '',
  star: 1,
  role: 'Normal',
  ban_status: 'Normal',
  ban_reason: '',
  qq_number: '',
  is_admin: false
})

// 添加用户表单
const addUserForm = reactive({
  username: '',
  password: '',
  role: 'Normal',
  star: 1,
  qq_number: '',
  avatar_url: ''
})

// 添加用户表单验证规则
const addUserRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 2, max: 20, message: '用户名长度在 2 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码不能少于6位', trigger: 'blur' }
  ],
  role: [
    { required: true, message: '请选择角色', trigger: 'change' }
  ]
}

// 方法
async function loadUsers() {
  loading.value = true
  try {
    const params: Record<string, string> = {
      page: currentPage.value.toString(),
      size: pageSize.value.toString()
    }
    
    if (searchQuery.value) {
      params.search = searchQuery.value
    }
    if (roleFilter.value) {
      params.role = roleFilter.value
    }
    if (statusFilter.value) {
      params.status = statusFilter.value
    }
    
    const response = await fetch(`http://127.0.0.1:15202/api/users?${new URLSearchParams(params)}`)
    const data = await response.json()
    
    if (data.code === 0) {
      userList.value = data.data.users || []
      total.value = data.data.total || 0
    } else {
      ElMessage.error(data.msg || '加载用户列表失败')
    }
  } catch (error) {
    console.error('加载用户列表失败:', error)
    ElMessage.error('加载用户列表失败')
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  currentPage.value = 1
  loadUsers()
}

function handleFilter() {
  currentPage.value = 1
  loadUsers()
}

function refreshUsers() {
  loadUsers()
}

function exportUsers() {
  ElMessage.info('导出功能开发中...')
}

function handleSelectionChange(selection: any[]) {
  selectedUsers.value = selection
}

function handleSizeChange(size: number) {
  pageSize.value = size
  loadUsers()
}

function handleCurrentChange(page: number) {
  currentPage.value = page
  loadUsers()
}

function viewUser(user: any) {
  currentUser.value = user
  userDialogVisible.value = true
}

function editUser(user: any) {
  currentUser.value = user
  editForm.username = user.username
  editForm.star = user.star
  editForm.role = user.role
  editForm.ban_status = user.ban_status
  editForm.ban_reason = user.ban_reason || ''
  editForm.qq_number = user.qq_number || ''
  editForm.is_admin = user.is_admin
  editDialogVisible.value = true
}

async function saveUserEdit() {
  try {
    const response = await fetch(`http://127.0.0.1:15202/api/users/${currentUser.value.username}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(editForm)
    })
    
    const data = await response.json()
    
    if (data.code === 0) {
      ElMessage.success('用户信息更新成功')
      editDialogVisible.value = false
      loadUsers()
    } else {
      ElMessage.error(data.msg || '更新失败')
    }
  } catch (error) {
    ElMessage.error('更新失败')
  }
}

async function deleteUser(user: any) {
  try {
    await ElMessageBox.confirm(`确定要删除用户 ${user.username} 吗？`, '确认删除')
    
    const response = await fetch(`http://127.0.0.1:15202/api/users/${user.username}`, {
      method: 'DELETE'
    })
    
    const data = await response.json()
    
    if (data.code === 0) {
      ElMessage.success('用户删除成功')
      loadUsers()
    } else {
      ElMessage.error(data.msg || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

async function batchDelete() {
  if (selectedUsers.value.length === 0) {
    ElMessage.warning('请选择要删除的用户')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedUsers.value.length} 个用户吗？`, '确认删除')
    
    const usernames = selectedUsers.value.map(user => user.username)
    const response = await fetch('http://127.0.0.1:15202/api/users/batch', {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ usernames })
    })
    
    const data = await response.json()
    
    if (data.code === 0) {
      ElMessage.success('批量删除成功')
      selectedUsers.value = []
      loadUsers()
    } else {
      ElMessage.error(data.msg || '批量删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('批量删除失败')
    }
  }
}

// 添加用户
async function handleAddUser() {
  if (!addUserFormRef.value) {
    ElMessage.error('表单引用不存在')
    return
  }

  addUserFormRef.value.validate((valid: boolean) => {
    if (valid) {
      addUser()
    } else {
      ElMessage.error('请检查输入信息')
    }
  })
}

async function addUser() {
  try {
    // 获取认证信息
    const token = localStorage.getItem('token') || ''
    const username = localStorage.getItem('username') || ''
    
    const response = await fetch('http://127.0.0.1:15202/api/users', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
        'X-Username': username
      },
      body: JSON.stringify(addUserForm)
    })
    
    const data = await response.json()
    
    if (data.code === 0) {
      ElMessage.success('用户添加成功')
      showAddUserDialog.value = false
      // 重置表单
      Object.assign(addUserForm, {
        username: '',
        password: '',
        role: 'Normal',
        star: 1,
        qq_number: '',
        avatar_url: ''
      })
      loadUsers() // 刷新用户列表
    } else {
      ElMessage.error(data.msg || '添加失败')
    }
  } catch (error) {
    console.error('添加用户失败:', error)
    ElMessage.error('添加用户失败')
  }
}

function getRoleTag(role: string): string {
  const tags = {
    Normal: 'info',
    Developer: 'warning',
    Elder: 'success'
  }
  return tags[role] || 'info'
}

function getRoleLabel(role: string): string {
  const labels = {
    Normal: '普通用户',
    Developer: '开发者',
    Elder: '元老'
  }
  return labels[role] || role
}

function getStatusTag(status: string): string {
  const tags = {
    Normal: 'success',
    Banned: 'danger'
  }
  return tags[status] || 'info'
}

function getStatusLabel(status: string): string {
  const labels = {
    Normal: '正常',
    Banned: '封禁'
  }
  return labels[status] || status
}

function formatTime(time: string): string {
  if (!time) return '-'
  return new Date(time).toLocaleString()
}

onMounted(() => {
  loadUsers()
})
</script>

<style scoped>
.user-manage {
  padding: 20px;
}

.manage-card {
  max-width: 1400px;
  margin: 0 auto;
}

.manage-header {
  text-align: center;
}

.manage-header h2 {
  margin: 0 0 10px 0;
  color: var(--el-text-color-primary);
}

.manage-header p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.manage-content {
  padding: 20px 0;
}

.search-section {
  margin-bottom: 20px;
}

.user-list {
  margin-bottom: 20px;
}

.pagination-wrapper {
  margin-top: 20px;
  text-align: center;
}

.user-detail {
  padding: 20px;
}

.detail-item {
  display: flex;
  margin-bottom: 15px;
  align-items: center;
}

.detail-item label {
  width: 100px;
  font-weight: bold;
  color: var(--el-text-color-primary);
}

.detail-item span {
  flex: 1;
  color: var(--el-text-color-regular);
}
</style> 