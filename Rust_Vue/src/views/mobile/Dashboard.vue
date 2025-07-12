<template>
  <div class="dashboard-mobile">
    <!-- 欢迎区域 -->
    <div class="welcome-section">
      <div class="welcome-background">
        <div class="bg-pattern"></div>
        <div class="bg-gradient"></div>
      </div>
      <div class="welcome-content">
        <div class="welcome-text">
          <div class="greeting-line">
            <span class="greeting">{{ greeting }}</span>
            <div class="time-indicator">
              <el-icon><Clock /></el-icon>
              <span>{{ currentTime }}</span>
            </div>
          </div>
          <h1 class="welcome-title">
            <span class="username">{{ username }}</span>
          </h1>
          <p class="welcome-subtitle">欢迎使用绳包管理系统</p>
          <div class="welcome-stats">
            <div class="mini-stat">
              <span class="stat-value">{{ userCount }}</span>
              <span class="stat-label">用户</span>
            </div>
            <div class="mini-stat">
              <span class="stat-value">{{ packageCount }}</span>
              <span class="stat-label">绳包</span>
            </div>
            <div class="mini-stat">
              <span class="stat-value">{{ logCount }}</span>
              <span class="stat-label">日志</span>
            </div>
          </div>
        </div>
        <div class="welcome-illustration">
          <div class="illustration-container">
            <div class="illustration-circle">
              <el-icon :size="28"><DataAnalysis /></el-icon>
            </div>
            <div class="floating-dots">
              <div class="dot dot-1"></div>
              <div class="dot dot-2"></div>
              <div class="dot dot-3"></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 快速统计卡片 -->
    <div class="stats-section">
      <div class="section-header">
        <h2 class="section-title">数据概览</h2>
        <div class="section-subtitle">实时系统数据</div>
      </div>
      <div class="stats-grid">
        <div class="stat-card users" @click="navigateTo('/users')">
          <div class="stat-header">
            <div class="stat-icon">
              <el-icon :size="20"><User /></el-icon>
            </div>
            <div class="stat-trend positive">
              <el-icon><TrendCharts /></el-icon>
              <span>+12%</span>
            </div>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ userCount }}</div>
            <div class="stat-label">总用户数</div>
            <div class="stat-description">活跃用户 {{ activeUsers }} 人</div>
          </div>
          <div class="stat-chart">
            <div class="chart-bar" style="height: 60%"></div>
            <div class="chart-bar" style="height: 80%"></div>
            <div class="chart-bar" style="height: 45%"></div>
            <div class="chart-bar" style="height: 90%"></div>
          </div>
        </div>

        <div class="stat-card packages" @click="navigateTo('/packages')">
          <div class="stat-header">
            <div class="stat-icon">
              <el-icon :size="20"><Box /></el-icon>
            </div>
            <div class="stat-trend positive">
              <el-icon><TrendCharts /></el-icon>
              <span>+8%</span>
            </div>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ packageCount }}</div>
            <div class="stat-label">绳包总数</div>
            <div class="stat-description">可用绳包 {{ availablePackages }} 个</div>
          </div>
          <div class="stat-chart">
            <div class="chart-bar" style="height: 70%"></div>
            <div class="chart-bar" style="height: 85%"></div>
            <div class="chart-bar" style="height: 55%"></div>
            <div class="chart-bar" style="height: 95%"></div>
          </div>
        </div>

        <div class="stat-card logs" @click="navigateTo('/logs')">
          <div class="stat-header">
            <div class="stat-icon">
              <el-icon :size="20"><Document /></el-icon>
            </div>
            <div class="stat-trend neutral">
              <el-icon><TrendCharts /></el-icon>
              <span>+5%</span>
            </div>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ logCount }}</div>
            <div class="stat-label">系统日志</div>
            <div class="stat-description">今日新增 {{ todayLogs }} 条</div>
          </div>
          <div class="stat-chart">
            <div class="chart-bar" style="height: 50%"></div>
            <div class="chart-bar" style="height: 75%"></div>
            <div class="chart-bar" style="height: 65%"></div>
            <div class="chart-bar" style="height: 80%"></div>
          </div>
        </div>

        <div class="stat-card system" @click="navigateTo('/stats')">
          <div class="stat-header">
            <div class="stat-icon">
              <el-icon :size="20"><Setting /></el-icon>
            </div>
            <div class="stat-trend positive">
              <el-icon><TrendCharts /></el-icon>
              <span>正常</span>
            </div>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ systemStatus }}</div>
            <div class="stat-label">系统状态</div>
            <div class="stat-description">运行时间 {{ uptime }}</div>
          </div>
          <div class="stat-chart">
            <div class="chart-bar" style="height: 90%"></div>
            <div class="chart-bar" style="height: 95%"></div>
            <div class="chart-bar" style="height: 88%"></div>
            <div class="chart-bar" style="height: 92%"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- 快速操作区域 -->
    <div class="quick-actions">
      <div class="section-header">
        <h2 class="section-title">快速操作</h2>
        <div class="section-subtitle">常用功能入口</div>
      </div>
      <div class="actions-grid">
        <div class="action-card" @click="navigateTo('/users')">
          <div class="action-icon">
            <el-icon :size="24"><User /></el-icon>
          </div>
          <div class="action-content">
            <h3 class="action-title">用户管理</h3>
            <p class="action-desc">管理系统用户和权限</p>
          </div>
          <div class="action-arrow">
            <el-icon><Right /></el-icon>
          </div>
        </div>

        <div class="action-card" @click="navigateTo('/packages')">
          <div class="action-icon">
            <el-icon :size="24"><Box /></el-icon>
          </div>
          <div class="action-content">
            <h3 class="action-title">绳包管理</h3>
            <p class="action-desc">管理绳包信息和状态</p>
          </div>
          <div class="action-arrow">
            <el-icon><Right /></el-icon>
          </div>
        </div>

        <div class="action-card" @click="navigateTo('/logs')">
          <div class="action-icon">
            <el-icon :size="24"><Document /></el-icon>
          </div>
          <div class="action-content">
            <h3 class="action-title">日志查看</h3>
            <p class="action-desc">查看系统运行日志</p>
          </div>
          <div class="action-arrow">
            <el-icon><Right /></el-icon>
          </div>
        </div>

        <div class="action-card" @click="navigateTo('/stats')">
          <div class="action-icon">
            <el-icon :size="24"><DataAnalysis /></el-icon>
          </div>
          <div class="action-content">
            <h3 class="action-title">统计信息</h3>
            <p class="action-desc">查看系统统计数据</p>
          </div>
          <div class="action-arrow">
            <el-icon><Right /></el-icon>
          </div>
        </div>
      </div>
    </div>

    <!-- 系统状态区域 -->
    <div class="system-status">
      <div class="section-header">
        <h2 class="section-title">系统状态</h2>
        <div class="section-subtitle">实时监控信息</div>
      </div>
      <div class="status-grid">
        <div class="status-card">
          <div class="status-header">
            <div class="status-icon">
              <el-icon :size="16"><Setting /></el-icon>
            </div>
            <div class="status-label">CPU使用率</div>
          </div>
          <div class="status-value">{{ cpuUsage }}%</div>
          <div class="status-progress">
            <el-progress :percentage="cpuUsage" :color="getProgressColor(cpuUsage)" :stroke-width="6" />
          </div>
        </div>

        <div class="status-card">
          <div class="status-header">
            <div class="status-icon">
              <el-icon :size="16"><Document /></el-icon>
            </div>
            <div class="status-label">内存使用率</div>
          </div>
          <div class="status-value">{{ memoryUsage }}%</div>
          <div class="status-progress">
            <el-progress :percentage="memoryUsage" :color="getProgressColor(memoryUsage)" :stroke-width="6" />
          </div>
        </div>

        <div class="status-card">
          <div class="status-header">
            <div class="status-icon">
              <el-icon :size="16"><Link /></el-icon>
            </div>
            <div class="status-label">网络状态</div>
          </div>
          <div class="status-value">{{ networkStatus }}</div>
          <div class="status-indicator">
            <div class="indicator-dot" :class="networkStatus === '正常' ? 'online' : 'offline'"></div>
            <span class="indicator-text">{{ networkStatus === '正常' ? '连接正常' : '连接异常' }}</span>
          </div>
        </div>

        <div class="status-card">
          <div class="status-header">
            <div class="status-icon">
              <el-icon :size="16"><Clock /></el-icon>
            </div>
            <div class="status-label">运行时间</div>
          </div>
          <div class="status-value">{{ uptime }}</div>
          <div class="status-description">系统稳定运行中</div>
        </div>
      </div>
    </div>

    <!-- 最近活动 -->
    <div class="recent-activities">
      <div class="section-header">
        <h2 class="section-title">最近活动</h2>
        <div class="section-subtitle">系统动态更新</div>
      </div>
      <div class="activities-list">
        <div v-for="activity in recentActivities" :key="activity.id" class="activity-item">
          <div class="activity-icon" :class="activity.type">
            <el-icon :size="14">
              <component :is="activity.icon" />
            </el-icon>
          </div>
          <div class="activity-content">
            <div class="activity-title">{{ activity.title }}</div>
            <div class="activity-time">{{ activity.time }}</div>
          </div>
          <div class="activity-status" :class="activity.status">
            {{ activity.status }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { 
  User, 
  Box, 
  Document, 
  DataAnalysis, 
  Setting, 
  Right,
  Link,
  Clock,
  Plus,
  Edit,
  Delete,
  Warning
} from '@element-plus/icons-vue'

const router = useRouter()

// 响应式数据
const userCount = ref(156)
const packageCount = ref(342)
const logCount = ref(1247)
const activeUsers = ref(89)
const availablePackages = ref(298)
const todayLogs = ref(23)
const systemStatus = ref('正常')
const uptime = ref('15天 8小时 32分钟')
const cpuUsage = ref(45)
const memoryUsage = ref(62)
const networkStatus = ref('正常')

// 计算属性
const greeting = computed(() => {
  const hour = new Date().getHours()
  if (hour < 6) return '夜深了'
  if (hour < 12) return '早上好'
  if (hour < 18) return '下午好'
  return '晚上好'
})

const currentTime = computed(() => {
  return new Date().toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
  })
})

const username = computed(() => {
  const userInfo = localStorage.getItem('userInfo')
  if (userInfo) {
    try {
      const user = JSON.parse(userInfo)
      return user.username || '管理员'
    } catch {
      return '管理员'
    }
  }
  return '管理员'
})

// 最近活动数据
const recentActivities = ref([
  {
    id: 1,
    type: 'user',
    icon: 'User',
    title: '新用户注册',
    time: '2分钟前',
    status: '成功'
  },
  {
    id: 2,
    type: 'package',
    icon: 'Box',
    title: '绳包状态更新',
    time: '5分钟前',
    status: '完成'
  },
  {
    id: 3,
    type: 'system',
    icon: 'Setting',
    title: '系统维护完成',
    time: '15分钟前',
    status: '成功'
  },
  {
    id: 4,
    type: 'warning',
    icon: 'Warning',
    title: '异常登录检测',
    time: '1小时前',
    status: '已处理'
  },
  {
    id: 5,
    type: 'package',
    icon: 'Box',
    title: '绳包借出',
    time: '2小时前',
    status: '完成'
  }
])

// 方法
function navigateTo(path: string) {
  router.push(path)
}

function getProgressColor(percentage: number) {
  if (percentage < 50) return '#67C23A'
  if (percentage < 80) return '#E6A23C'
  return '#F56C6C'
}

// 模拟数据更新
onMounted(() => {
  // 模拟实时数据更新
  setInterval(() => {
    cpuUsage.value = Math.floor(Math.random() * 30) + 30
    memoryUsage.value = Math.floor(Math.random() * 20) + 50
  }, 5000)
})
</script>

<style scoped>
.dashboard-mobile {
  padding: 16px;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  min-height: 100vh;
  position: relative;
}

/* 欢迎区域 */
.welcome-section {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.welcome-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: -1;
  overflow: hidden;
}

.bg-pattern {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: radial-gradient(circle at 25% 25%, rgba(64, 158, 255, 0.1) 0%, transparent 50%),
                    radial-gradient(circle at 75% 75%, rgba(103, 194, 58, 0.1) 0%, transparent 50%);
  background-size: 100px 100px;
  opacity: 0.3;
  animation: movePattern 20s linear infinite;
}

.bg-gradient {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, rgba(64, 158, 255, 0.05) 0%, rgba(103, 194, 58, 0.05) 100%);
  z-index: -1;
}

@keyframes movePattern {
  0% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(-50px, -50px);
  }
}

.welcome-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: relative;
  z-index: 1;
}

.welcome-text {
  flex: 1;
}

.greeting-line {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.greeting {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.time-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
  color: var(--text-secondary);
  background: rgba(255, 255, 255, 0.1);
  padding: 4px 8px;
  border-radius: 12px;
  backdrop-filter: blur(10px);
}

.welcome-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.username {
  color: var(--brand-color);
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.welcome-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0 0 16px 0;
}

.welcome-stats {
  display: flex;
  gap: 20px;
  margin-top: 16px;
}

.mini-stat {
  text-align: center;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.mini-stat .stat-value {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
  display: block;
}

.mini-stat .stat-label {
  font-size: 10px;
  color: var(--text-secondary);
  font-weight: 500;
  display: block;
  margin-top: 2px;
}

.welcome-illustration {
  display: flex;
  align-items: center;
  justify-content: center;
}

.illustration-container {
  position: relative;
  width: 70px;
  height: 70px;
}

.illustration-circle {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 8px 24px rgba(64, 158, 255, 0.3);
  animation: pulse 2s infinite;
  position: relative;
  z-index: 2;
}

.floating-dots {
  position: absolute;
  top: -10px;
  left: -10px;
  width: calc(100% + 20px);
  height: calc(100% + 20px);
  pointer-events: none;
}

.dot {
  position: absolute;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.6);
  animation: float 3s infinite ease-in-out;
}

.dot-1 {
  top: 20%;
  left: 20%;
  animation-delay: -0.5s;
}
.dot-2 {
  top: 60%;
  right: 20%;
  animation-delay: -1s;
}
.dot-3 {
  bottom: 20%;
  left: 50%;
  animation-delay: -1.5s;
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.05);
    opacity: 0.9;
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) scale(1);
    opacity: 0.6;
  }
  50% {
    transform: translateY(-8px) scale(1.1);
    opacity: 1;
  }
}

/* 统计网格 */
.stats-section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  margin-bottom: 16px;
}

.section-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 4px 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.section-subtitle {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 16px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  box-shadow: var(--shadow-light);
  cursor: pointer;
  backdrop-filter: blur(10px);
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
  transform: translateY(-4px);
  box-shadow: var(--shadow-base);
  border-color: var(--brand-color);
}

.stat-card:hover::before {
  transform: scaleX(1);
}

.stat-card:hover::after {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

.stat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.stat-icon {
  width: 36px;
  height: 36px;
  background: var(--bg-secondary);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--brand-color);
  transition: all 0.3s ease;
}

.stat-card:hover .stat-icon {
  background: var(--brand-color);
  color: white;
  transform: scale(1.1) rotate(5deg);
}

.stat-trend {
  display: flex;
  align-items: center;
  gap: 2px;
  font-size: 10px;
  font-weight: 600;
  padding: 4px 8px;
  border-radius: 8px;
}

.stat-trend.positive {
  color: var(--success-color);
  background: rgba(103, 194, 58, 0.1);
}

.stat-trend.neutral {
  color: var(--warning-color);
  background: rgba(230, 162, 60, 0.1);
}

.stat-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-number {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
  font-weight: 500;
}

.stat-description {
  font-size: 9px;
  color: var(--text-tertiary);
}

.stat-chart {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  height: 24px;
  margin-top: 12px;
  background: var(--bg-secondary);
  border-radius: 12px;
  overflow: hidden;
  padding: 0 6px;
  gap: 2px;
}

.chart-bar {
  flex: 1;
  background: linear-gradient(to top, var(--brand-color), var(--brand-color-light));
  border-radius: 4px;
  transition: height 0.5s ease-in-out;
  min-height: 4px;
}

/* 快速操作 */
.quick-actions {
  margin-bottom: 24px;
}

.actions-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.action-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: var(--shadow-light);
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(10px);
}

.action-card::before {
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

.action-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
  border-color: var(--brand-color);
}

.action-card:hover::before {
  transform: scaleX(1);
}

.action-icon {
  width: 52px;
  height: 52px;
  background: var(--bg-secondary);
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--brand-color);
  transition: all 0.3s ease;
  flex-shrink: 0;
}

.action-card:hover .action-icon {
  background: var(--brand-color);
  color: white;
  transform: scale(1.1) rotate(5deg);
}

.action-content {
  flex: 1;
  min-width: 0;
}

.action-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.action-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
}

.action-arrow {
  color: var(--text-tertiary);
  transition: all 0.3s ease;
}

.action-card:hover .action-arrow {
  color: var(--brand-color);
  transform: translateX(4px);
}

/* 系统状态 */
.system-status {
  margin-bottom: 24px;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.status-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 16px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  box-shadow: var(--shadow-light);
  backdrop-filter: blur(10px);
}

.status-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
  border-color: var(--brand-color);
}

.status-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.status-icon {
  width: 28px;
  height: 28px;
  background: var(--bg-secondary);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--brand-color);
}

.status-label {
  font-size: 11px;
  color: var(--text-secondary);
  font-weight: 500;
}

.status-value {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.status-progress {
  margin-bottom: 4px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
}

.indicator-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.indicator-dot.online {
  background: var(--success-color);
  box-shadow: 0 0 8px rgba(103, 194, 58, 0.5);
  animation: pulse 2s infinite;
}

.indicator-dot.offline {
  background: var(--danger-color);
  box-shadow: 0 0 8px rgba(245, 108, 108, 0.5);
}

.indicator-text {
  font-size: 10px;
  color: var(--text-secondary);
}

.status-description {
  font-size: 9px;
  color: var(--text-tertiary);
}

/* 最近活动 */
.recent-activities {
  margin-bottom: 24px;
}

.activities-list {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-light);
  backdrop-filter: blur(10px);
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 0;
  border-bottom: 1px solid var(--border-color);
  transition: all 0.3s ease;
}

.activity-item:last-child {
  border-bottom: none;
}

.activity-item:hover {
  background: var(--bg-secondary);
  margin: 0 -20px;
  padding: 16px 20px;
  border-radius: 12px;
}

.activity-icon {
  width: 32px;
  height: 32px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 12px;
  flex-shrink: 0;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.activity-icon.user {
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light));
}

.activity-icon.package {
  background: linear-gradient(135deg, var(--success-color), var(--success-color-light));
}

.activity-icon.system {
  background: linear-gradient(135deg, var(--warning-color), var(--warning-color-light));
}

.activity-icon.warning {
  background: linear-gradient(135deg, var(--danger-color), var(--danger-color-light));
}

.activity-content {
  flex: 1;
  min-width: 0;
}

.activity-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.activity-time {
  font-size: 10px;
  color: var(--text-secondary);
}

.activity-status {
  font-size: 10px;
  font-weight: 500;
  padding: 4px 8px;
  border-radius: 8px;
  flex-shrink: 0;
}

.activity-status.成功,
.activity-status.完成 {
  color: var(--success-color);
  background: rgba(103, 194, 58, 0.1);
}

.activity-status.已处理 {
  color: var(--warning-color);
  background: rgba(230, 162, 60, 0.1);
}

/* 深色模式适配 */
.dark .stat-card,
.dark .action-card,
.dark .status-card,
.dark .activities-list {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
}

/* 响应式设计 */
@media (max-width: 480px) {
  .dashboard-mobile {
    padding: 12px;
  }
  
  .welcome-content {
    flex-direction: column;
    text-align: center;
    gap: 20px;
  }
  
  .welcome-title {
    font-size: 24px;
  }
  
  .welcome-stats {
    justify-content: center;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }
  
  .status-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }
  
  .action-card {
    padding: 16px;
  }
  
  .action-icon {
    width: 48px;
    height: 48px;
  }
  
  .action-title {
    font-size: 15px;
  }
  
  .action-desc {
    font-size: 11px;
  }
}

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .stat-card,
  .action-card,
  .status-card,
  .activity-item {
    transition: none;
  }
  
  .stat-card:hover,
  .action-card:hover,
  .status-card:hover {
    transform: none;
  }
  
  .illustration-circle,
  .dot {
    animation: none;
  }
}
</style> 