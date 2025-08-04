<template>
  <div class="admin-page resource-record">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><List /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">资源记录</h1>
            <p class="page-subtitle">记录系统中所有资源的创建、修改、删除等操作历史</p>
          </div>
        </div>
        <div class="header-actions">
          <div class="refresh-controls">
            <el-switch
              v-model="autoRefresh"
              active-text="自动刷新"
              inactive-text="手动刷新"
              style="margin-right: 10px;"
            />
            <span v-if="autoRefresh" class="refresh-info">每 {{ refreshInterval }}s 刷新</span>
            <el-button @click="handleRefresh" :loading="loading">
              <el-icon><Refresh /></el-icon>
              立即刷新
            </el-button>
          </div>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Document /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.total_records }}</div>
            <div class="stat-label">总记录数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Plus /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.create_count }}</div>
            <div class="stat-label">创建操作</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Edit /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.update_count }}</div>
            <div class="stat-label">更新操作</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Delete /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ stats.delete_count }}</div>
            <div class="stat-label">删除操作</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 筛选条件 -->
    <div class="search-section">
      <div class="search-left">
        <el-select 
          v-model="filterForm.resource_type" 
          placeholder="资源类型" 
          clearable
          style="width: 150px"
        >
          <el-option label="全部" value="" />
          <el-option label="绳包" value="Package" />
          <el-option label="用户" value="User" />
          <el-option label="分类" value="Category" />
          <el-option label="评论" value="Comment" />
          <el-option label="设置" value="Settings" />
        </el-select>
        
        <el-select 
          v-model="filterForm.action" 
          placeholder="操作类型" 
          clearable
          style="width: 150px"
        >
          <el-option label="全部" value="" />
          <el-option label="创建" value="Create" />
          <el-option label="更新" value="Update" />
          <el-option label="删除" value="Delete" />
          <el-option label="下载" value="Download" />
          <el-option label="上传" value="Upload" />
          <el-option label="标星" value="Star" />
          <el-option label="封禁" value="Ban" />
        </el-select>
        
        <el-input 
          v-model="filterForm.user_id" 
          placeholder="输入用户ID" 
          clearable
          style="width: 150px"
        />
        
        <el-date-picker
          v-model="filterForm.date_range"
          type="datetimerange"
          range-separator="至"
          start-placeholder="开始时间"
          end-placeholder="结束时间"
          format="YYYY-MM-DD HH:mm:ss"
          value-format="YYYY-MM-DD HH:mm:ss"
          style="width: 350px"
        />
      </div>
      
      <div class="search-right">
        <el-button type="primary" @click="handleFilter">
          <el-icon><Search /></el-icon>
          筛选
        </el-button>
        <el-button @click="resetFilter">
          <el-icon><RefreshRight /></el-icon>
          重置
        </el-button>
        <el-button type="success" @click="exportRecords">
          <el-icon><Download /></el-icon>
          导出记录
        </el-button>
      </div>
    </div>

    <!-- 资源记录列表 -->
    <div class="table-section">
      <el-table 
        :data="recordList" 
        v-loading="loading"
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
        border
        stripe
      >
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="resource_type" label="资源类型" width="120">
          <template #default="{ row }">
            <el-tag :type="getResourceTypeTag(row.resource_type)">
              {{ getResourceTypeLabel(row.resource_type) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="resource_id" label="资源ID" width="100" />
        <el-table-column prop="action" label="操作类型" width="100">
          <template #default="{ row }">
            <el-tag :type="getActionTag(row.action)">
              {{ getActionLabel(row.action) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="user_id" label="操作用户" width="100" />
        <el-table-column prop="details" label="操作详情" min-width="250" show-overflow-tooltip />
        <el-table-column prop="created_at" label="操作时间" width="180">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="viewRecord(row)">
              <el-icon><View /></el-icon>
              查看
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-section">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </div>

    <!-- 其余对话框保持不变 -->
    <!-- 记录详情对话框 -->
    <el-dialog 
      v-model="recordDialogVisible" 
      title="记录详情" 
      width="800px"
    >
      <div class="record-detail" v-if="currentRecord">
        <div class="detail-item">
          <label>记录ID:</label>
          <span>{{ currentRecord.id }}</span>
        </div>
        <div class="detail-item">
          <label>资源类型:</label>
          <el-tag :type="getResourceTypeTag(currentRecord.resource_type)">
            {{ getResourceTypeLabel(currentRecord.resource_type) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>资源ID:</label>
          <span>{{ currentRecord.resource_id }}</span>
        </div>
        <div class="detail-item">
          <label>操作用户:</label>
          <span>{{ currentRecord.user_id }}</span>
        </div>
        <div class="detail-item">
          <label>操作类型:</label>
          <el-tag :type="getActionTag(currentRecord.action)">
            {{ getActionLabel(currentRecord.action) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>操作时间:</label>
          <span>{{ formatTime(currentRecord.timestamp) }}</span>
        </div>
        <div class="detail-item">
          <label>IP地址:</label>
          <span>{{ currentRecord.ip_address }}</span>
        </div>
        <div class="detail-item" v-if="currentRecord.old_data">
          <label>旧数据:</label>
          <div class="data-box old-data-box">
            <pre>{{ formatJson(currentRecord.old_data) }}</pre>
          </div>
        </div>
        <div class="detail-item" v-if="currentRecord.new_data">
          <label>新数据:</label>
          <div class="data-box new-data-box">
            <pre>{{ formatJson(currentRecord.new_data) }}</pre>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button @click="recordDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- 删除图表容器 -->
  </div>
</template>

<script setup lang="ts">
// 导入所需依赖
import { ref, reactive, onMounted, watch, onUnmounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { resourceRecordApi, ResourceRecord } from '../../api/resourceRecords'
// 删除echarts相关导入
// import * as echarts from 'echarts'
// import type { EChartsType } from 'echarts'
// import { saveAs } from 'file-saver'
// import { utils, writeFile } from 'xlsx'

// 定义Timeout类型
type Timeout = ReturnType<typeof setTimeout>

// 响应式数据
const loading = ref(false)
const recordList = ref<ResourceRecord[]>([])
const selectedRecords = ref<ResourceRecord[]>([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const recordDialogVisible = ref(false)
const currentRecord = ref<ResourceRecord | null>(null)

const stats = reactive({
  total_records: 0,
  create_count: 0,
  update_count: 0,
  delete_count: 0
})

const filterForm = reactive({
  resource_type: '',
  action: '',
  user_id: '',
  date_range: [] as Date[]
})

// 添加图表相关的响应式数据
// const chartContainer = ref<HTMLElement | null>(null)
// const chartInstance = ref<EChartsType | null>(null)
// const chartData = reactive({
//   dates: [] as string[],
//   counts: [] as number[],
//   byType: {} as Record<string, number>
// })

const autoRefresh = ref(false)
const refreshInterval = ref(30)
let refreshTimer: Timeout | null = null

// 方法
async function loadRecords() {
  loading.value = true
  try {
    // 构建参数
    const params: any = {
      page: currentPage.value,
      pageSize: pageSize.value,
      resource_type: filterForm.resource_type || undefined,
      action: filterForm.action || undefined,
      user_id: filterForm.user_id ? parseInt(filterForm.user_id) : undefined
    }
    
    // 添加日期范围
    if (filterForm.date_range && filterForm.date_range.length === 2) {
      params.start_date = formatDateForApi(filterForm.date_range[0])
      params.end_date = formatDateForApi(filterForm.date_range[1])
    }
    
    console.log('加载资源记录，参数:', params)
    const response = await resourceRecordApi.getResourceRecords(params)
    if (response.code === 0 && response.data) {
      recordList.value = response.data.list || []
      total.value = response.data.total || 0
      stats.total_records = total.value
      
      // 更新统计数据
      updateStats()
      console.log('加载到资源记录:', recordList.value.length)
    } else {
              console.error('加载资源记录返回错误:', response.msg || response.message)
        ElMessage.error(response.msg || response.message || '加载资源记录失败')
    }
  } catch (error) {
    console.error('加载资源记录失败:', error)
    ElMessage.error('加载资源记录失败')
  } finally {
    loading.value = false
  }
}

// 格式化日期为API所需的格式
function formatDateForApi(date: Date): string {
  if (!date) return ''
  return date.toISOString().slice(0, 19).replace('T', ' ')
}

// 更新统计数据
function updateStats() {
  if (recordList.value && recordList.value.length > 0) {
    // 直接从列表中计算统计数据
    stats.create_count = recordList.value.filter(r => r.action === 'Create').length
    stats.update_count = recordList.value.filter(r => r.action === 'Update').length
    stats.delete_count = recordList.value.filter(r => r.action === 'Delete').length
    
    // 加载完整的统计数据
    loadActionStats()
  }
}

// 加载操作统计数据
async function loadActionStats() {
  try {
    // 构建统计查询参数
    const params: any = {}
    
    // 添加日期范围
    if (filterForm.date_range && filterForm.date_range.length === 2) {
      params.start_date = formatDateForApi(filterForm.date_range[0])
      params.end_date = formatDateForApi(filterForm.date_range[1])
    }
    
    // 添加资源类型
    if (filterForm.resource_type) {
      params.resource_type = filterForm.resource_type
    }
    
    // 添加用户ID
    if (filterForm.user_id) {
      params.user_id = parseInt(filterForm.user_id)
    }
    
    console.log('加载资源操作统计，参数:', params)
    const response = await resourceRecordApi.getResourceActionStats(params)
    if (response.code === 0 && response.data) {
      // 更新图表数据
      // chartData.dates = response.data.by_day.map((item: any) => item.date)
      // chartData.counts = response.data.by_day.map((item: any) => item.count)
      
      // 更新统计数据
      stats.create_count = response.data.create_count || 0
      stats.update_count = response.data.update_count || 0
      stats.delete_count = response.data.delete_count || 0
      
      // 渲染图表
      // renderChart()
    } else {
      console.error('加载统计数据返回错误:', response.message)
    }
  } catch (error) {
    console.error('加载统计数据失败:', error)
  }
}

// 格式化JSON数据以便显示
function formatJson(jsonStr: string | null | undefined): string {
  if (!jsonStr) return ''
  try {
    // 如果已经是对象，先转为字符串
    const jsonObj = typeof jsonStr === 'object' ? jsonStr : JSON.parse(jsonStr)
    return JSON.stringify(jsonObj, null, 2)
  } catch (e) {
    return String(jsonStr)
  }
}

// 格式化时间戳
function formatTime(timestamp: string | number): string {
  if (!timestamp) return ''
  
  // 如果是数字时间戳，转换为日期
  if (typeof timestamp === 'number') {
    const date = new Date(timestamp * 1000) // 秒转毫秒
    return date.toLocaleString()
  }
  
  // 如果是字符串日期，直接返回
  return timestamp
}

function handleFilter() {
  currentPage.value = 1
  loadRecords()
}

function resetFilter() {
  Object.assign(filterForm, {
    resource_type: '',
    action: '',
    user_id: '',
    date_range: []
  })
  handleFilter()
}

function handleSelectionChange(selection: ResourceRecord[]) {
  selectedRecords.value = selection
}

function handleSizeChange(size: number) {
  pageSize.value = size
  loadRecords()
}

function handleCurrentChange(page: number) {
  currentPage.value = page
  loadRecords()
}

function viewRecord(record: ResourceRecord) {
  currentRecord.value = record
  recordDialogVisible.value = true
}

async function deleteRecord(record: ResourceRecord) {
  try {
    await ElMessageBox.confirm('确定要删除这条记录吗？', '确认删除')
    const response = await resourceRecordApi.deleteResourceRecord(record.id)
    if (response.code === 0) {
      ElMessage.success('记录已删除')
      loadRecords()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

async function batchDelete() {
  if (selectedRecords.value.length === 0) {
    ElMessage.warning('请选择要删除的记录')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedRecords.value.length} 条记录吗？`, '确认删除')
    // 调用API批量删除
    const recordIds = selectedRecords.value.map(record => record.id)
    const response = await resourceRecordApi.batchDeleteResourceRecords(recordIds)
    
    if (response.code === 0) {
      ElMessage.success('批量删除成功')
      loadRecords()
    } else {
      ElMessage.error(`删除失败: ${response.message}`)
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

// 导出记录为Excel文件
function exportRecords() {
  try {
    // 检查是否有记录可导出
    if (recordList.value.length === 0) {
      ElMessage.warning('没有记录可导出')
      return
    }
    
    // 由于当前环境不支持文件导出库，提示用户手动导出
    ElMessage.info('当前版本不支持自动导出，请使用浏览器保存功能手动导出数据')

    // 备注：如果需要支持导出功能，需要安装相关依赖
    // npm install file-saver xlsx
    
    // 以下代码在安装相关依赖后可以取消注释使用
    /*
    // 准备导出数据
    const exportData = recordList.value.map(record => ({
      ID: record.id,
      资源类型: getResourceTypeLabel(record.resource_type),
      资源ID: record.resource_id,
      资源名称: record.resource_name || '',
      操作用户: record.user_name || record.user_id,
      操作类型: getActionLabel(record.action),
      操作时间: formatTime(record.timestamp),
      IP地址: record.ip_address || ''
    }))
    
    // 创建工作表
    const worksheet = utils.json_to_sheet(exportData)
    
    // 创建工作簿
    const workbook = utils.book_new()
    utils.book_append_sheet(workbook, worksheet, '资源操作记录')
    
    // 生成Excel文件
    const now = new Date()
    const dateStr = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`
    writeFile(workbook, `资源操作记录_${dateStr}.xlsx`)
    
    ElMessage.success('导出成功')
    */
  } catch (error) {
    console.error('导出记录失败:', error)
    ElMessage.error('导出失败')
  }
}

function handleRefresh() {
  loadRecords()
}

function getResourceTypeLabel(type: string): string {
  const labels: { [key: string]: string } = {
    Package: '绳包',
    User: '用户',
    Category: '分类',
    Comment: '评论',
    Settings: '设置',
    Backup: '备份',
    Announcement: '公告',
    Resource: '资源',
    Log: '日志',
    Role: '角色',
    Permission: '权限',
    File: '文件',
    Tag: '标签',
    Image: '图片',
    Video: '视频',
    Audio: '音频',
    Document: '文档'
  }
  return labels[type] || type
}

function getResourceTypeTag(type: string): string {
  const tags: { [key: string]: string } = {
    Package: 'primary',
    User: 'success',
    Category: 'warning',
    Comment: 'info',
    Settings: 'danger',
    Backup: 'info',
    Announcement: 'warning',
    Resource: 'primary',
    Log: 'info',
    Role: 'success',
    Permission: 'danger',
    File: 'primary',
    Tag: 'warning',
    Image: 'primary',
    Video: 'success',
    Audio: 'warning',
    Document: 'info'
  }
  return tags[type] || 'info'
}

function getActionLabel(action: string): string {
  const labels: { [key: string]: string } = {
    Create: '创建',
    Update: '更新',
    Delete: '删除',
    Download: '下载',
    Upload: '上传',
    Star: '标星',
    Ban: '封禁',
    StatusChange: '状态变更',
    Import: '导入',
    Export: '导出',
    Approve: '审核通过',
    Reject: '拒绝',
    Restore: '恢复',
    Move: '移动',
    Copy: '复制',
    Share: '分享',
    Like: '点赞',
    Dislike: '点踩',
    Comment: '评论',
    Reply: '回复'
  }
  return labels[action] || action
}

function getActionTag(action: string): string {
  const tags: { [key: string]: string } = {
    Create: 'success',
    Update: 'warning',
    Delete: 'danger',
    Download: 'info',
    Upload: 'primary',
    Star: 'warning',
    Ban: 'danger',
    StatusChange: 'warning',
    Import: 'primary',
    Export: 'info',
    Approve: 'success',
    Reject: 'danger',
    Restore: 'success',
    Move: 'warning',
    Copy: 'info',
    Share: 'primary',
    Like: 'success',
    Dislike: 'danger',
    Comment: 'info',
    Reply: 'info'
  }
  return tags[action] || 'info'
}

// 监听筛选条件变化
watch([() => filterForm.resource_type, () => filterForm.action, () => filterForm.user_id, () => filterForm.date_range], () => {
  // 当筛选条件变化时，自动更新筛选
  handleFilter()
}, { deep: true })

// 组件挂载完成后执行
onMounted(() => {
  loadRecords()
})

// 设置自动刷新
watch(autoRefresh, (newVal) => {
  if (newVal) {
    // 启动定时器
    refreshTimer = window.setInterval(() => {
      loadRecords()
    }, refreshInterval.value * 1000)
  } else {
    // 清除定时器
    if (refreshTimer !== null) {
      clearInterval(refreshTimer)
      refreshTimer = null
    }
  }
})

// 在组件销毁时清除定时器
onUnmounted(() => {
  if (refreshTimer !== null) {
    clearInterval(refreshTimer)
  }
  // 销毁图表实例
  // if (chartInstance.value) {
  //   chartInstance.value.dispose()
  // }
})

// 渲染图表
// function renderChart() {
//   if (!chartContainer.value) return
  
//   // 如果已存在图表实例，销毁它
//   if (chartInstance.value) {
//     chartInstance.value.dispose()
//   }
  
//   // 创建新的图表实例
//   chartInstance.value = echarts.init(chartContainer.value)
  
//   // 图表配置
//   const option = {
//     title: {
//       text: '资源操作统计',
//       textStyle: {
//         color: '#333'
//       }
//     },
//     tooltip: {
//       trigger: 'axis'
//     },
//     legend: {
//       data: ['操作次数']
//     },
//     xAxis: {
//       type: 'category',
//       data: chartData.dates
//     },
//     yAxis: {
//       type: 'value',
//       name: '次数'
//     },
//     series: [
//       {
//         name: '操作次数',
//         type: 'line',
//         data: chartData.counts,
//         smooth: true,
//         itemStyle: {
//           color: '#409EFF'
//         },
//         areaStyle: {
//           color: {
//             type: 'linear',
//             x: 0,
//             y: 0,
//             x2: 0,
//             y2: 1,
//             colorStops: [
//               {
//                 offset: 0,
//                 color: 'rgba(64, 158, 255, 0.5)'
//               },
//               {
//                 offset: 1,
//                 color: 'rgba(64, 158, 255, 0.1)'
//               }
//             ]
//           }
//         }
//       }
//     ]
//   }
  
//   // 设置图表选项
//   chartInstance.value.setOption(option)
  
//   // 窗口大小变化时调整图表大小
//   window.addEventListener('resize', () => {
//     chartInstance.value && chartInstance.value.resize()
//   })
// }
</script>

<style scoped>
/* 资源记录页面特定样式 */
.refresh-controls {
  display: flex;
  align-items: center;
}

.refresh-info {
  margin: 0 10px;
  color: var(--text-secondary);
  font-size: 14px;
}

/* 保持其他特定样式不变 */
.resource-record {
  padding: 20px;
}

.record-card {
  max-width: 1400px;
  margin: 0 auto;
}

.record-header {
  text-align: center;
}

.record-header h2 {
  margin: 0 0 10px 0;
  color: var(--el-text-color-primary);
}

.record-header p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.record-content {
  padding: 20px 0;
}

.auto-refresh {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 20px;
  padding: 10px 20px;
  background: var(--el-bg-color-page);
  border-radius: 8px;
}

.refresh-info {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.filter-section {
  margin-bottom: 20px;
  padding: 20px;
  background: var(--el-bg-color-page);
  border-radius: 8px;
}

.filter-form {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.stats-section {
  margin-bottom: 20px;
}

.stats-card {
  text-align: center;
}

.stats-item {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15px;
}

.stats-icon {
  font-size: 24px;
}

.stats-content {
  text-align: left;
}

.stats-value {
  font-size: 24px;
  font-weight: bold;
  color: var(--el-text-color-primary);
}

.stats-label {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.record-list {
  margin-bottom: 20px;
}

.data-comparison {
  max-width: 300px;
}

.old-data,
.new-data {
  margin-bottom: 8px;
}

.old-data strong {
  color: var(--el-color-danger);
}

.new-data strong {
  color: var(--el-color-success);
}

.old-data pre,
.new-data pre {
  margin: 5px 0;
  font-size: 12px;
  background: var(--el-bg-color-page);
  padding: 5px;
  border-radius: 4px;
  max-height: 100px;
  overflow-y: auto;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.batch-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 15px;
  background: var(--el-bg-color-page);
  border-radius: 8px;
  margin-top: 20px;
}

.selected-count {
  margin-left: auto;
  color: var(--el-text-color-secondary);
}

.record-detail {
  padding: 20px;
}

.detail-item {
  display: flex;
  margin-bottom: 15px;
  align-items: flex-start;
}

.detail-item label {
  width: 100px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.detail-item span {
  flex: 1;
  color: var(--el-text-color-regular);
}

.data-box {
  flex: 1;
  padding: 10px;
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
}

.old-data-box {
  background: var(--el-color-danger-light-9);
  border: 1px solid var(--el-color-danger-light-5);
}

.new-data-box {
  background: var(--el-color-success-light-9);
  border: 1px solid var(--el-color-success-light-5);
}

.data-box pre {
  margin: 0;
  font-size: 12px;
  line-height: 1.4;
}
</style> 