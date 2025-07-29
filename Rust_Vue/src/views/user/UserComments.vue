<template>
  <div class="user-comments-page">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <h2>我的评论</h2>
        <div class="header-stats">
          <el-tag type="success" size="large">共 {{ totalComments }} 条评论</el-tag>
        </div>
      </div>
    </div>

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
        <el-form-item label="资源筛选:">
          <el-input 
            v-model="searchForm.resource_name" 
            placeholder="输入资源名称" 
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

    <!-- 评论列表 -->
    <el-card shadow="hover" class="comments-card">
      <div v-loading="loading">
        <div v-if="commentList.length === 0 && !loading" class="empty-state">
          <el-empty description="还没有发表任何评论">
            <el-button type="primary" @click="$router.push('/home')">
              去浏览资源
            </el-button>
          </el-empty>
        </div>
        <div v-else class="comments-list">
          <div 
            v-for="comment in commentList" 
            :key="comment.id"
            class="comment-item"
            :class="{ 'hidden-comment': comment.status === 'Hidden' }"
          >
            <!-- 评论头部 -->
            <div class="comment-header">
              <div class="comment-info">
                <el-tag 
                  :type="comment.status === 'Active' ? 'success' : 'warning'" 
                  size="small"
                >
                  {{ comment.status === 'Active' ? '正常' : '已隐藏' }}
                </el-tag>
                <span class="comment-time">
                  <el-icon><Clock /></el-icon>
                  {{ formatDate(comment.created_at) }}
                </span>
                <span class="comment-resource" v-if="comment.target_title">
                  <el-icon><Document /></el-icon>
                  <span class="resource-label">所属资源：</span>
                  <el-button 
                    type="text" 
                    size="small" 
                    @click="$router.push(`/resource/${comment.target_id}`)"
                    class="resource-link"
                  >
                    {{ comment.target_title }}
                  </el-button>
                </span>
              </div>
              <div class="comment-actions">
                <el-button 
                  type="text" 
                  size="small" 
                  icon="Edit" 
                  @click="openEditDialog(comment)"
                  v-if="comment.status === 'Active'"
                >
                  编辑
                </el-button>
                <el-button 
                  type="text" 
                  size="small" 
                  icon="Delete" 
                  @click="handleDelete(comment)"
                  class="delete-btn"
                >
                  删除
                </el-button>
              </div>
            </div>

            <!-- 评论内容 -->
            <div class="comment-content">
              <div class="content-text">{{ comment.content }}</div>
            </div>

            <!-- 评论统计 -->
            <div class="comment-stats">
              <div class="stat-item">
                <el-icon><Star /></el-icon>
                <span>{{ comment.likes || 0 }} 赞</span>
              </div>
              <div class="stat-item" v-if="comment.dislikes">
                <el-icon><StarFilled /></el-icon>
                <span>{{ comment.dislikes || 0 }} 踩</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 分页 -->
        <div class="pagination-wrapper" v-if="totalComments > 0">
          <el-pagination
            v-model:current-page="pagination.page"
            v-model:page-size="pagination.pageSize"
            :page-sizes="[10, 20, 50]"
            :total="totalComments"
            layout="total, sizes, prev, pager, next, jumper"
            @size-change="loadComments"
            @current-change="loadComments"
          />
        </div>
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
import { Clock, Document, Edit, Delete, Search, Refresh, Star, StarFilled } from '@element-plus/icons-vue'
import { commentApi, type Comment } from '@/api/comments'
import { getUserInfo } from '@/utils/auth'

const editFormRef = ref<InstanceType<typeof ElForm> | null>(null)

// 基础数据
const loading = ref(false)
const editing = ref(false)
const showEditDialog = ref(false)

const userInfo = getUserInfo()
const commentList = ref<Comment[]>([])
const currentEditComment = ref<Comment | null>(null)
const dateRange = ref<[string, string] | null>(null)

// 分页数据
const pagination = reactive({
  page: 1,
  pageSize: 10
})

// 搜索表单
const searchForm = reactive({
  status: '',
  resource_name: '',
  start_date: '',
  end_date: ''
})

// 编辑表单
const editForm = reactive({
  content: ''
})

// 表单验证规则
const editRules = {
  content: [
    { required: true, message: '请输入评论内容', trigger: 'blur' },
    { min: 5, max: 500, message: '评论内容长度在 5 到 500 个字符之间', trigger: 'blur' }
  ]
}

// 计算属性
const totalComments = computed(() => commentList.value.length)

// 加载评论列表
const loadComments = async () => {
  if (!userInfo?.id) return
  
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

    const res = await commentApi.getUserComments(userInfo.id, params)
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
  searchForm.start_date = ''
  searchForm.end_date = ''
  dateRange.value = null
  pagination.page = 1
  loadComments()
}

// 打开编辑对话框
const openEditDialog = (comment: Comment) => {
  currentEditComment.value = comment
  editForm.content = comment.content
  showEditDialog.value = true
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
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  }
  
  // 其他情况显示完整日期
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

onMounted(() => {
  loadComments()
})
</script>

<style scoped>
.user-comments-page {
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

.header-stats {
  display: flex;
  align-items: center;
  gap: 12px;
}

.filter-card,
.comments-card {
  margin-bottom: 24px;
  border-radius: 8px;
}

.search-form {
  margin: 0;
}

.empty-state {
  padding: 60px 0;
  text-align: center;
}

.comments-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.comment-item {
  padding: 20px;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  background: white;
  transition: all 0.3s;
}

.comment-item:hover {
  border-color: #409EFF;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.1);
}

.comment-item.hidden-comment {
  opacity: 0.7;
  background-color: #fafafa;
}

.comment-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid #f0f0f0;
}

.comment-info {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
}

.comment-time,
.comment-resource {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #909399;
}

.comment-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.delete-btn {
  color: #f56c6c !important;
}

.comment-content {
  margin-bottom: 16px;
}

.content-text {
  color: #303133;
  font-size: 14px;
  line-height: 1.6;
  word-break: break-word;
}

.comment-resource {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #606266;
  font-size: 13px;
}

.resource-label {
  color: #909399;
}

.resource-link {
  font-weight: 500;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.comment-stats {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 12px;
  color: #909399;
  padding-top: 12px;
  border-top: 1px solid #f0f0f0;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
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
  
  .comment-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .comment-info {
    flex-wrap: wrap;
  }
  
  .search-form .el-form-item {
    margin-bottom: 12px;
  }
}
</style> 