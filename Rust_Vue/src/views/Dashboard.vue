<template>
  <div class="dashboard">
    <!-- 数据更新通知 -->
    <el-alert
      v-if="showUpdateNotification"
      title="数据已更新"
      type="success"
      :closable="false"
      show-icon
      class="update-notification"
    >
      <template #default>
        仪表盘数据已自动更新 ({{ lastUpdateTime }})
      </template>
    </el-alert>

    <!-- 欢迎卡片 -->
    <el-card class="welcome-card">
      <div class="welcome-content">
        <div class="welcome-text">
          <h2>管理系统仪表盘</h2>
          <p class="welcome">欢迎使用绳包管理系统！</p>
          <p class="time">{{ currentTime }}</p>
        </div>
        <div class="welcome-icon">
          <el-icon :size="56">
            <Setting />
          </el-icon>
        </div>
      </div>
    </el-card>

    <!-- 自动刷新状态栏 -->
    <div class="auto-refresh-status">
      <div class="status-info">
        <el-icon :class="{ 'rotating': isLoading }">
          <Refresh />
        </el-icon>
        <span v-if="isLoading" class="status-text">正在更新数据...</span>
        <span v-else-if="autoRefreshEnabled" class="status-text">
          <span class="desktop-only">自动刷新已启用 ({{ Math.round(refreshInterval / 1000) }}秒间隔)</span>
          <span class="mobile-only">自动刷新已启用</span>
        </span>
        <span v-else class="status-text">自动刷新已禁用</span>
      </div>
      <div class="status-actions">
        <el-button 
          size="small" 
          type="primary" 
          :loading="isLoading"
          @click="refreshData"
          class="refresh-btn"
        >
          <span class="desktop-only">立即刷新</span>
          <span class="mobile-only">刷新</span>
        </el-button>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-cards" :key="`stats-${forceUpdate}`">
      <el-card class="stat-card" :class="{ 'data-update': !isLoading }">
        <div class="stat-content">
          <div class="stat-icon user-icon">
            <el-icon :size="26">
              <User />
            </el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-label">用户总数</div>
            <div class="stat-value">{{ userCount }}</div>
            <div class="stat-change" :class="{ positive: userGrowth > 0, negative: userGrowth < 0 }">
              {{ userGrowth > 0 ? '+' : '' }}{{ userGrowth }}% 本月
            </div>
          </div>
        </div>
      </el-card>

      <el-card class="stat-card" :class="{ 'data-update': !isLoading }">
        <div class="stat-content">
          <div class="stat-icon package-icon">
            <el-icon :size="26">
              <Box />
            </el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-label">绳包总数</div>
            <div class="stat-value">{{ packageCount }}</div>
            <div class="stat-change" :class="{ positive: packageGrowth > 0, negative: packageGrowth < 0 }">
              {{ packageGrowth > 0 ? '+' : '' }}{{ packageGrowth }}% 本月
            </div>
          </div>
        </div>
      </el-card>

      <el-card class="stat-card" :class="{ 'data-update': !isLoading }">
        <div class="stat-content">
          <div class="stat-icon download-icon">
            <el-icon :size="26">
              <Download />
            </el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-label">总下载量</div>
            <div class="stat-value">{{ totalDownloads }}</div>
            <div class="stat-change" :class="{ positive: downloadGrowth > 0, negative: downloadGrowth < 0 }">
              {{ downloadGrowth > 0 ? '+' : '' }}{{ downloadGrowth }}% 本月
            </div>
          </div>
        </div>
      </el-card>

      <el-card class="stat-card" :class="{ 'data-update': !isLoading }">
        <div class="stat-content">
          <div class="stat-icon active-icon">
            <el-icon :size="26">
              <TrendCharts />
            </el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-label">活跃用户</div>
            <div class="stat-value">{{ activeUsers }}</div>
            <div class="stat-change" :class="{ positive: activeGrowth > 0, negative: activeGrowth < 0 }">
              {{ activeGrowth > 0 ? '+' : '' }}{{ activeGrowth }}% 本月
            </div>
          </div>
        </div>
      </el-card>
    </div>

    <!-- 图表和详细信息 -->
    <div class="dashboard-content" :key="`content-${forceUpdate}`">
      <!-- 左侧：图表和活动 -->
      <div class="left-content">
        <!-- 神包下载趋势图 -->
        <el-card class="chart-card">
          <template #header>
            <div class="card-header">
              <div class="header-left">
                <span>神包下载趋势</span>
                <div class="update-info">
                  <small v-if="lastUpdateTime">最后更新: {{ lastUpdateTime }}</small>
                  <el-switch
                    v-model="autoRefreshEnabled"
                    size="small"
                    style="margin-left: 12px;"
                    @change="toggleAutoRefresh"
                  />
                  <small style="margin-left: 8px;">自动刷新</small>
                </div>
              </div>
              <div class="header-right">
                <el-button 
                  type="text" 
                  :loading="isLoading"
                  @click="refreshData"
                >
                  <el-icon>
                    <Refresh />
                  </el-icon>
                  刷新
                </el-button>
              </div>
            </div>
          </template>
          <div class="chart-container">
            <v-chart 
              v-if="packageDownloadTrends.length > 0" 
              :option="packageChartOption" 
              class="chart"
              :key="`chart-${forceUpdate}`"
              @resize="handleChartResize"
            />
            <div v-else class="chart-placeholder">
              <el-icon :size="48">
                <DataAnalysis />
              </el-icon>
              <p>神包下载趋势图表</p>
              <small>暂无数据</small>
            </div>
          </div>
        </el-card>

        <!-- 最近活动 -->
        <el-card class="activity-card">
          <template #header>
            <div class="card-header">
              <span>最近活动</span>
            </div>
          </template>
          <div class="activity-list">
            <div v-if="recentActivities.length === 0" class="empty-state">
              <el-icon :size="48" style="color: #c0c4cc; margin-bottom: 16px;">
                <DataAnalysis />
              </el-icon>
              <p style="color: #909399; margin: 0;">暂无活动数据</p>
            </div>
            <div v-else v-for="activity in recentActivities" :key="activity.id" class="activity-item">
              <div class="activity-icon" :class="getActivityTypeClass(activity.type)">
                <el-icon :size="18">
                  <component :is="getActivityIcon(activity.type)" />
                </el-icon>
              </div>
              <div class="activity-content">
                <div class="activity-title">{{ activity.title }}</div>
                <div class="activity-time">{{ activity.time }}</div>
              </div>
            </div>
          </div>
        </el-card>
      </div>

      <!-- 右侧：系统状态和快速操作 -->
      <div class="right-content">
        <!-- 系统状态 -->
        <el-card class="status-card">
          <template #header>
            <div class="card-header">
              <span>系统状态</span>
            </div>
          </template>
          <div class="status-list">
            <div class="status-item">
              <span class="status-label">
                <el-icon :size="16" style="margin-right: 8px; color: #409eff;">
                  <DataAnalysis />
                </el-icon>
                数据库连接
              </span>
              <el-tag :type="systemStatus.database === '正常' ? 'success' : 'danger'" size="small">{{ systemStatus.database }}</el-tag>
            </div>
            <div class="status-item">
              <span class="status-label">
                <el-icon :size="16" style="margin-right: 8px; color: #67c23a;">
                  <Connection />
                </el-icon>
                API服务
              </span>
              <el-tag :type="systemStatus.api_service === '正常' ? 'success' : 'danger'" size="small">{{ systemStatus.api_service }}</el-tag>
            </div>
            <div class="status-item">
              <span class="status-label">
                <el-icon :size="16" style="margin-right: 8px; color: #e6a23c;">
                  <Document />
                </el-icon>
                日志系统
              </span>
              <el-tag :type="systemStatus.log_system === '正常' ? 'success' : 'warning'" size="small">{{ systemStatus.log_system }}</el-tag>
            </div>
            <div class="status-item">
              <span class="status-label">
                <el-icon :size="16" style="margin-right: 8px; color: #f56c6c;">
                  <Folder />
                </el-icon>
                存储空间
              </span>
              <el-tag :type="systemStatus.storage.includes('%') ? 'warning' : 'success'" size="small">{{ systemStatus.storage }}</el-tag>
            </div>
          </div>
        </el-card>

        <!-- 快速操作 -->
        <el-card class="quick-actions-card">
          <template #header>
            <div class="card-header">
              <span>快速操作</span>
            </div>
          </template>
          <div class="quick-actions">
            <el-button type="primary" @click="goToUsers" class="action-btn">
              <el-icon>
                <User />
              </el-icon>
              <span class="desktop-only">添加用户</span>
              <span class="mobile-only">用户</span>
            </el-button>
            <el-button type="success" @click="goToPackages" class="action-btn">
              <el-icon>
                <Box />
              </el-icon>
              <span class="desktop-only">添加绳包</span>
              <span class="mobile-only">绳包</span>
            </el-button>
            <el-button type="warning" @click="goToLogs" class="action-btn">
              <el-icon>
                <Document />
              </el-icon>
              <span class="desktop-only">查看日志</span>
              <span class="mobile-only">日志</span>
            </el-button>
            <el-button type="info" @click="goToStats" class="action-btn">
              <el-icon>
                <DataAnalysis />
              </el-icon>
              <span class="desktop-only">查看统计</span>
              <span class="mobile-only">统计</span>
            </el-button>
          </div>
        </el-card>

        <!-- 热门绳包 -->
        <el-card class="popular-card">
          <template #header>
            <div class="card-header">
              <span>热门绳包</span>
            </div>
          </template>
          <div class="popular-list">
            <div v-if="popularPackages.length === 0" class="empty-state">
              <el-icon :size="48" style="color: #c0c4cc; margin-bottom: 16px;">
                <Box />
              </el-icon>
              <p style="color: #909399; margin: 0;">暂无绳包数据</p>
            </div>
            <div v-else v-for="pkg in popularPackages" :key="pkg.id" class="popular-item">
              <div class="package-info">
                <div class="package-name">{{ pkg.name }}</div>
                <div class="package-author">{{ pkg.author }}</div>
              </div>
              <div class="package-stats">
                <div class="package-downloads">{{ pkg.downloads }} 下载</div>
                <el-tag :type="getTrendType(pkg.downloads)" size="small" class="trend-tag">
                  {{ getTrendText(pkg.downloads) }}
                </el-tag>
              </div>
            </div>
          </div>
        </el-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { getDashboardData } from '../api'
import VChart from 'vue-echarts'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart, BarChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
} from 'echarts/components'
import {
  House,
  User,
  Box,
  Download,
  TrendCharts,
  DataAnalysis,
  Connection,
  Document,
  Folder,
  Setting,
  Refresh
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

// 注册 ECharts 组件
use([
  CanvasRenderer,
  LineChart,
  BarChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
])

const router = useRouter()
const currentTime = ref('')
const userCount = ref(0)
const packageCount = ref(0)
const totalDownloads = ref(0)
const activeUsers = ref(0)
const userGrowth = ref(12)
const packageGrowth = ref(8)
const downloadGrowth = ref(25)
const activeGrowth = ref(15)
const downloadTrends = ref<Array<{date: string, downloads: number}>>([])
const packageDownloadTrends = ref<Array<{name: string, downloads: number}>>([])
const systemStatus = ref({
  database: '正常',
  api_service: '正常',
  log_system: '正常',
  storage: '75%'
})

// 自动更新相关状态
const isLoading = ref(false)
const lastUpdateTime = ref('')
const autoRefreshEnabled = ref(true)
const refreshInterval = ref(30000) // 30秒自动刷新
const updateCount = ref(0)
const showUpdateNotification = ref(false)

// 最近活动数据
const recentActivities = ref<any[]>([])

// 热门绳包数据
const popularPackages = ref<any[]>([])

// 响应式布局相关
const isMobile = ref(false)
const isTablet = ref(false)
const windowWidth = ref(0)
const forceUpdate = ref(0)
let resizeTimeout: number | null = null

// 检测设备类型
const detectDeviceType = () => {
  const width = window.innerWidth
  const oldWidth = windowWidth.value
  windowWidth.value = width
  isMobile.value = width <= 768
  isTablet.value = width > 768 && width <= 1200
  
  // 只在宽度真正变化时记录日志
  if (oldWidth !== width) {
    console.log('设备检测:', { 
      oldWidth, 
      newWidth: width, 
      isMobile: isMobile.value, 
      isTablet: isTablet.value,
      timestamp: new Date().toLocaleTimeString()
    })
  }
}

// 处理窗口大小变化
const handleResize = () => {
  // 清除之前的定时器
  if (resizeTimeout) {
    clearTimeout(resizeTimeout)
  }
  
  // 防抖处理，300ms后执行
  resizeTimeout = window.setTimeout(() => {
    detectDeviceType()
    // 强制重新渲染图表和布局
    forceUpdate.value++
    
    // 延迟执行，确保DOM更新完成
    nextTick(() => {
      // 触发图表重新渲染
      if (packageDownloadTrends.value.length > 0) {
        // 这里可以触发ECharts的resize事件
        console.log('窗口大小变化，触发重新渲染')
      }
    })
  }, 300)
}

// 处理图表大小变化
const handleChartResize = () => {
  console.log('图表大小变化事件触发')
}

// 根据活动类型获取图标组件
const getActivityIcon = (iconName: string) => {
  switch (iconName.toLowerCase()) {
    case 'user':
      return User
    case 'box':
      return Box
    case 'download':
      return Download
    case 'setting':
      return Setting
    default:
      return User
  }
}

// 根据活动类型获取样式类
const getActivityTypeClass = (type: string) => {
  switch (type.toLowerCase()) {
    case 'user':
      return 'user'
    case 'package':
      return 'package'
    case 'download':
      return 'download'
    case 'system':
      return 'system'
    default:
      return 'user'
  }
}

// 图表配置
const chartOption = computed(() => ({
  title: {
    text: '最近7天下载趋势',
    left: 'center',
    textStyle: {
      fontSize: 14,
      fontWeight: 'normal'
    }
  },
  tooltip: {
    trigger: 'axis',
    formatter: '{b}: {c} 次下载'
  },
  xAxis: {
    type: 'category',
    data: downloadTrends.value.map(trend => formatDate(trend.date)),
    axisLabel: {
      rotate: 45
    }
  },
  yAxis: {
    type: 'value',
    name: '下载次数'
  },
  series: [
    {
      name: '下载量',
      type: 'line',
      data: downloadTrends.value.map(trend => trend.downloads),
      smooth: true,
      lineStyle: {
        color: '#409eff',
        width: 3
      },
      itemStyle: {
        color: '#409eff'
      },
      areaStyle: {
        color: {
          type: 'linear',
          x: 0,
          y: 0,
          x2: 0,
          y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(64, 158, 255, 0.3)' },
            { offset: 1, color: 'rgba(64, 158, 255, 0.1)' }
          ]
        }
      }
    }
  ],
  grid: {
    left: '3%',
    right: '4%',
    bottom: '15%',
    top: '15%',
    containLabel: true
  }
}))

// 神包下载趋势图表配置
const packageChartOption = computed(() => ({
  title: {
    text: '神包下载量趋势',
    left: 'center',
    textStyle: {
      fontSize: 14,
      fontWeight: 'normal'
    }
  },
  tooltip: {
    trigger: 'axis',
    formatter: function(params: any) {
      const data = params[0]
      return `${data.name}<br/>下载量: ${data.value}`
    }
  },
  xAxis: {
    type: 'category',
    data: packageDownloadTrends.value.map(item => item.name),
    axisLabel: {
      rotate: 45,
      fontSize: 10
    }
  },
  yAxis: {
    type: 'value',
    name: '下载量'
  },
  series: [
    {
      name: '下载量',
      type: 'line',
      data: packageDownloadTrends.value.map(item => item.downloads),
      smooth: true,
      lineStyle: {
        color: '#409eff',
        width: 3
      },
      itemStyle: {
        color: function(params: any) {
          const val = params.value
          if (val >= 200) return '#ff4757'
          if (val >= 100) return '#ffa502'
          if (val >= 50) return '#2ed573'
          return '#70a1ff'
        }
      },
      areaStyle: {
        color: {
          type: 'linear',
          x: 0,
          y: 0,
          x2: 0,
          y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(64, 158, 255, 0.18)' },
            { offset: 1, color: 'rgba(64, 158, 255, 0.03)' }
          ]
        }
      },
      symbol: 'circle',
      symbolSize: 10,
      emphasis: {
        focus: 'series',
        itemStyle: {
          borderWidth: 3,
          borderColor: '#fff',
          shadowBlur: 8,
          shadowColor: 'rgba(64,158,255,0.3)'
        }
      },
      label: {
        show: true,
        position: 'top',
        color: '#409eff',
        fontWeight: 'bold',
        fontSize: 12
      }
    }
  ],
  grid: {
    left: '3%',
    right: '4%',
    bottom: '15%',
    top: '15%',
    containLabel: true
  }
}))

// 根据下载量获取柱状图颜色
const getBarColor = (downloads: number): string => {
  if (downloads >= 200) return '#ff4757' // 红色 - 热门
  if (downloads >= 100) return '#ffa502' // 橙色 - 流行
  if (downloads >= 50) return '#2ed573' // 绿色 - 正常
  return '#70a1ff' // 蓝色 - 较低
}

// 获取趋势类型
const getTrendType = (downloads: number): string => {
  if (downloads >= 200) return 'danger' // 热门
  if (downloads >= 100) return 'warning' // 流行
  if (downloads >= 50) return 'success' // 正常
  return 'info' // 较低
}

// 获取趋势文本
const getTrendText = (downloads: number): string => {
  if (downloads >= 200) return '热门'
  if (downloads >= 100) return '流行'
  if (downloads >= 50) return '正常'
  return '较低'
}

let timeInterval: number
let dataRefreshInterval: number

// 更新时间
const updateTime = () => {
  const now = new Date()
  currentTime.value = now.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 格式化日期
const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
}

// 更新最后刷新时间
const updateLastRefreshTime = () => {
  const now = new Date()
  lastUpdateTime.value = now.toLocaleString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 加载数据
const loadData = async () => {
  if (isLoading.value) return // 防止重复请求
  
  try {
    isLoading.value = true
    console.log('开始加载仪表盘数据...')
    
    const dashboardRes = await getDashboardData()
    console.log('仪表盘API响应:', dashboardRes)
    console.log('响应数据结构:', JSON.stringify(dashboardRes, null, 2))
    
    if (dashboardRes.code === 0 && dashboardRes.data) {
      const data = dashboardRes.data
      
      // 检查数据是否有变化
      const hasChanges = checkDataChanges(data)
      
      // 更新统计数据
      userCount.value = data.user_count || 0
      packageCount.value = data.package_count || 0
      totalDownloads.value = data.total_downloads || 0
      activeUsers.value = data.active_users || 0
      
      console.log('统计数据:', {
        userCount: userCount.value,
        packageCount: packageCount.value,
        totalDownloads: totalDownloads.value,
        activeUsers: activeUsers.value
      })
      
      // 更新下载趋势数据
      if (data.download_trends && Array.isArray(data.download_trends)) {
        downloadTrends.value = data.download_trends.map((trend: any) => ({
          date: trend.date,
          downloads: trend.downloads
        }))
        console.log('下载趋势数据:', downloadTrends.value)
      } else {
        console.log('没有下载趋势数据')
        downloadTrends.value = []
      }
      
      // 更新最近活动
      if (data.recent_activities) {
        recentActivities.value = data.recent_activities.map((activity: any) => ({
          id: activity.id,
          type: activity.type_, // 后端字段名是 type_
          icon: activity.icon,
          title: activity.title,
          time: activity.time
        }))
        console.log('最近活动数据:', recentActivities.value)
      } else {
        console.log('没有最近活动数据')
        recentActivities.value = []
      }
      
      // 更新热门绳包
      if (data.popular_packages) {
        popularPackages.value = data.popular_packages.map((pkg: any) => ({
          id: pkg.id,
          name: pkg.name,
          author: pkg.author,
          downloads: pkg.downloads
        }))
        
        // 更新神包下载趋势数据
        packageDownloadTrends.value = data.popular_packages
          .map((pkg: any) => ({
            name: pkg.name,
            downloads: pkg.downloads
          }))
          .sort((a: any, b: any) => b.downloads - a.downloads) // 按下载量降序排列
        
        console.log('热门绳包数据:', popularPackages.value)
        console.log('绳包下载趋势数据:', packageDownloadTrends.value)
      } else {
        console.log('没有热门绳包数据')
        popularPackages.value = []
        packageDownloadTrends.value = []
      }
      
      // 更新系统状态
      if (data.system_status) {
        systemStatus.value = data.system_status
        console.log('系统状态数据:', systemStatus.value)
      } else {
        console.log('没有系统状态数据')
      }
      
      // 更新最后刷新时间
      updateLastRefreshTime()
      
      // 更新计数器
      updateCount.value++
      
      console.log('数据加载完成，更新次数:', updateCount.value)
      
      // 如果有数据变化，显示通知
      if (hasChanges && updateCount.value > 1) {
        showUpdateNotification.value = true
        setTimeout(() => {
          showUpdateNotification.value = false
        }, 3000)
      }
    } else {
      console.error('仪表盘API返回错误:', dashboardRes)
    }
  } catch (error: any) {
    console.error('加载数据失败:', error)
    console.error('错误类型:', typeof error)
    console.error('错误详情:', {
      name: error?.name,
      message: error?.message,
      stack: error?.stack
    })
    
    // 检查是否是网络错误或服务不可用
    const errorMessage = error instanceof Error ? error.message : String(error)
    console.log('错误消息:', errorMessage)
    
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED') ||
        errorMessage.includes('Service unavailable')) {
      
      // 显示服务异常提示
      ElMessage.error('服务异常已安全退出！')
      
      // 延迟跳转到登录页面
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      
      return
    }
    
    // 其他错误显示通用提示
    ElMessage.error('数据加载失败，请稍后重试')
  } finally {
    isLoading.value = false
  }
}

// 检查数据变化
const checkDataChanges = (newData: any) => {
  // 简单的数据变化检查
  return newData.user_count !== userCount.value ||
         newData.package_count !== packageCount.value ||
         newData.total_downloads !== totalDownloads.value
}

// 刷新数据
const refreshData = () => {
  loadData()
}

// 切换自动刷新
const toggleAutoRefresh = () => {
  console.log('toggleAutoRefresh被调用，当前状态:', autoRefreshEnabled.value)
  console.log('自动刷新状态:', autoRefreshEnabled.value ? '启用' : '禁用')
  
  if (autoRefreshEnabled.value) {
    console.log('启动自动刷新')
    startAutoRefresh()
  } else {
    console.log('停止自动刷新')
    stopAutoRefresh()
  }
}

// 开始自动刷新
const startAutoRefresh = () => {
  stopAutoRefresh() // 先停止之前的定时器
  if (autoRefreshEnabled.value) {
    dataRefreshInterval = setInterval(() => {
      if (autoRefreshEnabled.value && !isLoading.value) {
        loadData()
      }
    }, refreshInterval.value)
  }
}

// 停止自动刷新
const stopAutoRefresh = () => {
  if (dataRefreshInterval) {
    clearInterval(dataRefreshInterval)
    dataRefreshInterval = 0
  }
}

// 页面跳转
const goToUsers = () => router.push('/users')
const goToPackages = () => router.push('/packages')
const goToLogs = () => router.push('/logs')
const goToStats = () => router.push('/stats')

onMounted(() => {
  console.log('仪表盘组件挂载，开始初始化...')
  
  // 初始化设备检测
  detectDeviceType()
  
  // 添加窗口大小变化监听
  window.addEventListener('resize', handleResize)
  
  loadData()
  updateTime()
  updateLastRefreshTime()
  
  // 启动时间更新
  timeInterval = setInterval(updateTime, 1000)
  
  // 启动自动刷新
  if (autoRefreshEnabled.value) {
    startAutoRefresh()
  }
})

onUnmounted(() => {
  // 移除窗口大小变化监听
  window.removeEventListener('resize', handleResize)
  
  // 清理定时器
  if (resizeTimeout) {
    clearTimeout(resizeTimeout)
  }
  
  if (timeInterval) {
    clearInterval(timeInterval)
  }
  stopAutoRefresh()
})
</script>

<style scoped>
.dashboard {
  padding: 0;
}

.welcome-card {
  margin-bottom: 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: var(--text-primary);
  border: none;
  border-radius: 1rem;
}

.welcome-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.welcome-text h2 {
  margin: 0 0 0.5rem 0;
  font-size: 1.75rem;
  font-weight: bold;
}

.welcome {
  margin: 0 0 0.5rem 0;
  font-size: 1rem;
  opacity: 0.9;
}

.time {
  margin: 0;
  font-size: 0.875rem;
  opacity: 0.8;
}

.welcome-icon {
  font-size: 3.5rem;
  opacity: 0.4;
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.welcome-card:hover .welcome-icon {
  transform: scale(1.1);
  opacity: 0.6;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(15.625rem, 1fr));
  gap: 1.25rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  border-radius: 0.75rem;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-0.125rem);
  box-shadow: 0 0.5rem 1.5625rem rgba(0, 0, 0, 0.1);
}

.stat-content {
  display: flex;
  align-items: center;
  padding: 0.5rem 0;
}

.stat-icon {
  width: 3.75rem;
  height: 3.75rem;
  border-radius: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.625rem;
  color: #fff;
  margin-right: 1rem;
  box-shadow: 0 0.25rem 0.75rem rgba(0, 0, 0, 0.15);
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.stat-card:hover .stat-icon {
  transform: scale(1.05);
  box-shadow: 0 0.375rem 1.25rem rgba(0, 0, 0, 0.2);
}

.user-icon { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
.package-icon { background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); }
.download-icon { background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); }
.active-icon { background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); }

.stat-info {
  flex: 1;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.875rem;
  margin-bottom: 0.25rem;
}

.stat-value {
  font-size: 1.75rem;
  font-weight: bold;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.stat-change {
  font-size: 0.75rem;
  font-weight: 500;
}

.stat-change.positive {
  color: #67c23a;
}

.stat-change.negative {
  color: #f56c6c;
}

.dashboard-content {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 1.5rem;
}

.left-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.right-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.chart-card, .activity-card, .status-card, .quick-actions-card, .popular-card {
  border-radius: 0.75rem;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: bold;
  font-size: 1rem;
}

.header-left {
  display: flex;
  align-items: center;
}

.update-info {
  display: flex;
  align-items: center;
  margin-left: 0.9375rem;
  color: var(--text-secondary);
  font-size: 0.75rem;
}

.header-right {
  display: flex;
  align-items: center;
}

.chart-container {
  height: 18.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.chart {
  width: 100%;
  height: 100%;
}

.chart-placeholder {
  text-align: center;
  color: var(--text-secondary);
}

.chart-placeholder i {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

/* 加载状态样式 */
.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

/* 数据更新动画 */
.data-update {
  animation: dataUpdate 0.5s ease-in-out;
}

@keyframes dataUpdate {
  0% {
    opacity: 0.7;
    transform: scale(0.98);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}

.activity-list {
  max-height: 18.75rem;
  overflow-y: auto;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2.5rem 1.25rem;
  text-align: center;
}

.activity-item {
  display: flex;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--border-color);
}

.activity-item:last-child {
  border-bottom: none;
}

.activity-icon {
  width: 2.75rem;
  height: 2.75rem;
  border-radius: 0.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 0.75rem;
  font-size: 1.125rem;
  color: #fff;
  box-shadow: 0 0.125rem 0.5rem rgba(0, 0, 0, 0.1);
  transition: transform 0.2s ease;
}

.activity-item:hover .activity-icon {
  transform: scale(1.1);
}

.activity-icon.user { background: linear-gradient(135deg, #409eff 0%, #36a3f7 100%); }
.activity-icon.package { background: linear-gradient(135deg, #67c23a 0%, #5daf32 100%); }
.activity-icon.download { background: linear-gradient(135deg, #e6a23c 0%, #d49426 100%); }
.activity-icon.system { background: linear-gradient(135deg, #f56c6c 0%, #e74c3c 100%); }

.activity-content {
  flex: 1;
}

.activity-title {
  font-weight: 500;
  margin-bottom: 0.25rem;
  font-size: 0.875rem;
}

.activity-time {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.status-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
}

.status-label {
  font-weight: 500;
  font-size: 0.875rem;
}

.quick-actions {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.quick-actions .el-button {
  width: auto;
  justify-content: center;
  margin: 0 auto;
  padding: 0.75rem 1.5rem;
  font-size: 0.875rem;
}

.popular-list {
  max-height: 12.5rem;
  overflow-y: auto;
}

.popular-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--border-color);
}

.popular-item:last-child {
  border-bottom: none;
}

.package-name {
  font-weight: 500;
  margin-bottom: 0.125rem;
  font-size: 0.875rem;
}

.package-author {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.package-stats {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.25rem;
}

.package-downloads {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.trend-tag {
  font-size: 0.625rem;
  padding: 0.125rem 0.375rem;
}

.auto-refresh-status {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1.25rem;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 0.5rem;
  margin-bottom: 1.5rem;
}

.status-info {
  display: flex;
  align-items: center;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.status-info .el-icon {
  margin-right: 0.5rem;
  font-size: 1.125rem;
  color: #409eff;
  animation: rotating 2s linear infinite;
}

.status-info .el-icon.rotating {
  animation: rotating 2s linear infinite;
}

@keyframes rotating {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.status-actions {
  display: flex;
  align-items: center;
}

.update-notification {
  position: fixed;
  top: 0.625rem;
  right: 0.625rem;
  z-index: 1000;
  width: 18.75rem;
  box-shadow: 0 0.25rem 0.75rem rgba(0, 0, 0, 0.1);
}

/* 响应式适配 */

/* 平板适配 (768px - 1200px) */
@media (max-width: 1200px) {
  .dashboard-content {
    grid-template-columns: 1fr;
  }
  
  .stats-cards {
    grid-template-columns: repeat(auto-fit, minmax(12.5rem, 1fr));
  }
  
  .chart-container {
    height: 16rem;
  }
  
  .activity-list {
    max-height: 16rem;
  }
  
  .popular-list {
    max-height: 12rem;
  }
}

/* 手机端适配 (≤768px) */
@media (max-width: 768px) {
  .dashboard {
    padding: 0.5rem;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
  }
  
  /* 欢迎卡片优化 */
  .welcome-card {
    margin-bottom: 1rem;
    border-radius: 0.75rem;
    padding: 1rem;
  }
  
  .welcome-content {
    flex-direction: column;
    text-align: center;
    gap: 1rem;
  }
  
  .welcome-text h2 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }
  
  .welcome {
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
  }
  
  .time {
    font-size: 0.75rem;
  }
  
  .welcome-icon {
    font-size: 2.5rem;
    margin-top: 0.5rem;
  }
  
  /* 统计卡片优化 */
  .stats-cards {
    grid-template-columns: 1fr;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  
  .stat-card {
    border-radius: 0.75rem;
    padding: 1rem;
    transition: all 0.2s ease;
    touch-action: manipulation;
  }
  
  .stat-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }
  
  .stat-card:active {
    transform: translateY(0);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }
  
  .stat-content {
    padding: 0.75rem 0;
  }
  
  .stat-icon {
    width: 2.75rem;
    height: 2.75rem;
    font-size: 1.125rem;
    margin-right: 0.75rem;
    border-radius: 0.75rem;
  }
  
  .stat-value {
    font-size: 1.5rem;
    line-height: 1.2;
  }
  
  .stat-label {
    font-size: 0.8rem;
  }
  
  .stat-change {
    font-size: 0.7rem;
  }
  
  /* 自动刷新状态栏优化 */
  .auto-refresh-status {
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 0.75rem;
    margin-bottom: 1rem;
  }
  
  .status-info {
    justify-content: center;
    font-size: 0.8rem;
  }
  
  .status-info .el-icon {
    font-size: 1rem;
    margin-right: 0.5rem;
  }
  
  .status-actions {
    justify-content: center;
  }
  
  .status-actions .el-button {
    width: 100%;
    max-width: 12rem;
  }
  
  /* 更新通知优化 */
  .update-notification {
    width: calc(100vw - 1rem);
    right: 0.5rem;
    left: 0.5rem;
    top: 0.5rem;
    border-radius: 0.75rem;
  }
  
  /* 主要内容区域优化 */
  .dashboard-content {
    grid-template-columns: 1fr;
    gap: 0.75rem;
  }
  
  .left-content, .right-content {
    gap: 0.75rem;
  }
  
  /* 图表容器优化 */
  .chart-container {
    height: 14rem;
    border-radius: 0.75rem;
  }
  
  .chart-placeholder {
    padding: 2rem 1rem;
  }
  
  .chart-placeholder .el-icon {
    font-size: 2.5rem;
    margin-bottom: 0.75rem;
  }
  
  .chart-placeholder p {
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
  }
  
  .chart-placeholder small {
    font-size: 0.75rem;
  }
  
  /* 卡片头部优化 */
  .card-header {
    font-size: 0.875rem;
    padding: 0.75rem 1rem;
  }
  
  .header-left {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .update-info {
    display: none;
  }
  
  .header-right .el-button {
    font-size: 0.8rem;
    padding: 0.5rem 0.75rem;
  }
  
  /* 活动列表优化 */
  .activity-list {
    max-height: 14rem;
    padding: 0 0.5rem;
    -webkit-overflow-scrolling: touch;
    scrollbar-width: thin;
    scrollbar-color: rgba(0, 0, 0, 0.2) transparent;
  }
  
  .activity-list::-webkit-scrollbar {
    width: 4px;
  }
  
  .activity-list::-webkit-scrollbar-track {
    background: transparent;
  }
  
  .activity-list::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 2px;
  }
  
  .activity-item {
    padding: 0.75rem 0;
    border-bottom: 1px solid var(--border-color);
    transition: all 0.2s ease;
    touch-action: manipulation;
    cursor: pointer;
  }
  
  .activity-item:hover {
    background-color: rgba(64, 158, 255, 0.05);
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    margin: 0 -0.5rem;
    border-radius: 0.5rem;
  }
  
  .activity-item:active {
    background-color: rgba(64, 158, 255, 0.1);
  }
  
  .activity-item:last-child {
    border-bottom: none;
  }
  
  .activity-icon {
    width: 2.25rem;
    height: 2.25rem;
    font-size: 0.875rem;
    margin-right: 0.75rem;
    border-radius: 0.5rem;
  }
  
  .activity-title {
    font-size: 0.8rem;
    line-height: 1.3;
  }
  
  .activity-time {
    font-size: 0.7rem;
    margin-top: 0.25rem;
  }
  
  /* 系统状态优化 */
  .status-list {
    gap: 0.5rem;
    padding: 0 0.5rem;
  }
  
  .status-item {
    padding: 0.75rem 0;
    border-bottom: 1px solid var(--border-color);
  }
  
  .status-item:last-child {
    border-bottom: none;
  }
  
  .status-label {
    font-size: 0.8rem;
    line-height: 1.3;
  }
  
  .status-label .el-icon {
    font-size: 0.875rem;
    margin-right: 0.5rem;
  }
  
  /* 快速操作优化 */
  .quick-actions {
    gap: 0.5rem;
    padding: 0 0.5rem;
  }
  
  .quick-actions .el-button {
    width: 100%;
    padding: 0.875rem 1rem;
    font-size: 0.875rem;
    height: 2.75rem;
    border-radius: 0.5rem;
    justify-content: center;
    transition: all 0.2s ease;
    touch-action: manipulation;
  }
  
  .quick-actions .el-button:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }
  
  .quick-actions .el-button:active {
    transform: translateY(0);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  }
  
  .quick-actions .el-button .el-icon {
    margin-right: 0.5rem;
    font-size: 1rem;
  }
  
  /* 热门绳包优化 */
  .popular-list {
    max-height: 12rem;
    padding: 0 0.5rem;
    -webkit-overflow-scrolling: touch;
    scrollbar-width: thin;
    scrollbar-color: rgba(0, 0, 0, 0.2) transparent;
  }
  
  .popular-list::-webkit-scrollbar {
    width: 4px;
  }
  
  .popular-list::-webkit-scrollbar-track {
    background: transparent;
  }
  
  .popular-list::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 2px;
  }
  
  .popular-item {
    padding: 0.75rem 0;
    border-bottom: 1px solid var(--border-color);
    transition: all 0.2s ease;
    touch-action: manipulation;
    cursor: pointer;
  }
  
  .popular-item:hover {
    background-color: rgba(103, 194, 58, 0.05);
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    margin: 0 -0.5rem;
    border-radius: 0.5rem;
  }
  
  .popular-item:active {
    background-color: rgba(103, 194, 58, 0.1);
  }
  
  .popular-item:last-child {
    border-bottom: none;
  }
  
  .package-name {
    font-size: 0.8rem;
    line-height: 1.3;
    margin-bottom: 0.25rem;
  }
  
  .package-author {
    font-size: 0.7rem;
    line-height: 1.2;
  }
  
  .package-downloads {
    font-size: 0.7rem;
    margin-bottom: 0.25rem;
  }
  
  .trend-tag {
    font-size: 0.6rem;
    padding: 0.125rem 0.375rem;
  }
  
  /* 空状态优化 */
  .empty-state {
    padding: 2rem 1rem;
  }
  
  .empty-state .el-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }
  
  .empty-state p {
    font-size: 0.875rem;
    margin: 0;
  }
}

/* 桌面端和移动端显示控制 */
.desktop-only {
  display: inline;
}

.mobile-only {
  display: none;
}

/* 强制响应式重新渲染 */
.dashboard {
  will-change: transform;
  transform: translateZ(0);
  min-height: 100vh;
  box-sizing: border-box;
}

/* 确保所有容器都能正确响应 */
.dashboard * {
  box-sizing: border-box;
}

/* 响应式过渡效果 */
.stats-cards,
.dashboard-content,
.chart-container {
  transition: all 0.3s ease;
}

/* 确保网格布局在窗口大小变化时正确重新计算 */
.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(15.625rem, 1fr));
  gap: 1.25rem;
  margin-bottom: 1.5rem;
}

.dashboard-content {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 1.5rem;
}

/* 确保图表容器响应式 */
.chart-container {
  position: relative;
  overflow: hidden;
}

.chart {
  width: 100% !important;
  height: 100% !important;
}

/* 超小屏幕适配 (≤480px) */
@media (max-width: 480px) {
  .desktop-only {
    display: none;
  }
  
  .mobile-only {
    display: inline;
  }
  
  .dashboard {
    padding: 0.25rem;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
  }
  
  .welcome-card {
    padding: 0.75rem;
  }
  
  .welcome-text h2 {
    font-size: 1.25rem;
  }
  
  .welcome {
    font-size: 0.8rem;
  }
  
  .time {
    font-size: 0.7rem;
  }
  
  .welcome-icon {
    font-size: 2rem;
  }
  
  .stats-cards {
    gap: 0.5rem;
  }
  
  .stat-card {
    padding: 0.75rem;
  }
  
  .stat-content {
    padding: 0.5rem 0;
  }
  
  .stat-icon {
    width: 2.5rem;
    height: 2.5rem;
    font-size: 1rem;
    margin-right: 0.5rem;
  }
  
  .stat-value {
    font-size: 1.25rem;
  }
  
  .stat-label {
    font-size: 0.75rem;
  }
  
  .stat-change {
    font-size: 0.65rem;
  }
  
  .auto-refresh-status {
    padding: 0.75rem;
    gap: 0.5rem;
  }
  
  .status-info {
    font-size: 0.75rem;
  }
  
  .chart-container {
    height: 12rem;
  }
  
  .activity-list {
    max-height: 12rem;
    -webkit-overflow-scrolling: touch;
  }
  
  .popular-list {
    max-height: 10rem;
    -webkit-overflow-scrolling: touch;
  }
  
  .quick-actions .el-button {
    padding: 0.75rem;
    font-size: 0.8rem;
    height: 2.5rem;
  }
  
  .card-header {
    font-size: 0.8rem;
    padding: 0.5rem 0.75rem;
  }
  
  .activity-item {
    padding: 0.5rem 0;
  }
  
  .activity-icon {
    width: 2rem;
    height: 2rem;
    font-size: 0.75rem;
    margin-right: 0.5rem;
  }
  
  .activity-title {
    font-size: 0.75rem;
  }
  
  .activity-time {
    font-size: 0.65rem;
  }
  
  .status-item {
    padding: 0.5rem 0;
  }
  
  .status-label {
    font-size: 0.75rem;
  }
  
  .popular-item {
    padding: 0.5rem 0;
  }
  
  .package-name {
    font-size: 0.75rem;
  }
  
  .package-author {
    font-size: 0.65rem;
  }
  
  .package-downloads {
    font-size: 0.65rem;
  }
  
  .trend-tag {
    font-size: 0.55rem;
    padding: 0.1rem 0.25rem;
  }
  
  .empty-state {
    padding: 1.5rem 0.75rem;
  }
  
  .empty-state .el-icon {
    font-size: 2rem;
    margin-bottom: 0.75rem;
  }
  
  .empty-state p {
    font-size: 0.8rem;
  }
}

/* 移动端显示控制 (≤768px) */
@media (max-width: 768px) {
  .desktop-only {
    display: none !important;
  }
  
  .mobile-only {
    display: inline !important;
  }
}

/* 确保响应式布局立即生效 */
@media (max-width: 1200px) {
  .dashboard-content {
    grid-template-columns: 1fr !important;
  }
}

@media (max-width: 768px) {
  .stats-cards {
    grid-template-columns: 1fr !important;
  }
}
</style> 