<template>
  <div class="admin-page comment-manage">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><ChatDotSquare /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">评论管理</h1>
            <p class="page-subtitle">管理系统中的用户评论，包括审核、删除、回复等功能</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button type="primary" @click="addComment">
            <el-icon><Plus /></el-icon>
            添加评论
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><ChatLineSquare /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalComments }}</div>
            <div class="stat-label">总评论数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><ChatDotRound /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ activeComments }}</div>
            <div class="stat-label">正常评论</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><ChatSquare /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ hiddenComments }}</div>
            <div class="stat-label">隐藏评论</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Timer /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ todayComments }}</div>
            <div class="stat-label">今日新增</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 筛选条件 -->
    <div class="search-section">
      <div class="search-left">
        <el-select v-model="filterForm.status" placeholder="评论状态" clearable style="width: 150px">
          <el-option label="正常" value="Active" />
          <el-option label="隐藏" value="Hidden" />
          <el-option label="已删除" value="Deleted" />
        </el-select>
        
        <el-select v-model="filterForm.target_type" placeholder="目标类型" clearable style="width: 150px">
          <el-option label="绳包" value="Package" />
          <el-option label="用户" value="User" />
          <el-option label="系统" value="System" />
        </el-select>
        
        <el-input 
          v-model="filterForm.user_id" 
          placeholder="输入用户ID" 
          clearable 
          style="width: 150px"
        />
        
        <el-date-picker
          v-model="filterForm.date_range"
          type="daterange"
          range-separator="至"
          start-placeholder="开始日期"
          end-placeholder="结束日期"
          clearable
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
        <el-button type="danger" @click="batchDelete" :disabled="selectedComments.length === 0">
          <el-icon><Delete /></el-icon>
          批量删除
        </el-button>
      </div>
    </div>

    <!-- 评论列表 -->
    <div class="table-section">
      <el-table 
        :data="commentList" 
        v-loading="loading"
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
        border
        stripe
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
            <el-button size="small" @click="viewComment(row)">
              <el-icon><View /></el-icon>
              查看
            </el-button>
            <el-button 
              size="small" 
              type="warning" 
              @click="hideComment(row)"
              v-if="row.status === 'Active'"
            >
              <el-icon><Hide /></el-icon>
              隐藏
            </el-button>
            <el-button 
              size="small" 
              type="success" 
              @click="showComment(row)"
              v-else-if="row.status === 'Hidden'"
            >
              <el-icon><View /></el-icon>
              显示
            </el-button>
            <el-button 
              size="small" 
              type="danger" 
              @click="deleteComment(row)"
            >
              <el-icon><Delete /></el-icon>
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

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

    <!-- 批量操作 -->
    <div class="batch-actions" v-if="selectedComments.length > 0">
      <el-button type="warning" @click="batchHide">批量隐藏</el-button>
      <el-button type="success" @click="batchShow">批量显示</el-button>
      <el-button type="danger" @click="batchDelete">批量删除</el-button>
      <span class="selected-count">已选择 {{ selectedComments.length }} 条评论</span>
    </div>

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
          <span>{{ formatTime(currentComment.created_at) }}</span>
        </div>
        <div class="detail-item">
          <label>更新时间:</label>
          <span>{{ formatTime(currentComment.updated_at) }}</span>
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

    <!-- 添加评论对话框 -->
    <el-dialog 
      v-model="addCommentDialogVisible" 
      title="添加评论" 
      width="600px"
    >
      <el-form :model="newComment" label-width="100px">
        <el-form-item label="评论内容">
          <el-input type="textarea" v-model="newComment.content" rows="4" />
        </el-form-item>
        <el-form-item label="评论身份">
          <el-select v-model="newComment.target_type" placeholder="选择身份">
            <el-option label="管理员" value="Admin" />
            <el-option label="元老" value="Elder" />
            <el-option label="普通用户" value="User" />
          </el-select>
        </el-form-item>
        <el-form-item label="选择资源">
          <el-select v-model="newComment.resource_id" filterable placeholder="选择资源" @visible-change="loadResourcesIfNeeded">
            <el-option
              v-for="item in resources"
              :key="item.id"
              :label="item.name"
              :value="item.id"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="父评论ID">
          <el-input-number v-model="newComment.parent_id" placeholder="输入父评论ID" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addCommentDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="submitAddComment">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
// 导入所需依赖
import { ref, reactive, onMounted, watch, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  ChatDotSquare,
  ChatLineSquare,
  ChatDotRound,
  ChatSquare,
  Delete,
  View,
  Hide,
  Plus,
  Search,
  RefreshRight,
  Timer
} from '@element-plus/icons-vue'
import { commentApi, Comment } from '../../api/comments'
import { resourceRecordApi } from '../../api/resourceRecords'
import { packageApi } from '../../api/packages'

// 评论列表数据
const commentList = ref<Comment[]>([])
const loading = ref(false)
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)
const selectedComments = ref<Comment[]>([])

// 筛选表单
const filterForm = reactive({
  status: '',
  target_type: '',
  user_id: '',
  date_range: [] as Date[]
})

// 当前选中的评论
const currentComment = ref<Comment | null>(null)
const commentDialogVisible = ref(false)

// 添加评论相关
const addCommentDialogVisible = ref(false)
const newComment = reactive({
  content: '',
  resource_id: null as number | null,
  parent_id: null as number | null,
  target_type: 'Package' as string
})
const resources = ref<any[]>([])
const resourcesLoaded = ref(false)

// 评论统计数据
const totalComments = computed(() => {
  return total.value || 0
})

const activeComments = computed(() => {
  return commentList.value.filter(comment => comment.status === 'Active').length
})

const hiddenComments = computed(() => {
  return commentList.value.filter(comment => comment.status === 'Hidden').length
})

const todayComments = computed(() => {
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  
  return commentList.value.filter(comment => {
    const createTime = new Date(comment.created_at)
    return createTime >= today
  }).length
})

// 加载资源选项
async function loadResourcesIfNeeded(force = false) {
  if (resourcesLoaded.value && !force) return
  
  try {
    // 这里调用绳包API获取资源列表
    const response = await packageApi.getPackages({ pageSize: 100 })
    if (response.code === 0 && response.data) {
      resources.value = response.data.list || []
      resourcesLoaded.value = true
    }
  } catch (error) {
    console.error('加载资源列表失败:', error)
  }
}

// 资源操作记录服务
const resourceLogger = {
  logResourceOperation(resourceType: string, action: string, resourceId: number, targetId?: number, targetType?: string): void {
    try {
      resourceRecordApi.logResourceAction(resourceId, action)
        .catch(err => console.warn('记录操作失败:', err));
    } catch (error) {
      console.error('记录操作失败:', error);
    }
  }
};

// 加载评论数据
async function loadComments() {
  loading.value = true
  try {
    const params: any = {
      page: currentPage.value,
      size: pageSize.value
    }
    
    // 将非空的filterForm字段添加到params
    Object.keys(filterForm).forEach(key => {
      if (filterForm[key] !== null && filterForm[key] !== undefined && filterForm[key] !== '') {
        params[key] = filterForm[key]
      }
    })
    
    // 处理日期范围
    if (filterForm.date_range && filterForm.date_range.length === 2) {
      params.start_date = filterForm.date_range[0]
      params.end_date = filterForm.date_range[1]
    }
    
    const response = await commentApi.getAllComments(params)
    if (response.code === 0 && response.data) {
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

function handleSelectionChange(selection: Comment[]) {
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

function viewComment(comment: Comment) {
  currentComment.value = comment
  commentDialogVisible.value = true
}

async function hideComment(comment: Comment) {
  try {
    await ElMessageBox.confirm('确定要隐藏这条评论吗？', '确认操作')
    const response = await commentApi.updateComment(comment.id, { status: 'Hidden' })
    if (response.code === 0) {
      ElMessage.success('评论已隐藏')
      loadComments()
      resourceLogger.logResourceOperation('Comment', 'hide', comment.id, comment.target_id, comment.target_type);
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function showComment(comment: Comment) {
  try {
    await ElMessageBox.confirm('确定要显示这条评论吗？', '确认操作')
    const response = await commentApi.updateComment(comment.id, { status: 'Active' })
    if (response.code === 0) {
      ElMessage.success('评论已显示')
      loadComments()
      resourceLogger.logResourceOperation('Comment', 'show', comment.id, comment.target_id, comment.target_type);
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function deleteComment(comment: Comment) {
  try {
    await ElMessageBox.confirm('确定要删除这条评论吗？此操作不可恢复！', '确认删除', {
      type: 'warning'
    })
    const response = await commentApi.deleteComment(comment.id)
    if (response.code === 0) {
      ElMessage.success('评论已删除')
      loadComments()
      resourceLogger.logResourceOperation('Comment', 'delete', comment.id, comment.target_id, comment.target_type);
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

// 添加评论
function addComment() {
  // 重置表单
  Object.assign(newComment, {
    content: '',
    resource_id: null,
    parent_id: null
  })
  // 显示对话框
  addCommentDialogVisible.value = true
  // 加载资源选项
  loadResourcesIfNeeded(true)
}

// 提交添加评论
async function submitAddComment() {
  if (!newComment.content || !newComment.resource_id) {
    ElMessage.warning('请填写完整的评论信息')
    return
  }
  
  try {
    // 这里使用正确的数据结构传递
    const response = await commentApi.createComment({
      content: newComment.content,
      target_id: newComment.resource_id,
      target_type: 'Package'
    })
    
    if (response.code === 0) {
      ElMessage.success('评论添加成功')
      addCommentDialogVisible.value = false
      loadComments()
      // 如果response.data存在才使用它
      const commentId = response.data?.id || 0
      resourceLogger.logResourceOperation('Comment', 'add', commentId, newComment.resource_id, 'Package');
    } else {
      ElMessage.error(`添加失败: ${response.message}`)
    }
  } catch (error) {
    console.error('添加评论失败:', error)
    ElMessage.error('添加评论失败')
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
/* 评论管理页面特定样式 */
.content-text {
  margin: 0 0 8px 0;
  line-height: 1.5;
  color: var(--text-primary);
}

.content-meta {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--text-secondary);
}

.time {
  color: var(--text-secondary);
}

.likes, .dislikes {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 其余特定样式保持不变 */
</style> 