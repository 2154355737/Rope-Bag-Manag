<template>
  <div class="admin-page user-action-log">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><Monitor /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">用户行为记录</h1>
            <p class="page-subtitle">查看和管理用户的所有行为活动记录</p>
          </div>
        </div>
      <div class="header-actions">
          <el-button type="danger" @click="clearAllLogs">
            <el-icon><Delete /></el-icon>
            清空日志
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><DataLine /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.total_actions }}</div>
            <div class="stat-label">总行为数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Check /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.success_actions }}</div>
            <div class="stat-label">成功操作</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><WarningFilled /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.failed_actions }}</div>
            <div class="stat-label">失败操作</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><User /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.active_users }}</div>
            <div class="stat-label">活跃用户</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 筛选条件 -->
    <div class="search-section">
      <div class="search-left">
      <el-form :inline="true" :model="filterForm" class="filter-form">
        <el-form-item label="行为类型">
          <el-select v-model="filterForm.action_type" placeholder="选择行为类型" clearable>
            <el-option label="全部" value="" />
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
        </el-form-item>
        <el-form-item label="用户ID">
          <el-input v-model="filterForm.user_id" placeholder="输入用户ID" clearable />
        </el-form-item>
        <el-form-item label="操作结果">
          <el-select v-model="filterForm.success" placeholder="选择结果" clearable>
            <el-option label="全部" value="" />
            <el-option label="成功" :value="true" />
            <el-option label="失败" :value="false" />
          </el-select>
        </el-form-item>
        <el-form-item label="时间范围">
          <el-date-picker
            v-model="filterForm.date_range"
            type="datetimerange"
            range-separator="至"
            start-placeholder="开始时间"
            end-placeholder="结束时间"
            format="YYYY-MM-DD HH:mm:ss"
            value-format="YYYY-MM-DD HH:mm:ss"
          />
        </el-form-item>
      </el-form>
      </div>
      <div class="search-right">
        <el-button type="primary" @click="handleFilter">
          <el-icon><Search /></el-icon>
          筛选
        </el-button>
        <el-button @click="resetFilter">
          <el-icon><Refresh /></el-icon>
          重置
        </el-button>
      </div>
    </div>

    <!-- 行为记录列表 -->
    <div class="content-section log-list-container" v-loading="loading">
      <div class="log-list">
        <div 
          v-for="(action, index) in actionList" 
          :key="action.id" 
          class="log-item"
        >
          <div class="log-level">{{ getActionTypeLabel(action.action_type) }}</div>
          <div class="log-time">{{ formatTime(action.timestamp) }}</div>
          <div class="log-message">{{ action.description }}</div>
          <div v-if="action.error_message" class="log-error">{{ action.error_message }}</div>
        </div>
      </div>

      <!-- 没有数据时显示 -->
      <el-empty v-if="actionList.length === 0 && !loading" description="暂无记录"></el-empty>
    </div>

      <!-- 分页 -->
    <div class="pagination-section">
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

    <!-- 行为详情对话框 -->
    <el-dialog 
      v-model="detailDialogVisible"
      title="行为详情" 
      width="600px"
      :close-on-click-modal="false"
    >
      <div v-if="selectedAction" class="action-detail">
        <div class="detail-item">
          <label>ID:</label>
          <span>{{ selectedAction.id }}</span>
        </div>
        <div class="detail-item">
          <label>用户ID:</label>
          <span>{{ selectedAction.user_id }}</span>
        </div>
        <div class="detail-item">
          <label>行为类型:</label>
          <span>{{ getActionTypeLabel(selectedAction.action_type) }}</span>
        </div>
        <div class="detail-item">
          <label>时间:</label>
          <span>{{ formatTime(selectedAction.timestamp) }}</span>
        </div>
        <div class="detail-item">
          <label>IP地址:</label>
          <span>{{ selectedAction.ip_address || 'N/A' }}</span>
        </div>
        <div class="detail-item">
          <label>成功状态:</label>
          <span :style="{ color: selectedAction.success ? '#67C23A' : '#F56C6C' }">
            {{ selectedAction.success ? '成功' : '失败' }}
          </span>
        </div>
        <div class="detail-item">
          <label>描述:</label>
          <div class="description-box">{{ selectedAction.description }}</div>
        </div>
        <div class="detail-item" v-if="selectedAction.error_message">
          <label>错误信息:</label>
          <div class="error-box">{{ selectedAction.error_message }}</div>
        </div>
      </div>
    </el-dialog>

    <!-- 确认对话框 -->
    <el-dialog
      v-model="confirmDialogVisible"
      title="确认操作"
      width="400px"
    >
      <div class="confirm-content">
        <p>{{ confirmMessage }}</p>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="confirmDialogVisible = false">取消</el-button>
          <el-button type="danger" @click="confirmAction">确认</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { userActionApi, UserAction } from '../../api/userActions'
import { 
  Monitor, 
  Delete, 
  Search, 
  Refresh, 
  DataLine, 
  Check, 
  WarningFilled, 
  User 
} from '@element-plus/icons-vue'

// 扩展UserAction接口添加我们需要的字段
interface ExtendedUserAction extends UserAction {
  description: string;
  timestamp: number;
  success: boolean;
  error_message?: string;
}

// 响应式数据
const loading = ref(false)
const actionList = ref<ExtendedUserAction[]>([])
const selectedActions = ref<ExtendedUserAction[]>([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const actionDialogVisible = ref(false)
const currentAction = ref<ExtendedUserAction | null>(null)
// 新增变量
const detailDialogVisible = ref(false)
const selectedAction = ref<ExtendedUserAction | null>(null)
const confirmDialogVisible = ref(false)
const confirmMessage = ref('')
const confirmAction = ref(() => {})

const stats = reactive({
  total_actions: 0,
  success_actions: 0,
  failed_actions: 0,
  active_users: 0
})

const filterForm = reactive({
  action_type: '',
  user_id: '',
  success: '',
  date_range: [] as string[]
})

// 方法
async function loadActions() {
  loading.value = true
  try {
    const params: any = {
      page: currentPage.value,
      page_size: pageSize.value,
    }
    
    // 处理筛选条件
    if (filterForm.action_type) {
      params.action_type = filterForm.action_type
    }
    
    if (filterForm.user_id) {
      params.user_id = Number(filterForm.user_id)
    }
    
    if (filterForm.success !== '') {
      params.success = filterForm.success
    }
    
    if (filterForm.date_range && filterForm.date_range.length === 2) {
      params.start_time = filterForm.date_range[0]
      params.end_time = filterForm.date_range[1]
    }
    
    const response = await userActionApi.getUserActions(params)
    if (response.code === 0 && response.data) {
      // 转换API返回的数据到我们的扩展类型
      actionList.value = (response.data.actions || []).map(action => ({
        ...action,
        description: action.details || '',
        timestamp: new Date(action.created_at).getTime() / 1000,
        success: true, // 假设所有记录都是成功的，除非有明确的错误信息
        error_message: undefined
      }))
      total.value = response.data.total || 0
      // 更新统计信息
      updateStats()
    }
  } catch (error) {
    console.error('加载行为记录失败:', error)
    ElMessage.error('加载行为记录失败')
  } finally {
    loading.value = false
  }
}

function updateStats() {
  // 更新统计数据
  stats.total_actions = total.value
  stats.success_actions = actionList.value.filter(a => a.success).length
  stats.failed_actions = actionList.value.filter(a => !a.success).length
  stats.active_users = new Set(actionList.value.map(a => a.user_id)).size
}

function handleFilter() {
  currentPage.value = 1
  loadActions()
}

function resetFilter() {
  Object.assign(filterForm, {
    action_type: '',
    user_id: '',
    success: '',
    date_range: []
  })
  handleFilter()
}

function handleSelectionChange(selection: ExtendedUserAction[]) {
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

function viewAction(action: ExtendedUserAction) {
  // 更新到新变量
  selectedAction.value = action
  detailDialogVisible.value = true
}

async function deleteAction(action: ExtendedUserAction) {
  try {
    await ElMessageBox.confirm('确定要删除这条行为记录吗？', '确认删除')
    const response = await userActionApi.deleteUserAction(action.id)
    if (response.code === 0) {
      ElMessage.success('记录已删除')
      loadActions()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

async function batchDelete() {
  if (selectedActions.value.length === 0) {
    ElMessage.warning('请选择要删除的记录')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedActions.value.length} 条记录吗？`, '确认删除')
    // 批量删除逻辑
    const ids = selectedActions.value.map(action => action.id)
    const response = await userActionApi.batchDeleteUserActions(ids)
    if (response.code === 0) {
      ElMessage.success('批量删除成功')
      loadActions()
      selectedActions.value = []
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

// 清空所有日志
async function clearAllLogs() {
  confirmMessage.value = '确定要清空所有行为记录吗？此操作不可恢复！'
  confirmAction.value = async () => {
    try {
    // 使用批量删除API删除所有记录
    const params = {
      page: 1,
      page_size: 1000 // 获取尽可能多的记录ID
    }
    const response = await userActionApi.getUserActions(params)
    
    if (response.code === 0 && response.data && response.data.actions.length > 0) {
      const actionIds = response.data.actions.map(action => action.id)
      const deleteResponse = await userActionApi.batchDeleteUserActions(actionIds)
      
      if (deleteResponse.code === 0) {
          ElMessage.success('所有行为记录已清空')
          loadActions() // 重新加载数据
        } else {
          ElMessage.error('清空记录失败')
        }
      } else {
        ElMessage.info('没有记录可清空')
    }
  } catch (error) {
      console.error('清空记录失败:', error)
      ElMessage.error('清空记录失败')
    } finally {
      confirmDialogVisible.value = false
    }
  }
  confirmDialogVisible.value = true
}

function exportActions() {
  ElMessage.info('导出功能开发中...')
}

function getActionTypeTag(type: string): string {
  const tags: Record<string, string> = {
    Login: 'success',
    Logout: 'info',
    Register: 'primary',
    Upload: 'warning',
    Download: 'success',
    Comment: 'info',
    Like: 'success',
    Share: 'primary',
    Settings: 'warning',
    Admin: 'danger',
    System: 'info'
  }
  return tags[type] || 'info'
}

// 获取行为类型标签
function getActionTypeLabel(type: string): string {
  const typeMap: Record<string, string> = {
    'Login': '登录',
    'Logout': '登出',
    'Register': '注册',
    'Upload': '上传',
    'Download': '下载',
    'Comment': '评论',
    'Like': '点赞',
    'Share': '分享',
    'Settings': '设置',
    'Admin': '管理操作'
  }
  return typeMap[type] || type
}

// 格式化时间
function formatTime(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 加载初始数据
onMounted(() => {
  loadActions()
})
</script>

<style scoped>
/* 用户行为日志页面特定样式 */
.log-list-container {
  margin-bottom: 20px;
}

.log-list {
  display: flex;
  flex-direction: column;
  color: var(--text-primary);
  font-size: 15px;
  font-weight: 400;
}

.log-item {
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 5px;
  transition: all 0.3s ease;
}

.log-item:hover {
  background-color: var(--bg-secondary);
  transform: translateX(4px);
}

.log-level {
  font-weight: 500;
  margin-bottom: 5px;
  color: var(--brand-color);
}

.log-time {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.log-message {
  line-height: 1.5;
  word-break: break-word;
  margin-bottom: 5px;
}

.log-error {
  color: var(--danger-color);
  margin-top: 5px;
}

.action-detail {
  padding: 20px;
}

.detail-item {
  display: flex;
  margin-bottom: 15px;
  align-items: flex-start;
}

.detail-item label {
  width: 100px;
  font-weight: 600;
  color: var(--text-primary);
}

.detail-item span {
  flex: 1;
  color: var(--text-secondary);
}

.description-box,
.error-box {
  flex: 1;
  padding: 10px;
  background: var(--bg-secondary);
  border-radius: 4px;
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
}

.error-box {
  color: var(--danger-color);
}
</style> 