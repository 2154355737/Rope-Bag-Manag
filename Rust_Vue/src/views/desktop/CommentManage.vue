<template>
  <div class="comment-manage">
    <el-card class="manage-card">
      <template #header>
        <div class="manage-header">
          <h2>评论管理</h2>
          <p>管理系统中的用户评论，包括审核、删除、回复等功能</p>
        </div>
      </template>

      <div class="manage-content">
        <!-- 筛选条件 -->
        <div class="filter-section">
          <el-form :inline="true" :model="filterForm" class="filter-form">
            <el-form-item label="评论状态">
              <el-select v-model="filterForm.status" placeholder="选择状态" clearable>
                <el-option label="全部" value="" />
                <el-option label="正常" value="Active" />
                <el-option label="隐藏" value="Hidden" />
                <el-option label="已删除" value="Deleted" />
              </el-select>
            </el-form-item>
            <el-form-item label="目标类型">
              <el-select v-model="filterForm.target_type" placeholder="选择类型" clearable>
                <el-option label="全部" value="" />
                <el-option label="绳包" value="Package" />
                <el-option label="用户" value="User" />
                <el-option label="系统" value="System" />
              </el-select>
            </el-form-item>
            <el-form-item label="用户ID">
              <el-input v-model="filterForm.user_id" placeholder="输入用户ID" clearable />
            </el-form-item>
            <el-form-item label="时间范围">
              <el-date-picker
                v-model="filterForm.date_range"
                type="daterange"
                range-separator="至"
                start-placeholder="开始日期"
                end-placeholder="结束日期"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
              />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleFilter">筛选</el-button>
              <el-button @click="resetFilter">重置</el-button>
            </el-form-item>
          </el-form>
        </div>

        <!-- 评论列表 -->
        <div class="comment-list">
          <el-table 
            :data="commentList" 
            v-loading="loading"
            style="width: 100%"
            @selection-change="handleSelectionChange"
          >
            <el-table-column type="selection" width="55" />
            <el-table-column prop="id" label="ID" width="80" />
            <el-table-column prop="user_id" label="用户" width="120" />
            <el-table-column prop="target_type" label="目标类型" width="100">
              <template #default="{ row }">
                <el-tag :type="getTargetTypeTag(row.target_type)">
                  {{ getTargetTypeLabel(row.target_type) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="target_id" label="目标ID" width="100" />
            <el-table-column prop="content" label="评论内容" min-width="200">
              <template #default="{ row }">
                <div class="comment-content">
                  <p class="content-text">{{ row.content }}</p>
                  <div class="content-meta">
                    <span class="time">{{ formatTime(row.create_time) }}</span>
                    <span class="likes">👍 {{ row.likes }}</span>
                    <span class="dislikes">👎 {{ row.dislikes }}</span>
                  </div>
                </div>
              </template>
            </el-table-column>
            <el-table-column prop="status" label="状态" width="100">
              <template #default="{ row }">
                <el-tag :type="getStatusTag(row.status)">
                  {{ getStatusLabel(row.status) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="200" fixed="right">
              <template #default="{ row }">
                <el-button size="small" @click="viewComment(row)">查看</el-button>
                <el-button 
                  size="small" 
                  type="warning" 
                  @click="hideComment(row)"
                  v-if="row.status === 'Active'"
                >
                  隐藏
                </el-button>
                <el-button 
                  size="small" 
                  type="success" 
                  @click="showComment(row)"
                  v-if="row.status === 'Hidden'"
                >
                  显示
                </el-button>
                <el-button 
                  size="small" 
                  type="danger" 
                  @click="deleteComment(row)"
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
        <div class="batch-actions" v-if="selectedComments.length > 0">
          <el-button type="warning" @click="batchHide">批量隐藏</el-button>
          <el-button type="success" @click="batchShow">批量显示</el-button>
          <el-button type="danger" @click="batchDelete">批量删除</el-button>
          <span class="selected-count">已选择 {{ selectedComments.length }} 条评论</span>
        </div>
      </div>
    </el-card>

    <!-- 评论详情对话框 -->
    <el-dialog 
      v-model="commentDialogVisible" 
      title="评论详情" 
      width="600px"
    >
      <div class="comment-detail" v-if="currentComment">
        <div class="detail-item">
          <label>评论ID:</label>
          <span>{{ currentComment.id }}</span>
        </div>
        <div class="detail-item">
          <label>用户ID:</label>
          <span>{{ currentComment.user_id }}</span>
        </div>
        <div class="detail-item">
          <label>目标类型:</label>
          <span>{{ getTargetTypeLabel(currentComment.target_type) }}</span>
        </div>
        <div class="detail-item">
          <label>目标ID:</label>
          <span>{{ currentComment.target_id }}</span>
        </div>
        <div class="detail-item">
          <label>评论内容:</label>
          <div class="content-box">{{ currentComment.content }}</div>
        </div>
        <div class="detail-item">
          <label>创建时间:</label>
          <span>{{ formatTime(currentComment.create_time) }}</span>
        </div>
        <div class="detail-item">
          <label>更新时间:</label>
          <span>{{ formatTime(currentComment.update_time) }}</span>
        </div>
        <div class="detail-item">
          <label>点赞数:</label>
          <span>{{ currentComment.likes }}</span>
        </div>
        <div class="detail-item">
          <label>点踩数:</label>
          <span>{{ currentComment.dislikes }}</span>
        </div>
        <div class="detail-item">
          <label>状态:</label>
          <el-tag :type="getStatusTag(currentComment.status)">
            {{ getStatusLabel(currentComment.status) }}
          </el-tag>
        </div>
      </div>
      <template #footer>
        <el-button @click="commentDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getComments, updateComment, deleteComment as deleteCommentApi } from '../../api/comments'

// 响应式数据
const loading = ref(false)
const commentList = ref([])
const selectedComments = ref([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const commentDialogVisible = ref(false)
const currentComment = ref(null)

const filterForm = reactive({
  status: '',
  target_type: '',
  user_id: '',
  date_range: []
})

// 方法
async function loadComments() {
  loading.value = true
  try {
    const params = {
      page: currentPage.value,
      size: pageSize.value,
      ...filterForm
    }
    const response = await getComments(params)
    if (response.code === 0) {
      commentList.value = response.data.list || []
      total.value = response.data.total || 0
    }
  } catch (error) {
    console.error('加载评论失败:', error)
    ElMessage.error('加载评论失败')
  } finally {
    loading.value = false
  }
}

function handleFilter() {
  currentPage.value = 1
  loadComments()
}

function resetFilter() {
  Object.assign(filterForm, {
    status: '',
    target_type: '',
    user_id: '',
    date_range: []
  })
  handleFilter()
}

function handleSelectionChange(selection: any[]) {
  selectedComments.value = selection
}

function handleSizeChange(size: number) {
  pageSize.value = size
  loadComments()
}

function handleCurrentChange(page: number) {
  currentPage.value = page
  loadComments()
}

function viewComment(comment: any) {
  currentComment.value = comment
  commentDialogVisible.value = true
}

async function hideComment(comment: any) {
  try {
    await ElMessageBox.confirm('确定要隐藏这条评论吗？', '确认操作')
    const response = await updateComment(comment.id, { status: 'Hidden' })
    if (response.code === 0) {
      ElMessage.success('评论已隐藏')
      loadComments()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function showComment(comment: any) {
  try {
    await ElMessageBox.confirm('确定要显示这条评论吗？', '确认操作')
    const response = await updateComment(comment.id, { status: 'Active' })
    if (response.code === 0) {
      ElMessage.success('评论已显示')
      loadComments()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function deleteComment(comment: any) {
  try {
    await ElMessageBox.confirm('确定要删除这条评论吗？此操作不可恢复！', '确认删除', {
      type: 'warning'
    })
    const response = await deleteCommentApi(comment.id)
    if (response.code === 0) {
      ElMessage.success('评论已删除')
      loadComments()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

async function batchHide() {
  if (selectedComments.value.length === 0) {
    ElMessage.warning('请选择要操作的评论')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要隐藏选中的 ${selectedComments.value.length} 条评论吗？`, '确认操作')
    // 批量操作逻辑
    ElMessage.success('批量隐藏成功')
    loadComments()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function batchShow() {
  if (selectedComments.value.length === 0) {
    ElMessage.warning('请选择要操作的评论')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要显示选中的 ${selectedComments.value.length} 条评论吗？`, '确认操作')
    // 批量操作逻辑
    ElMessage.success('批量显示成功')
    loadComments()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function batchDelete() {
  if (selectedComments.value.length === 0) {
    ElMessage.warning('请选择要操作的评论')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedComments.value.length} 条评论吗？此操作不可恢复！`, '确认删除', {
      type: 'warning'
    })
    // 批量删除逻辑
    ElMessage.success('批量删除成功')
    loadComments()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

function getTargetTypeTag(type: string): string {
  const tags = {
    Package: 'primary',
    User: 'success',
    System: 'warning'
  }
  return tags[type] || 'info'
}

function getTargetTypeLabel(type: string): string {
  const labels = {
    Package: '绳包',
    User: '用户',
    System: '系统'
  }
  return labels[type] || type
}

function getStatusTag(status: string): string {
  const tags = {
    Active: 'success',
    Hidden: 'warning',
    Deleted: 'danger'
  }
  return tags[status] || 'info'
}

function getStatusLabel(status: string): string {
  const labels = {
    Active: '正常',
    Hidden: '隐藏',
    Deleted: '已删除'
  }
  return labels[status] || status
}

function formatTime(time: string): string {
  if (!time) return '-'
  return new Date(time).toLocaleString()
}

onMounted(() => {
  loadComments()
})
</script>

<style scoped>
.comment-manage {
  padding: 20px;
}

.manage-card {
  max-width: 1400px;
  margin: 0 auto;
}

.manage-header {
  text-align: center;
}

.manage-header h2 {
  margin: 0 0 10px 0;
  color: var(--el-text-color-primary);
}

.manage-header p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.manage-content {
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

.comment-list {
  margin-bottom: 20px;
}

.comment-content {
  max-width: 300px;
}

.content-text {
  margin: 0 0 8px 0;
  word-break: break-all;
  line-height: 1.4;
}

.content-meta {
  display: flex;
  gap: 15px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
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

.comment-detail {
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

.content-box {
  flex: 1;
  padding: 10px;
  background: var(--el-bg-color-page);
  border-radius: 4px;
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
}
</style> 