<template>
  <div class="elder-resource-manage">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <h2>资源审核管理</h2>
        <div class="header-actions">
          <el-button type="primary" icon="Refresh" @click="loadResources">
            刷新列表
          </el-button>
        </div>
      </div>
    </div>

    <!-- 审核统计 -->
    <el-row :gutter="20" class="stats-row">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#E6A23C"><Clock /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.pending }}</div>
              <div class="stat-label">待审核</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#67C23A"><Select /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.approved }}</div>
              <div class="stat-label">已通过</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#F56C6C"><CloseBold /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.rejected }}</div>
              <div class="stat-label">已拒绝</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#409EFF"><Document /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.total }}</div>
              <div class="stat-label">总资源数</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 筛选和搜索 -->
    <el-card shadow="hover" class="filter-card">
      <el-form :model="searchForm" :inline="true" class="search-form">
        <el-form-item label="审核状态:">
          <el-select v-model="searchForm.status" placeholder="全部状态" clearable style="width: 140px">
            <el-option label="全部" value="" />
            <el-option label="待审核" value="Pending" />
            <el-option label="已通过" value="Active" />
            <el-option label="已拒绝" value="Rejected" />
          </el-select>
        </el-form-item>
        <el-form-item label="资源名称:">
          <el-input 
            v-model="searchForm.search" 
            placeholder="输入资源名称" 
            clearable 
            style="width: 200px"
            @keyup.enter="loadResources"
          />
        </el-form-item>
        <el-form-item label="提交时间:">
          <el-date-picker
            v-model="dateRange"
            type="daterange"
            range-separator="至"
            start-placeholder="开始日期"
            end-placeholder="结束日期"
            style="width: 240px"
            format="YYYY-MM-DD"
            value-format="YYYY-MM-DD"
            @change="handleDateChange"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" icon="Search" @click="loadResources">搜索</el-button>
          <el-button icon="Refresh" @click="resetSearch">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 批量操作 -->
    <el-card shadow="hover" class="batch-card" v-if="selectedResources.length > 0">
      <div class="batch-actions">
        <span class="selection-info">已选择 {{ selectedResources.length }} 个资源</span>
        <div class="action-buttons">
          <el-button type="success" icon="Select" @click="batchReview('approved')" :loading="batchLoading">
            批量通过
          </el-button>
          <el-button type="danger" icon="CloseBold" @click="batchReview('rejected')" :loading="batchLoading">
            批量拒绝
          </el-button>
          <el-button @click="clearSelection">取消选择</el-button>
        </div>
      </div>
    </el-card>

    <!-- 资源列表 -->
    <el-card shadow="hover" class="resource-list-card">
      <el-table 
        :data="resourceList" 
        v-loading="loading" 
        @selection-change="handleSelectionChange"
        row-key="id"
      >
        <el-table-column type="selection" width="55" />
        
        <el-table-column prop="name" label="资源名称" min-width="200">
          <template #default="{ row }">
            <div class="resource-name-cell">
              <div class="name">{{ row.name }}</div>
              <div class="author">作者: {{ row.author }}</div>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="version" label="版本" width="100">
          <template #default="{ row }">
            <span>{{ row.version || '未设置' }}</span>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)" size="small">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="created_at" label="提交时间" width="120">
          <template #default="{ row }">
            {{ formatDate(row.created_at) }}
          </template>
        </el-table-column>

        <el-table-column prop="reviewed_at" label="审核时间" width="120">
          <template #default="{ row }">
            <span v-if="row.reviewed_at">{{ formatDate(row.reviewed_at) }}</span>
            <span v-else class="text-muted">未审核</span>
          </template>
        </el-table-column>

        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <div class="table-actions">
              <el-button 
                type="text" 
                size="small" 
                @click="viewResource(row)"
                icon="View"
              >
                查看
              </el-button>
              <el-button 
                v-if="row.status === 'Pending'"
                type="text" 
                size="small" 
                @click="openReviewDialog(row)"
                icon="Edit"
              >
                审核
              </el-button>
              <el-button 
                v-if="row.status === 'Pending'"
                type="text" 
                size="small" 
                @click="quickReview(row, 'approved')"
                class="approve-btn"
              >
                通过
              </el-button>
              <el-button 
                v-if="row.status === 'Pending'"
                type="text" 
                size="small" 
                @click="quickReview(row, 'rejected')"
                class="reject-btn"
              >
                拒绝
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-wrapper">
        <el-pagination
          v-model:current-page="pagination.page"
          v-model:page-size="pagination.pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="totalResources"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="loadResources"
          @current-change="loadResources"
        />
      </div>
    </el-card>

    <!-- 资源详情对话框 -->
    <el-dialog v-model="showDetailDialog" title="资源详情" width="700px">
      <div v-if="currentResource" class="resource-detail">
        <div class="detail-section">
          <h4>基本信息</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="label">资源名称:</span>
              <span class="value">{{ currentResource.name }}</span>
            </div>
            <div class="detail-item">
              <span class="label">作者:</span>
              <span class="value">{{ currentResource.author }}</span>
            </div>
            <div class="detail-item">
              <span class="label">版本:</span>
              <span class="value">{{ currentResource.version || '未设置' }}</span>
            </div>
            <div class="detail-item">
              <span class="label">状态:</span>
              <el-tag :type="getStatusType(currentResource.status)">
                {{ getStatusText(currentResource.status) }}
              </el-tag>
            </div>
          </div>
        </div>

        <div class="detail-section">
          <h4>描述信息</h4>
          <div class="description-content">
            {{ currentResource.description || '暂无描述' }}
          </div>
        </div>

        <div class="detail-section">
          <h4>文件信息</h4>
          <div class="file-info">
            <div class="file-link">
              <span class="label">下载链接:</span>
              <el-link :href="currentResource.file_url" target="_blank" type="primary">
                {{ currentResource.file_url }}
              </el-link>
            </div>
            <div v-if="currentResource.file_size" class="file-size">
              <span class="label">文件大小:</span>
              <span class="value">{{ formatFileSize(currentResource.file_size) }}</span>
            </div>
          </div>
        </div>

        <div class="detail-section" v-if="currentResource.review_comment">
          <h4>审核意见</h4>
          <div class="review-comment">
            {{ currentResource.review_comment }}
          </div>
        </div>
      </div>
    </el-dialog>

    <!-- 审核对话框 -->
    <el-dialog v-model="showReviewDialog" title="资源审核" width="600px" :close-on-click-modal="false">
      <div v-if="currentResource">
        <div class="review-resource-info">
          <h4>{{ currentResource.name }}</h4>
          <p class="resource-summary">
            作者: {{ currentResource.author }} | 
            版本: {{ currentResource.version || '未设置' }} | 
            提交时间: {{ formatDate(currentResource.created_at) }}
          </p>
        </div>

        <el-form :model="reviewForm" :rules="reviewRules" ref="reviewFormRef" label-width="80px">
          <el-form-item label="审核结果" prop="status">
            <el-radio-group v-model="reviewForm.status">
              <el-radio label="approved">
                <el-icon color="#67C23A"><Select /></el-icon>
                通过
              </el-radio>
              <el-radio label="rejected">
                <el-icon color="#F56C6C"><CloseBold /></el-icon>
                拒绝
              </el-radio>
            </el-radio-group>
          </el-form-item>
          <el-form-item label="审核意见" prop="comment">
            <el-input 
              v-model="reviewForm.comment" 
              type="textarea" 
              :rows="4" 
              placeholder="请输入审核意见（可选）"
              show-word-limit
              maxlength="200"
            />
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showReviewDialog = false">取消</el-button>
          <el-button type="primary" @click="handleSubmitReview" :loading="reviewing">
            提交审核
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox, ElForm } from 'element-plus'
import { 
  Clock, Select, CloseBold, Document, Search, Refresh, View, Edit 
} from '@element-plus/icons-vue'
import { packageApi, type Package, type ReviewResourceRequest } from '@/api/packages'

const reviewFormRef = ref<InstanceType<typeof ElForm> | null>(null)

// 基础数据
const loading = ref(false)
const reviewing = ref(false)
const batchLoading = ref(false)
const showDetailDialog = ref(false)
const showReviewDialog = ref(false)

const resourceList = ref<Package[]>([])
const currentResource = ref<Package | null>(null)
const selectedResources = ref<Package[]>([])
const dateRange = ref<[string, string] | null>(null)

// 分页数据
const pagination = reactive({
  page: 1,
  pageSize: 20
})

// 搜索表单
const searchForm = reactive({
  status: '',
  search: '',
  start_date: '',
  end_date: ''
})

// 审核表单
const reviewForm = reactive<ReviewResourceRequest>({
  status: 'approved',
  comment: ''
})

const reviewRules = {
  status: [
    { required: true, message: '请选择审核结果', trigger: 'change' }
  ]
}

// 计算属性
const totalResources = computed(() => resourceList.value.length)
const stats = computed(() => {
  const pending = resourceList.value.filter(r => r.status === 'Pending').length
  const approved = resourceList.value.filter(r => r.status === 'Active').length
  const rejected = resourceList.value.filter(r => r.status === 'Rejected').length
  const total = resourceList.value.length
  
  return { pending, approved, rejected, total }
})

// 加载资源列表
const loadResources = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.page,
      pageSize: pagination.pageSize,
      ...searchForm
    }
    
    // 过滤空参数
    Object.keys(params).forEach(key => {
      if (params[key] === '' || params[key] === null) {
        delete params[key]
      }
    })

    // 如果有状态筛选，使用普通资源接口，否则使用待审核接口
    const res = searchForm.status 
      ? await packageApi.getPackages(params)
      : await packageApi.getPendingResources(params)
      
    if (res.code === 0) {
      resourceList.value = res.data?.list || []
    } else {
      ElMessage.error(res.message || '加载资源列表失败')
    }
  } catch (error) {
    console.error('加载资源列表失败:', error)
    ElMessage.error('加载资源列表失败')
  } finally {
    loading.value = false
  }
}

// 处理日期范围变化
const handleDateChange = (dates: [string, string] | null) => {
  if (dates && dates.length === 2) {
    searchForm.start_date = dates[0]
    searchForm.end_date = dates[1]
  } else {
    searchForm.start_date = ''
    searchForm.end_date = ''
  }
}

// 重置搜索
const resetSearch = () => {
  searchForm.status = ''
  searchForm.search = ''
  searchForm.start_date = ''
  searchForm.end_date = ''
  dateRange.value = null
  pagination.page = 1
  loadResources()
}

// 处理选择变化
const handleSelectionChange = (selection: Package[]) => {
  selectedResources.value = selection
}

// 清除选择
const clearSelection = () => {
  selectedResources.value = []
}

// 查看资源详情
const viewResource = (resource: Package) => {
  currentResource.value = resource
  showDetailDialog.value = true
}

// 打开审核对话框
const openReviewDialog = (resource: Package) => {
  currentResource.value = resource
  reviewForm.status = 'approved'
  reviewForm.comment = ''
  showReviewDialog.value = true
}

// 快速审核
const quickReview = async (resource: Package, status: 'approved' | 'rejected') => {
  try {
    const res = await packageApi.reviewResource(resource.id, { status })
    if (res.code === 0) {
      ElMessage.success(status === 'approved' ? '审核通过' : '审核拒绝')
      loadResources()
    } else {
      ElMessage.error(res.message || '审核失败')
    }
  } catch (error) {
    console.error('快速审核失败:', error)
    ElMessage.error('审核失败')
  }
}

// 批量审核
const batchReview = async (status: 'approved' | 'rejected') => {
  if (selectedResources.value.length === 0) {
    ElMessage.warning('请先选择要审核的资源')
    return
  }

  try {
    await ElMessageBox.confirm(
      `确定要${status === 'approved' ? '批量通过' : '批量拒绝'}选中的 ${selectedResources.value.length} 个资源吗？`,
      '确认批量审核',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    batchLoading.value = true
    const promises = selectedResources.value.map(resource => 
      packageApi.reviewResource(resource.id, { status })
    )
    
    await Promise.all(promises)
    ElMessage.success(`批量${status === 'approved' ? '通过' : '拒绝'}成功`)
    clearSelection()
    loadResources()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('批量审核失败:', error)
      ElMessage.error('批量审核失败')
    }
  } finally {
    batchLoading.value = false
  }
}

// 提交审核
const handleSubmitReview = async () => {
  if (!reviewFormRef.value || !currentResource.value) return
  
  try {
    await reviewFormRef.value.validate()
    reviewing.value = true
    
    const res = await packageApi.reviewResource(currentResource.value.id, reviewForm)
    if (res.code === 0) {
      ElMessage.success('审核完成')
      showReviewDialog.value = false
      loadResources()
    } else {
      ElMessage.error(res.message || '审核失败')
    }
  } catch (error) {
    console.error('提交审核失败:', error)
    ElMessage.error('审核失败')
  } finally {
    reviewing.value = false
  }
}

// 获取状态类型
const getStatusType = (status: string) => {
  const statusMap: Record<string, string> = {
    'Active': 'success',
    'Pending': 'warning',
    'Rejected': 'danger',
    'Inactive': 'info'
  }
  return statusMap[status] || 'info'
}

// 获取状态文本
const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'Active': '已通过',
    'Pending': '待审核',
    'Rejected': '已拒绝',
    'Inactive': '已下架'
  }
  return statusMap[status] || status
}

// 格式化日期
const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('zh-CN')
}

// 格式化文件大小
const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

onMounted(() => {
  loadResources()
})
</script>

<style scoped>
.elder-resource-manage {
  padding: 24px;
  background-color: #f5f7fa;
  min-height: calc(100vh - 60px);
}

.page-header {
  margin-bottom: 24px;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-content h2 {
  margin: 0;
  color: #303133;
  font-size: 28px;
  font-weight: 600;
}

.stats-row {
  margin-bottom: 24px;
}

.stat-card {
  border-radius: 8px;
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 8px;
  background-color: #f0f9ff;
}

.stat-icon .el-icon {
  font-size: 24px;
}

.stat-info {
  flex: 1;
}

.stat-number {
  font-size: 24px;
  font-weight: 600;
  color: #303133;
  line-height: 1;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: #909399;
}

.filter-card,
.batch-card,
.resource-list-card {
  margin-bottom: 24px;
  border-radius: 8px;
}

.search-form {
  margin: 0;
}

.batch-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.selection-info {
  color: #409EFF;
  font-weight: 500;
}

.action-buttons {
  display: flex;
  gap: 12px;
}

.resource-name-cell .name {
  font-weight: 500;
  color: #303133;
  margin-bottom: 4px;
}

.resource-name-cell .author {
  font-size: 12px;
  color: #909399;
}

.table-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.approve-btn {
  color: #67C23A !important;
}

.reject-btn {
  color: #F56C6C !important;
}

.text-muted {
  color: #c0c4cc;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  padding: 24px 0;
}

.resource-detail {
  max-height: 600px;
  overflow-y: auto;
}

.detail-section {
  margin-bottom: 24px;
}

.detail-section h4 {
  margin: 0 0 12px 0;
  color: #303133;
  font-size: 16px;
  font-weight: 600;
  border-bottom: 1px solid #e4e7ed;
  padding-bottom: 8px;
}

.detail-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.detail-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.detail-item .label {
  font-weight: 500;
  color: #606266;
  min-width: 80px;
}

.detail-item .value {
  color: #303133;
  flex: 1;
}

.description-content {
  color: #303133;
  line-height: 1.6;
  padding: 12px;
  background-color: #f8f9fa;
  border-radius: 6px;
  word-break: break-word;
}

.file-info {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.file-link,
.file-size {
  display: flex;
  align-items: center;
  gap: 8px;
}

.review-comment {
  color: #303133;
  line-height: 1.6;
  padding: 12px;
  background-color: #f0f9ff;
  border-radius: 6px;
  border-left: 4px solid #409EFF;
}

.review-resource-info {
  margin-bottom: 20px;
  padding: 16px;
  background-color: #f8f9fa;
  border-radius: 6px;
}

.review-resource-info h4 {
  margin: 0 0 8px 0;
  color: #303133;
}

.resource-summary {
  margin: 0;
  color: #606266;
  font-size: 14px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

@media screen and (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }
  
  .detail-grid {
    grid-template-columns: 1fr;
  }
  
  .batch-actions {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .search-form .el-form-item {
    margin-bottom: 12px;
  }
}
</style> 