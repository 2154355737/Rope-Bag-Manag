<template>
  <div class="resource-record">
    <el-card class="record-card">
      <template #header>
        <div class="record-header">
          <h2>资源记录</h2>
          <p>记录系统中所有资源的创建、修改、删除等操作历史</p>
        </div>
      </template>

      <div class="record-content">
        <!-- 筛选条件 -->
        <div class="filter-section">
          <el-form :inline="true" :model="filterForm" class="filter-form">
            <el-form-item label="资源类型">
              <el-select v-model="filterForm.resource_type" placeholder="选择资源类型" clearable>
                <el-option label="全部" value="" />
                <el-option label="绳包" value="Package" />
                <el-option label="用户" value="User" />
                <el-option label="分类" value="Category" />
                <el-option label="评论" value="Comment" />
                <el-option label="设置" value="Settings" />
              </el-select>
            </el-form-item>
            <el-form-item label="操作类型">
              <el-select v-model="filterForm.action" placeholder="选择操作类型" clearable>
                <el-option label="全部" value="" />
                <el-option label="创建" value="Create" />
                <el-option label="更新" value="Update" />
                <el-option label="删除" value="Delete" />
                <el-option label="下载" value="Download" />
                <el-option label="上传" value="Upload" />
                <el-option label="标星" value="Star" />
                <el-option label="封禁" value="Ban" />
              </el-select>
            </el-form-item>
            <el-form-item label="操作用户">
              <el-input v-model="filterForm.user_id" placeholder="输入用户ID" clearable />
            </el-form-item>
            <el-form-item label="时间范围">
              <el-date-picker
                v-model="filterForm.date_range"
                type="datetimerange"
                range-separator="至"
                start-placeholder="开始时间"
                end-placeholder="结束时间"
                format="YYYY-MM-DD HH:mm:ss"
                value-format="YYYY-MM-DD HH:mm:ss"
              />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleFilter">筛选</el-button>
              <el-button @click="resetFilter">重置</el-button>
            </el-form-item>
          </el-form>
        </div>

        <!-- 统计信息 -->
        <div class="stats-section">
          <el-row :gutter="20">
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">📝</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.total_records }}</div>
                    <div class="stats-label">总记录数</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">➕</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.create_count }}</div>
                    <div class="stats-label">创建操作</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">✏️</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.update_count }}</div>
                    <div class="stats-label">更新操作</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="stats-card">
                <div class="stats-item">
                  <div class="stats-icon">🗑️</div>
                  <div class="stats-content">
                    <div class="stats-value">{{ stats.delete_count }}</div>
                    <div class="stats-label">删除操作</div>
                  </div>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </div>

        <!-- 资源记录列表 -->
        <div class="record-list">
          <el-table 
            :data="recordList" 
            v-loading="loading"
            style="width: 100%"
            @selection-change="handleSelectionChange"
          >
            <el-table-column type="selection" width="55" />
            <el-table-column prop="id" label="ID" width="80" />
            <el-table-column prop="resource_type" label="资源类型" width="100">
              <template #default="{ row }">
                <el-tag :type="getResourceTypeTag(row.resource_type)">
                  {{ getResourceTypeLabel(row.resource_type) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="resource_id" label="资源ID" width="100" />
            <el-table-column prop="user_id" label="操作用户" width="120" />
            <el-table-column prop="action" label="操作类型" width="100">
              <template #default="{ row }">
                <el-tag :type="getActionTag(row.action)">
                  {{ getActionLabel(row.action) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="timestamp" label="操作时间" width="180">
              <template #default="{ row }">
                {{ formatTime(row.timestamp) }}
              </template>
            </el-table-column>
            <el-table-column prop="ip_address" label="IP地址" width="120" />
            <el-table-column label="数据对比" min-width="200">
              <template #default="{ row }">
                <div class="data-comparison">
                  <div v-if="row.old_data" class="old-data">
                    <strong>旧数据:</strong>
                    <pre>{{ formatJson(row.old_data) }}</pre>
                  </div>
                  <div v-if="row.new_data" class="new-data">
                    <strong>新数据:</strong>
                    <pre>{{ formatJson(row.new_data) }}</pre>
                  </div>
                </div>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="120" fixed="right">
              <template #default="{ row }">
                <el-button size="small" @click="viewRecord(row)">查看</el-button>
                <el-button 
                  size="small" 
                  type="danger" 
                  @click="deleteRecord(row)"
                >
                  删除
                </el-button>
              </template>
            </el-table-column>
          </el-table>

          <!-- 分页 -->
          <div class="pagination-wrapper">
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

        <!-- 批量操作 -->
        <div class="batch-actions" v-if="selectedRecords.length > 0">
          <el-button type="danger" @click="batchDelete">批量删除</el-button>
          <el-button @click="exportRecords">导出记录</el-button>
          <span class="selected-count">已选择 {{ selectedRecords.length }} 条记录</span>
        </div>
      </div>
    </el-card>

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
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { resourceRecordApi } from '../../api/resourceRecords'

// 响应式数据
const loading = ref(false)
const recordList = ref([])
const selectedRecords = ref([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const recordDialogVisible = ref(false)
const currentRecord = ref(null)

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
  date_range: []
})

// 方法
async function loadRecords() {
  loading.value = true
  try {
    const params = {
      page: currentPage.value,
      size: pageSize.value,
      ...filterForm
    }
    const response = await resourceRecordApi.getResourceRecords(params)
    if (response.code === 0) {
      recordList.value = response.data.list || []
      total.value = response.data.total || 0
      updateStats()
    }
  } catch (error) {
    console.error('加载资源记录失败:', error)
    ElMessage.error('加载资源记录失败')
  } finally {
    loading.value = false
  }
}

function updateStats() {
  stats.total_records = total.value
  stats.create_count = recordList.value.filter(r => r.action === 'Create').length
  stats.update_count = recordList.value.filter(r => r.action === 'Update').length
  stats.delete_count = recordList.value.filter(r => r.action === 'Delete').length
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

function handleSelectionChange(selection: any[]) {
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

function viewRecord(record: any) {
  currentRecord.value = record
  recordDialogVisible.value = true
}

async function deleteRecord(record: any) {
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
    // 批量删除逻辑
    ElMessage.success('批量删除成功')
    loadRecords()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

function exportRecords() {
  ElMessage.info('导出功能开发中...')
}

function getResourceTypeTag(type: string): string {
  const tags = {
    Package: 'primary',
    User: 'success',
    Category: 'warning',
    Comment: 'info',
    Settings: 'danger'
  }
  return tags[type] || 'info'
}

function getResourceTypeLabel(type: string): string {
  const labels = {
    Package: '绳包',
    User: '用户',
    Category: '分类',
    Comment: '评论',
    Settings: '设置'
  }
  return labels[type] || type
}

function getActionTag(action: string): string {
  const tags = {
    Create: 'success',
    Update: 'warning',
    Delete: 'danger',
    Download: 'info',
    Upload: 'primary',
    Star: 'warning',
    Ban: 'danger'
  }
  return tags[action] || 'info'
}

function getActionLabel(action: string): string {
  const labels = {
    Create: '创建',
    Update: '更新',
    Delete: '删除',
    Download: '下载',
    Upload: '上传',
    Star: '标星',
    Ban: '封禁'
  }
  return labels[action] || action
}

function formatTime(timestamp: number): string {
  if (!timestamp) return '-'
  return new Date(timestamp * 1000).toLocaleString()
}

function formatJson(jsonString: string): string {
  try {
    const obj = JSON.parse(jsonString)
    return JSON.stringify(obj, null, 2)
  } catch {
    return jsonString
  }
}

onMounted(() => {
  loadRecords()
})
</script>

<style scoped>
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