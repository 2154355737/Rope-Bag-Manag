<template>
  <div class="user-actions">
    <el-card class="actions-card">
      <template #header>
        <div class="actions-header">
          <h2>用户行为记录</h2>
          <p>记录和分析用户在系统中的各种行为操作</p>
        </div>
      </template>

      <div class="actions-content">
        <!-- 搜索和过滤 -->
        <div class="search-section">
          <el-row :gutter="20">
            <el-col :span="4">
              <el-input
                v-model="searchQuery.user_id"
                placeholder="输入用户ID"
                clearable
                @input="handleSearch"
              >
                <template #prefix>
                  <el-icon><User /></el-icon>
                </template>
              </el-input>
            </el-col>
            <el-col :span="4">
              <el-select v-model="searchQuery.action_type" placeholder="行为类型" clearable @change="handleSearch">
                <el-option label="登录" value="Login" />
                <el-option label="登出" value="Logout" />
                <el-option label="注册" value="Register" />
                <el-option label="上传" value="Upload" />
                <el-option label="下载" value="Download" />
                <el-option label="评论" value="Comment" />
                <el-option label="点赞" value="Like" />
                <el-option label="分享" value="Share" />
                <el-option label="设置" value="Settings" />
                <el-option label="管理操作" value="Admin" />
              </el-select>
            </el-col>
            <el-col :span="4">
              <el-date-picker
                v-model="searchQuery.start_time"
                type="datetime"
                placeholder="开始时间"
                format="YYYY-MM-DD HH:mm:ss"
                value-format="YYYY-MM-DD HH:mm:ss"
                @change="handleSearch"
              />
            </el-col>
            <el-col :span="4">
              <el-date-picker
                v-model="searchQuery.end_time"
                type="datetime"
                placeholder="结束时间"
                format="YYYY-MM-DD HH:mm:ss"
                value-format="YYYY-MM-DD HH:mm:ss"
                @change="handleSearch"
              />
            </el-col>
            <el-col :span="8">
              <el-button type="primary" @click="refreshActions">刷新</el-button>
              <el-button type="success" @click="exportActions">导出</el-button>
              <el-button type="danger" @click="batchDelete" :disabled="selectedActions.length === 0">
                批量删除
              </el-button>
            </el-col>
          </el-row>
        </div>

        <!-- 统计信息 -->
        <div class="stats-section">
          <el-row :gutter="20">
            <el-col :span="12">
              <el-card class="stat-card">
                <div class="stat-content">
                  <div class="stat-icon">📊</div>
                  <div class="stat-info">
                    <div class="stat-value">{{ stats.total_actions }}</div>
                    <div class="stat-label">总行为数</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="12">
              <el-card class="stat-card">
                <div class="stat-content">
                  <div class="stat-icon">👥</div>
                  <div class="stat-info">
                    <div class="stat-value">{{ stats.active_users }}</div>
                    <div class="stat-label">活跃用户</div>
                  </div>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </div>

        <!-- 行为记录列表 -->
        <div class="actions-list">
          <el-table 
            :data="actionsList" 
            v-loading="loading"
            style="width: 100%"
            @selection-change="handleSelectionChange"
          >
            <el-table-column type="selection" width="55" />
            <el-table-column prop="id" label="ID" width="80" />
            <el-table-column prop="user_id" label="用户ID" width="120" />
            <el-table-column prop="action_type" label="行为类型" width="120">
              <template #default="{ row }">
                <el-tag :type="getActionTypeTag(row.action_type)">
                  {{ getActionTypeLabel(row.action_type) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="target_type" label="目标类型" width="120" />
            <el-table-column prop="target_id" label="目标ID" width="120" />
            <el-table-column prop="details" label="行为描述" min-width="200" />
            <el-table-column prop="created_at" label="时间" width="180">
              <template #default="{ row }">
                {{ formatTime(row.created_at) }}
              </template>
            </el-table-column>
            <el-table-column label="操作" width="120" fixed="right">
              <template #default="{ row }">
                <el-button size="small" @click="viewAction(row)">查看</el-button>
                <el-button 
                  size="small" 
                  type="danger" 
                  @click="deleteAction(row)"
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

    <!-- 行为详情对话框 -->
    <el-dialog 
      v-model="actionDialogVisible" 
      title="行为详情" 
      width="600px"
    >
      <div class="action-detail" v-if="currentAction">
        <div class="detail-item">
          <label>记录ID:</label>
          <span>{{ currentAction.id }}</span>
        </div>
        <div class="detail-item">
          <label>用户ID:</label>
          <span>{{ currentAction.user_id }}</span>
        </div>
        <div class="detail-item">
          <label>行为类型:</label>
          <el-tag :type="getActionTypeTag(currentAction.action_type)">
            {{ getActionTypeLabel(currentAction.action_type) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>目标类型:</label>
          <span>{{ currentAction.target_type }}</span>
        </div>
        <div class="detail-item">
          <label>目标ID:</label>
          <span>{{ currentAction.target_id }}</span>
        </div>
        <div class="detail-item">
          <label>行为描述:</label>
          <span>{{ currentAction.details }}</span>
        </div>
        <div class="detail-item">
          <label>IP地址:</label>
          <span>{{ currentAction.ip_address }}</span>
        </div>
        <div class="detail-item">
          <label>用户代理:</label>
          <span>{{ currentAction.user_agent }}</span>
        </div>
        <div class="detail-item">
          <label>操作结果:</label>
          <el-tag :type="currentAction.success ? 'success' : 'danger'">
            {{ currentAction.success ? '成功' : '失败' }}
          </el-tag>
        </div>
        <div class="detail-item" v-if="currentAction.error_message">
          <label>错误信息:</label>
          <span>{{ currentAction.error_message }}</span>
        </div>
        <div class="detail-item">
          <label>时间:</label>
          <span>{{ formatTime(currentAction.created_at) }}</span>
        </div>
      </div>
      <template #footer>
        <el-button @click="actionDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
// 导入所需依赖
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { User } from '@element-plus/icons-vue'
import { userActionApi, UserAction, UserActionStats } from '../../api'

// 响应式数据
const loading = ref(false)
const actionsList = ref<any[]>([])
const selectedActions = ref<any[]>([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const actionDialogVisible = ref(false)
const currentAction = ref<any>(null)

const searchQuery = reactive({
  user_id: '',
  action_type: '',
  start_time: '',
  end_time: ''
})

const stats = reactive({
  total_actions: 0,
  active_users: 0
})

// 方法
async function loadActions() {
  loading.value = true
  try {
    const params: Record<string, string> = {
      page: currentPage.value.toString(),
      page_size: pageSize.value.toString()
    }
    
    if (searchQuery.user_id) {
      params.user_id = searchQuery.user_id
    }
    if (searchQuery.action_type) {
      params.action_type = searchQuery.action_type
    }
    if (searchQuery.start_time) {
      params.start_time = searchQuery.start_time
    }
    if (searchQuery.end_time) {
      params.end_time = searchQuery.end_time
    }
    
    const response = await userActionApi.getUserActions(params)
    const data = response
    
    if (data.code === 0) {
      actionsList.value = data.data.actions || []
      total.value = data.data.total || 0
      updateStats()
    } else {
      ElMessage.error(data.message || '加载行为记录失败')
    }
  } catch (error) {
    console.error('加载行为记录失败:', error)
    ElMessage.error('加载行为记录失败')
  } finally {
    loading.value = false
  }
}

function updateStats() {
  const actions = actionsList.value
  stats.total_actions = actions.length
  
  // 计算活跃用户数（去重）
  const uniqueUsers = new Set(actions.map(a => a.user_id))
  stats.active_users = uniqueUsers.size
}

function handleSearch() {
  currentPage.value = 1
  loadActions()
}

function refreshActions() {
  loadActions()
}

function exportActions() {
  ElMessage.info('导出功能开发中...')
}

function handleSelectionChange(selection: any[]) {
  selectedActions.value = selection
}

function handleSizeChange(size: number) {
  pageSize.value = size
  loadActions()
}

function handleCurrentChange(page: number) {
  currentPage.value = page
  loadActions()
}

function viewAction(action: any) {
  currentAction.value = action
  actionDialogVisible.value = true
}

async function deleteAction(action: any) {
  try {
    await ElMessageBox.confirm(`确定要删除这条行为记录吗？`, '确认删除')
    
    const response = await userActionApi.deleteUserAction(action.id)
    
    if (response.code === 0) {
      ElMessage.success('行为记录删除成功')
      loadActions()
    } else {
      ElMessage.error(response.message || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

async function batchDelete() {
  if (selectedActions.value.length === 0) {
    ElMessage.warning('请选择要删除的行为记录')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedActions.value.length} 条行为记录吗？`, '确认删除')
    
    const actionIds = selectedActions.value.map(action => action.id)
    const response = await userActionApi.batchDeleteUserActions(actionIds)
    
    if (response.code === 0) {
      ElMessage.success('批量删除成功')
      selectedActions.value = []
      loadActions()
    } else {
      ElMessage.error(response.message || '批量删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('批量删除失败')
    }
  }
}

function getActionTypeTag(actionType: string): string {
  const tags = {
    Login: 'success',
    Logout: 'info',
    Register: 'warning',
    Upload: 'primary',
    Download: 'success',
    Comment: 'info',
    Like: 'warning',
    Share: 'primary',
    Settings: 'info',
    Admin: 'danger'
  }
  return tags[actionType] || 'info'
}

function getActionTypeLabel(actionType: string): string {
  const labels = {
    Login: '登录',
    Logout: '登出',
    Register: '注册',
    Upload: '上传',
    Download: '下载',
    Comment: '评论',
    Like: '点赞',
    Share: '分享',
    Settings: '设置',
    Admin: '管理操作',
    PageView: '页面访问',
    View: '查看',
    Navigation: '页面导航',
    Search: '搜索',
    EditPackage: '编辑资源',
    DeletePackage: '删除资源',
    CreatePackage: '创建资源',
    UpdatePackage: '更新资源',
    UpdateProfile: '更新资料',
    DeleteComment: '删除评论',
    SessionStart: '会话开始',
    SessionEnd: '会话结束',
    TabHidden: '离开页面',
    TabVisible: '回到页面',
    UserIdle: '用户闲置',
    UserActive: '用户活跃',
    BrowseCategory: '浏览分类'
  }
  return labels[actionType] || actionType
}

function formatTime(timestamp: string): string {
  if (!timestamp) return '-'
  try {
    return new Date(timestamp).toLocaleString()
  } catch (e) {
    return timestamp
  }
}

onMounted(() => {
  loadActions()
})
</script>

<style scoped>
.user-actions {
  padding: 20px;
}

.actions-card {
  max-width: 1400px;
  margin: 0 auto;
}

.actions-header {
  text-align: center;
}

.actions-header h2 {
  margin: 0 0 10px 0;
  color: var(--el-text-color-primary);
}

.actions-header p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.actions-content {
  padding: 20px 0;
}

.search-section {
  margin-bottom: 20px;
}

.stats-section {
  margin-bottom: 20px;
}

.stat-card {
  text-align: center;
}

.stat-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15px;
}

.stat-icon {
  font-size: 32px;
}

.stat-info {
  text-align: left;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: var(--el-text-color-primary);
}

.stat-label {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.actions-list {
  margin-bottom: 20px;
}

.pagination-wrapper {
  margin-top: 20px;
  text-align: center;
}

.action-detail {
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