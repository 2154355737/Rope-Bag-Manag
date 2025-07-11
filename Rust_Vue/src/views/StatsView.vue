<template>
  <div class="stats-view">
    <el-card>
      <div class="stats-header">
        <h2>统计信息</h2>
        <div class="header-controls">
          <el-select v-model="timeRange" placeholder="时间范围" @change="loadStats" style="width: 120px; margin-right: 12px;">
            <el-option label="全部" value="all" />
            <el-option label="今日" value="today" />
            <el-option label="本周" value="week" />
            <el-option label="本月" value="month" />
          </el-select>
          <el-button @click="loadStats" :loading="loading">刷新</el-button>
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
              <div class="stat-label">API调用总数</div>
              <div class="stat-value">{{ totalApiCalls }}</div>
            </div>
          </div>
        </el-card>
        
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon download-icon">
              <el-icon :size="24"><Download /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-label">总下载量</div>
              <div class="stat-value">{{ totalDownloads }}</div>
            </div>
          </div>
        </el-card>
        
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon package-icon">
              <el-icon :size="24"><Box /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-label">活跃API</div>
              <div class="stat-value">{{ activeApis }}</div>
            </div>
          </div>
        </el-card>
        
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon trend-icon">
              <el-icon :size="24"><TrendCharts /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-label">高频API</div>
              <div class="stat-value">{{ hotApis }}</div>
            </div>
          </div>
        </el-card>
      </div>

      <div v-if="stats" class="stats-content">
        <!-- API调用统计 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>API 调用统计</h3>
            <el-button size="small" @click="exportApiStats">导出</el-button>
          </div>
          <el-table :data="apiCountsArr" style="width: 100%; margin-top: 8px;" size="small">
            <el-table-column prop="api" label="接口名称" min-width="200" />
            <el-table-column prop="count" label="调用次数" width="120">
              <template #default="scope">
                <el-tag :type="getApiTagType(scope.row.count)" size="small">
                  {{ scope.row.count }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="percentage" label="占比" width="100">
              <template #default="scope">
                {{ scope.row.percentage }}%
              </template>
            </el-table-column>
          </el-table>
        </div>
        
        <!-- API调用统计图表 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>API调用趋势图表</h3>
            <div class="chart-controls">
              <el-radio-group v-model="chartType" size="small" @change="updateChart">
                <el-radio-button value="bar">柱状图</el-radio-button>
                <el-radio-button value="line">折线图</el-radio-button>
                <el-radio-button value="pie">饼图</el-radio-button>
              </el-radio-group>
            </div>
          </div>
          <div class="trend-chart-container">
            <div v-if="apiCountsArr.length > 0" ref="trendChartRef" class="trend-chart"></div>
            <div v-else class="chart-placeholder">
              <el-icon :size="48" style="color: #c0c4cc; margin-bottom: 16px;">
                <DataAnalysis />
              </el-icon>
              <p style="color: #909399; margin: 0;">API调用趋势图表</p>
              <small style="color: #c0c4cc;">暂无API调用数据</small>
            </div>
          </div>
        </div>
        
        <!-- API使用详细 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>API使用详细</h3>
            <el-button size="small" @click="exportApiDetails">导出</el-button>
          </div>
          <el-table :data="apiDetailsArr" style="width: 100%; margin-top: 8px;" size="small">
            <el-table-column prop="api" label="接口名称" min-width="200" show-overflow-tooltip />
            <el-table-column prop="count" label="调用次数" width="120">
              <template #default="scope">
                <span :class="getApiUsageClass(scope.row.count)">
                  {{ scope.row.count }}
                </span>
              </template>
            </el-table-column>
            <el-table-column prop="status" label="使用状态" width="120">
              <template #default="scope">
                <el-tag :type="getApiStatusType(scope.row.count)" size="small">
                  {{ getApiStatusText(scope.row.count) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="percentage" label="占比" width="100">
              <template #default="scope">
                {{ scope.row.percentage }}%
              </template>
            </el-table-column>
            <el-table-column prop="lastUsed" label="最后使用" width="150">
              <template #default="scope">
                <span class="last-used-time">{{ formatLastUsedTime(scope.row.lastUsed) }}</span>
              </template>
            </el-table-column>
          </el-table>
        </div>

        <!-- 实时统计 -->
        <div class="stat-section">
          <div class="section-header">
            <h3>实时统计</h3>
            <el-switch v-model="autoRefresh" active-text="自动刷新" />
          </div>
          <div class="realtime-stats">
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
import { ref, computed, onMounted, nextTick, onUnmounted, watch } from 'vue'
import { getStats, getPackages } from '../api'
import * as echarts from 'echarts'
import { ElMessage } from 'element-plus'
import { useRouter } from 'vue-router'
import { Connection, Download, Box, TrendCharts } from '@element-plus/icons-vue'

const stats = ref<any>(null)
const packages = ref<any[]>([])
const trendChartRef = ref<HTMLElement>()
const loading = ref(false)
const timeRange = ref('all')
const chartType = ref('bar')
const autoRefresh = ref(false)
const lastUpdateTime = ref('')
const dataStatus = ref('normal')
let trendChart: echarts.ECharts | null = null
let refreshTimer: number | null = null
const router = useRouter()

// 计算属性
const totalApiCalls = computed(() => {
  if (!stats.value?.api_counts) return 0
  return Object.values(stats.value.api_counts).reduce((sum: number, count: any) => sum + count, 0)
})



const activeApis = computed(() => {
  if (!stats.value?.api_counts) return 0
  return Object.keys(stats.value.api_counts).length
})

const hotApis = computed(() => {
  if (!apiCountsArr.value) return 0
  return apiCountsArr.value.filter(item => item.count >= 100).length
})

const apiCountsArr = computed(() => {
  if (!stats.value?.api_counts) return []
  const total = totalApiCalls.value
  return Object.entries(stats.value.api_counts)
    .map(([api, count]) => ({ 
      api, 
      count: count as number,
      percentage: total > 0 ? Math.round((count as number / total) * 100) : 0
    }))
    .sort((a, b) => b.count - a.count)
})

const apiDetailsArr = computed(() => {
  if (!stats.value?.api_counts) return []
  
  const total = totalApiCalls.value
  const now = Date.now()
  
  return Object.entries(stats.value.api_counts).map(([api, count]) => {
    // 模拟最后使用时间（基于调用次数生成一个相对时间）
    const lastUsed = now - (Math.random() * 24 * 60 * 60 * 1000) // 随机生成过去24小时内的时间
    
    return {
      api,
      count: count as number,
      status: getApiUsageLevel(count as number),
      percentage: total > 0 ? Math.round((count as number / total) * 100) : 0,
      lastUsed
    }
  }).sort((a, b) => b.count - a.count)
})

// 工具函数


function getApiTagType(count: number): string {
  if (count >= 1000) return 'danger'
  if (count >= 500) return 'warning'
  if (count >= 100) return 'success'
  return 'info'
}

function getApiUsageLevel(count: number): string {
  if (count >= 1000) return 'high'
  if (count >= 500) return 'medium'
  if (count >= 100) return 'normal'
  return 'low'
}

function getApiStatusType(count: number): string {
  if (count >= 1000) return 'danger'
  if (count >= 500) return 'warning'
  if (count >= 100) return 'success'
  return 'info'
}

function getApiStatusText(count: number): string {
  if (count >= 1000) return '高频'
  if (count >= 500) return '中频'
  if (count >= 100) return '正常'
  return '低频'
}

function getApiUsageClass(count: number): string {
  if (count >= 1000) return 'api-usage-high'
  if (count >= 500) return 'api-usage-medium'
  if (count >= 100) return 'api-usage-normal'
  return 'api-usage-low'
}

function formatLastUsedTime(timestamp: number): string {
  const now = Date.now()
  const diff = now - timestamp
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

// 图表相关
function initTrendChart() {
  if (!trendChartRef.value) return
  
  // 检测深色模式
  const isDark = document.body.classList.contains('dark') || 
                 document.documentElement.classList.contains('dark')
  
  trendChart = echarts.init(trendChartRef.value, undefined, {
    renderer: 'canvas',
    useDirtyRect: true
  })
  
  // 监听主题变化
  const observer = new MutationObserver(() => {
    if (trendChart) {
      updateChart()
    }
  })
  
  observer.observe(document.body, {
    attributes: true,
    attributeFilter: ['class']
  })
  
  updateChart()
  
  // 响应式处理
  window.addEventListener('resize', () => {
    if (trendChart) {
      trendChart.resize()
    }
  })
}

function updateChart() {
  if (!trendChart) return
  
  const option = getChartOption()
  trendChart.setOption(option, true)
}

function getChartOption() {
  const data = apiCountsArr.value.slice(0, 10) // 只显示前10个API
  
  // 检测深色模式
  const isDark = document.body.classList.contains('dark') || 
                 document.documentElement.classList.contains('dark')
  
  const textColor = isDark ? '#ffffff' : '#333333'
  const backgroundColor = isDark ? '#1a1a1a' : '#ffffff'
  const axisLineColor = isDark ? '#444444' : '#e4e7ed'
  const gridColor = isDark ? '#333333' : '#f5f5f5'
  
  if (chartType.value === 'pie') {
    return {
      backgroundColor: backgroundColor,
      title: {
        text: 'API调用量分布',
        left: 'center',
        textStyle: {
          color: textColor
        }
      },
      tooltip: {
        trigger: 'item',
        formatter: '{a} <br/>{b}: {c} ({d}%)',
        backgroundColor: isDark ? '#2a2a2a' : '#ffffff',
        borderColor: isDark ? '#444444' : '#e4e7ed',
        textStyle: {
          color: textColor
        }
      },
      series: [{
        name: 'API调用量',
        type: 'pie',
        radius: '50%',
        data: data.map(item => ({
          name: item.api,
          value: item.count,
          itemStyle: {
            color: getApiBarColor(item.count)
          }
        }))
      }]
    }
  } else if (chartType.value === 'line') {
    return {
      backgroundColor: backgroundColor,
      title: {
        text: 'API调用量趋势',
        left: 'center',
        textStyle: {
          color: textColor
        }
      },
      tooltip: {
        trigger: 'axis',
        backgroundColor: isDark ? '#2a2a2a' : '#ffffff',
        borderColor: isDark ? '#444444' : '#e4e7ed',
        textStyle: {
          color: textColor
        }
      },
      xAxis: {
        type: 'category',
        data: data.map(item => item.api),
        axisLabel: {
          rotate: 45,
          fontSize: 10,
          color: textColor
        },
        axisLine: {
          lineStyle: {
            color: axisLineColor
          }
        }
      },
      yAxis: {
        type: 'value',
        name: '调用次数',
        nameTextStyle: {
          color: textColor
        },
        axisLabel: {
          color: textColor
        },
        axisLine: {
          lineStyle: {
            color: axisLineColor
          }
        },
        splitLine: {
          lineStyle: {
            color: gridColor
          }
        }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '15%',
        containLabel: true
      },
      series: [{
        name: 'API调用量',
        type: 'line',
        data: data.map(item => ({
          value: item.count,
          itemStyle: {
            color: getApiBarColor(item.count)
          }
        })),
        smooth: true,
        label: {
          show: true,
          position: 'top',
          color: textColor
        }
      }]
    }
  } else {
    return {
      backgroundColor: backgroundColor,
      title: {
        text: 'API调用量统计',
        left: 'center',
        textStyle: {
          color: textColor
        }
      },
      tooltip: {
        trigger: 'axis',
        backgroundColor: isDark ? '#2a2a2a' : '#ffffff',
        borderColor: isDark ? '#444444' : '#e4e7ed',
        textStyle: {
          color: textColor
        }
      },
      xAxis: {
        type: 'category',
        data: data.map(item => item.api),
        axisLabel: {
          rotate: 45,
          fontSize: 10,
          color: textColor
        },
        axisLine: {
          lineStyle: {
            color: axisLineColor
          }
        }
      },
      yAxis: {
        type: 'value',
        name: '调用次数',
        nameTextStyle: {
          color: textColor
        },
        axisLabel: {
          color: textColor
        },
        axisLine: {
          lineStyle: {
            color: axisLineColor
          }
        },
        splitLine: {
          lineStyle: {
            color: gridColor
          }
        }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '15%',
        containLabel: true
      },
      series: [{
        name: 'API调用量',
        type: 'bar',
        data: data.map(item => ({
          value: item.count,
          itemStyle: {
            color: getApiBarColor(item.count)
          }
        })),
        label: {
          show: true,
          position: 'top',
          color: textColor
        }
      }]
    }
  }
}

function getBarColor(count: number): string {
  if (count >= 200) return '#ff4757'
  if (count >= 100) return '#ffa502'
  if (count >= 50) return '#2ed573'
  return '#70a1ff'
}

function getApiBarColor(count: number): string {
  if (count >= 1000) return '#ff4757' // 红色 - 高频调用
  if (count >= 500) return '#ffa502'   // 橙色 - 中高频调用
  if (count >= 100) return '#2ed573'   // 绿色 - 中频调用
  if (count >= 50) return '#70a1ff'    // 蓝色 - 低频调用
  return '#a4b0be'                     // 灰色 - 极低频调用
}

// 导出功能
function exportApiStats() {
  const data = apiCountsArr.value
  const csv = ['接口名称,调用次数,占比\n']
  data.forEach(item => {
    csv.push(`${item.api},${item.count},${item.percentage}%\n`)
  })
  downloadCSV(csv.join(''), 'api_stats.csv')
}

function exportApiDetails() {
  const data = apiDetailsArr.value
  const csv = ['接口名称,调用次数,使用状态,占比,最后使用\n']
  data.forEach(item => {
    csv.push(`${item.api},${item.count},${getApiStatusText(item.count)},${item.percentage}%,${formatLastUsedTime(item.lastUsed)}\n`)
  })
  downloadCSV(csv.join(''), 'api_details.csv')
}

function downloadCSV(content: string, filename: string) {
  const blob = new Blob([content], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = filename
  link.click()
  URL.revokeObjectURL(link.href)
}

// 自动刷新
function startAutoRefresh() {
  if (autoRefresh.value) {
    refreshTimer = window.setInterval(() => {
      loadStats()
    }, 30000) // 30秒刷新一次
  }
}

function stopAutoRefresh() {
  if (refreshTimer) {
    clearInterval(refreshTimer)
    refreshTimer = null
  }
}

async function loadStats() {
  try {
    loading.value = true
    const [statsRes, packagesRes] = await Promise.all([
      getStats(),
      getPackages()
    ])
    
    if (statsRes.code === 0 && statsRes.data) {
      stats.value = statsRes.data
    }
    
    if (packagesRes.code === 0 && packagesRes.data?.绳包列表) {
      packages.value = packagesRes.data.绳包列表
    }
    
    lastUpdateTime.value = new Date().toLocaleString()
    dataStatus.value = 'normal'
    
    await nextTick()
    if (apiCountsArr.value.length > 0) {
      if (trendChart) {
        updateChart()
      } else {
        initTrendChart()
      }
    }
  } catch (error) {
    console.error('加载统计数据失败:', error)
    dataStatus.value = 'error'
    
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED')) {
      
      ElMessage.error('服务异常已安全退出！')
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      return
    }
    
    ElMessage.error('统计数据加载失败，请稍后重试')
  } finally {
    loading.value = false
  }
}

// 监听自动刷新开关
watch(autoRefresh, (newVal) => {
  if (newVal) {
    startAutoRefresh()
  } else {
    stopAutoRefresh()
  }
})

onMounted(() => {
  loadStats()
})

onUnmounted(() => {
  stopAutoRefresh()
  if (trendChart) {
    trendChart.dispose()
  }
})
</script>

<style scoped>
.stats-view {
  padding: 16px;
}

.stats-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.header-controls {
  display: flex;
  align-items: center;
}

.overview-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  border-radius: 8px;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
}

.api-icon { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
.download-icon { background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); }
.package-icon { background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); }
.trend-icon { background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); }

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

/* 深色模式下的统计卡片文字颜色 */
.dark .stat-label {
  color: #a0a0a0;
}

.dark .stat-value {
  color: #ffffff;
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
  font-size: 18px;
  color: #333;
}

/* 深色模式下的标题颜色 */
.dark .section-header h3 {
  color: #ffffff;
}

.chart-controls {
  display: flex;
  gap: 8px;
}

.trend-chart-container {
  margin-top: 16px;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  padding: 16px;
  background-color: #fafafa;
}

/* 深色模式下的图表容器 */
.dark .trend-chart-container {
  border-color: #444444;
  background-color: #1a1a1a;
}

.trend-chart {
  width: 100%;
  height: 400px;
  min-height: 300px;
}

.chart-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 300px;
  text-align: center;
  color: #909399;
}

/* 深色模式下的图表占位符 */
.dark .chart-placeholder {
  color: #a0a0a0;
}

.dark .chart-placeholder p {
  color: #b0b0b0;
}

.dark .chart-placeholder small {
  color: #808080;
}

.chart-placeholder p {
  margin: 8px 0 4px 0;
  font-size: 14px;
}

.chart-placeholder small {
  font-size: 12px;
}

/* 移动端图表适配 */
@media (max-width: 768px) {
  .trend-chart {
    height: 300px;
    min-height: 250px;
  }
  
  .trend-chart-container {
    padding: 12px;
  }
  
  .chart-controls {
    flex-direction: column;
    gap: 8px;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .chart-placeholder {
    height: 250px;
  }
  
  .chart-placeholder .el-icon {
    font-size: 36px;
  }
  
  .chart-placeholder p {
    font-size: 12px;
  }
  
  .chart-placeholder small {
    font-size: 10px;
  }
}

.realtime-stats {
  display: flex;
  gap: 24px;
  flex-wrap: wrap;
}

.realtime-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.realtime-item .label {
  font-size: 14px;
  color: #666;
}

.realtime-item .value {
  font-size: 14px;
  color: #333;
  font-weight: 500;
}

/* 深色模式下的实时统计文字颜色 */
.dark .realtime-item .label {
  color: #a0a0a0;
}

.dark .realtime-item .value {
  color: #ffffff;
}

/* 深色模式下的API使用状态颜色增强 */
.dark .api-usage-high {
  color: #ff6b6b;
}

.dark .api-usage-medium {
  color: #ffd93d;
}

.dark .api-usage-normal {
  color: #6bcf7f;
}

.dark .api-usage-low {
  color: #74c0fc;
}

/* 深色模式下的表格文字颜色 */
.dark .el-table {
  color: #ffffff;
}

.dark .el-table th {
  background-color: #2a2a2a;
  color: #ffffff;
}

.dark .el-table td {
  background-color: #1a1a1a;
  color: #ffffff;
}

.dark .el-table--striped .el-table__body tr.el-table__row--striped td {
  background-color: #222222;
}

/* 深色模式下的卡片样式 */
.dark .el-card {
  background-color: #1a1a1a;
  border-color: #444444;
}

.dark .el-card__body {
  color: #ffffff;
}

/* 深色模式下的按钮和控件样式 */
.dark .el-button {
  border-color: #444444;
}

.dark .el-select .el-input__wrapper {
  background-color: #2a2a2a;
  border-color: #444444;
}

.dark .el-radio-group .el-radio-button__inner {
  background-color: #2a2a2a;
  border-color: #444444;
  color: #ffffff;
}

.dark .el-radio-group .el-radio-button__original-radio:checked + .el-radio-button__inner {
  background-color: #409eff;
  border-color: #409eff;
  color: #ffffff;
}

/* 深色模式下的标签样式 */
.dark .el-tag {
  background-color: #2a2a2a;
  border-color: #444444;
}

.dark .el-tag--success {
  background-color: #1e3a2e;
  border-color: #4caf50;
  color: #4caf50;
}

.dark .el-tag--warning {
  background-color: #3a2e1e;
  border-color: #ff9800;
  color: #ff9800;
}

.dark .el-tag--danger {
  background-color: #3a1e1e;
  border-color: #f44336;
  color: #f44336;
}

.dark .el-tag--info {
  background-color: #1e2a3a;
  border-color: #2196f3;
  color: #2196f3;
}

/* 深色模式下的开关样式 */
.dark .el-switch__core {
  background-color: #444444;
}

.dark .el-switch__core.is-checked {
  background-color: #409eff;
}

.api-usage-high {
  color: #ff4757;
  font-weight: bold;
}

.api-usage-medium {
  color: #ffa502;
  font-weight: bold;
}

.api-usage-normal {
  color: #2ed573;
  font-weight: bold;
}

.api-usage-low {
  color: #70a1ff;
  font-weight: bold;
}

.last-used-time {
  color: #909399;
  font-size: 12px;
}

/* 深色模式下的时间显示 */
.dark .last-used-time {
  color: #a0a0a0;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .stats-view {
    padding: 8px;
  }
  
  .stats-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .header-controls {
    width: 100%;
    justify-content: space-between;
  }
  
  .overview-cards {
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }
  
  .stat-content {
    gap: 12px;
  }
  
  .stat-icon {
    width: 40px;
    height: 40px;
  }
  
  .stat-value {
    font-size: 20px;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .chart-controls {
    width: 100%;
    justify-content: center;
  }
  
  .trend-chart {
    height: 300px;
  }
  
  .realtime-stats {
    flex-direction: column;
    gap: 12px;
  }
  
  /* API使用详细表格移动端适配 */
  .el-table {
    font-size: 12px;
  }
  
  .el-table .el-table__cell {
    padding: 8px 4px;
  }
  
  .last-used-time {
    font-size: 10px;
  }
}

@media (max-width: 480px) {
  .overview-cards {
    grid-template-columns: 1fr;
  }
  
  .trend-chart {
    height: 250px;
  }
}
</style> 