<template>
  <div class="community-home">
    <!-- 顶部导航栏 -->
    <div class="community-header">
      <div class="header-content">
        <div class="logo-section">
          <div class="logo-container">
            <div class="logo-icon">📚</div>
            <div class="logo-text">
              <h1 class="site-title">资源社区</h1>
              <p class="site-subtitle">分享、发现、学习</p>
            </div>
          </div>
        </div>
        
        <div class="search-section">
          <el-input
            v-model="searchQuery"
            placeholder="搜索资源..."
            class="search-input"
            clearable
            @keyup.enter="handleSearch"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          <el-button type="primary" @click="handleSearch" class="search-btn">
            <el-icon><Search /></el-icon>
            搜索
          </el-button>
        </div>
        
        <div class="user-section">
          <ThemeSwitcher />
          <el-button v-if="!isLoggedIn" type="primary" @click="goToLogin" class="login-btn">
            <el-icon><User /></el-icon>
            登录
          </el-button>
          <el-button v-if="isLoggedIn" type="success" @click="goToAdmin" class="admin-btn">
            <el-icon><Setting /></el-icon>
            管理后台
          </el-button>
        </div>
      </div>
    </div>

    <!-- 分类导航 -->
    <div class="category-nav">
      <div class="category-container">
        <el-tabs v-model="activeCategory" @tab-click="handleCategoryChange" class="category-tabs">
          <el-tab-pane label="全部" name="all"></el-tab-pane>
          <el-tab-pane label="热门" name="hot"></el-tab-pane>
          <el-tab-pane label="最新" name="latest"></el-tab-pane>
          <el-tab-pane label="推荐" name="recommended"></el-tab-pane>
          <el-tab-pane label="教程" name="tutorial"></el-tab-pane>
          <el-tab-pane label="工具" name="tool"></el-tab-pane>
          <el-tab-pane label="模板" name="template"></el-tab-pane>
        </el-tabs>
      </div>
    </div>

    <!-- 筛选工具栏 -->
    <div class="filter-toolbar">
      <div class="filter-content">
        <div class="filter-left">
          <el-select v-model="sortBy" placeholder="排序方式" @change="handleSortChange" class="filter-select">
            <el-option label="最新发布" value="latest"></el-option>
            <el-option label="最多下载" value="downloads"></el-option>
            <el-option label="最多点赞" value="likes"></el-option>
            <el-option label="最多收藏" value="favorites"></el-option>
          </el-select>
          
          <el-select v-model="filterType" placeholder="资源类型" @change="handleFilterChange" class="filter-select">
            <el-option label="全部类型" value="all"></el-option>
            <el-option label="免费资源" value="free"></el-option>
            <el-option label="付费资源" value="paid"></el-option>
          </el-select>
        </div>
        
        <div class="filter-right">
          <el-button type="primary" @click="showUploadDialog = true" class="upload-btn">
            <el-icon><Upload /></el-icon>
            上传资源
          </el-button>
        </div>
      </div>
    </div>

    <!-- 资源列表 -->
    <div class="resources-container">
      <div class="resources-grid">
        <div 
          v-for="resource in filteredResources" 
          :key="resource.id" 
          class="resource-card"
          @click="viewResource(resource.id)"
        >
                      <div class="resource-image">
              <img :src="resource.cover || '/placeholder.jpg'" :alt="resource.绳包名称">
            <div class="resource-overlay">
              <el-button type="primary" size="small" @click.stop="downloadResource(resource.id)" class="download-btn">
                <el-icon><Download /></el-icon>
                下载
              </el-button>
            </div>
            <div class="resource-badge" v-if="resource.category">
              {{ getCategoryLabel(resource.category) }}
            </div>
          </div>
          
          <div class="resource-info">
            <h3 class="resource-title">{{ resource.绳包名称 }}</h3>
            <p class="resource-description">{{ resource.简介 }}</p>
            
            <div class="resource-meta">
              <div class="meta-item">
                <el-icon><User /></el-icon>
                <span>{{ resource.作者 }}</span>
              </div>
              <div class="meta-item">
                <el-icon><Calendar /></el-icon>
                <span>{{ formatDate(resource.上架时间) }}</span>
              </div>
            </div>
            
            <div class="resource-stats">
              <div class="stat-item">
                <el-icon><Download /></el-icon>
                <span>{{ formatNumber(resource.下载次数) }}</span>
              </div>
              <div class="stat-item">
                <el-icon><Star /></el-icon>
                <span>{{ formatNumber(resource.likes || 0) }}</span>
              </div>
              <div class="stat-item">
                <el-icon><Star /></el-icon>
                <span>{{ formatNumber(resource.favorites || 0) }}</span>
              </div>
            </div>
            
            <div class="resource-tags">
              <el-tag 
                v-for="tag in resource.标签" 
                :key="tag" 
                size="small" 
                class="resource-tag"
              >
                {{ tag }}
              </el-tag>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 分页 -->
      <div class="pagination-container">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :total="totalResources"
          :page-sizes="[12, 24, 48, 96]"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
          class="pagination"
        />
      </div>
    </div>

    <!-- 上传对话框 -->
    <el-dialog
      v-model="showUploadDialog"
      title="上传资源"
      width="600px"
      :before-close="handleUploadClose"
      class="upload-dialog"
    >
      <el-form :model="uploadForm" :rules="uploadRules" ref="uploadFormRef" label-width="100px" class="upload-form">
        <el-form-item label="资源标题" prop="title">
          <el-input v-model="uploadForm.title" placeholder="请输入资源标题"></el-input>
        </el-form-item>
        
        <el-form-item label="资源描述" prop="description">
          <el-input
            v-model="uploadForm.description"
            type="textarea"
            :rows="4"
            placeholder="请输入资源描述"
          ></el-input>
        </el-form-item>
        
        <el-form-item label="资源分类" prop="category">
          <el-select v-model="uploadForm.category" placeholder="选择分类">
            <el-option label="教程" value="tutorial"></el-option>
            <el-option label="工具" value="tool"></el-option>
            <el-option label="模板" value="template"></el-option>
            <el-option label="其他" value="other"></el-option>
          </el-select>
        </el-form-item>
        
        <el-form-item label="资源标签">
          <el-input
            v-model="uploadForm.tagsInput"
            placeholder="输入标签，用逗号分隔"
            @keyup.enter="addTag"
          ></el-input>
          <div class="tags-container">
            <el-tag
              v-for="tag in uploadForm.tags"
              :key="tag"
              closable
              @close="removeTag(tag)"
              class="upload-tag"
            >
              {{ tag }}
            </el-tag>
          </div>
        </el-form-item>
        
        <el-form-item label="资源文件" prop="file">
          <el-upload
            ref="uploadRef"
            :auto-upload="false"
            :on-change="handleFileChange"
            :limit="1"
            accept=".zip,.rar,.7z,.pdf,.doc,.docx"
            class="file-upload"
          >
            <el-button type="primary">选择文件</el-button>
            <template #tip>
              <div class="el-upload__tip">
                支持 zip, rar, 7z, pdf, doc, docx 格式，文件大小不超过100MB
              </div>
            </template>
          </el-upload>
        </el-form-item>
        
        <el-form-item label="封面图片">
          <el-upload
            ref="coverUploadRef"
            :auto-upload="false"
            :on-change="handleCoverChange"
            :limit="1"
            accept="image/*"
            class="cover-upload"
          >
            <el-button type="primary">选择封面</el-button>
            <template #tip>
              <div class="el-upload__tip">
                支持 jpg, png, gif 格式，建议尺寸 300x200
              </div>
            </template>
          </el-upload>
        </el-form-item>
      </el-form>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showUploadDialog = false">取消</el-button>
          <el-button type="primary" @click="submitUpload" :loading="uploading">
            上传资源
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Search,
  User,
  Setting,
  Download,
  Star,
  Calendar,
  Upload
} from '@element-plus/icons-vue'
import type { FormInstance, UploadFile } from 'element-plus'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import { communityApi } from '@/api/community'
import type { Resource, UploadForm } from '../../../types'
import { getUserInfo } from '@/utils/auth'

const router = useRouter()

// 响应式数据
const searchQuery = ref('')
const activeCategory = ref('all')
const sortBy = ref('latest')
const filterType = ref('all')
const currentPage = ref(1)
const pageSize = ref(12)
const totalResources = ref(0)
const showUploadDialog = ref(false)
const uploading = ref(false)
const loading = ref(false)

// 分类标签映射
const categoryLabels = {
  tutorial: '教程',
  tool: '工具',
  template: '模板',
  other: '其他'
}

// 资源数据
const resources = ref<Resource[]>([])

// 上传表单
const uploadForm = reactive<UploadForm>({
  title: '',
  description: '',
  category: '',
  tags: [],
  tagsInput: '',
  file: undefined,
  cover: undefined
})

const uploadRules = {
  title: [
    { required: true, message: '请输入资源标题', trigger: 'blur' },
    { min: 2, max: 50, message: '标题长度在2到50个字符', trigger: 'blur' }
  ],
  description: [
    { required: true, message: '请输入资源描述', trigger: 'blur' },
    { min: 10, max: 500, message: '描述长度在10到500个字符', trigger: 'blur' }
  ],
  category: [
    { required: true, message: '请选择资源分类', trigger: 'change' }
  ],
  file: [
    { required: true, message: '请选择资源文件', trigger: 'change' }
  ]
}

// 计算属性
const isLoggedIn = computed(() => {
  return localStorage.getItem('isLoggedIn') === 'true'
})

const filteredResources = computed(() => {
  let filtered = resources.value

  // 分类筛选
  if (activeCategory.value !== 'all') {
    filtered = filtered.filter(resource => resource.category === activeCategory.value)
  }

  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(resource =>
      resource.绳包名称.toLowerCase().includes(query) ||
      resource.简介.toLowerCase().includes(query) ||
      resource.作者.toLowerCase().includes(query)
    )
  }

  // 排序
  switch (sortBy.value) {
    case 'latest':
      filtered.sort((a, b) => new Date(b.上架时间).getTime() - new Date(a.上架时间).getTime())
      break
    case 'downloads':
      filtered.sort((a, b) => b.下载次数 - a.下载次数)
      break
    case 'likes':
      filtered.sort((a, b) => (b.likes || 0) - (a.likes || 0))
      break
    case 'favorites':
      filtered.sort((a, b) => (b.favorites || 0) - (a.favorites || 0))
      break
  }

  return filtered
})

// 方法
const loadResources = async () => {
  try {
    loading.value = true
    const res = await communityApi.getResources({
      page: currentPage.value,
      pageSize: pageSize.value,
      category: activeCategory.value,
      search: searchQuery.value
    })
    
    if (res.code === 0 && res.data) {
      resources.value = res.data.resources || res.data.绳包列表 || []
      totalResources.value = res.data.total || resources.value.length
    } else {
      ElMessage.error(res.msg || '加载资源失败')
    }
  } catch (error) {
    console.error('加载资源失败:', error)
    ElMessage.error('加载资源失败')
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  currentPage.value = 1
  loadResources()
}

const handleCategoryChange = () => {
  currentPage.value = 1
  loadResources()
}

const handleSortChange = () => {
  loadResources()
}

const handleFilterChange = () => {
  loadResources()
}

const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  loadResources()
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
  loadResources()
}

const viewResource = (id: number) => {
  router.push(`/community/resource/${id}`)
}

const downloadResource = async (id: number) => {
  try {
    const res = await communityApi.downloadResource(id)
    if (res.code === 0) {
      ElMessage.success('下载统计成功')
      // 重新加载资源以更新下载次数
      loadResources()
    } else {
      ElMessage.error(res.msg || '下载失败')
    }
  } catch (error) {
    console.error('下载失败:', error)
    ElMessage.error('下载失败')
  }
}

const goToLogin = () => {
  router.push('/login')
}

const goToAdmin = () => {
  // 调试输出localStorage登录状态
  console.log('isLoggedIn:', localStorage.getItem('isLoggedIn'))
  console.log('userInfo:', localStorage.getItem('userInfo'))
  const user = getUserInfo && getUserInfo()
  if (!user) {
    router.push('/login')
    return
  }
  if (user.role === 'admin' || user.role === 'moderator') {
    router.push('/admin')
  } else if (user.role === 'elder') {
    router.push('/elder')
  } else if (user.role === 'user') {
    router.push('/user')
  } else {
    router.push('/403')
  }
}

const getCategoryLabel = (category: string) => {
  return categoryLabels[category as keyof typeof categoryLabels] || category
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('zh-CN')
}

const formatNumber = (num: number) => {
  return num.toLocaleString()
}

const addTag = () => {
  const tag = uploadForm.tagsInput.trim()
  if (tag && !uploadForm.tags.includes(tag)) {
    uploadForm.tags.push(tag)
    uploadForm.tagsInput = ''
  }
}

const removeTag = (tag: string) => {
  const index = uploadForm.tags.indexOf(tag)
  if (index > -1) {
    uploadForm.tags.splice(index, 1)
  }
}

const handleFileChange = (file: UploadFile) => {
  uploadForm.file = file.raw as File | undefined
}

const handleCoverChange = (file: UploadFile) => {
  uploadForm.cover = file.raw as File | undefined
}

const submitUpload = async () => {
  if (!uploadForm.file) {
    ElMessage.error('请选择资源文件')
    return
  }

  try {
    uploading.value = true
    const res = await communityApi.createResource({
      title: uploadForm.title,
      description: uploadForm.description,
      category: uploadForm.category,
      tags: uploadForm.tags,
      status: 'active',
      file: uploadForm.file,
      cover: uploadForm.cover
    })

    if (res.code === 0) {
      ElMessage.success('资源上传成功')
      showUploadDialog.value = false
      // 重置表单
      Object.assign(uploadForm, {
        title: '',
        description: '',
        category: '',
        tags: [],
        tagsInput: '',
        file: undefined,
        cover: undefined
      })
      // 重新加载资源
      loadResources()
    } else {
      ElMessage.error(res.msg || '上传失败')
    }
  } catch (error) {
    console.error('上传失败:', error)
    ElMessage.error('上传失败')
  } finally {
    uploading.value = false
  }
}

const handleUploadClose = () => {
  showUploadDialog.value = false
  // 重置表单
  Object.assign(uploadForm, {
    title: '',
    description: '',
    category: '',
    tags: [],
    tagsInput: '',
    file: undefined,
    cover: undefined
  })
}

// 生命周期
onMounted(() => {
  loadResources()
})
</script>

<style scoped>
.community-home {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-dark) 100%);
  color: var(--text-primary);
}

.community-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  padding: var(--spacing-lg) 0;
  position: sticky;
  top: 0;
  z-index: var(--z-index-sticky);
  box-shadow: var(--shadow-light);
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 var(--spacing-lg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-lg);
}

.logo-section {
  flex-shrink: 0;
}

.logo-container {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.logo-icon {
  font-size: 2.5rem;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
}

.logo-text {
  display: flex;
  flex-direction: column;
}

.site-title {
  margin: 0;
  font-size: var(--font-size-xxl);
  font-weight: var(--font-weight-bold);
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-dark));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  line-height: 1.2;
}

.site-subtitle {
  margin: var(--spacing-xs) 0 0 0;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: var(--font-weight-normal);
}

.search-section {
  flex: 1;
  max-width: 500px;
  display: flex;
  gap: var(--spacing-md);
}

.search-input {
  flex: 1;
}

.search-input :deep(.el-input__wrapper) {
  border-radius: var(--border-radius-large);
  box-shadow: var(--shadow-light);
  transition: all var(--transition-base);
}

.search-input :deep(.el-input__wrapper:hover) {
  box-shadow: var(--shadow-base);
}

.search-btn {
  border-radius: var(--border-radius-large);
  padding: 0 var(--spacing-lg);
  font-weight: var(--font-weight-medium);
}

.user-section {
  flex-shrink: 0;
  display: flex;
  gap: var(--spacing-md);
}

.login-btn, .admin-btn {
  border-radius: var(--border-radius-large);
  padding: 0 var(--spacing-lg);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-base);
}

.login-btn:hover, .admin-btn:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
}

.category-nav {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  padding: 0 var(--spacing-lg);
}

.category-container {
  max-width: 1200px;
  margin: 0 auto;
}

.category-tabs :deep(.el-tabs__header) {
  margin: 0;
}

.category-tabs :deep(.el-tabs__nav-wrap) {
  padding: var(--spacing-md) 0;
}

.category-tabs :deep(.el-tabs__item) {
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-base);
}

.category-tabs :deep(.el-tabs__item:hover) {
  color: var(--brand-color);
}

.category-tabs :deep(.el-tabs__item.is-active) {
  color: var(--brand-color);
  font-weight: var(--font-weight-semibold);
}

.filter-toolbar {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px);
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
}

.filter-content {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--spacing-lg);
}

.filter-left {
  display: flex;
  gap: var(--spacing-md);
}

.filter-select {
  min-width: 150px;
}

.filter-select :deep(.el-input__wrapper) {
  border-radius: var(--border-radius-base);
}

.upload-btn {
  border-radius: var(--border-radius-large);
  padding: 0 var(--spacing-lg);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-base);
}

.upload-btn:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
}

.resources-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: var(--spacing-xl) var(--spacing-lg);
}

.resources-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-xl);
}

.resource-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: var(--border-radius-extra-large);
  overflow: hidden;
  box-shadow: var(--shadow-light);
  transition: all var(--transition-base);
  cursor: pointer;
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
}

.resource-card:hover {
  transform: translateY(-8px);
  box-shadow: var(--shadow-dark);
  border-color: var(--brand-color-light);
}

.resource-image {
  position: relative;
  height: 200px;
  overflow: hidden;
}

.resource-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--transition-base);
}

.resource-card:hover .resource-image img {
  transform: scale(1.1);
}

.resource-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity var(--transition-base);
}

.resource-card:hover .resource-overlay {
  opacity: 1;
}

.download-btn {
  border-radius: var(--border-radius-base);
  font-weight: var(--font-weight-medium);
}

.resource-badge {
  position: absolute;
  top: var(--spacing-md);
  right: var(--spacing-md);
  background: var(--brand-color);
  color: white;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--border-radius-base);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
}

.resource-info {
  padding: var(--spacing-lg);
}

.resource-title {
  margin: 0 0 var(--spacing-md) 0;
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.resource-description {
  margin: 0 0 var(--spacing-md) 0;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.resource-meta {
  display: flex;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.meta-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.resource-stats {
  display: flex;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.resource-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-xs);
}

.resource-tag {
  font-size: var(--font-size-xs);
  border-radius: var(--border-radius-base);
}

.pagination-container {
  display: flex;
  justify-content: center;
  padding: var(--spacing-lg) 0;
}

.pagination :deep(.el-pagination) {
  --el-pagination-bg-color: rgba(255, 255, 255, 0.9);
  --el-pagination-border-radius: var(--border-radius-base);
  backdrop-filter: blur(10px);
}

.upload-dialog :deep(.el-dialog) {
  border-radius: var(--border-radius-extra-large);
  box-shadow: var(--shadow-dark);
}

.upload-dialog :deep(.el-dialog__header) {
  border-bottom: 1px solid var(--border-color-light);
  padding: var(--spacing-lg);
}

.upload-dialog :deep(.el-dialog__body) {
  padding: var(--spacing-lg);
}

.upload-dialog :deep(.el-dialog__footer) {
  border-top: 1px solid var(--border-color-light);
  padding: var(--spacing-lg);
}

.upload-form :deep(.el-form-item__label) {
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.upload-form :deep(.el-input__wrapper) {
  border-radius: var(--border-radius-base);
}

.tags-container {
  margin-top: var(--spacing-sm);
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-xs);
}

.upload-tag {
  border-radius: var(--border-radius-base);
}

.file-upload, .cover-upload {
  width: 100%;
}

.file-upload :deep(.el-upload), .cover-upload :deep(.el-upload) {
  width: 100%;
}

.file-upload :deep(.el-upload-dragger), .cover-upload :deep(.el-upload-dragger) {
  border-radius: var(--border-radius-base);
  border: 2px dashed var(--border-color);
  transition: all var(--transition-base);
}

.file-upload :deep(.el-upload-dragger:hover), .cover-upload :deep(.el-upload-dragger:hover) {
  border-color: var(--brand-color);
  background-color: var(--bg-secondary);
}

/* 深色模式适配 */
.dark .community-home {
  background: linear-gradient(135deg, var(--brand-color-dark) 0%, var(--brand-color) 100%);
}

.dark .community-header,
.dark .category-nav,
.dark .filter-toolbar {
  background: rgba(44, 44, 44, 0.95);
  border-bottom-color: rgba(255, 255, 255, 0.1);
}

.dark .resource-card {
  background: rgba(44, 44, 44, 0.95);
  border-color: rgba(255, 255, 255, 0.1);
}

.dark .resource-card:hover {
  border-color: var(--brand-color-light);
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .header-content,
  .category-container,
  .filter-content,
  .resources-container {
    max-width: 100%;
    padding-left: var(--spacing-md);
    padding-right: var(--spacing-md);
  }
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    gap: var(--spacing-md);
  }
  
  .search-section {
    max-width: 100%;
  }
  
  .filter-content {
    flex-direction: column;
    gap: var(--spacing-md);
  }
  
  .filter-left {
    width: 100%;
    justify-content: space-between;
  }
  
  .resources-grid {
    grid-template-columns: 1fr;
    gap: var(--spacing-md);
  }
  
  .logo-container {
    justify-content: center;
  }
  
  .site-title {
    font-size: var(--font-size-xl);
  }
}

@media (max-width: 480px) {
  .header-content,
  .category-container,
  .filter-content,
  .resources-container {
    padding-left: var(--spacing-sm);
    padding-right: var(--spacing-sm);
  }
  
  .site-title {
    font-size: var(--font-size-lg);
  }
  
  .logo-icon {
    font-size: 2rem;
  }
  
  .resource-card {
    border-radius: var(--border-radius-large);
  }
  
  .resource-info {
    padding: var(--spacing-md);
  }
}

/* 动画效果 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.resource-card {
  animation: fadeInUp 0.6s ease-out;
}

.resource-card:nth-child(1) { animation-delay: 0.1s; }
.resource-card:nth-child(2) { animation-delay: 0.2s; }
.resource-card:nth-child(3) { animation-delay: 0.3s; }
.resource-card:nth-child(4) { animation-delay: 0.4s; }
.resource-card:nth-child(5) { animation-delay: 0.5s; }
.resource-card:nth-child(6) { animation-delay: 0.6s; }

/* 无障碍支持 */
@media (prefers-reduced-motion: reduce) {
  .resource-card,
  .resource-card:hover,
  .resource-image img,
  .resource-overlay {
    animation: none;
    transition: none;
  }
}

/* 高对比度模式 */
@media (prefers-contrast: high) {
  .resource-card {
    border: 2px solid var(--border-color);
  }
  
  .resource-card:hover {
    border-color: var(--brand-color);
  }
}
</style> 