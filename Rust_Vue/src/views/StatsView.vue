<template>
  <div class="stats-view">
    <el-card>
      <h2>统计信息</h2>
      <el-button class="refresh-btn" @click="loadStats">刷新</el-button>
      <div v-if="stats">
        <div class="stat-section">
          <strong>API 调用次数：</strong>
          <el-table :data="apiCountsArr" style="width: 100%; margin-top: 8px;">
            <el-table-column prop="api" label="接口" width="220" />
            <el-table-column prop="count" label="次数" width="120" />
          </el-table>
        </div>
        
        <div class="stat-section">
          <strong>神包下载趋势：</strong>
          <div class="trend-chart-container">
            <div ref="trendChartRef" class="trend-chart"></div>
          </div>
        </div>
        
        <div class="stat-section">
          <strong>下载量详情：</strong>
          <el-table :data="downloadsArr" style="width: 100%; margin-top: 8px;">
            <el-table-column prop="id" label="绳包ID" width="120" />
            <el-table-column prop="name" label="绳包名称" width="200" />
            <el-table-column prop="count" label="下载量" width="120">
              <template #default="scope">
                <span :class="getDownloadClass(scope.row.count)">
                  {{ scope.row.count }}
                </span>
              </template>
            </el-table-column>
            <el-table-column prop="trend" label="趋势" width="120">
              <template #default="scope">
                <el-tag :type="getTrendType(scope.row.count)" size="small">
                  {{ getTrendText(scope.row.count) }}
                </el-tag>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import { getStats, getPackages } from '../api'
import * as echarts from 'echarts'
import { ElMessage } from 'element-plus'
import { useRouter } from 'vue-router'

const stats = ref<any>(null)
const packages = ref<any[]>([])
const trendChartRef = ref<HTMLElement>()
let trendChart: echarts.ECharts | null = null
const router = useRouter()

const apiCountsArr = computed(() => {
  if (!stats.value?.api_counts) return []
  return Object.entries(stats.value.api_counts).map(([api, count]) => ({ api, count }))
})

const downloadsArr = computed(() => {
  if (!stats.value?.downloads || !packages.value) return []
  
  return Object.entries(stats.value.downloads).map(([id, count]) => {
    const packageInfo = packages.value.find(pkg => pkg.id === parseInt(id))
    return {
      id,
      name: packageInfo?.绳包名称 || `绳包${id}`,
      count: count as number,
      trend: getTrendLevel(count as number)
    }
  }).sort((a, b) => b.count - a.count) // 按下载量降序排列
})

// 根据下载量判断趋势等级
function getTrendLevel(count: number): string {
  if (count >= 200) return 'hot'
  if (count >= 100) return 'popular'
  if (count >= 50) return 'normal'
  return 'low'
}

// 获取趋势类型
function getTrendType(count: number): string {
  if (count >= 200) return 'danger' // 热门
  if (count >= 100) return 'warning' // 流行
  if (count >= 50) return 'success' // 正常
  return 'info' // 较低
}

// 获取趋势文本
function getTrendText(count: number): string {
  if (count >= 200) return '热门'
  if (count >= 100) return '流行'
  if (count >= 50) return '正常'
  return '较低'
}

// 获取下载量样式类
function getDownloadClass(count: number): string {
  if (count >= 200) return 'download-high'
  if (count >= 100) return 'download-medium'
  if (count >= 50) return 'download-normal'
  return 'download-low'
}

// 初始化趋势图表
function initTrendChart() {
  if (!trendChartRef.value) return
  
  trendChart = echarts.init(trendChartRef.value)
  
  const option = {
    title: {
      text: '神包下载量趋势',
      left: 'center',
      textStyle: {
        fontSize: 16,
        fontWeight: 'bold'
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
      data: downloadsArr.value.map(item => item.name),
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
        type: 'bar',
        data: downloadsArr.value.map(item => ({
          value: item.count,
          itemStyle: {
            color: getBarColor(item.count)
          }
        })),
        label: {
          show: true,
          position: 'top',
          formatter: '{c}'
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
  }
  
  trendChart.setOption(option)
}

// 根据下载量获取柱状图颜色
function getBarColor(count: number): string {
  if (count >= 200) return '#ff4757' // 红色 - 热门
  if (count >= 100) return '#ffa502' // 橙色 - 流行
  if (count >= 50) return '#2ed573' // 绿色 - 正常
  return '#70a1ff' // 蓝色 - 较低
}

// 更新图表
function updateTrendChart() {
  if (!trendChart) return
  
  const option = {
    xAxis: {
      data: downloadsArr.value.map(item => item.name)
    },
    series: [
      {
        data: downloadsArr.value.map(item => ({
          value: item.count,
          itemStyle: {
            color: getBarColor(item.count)
          }
        }))
      }
    ]
  }
  
  trendChart.setOption(option)
}

async function loadStats() {
  try {
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
    
    // 等待DOM更新后初始化图表
    await nextTick()
    if (trendChart) {
      updateTrendChart()
    } else {
      initTrendChart()
    }
  } catch (error) {
    console.error('加载统计数据失败:', error)
    
    // 检查是否是网络错误或服务不可用
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED')) {
      
      // 显示服务异常提示
      ElMessage.error('服务异常已安全退出！')
      
      // 延迟跳转到登录页面
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      
      return
    }
    
    // 其他错误显示通用提示
    ElMessage.error('统计数据加载失败，请稍后重试')
  }
}

onMounted(loadStats)
</script>

<style scoped>
.stats-view {
  padding: 32px;
}
.refresh-btn {
  margin-bottom: 18px;
}
.stat-section {
  margin-bottom: 32px;
}
.trend-chart-container {
  margin-top: 16px;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 16px;
  background-color: #fafafa;
}
.trend-chart {
  width: 100%;
  height: 400px;
}
.download-high {
  color: #ff4757;
  font-weight: bold;
}
.download-medium {
  color: #ffa502;
  font-weight: bold;
}
.download-normal {
  color: #2ed573;
  font-weight: bold;
}
.download-low {
  color: #70a1ff;
}
</style> 