<template>
  <div class="elder-comments-page">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <h2>评论管理</h2>
        <div class="header-tabs">
          <el-radio-group v-model="currentTab" @change="handleTabChange">
            <el-radio-button label="my">我的评论</el-radio-button>
            <el-radio-button label="all">全站评论</el-radio-button>
          </el-radio-group>
        </div>
      </div>
    </div>

    <!-- 统计信息 -->
    <el-row :gutter="20" class="stats-row">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#409EFF"><ChatDotRound /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.total }}</div>
              <div class="stat-label">{{ currentTab === 'my' ? '我的评论' : '总评论数' }}</div>
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
              <div class="stat-number">{{ stats.active }}</div>
              <div class="stat-label">正常显示</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#E6A23C"><Hide /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.hidden }}</div>
              <div class="stat-label">已隐藏</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#F56C6C"><Star /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ stats.totalLikes }}</div>
              <div class="stat-label">总点赞数</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 筛选和搜索 -->
    <el-card shadow="hover" class="filter-card">
      <el-form :model="searchForm" :inline="true" class="search-form">
        <el-form-item label="状态筛选:">
          <el-select v-model="searchForm.status" placeholder="全部状态" clearable style="width: 120px">
            <el-option label="全部" value="" />
            <el-option label="正常" value="Active" />
            <el-option label="已隐藏" value="Hidden" />
          </el-select>
        </el-form-item>
        <el-form-item label="资源筛选:" v-if="currentTab === 'all'">
          <el-input 
            v-model="searchForm.resource_name" 
            placeholder="输入资源名称" 
            clearable 
            style="width: 160px"
            @keyup.enter="loadComments"
          />
        </el-form-item>
        <el-form-item label="用户筛选:" v-if="currentTab === 'all'">
          <el-input 
            v-model="searchForm.username" 
            placeholder="输入用户名" 
            clearable 
            style="width: 140px"
            @keyup.enter="loadComments"
          />
        </el-form-item>
        <el-form-item label="内容搜索:">
          <el-input 
            v-model="searchForm.search" 
            placeholder="搜索评论内容" 
            clearable 
            style="width: 200px"
            @keyup.enter="loadComments"
          />
        </el-form-item>
        <el-form-item label="时间范围:">
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
          <el-button type="primary" icon="Search" @click="loadComments">搜索</el-button>
          <el-button icon="Refresh" @click="resetSearch">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 批量操作 -->
    <el-card shadow="hover" class="batch-card" v-if="selectedComments.length > 0">
      <div class="batch-actions">
        <span class="selection-info">已选择 {{ selectedComments.length }} 条评论</span>
        <div class="action-buttons">
          <el-button type="success" icon="Select" @click="batchUpdateStatus('Active')" :loading="batchLoading">
            批量显示
          </el-button>
          <el-button type="warning" icon="Hide" @click="batchUpdateStatus('Hidden')" :loading="batchLoading">
            批量隐藏
          </el-button>
          <el-button type="danger" icon="Delete" @click="batchDelete" :loading="batchLoading">
            批量删除
          </el-button>
          <el-button @click="clearSelection">取消选择</el-button>
        </div>
      </div>
    </el-card>

    <!-- 评论列表 -->
    <el-card shadow="hover" class="comments-card">
      <el-table 
        :data="commentList" 
        v-loading="loading" 
        @selection-change="handleSelectionChange"
        row-key="id"
      >
        <el-table-column type="selection" width="55" />
        
        <el-table-column prop="content" label="评论内容" min-width="300">
          <template #default="{ row }">
            <div class="comment-content-cell">
              <div class="content" :class="{ 'hidden-content': row.status === 'Hidden' }">
                {{ row.content }}
              </div>
              <div class="comment-meta">
                <span v-if="currentTab === 'all'" class="author">
                  作者: {{ row.author_name || '未知' }}
                </span>
                <span class="likes">
                  <el-icon><Star /></el-icon>
                  {{ row.likes || 0 }}
                </span>
              </div>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="resource" label="关联资源" width="150" v-if="currentTab === 'all'">
          <template #default="{ row }">
            <el-button 
              v-if="row.resource_id"
              type="text" 
              size="small"
              @click="$router.push(`/resource/${row.resource_id}`)"
            >
              查看资源
            </el-button>
            <span v-else class="text-muted">无关联</span>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="状态" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 'Active' ? 'success' : 'warning'" size="small">
              {{ row.status === 'Active' ? '正常' : '隐藏' }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="created_at" label="发表时间" width="120">
          <template #default="{ row }">
            {{ formatDate(row.created_at) }}
          </template>
        </el-table-column>

        <el-table-column label="操作" width="180" fixed="right">
          <template #default="{ row }">
            <div class="table-actions">
              <el-button 
                v-if="currentTab === 'my' && row.status === 'Active'"
                type="text" 
                size="small" 
                icon="Edit"
                @click="openEditDialog(row)"
              >
                编辑
              </el-button>
              <el-button 
                v-if="currentTab === 'all'"
                type="text" 
                size="small" 
                :icon="row.status === 'Active' ? 'Hide' : 'View'"
                @click="toggleCommentStatus(row)"
              >
                {{ row.status === 'Active' ? '隐藏' : '显示' }}
              </el-button>
              <el-button 
                type="text" 
                size="small" 
                icon="Delete"
                @click="handleDelete(row)"
                class="delete-btn"
              >
                删除
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
          :total="totalComments"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="loadComments"
          @current-change="loadComments"
        />
      </div>
    </el-card>

    <!-- 编辑评论对话框 -->
    <el-dialog v-model="showEditDialog" title="编辑评论" width="600px" :close-on-click-modal="false">
      <el-form :model="editForm" :rules="editRules" ref="editFormRef" label-width="80px">
        <el-form-item label="评论内容" prop="content">
          <el-input 
            v-model="editForm.content" 
            type="textarea" 
            :rows="6" 
            placeholder="请输入评论内容"
            show-word-limit
            maxlength="500"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showEditDialog = false">取消</el-button>
          <el-button type="primary" @click="handleEdit" :loading="editing">
            保存修改
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
  ChatDotRound, Select, Hide, Star, Search, Refresh, Edit, Delete, View 
} from '@element-plus/icons-vue'
import { commentApi, type Comment } from '@/api/comments'
import { getUserInfo } from '@/utils/auth'

const editFormRef = ref<InstanceType<typeof ElForm> | null>(null)

// 基础数据
const loading = ref(false)
const editing = ref(false)
const batchLoading = ref(false)
const showEditDialog = ref(false)
const currentTab = ref<'my' | 'all'>('my')

const userInfo = getUserInfo()
const commentList = ref<Comment[]>([])
const selectedComments = ref<Comment[]>([])
const currentEditComment = ref<Comment | null>(null)
const dateRange = ref<[string, string] | null>(null)

// 分页数据
const pagination = reactive({
  page: 1,
  pageSize: 20
})

// 搜索表单
const searchForm = reactive({
  status: '',
  resource_name: '',
  username: '',
  search: '',
  start_date: '',
  end_date: ''
})

// 编辑表单
const editForm = reactive({
  content: ''
})

const editRules = {
  content: [
    { required: true, message: '请输入评论内容', trigger: 'blur' },
    { min: 5, max: 500, message: '评论内容长度在 5 到 500 个字符之间', trigger: 'blur' }
  ]
}

// 计算属性
const totalComments = computed(() => commentList.value.length)
const stats = computed(() => {
  const total = commentList.value.length
  const active = commentList.value.filter(c => c.status === 'Active').length
  const hidden = commentList.value.filter(c => c.status === 'Hidden').length
  const totalLikes = commentList.value.reduce((sum, c) => sum + (c.likes || 0), 0)
  
  return { total, active, hidden, totalLikes }
})

// 切换标签页
const handleTabChange = (tab: 'my' | 'all') => {
  currentTab.value = tab
  resetSearch()
}

// 加载评论列表
const loadComments = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.page,
      size: pagination.pageSize,
      ...searchForm
    }
    
    // 过滤空参数
    Object.keys(params).forEach(key => {
      if (params[key] === '' || params[key] === null) {
        delete params[key]
      }
    })

    let res
    if (currentTab.value === 'my') {
      // 获取我的评论
      if (!userInfo?.id) return
      res = await commentApi.getUserComments(userInfo.id, params)
    } else {
      // 获取全站评论
      res = await commentApi.getAllComments(params)
    }
    
    if (res.code === 0) {
      commentList.value = res.data?.list || []
    } else {
      ElMessage.error(res.message || '加载评论失败')
    }
  } catch (error) {
    console.error('加载评论列表失败:', error)
    ElMessage.error('加载评论列表失败')
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
  searchForm.resource_name = ''
  searchForm.username = ''
  searchForm.search = ''
  searchForm.start_date = ''
  searchForm.end_date = ''
  dateRange.value = null
  pagination.page = 1
  loadComments()
}

// 处理选择变化
const handleSelectionChange = (selection: Comment[]) => {
  selectedComments.value = selection
}

// 清除选择
const clearSelection = () => {
  selectedComments.value = []
}

// 打开编辑对话框
const openEditDialog = (comment: Comment) => {
  currentEditComment.value = comment
  editForm.content = comment.content
  showEditDialog.value = true
}

// 切换评论状态
const toggleCommentStatus = async (comment: Comment) => {
  const newStatus = comment.status === 'Active' ? 'Hidden' : 'Active'
  const actionText = newStatus === 'Active' ? '显示' : '隐藏'
  
  try {
    const res = await commentApi.updateComment(comment.id, { status: newStatus })
    if (res.code === 0) {
      ElMessage.success(`评论已${actionText}`)
      loadComments()
    } else {
      ElMessage.error(res.message || `${actionText}失败`)
    }
  } catch (error) {
    console.error(`切换评论状态失败:`, error)
    ElMessage.error(`${actionText}失败`)
  }
}

// 批量更新状态
const batchUpdateStatus = async (status: 'Active' | 'Hidden') => {
  if (selectedComments.value.length === 0) {
    ElMessage.warning('请先选择要操作的评论')
    return
  }

  const actionText = status === 'Active' ? '显示' : '隐藏'
  
  try {
    await ElMessageBox.confirm(
      `确定要批量${actionText}选中的 ${selectedComments.value.length} 条评论吗？`,
      `确认批量${actionText}`,
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    batchLoading.value = true
    const commentIds = selectedComments.value.map(c => c.id)
    
    const res = await commentApi.batchUpdateStatus(commentIds, status)
    if (res.code === 0) {
      ElMessage.success(`批量${actionText}成功`)
      clearSelection()
      loadComments()
    } else {
      ElMessage.error(res.message || `批量${actionText}失败`)
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('批量更新状态失败:', error)
      ElMessage.error(`批量${actionText}失败`)
    }
  } finally {
    batchLoading.value = false
  }
}

// 批量删除
const batchDelete = async () => {
  if (selectedComments.value.length === 0) {
    ElMessage.warning('请先选择要删除的评论')
    return
  }

  try {
    await ElMessageBox.confirm(
      `确定要批量删除选中的 ${selectedComments.value.length} 条评论吗？此操作不可恢复。`,
      '确认批量删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    batchLoading.value = true
    const commentIds = selectedComments.value.map(c => c.id)
    
    const res = await commentApi.batchDeleteComments(commentIds)
    if (res.code === 0) {
      ElMessage.success('批量删除成功')
      clearSelection()
      loadComments()
    } else {
      ElMessage.error(res.message || '批量删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('批量删除失败:', error)
      ElMessage.error('批量删除失败')
    }
  } finally {
    batchLoading.value = false
  }
}

// 处理编辑
const handleEdit = async () => {
  if (!editFormRef.value || !currentEditComment.value) return
  
  try {
    await editFormRef.value.validate()
    editing.value = true
    
    const res = await commentApi.updateComment(currentEditComment.value.id, {
      content: editForm.content
    })
    
    if (res.code === 0) {
      ElMessage.success('评论更新成功')
      showEditDialog.value = false
      loadComments()
    } else {
      ElMessage.error(res.message || '更新失败')
    }
  } catch (error) {
    console.error('更新评论失败:', error)
    ElMessage.error('更新失败')
  } finally {
    editing.value = false
  }
}

// 处理删除
const handleDelete = async (comment: Comment) => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这条评论吗？此操作不可恢复。',
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    
    const res = await commentApi.deleteComment(comment.id)
    if (res.code === 0) {
      ElMessage.success('删除成功')
      loadComments()
    } else {
      ElMessage.error(res.message || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除评论失败:', error)
      ElMessage.error('删除失败')
    }
  }
}

// 格式化日期
const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  // 如果是今天
  if (diff < 86400000 && date.getDate() === now.getDate()) {
    return date.toLocaleTimeString('zh-CN', { 
      hour: '2-digit', 
      minute: '2-digit' 
    })
  }
  
  // 如果是本年
  if (date.getFullYear() === now.getFullYear()) {
    return date.toLocaleDateString('zh-CN', { 
      month: 'short', 
      day: 'numeric'
    })
  }
  
  return date.toLocaleDateString('zh-CN')
}

onMounted(() => {
  loadComments()
})
</script>

<style scoped>
.elder-comments-page {
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
.comments-card {
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

.comment-content-cell .content {
  color: #303133;
  line-height: 1.6;
  margin-bottom: 8px;
  word-break: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.comment-content-cell .content.hidden-content {
  opacity: 0.6;
  text-decoration: line-through;
}

.comment-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: #909399;
}

.author {
  color: #606266;
}

.likes {
  display: flex;
  align-items: center;
  gap: 4px;
}

.table-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.delete-btn {
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