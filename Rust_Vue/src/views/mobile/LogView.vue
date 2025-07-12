<template>
  <div class="log-view-mobile">
    <!-- 背景装饰 -->
    <div class="background-decoration">
      <div class="decoration-circle circle-1"></div>
      <div class="decoration-circle circle-2"></div>
      <div class="decoration-circle circle-3"></div>
      <div class="decoration-circle circle-4"></div>
      <div class="floating-particles">
        <div class="particle" v-for="i in 8" :key="i" :style="{ animationDelay: `${i * 0.3}s` }"></div>
      </div>
    </div>

    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="28"><Document /></el-icon>
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
          <el-icon :size="20"><Document /></el-icon>
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
          <el-icon :size="20"><CircleCheck /></el-icon>
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
          <el-icon :size="20"><Warning /></el-icon>
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
          <el-icon :size="20"><Close /></el-icon>
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
        :pager-count="3"
        :hide-on-single-page="true"
        class="pagination"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
// 定义组件名称用于缓存
defineOptions({
  name: 'LogViewMobile'
})

import { ref, onMounted, watch, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Document, 
  Search, 
  Refresh, 
  Download, 
  Delete, 
  CircleCheck, 
  Warning, 
  Close, 
  Clock,
  DataAnalysis
} from '@element-plus/icons-vue'
import { getLogEntries } from '../../api'
import axios from 'axios'
import { useRouter } from 'vue-router'

const logs = ref<any[]>([])
const page = ref(1)
const pageSize = 20
const total = ref(0)
const search = ref('')
const level = ref('')
const loading = ref(false)
const router = useRouter()

// 计算不同级别的日志数量
const infoCount = computed(() => logs.value.filter(log => log.level === 'INFO').length)
const warnCount = computed(() => logs.value.filter(log => log.level === 'WARN').length)
const errorCount = computed(() => logs.value.filter(log => log.level === 'ERROR').length)

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
      ElMessage.success('日志导出成功')
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
.log-view-mobile {
  padding: 16px;
  min-height: 100vh;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

/* 背景装饰 */
.background-decoration {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: -1; /* 确保装饰在背景后面 */
  overflow: hidden;
}

.decoration-circle {
  position: absolute;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 50%;
  filter: blur(50px);
  opacity: 0.5;
  animation: float 10s infinite ease-in-out;
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
  right: 100px;
  background: var(--brand-color);
}

.circle-3 {
  width: 150px;
  height: 150px;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: var(--brand-color-dark);
}

.circle-4 {
  width: 100px;
  height: 100px;
  bottom: 200px;
  left: 200px;
  background: var(--brand-color-light);
}

.floating-particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.particle {
  position: absolute;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  animation: float 8s infinite ease-in-out;
}

@keyframes float {
  0% {
    transform: translateY(0) translateX(0) scale(1);
    opacity: 0.5;
  }
  25% {
    transform: translateY(-10px) translateX(10px) scale(1.1);
    opacity: 0.7;
  }
  50% {
    transform: translateY(0) translateX(0) scale(1);
    opacity: 0.5;
  }
  75% {
    transform: translateY(10px) translateX(-10px) scale(1.1);
    opacity: 0.7;
  }
  100% {
    transform: translateY(0) translateX(0) scale(1);
    opacity: 0.5;
  }
}

/* 页面头部 */
.page-header {
  margin-bottom: 24px;
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  animation: slide-up 0.8s ease-out;
}

@keyframes slide-up {
  from {
    opacity: 0;
    transform: translateY(20px);
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
  position: relative;
  overflow: hidden;
  transition: all 0.3s ease;
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
  transform: translateY(-2px);
  box-shadow: 0 8px 32px rgba(64, 158, 255, 0.4);
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
  letter-spacing: -0.5px;
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.refresh-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border-radius: 8px;
  font-weight: 500;
  font-size: 12px;
  transition: all 0.3s ease;
  background: var(--brand-color);
  color: white;
  border: none;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
  position: relative;
  overflow: hidden;
}

.refresh-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.refresh-btn:hover::before {
  left: 100%;
}

.refresh-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 18px rgba(64, 158, 255, 0.4);
  background: var(--brand-color-dark);
}

/* 统计卡片 */
.stats-cards {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-bottom: 24px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 12px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  box-shadow: var(--shadow-light);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.stat-card::before {
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
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
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
  font-size: 16px;
  padding: 8px;
  background: var(--bg-secondary);
  border-radius: 8px;
  transition: all 0.3s ease;
}

.stat-card:hover .stat-icon {
  background: var(--brand-color);
  color: #ffffff;
  transform: scale(1.1);
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
  margin-bottom: 2px;
}

.stat-label {
  font-size: 10px;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.stat-trend {
  display: flex;
  align-items: center;
  gap: 2px;
  font-size: 10px;
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
  border-radius: 16px;
  padding: 20px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  animation: slide-up 0.8s ease-out 0.2s both;
}

.search-header {
  margin-bottom: 16px;
}

.search-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 6px;
}

.search-filters {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.filter-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
}

.search-input, .level-select {
  border-radius: 8px;
  border: 1px solid var(--border-color-lighter);
  background-color: var(--bg-input);
  color: var(--text-primary);
  transition: all 0.3s ease;
}

.search-input:focus, .level-select:focus {
  border-color: var(--brand-color);
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

/* 操作区域 */
.action-section {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
  animation: slide-up 0.8s ease-out 0.4s both;
}

.export-btn, .clear-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 12px 16px;
  border-radius: 10px;
  font-weight: 600;
  font-size: 14px;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.export-btn {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
  border: none;
  color: white;
  box-shadow: 0 4px 12px rgba(103, 194, 58, 0.3);
}

.clear-btn {
  background: linear-gradient(135deg, var(--danger-color) 0%, var(--danger-color-light) 100%);
  border: none;
  color: white;
  box-shadow: 0 4px 12px rgba(245, 108, 108, 0.3);
}

.export-btn::before, .clear-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.export-btn:hover::before, .clear-btn:hover::before {
  left: 100%;
}

.export-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 18px rgba(103, 194, 58, 0.4);
  background: var(--success-color-dark);
}

.clear-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 18px rgba(245, 108, 108, 0.4);
  background: var(--danger-color-dark);
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 40px 20px;
}

.empty-icon {
  color: var(--text-tertiary);
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

/* 日志列表 */
.log-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 24px;
  animation: slide-up 0.8s ease-out 0.6s both;
}

.log-card {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  box-shadow: var(--shadow-light);
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
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
  border-color: var(--brand-color);
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
  margin-bottom: 12px;
}

.log-level {
  display: flex;
  align-items: center;
}

.level-tag {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  font-weight: 500;
}

.log-time {
  display: flex;
  align-items: center;
  gap: 4px;
}

.time-icon {
  color: var(--text-tertiary);
  font-size: 12px;
}

.time-text {
  font-family: 'Courier New', monospace;
  font-size: 10px;
  color: var(--text-secondary);
}

.log-content {
  margin-bottom: 8px;
}

.log-message {
  font-size: 12px;
  color: var(--text-primary);
  line-height: 1.4;
  font-family: 'Courier New', monospace;
  margin: 0;
  word-break: break-word;
}

/* 分页区域 */
.pagination-section {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
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

/* 深色模式适配 */
.dark .stat-card {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
}

.dark .search-section,
.dark .log-card,
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
  .log-card,
  .refresh-btn,
  .export-btn,
  .clear-btn {
    transition: none;
  }
  
  .stat-card:hover,
  .log-card:hover,
  .refresh-btn:hover,
  .export-btn:hover,
  .clear-btn:hover {
    transform: none;
  }
}

/* 移动端特殊优化 */
@media (max-width: 480px) {
  .log-view-mobile {
    padding: 12px;
  }
  
  .page-header {
    padding: 16px;
  }
  
  .header-content {
    flex-direction: column;
    gap: 12px;
    align-items: flex-start;
  }
  
  .page-title {
    font-size: 18px;
  }
  
  .stats-cards {
    grid-template-columns: 1fr;
    gap: 8px;
  }
  
  .search-section {
    padding: 16px;
  }
  
  .action-section {
    flex-direction: column;
    gap: 8px;
  }
  
  .log-card {
    padding: 12px;
  }
  
  .log-message {
    font-size: 11px;
  }
  
  .level-tag {
    font-size: 9px;
  }
  
  .time-text {
    font-size: 9px;
  }
}
</style>