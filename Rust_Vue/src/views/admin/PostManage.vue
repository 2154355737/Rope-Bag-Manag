<template>
  <div class="post-manage">
    <div class="page-header">
      <h1>帖子管理</h1>
      <p>管理社区帖子内容</p>
    </div>

    <!-- 筛选和搜索 -->
    <div class="filters">
      <el-row :gutter="16">
        <el-col :span="6">
          <el-input
            v-model="searchQuery"
            placeholder="搜索帖子标题或内容..."
            clearable
            @keyup.enter="loadPosts"
            @clear="loadPosts"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </el-col>
        <el-col :span="4">
          <el-select v-model="selectedStatus" placeholder="选择状态" clearable @change="loadPosts">
            <el-option label="全部" value="" />
            <el-option label="已发布" value="Published" />
            <el-option label="草稿" value="Draft" />
            <el-option label="已归档" value="Archived" />
          </el-select>
        </el-col>
        <el-col :span="4">
          <el-select v-model="selectedCategory" placeholder="选择分类" clearable @change="loadPosts">
            <el-option label="全部" value="" />
            <el-option
              v-for="category in categories"
              :key="category.id"
              :label="category.name"
              :value="category.id"
            />
          </el-select>
        </el-col>
        <el-col :span="6">
          <div class="search-buttons">
            <el-button @click="refreshData">
              <el-icon><Refresh /></el-icon>
              刷新
            </el-button>
            <el-button type="primary" @click="loadPosts">
              <el-icon><Search /></el-icon>
              查询
            </el-button>
          </div>
          <el-button @click="resetFilters" style="margin-top: 10px;">重置</el-button>
        </el-col>
      </el-row>
    </div>

    <!-- 帖子列表 -->
    <div class="table-container">
      <el-table
        :data="posts"
        v-loading="loading"
        stripe
        style="width: 100%"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        
        <el-table-column prop="id" label="ID" width="80" />
        
        <el-table-column prop="title" label="标题" min-width="200">
          <template #default="{ row }">
            <div class="post-title">
              <el-link type="primary" @click="viewPost(row.id)">
                {{ row.title }}
              </el-link>
              <div class="post-badges">
                <el-tag v-if="row.is_pinned" type="warning" size="small">置顶</el-tag>
                <el-tag v-if="row.is_featured" type="success" size="small">精选</el-tag>
              </div>
            </div>
          </template>
        </el-table-column>
        
        <el-table-column prop="author_name" label="作者" width="120" />
        
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag 
              :type="getStatusType(row.status)" 
              size="small"
            >
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        
        <el-table-column label="统计" width="120">
          <template #default="{ row }">
            <div class="post-stats">
              <span><el-icon><View /></el-icon> {{ row.view_count }}</span>
              <span><el-icon><ChatDotRound /></el-icon> {{ row.comment_count }}</span>
              <span><el-icon><Star /></el-icon> {{ row.like_count }}</span>
            </div>
          </template>
        </el-table-column>
        
        <el-table-column prop="created_at" label="创建时间" width="160">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
        
        <el-table-column label="操作" width="300" fixed="right">
          <template #default="{ row }">
            <el-button-group>
              <el-button size="small" @click="viewPost(row.id)">
                <el-icon><View /></el-icon>
                查看
              </el-button>
              <el-button size="small" type="primary" @click="editPost(row)">
                <el-icon><Edit /></el-icon>
                编辑
              </el-button>
                             <el-dropdown @command="(command: string) => handleAction(command, row)">
                <el-button size="small">
                  更多<el-icon class="el-icon--right"><ArrowDown /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item :command="`pin-${row.id}`">
                      {{ row.is_pinned ? '取消置顶' : '置顶' }}
                    </el-dropdown-item>
                    <el-dropdown-item :command="`feature-${row.id}`">
                      {{ row.is_featured ? '取消精选' : '设为精选' }}
                    </el-dropdown-item>
                    <el-dropdown-item :command="`archive-${row.id}`" v-if="row.status !== 'Archived'">
                      归档
                    </el-dropdown-item>
                    <el-dropdown-item :command="`publish-${row.id}`" v-if="row.status !== 'Published'">
                      发布
                    </el-dropdown-item>
                    <el-dropdown-item :command="`delete-${row.id}`" divided>
                      <span style="color: #f56c6c">删除</span>
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </el-button-group>
          </template>
        </el-table-column>
      </el-table>

      <!-- 批量操作 -->
      <div class="batch-actions" v-if="selectedPosts.length > 0">
        <span>已选择 {{ selectedPosts.length }} 个帖子</span>
        <el-button-group>
          <el-button size="small" @click="batchPin">批量置顶</el-button>
          <el-button size="small" @click="batchFeature">批量精选</el-button>
          <el-button size="small" @click="batchArchive">批量归档</el-button>
          <el-button size="small" type="danger" @click="batchDelete">批量删除</el-button>
        </el-button-group>
      </div>

      <!-- 分页 -->
      <div class="pagination">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="loadPosts"
          @current-change="loadPosts"
        />
      </div>
    </div>

    <!-- 编辑帖子对话框 -->
    <el-dialog
      v-model="showEditDialog"
      title="编辑帖子"
      width="800px"
      @close="resetEditForm"
    >
      <el-form
        ref="editFormRef"
        :model="editForm"
        :rules="editRules"
        label-width="100px"
      >
        <el-form-item label="标题" prop="title">
          <el-input v-model="editForm.title" maxlength="200" show-word-limit />
        </el-form-item>
        
        <el-form-item label="内容" prop="content">
          <el-input
            v-model="editForm.content"
            type="textarea"
            :rows="10"
            maxlength="10000"
            show-word-limit
          />
        </el-form-item>
        
        <el-form-item label="分类">
          <el-select v-model="editForm.category_id" placeholder="选择分类">
            <el-option
              v-for="category in categories"
              :key="category.id"
              :label="category.name"
              :value="category.id"
            />
          </el-select>
        </el-form-item>
        
        <el-form-item label="标签">
          <el-select
            v-model="editForm.tags"
            multiple
            filterable
            allow-create
            placeholder="选择或创建标签"
          >
            <el-option
              v-for="tag in allTags"
              :key="tag.id"
              :label="tag.name"
              :value="tag.name"
            />
          </el-select>
        </el-form-item>
        
        <el-form-item label="状态">
          <el-radio-group v-model="editForm.status">
            <el-radio label="Draft">草稿</el-radio>
            <el-radio label="Published">已发布</el-radio>
            <el-radio label="Archived">已归档</el-radio>
          </el-radio-group>
        </el-form-item>
        
        <el-form-item label="选项">
          <el-checkbox v-model="editForm.is_pinned">置顶</el-checkbox>
          <el-checkbox v-model="editForm.is_featured">精选</el-checkbox>
        </el-form-item>
      </el-form>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showEditDialog = false">取消</el-button>
          <el-button type="primary" @click="savePost" :loading="saving">
            保存
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  View,
  Edit,
  Delete,
  Plus,
  Search,
  ChatDotRound,
  Star,
  ArrowDown,
  Refresh
} from '@element-plus/icons-vue'
import { postApi, categoryApi, tagApi } from '@/api'
import type { Post, UpdatePostRequest, Category, Tag } from '@/api'
import { formatDate } from '@/utils/format'

const router = useRouter()

// 格式化时间函数
const formatTime = (timeStr: string) => {
  return formatDate(timeStr, 'YYYY-MM-DD HH:mm')
}

// 响应式数据
const loading = ref(false)
const saving = ref(false)
const posts = ref<Post[]>([])
const categories = ref<Category[]>([])
const allTags = ref<Tag[]>([])
const selectedPosts = ref<Post[]>([])
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(20)

// 搜索筛选
const searchQuery = ref('')
const selectedStatus = ref('')
const selectedCategory = ref('')

// 编辑相关
const showEditDialog = ref(false)
const editFormRef = ref()
const editForm = reactive<UpdatePostRequest & { id?: number }>({
  title: '',
  content: '',
  category_id: undefined,
  tags: [],
  status: 'Draft',
  is_pinned: false,
  is_featured: false
})

const editRules = {
  title: [
    { required: true, message: '请输入标题', trigger: 'blur' },
    { min: 1, max: 200, message: '标题长度在 1 到 200 个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入内容', trigger: 'blur' },
    { min: 10, message: '内容至少 10 个字符', trigger: 'blur' }
  ]
}

// 方法
const loadPosts = async () => {
  loading.value = true
  try {
    console.log('🔍 [PostManage] 开始加载帖子')
    
    const params = {
      page: currentPage.value,
      page_size: pageSize.value,
      search: searchQuery.value || undefined,
      status: selectedStatus.value || undefined,
      category_id: selectedCategory.value ? parseInt(selectedCategory.value) : undefined
    }
    
    console.log('🔍 [PostManage] 请求参数:', params)
    
    const response = await postApi.getPosts(params)
    console.log('🔍 [PostManage] API响应:', response)

    if (response.code === 0 && response.data) {
      const postsList = Array.isArray(response.data) ? response.data : response.data.list || []
      const postsTotal = Array.isArray(response.data) ? response.data.length : response.data.total || postsList.length
      
      console.log('🔍 [PostManage] 解析后的帖子列表:', postsList)
      console.log('🔍 [PostManage] 总数:', postsTotal)
      
      posts.value = postsList
      total.value = postsTotal
      
      if (postsList.length === 0) {
        console.warn('⚠️ [PostManage] 帖子列表为空')
      }
    } else {
      console.error('❌ [PostManage] API返回错误:', response)
      ElMessage.error(response.msg || response.message || '加载帖子失败')
    }
  } catch (error) {
    console.error('❌ [PostManage] 请求异常:', error)
    ElMessage.error('加载帖子失败')
  } finally {
    loading.value = false
  }
}

const loadCategories = async () => {
  try {
    const response = await categoryApi.getCategories()
    if (response.code === 0 && response.data) {
      categories.value = response.data?.list || response.data || []
    }
  } catch (error) {
    console.error('加载分类失败:', error)
  }
}

const loadTags = async () => {
  try {
    const response = await tagApi.getAllTags()
    if (response.code === 0 && response.data) {
      allTags.value = response.data
    }
  } catch (error) {
    console.error('加载标签失败:', error)
  }
}

const resetFilters = () => {
  searchQuery.value = ''
  selectedStatus.value = ''
  selectedCategory.value = ''
  currentPage.value = 1
  loadPosts()
}

const handleSelectionChange = (selection: Post[]) => {
  selectedPosts.value = selection
}

const viewPost = (id: number) => {
  router.push(`/posts/${id}`)
}

const editPost = (post: Post) => {
  editForm.id = post.id
  editForm.title = post.title
  editForm.content = post.content
  editForm.category_id = post.category_id
  editForm.status = post.status
  editForm.is_pinned = post.is_pinned
  editForm.is_featured = post.is_featured
  editForm.tags = [] // 需要加载帖子的标签
  showEditDialog.value = true
  
  // 加载帖子标签
  loadPostTags(post.id)
}

const loadPostTags = async (postId: number) => {
  try {
    const response = await postApi.getPostTags(postId)
    if (response.code === 0 && response.data) {
      editForm.tags = response.data.map((tag: Tag) => tag.name)
    }
  } catch (error) {
    console.error('加载帖子标签失败:', error)
  }
}

const resetEditForm = () => {
  editForm.id = undefined
  editForm.title = ''
  editForm.content = ''
  editForm.category_id = undefined
  editForm.status = 'Draft'
  editForm.is_pinned = false
  editForm.is_featured = false
  editForm.tags = []
}

const savePost = async () => {
  if (!editFormRef.value) return
  
  try {
    await editFormRef.value.validate()
    saving.value = true
    
    const { id, ...updateData } = editForm
    const response = await postApi.updatePost(id!, updateData)
    
    if (response.code === 0) {
      ElMessage.success('帖子更新成功')
      showEditDialog.value = false
      loadPosts()
    } else {
      ElMessage.error(response.msg || '更新失败')
    }
  } catch (error) {
    console.error('保存帖子失败:', error)
    ElMessage.error('保存失败')
  } finally {
    saving.value = false
  }
}

const handleAction = async (command: string, post: Post) => {
  const [action, id] = command.split('-')
  const postId = parseInt(id)
  
  try {
    let updateData: Partial<UpdatePostRequest> = {}
    let message = ''
    
    switch (action) {
      case 'pin':
        updateData.is_pinned = !post.is_pinned
        message = post.is_pinned ? '取消置顶成功' : '置顶成功'
        break
      case 'feature':
        updateData.is_featured = !post.is_featured
        message = post.is_featured ? '取消精选成功' : '设为精选成功'
        break
      case 'archive':
        updateData.status = 'Archived'
        message = '归档成功'
        break
      case 'publish':
        updateData.status = 'Published'
        message = '发布成功'
        break
      case 'delete':
        await ElMessageBox.confirm('确定要删除这个帖子吗？', '确认删除', {
          type: 'warning'
        })
        const deleteResponse = await postApi.deletePost(postId)
        if (deleteResponse.code === 0) {
          ElMessage.success('删除成功')
          loadPosts()
        }
        return
    }
    
    const response = await postApi.updatePost(postId, updateData)
    if (response.code === 0) {
      ElMessage.success(message)
      loadPosts()
    } else {
      ElMessage.error(response.msg || '操作失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('操作失败:', error)
      ElMessage.error('操作失败')
    }
  }
}

// 批量操作
const batchPin = async () => {
  try {
    await ElMessageBox.confirm(`确定要置顶选中的 ${selectedPosts.value.length} 个帖子吗？`, '批量置顶')
    // 这里需要后端支持批量操作API
    ElMessage.info('批量置顶功能开发中...')
  } catch (error) {
    // 用户取消
  }
}

const batchFeature = async () => {
  try {
    await ElMessageBox.confirm(`确定要将选中的 ${selectedPosts.value.length} 个帖子设为精选吗？`, '批量精选')
    ElMessage.info('批量精选功能开发中...')
  } catch (error) {
    // 用户取消
  }
}

const batchArchive = async () => {
  try {
    await ElMessageBox.confirm(`确定要归档选中的 ${selectedPosts.value.length} 个帖子吗？`, '批量归档')
    ElMessage.info('批量归档功能开发中...')
  } catch (error) {
    // 用户取消
  }
}

const batchDelete = async () => {
  try {
    await ElMessageBox.confirm(
      `确定要删除选中的 ${selectedPosts.value.length} 个帖子吗？此操作不可恢复！`,
      '批量删除',
      { type: 'warning' }
    )
    ElMessage.info('批量删除功能开发中...')
  } catch (error) {
    // 用户取消
  }
}

const refreshData = () => {
  loadPosts()
}

const getStatusType = (status: string) => {
  switch (status) {
    case 'Published': return 'success'
    case 'Draft': return 'info'
    case 'Archived': return 'warning'
    default: return 'info'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'Published': return '已发布'
    case 'Draft': return '草稿'
    case 'Archived': return '已归档'
    default: return status
  }
}

// 初始化
onMounted(() => {
  loadPosts()
  loadCategories()
  loadTags()
})
</script>

<style scoped>
.post-manage {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
}

.page-header h1 {
  margin: 0 0 8px 0;
  font-size: 24px;
  font-weight: 600;
}

.page-header p {
  margin: 0;
  color: #666;
}

.filters {
  margin-bottom: 20px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.table-container {
  background: white;
  border-radius: 8px;
  padding: 16px;
}

.post-title {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.post-badges {
  display: flex;
  gap: 4px;
}

.post-stats {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
  color: #666;
}

.post-stats span {
  display: flex;
  align-items: center;
  gap: 4px;
}

.search-buttons {
  display: flex;
  gap: 10px;
  margin-bottom: 10px;
}

.batch-actions {
  margin: 16px 0;
  padding: 12px;
  background: #f0f9ff;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.pagination {
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style> 