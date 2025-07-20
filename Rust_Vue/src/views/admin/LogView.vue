<template>
  <div class="log-view-desktop">
    <!-- 背景装饰 -->
    <div class="background-decoration">
      <div class="decoration-circle circle-1"></div>
      <div class="decoration-circle circle-2"></div>
      <div class="decoration-circle circle-3"></div>
      <div class="decoration-circle circle-4"></div>
      <div class="floating-particles">
        <div class="particle" v-for="i in 10" :key="i" :style="{ animationDelay: `${i * 0.3}s` }"></div>
      </div>
    </div>

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
          <el-button @click="refresh" class="refresh-btn">
            <el-icon><Refresh /></el-icon>
            刷新
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-cards">
      <div class="stat-card total">
        <div class="stat-icon">
          <el-icon :size="24"><Document /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-number">{{ total }}</div>
          <div class="stat-label">总日志</div>
          <div class="stat-trend">
            <el-icon><TrendCharts /></el-icon>
            <span>实时</span>
          </div>
        </div>
      </div>
      
      <div class="stat-card info">
        <div class="stat-icon">
          <el-icon :size="24"><CircleCheck /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-number">{{ infoCount }}</div>
          <div class="stat-label">信息</div>
          <div class="stat-trend positive">
            <el-icon><TrendCharts /></el-icon>
            <span>正常</span>
          </div>
        </div>
      </div>
      
      <div class="stat-card warning">
        <div class="stat-icon">
          <el-icon :size="24"><Warning /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-number">{{ warnCount }}</div>
          <div class="stat-label">警告</div>
          <div class="stat-trend warning">
            <el-icon><TrendCharts /></el-icon>
            <span>关注</span>
          </div>
        </div>
      </div>
      
      <div class="stat-card error">
        <div class="stat-icon">
          <el-icon :size="24"><Close /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-number">{{ errorCount }}</div>
          <div class="stat-label">错误</div>
          <div class="stat-trend negative">
            <el-icon><TrendCharts /></el-icon>
            <span>处理</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和筛选区域 -->
    <div class="search-section">
      <div class="search-header">
        <h3 class="search-title">
          <el-icon><Search /></el-icon>
          日志筛选
        </h3>
      </div>
      
      <div class="search-filters">
        <div class="filter-group">
          <label class="filter-label">关键词搜索</label>
          <el-input 
            v-model="search" 
            placeholder="搜索日志内容..." 
            clearable 
            class="search-input" 
            @input="onSearch"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </div>
        
        <div class="filter-group">
          <label class="filter-label">日志级别</label>
          <el-select v-model="level" placeholder="全部级别" clearable class="level-select" @change="onSearch">
            <el-option label="全部" value="" />
            <el-option label="INFO" value="INFO" />
            <el-option label="WARN" value="WARN" />
            <el-option label="ERROR" value="ERROR" />
          </el-select>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="action-section">
      <el-button @click="exportLogs" type="success" class="export-btn">
        <el-icon><Download /></el-icon>
        导出日志
      </el-button>
      <el-button @click="clearLogs" type="danger" class="clear-btn">
        <el-icon><Delete /></el-icon>
        清空日志
      </el-button>
    </div>

    <!-- 日志列表 -->
    <div v-if="loading" class="loading-container">
      <el-skeleton :rows="3" animated />
    </div>
    
    <div v-else-if="logs.length === 0" class="empty-state">
      <div class="empty-icon">
        <el-icon :size="64"><Document /></el-icon>
      </div>
      <h3 class="empty-title">暂无日志数据</h3>
      <p class="empty-desc">系统运行日志将在这里显示</p>
    </div>
    
    <div v-else class="log-list">
      <div
        v-for="log in logs"
        :key="log.timestamp + log.message"
        class="log-card"
      >
        <div class="log-header">
          <div class="log-level">
            <el-tag 
              :type="getLevelTagType(log.level)" 
              size="small" 
              class="level-tag"
            >
              <el-icon v-if="log.level === 'ERROR'"><Close /></el-icon>
              <el-icon v-else-if="log.level === 'WARN'"><Warning /></el-icon>
              <el-icon v-else><CircleCheck /></el-icon>
              {{ log.level }}
            </el-tag>
          </div>
          <div class="log-time">
            <el-icon class="time-icon"><Clock /></el-icon>
            <span class="time-text">{{ log.timestamp }}</span>
          </div>
        </div>
        
        <div class="log-content">
          <p class="log-message">{{ log.message }}</p>
        </div>
      </div>
    </div>

    <!-- 分页 -->
    <div v-if="logs.length > 0" class="pagination-section">
      <el-pagination
        background
        layout="prev, pager, next"
        :total="total"
        :page-size="pageSize"
        :current-page="page"
        @current-change="loadLogs"
        :pager-count="5"
        :hide-on-single-page="true"
        class="pagination"
      />
    </div>
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
  TrendCharts
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

// 统计数据
const infoCount = ref(0)
const warnCount = ref(0)
const errorCount = ref(0)

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
  try {
    await ElMessageBox.confirm('确定要清空所有日志吗？此操作不可恢复！', '清空日志', { type: 'warning' })
    
    const params: any = {}
    if (level.value) params.level = level.value
    
    const response = await logsApi.clearLogs(params)
    
    if (response.code === 0) {
      ElMessage.success(`成功清除${response.data.deleted_count || 0}条日志`)
      loadLogs(1) // 刷新日志列表
    } else {
      ElMessage.error(response.message || '清空失败')
    }
  } catch (e) {
    // 用户取消操作或发生错误
    if (e !== 'cancel') {
      console.error('清空日志失败:', e)
      ElMessage.error('清空日志失败')
    }
  }
}

watch(page, (val) => loadLogs(val))
onMounted(() => loadLogs(1))
</script>

<style scoped>
.log-view-desktop {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
  min-height: 100vh;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  position: relative;
  overflow: hidden;
  animation: gradient-shift 15s ease-in-out infinite;
}

@keyframes gradient-shift {
  0%, 100% {
    background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  }
  50% {
    background: linear-gradient(135deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
  }
}

/* 背景装饰 */
.background-decoration {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: -1;
  pointer-events: none;
}

.decoration-circle {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  filter: blur(50px);
  opacity: 0.5;
  animation: pulse 10s infinite;
}

.circle-1 {
  width: 300px;
  height: 300px;
  top: -100px;
  left: -100px;
  background: var(--brand-color-light);
}

.circle-2 {
  width: 200px;
  height: 200px;
  bottom: 100px;
  right: 200px;
  background: var(--brand-color);
}

.circle-3 {
  width: 150px;
  height: 150px;
  bottom: 300px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--brand-color-dark);
}

.circle-4 {
  width: 250px;
  height: 250px;
  top: 400px;
  right: 100px;
  background: var(--brand-color-light);
}

.floating-particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  overflow: hidden;
}

.particle {
  position: absolute;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 50%;
  animation: float 10s infinite;
}

@keyframes pulse {
  0% {
    transform: scale(1);
    opacity: 0.5;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.3;
  }
  100% {
    transform: scale(1);
    opacity: 0.5;
  }
}

@keyframes float {
  0% {
    transform: translateY(0) translateX(0) rotate(0deg);
    opacity: 0;
  }
  25% {
    opacity: 0.5;
  }
  50% {
    transform: translateY(-20px) translateX(10px) rotate(5deg);
    opacity: 0.8;
  }
  75% {
    opacity: 0.5;
  }
  100% {
    transform: translateY(0) translateX(0) rotate(0deg);
    opacity: 0;
  }
}

/* 页面头部 */
.page-header {
  margin-bottom: 32px;
  background: var(--bg-card);
  border-radius: 20px;
  padding: 32px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  background-color: rgba(255, 255, 255, 0.1);
  animation: slide-up 0.8s ease-out;
}

@keyframes slide-up {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
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
  transition: transform 0.3s ease;
  position: relative;
  overflow: hidden;
}

.header-icon::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(45deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transform: translateX(-100%) translateY(-100%) rotate(45deg);
  transition: transform 0.6s ease;
}

.header-icon:hover::before {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

.header-icon:hover {
  transform: translateY(-5px);
  box-shadow: 0 12px 40px rgba(64, 158, 255, 0.4);
}

.header-icon:hover {
  transform: translateY(-5px);
  box-shadow: 0 12px 40px rgba(64, 158, 255, 0.4);
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  font-size: 32px;
  font-weight: 800;
  color: var(--text-primary);
  margin: 0;
  letter-spacing: -1px;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
}

.page-subtitle {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.header-actions {
  display: flex;
  gap: 12px;
}

.refresh-btn, .export-btn, .clear-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  border-radius: 12px;
  font-weight: 600;
  transition: all 0.3s ease;
  background: var(--brand-color);
  color: white;
  border: none;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
  position: relative;
  overflow: hidden;
}

.refresh-btn::before, .export-btn::before, .clear-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.refresh-btn:hover::before, .export-btn:hover::before, .clear-btn:hover::before {
  left: 100%;
}

.refresh-btn:hover, .export-btn:hover, .clear-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 18px rgba(64, 158, 255, 0.4);
  background: var(--brand-color-dark);
}

.export-btn:hover {
  background: var(--success-color);
}

.clear-btn {
  background: var(--danger-color);
}

.clear-btn:hover {
  background: var(--danger-color-dark);
}

/* 统计卡片 */
.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 24px;
  margin-bottom: 32px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 32px;
  display: flex;
  align-items: center;
  gap: 20px;
  border: 1px solid var(--border-color);
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  box-shadow: var(--shadow-light);
  backdrop-filter: blur(5px);
  -webkit-backdrop-filter: blur(5px);
  background-color: rgba(255, 255, 255, 0.1);
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
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
  background: linear-gradient(45deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transform: translateX(-100%) translateY(-100%) rotate(45deg);
  transition: transform 0.6s ease;
}

.stat-card:hover {
  transform: translateY(-8px);
  box-shadow: var(--shadow-dark);
  border-color: var(--brand-color);
}

.stat-card:hover::before {
  transform: scaleX(1);
}

.stat-card:hover::after {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

.stat-icon {
  color: var(--brand-color);
  flex-shrink: 0;
  font-size: 20px;
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 16px;
  transition: all 0.3s ease;
}

.stat-card:hover .stat-icon {
  background: var(--brand-color);
  color: #ffffff;
  transform: scale(1.1) rotate(5deg);
}

.stat-content {
  flex: 1;
}

.stat-number {
  font-size: 36px;
  font-weight: 800;
  color: var(--text-primary);
  line-height: 1;
  margin-bottom: 8px;
  letter-spacing: -2px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.stat-label {
  font-size: 16px;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 8px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.stat-trend {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
  font-weight: 600;
}

.stat-trend.positive {
  color: var(--success-color);
}

.stat-trend.warning {
  color: var(--warning-color);
}

.stat-trend.negative {
  color: var(--danger-color);
}

/* 搜索区域 */
.search-section {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 32px;
  margin-bottom: 32px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(5px);
  -webkit-backdrop-filter: blur(5px);
  background-color: rgba(255, 255, 255, 0.1);
  animation: slide-up 0.8s ease-out 0.2s both;
}

.search-header {
  margin-bottom: 24px;
}

.search-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 8px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.search-filters {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.search-input, .level-select {
  border-radius: 12px;
  border: 1px solid var(--border-color-lighter);
  background-color: var(--bg-input);
  color: var(--text-primary);
  transition: all 0.3s ease;
}

.search-input:focus, .level-select:focus {
  border-color: var(--brand-color);
  box-shadow: 0 0 0 2px var(--brand-color), 0 0 0 4px var(--brand-color-light);
}

/* 操作按钮 */
.action-section {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-bottom: 32px;
  animation: slide-up 0.8s ease-out 0.4s both;
}

.export-btn, .clear-btn {
  padding: 12px 20px;
  border-radius: 12px;
  font-weight: 600;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
}

.export-btn:hover {
  background: var(--success-color);
  box-shadow: 0 6px 18px rgba(64, 158, 255, 0.4);
}

.clear-btn:hover {
  background: var(--danger-color);
  box-shadow: 0 6px 18px rgba(255, 71, 71, 0.4);
}

/* 日志列表 */
.log-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: var(--bg-card);
  border-radius: 20px;
  padding: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(5px);
  -webkit-backdrop-filter: blur(5px);
  background-color: rgba(255, 255, 255, 0.1);
  animation: slide-up 0.8s ease-out 0.6s both;
}

.log-card {
  background: var(--bg-input);
  border-radius: 16px;
  padding: 16px;
  border: 1px solid var(--border-color-lighter);
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
  gap: 8px;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(5px);
  -webkit-backdrop-filter: blur(5px);
}

.log-card::before {
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

.log-card::after {
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

.log-card:hover {
  transform: translateY(-3px);
  box-shadow: var(--shadow-base);
  border-color: var(--brand-color-light);
}

.log-card:hover::before {
  transform: scaleX(1);
}

.log-card:hover::after {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-color-lighter);
}

.log-level {
  display: flex;
  align-items: center;
  gap: 8px;
}

.level-tag {
  display: flex;
  align-items: center;
  gap: 4px;
  font-weight: 500;
  border-radius: 8px;
  padding: 4px 10px;
  font-size: 13px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.log-time {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary);
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.time-icon {
  color: var(--text-tertiary);
  font-size: 14px;
}

.log-content {
  font-family: 'Courier New', monospace;
  font-size: 13px;
  color: var(--text-primary);
  line-height: 1.4;
  word-break: break-all;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.log-message {
  margin: 0;
  padding: 0;
}

/* 分页 */
.pagination-section {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(5px);
  -webkit-backdrop-filter: blur(5px);
  background-color: rgba(255, 255, 255, 0.1);
  animation: slide-up 0.8s ease-out 0.8s both;
}

.pagination {
  display: flex;
  justify-content: center;
}

.el-pagination .el-pager li {
  background-color: var(--bg-input);
  border: 1px solid var(--border-color-lighter);
  color: var(--text-primary);
  border-radius: 8px;
  margin: 0 4px;
  transition: all 0.3s ease;
}

.el-pagination .el-pager li:hover {
  background-color: var(--brand-color-light);
  border-color: var(--brand-color);
  color: var(--brand-color);
}

.el-pagination .el-pager li.is-active {
  background-color: var(--brand-color);
  color: white;
  border-color: var(--brand-color);
}

.el-pagination .el-pager li {
  background-color: var(--bg-input);
  border: 1px solid var(--border-color-lighter);
  color: var(--text-primary);
  border-radius: 8px;
  margin: 0 4px;
  transition: all 0.3s ease;
}

.el-pagination .el-pager li:hover {
  background-color: var(--brand-color-light);
  border-color: var(--brand-color);
  color: var(--brand-color);
}

.el-pagination .el-pager li.is-active {
  background-color: var(--brand-color);
  color: white;
  border-color: var(--brand-color);
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 60px 20px;
  background: var(--bg-card);
  border-radius: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(5px);
  -webkit-backdrop-filter: blur(5px);
  background-color: rgba(255, 255, 255, 0.1);
}

.empty-icon {
  color: var(--text-tertiary);
  margin-bottom: 16px;
}

.empty-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

.empty-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
}

/* 表格样式 */
.log-table {
  border-radius: 16px;
  overflow: hidden;
}

.timestamp-cell {
  display: flex;
  align-items: center;
  gap: 8px;
}

.time-icon {
  color: var(--text-tertiary);
  font-size: 14px;
}

.timestamp-text {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  color: var(--text-secondary);
}

.level-tag {
  display: flex;
  align-items: center;
  gap: 4px;
  font-weight: 500;
}

.message-cell {
  max-width: 600px;
}

.message-text {
  color: var(--text-primary);
  line-height: 1.4;
  font-family: 'Courier New', monospace;
  font-size: 13px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .log-view-desktop {
    padding: 16px;
  }
  
  .page-header {
    padding: 24px;
  }
  
  .header-content {
    flex-direction: column;
    gap: 20px;
    align-items: flex-start;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .stats-cards {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .search-section {
    padding: 24px;
  }
  
  .search-filters {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .action-section {
    flex-direction: column;
    gap: 12px;
  }
  
  .log-list {
    padding: 16px;
  }
  
  .log-header {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;
  }
}

@media (max-width: 480px) {
  .log-view-desktop {
    padding: 12px;
  }
  
  .page-header {
    padding: 20px;
  }
  
  .page-title {
    font-size: 20px;
  }
  
  .stats-cards {
    grid-template-columns: 1fr;
    gap: 12px;
  }
  
  .search-section {
    padding: 20px;
  }
  
  .action-section {
    gap: 8px;
  }
  
  .log-list {
    padding: 16px;
  }
  
  .log-card {
    padding: 12px;
  }
  
  .log-message {
    font-size: 13px;
  }
  
  .level-tag {
    font-size: 11px;
  }
  
  .time-text {
    font-size: 11px;
  }
}

/* 深色模式适配 */
.dark .stat-card {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
}

.dark .search-section,
.dark .log-list,
.dark .pagination-section {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
}

.dark .page-header {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
}

.dark .action-section {
  background: transparent;
}

/* 主题适配 */
.blue .stat-card::before,
.blue .log-card::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-card::before,
.green .log-card::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .stat-card::before,
.orange .log-card::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .stat-card::before,
.purple .log-card::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .stat-card,
  .refresh-btn,
  .export-btn,
  .clear-btn {
    transition: none;
  }
  
  .stat-card:hover,
  .refresh-btn:hover,
  .export-btn:hover,
  .clear-btn:hover {
    transform: none;
  }
}
</style>