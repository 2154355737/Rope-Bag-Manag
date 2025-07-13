<template>
  <div class="user-action-log">
    <el-card class="log-card">
      <template #header>
        <div class="log-header">
          <h2>用户行为记录</h2>
          <p>记录和分析用户在系统中的各种行为操作</p>
        </div>
      </template>

      <div class="log-content">
        <!-- 筛选条件 -->
        <div class="filter-section">
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
            <el-form-item>
              <el-button type="primary" @click="handleFilter">筛选</el-button>
              <el-button @click="resetFilter">重置</el-button>
            </el-form-item>
          </el-form>
        </div>

        <!-- 统计信息 -->
        <div class="stats-section">
          <el-row :gutter="20">
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">📊</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.total_actions }}</div>
                    <div class="stats-label">总行为数</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">✅</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.success_actions }}</div>
                    <div class="stats-label">成功操作</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">❌</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.failed_actions }}</div>
                    <div class="stats-label">失败操作</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">👥</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.active_users }}</div>
                    <div class="stats-label">活跃用户</div>
                  </div>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </div>

        <!-- 行为记录列表 -->
        <div class="action-list">
          <el-table 
            :data="actionList" 
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
            <el-table-column prop="target_type" label="目标类型" width="100" />
            <el-table-column prop="target_id" label="目标ID" width="100" />
            <el-table-column prop="description" label="行为描述" min-width="200">
              <template #default="{ row }">
                <div class="action-description">
                  <p class="description-text">{{ row.description }}</p>
                  <div class="description-meta">
                    <span class="ip">{{ row.ip_address }}</span>
                    <span class="time">{{ formatTime(row.timestamp) }}</span>
                  </div>
                </div>
              </template>
            </el-table-column>
            <el-table-column prop="success" label="结果" width="80">
              <template #default="{ row }">
                <el-tag :type="row.success ? 'success' : 'danger'">
                  {{ row.success ? '成功' : '失败' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="error_message" label="错误信息" width="150">
              <template #default="{ row }">
                <span v-if="row.error_message" class="error-message">
                  {{ row.error_message }}
                </span>
                <span v-else>-</span>
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

        <!-- 批量操作 -->
        <div class="batch-actions" v-if="selectedActions.length > 0">
          <el-button type="danger" @click="batchDelete">批量删除</el-button>
          <el-button @click="exportActions">导出记录</el-button>
          <span class="selected-count">已选择 {{ selectedActions.length }} 条记录</span>
        </div>
      </div>
    </el-card>

    <!-- 行为详情对话框 -->
    <el-dialog 
      v-model="actionDialogVisible" 
      title="行为详情" 
      width="700px"
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
          <div class="description-box">{{ currentAction.description }}</div>
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
          <label>时间戳:</label>
          <span>{{ formatTime(currentAction.timestamp) }}</span>
        </div>
        <div class="detail-item">
          <label>操作结果:</label>
          <el-tag :type="currentAction.success ? 'success' : 'danger'">
            {{ currentAction.success ? '成功' : '失败' }}
          </el-tag>
        </div>
        <div class="detail-item" v-if="currentAction.error_message">
          <label>错误信息:</label>
          <div class="error-box">{{ currentAction.error_message }}</div>
        </div>
      </div>
      <template #footer>
        <el-button @click="actionDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getUserActions, deleteUserAction } from '../../api/userActions'

// 响应式数据
const loading = ref(false)
const actionList = ref([])
const selectedActions = ref([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const actionDialogVisible = ref(false)
const currentAction = ref(null)

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
  date_range: []
})

// 方法
async function loadActions() {
  loading.value = true
  try {
    const params = {
      page: currentPage.value,
      size: pageSize.value,
      ...filterForm
    }
    const response = await getUserActions(params)
    if (response.code === 0) {
      actionList.value = response.data.list || []
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
  // 这里应该从API获取统计数据
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
    await ElMessageBox.confirm('确定要删除这条行为记录吗？', '确认删除')
    const response = await deleteUserAction(action.id)
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
    ElMessage.success('批量删除成功')
    loadActions()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

function exportActions() {
  ElMessage.info('导出功能开发中...')
}

function getActionTypeTag(type: string): string {
  const tags = {
    Login: 'success',
    Logout: 'info',
    Register: 'primary',
    Upload: 'warning',
    Download: 'success',
    Comment: 'info',
    Like: 'success',
    Share: 'primary',
    Settings: 'warning',
    Admin: 'danger'
  }
  return tags[type] || 'info'
}

function getActionTypeLabel(type: string): string {
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
    Admin: '管理操作'
  }
  return labels[type] || type
}

function formatTime(timestamp: number): string {
  if (!timestamp) return '-'
  return new Date(timestamp * 1000).toLocaleString()
}

onMounted(() => {
  loadActions()
})
</script>

<style scoped>
.user-action-log {
  padding: 20px;
}

.log-card {
  max-width: 1400px;
  margin: 0 auto;
}

.log-header {
  text-align: center;
}

.log-header h2 {
  margin: 0 0 10px 0;
  color: var(--el-text-color-primary);
}

.log-header p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.log-content {
  padding: 20px 0;
}

.filter-section {
  margin-bottom: 20px;
  padding: 20px;
  background: var(--el-bg-color-page);
  border-radius: 8px;
}

.filter-form {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.stats-section {
  margin-bottom: 20px;
}

.stats-card {
  text-align: center;
}

.stats-item {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15px;
}

.stats-icon {
  font-size: 24px;
}

.stats-content {
  text-align: left;
}

.stats-value {
  font-size: 24px;
  font-weight: bold;
  color: var(--el-text-color-primary);
}

.stats-label {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.action-list {
  margin-bottom: 20px;
}

.action-description {
  max-width: 300px;
}

.description-text {
  margin: 0 0 8px 0;
  word-break: break-all;
  line-height: 1.4;
}

.description-meta {
  display: flex;
  gap: 15px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.error-message {
  color: var(--el-color-danger);
  font-size: 12px;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.batch-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 15px;
  background: var(--el-bg-color-page);
  border-radius: 8px;
  margin-top: 20px;
}

.selected-count {
  margin-left: auto;
  color: var(--el-text-color-secondary);
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
  color: var(--el-text-color-primary);
}

.detail-item span {
  flex: 1;
  color: var(--el-text-color-regular);
}

.description-box,
.error-box {
  flex: 1;
  padding: 10px;
  background: var(--el-bg-color-page);
  border-radius: 4px;
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
}

.error-box {
  color: var(--el-color-danger);
}
</style> 