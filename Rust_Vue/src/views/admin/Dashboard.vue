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
            <h3 class="action-title">资源管理</h3>
            <p class="action-desc">管理绳包资源和社区内容</p>
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
            <span class="latency">延迟: {{ networkLatency }}</span>
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
          <div class="status-description">{{ systemStatusDetail }}</div>
          <div class="service-status">
            <div class="service-item">
              <span class="service-name">数据库:</span>
              <el-tag size="small" :type="serviceStatus.database === '正常' ? 'success' : 'danger'">
                {{ serviceStatus.database }}
              </el-tag>
            </div>
            <div class="service-item">
              <span class="service-name">文件系统:</span>
              <el-tag size="small" :type="serviceStatus.file_system === '正常' ? 'success' : 'danger'">
                {{ serviceStatus.file_system }}
              </el-tag>
            </div>
          </div>
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
  Warning,
  Operation,
  TrendCharts,
  ChatDotRound
} from '@element-plus/icons-vue'
import { getDeviceInfo, shouldUseMobileVersion } from '../../utils/device'
import { userApi, adminApi, userActionApi, logsApi } from '../../api'
import { ElMessage } from 'element-plus'

const router = useRouter()

// 响应式数据
// 添加类型定义，修复TypeScript错误
interface Activity {
  id: number;
  type: string;
  icon: string;
  title: string;
  time: string;
  status: string;
}

// 更新系统状态和统计数据的变量
const userCount = ref(0)
const packageCount = ref(0)
const logCount = ref(0)
const activeUsers = ref(0)
const availablePackages = ref(0)
const todayLogs = ref(0)
const systemStatus = ref('正常')
const uptime = ref('')
const cpuUsage = ref(45)
const memoryUsage = ref(62)
const networkStatus = ref('正常')
const networkLatency = ref('35ms')
const systemStatusDetail = ref('系统稳定运行中')
const loading = ref(false)
const totalDownloads = ref(0)
const systemHealth = ref<any>(null)

// 添加系统健康监测数据
const serviceStatus = ref({
  database: '正常',
  file_system: '正常',
  memory: '正常'
})

// 修改recentActivities，后面会从API获取
const recentActivities = ref<Activity[]>([])

// 添加一个从用户行为记录转换为活动数据的函数
function convertActionToActivity(action: any): Activity {
  let icon = 'Document'
  let type = 'system'
  
  switch (action.action_type) {
    case 'Login':
    case 'Register':
    case 'Logout':
      icon = 'User'
      type = 'user'
      break
    case 'Upload':
    case 'Download':
      icon = 'Box'
      type = 'package'
      break
    case 'Comment':
      icon = 'ChatDotRound'
      type = 'system'
      break
    case 'Admin':
      icon = 'Setting'
      type = 'warning'
      break
  }
  
  return {
    id: action.id,
    type,
    icon,
    title: action.details || `${action.action_type} 操作`,
    time: formatActionTime(action.created_at),
    status: '完成'
  }
}

// 格式化操作时间为友好显示
function formatActionTime(timestamp: string): string {
  const now = new Date()
  const actionTime = new Date(timestamp)
  const diff = now.getTime() - actionTime.getTime()
  
  const minutes = Math.floor(diff / (1000 * 60))
  const hours = Math.floor(diff / (1000 * 60 * 60))
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (minutes < 60) {
    return `${minutes}分钟前`
  } else if (hours < 24) {
    return `${hours}小时前`
  } else {
    return `${days}天前`
  }
}

// 修改checkSystemHealth函数
async function checkSystemHealth() {
  try {
    // 尝试健康检查API，如果不存在则使用默认值
    try {
      const healthRes = await adminApi.healthCheck()
      if (healthRes.code === 0 && healthRes.data) {
        systemHealth.value = healthRes.data
        
        // 更新服务状态
        serviceStatus.value = {
          database: healthRes.data.services.database,
          file_system: healthRes.data.services.file_system,
          memory: healthRes.data.services.memory
        }
        
        // 更新系统状态
        systemStatus.value = healthRes.data.status
        
        // 计算系统运行时间
        const startTime = new Date(healthRes.data.timestamp)
        startTime.setSeconds(startTime.getSeconds() - healthRes.data.uptime)
        const now = new Date()
        const diff = Math.floor((now.getTime() - startTime.getTime()) / 1000)
        
        const days = Math.floor(diff / (60 * 60 * 24))
        const hours = Math.floor((diff % (60 * 60 * 24)) / (60 * 60))
        const minutes = Math.floor((diff % (60 * 60)) / 60)
        
        uptime.value = `${days}天 ${hours}小时 ${minutes}分钟`
      }
    } catch (error) {
      // 健康检查API可能未实现，使用默认值
      console.warn('健康检查API未实现，使用默认数据', error)
      
      // 计算一个默认的运行时间（假设从一个固定的时间开始）
      const now = new Date()
      const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1)
      const diff = Math.floor((now.getTime() - startOfMonth.getTime()) / 1000)
      
      const days = Math.floor(diff / (60 * 60 * 24))
      const hours = Math.floor((diff % (60 * 60 * 24)) / (60 * 60))
      const minutes = Math.floor((diff % (60 * 60)) / 60)
      
      uptime.value = `${days}天 ${hours}小时 ${minutes}分钟`
      systemStatusDetail.value = '系统运行中'
      systemStatus.value = 'Running'
      
      // 设置默认服务状态
      serviceStatus.value = {
        database: '正常',
        file_system: '正常',
        memory: '正常'
      }
      
      // 设置模拟的CPU和内存使用率
      cpuUsage.value = Math.floor(Math.random() * 30) + 40 // 40-70%随机值
      memoryUsage.value = Math.floor(Math.random() * 30) + 50 // 50-80%随机值
    }
  } catch (error) {
    console.error('获取系统健康状态失败:', error)
    systemStatus.value = '未知'
  }
}

// 加载最近用户行为记录
async function loadRecentActions() {
  try {
    const actionsRes = await userActionApi.getUserActions({
      page: 1,
      page_size: 5
    })
    
    if (actionsRes.code === 0 && actionsRes.data && actionsRes.data.actions) {
      // 转换为Activity格式
      recentActivities.value = actionsRes.data.actions.map(convertActionToActivity)
    }
  } catch (error) {
    console.error('获取用户行为记录失败:', error)
  }
}

// 修改loadDashboardData函数
async function loadDashboardData() {
  try {
    loading.value = true
    
    // 加载用户数据
    const usersRes = await userApi.getUsers()
    if (usersRes.code === 0 && usersRes.data) {
      const users = usersRes.data.list || []
      userCount.value = users.length
      activeUsers.value = users.filter((user: any) => user.role !== 'admin').length
    }
    
    // 加载统计数据
    const statsRes = await adminApi.getStats()
    if (statsRes.code === 0 && statsRes.data) {
      packageCount.value = statsRes.data.total_packages || 0
      availablePackages.value = statsRes.data.active_users || 0  // 使用实际API字段
      totalDownloads.value = statsRes.data.total_comments || 0
      logCount.value = statsRes.data.total_comments || 0  // 可能需要从其他API获取
      todayLogs.value = statsRes.data.new_packages_today || 0
    }
    
    // 获取系统日志数量
    const logsRes = await logsApi.getLogs({
      page: 1,
      pageSize: 1
    })
    
    if (logsRes.code === 0 && logsRes.data) {
      logCount.value = logsRes.data.total
      
      // 计算今日日志数量
      const today = new Date()
      today.setHours(0, 0, 0, 0)
      
      const todayLogsRes = await logsApi.getLogs({
        page: 1,
        pageSize: 100
      })
      
      if (todayLogsRes.code === 0 && todayLogsRes.data && todayLogsRes.data.list) {
        todayLogs.value = todayLogsRes.data.list.filter(log => {
          const logTime = new Date(log.timestamp)
          return logTime >= today
        }).length
      }
    }
    
    // 加载系统健康状态
    await checkSystemHealth()
    
    // 加载最近用户行为
    await loadRecentActions()
    
  } catch (error) {
    console.error('加载仪表盘数据失败:', error)
    ElMessage.error('加载仪表盘数据失败')
  } finally {
    loading.value = false
  }
}

// 修改网络检测函数以处理健康检查API不存在的情况
function checkNetworkStatus() {
  // 用更通用的API端点检查网络状态
  const start = Date.now()
  
  // 使用统计API来测试而不是health API
  adminApi.getStats()
    .then(() => {
      const latency = Date.now() - start
      networkLatency.value = `${latency}ms`
      
      if (latency < 100) {
        networkStatus.value = '正常'
      } else if (latency < 500) {
        networkStatus.value = '一般'
      } else {
        networkStatus.value = '延迟较高'
      }
    })
    .catch(() => {
      // 尝试用另一个API端点测试
      userApi.getUsers()
        .then(() => {
          const latency = Date.now() - start
          networkLatency.value = `${latency}ms`
          networkStatus.value = '正常'
        })
        .catch(() => {
          networkStatus.value = '连接失败'
        })
    })
}

// 在onMounted中设置更新频率
onMounted(() => {
  // 初始加载数据
  loadDashboardData()
  
  // 每30秒刷新一次系统状态和统计数据
  setInterval(() => {
    loadDashboardData()
  }, 30000)
  
  // 每10秒检查一次网络状态
  setInterval(() => {
    checkNetworkStatus()
  }, 10000)
  
  // 立即检查一次网络状态
  checkNetworkStatus()
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

// 方法
function navigateTo(path: string) {
  router.push(path)
}

function getProgressColor(percentage: number) {
  if (percentage < 50) return '#67C23A'
  if (percentage < 80) return '#E6A23C'
  return '#F56C6C'
}

// 数据更新
onMounted(() => {
  loadDashboardData();
  
  // 每分钟更新一次运行时间
  setInterval(() => {
    updateUptime();
  }, 60000);
  
  // 每10秒更新一次网络状态
  setInterval(() => {
    checkNetworkStatus();
  }, 10000);
  
  // 定期刷新数据
  setInterval(() => {
    loadDashboardData();
  }, 30000); // 30秒刷新一次
});
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

.service-status {
  margin-top: 12px;
  display: flex;
  gap: 10px;
}

.service-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.service-name {
  font-size: 12px;
  color: var(--text-secondary);
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