<template>
  <div class="log-view">
    <el-card>
      <h2>日志查看</h2>
      <div class="log-controls">
        <div class="search-section">
          <el-input v-model="search" placeholder="搜索日志内容..." clearable class="search-input" @input="onSearch" />
          <el-select v-model="level" placeholder="全部级别" clearable class="level-select" @change="onSearch">
            <el-option label="全部" value="" />
            <el-option label="INFO" value="INFO" />
            <el-option label="WARN" value="WARN" />
            <el-option label="ERROR" value="ERROR" />
          </el-select>
        </div>
        <div class="action-section">
          <el-button @click="refresh" size="small">刷新</el-button>
          <el-button @click="exportLogs" type="success" size="small">导出</el-button>
          <el-button @click="clearLogs" type="danger" size="small">清空</el-button>
        </div>
        <div class="pagination-section">
          <el-pagination
            background
            layout="prev, pager, next"
            :total="total"
            :page-size="pageSize"
            :current-page="page"
            @current-change="loadLogs"
            size="small"
          />
        </div>
      </div>
      <el-table :data="logs" style="width: 100%; margin-top: 12px;" size="small">
        <el-table-column prop="timestamp" label="时间" width="140" />
        <el-table-column prop="level" label="级别" width="70">
          <template #default="scope">
            <el-tag :type="scope.row.level === 'ERROR' ? 'danger' : (scope.row.level === 'WARN' ? 'warning' : 'success')" size="small">
              {{ scope.row.level }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="message" label="内容" show-overflow-tooltip />
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getLogEntries } from '../api'
import axios from 'axios'
import { useRouter } from 'vue-router'

const logs = ref<any[]>([])
const page = ref(1)
const pageSize = 20
const total = ref(0)
const search = ref('')
const level = ref('')
const router = useRouter()

function getUsername() {
  const userInfo = localStorage.getItem('userInfo')
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      return user.username || 'admin'
    } catch {
      return 'admin'
    }
  }
  return 'admin'
}

async function loadLogs(val = 1) {
  page.value = val
  try {
    const username = getUsername()
    const params: any = {
      page: page.value,
      page_size: pageSize,
      username
    }
    if (search.value) params.search = search.value
    if (level.value) params.level = level.value
    // 直接用axios，便于处理blob导出
    const res = await axios.get('/api/logs/entries', { params })
    if (res.data.code === 0 && res.data.data) {
      logs.value = res.data.data.entries || []
      total.value = res.data.data.total || 0
    }
  } catch (error) {
    console.error('加载日志失败:', error)
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
    ElMessage.error('日志加载失败，请稍后重试')
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
    const username = getUsername()
    const params: any = {
      page: 1,
      page_size: total.value || 1000,
      username
    }
    if (search.value) params.search = search.value
    if (level.value) params.level = level.value
    const res = await axios.get('/api/logs/entries', { params })
    if (res.data.code === 0 && res.data.data) {
      const logText = (res.data.data.entries || []).map((l: any) => `[${l.timestamp}] [${l.level}] ${l.message}`).join('\n')
      const blob = new Blob([logText], { type: 'text/plain' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = 'logs.txt'
      a.click()
      URL.revokeObjectURL(url)
    }
  } catch (e) {
    ElMessage.error('导出失败')
  }
}

async function clearLogs() {
  try {
    await ElMessageBox.confirm('确定要清空所有日志吗？此操作不可恢复！', '清空日志', { type: 'warning' })
    const username = getUsername()
    const res = await axios.get('/api/logs/clear', { params: { username } })
    if (res.data.code === 0) {
      ElMessage.success('日志已清空')
      loadLogs(1)
    } else {
      ElMessage.error(res.data.msg || '清空失败')
    }
  } catch (e) {
    // 用户取消
  }
}

watch(page, (val) => loadLogs(val))
onMounted(() => loadLogs(1))
</script>

<style scoped>
.log-view {
  padding: 16px;
}

.log-controls {
  margin-bottom: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.search-section {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 200px;
}

.level-select {
  width: 120px;
}

.action-section {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.pagination-section {
  display: flex;
  justify-content: center;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .log-view {
    padding: 8px;
  }
  
  .log-controls {
    gap: 8px;
  }
  
  .search-section {
    flex-direction: column;
    gap: 8px;
  }
  
  .search-input {
    width: 100%;
    min-width: unset;
  }
  
  .level-select {
    width: 100%;
  }
  
  .action-section {
    justify-content: center;
    gap: 6px;
  }
  
  .action-section .el-button {
    flex: 1;
    min-width: 80px;
  }
  
  .pagination-section {
    margin-top: 8px;
  }
  
  /* 表格移动端适配 */
  :deep(.el-table) {
    font-size: 12px;
  }
  
  :deep(.el-table th) {
    padding: 8px 4px;
  }
  
  :deep(.el-table td) {
    padding: 6px 4px;
  }
  
  :deep(.el-tag) {
    font-size: 10px;
    padding: 2px 4px;
  }
}

/* 超小屏幕适配 */
@media (max-width: 480px) {
  .log-view {
    padding: 4px;
  }
  
  .search-section {
    gap: 6px;
  }
  
  .action-section {
    gap: 4px;
  }
  
  .action-section .el-button {
    min-width: 70px;
    font-size: 12px;
    padding: 6px 8px;
  }
  
  :deep(.el-table) {
    font-size: 11px;
  }
  
  :deep(.el-table th) {
    padding: 6px 2px;
  }
  
  :deep(.el-table td) {
    padding: 4px 2px;
  }
}
</style> 