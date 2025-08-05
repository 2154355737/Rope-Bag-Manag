<template>
  <div class="posts-page">
    <!-- 页面头部 -->
    <header class="page-header">
      <div class="header-container">
        <div class="header-content">
          <div class="header-main">
            <h1 class="page-title">社区帖子</h1>
            <p class="page-description">分享你的想法，与社区成员交流互动</p>
          </div>
          <div class="header-actions">
            <el-button 
              v-if="isLoggedIn" 
              type="primary" 
              size="large"
              @click="showCreateDialog = true"
              class="create-btn"
            >
              <el-icon><EditPen /></el-icon>
              发布帖子
            </el-button>
            <el-button 
              v-else
              type="primary" 
              size="large"
              @click="$router.push('/login')"
              class="login-btn"
            >
              <el-icon><User /></el-icon>
              登录发帖
            </el-button>
          </div>
        </div>
      </div>
    </header>

    <!-- 主内容区域 -->
    <main class="main-content">
      <div class="content-container">
        <!-- 搜索和筛选区域 -->
        <div class="filter-section">
          <div class="filter-card">
            <!-- 搜索栏 -->
            <div class="search-area">
              <div class="search-input-wrapper">
                <el-input
                  v-model="searchQuery"
                  placeholder="搜索帖子标题、内容..."
                  clearable
                  size="large"
                  class="search-input"
                  @keyup.enter="handleSearch"
                  @clear="handleSearch"
                >
                  <template #prefix>
                    <el-icon><Search /></el-icon>
                  </template>
                </el-input>
                <el-button 
                  type="primary" 
                  size="large" 
                  @click="handleSearch"
                  class="search-btn"
                >
                  搜索
                </el-button>
              </div>
            </div>
            
            <!-- 筛选器 -->
            <div class="filters-area">
              <div class="filter-group">
                <label class="filter-label">分类筛选</label>
                <el-select 
                  v-model="selectedCategory" 
                  placeholder="选择分类" 
                  clearable 
                  size="large"
                  @change="handleSearch"
                  class="filter-select"
                >
                  <el-option
                    v-for="category in categories"
                    :key="category.id"
                    :label="category.name"
                    :value="category.id"
                  />
                </el-select>
              </div>
              
              <div class="filter-group">
                <label class="filter-label">状态筛选</label>
                <el-select 
                  v-model="selectedStatus" 
                  placeholder="选择状态" 
                  clearable 
                  size="large"
                  @change="handleSearch"
                  class="filter-select"
                >
                  <el-option label="已发布" value="Published" />
                  <el-option label="草稿" value="Draft" />
                  <el-option label="已归档" value="Archived" />
                </el-select>
              </div>
              
              <div class="filter-group">
                <label class="filter-label">排序方式</label>
                <el-select 
                  v-model="sortBy" 
                  size="large"
                  @change="handleSearch"
                  class="filter-select"
                >
                  <el-option label="最新发布" value="created_at" />
                  <el-option label="最多浏览" value="view_count" />
                  <el-option label="最多点赞" value="like_count" />
                  <el-option label="最多评论" value="comment_count" />
                </el-select>
              </div>
            </div>
          </div>
          
          <!-- 热门标签 -->
          <div v-if="popularTags.length > 0" class="tags-section">
            <div class="tags-card">
              <div class="tags-header">
                <h3 class="tags-title">
                  <el-icon><CollectionTag /></el-icon>
                  热门标签
                </h3>
              </div>
              <div class="tags-list">
                <el-tag
                  v-for="tag in popularTags"
                  :key="tag.id"
                  :type="getTagType(tag.id)"
                  size="large"
                  effect="light"
                  class="tag-item"
                  @click="selectTag(tag.name)"
                >
                  {{ tag.name }}
                </el-tag>
              </div>
            </div>
          </div>
        </div>

        <!-- 帖子列表 -->
        <div class="posts-section">
          <div v-loading="loading" class="posts-container">
            <!-- 空状态 -->
            <div v-if="posts.length === 0 && !loading" class="empty-state">
              <div class="empty-card">
                <div class="empty-icon">
                  <el-icon><DocumentDelete /></el-icon>
                </div>
                <h3 class="empty-title">暂无帖子</h3>
                <p class="empty-description">
                  {{ searchQuery ? '没有找到匹配的帖子' : '还没有人发布帖子，来发布第一个吧！' }}
                </p>
                <el-button 
                  v-if="isLoggedIn && !searchQuery" 
                  type="primary" 
                  @click="showCreateDialog = true"
                >
                  发布帖子
                </el-button>
              </div>
            </div>
            
            <!-- 帖子网格 -->
            <div v-else class="posts-grid">
              <article
                v-for="post in posts"
                :key="post.id"
                class="post-card"
                @click="viewPost(post.id)"
              >
                <!-- 帖子头部 -->
                <header class="post-header">
                  <div class="post-meta">
                    <div class="author-info">
                      <el-avatar :size="32" class="author-avatar">
                        {{ post.author_name?.charAt(0) || 'U' }}
                      </el-avatar>
                      <div class="author-details">
                        <span class="author-name">{{ post.author_name }}</span>
                        <span class="post-time">{{ formatTime(post.created_at) }}</span>
                      </div>
                    </div>
                    <div class="post-badges">
                      <el-tag v-if="post.is_pinned" type="warning" size="small" effect="dark">
                        <el-icon><Top /></el-icon>
                        置顶
                      </el-tag>
                      <el-tag v-if="post.is_featured" type="success" size="small" effect="dark">
                        <el-icon><Star /></el-icon>
                        精选
                      </el-tag>
                      <el-tag v-if="post.status === 'Draft'" type="info" size="small" effect="light">
                        草稿
                      </el-tag>
                    </div>
                  </div>
                </header>
                
                <!-- 帖子内容 -->
                <div class="post-content">
                  <h3 class="post-title">{{ post.title }}</h3>
                  <p class="post-excerpt">{{ getExcerpt(post.content) }}</p>
                  
                  <!-- 帖子标签 -->
                  <div v-if="post.tags && post.tags.length > 0" class="post-tags">
                    <el-tag
                      v-for="tag in post.tags.slice(0, 3)"
                      :key="tag"
                      size="small"
                      effect="plain"
                      class="post-tag"
                    >
                      {{ tag }}
                    </el-tag>
                    <span v-if="post.tags.length > 3" class="more-tags">
                      +{{ post.tags.length - 3 }}
                    </span>
                  </div>
                </div>
                
                <!-- 帖子底部统计 -->
                <footer class="post-footer">
                  <div class="post-stats">
                    <div class="stat-item">
                      <el-icon><View /></el-icon>
                      <span>{{ formatNumber(post.view_count) }}</span>
                    </div>
                    <div class="stat-item">
                      <el-icon><ChatDotRound /></el-icon>
                      <span>{{ formatNumber(post.comment_count) }}</span>
                    </div>
                    <div class="stat-item">
                      <el-icon><Star /></el-icon>
                      <span>{{ formatNumber(post.like_count) }}</span>
                    </div>
                  </div>
                  
                  <div class="post-category">
                    <el-tag 
                      v-if="post.category_name"
                      :type="getCategoryType(post.category_id)"
                      size="small"
                      effect="light"
                    >
                      {{ post.category_name }}
                    </el-tag>
                  </div>
                </footer>
              </article>
            </div>
          </div>
        </div>

        <!-- 分页器 -->
        <div v-if="total > 0" class="pagination-section">
          <div class="pagination-wrapper">
            <div class="pagination-info">
              <span class="info-text">
                共 {{ total }} 个帖子
                <template v-if="searchQuery">
                  ，搜索 "{{ searchQuery }}"
                </template>
              </span>
            </div>
            <el-pagination
              v-model:current-page="currentPage"
              v-model:page-size="pageSize"
              :page-sizes="[12, 24, 48]"
              :total="total"
              layout="total, sizes, prev, pager, next, jumper"
              background
              @size-change="handleSizeChange"
              @current-change="handleCurrentChange"
              class="pagination"
            />
          </div>
        </div>
      </div>
    </main>

    <!-- 创建帖子对话框 -->
    <el-dialog
      v-model="showCreateDialog"
      title="发布新帖子"
      width="800px"
      :before-close="handleCloseDialog"
      class="create-dialog"
    >
      <div class="create-form-container">
        <el-form 
          ref="createFormRef" 
          :model="createForm" 
          :rules="createRules" 
          label-width="80px"
          class="create-form"
        >
          <el-form-item label="标题" prop="title">
            <el-input 
              v-model="createForm.title" 
              placeholder="请输入帖子标题" 
              maxlength="100"
              show-word-limit
              size="large"
            />
          </el-form-item>
          
          <el-form-item label="分类" prop="category_id">
            <el-select 
              v-model="createForm.category_id" 
              placeholder="选择分类" 
              clearable
              size="large"
              style="width: 100%"
            >
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
              size="large"
              style="width: 100%"
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
              placeholder="请输入帖子内容..."
              maxlength="2000"
              show-word-limit
              resize="vertical"
            />
          </el-form-item>
          
          <el-form-item label="状态">
            <el-radio-group v-model="createForm.status" size="large">
              <el-radio value="Draft">
                <el-icon><Document /></el-icon>
                草稿
              </el-radio>
              <el-radio value="Published">
                <el-icon><Promotion /></el-icon>
                发布
              </el-radio>
            </el-radio-group>
          </el-form-item>
        </el-form>
      </div>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showCreateDialog = false" size="large">
            取消
          </el-button>
          <el-button 
            type="primary" 
            @click="handleCreatePost" 
            :loading="creating"
            size="large"
          >
            {{ creating ? '发布中...' : '发布帖子' }}
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  EditPen, 
  User,
  Search, 
  CollectionTag,
  DocumentDelete,
  Top,
  Star,
  View, 
  ChatDotRound,
  Document,
  Promotion
} from '@element-plus/icons-vue'
import { postApi, tagApi, categoryApi } from '@/api'
import type { Post, CreatePostRequest, Tag, Category } from '@/api'
import { getUserInfo } from '@/utils/auth'
import { formatDate } from '@/utils/format'

// 格式化时间函数
const formatTime = (timeStr: string) => {
  return formatDate(timeStr, 'YYYY-MM-DD HH:mm')
}

// 格式化数字显示
const formatNumber = (num: number): string => {
  if (num < 1000) return num.toString()
  if (num < 10000) return (num / 1000).toFixed(1) + 'K'
  if (num < 1000000) return (num / 10000).toFixed(1) + '万'
  return (num / 1000000).toFixed(1) + 'M'
}

const router = useRouter()

// 响应式数据
const loading = ref(false)
const creating = ref(false)
const posts = ref<Post[]>([])
const categories = ref<Category[]>([])
const popularTags = ref<Tag[]>([])
const allTags = ref<Tag[]>([])
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(12)
const searchQuery = ref('')
const selectedCategory = ref<number>()
const selectedStatus = ref<string>()
const sortBy = ref('created_at')

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
    { min: 1, max: 100, message: '标题长度在 1 到 100 个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入内容', trigger: 'blur' },
    { min: 10, message: '内容至少 10 个字符', trigger: 'blur' }
  ]
}

// 计算属性
const isLoggedIn = computed(() => !!getUserInfo())

// 方法
const loadPosts = async () => {
  loading.value = true
  try {
    const params = {
      page: currentPage.value,
      page_size: pageSize.value,
      search: searchQuery.value || undefined,
      category_id: selectedCategory.value,
      status: selectedStatus.value,
      sort_by: sortBy.value
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
  return content.length > 120 ? content.substring(0, 120) + '...' : content
}

const getTagType = (tagId: number) => {
  const types = ['primary', 'success', 'warning', 'danger', 'info']
  return types[tagId % types.length]
}

const getCategoryType = (categoryId: number | undefined) => {
  if (categoryId === undefined) return 'info'
  const types = ['primary', 'success', 'warning', 'danger', 'info']
  return types[categoryId % types.length]
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
  }).catch(() => {
    // 用户取消
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
.posts-page {
  min-height: 100vh;
  background: var(--bg-primary);
}

/* ===== 页面头部 ===== */
.page-header {
  background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-secondary) 100%);
  color: white;
  padding: var(--space-12) 0 var(--space-8);
  position: relative;
  overflow: hidden;
}

.page-header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><pattern id="grid" width="10" height="10" patternUnits="userSpaceOnUse"><path d="M 10 0 L 0 0 0 10" fill="none" stroke="rgba(255,255,255,0.1)" stroke-width="0.5"/></pattern></defs><rect width="100" height="100" fill="url(%23grid)"/></svg>');
  opacity: 0.3;
}

.header-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 var(--space-6);
  position: relative;
  z-index: 1;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-8);
}

.header-main {
  flex: 1;
}

.page-title {
  font-size: var(--font-size-4xl);
  font-weight: 800;
  margin: 0 0 var(--space-3) 0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.page-description {
  font-size: var(--font-size-lg);
  margin: 0;
  opacity: 0.9;
  line-height: 1.5;
}

.header-actions {
  flex-shrink: 0;
}

.create-btn,
.login-btn {
  height: 48px;
  padding: 0 var(--space-8);
  font-size: var(--font-size-base);
  font-weight: 600;
  border-radius: var(--radius-xl);
  background: rgba(255, 255, 255, 0.2);
  border: 2px solid rgba(255, 255, 255, 0.3);
  color: white;
  backdrop-filter: blur(10px);
  transition: var(--transition-normal);
  box-shadow: var(--shadow-lg);
}

.create-btn:hover,
.login-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  border-color: rgba(255, 255, 255, 0.5);
  transform: translateY(-2px);
  box-shadow: var(--shadow-xl);
}

/* ===== 主内容区域 ===== */
.main-content {
  padding: var(--space-8) 0;
}

.content-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 var(--space-6);
  display: flex;
  flex-direction: column;
  gap: var(--space-8);
}

/* ===== 筛选区域 ===== */
.filter-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.filter-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-8);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.search-area {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.search-input-wrapper {
  display: flex;
  gap: var(--space-4);
  align-items: flex-end;
}

.search-input {
  flex: 1;
}

.search-input :deep(.el-input__wrapper) {
  border-radius: var(--radius-xl);
  border: 2px solid var(--border-color);
  box-shadow: var(--shadow-sm);
  transition: var(--transition-normal);
}

.search-input :deep(.el-input__wrapper):hover {
  border-color: var(--color-primary);
}

.search-input :deep(.el-input__wrapper.is-focus) {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.search-btn {
  height: 48px;
  padding: 0 var(--space-8);
  border-radius: var(--radius-xl);
  font-weight: 600;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border: none;
  box-shadow: var(--shadow-md);
  transition: var(--transition-normal);
}

.search-btn:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.filters-area {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--space-6);
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.filter-label {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.filter-select :deep(.el-select__wrapper) {
  border-radius: var(--radius-lg);
  border: 2px solid var(--border-color);
  transition: var(--transition-normal);
}

.filter-select :deep(.el-select__wrapper):hover {
  border-color: var(--color-primary);
}

/* ===== 标签区域 ===== */
.tags-section {
  margin-top: var(--space-4);
}

.tags-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-6);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
}

.tags-header {
  margin-bottom: var(--space-4);
}

.tags-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.tags-title .el-icon {
  color: var(--color-primary);
}

.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
}

.tag-item {
  cursor: pointer;
  transition: var(--transition-normal);
  font-weight: 500;
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius-lg);
}

.tag-item:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

/* ===== 帖子区域 ===== */
.posts-section {
  flex: 1;
}

.posts-container {
  min-height: 400px;
}

.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

.empty-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-12);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
  text-align: center;
  max-width: 400px;
}

.empty-icon {
  font-size: 64px;
  color: var(--text-tertiary);
  margin-bottom: var(--space-6);
}

.empty-title {
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--space-3) 0;
}

.empty-description {
  color: var(--text-secondary);
  margin: 0 0 var(--space-6) 0;
  line-height: 1.5;
}

.posts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
  gap: var(--space-6);
}

.post-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-6);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
  transition: var(--transition-normal);
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  height: fit-content;
}

.post-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
  border-color: var(--color-primary);
}

.post-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.post-meta {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-3);
}

.author-info {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex: 1;
}

.author-avatar {
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  color: white;
  font-weight: 600;
  flex-shrink: 0;
}

.author-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.author-name {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.post-time {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.post-badges {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}

.post-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  flex: 1;
}

.post-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-excerpt {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
}

.post-tag {
  font-size: var(--font-size-xs);
  padding: var(--space-1) var(--space-2);
}

.more-tags {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  font-weight: 500;
}

.post-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  padding-top: var(--space-3);
  border-top: 1px solid var(--border-color-light);
}

.post-stats {
  display: flex;
  gap: var(--space-4);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
}

.stat-item .el-icon {
  font-size: 16px;
}

.post-category {
  flex-shrink: 0;
}

/* ===== 分页区域 ===== */
.pagination-section {
  margin-top: var(--space-8);
}

.pagination-wrapper {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  padding: var(--space-6);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
}

.pagination-info {
  flex: 1;
}

.info-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.pagination :deep(.el-pager li) {
  border-radius: var(--radius-md);
  margin: 0 var(--space-1);
  transition: var(--transition-fast);
}

/* ===== 创建对话框 ===== */
.create-dialog :deep(.el-dialog) {
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.create-dialog :deep(.el-dialog__header) {
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  color: white;
  padding: var(--space-6);
}

.create-dialog :deep(.el-dialog__title) {
  color: white;
  font-weight: 600;
}

.create-form-container {
  padding: var(--space-6);
}

.create-form :deep(.el-form-item__label) {
  font-weight: 600;
  color: var(--text-primary);
}

.create-form :deep(.el-input__wrapper) {
  border-radius: var(--radius-lg);
  border: 2px solid var(--border-color);
  transition: var(--transition-normal);
}

.create-form :deep(.el-input__wrapper):hover {
  border-color: var(--color-primary);
}

.create-form :deep(.el-input__wrapper.is-focus) {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.create-form :deep(.el-textarea__inner) {
  border-radius: var(--radius-lg);
  border: 2px solid var(--border-color);
  transition: var(--transition-normal);
  line-height: 1.6;
}

.create-form :deep(.el-textarea__inner):focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding: var(--space-6);
  background: var(--bg-elevated);
  border-top: 1px solid var(--border-color);
}

.dialog-footer .el-button {
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-6);
  font-weight: 600;
}

/* ===== 响应式设计 ===== */
@media (max-width: 1024px) {
  .header-content {
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-6);
    text-align: center;
  }
  
  .filters-area {
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  }
  
  .posts-grid {
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  }
}

@media (max-width: 768px) {
  .content-container {
    padding: 0 var(--space-4);
  }
  
  .page-header {
    padding: var(--space-8) 0 var(--space-6);
  }
  
  .header-container {
    padding: 0 var(--space-4);
  }
  
  .page-title {
    font-size: var(--font-size-2xl);
  }
  
  .page-description {
    font-size: var(--font-size-base);
  }
  
  .filter-card {
    padding: var(--space-6);
  }
  
  .search-input-wrapper {
    flex-direction: column;
    gap: var(--space-3);
  }
  
  .search-btn {
    width: 100%;
  }
  
  .filters-area {
    grid-template-columns: 1fr;
  }
  
  .posts-grid {
    grid-template-columns: 1fr;
  }
  
  .post-card {
    padding: var(--space-4);
  }
  
  .pagination-wrapper {
    flex-direction: column;
    gap: var(--space-4);
    text-align: center;
  }
}

@media (max-width: 480px) {
  .content-container {
    padding: 0 var(--space-3);
  }
  
  .header-container {
    padding: 0 var(--space-3);
  }
  
  .filter-card,
  .tags-card {
    padding: var(--space-4);
  }
  
  .create-dialog :deep(.el-dialog) {
    width: 95% !important;
    margin: var(--space-4);
  }
}
</style> 