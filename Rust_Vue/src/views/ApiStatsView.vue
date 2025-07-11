<template>
  <div class="api-stats-view">
    <el-card>
      <div class="stats-header">
        <h2>API调用统计</h2>
        <div class="header-controls">
          <el-select v-model="timeRange" placeholder="时间范围" @change="loadApiStats" style="width: 120px; margin-right: 12px;">
            <el-option label="全部" value="all" />
            <el-option label="今日" value="today" />
            <el-option label="本周" value="week" />
            <el-option label="本月" value="month" />
          </el-select>
          <el-button @click="loadApiStats" :loading="loading">刷新</el-button>
        </div>
      </div>

      <!-- 总体统计卡片 -->
      <div class="overview-cards">
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon api-icon">
              <el-icon :size="24"><Connection /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-label">总调用次数</div>
              <div class="stat-value">{{ apiStats.total_calls || 0 }}</div>
            </div>
          </div>
        </el-card>
        
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon success-icon">
              <el-icon :size="24"><CircleCheck /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-label">成功率</div>
              <div class="stat-value">{{ (apiStats.success_rate || 0).toFixed(1) }}%</div>
            </div>
          </div>
        </el-card>
        
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon time-icon">
              <el-icon :size="24"><Clock /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-label">平均响应时间</div>
              <div class="stat-value">{{ apiStats.avg_response_time_ms || 0 }}ms</div>
            </div>
          </div>
        </el-card>
        
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon user-icon">
              <el-icon :size="24"><User /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-label">活跃用户</div>
              <div class="stat-value">{{ apiStats.performance_summary?.total_users || 0 }}</div>
            </div>
          </div>
        </el-card>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="loading-container">
        <el-card>
          <div class="loading-content">
            <el-icon class="is-loading" :size="32">
              <Loading />
            </el-icon>
            <p>正在加载API调用统计数据...</p>
          </div>
        </el-card>
      </div>

      <div v-if="!loading && apiStats" class="stats-content">
        <!-- 热门API统计 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>热门API接口</h3>
            <el-button size="small" @click="exportTopApis">导出</el-button>
          </div>
          <el-table :data="apiStats.top_apis || []" style="width: 100%; margin-top: 8px;" size="small">
            <el-table-column prop="api_name" label="接口名称" min-width="200" />
            <el-table-column prop="call_count" label="调用次数" width="120">
              <template #default="scope">
                <el-tag :type="getApiTagType(scope.row.call_count)" size="small">
                  {{ scope.row.call_count }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="success_rate" label="成功率" width="100">
              <template #default="scope">
                <el-progress 
                  :percentage="scope.row.success_rate" 
                  :color="getSuccessRateColor(scope.row.success_rate)"
                  :show-text="false"
                  style="width: 80px;"
                />
                <span style="margin-left: 8px;">{{ scope.row.success_rate.toFixed(1) }}%</span>
              </template>
            </el-table-column>
            <el-table-column prop="avg_response_time_ms" label="平均响应时间" width="150">
              <template #default="scope">
                <span :class="getResponseTimeClass(scope.row.avg_response_time_ms)">
                  {{ scope.row.avg_response_time_ms }}ms
                </span>
              </template>
            </el-table-column>
          </el-table>
        </div>

        <!-- 最近调用记录 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>最近调用记录</h3>
            <el-button size="small" @click="loadRecentCalls">刷新</el-button>
          </div>
          <el-table :data="recentCalls" style="width: 100%; margin-top: 8px;" size="small">
            <el-table-column prop="timestamp" label="时间" width="180">
              <template #default="scope">
                {{ formatTimestamp(scope.row.timestamp) }}
              </template>
            </el-table-column>
            <el-table-column prop="api_name" label="接口名称" min-width="200" />
            <el-table-column prop="username" label="用户" width="120" />
            <el-table-column prop="response_time_ms" label="响应时间" width="120">
              <template #default="scope">
                <span :class="getResponseTimeClass(scope.row.response_time_ms)">
                  {{ scope.row.response_time_ms }}ms
                </span>
              </template>
            </el-table-column>
            <el-table-column prop="status_code" label="状态码" width="100">
              <template #default="scope">
                <el-tag :type="getStatusTagType(scope.row.status_code)" size="small">
                  {{ scope.row.status_code }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="success" label="状态" width="80">
              <template #default="scope">
                <el-tag :type="scope.row.success ? 'success' : 'danger'" size="small">
                  {{ scope.row.success ? '成功' : '失败' }}
                </el-tag>
              </template>
            </el-table-column>
          </el-table>
        </div>

        <!-- 用户活动统计 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>用户活动统计</h3>
            <el-button size="small" @click="exportUserActivity">导出</el-button>
          </div>
          <el-table :data="apiStats.user_activity || []" style="width: 100%; margin-top: 8px;" size="small">
            <el-table-column prop="username" label="用户名" min-width="150" />
            <el-table-column prop="call_count" label="调用次数" width="120">
              <template #default="scope">
                <el-tag :type="getUserTagType(scope.row.call_count)" size="small">
                  {{ scope.row.call_count }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="last_activity" label="最后活动" width="180">
              <template #default="scope">
                {{ formatTimestamp(scope.row.last_activity) }}
              </template>
            </el-table-column>
            <el-table-column prop="favorite_apis" label="常用接口" min-width="200">
              <template #default="scope">
                <el-tag 
                  v-for="api in scope.row.favorite_apis" 
                  :key="api" 
                  size="small" 
                  style="margin-right: 4px; margin-bottom: 4px;"
                >
                  {{ api }}
                </el-tag>
              </template>
            </el-table-column>
          </el-table>
        </div>

        <!-- API性能详情 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>API性能详情</h3>
            <el-button size="small" @click="loadApiPerformance">刷新</el-button>
          </div>
          <el-table :data="apiPerformanceArr" style="width: 100%; margin-top: 8px;" size="small">
            <el-table-column prop="api_name" label="接口名称" min-width="200" />
            <el-table-column prop="total_calls" label="总调用" width="100" />
            <el-table-column prop="success_calls" label="成功" width="100">
              <template #default="scope">
                <span class="success-text">{{ scope.row.success_calls }}</span>
              </template>
            </el-table-column>
            <el-table-column prop="failed_calls" label="失败" width="100">
              <template #default="scope">
                <span class="error-text">{{ scope.row.failed_calls }}</span>
              </template>
            </el-table-column>
            <el-table-column prop="success_rate" label="成功率" width="100">
              <template #default="scope">
                <el-progress 
                  :percentage="scope.row.success_rate" 
                  :color="getSuccessRateColor(scope.row.success_rate)"
                  :show-text="false"
                  style="width: 80px;"
                />
                <span style="margin-left: 8px;">{{ scope.row.success_rate.toFixed(1) }}%</span>
              </template>
            </el-table-column>
            <el-table-column prop="avg_response_time_ms" label="平均响应时间" width="150">
              <template #default="scope">
                <span :class="getResponseTimeClass(scope.row.avg_response_time_ms)">
                  {{ scope.row.avg_response_time_ms }}ms
                </span>
              </template>
            </el-table-column>
            <el-table-column prop="unique_users_count" label="用户数" width="100" />
          </el-table>
        </div>

        <!-- 实时监控 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>实时监控</h3>
            <el-switch v-model="autoRefresh" active-text="自动刷新" />
          </div>
          <div class="realtime-stats">
            <div class="realtime-item">
              <span class="label">系统运行时间：</span>
              <span class="value">{{ apiStats.performance_summary?.system_uptime_hours || 0 }}小时</span>
            </div>
            <div class="realtime-item">
              <span class="label">峰值并发用户：</span>
              <span class="value">{{ apiStats.performance_summary?.peak_concurrent_users || 0 }}</span>
            </div>
            <div class="realtime-item">
              <span class="label">最后更新：</span>
              <span class="value">{{ lastUpdateTime }}</span>
            </div>
            <div class="realtime-item">
              <span class="label">数据状态：</span>
              <el-tag :type="dataStatus === 'normal' ? 'success' : 'warning'" size="small">
                {{ dataStatus === 'normal' ? '正常' : '异常' }}
              </el-tag>
            </div>
          </div>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { getApiCallStats, getApiPerformance, getRecentCalls } from '../api'
import { ElMessage } from 'element-plus'
import { Connection, CircleCheck, Clock, User, Loading } from '@element-plus/icons-vue'

const apiStats = ref<any>(null)
const apiPerformance = ref<any>(null)
const recentCalls = ref<any[]>([])
const loading = ref(false)
const timeRange = ref('all')
const autoRefresh = ref(false)
const lastUpdateTime = ref('')
const dataStatus = ref('normal')
let refreshTimer: number | null = null

// 初始化默认值
const defaultApiStats = {
  total_calls: 0,
  success_rate: 0,
  avg_response_time_ms: 0,
  top_apis: [],
  recent_calls: [],
  user_activity: [],
  performance_summary: {
    total_apis: 0,
    total_users: 0,
    system_uptime_hours: 0,
    peak_concurrent_users: 0
  }
}

// 计算属性
const apiPerformanceArr = computed(() => {
  if (!apiPerformance.value?.data) return []
  return Object.entries(apiPerformance.value.data).map(([api_name, data]: [string, any]) => ({
    api_name,
    ...data
  }))
})

// 加载API调用统计
const loadApiStats = async () => {
  try {
    loading.value = true
    const response = await getApiCallStats()
    if (response.code === 0) {
      apiStats.value = response.data || defaultApiStats
      lastUpdateTime.value = new Date().toLocaleString()
      dataStatus.value = 'normal'
      ElMessage.success('API调用统计加载成功')
    } else {
      apiStats.value = defaultApiStats
      ElMessage.error(response.msg || '加载失败')
    }
  } catch (error) {
    console.error('加载API调用统计失败:', error)
    apiStats.value = defaultApiStats
    ElMessage.error('加载失败')
    dataStatus.value = 'error'
  } finally {
    loading.value = false
  }
}

// 加载API性能详情
const loadApiPerformance = async () => {
  try {
    const response = await getApiPerformance()
    if (response.code === 0) {
      apiPerformance.value = response
    }
  } catch (error) {
    console.error('加载API性能详情失败:', error)
  }
}

// 加载最近调用记录
const loadRecentCalls = async () => {
  try {
    const response = await getRecentCalls()
    if (response.code === 0) {
      recentCalls.value = response.data || []
    }
  } catch (error) {
    console.error('加载最近调用记录失败:', error)
  }
}

// 格式化时间戳
const formatTimestamp = (timestamp: number) => {
  return new Date(timestamp * 1000).toLocaleString()
}

// 获取API标签类型
const getApiTagType = (count: number) => {
  if (count > 100) return 'danger'
  if (count > 50) return 'warning'
  if (count > 10) return 'success'
  return 'info'
}

// 获取用户标签类型
const getUserTagType = (count: number) => {
  if (count > 50) return 'danger'
  if (count > 20) return 'warning'
  if (count > 5) return 'success'
  return 'info'
}

// 获取状态标签类型
const getStatusTagType = (statusCode: number) => {
  if (statusCode >= 200 && statusCode < 300) return 'success'
  if (statusCode >= 400 && statusCode < 500) return 'warning'
  if (statusCode >= 500) return 'danger'
  return 'info'
}

// 获取响应时间样式
const getResponseTimeClass = (time: number) => {
  if (time > 1000) return 'slow-response'
  if (time > 500) return 'medium-response'
  return 'fast-response'
}

// 获取成功率颜色
const getSuccessRateColor = (rate: number) => {
  if (rate >= 95) return '#67C23A'
  if (rate >= 80) return '#E6A23C'
  return '#F56C6C'
}

// 导出功能
const exportTopApis = () => {
  if (!apiStats.value?.top_apis) return
  const data = apiStats.value.top_apis.map((api: any) => ({
    '接口名称': api.api_name,
    '调用次数': api.call_count,
    '成功率': `${api.success_rate.toFixed(1)}%`,
    '平均响应时间': `${api.avg_response_time_ms}ms`
  }))
  downloadCSV(data, '热门API统计')
}

const exportUserActivity = () => {
  if (!apiStats.value?.user_activity) return
  const data = apiStats.value.user_activity.map((user: any) => ({
    '用户名': user.username,
    '调用次数': user.call_count,
    '最后活动': formatTimestamp(user.last_activity),
    '常用接口': user.favorite_apis.join(', ')
  }))
  downloadCSV(data, '用户活动统计')
}

// 下载CSV文件
const downloadCSV = (data: any[], filename: string) => {
  if (data.length === 0) {
    ElMessage.warning('没有数据可导出')
    return
  }

  const headers = Object.keys(data[0])
  const csvContent = [
    headers.join(','),
    ...data.map(row => headers.map(header => `"${row[header]}"`).join(','))
  ].join('\n')

  const blob = new Blob(['\ufeff' + csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  const url = URL.createObjectURL(blob)
  link.setAttribute('href', url)
  link.setAttribute('download', `${filename}_${new Date().toISOString().slice(0, 10)}.csv`)
  link.style.visibility = 'hidden'
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}

// 自动刷新
watch(autoRefresh, (newVal) => {
  if (newVal) {
    refreshTimer = window.setInterval(() => {
      loadApiStats()
      loadRecentCalls()
    }, 30000) // 30秒刷新一次
  } else if (refreshTimer) {
    clearInterval(refreshTimer)
    refreshTimer = null
  }
})

// 生命周期
onMounted(() => {
  loadApiStats()
  loadApiPerformance()
  loadRecentCalls()
})

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer)
  }
})
</script>

<style scoped>
.api-stats-view {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.stats-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-controls {
  display: flex;
  align-items: center;
}

.overview-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  border-radius: 8px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.stat-content {
  display: flex;
  align-items: center;
  padding: 16px;
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 16px;
}

.api-icon {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.success-icon {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: white;
}

.time-icon {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  color: white;
}

.user-icon {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
  color: white;
}

.stat-info {
  flex: 1;
}

.stat-label {
  font-size: 14px;
  color: #666;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #333;
}

.stats-content {
  margin-top: 24px;
}

.stat-section {
  margin-bottom: 32px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3 {
  margin: 0;
  color: #333;
  font-size: 18px;
}

.realtime-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.realtime-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.realtime-item .label {
  font-weight: 500;
  color: #666;
}

.realtime-item .value {
  font-weight: 600;
  color: #333;
}

.success-text {
  color: #67C23A;
  font-weight: 600;
}

.error-text {
  color: #F56C6C;
  font-weight: 600;
}

.fast-response {
  color: #67C23A;
  font-weight: 600;
}

.medium-response {
  color: #E6A23C;
  font-weight: 600;
}

.slow-response {
  color: #F56C6C;
  font-weight: 600;
}

.loading-container {
  margin-top: 24px;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  text-align: center;
}

.loading-content p {
  margin-top: 16px;
  color: #666;
  font-size: 14px;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .api-stats-view {
    padding: 10px;
  }
  
  .stats-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .overview-cards {
    grid-template-columns: 1fr;
    gap: 12px;
  }
  
  .stat-content {
    padding: 12px;
  }
  
  .stat-icon {
    width: 40px;
    height: 40px;
    margin-right: 12px;
  }
  
  .stat-value {
    font-size: 20px;
  }
  
  .realtime-stats {
    grid-template-columns: 1fr;
    gap: 12px;
  }
  
  .realtime-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
}
</style> 