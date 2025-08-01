<template>
  <div class="posts-container">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <h1 class="page-title">社区帖子</h1>
        <p class="page-description">分享你的想法，与社区成员交流</p>
      </div>
      <div class="header-actions">
        <el-button type="primary" @click="showCreateDialog = true" v-if="isLoggedIn">
          <el-icon><Plus /></el-icon>
          发布帖子
        </el-button>
      </div>
    </div>

    <!-- 筛选和搜索 -->
    <div class="filter-section">
      <div class="filter-row">
        <div class="search-box">
          <el-input
            v-model="searchQuery"
            placeholder="搜索帖子..."
            clearable
            @keyup.enter="handleSearch"
            @clear="handleSearch"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </div>
        <div class="filter-actions">
          <el-select v-model="selectedCategory" placeholder="选择分类" clearable @change="handleSearch">
            <el-option
              v-for="category in categories"
              :key="category.id"
              :label="category.name"
              :value="category.id"
            />
          </el-select>
          <el-select v-model="selectedStatus" placeholder="状态" clearable @change="handleSearch">
            <el-option label="已发布" value="Published" />
            <el-option label="草稿" value="Draft" />
            <el-option label="已归档" value="Archived" />
          </el-select>
        </div>
      </div>
      
      <!-- 热门标签 -->
      <div class="popular-tags" v-if="popularTags.length > 0">
        <span class="tags-label">热门标签：</span>
        <el-tag
          v-for="tag in popularTags"
          :key="tag.id"
          :color="tag.color"
          class="tag-item"
          @click="selectTag(tag.name)"
        >
          {{ tag.name }}
        </el-tag>
      </div>
    </div>

    <!-- 帖子列表 -->
    <div class="posts-list" v-loading="loading">
      <div v-if="posts.length === 0 && !loading" class="empty-state">
        <el-empty description="暂无帖子" />
      </div>
      
      <div v-else class="posts-grid">
        <div
          v-for="post in posts"
          :key="post.id"
          class="post-card"
          @click="viewPost(post.id)"
        >
          <div class="post-header">
            <div class="post-meta">
              <span class="author">{{ post.author_name }}</span>
              <span class="time">{{ formatTime(post.created_at) }}</span>
            </div>
            <div class="post-status">
              <el-tag v-if="post.is_pinned" type="warning" size="small">置顶</el-tag>
              <el-tag v-if="post.is_featured" type="success" size="small">精选</el-tag>
              <el-tag v-if="post.status === 'Draft'" type="info" size="small">草稿</el-tag>
            </div>
          </div>
          
          <div class="post-content">
            <h3 class="post-title">{{ post.title }}</h3>
            <p class="post-excerpt">{{ getExcerpt(post.content) }}</p>
          </div>
          
          <div class="post-footer">
            <div class="post-stats">
              <span class="stat-item">
                <el-icon><View /></el-icon>
                {{ post.view_count }}
              </span>
              <span class="stat-item">
                <el-icon><ChatDotRound /></el-icon>
                {{ post.comment_count }}
              </span>
              <span class="stat-item">
                <el-icon><Star /></el-icon>
                {{ post.like_count }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 分页 -->
    <div class="pagination-wrapper" v-if="total > 0">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50]"
        :total="total"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </div>

    <!-- 创建帖子对话框 -->
    <el-dialog
      v-model="showCreateDialog"
      title="发布新帖子"
      width="60%"
      :before-close="handleCloseDialog"
    >
      <el-form ref="createFormRef" :model="createForm" :rules="createRules" label-width="80px">
        <el-form-item label="标题" prop="title">
          <el-input v-model="createForm.title" placeholder="请输入帖子标题" />
        </el-form-item>
        
        <el-form-item label="分类" prop="category_id">
          <el-select v-model="createForm.category_id" placeholder="选择分类" clearable>
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
            v-model="createForm.tags"
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
        
        <el-form-item label="内容" prop="content">
          <el-input
            v-model="createForm.content"
            type="textarea"
            :rows="8"
            placeholder="请输入帖子内容"
          />
        </el-form-item>
        
        <el-form-item label="状态">
          <el-radio-group v-model="createForm.status">
            <el-radio value="Draft">草稿</el-radio>
            <el-radio value="Published">发布</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showCreateDialog = false">取消</el-button>
          <el-button type="primary" @click="handleCreatePost" :loading="creating">
            发布
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Search, View, ChatDotRound, Star } from '@element-plus/icons-vue'
import { postApi, tagApi, categoryApi } from '@/api'
import type { Post, CreatePostRequest, Tag, Category } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { formatDate } from '@/utils/format'

// 格式化时间函数
const formatTime = (timeStr: string) => {
  return formatDate(timeStr, 'YYYY-MM-DD HH:mm')
}

const router = useRouter()
const authStore = useAuthStore()

// 响应式数据
const loading = ref(false)
const creating = ref(false)
const posts = ref<Post[]>([])
const categories = ref<Category[]>([])
const popularTags = ref<Tag[]>([])
const allTags = ref<Tag[]>([])
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)
const searchQuery = ref('')
const selectedCategory = ref<number>()
const selectedStatus = ref<string>()

// 创建帖子相关
const showCreateDialog = ref(false)
const createFormRef = ref()
const createForm = reactive<CreatePostRequest>({
  title: '',
  content: '',
  category_id: undefined,
  tags: [],
  status: 'Draft'
})

const createRules = {
  title: [
    { required: true, message: '请输入标题', trigger: 'blur' },
    { min: 1, max: 200, message: '标题长度在 1 到 200 个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入内容', trigger: 'blur' },
    { min: 10, message: '内容至少 10 个字符', trigger: 'blur' }
  ]
}

// 计算属性
const isLoggedIn = computed(() => authStore.isLoggedIn)

// 方法
const loadPosts = async () => {
  loading.value = true
  try {
    const params = {
      page: currentPage.value,
      page_size: pageSize.value,
      search: searchQuery.value || undefined,
      category_id: selectedCategory.value,
      status: selectedStatus.value
    }
    
    const response = await postApi.getPosts(params)
    if (response.code === 0 && response.data) {
      posts.value = response.data.list
      total.value = response.data.total
    }
  } catch (error) {
    console.error('加载帖子失败:', error)
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

const loadPopularTags = async () => {
  try {
    const response = await tagApi.getPopularTags()
    if (response.code === 0 && response.data) {
      popularTags.value = response.data
    }
  } catch (error) {
    console.error('加载热门标签失败:', error)
  }
}

const loadAllTags = async () => {
  try {
    const response = await tagApi.getAllTags()
    if (response.code === 0 && response.data) {
      allTags.value = response.data
    }
  } catch (error) {
    console.error('加载所有标签失败:', error)
  }
}

const handleSearch = () => {
  currentPage.value = 1
  loadPosts()
}

const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  loadPosts()
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
  loadPosts()
}

const selectTag = (tagName: string) => {
  searchQuery.value = tagName
  handleSearch()
}

const viewPost = (postId: number) => {
  router.push(`/posts/${postId}`)
}

const getExcerpt = (content: string) => {
  return content.length > 100 ? content.substring(0, 100) + '...' : content
}

const handleCreatePost = async () => {
  if (!createFormRef.value) return
  
  try {
    await createFormRef.value.validate()
    creating.value = true
    
    const response = await postApi.createPost(createForm)
    if (response.code === 0) {
      ElMessage.success('帖子发布成功')
      showCreateDialog.value = false
      resetCreateForm()
      loadPosts()
    } else {
      ElMessage.error(response.msg || '发布失败')
    }
  } catch (error) {
    console.error('发布帖子失败:', error)
    ElMessage.error('发布帖子失败')
  } finally {
    creating.value = false
  }
}

const handleCloseDialog = () => {
  ElMessageBox.confirm('确定要取消发布吗？未保存的内容将丢失。', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    showCreateDialog.value = false
    resetCreateForm()
  })
}

const resetCreateForm = () => {
  createForm.title = ''
  createForm.content = ''
  createForm.category_id = undefined
  createForm.tags = []
  createForm.status = 'Draft'
  if (createFormRef.value) {
    createFormRef.value.resetFields()
  }
}

// 生命周期
onMounted(() => {
  loadPosts()
  loadCategories()
  loadPopularTags()
  loadAllTags()
})
</script>

<style scoped>
.posts-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  color: white;
}

.header-content h1 {
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 600;
}

.header-content p {
  margin: 0;
  opacity: 0.9;
  font-size: 16px;
}

.filter-section {
  margin-bottom: 30px;
  padding: 20px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.filter-row {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
}

.search-box {
  flex: 1;
}

.filter-actions {
  display: flex;
  gap: 12px;
}

.popular-tags {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.tags-label {
  font-weight: 500;
  color: #666;
}

.tag-item {
  cursor: pointer;
  transition: all 0.3s;
}

.tag-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.posts-list {
  margin-bottom: 30px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
}

.posts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
}

.post-card {
  background: white;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  cursor: pointer;
  border: 1px solid #f0f0f0;
}

.post-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.post-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
}

.post-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.author {
  font-weight: 600;
  color: #333;
  font-size: 14px;
}

.time {
  color: #999;
  font-size: 12px;
}

.post-status {
  display: flex;
  gap: 4px;
}

.post-content {
  margin-bottom: 16px;
}

.post-title {
  margin: 0 0 12px 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
  line-height: 1.4;
}

.post-excerpt {
  margin: 0;
  color: #666;
  line-height: 1.6;
  font-size: 14px;
}

.post-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.post-stats {
  display: flex;
  gap: 16px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #999;
  font-size: 12px;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 30px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

@media (max-width: 768px) {
  .posts-container {
    padding: 10px;
  }
  
  .page-header {
    flex-direction: column;
    gap: 16px;
    text-align: center;
  }
  
  .filter-row {
    flex-direction: column;
  }
  
  .posts-grid {
    grid-template-columns: 1fr;
  }
}
</style> 