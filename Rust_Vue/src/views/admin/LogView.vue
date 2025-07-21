<template>
  <div class="admin-page log-view-desktop">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><Document /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">日志查看</h1>
            <p class="page-subtitle">查看和管理系统运行日志</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button @click="refresh">
            <el-icon><Refresh /></el-icon>
            刷新
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Document /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ total }}</div>
            <div class="stat-label">总日志</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><CircleCheck /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ infoCount }}</div>
            <div class="stat-label">信息</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Warning /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ warnCount }}</div>
            <div class="stat-label">警告</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><CircleClose /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ errorCount }}</div>
            <div class="stat-label">错误</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和筛选区域 -->
    <div class="search-section">
      <div class="search-left">
        <el-input 
          v-model="search" 
          placeholder="搜索日志内容..." 
          clearable 
          style="width: 250px"
          @input="onSearch"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        
        <el-select v-model="level" placeholder="日志等级" clearable style="width: 150px" @change="onSearch">
          <el-option label="全部" value="" />
          <el-option label="INFO" value="INFO" />
          <el-option label="WARN" value="WARN" />
          <el-option label="ERROR" value="ERROR" />
        </el-select>
        
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          range-separator="至"
          start-placeholder="开始日期"
          end-placeholder="结束日期"
          style="width: 350px"
          value-format="YYYY-MM-DD"
          @change="onSearch"
        />
      </div>
      
      <div class="search-right">
        <el-button @click="clearLogs" type="danger">
          <el-icon><Delete /></el-icon>
          清空日志
        </el-button>
        <el-button @click="exportLogs" type="success">
          <el-icon><Download /></el-icon>
          导出日志
        </el-button>
      </div>
    </div>

    <!-- 日志列表 -->
    <div class="table-section" v-loading="loading">
      <el-table
        :data="logs"
        stripe
        border
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
      >
        <el-table-column type="index" width="50" label="#" />
        <el-table-column prop="timestamp" label="时间" width="180" />
        <el-table-column prop="level" label="等级" width="100">
          <template #default="{ row }">
            <el-tag :type="getLevelTagType(row.level)" size="small">
              {{ row.level }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="source" label="来源" width="150" />
        <el-table-column prop="message" label="消息" min-width="400">
          <template #default="{ row }">
            <div class="message-cell">{{ row.message }}</div>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="viewLog(row)" type="primary">
              <el-icon><View /></el-icon>
              详情
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 空数据展示 -->
      <template v-if="!loading && logs.length === 0">
        <div class="empty-logs">
          <el-empty description="暂无日志数据"></el-empty>
        </div>
      </template>
    </div>

    <!-- 分页 -->
    <div class="pagination-section">
      <el-pagination
        :current-page="page"
        :page-size="pageSize"
        :page-sizes="[10, 20, 50, 100]"
        :total="total"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="loadLogs"
      />
    </div>

    <!-- 日志详情对话框 -->
    <el-dialog
      v-model="detailVisible"
      title="日志详情"
      width="700px"
      :close-on-click-modal="false"
    >
      <template v-if="selectedLog">
        <div class="log-detail">
          <div class="detail-item">
            <span class="detail-label">时间：</span>
            <span class="detail-value">{{ selectedLog.timestamp }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">等级：</span>
            <el-tag :type="getLevelTagType(selectedLog.level)">
              {{ selectedLog.level }}
            </el-tag>
          </div>
          <div class="detail-item">
            <span class="detail-label">来源：</span>
            <span class="detail-value">{{ selectedLog.source }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">消息：</span>
            <div class="detail-message">{{ selectedLog.message }}</div>
          </div>
          <div class="detail-item" v-if="selectedLog.stack">
            <span class="detail-label">堆栈：</span>
            <pre class="detail-stack">{{ selectedLog.stack }}</pre>
          </div>
          <div class="detail-item" v-if="selectedLog.meta">
            <span class="detail-label">元数据：</span>
            <pre class="detail-meta">{{ JSON.stringify(selectedLog.meta, null, 2) }}</pre>
          </div>
        </div>
      </template>
      <template #footer>
        <el-button @click="detailVisible = false">关闭</el-button>
        <el-button type="danger" @click="deleteLog(selectedLog)">删除</el-button>
      </template>
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
// 定义组件名称用于缓存
defineOptions({
  name: 'LogView'
})

import { ref, watch, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useRouter } from 'vue-router'
import { 
  Document, 
  Refresh, 
  Search, 
  Download, 
  Delete, 
  Clock,
  CircleCheck,
  Warning,
  Close,
  TrendCharts,
  View,
  CircleClose
} from '@element-plus/icons-vue'
import { adminApi, logsApi } from '../../api'

const router = useRouter()

// 响应式数据
const logs = ref<any[]>([])
const loading = ref(false)
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)
const search = ref('')
const level = ref('')
const dateRange = ref<any>(null)
const module = ref('')

// 详情对话框相关
const detailVisible = ref(false)
const selectedLog = ref<any>(null)

// 确认对话框相关
const confirmDialogVisible = ref(false)
const confirmMessage = ref('')
const confirmAction = ref(() => {})

// 统计数据
const infoCount = ref(0)
const warnCount = ref(0)
const errorCount = ref(0)

// 视图日志
function viewLog(log: any) {
  selectedLog.value = log
  detailVisible.value = true
}

// 删除日志
function deleteLog(log: any) {
  confirmMessage.value = `确定要删除这条日志吗？`
  confirmAction.value = async () => {
    try {
      // 实际删除逻辑
      const response = await logsApi.deleteLog(log.id)
      if (response.code === 0) {
        ElMessage.success('日志已删除')
        detailVisible.value = false
        loadLogs(page.value)
      } else {
        ElMessage.error(response.message || '删除失败')
      }
    } catch (e) {
      console.error('删除日志失败:', e)
      ElMessage.error('删除日志失败')
    } finally {
      confirmDialogVisible.value = false
    }
  }
  confirmDialogVisible.value = true
}

function handleSizeChange(size: number) {
  pageSize.value = size
  loadLogs(1)
}

function getUsername() {
  try {
    const userInfo = localStorage.getItem('userInfo')
    if (userInfo) {
      const user = JSON.parse(userInfo)
      return user.username || 'admin'
    }
  } catch {
    return 'admin'
  }
  return 'admin'
}

function getLevelTagType(level: string) {
  switch (level) {
    case 'ERROR':
      return 'danger'
    case 'WARN':
      return 'warning'
    case 'INFO':
    default:
      return 'success'
  }
}

async function loadLogs(val = 1) {
  page.value = val
  loading.value = true
  try {
    // 使用后端API获取日志
    const response = await adminApi.getLogs({
      page: page.value,
      pageSize: pageSize.value,
      level: level.value || undefined,
      search: search.value || undefined
    })
    
    if (response.code === 0 && response.data) {
      // 设置日志数据
      logs.value = response.data.list || []
      total.value = response.data.total || 0
    
    // 更新统计数据
      infoCount.value = logs.value.filter(log => log.level === 'INFO').length
      warnCount.value = logs.value.filter(log => log.level === 'WARN').length
      errorCount.value = logs.value.filter(log => log.level === 'ERROR').length
    } else {
      ElMessage.warning(response.message || '获取日志数据失败')
    }
  } catch (error) {
    console.error('加载日志失败:', error)
    ElMessage.error('日志加载失败，请稍后重试')
    logs.value = [] // 清空日志，避免显示错误数据
    total.value = 0
  } finally {
    loading.value = false
  }
}

function refresh() {
  loadLogs(page.value)
}

function onSearch() {
  loadLogs(1)
}

async function exportLogs() {
  try {
    const params: any = {}
    if (search.value) params.search = search.value
    if (level.value) params.level = level.value
    if (dateRange.value && dateRange.value[0] && dateRange.value[1]) {
      params.start_date = dateRange.value[0]
      params.end_date = dateRange.value[1]
    }
    
    const response = await logsApi.exportLogs(params)
    
    // 创建下载链接
    const blob = new Blob([response], { type: 'text/plain' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
    a.download = `system_logs_${new Date().toISOString().split('T')[0]}.txt`
      a.click()
      URL.revokeObjectURL(url)
      ElMessage.success('日志导出成功')
  } catch (e) {
    console.error('导出日志失败:', e)
    ElMessage.error('导出失败，请稍后重试')
  }
}

async function clearLogs() {
  confirmMessage.value = '确定要清空所有日志吗？此操作不可恢复！'
  confirmAction.value = async () => {
    try {
      const params: any = {}
      if (level.value) params.level = level.value
      
      const response = await logsApi.clearLogs(params)
      
      if (response.code === 0 && response.data) {
        ElMessage.success(`成功清除${response.data.deleted_count || 0}条日志`)
        loadLogs(1) // 刷新日志列表
      } else {
        ElMessage.error(response.message || '清空失败')
      }
    } catch (e) {
      console.error('清空日志失败:', e)
      ElMessage.error('清空日志失败')
    } finally {
      confirmDialogVisible.value = false
    }
  }
  confirmDialogVisible.value = true
}

watch(page, (val) => loadLogs(val))
onMounted(() => loadLogs(1))
</script>

<style scoped>
/* 日志查看页特定样式 */
.message-cell {
  word-break: break-word;
  white-space: pre-wrap;
  max-height: 150px;
  overflow-y: auto;
}

.log-detail {
  padding: 20px 0;
}

.detail-item {
  margin-bottom: 15px;
}

.detail-label {
  font-weight: 600;
  display: block;
  margin-bottom: 5px;
  color: var(--text-primary);
}

.detail-value {
  color: var(--text-secondary);
}

.detail-message,
.detail-stack,
.detail-meta {
  padding: 10px;
  background: var(--bg-secondary);
  border-radius: 4px;
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
  font-family: monospace;
  max-height: 300px;
  overflow-y: auto;
  color: var(--text-secondary);
}

.detail-stack {
  color: var(--warning-color);
}

.empty-logs {
  padding: 40px 0;
  text-align: center;
}

.confirm-content {
  padding: 20px 0;
}
</style>