<template>   
    <!-- 欢迎区域 -->
    <div class="welcome-section">
      <div class="welcome-content">
        <div class="welcome-text">
          <h1 class="welcome-title">
            <span class="greeting">{{ greeting }}</span>
            <span class="username">{{ username }}</span>
          </h1>
          <p class="welcome-subtitle">欢迎使用绳包管理系统</p>
          <div class="welcome-stats">
            <div class="stat-item">
              <el-icon><User /></el-icon>
              <span>{{ userCount }} 个用户</span>
            </div>
            <div class="stat-item">
              <el-icon><Box /></el-icon>
              <span>{{ packageCount }} 个绳包</span>
            </div>
            <div class="stat-item">
              <el-icon><Document /></el-icon>
              <span>{{ logCount }} 条日志</span>
            </div>
          </div>
        </div>
        <div class="welcome-illustration">
          <div class="illustration-circle">
            <el-icon :size="48"><DataAnalysis /></el-icon>
          </div>
        </div>
      </div>
    </div>

    <!-- 快速统计卡片 -->
    <div class="stats-grid">
      <div class="stat-card users">
        <div class="stat-header">
          <div class="stat-icon">
            <el-icon :size="24"><User /></el-icon>
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
      </div>

      <div class="stat-card packages">
        <div class="stat-header">
          <div class="stat-icon">
            <el-icon :size="24"><Box /></el-icon>
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
      </div>

      <div class="stat-card logs">
        <div class="stat-header">
          <div class="stat-icon">
            <el-icon :size="24"><Document /></el-icon>
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
      </div>

      <div class="stat-card system">
        <div class="stat-header">
          <div class="stat-icon">
            <el-icon :size="24"><Setting /></el-icon>
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
      </div>
    </div>

    <!-- 快速操作区域 -->
    <div class="quick-actions">
      <h2 class="section-title">
        <el-icon><Operation /></el-icon>
        快速操作
      </h2>
      <div class="actions-grid">
        <div class="action-card" @click="navigateTo('/users')">
          <div class="action-icon">
            <el-icon :size="32"><User /></el-icon>
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
            <el-icon :size="32"><Box /></el-icon>
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
            <el-icon :size="32"><Document /></el-icon>
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
            <el-icon :size="32"><DataAnalysis /></el-icon>
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
      <h2 class="section-title">
                    <el-icon><Setting /></el-icon>
        系统状态
      </h2>
      <div class="status-grid">
        <div class="status-card">
          <div class="status-header">
            <div class="status-icon">
              <el-icon :size="20"><Setting /></el-icon>
            </div>
            <div class="status-label">CPU使用率</div>
          </div>
          <div class="status-value">{{ cpuUsage }}%</div>
          <div class="status-progress">
            <el-progress :percentage="cpuUsage" :color="getProgressColor(cpuUsage)" />
          </div>
        </div>

        <div class="status-card">
          <div class="status-header">
                      <div class="status-icon">
            <el-icon :size="20"><Document /></el-icon>
          </div>
            <div class="status-label">内存使用率</div>
          </div>
          <div class="status-value">{{ memoryUsage }}%</div>
          <div class="status-progress">
            <el-progress :percentage="memoryUsage" :color="getProgressColor(memoryUsage)" />
          </div>
        </div>

        <div class="status-card">
          <div class="status-header">
            <div class="status-icon">
              <el-icon :size="20"><Link /></el-icon>
            </div>
            <div class="status-label">网络状态</div>
          </div>
          <div class="status-value">{{ networkStatus }}</div>
          <div class="status-indicator">
            <div class="indicator-dot" :class="networkStatus === '正常' ? 'online' : 'offline'"></div>
          </div>
        </div>

        <div class="status-card">
          <div class="status-header">
            <div class="status-icon">
              <el-icon :size="20"><Clock /></el-icon>
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
      <h2 class="section-title">
        <el-icon><Clock /></el-icon>
        最近活动
      </h2>
      <div class="activities-list">
        <div v-for="activity in recentActivities" :key="activity.id" class="activity-item">
          <div class="activity-icon" :class="activity.type">
            <el-icon :size="16">
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
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue'
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
import { getDeviceInfo, shouldUseMobileVersion } from '../../utils/device'

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

// 设备信息（用于调试）
const deviceInfo = ref(getDeviceInfo())
const shouldUseMobile = ref(shouldUseMobileVersion())

// 更新设备信息
const updateDeviceInfo = () => {
  deviceInfo.value = getDeviceInfo()
  shouldUseMobile.value = shouldUseMobileVersion()
}

// 监听窗口大小变化
onMounted(() => {
  updateDeviceInfo()
  window.addEventListener('resize', updateDeviceInfo)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateDeviceInfo)
})

// 计算属性
const greeting = computed(() => {
  const hour = new Date().getHours()
  if (hour < 6) return '夜深了'
  if (hour < 12) return '早上好'
  if (hour < 18) return '下午好'
  return '晚上好'
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
.dashboard {
  padding: 24px;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  min-height: 100vh;
}

/* 欢迎区域 */
.welcome-section {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 32px;
  margin-bottom: 32px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  position: relative;
  overflow: hidden;
}

.welcome-section::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(64, 158, 255, 0.1) 0%, rgba(103, 194, 58, 0.1) 100%);
  z-index: 0;
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

.welcome-title {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  display: flex;
  align-items: center;
  gap: 12px;
}

.greeting {
  color: var(--text-secondary);
}

.username {
  color: var(--brand-color);
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.welcome-subtitle {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0 0 24px 0;
}

.welcome-stats {
  display: flex;
  gap: 24px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary);
  font-size: 14px;
}

.welcome-illustration {
  display: flex;
  align-items: center;
  justify-content: center;
}

.illustration-circle {
  width: 120px;
  height: 120px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 8px 32px rgba(64, 158, 255, 0.3);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.05);
    opacity: 0.8;
  }
}

/* 统计网格 */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 24px;
  margin-bottom: 32px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  box-shadow: var(--shadow-light);
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
  margin-bottom: 16px;
}

.stat-icon {
  width: 48px;
  height: 48px;
  background: var(--bg-secondary);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--brand-color);
  transition: all 0.3s ease;
}

.stat-card:hover .stat-icon {
  background: var(--brand-color);
  color: white;
  transform: scale(1.1);
}

.stat-trend {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  font-weight: 600;
  padding: 4px 8px;
  border-radius: 6px;
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
  gap: 8px;
}

.stat-number {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 14px;
  color: var(--text-secondary);
  font-weight: 500;
}

.stat-description {
  font-size: 12px;
  color: var(--text-tertiary);
}

/* 快速操作 */
.quick-actions {
  margin-bottom: 32px;
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 24px 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
}

.action-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: var(--shadow-light);
  position: relative;
  overflow: hidden;
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
  width: 64px;
  height: 64px;
  background: var(--bg-secondary);
  border-radius: 16px;
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
  transform: scale(1.1);
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
  font-size: 14px;
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
  margin-bottom: 32px;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 20px;
}

.status-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  box-shadow: var(--shadow-light);
}

.status-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
  border-color: var(--brand-color);
}

.status-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.status-icon {
  width: 40px;
  height: 40px;
  background: var(--bg-secondary);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--brand-color);
}

.status-label {
  font-size: 14px;
  color: var(--text-secondary);
  font-weight: 500;
}

.status-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 12px;
}

.status-progress {
  margin-bottom: 8px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
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
}

.indicator-dot.offline {
  background: var(--danger-color);
  box-shadow: 0 0 8px rgba(245, 108, 108, 0.5);
}

.status-description {
  font-size: 12px;
  color: var(--text-tertiary);
}

/* 最近活动 */
.recent-activities {
  margin-bottom: 32px;
}

.activities-list {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-light);
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 16px;
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
  border-radius: 8px;
}

.activity-icon {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 12px;
  flex-shrink: 0;
}

.activity-icon.user {
  background: var(--brand-color);
}

.activity-icon.package {
  background: var(--success-color);
}

.activity-icon.system {
  background: var(--warning-color);
}

.activity-icon.warning {
  background: var(--danger-color);
}

.activity-content {
  flex: 1;
  min-width: 0;
}

.activity-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.activity-time {
  font-size: 12px;
  color: var(--text-secondary);
}

.activity-status {
  font-size: 12px;
  font-weight: 500;
  padding: 4px 8px;
  border-radius: 6px;
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
@media (max-width: 768px) {
  .dashboard {
    padding: 16px;
  }
  
  .welcome-content {
    flex-direction: column;
    text-align: center;
    gap: 24px;
  }
  
  .welcome-title {
    font-size: 24px;
  }
  
  .welcome-stats {
    justify-content: center;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .actions-grid {
    grid-template-columns: 1fr;
  }
  
  .status-grid {
    grid-template-columns: 1fr;
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
}
</style> 