<template>
  <div class="stats-mobile">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="24"><DataAnalysis /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">统计信息</h1>
            <p class="page-subtitle">系统运行数据概览</p>
          </div>
        </div>
        <div class="header-actions">
          <div class="time-display">
            <span class="current-time">{{ currentTime }}</span>
            <span class="current-date">{{ currentDate }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 主要统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card primary">
          <div class="stat-icon">
            <el-icon :size="20"><User /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalUsers }}</div>
            <div class="stat-label">总用户数</div>
            <div class="stat-trend positive">
              <el-icon><Top /></el-icon>
              <span>+12.5%</span>
            </div>
          </div>
        </div>

        <div class="stat-card success">
          <div class="stat-icon">
            <el-icon :size="20"><Box /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalPackages }}</div>
            <div class="stat-label">总绳包数</div>
            <div class="stat-trend positive">
              <el-icon><Top /></el-icon>
              <span>+8.3%</span>
            </div>
          </div>
        </div>

        <div class="stat-card warning">
          <div class="stat-icon">
            <el-icon :size="20"><Download /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalDownloads }}</div>
            <div class="stat-label">总下载量</div>
            <div class="stat-trend positive">
              <el-icon><Top /></el-icon>
              <span>+15.7%</span>
            </div>
          </div>
        </div>

        <div class="stat-card info">
          <div class="stat-icon">
            <el-icon :size="20"><View /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalViews }}</div>
            <div class="stat-label">总浏览量</div>
            <div class="stat-trend positive">
              <el-icon><Top /></el-icon>
              <span>+22.1%</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 图表区域 -->
    <div class="charts-section">
      <!-- 用户增长趋势 -->
      <div class="chart-card">
        <div class="chart-header">
          <h3 class="chart-title">用户增长趋势</h3>
          <div class="chart-actions">
            <el-button size="small" :type="timeRange === '7d' ? 'primary' : 'default'" @click="setTimeRange('7d')">7天</el-button>
            <el-button size="small" :type="timeRange === '30d' ? 'primary' : 'default'" @click="setTimeRange('30d')">30天</el-button>
            <el-button size="small" :type="timeRange === '90d' ? 'primary' : 'default'" @click="setTimeRange('90d')">90天</el-button>
          </div>
        </div>
        <div class="chart-container">
          <div class="chart-placeholder">
            <div class="chart-line"></div>
            <div class="chart-line"></div>
            <div class="chart-line"></div>
            <div class="chart-line"></div>
            <div class="chart-line"></div>
          </div>
          <div class="chart-data">
            <div class="data-point" style="left: 10%; top: 60%;"></div>
            <div class="data-point" style="left: 25%; top: 45%;"></div>
            <div class="data-point" style="left: 40%; top: 35%;"></div>
            <div class="data-point" style="left: 55%; top: 25%;"></div>
            <div class="data-point" style="left: 70%; top: 20%;"></div>
            <div class="data-point" style="left: 85%; top: 15%;"></div>
            <div class="data-point" style="left: 100%; top: 10%;"></div>
          </div>
        </div>
      </div>

      <!-- 下载量统计 -->
      <div class="chart-card">
        <div class="chart-header">
          <h3 class="chart-title">下载量统计</h3>
          <div class="chart-period">本月</div>
        </div>
        <div class="download-stats">
          <div class="download-item">
            <div class="download-bar" style="height: 60%;"></div>
            <div class="download-label">绳索</div>
            <div class="download-value">1,250</div>
          </div>
          <div class="download-item">
            <div class="download-bar" style="height: 80%;"></div>
            <div class="download-label">装备</div>
            <div class="download-value">890</div>
          </div>
          <div class="download-item">
            <div class="download-bar" style="height: 45%;"></div>
            <div class="download-label">工具</div>
            <div class="download-value">567</div>
          </div>
          <div class="download-item">
            <div class="download-bar" style="height: 70%;"></div>
            <div class="download-label">教程</div>
            <div class="download-value">1,023</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 实时活动 -->
    <div class="activity-section">
      <div class="section-header">
        <h3 class="section-title">实时活动</h3>
        <el-button size="small" type="primary" @click="refreshActivity">刷新</el-button>
      </div>
      
      <div class="activity-list">
        <div v-for="activity in recentActivities" :key="activity.id" class="activity-item">
          <div class="activity-icon" :class="activity.type">
            <el-icon :size="16">
              <component :is="getActivityIcon(activity.type)" />
            </el-icon>
          </div>
          <div class="activity-content">
            <div class="activity-text">{{ activity.text }}</div>
            <div class="activity-time">{{ formatTime(activity.time) }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 系统状态 -->
    <div class="system-status">
      <div class="section-header">
        <h3 class="section-title">系统状态</h3>
        <div class="status-indicator online">
          <div class="status-dot"></div>
          <span>在线</span>
        </div>
      </div>
      
      <div class="status-grid">
        <div class="status-item">
          <div class="status-label">CPU使用率</div>
          <div class="status-value">{{ systemStatus.cpu }}%</div>
          <div class="status-bar">
            <div class="status-progress" :style="{ width: systemStatus.cpu + '%' }"></div>
          </div>
        </div>
        
        <div class="status-item">
          <div class="status-label">内存使用率</div>
          <div class="status-value">{{ systemStatus.memory }}%</div>
          <div class="status-bar">
            <div class="status-progress" :style="{ width: systemStatus.memory + '%' }"></div>
          </div>
        </div>
        
        <div class="status-item">
          <div class="status-label">磁盘使用率</div>
          <div class="status-value">{{ systemStatus.disk }}%</div>
          <div class="status-bar">
            <div class="status-progress" :style="{ width: systemStatus.disk + '%' }"></div>
          </div>
        </div>
        
        <div class="status-item">
          <div class="status-label">网络状态</div>
          <div class="status-value">{{ systemStatus.network }}Mbps</div>
          <div class="status-bar">
            <div class="status-progress" :style="{ width: (systemStatus.network / 100) + '%' }"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { ElMessage } from 'element-plus'
import { 
  DataAnalysis,
  User, 
  Box, 
  Download, 
  View, 
  Top,
  Plus,
  Edit,
  Delete,
  Star
} from '@element-plus/icons-vue'

// 响应式数据
const currentTime = ref('')
const currentDate = ref('')
const timeRange = ref('7d')

// 统计数据
const totalUsers = ref(1256)
const totalPackages = ref(342)
const totalDownloads = ref(5678)
const totalViews = ref(12345)

// 系统状态
const systemStatus = ref({
  cpu: 45,
  memory: 62,
  disk: 78,
  network: 85
})

// 实时活动
const recentActivities = ref([
  {
    id: 1,
    type: 'download',
    text: '用户张三下载了基础绳索套装',
    time: new Date(Date.now() - 5 * 60 * 1000)
  },
  {
    id: 2,
    type: 'upload',
    text: '用户李四上传了新的攀岩装备',
    time: new Date(Date.now() - 15 * 60 * 1000)
  },
  {
    id: 3,
    type: 'register',
    text: '新用户王五注册了账号',
    time: new Date(Date.now() - 30 * 60 * 1000)
  },
  {
    id: 4,
    type: 'view',
    text: '用户赵六查看了救援绳索包',
    time: new Date(Date.now() - 45 * 60 * 1000)
  }
])

// 方法
function updateTime() {
  const now = new Date()
  currentTime.value = now.toLocaleTimeString('zh-CN', { 
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
  currentDate.value = now.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

function setTimeRange(range: string) {
  timeRange.value = range
  ElMessage.success(`已切换到${range === '7d' ? '7天' : range === '30d' ? '30天' : '90天'}数据`)
}

function refreshActivity() {
  ElMessage.success('活动数据已刷新')
}

function getActivityIcon(type: string) {
  const iconMap: Record<string, string> = {
    download: 'Download',
    upload: 'Plus',
    register: 'User',
    view: 'View',
    edit: 'Edit',
    delete: 'Delete',
    star: 'Star'
  }
  return iconMap[type] || 'Info'
}

function formatTime(time: Date) {
  const now = new Date()
  const diff = now.getTime() - time.getTime()
  const minutes = Math.floor(diff / (1000 * 60))
  const hours = Math.floor(diff / (1000 * 60 * 60))
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  return `${days}天前`
}

// 定时器
let timeInterval: NodeJS.Timeout

onMounted(() => {
  updateTime()
  timeInterval = setInterval(updateTime, 1000)
})

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval)
  }
})
</script>

<style scoped>
.stats-mobile {
  padding: 16px;
  min-height: 100vh;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

/* 页面头部 */
.page-header {
  margin-bottom: 24px;
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
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
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.time-display {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
}

.current-time {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.current-date {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 统计卡片 */
.stats-section {
  margin-bottom: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  gap: 12px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-medium);
}

.stat-card.primary .stat-icon {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.stat-card.success .stat-icon {
  background: linear-gradient(135deg, #67c23a 0%, #95d475 100%);
}

.stat-card.warning .stat-icon {
  background: linear-gradient(135deg, #e6a23c 0%, #f0c78a 100%);
}

.stat-card.info .stat-icon {
  background: linear-gradient(135deg, #909399 0%, #c8c9cc 100%);
}

.stat-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
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
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}

.stat-trend {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  margin-top: 4px;
}

.stat-trend.positive {
  color: #67c23a;
}

.stat-trend.negative {
  color: #f56c6c;
}

/* 图表区域 */
.charts-section {
  margin-bottom: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.chart-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
}

.chart-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.chart-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.chart-actions {
  display: flex;
  gap: 8px;
}

.chart-period {
  font-size: 12px;
  color: var(--text-secondary);
  padding: 4px 8px;
  background: var(--bg-primary);
  border-radius: 8px;
}

.chart-container {
  position: relative;
  height: 120px;
  background: var(--bg-primary);
  border-radius: 8px;
  overflow: hidden;
}

.chart-placeholder {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 8px;
}

.chart-line {
  height: 1px;
  background: var(--border-color);
  opacity: 0.3;
}

.chart-data {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}

.data-point {
  position: absolute;
  width: 8px;
  height: 8px;
  background: var(--brand-color);
  border-radius: 50%;
  transform: translate(-50%, -50%);
}

.download-stats {
  display: flex;
  justify-content: space-around;
  align-items: flex-end;
  height: 120px;
  gap: 16px;
}

.download-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.download-bar {
  width: 100%;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 4px;
  min-height: 20px;
}

.download-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.download-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

/* 活动区域 */
.activity-section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.activity-list {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 16px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-primary);
  border-radius: 12px;
  transition: all 0.3s ease;
}

.activity-item:hover {
  background: var(--bg-secondary);
}

.activity-icon {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.activity-icon.download {
  background: linear-gradient(135deg, #67c23a 0%, #95d475 100%);
}

.activity-icon.upload {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.activity-icon.register {
  background: linear-gradient(135deg, #e6a23c 0%, #f0c78a 100%);
}

.activity-icon.view {
  background: linear-gradient(135deg, #909399 0%, #c8c9cc 100%);
}

.activity-content {
  flex: 1;
  min-width: 0;
}

.activity-text {
  font-size: 14px;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.activity-time {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 系统状态 */
.system-status {
  margin-bottom: 24px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
}

.status-indicator.online {
  color: #67c23a;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

.status-grid {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 16px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.status-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.status-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.status-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.status-bar {
  height: 6px;
  background: var(--bg-primary);
  border-radius: 3px;
  overflow: hidden;
}

.status-progress {
  height: 100%;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 3px;
  transition: width 0.3s ease;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .chart-actions {
    flex-wrap: wrap;
  }
  
  .download-stats {
    gap: 8px;
  }
}

/* 深色模式适配 */
.dark .stats-mobile {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .page-header,
.dark .stat-card,
.dark .chart-card,
.dark .activity-list,
.dark .status-grid {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

/* 主题适配 */
.blue .stat-card::before,
.blue .chart-card::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-card::before,
.green .chart-card::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .stat-card::before,
.orange .chart-card::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .stat-card::before,
.purple .chart-card::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.blue .stat-card.primary .stat-icon,
.blue .header-icon {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-card.success .stat-icon {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .stat-card.warning .stat-icon {
  background: linear-gradient(135deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .stat-card.info .stat-icon {
  background: linear-gradient(135deg, var(--info-color) 0%, var(--info-color-light) 100%);
}
</style> 