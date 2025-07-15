<template>
  <div class="home-container">
    <!-- 顶部导航栏 -->
    <header class="header">
      <div class="header-content">
        <div class="logo">
          <div class="logo-icon">📚</div>
          <div class="logo-text">
            <h1>资源社区</h1>
            <p>分享、发现、学习</p>
          </div>
        </div>
        
        <div class="search">
          <el-input
            v-model="searchQuery"
            placeholder="搜索资源..."
            size="large"
            clearable
            @keyup.enter="handleSearch"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </div>
        
        <div class="actions">
          <ThemeSwitcher />
          <el-button 
            v-if="!isLoggedIn" 
            type="primary" 
            size="large"
            @click="goToLogin"
          >
            <el-icon><User /></el-icon>
            登录
          </el-button>
          <el-button 
            v-if="isLoggedIn" 
            type="success" 
            size="large"
            @click="goToAdmin"
          >
            <el-icon><Setting /></el-icon>
            管理后台
          </el-button>
        </div>
      </div>
    </header>

    <!-- 分类导航 -->
    <nav class="nav">
      <div class="nav-content">
        <el-tabs 
          v-model="activeCategory" 
          @tab-click="handleCategoryChange"
          class="category-tabs"
        >
          <el-tab-pane label="全部" name="all" />
          <el-tab-pane label="热门" name="hot" />
          <el-tab-pane label="最新" name="latest" />
          <el-tab-pane label="推荐" name="recommended" />
          <el-tab-pane label="教程" name="tutorial" />
          <el-tab-pane label="工具" name="tool" />
          <el-tab-pane label="模板" name="template" />
        </el-tabs>
      </div>
    </nav>

    <!-- 筛选工具栏 -->
    <div class="filter-bar">
      <div class="filter-content">
        <div class="filter-left">
          <el-select 
            v-model="sortBy" 
            placeholder="排序方式" 
            @change="handleSortChange"
            size="large"
          >
            <el-option label="最新发布" value="latest" />
            <el-option label="最多下载" value="downloads" />
            <el-option label="最多点赞" value="likes" />
            <el-option label="最多收藏" value="favorites" />
          </el-select>
          
          <el-select 
            v-model="filterType" 
            placeholder="资源类型" 
            @change="handleFilterChange"
            size="large"
          >
            <el-option label="全部类型" value="all" />
            <el-option label="免费资源" value="free" />
            <el-option label="付费资源" value="paid" />
          </el-select>
        </div>
        
        <div class="filter-right">
          <el-button 
            type="primary" 
            size="large"
            @click="showUploadDialog = true"
          >
            <el-icon><Upload /></el-icon>
            上传资源
          </el-button>
        </div>
      </div>
    </div>

    <!-- 主内容区 -->
    <main class="main">
      <div class="main-content">
        <!-- 公告区左侧 -->
        <div class="content-right">
          <div class="notice-card">
            <h3 class="notice-title">社区公告</h3>
            <ul class="notice-list">
              <li v-for="item in notices" :key="item.id">
                {{ item.text }}
              </li>
            </ul>
          </div>
        </div>
        <!-- 主内容区右侧 -->
        <div class="content-left">
          <!-- 统计卡片 -->
          <div class="stats">
            <div class="stats-grid">
              <div class="stat-card">
                <div class="stat-icon">
                  <el-icon><Document /></el-icon>
                </div>
                <div class="stat-info">
                  <div class="stat-number">{{ totalResources }}</div>
                  <div class="stat-label">总资源数</div>
                </div>
              </div>
              
              <div class="stat-card">
                <div class="stat-icon">
                  <el-icon><Download /></el-icon>
                </div>
                <div class="stat-info">
                  <div class="stat-number">{{ totalDownloads }}</div>
                  <div class="stat-label">总下载量</div>
                </div>
              </div>
              
              <div class="stat-card">
                <div class="stat-icon">
                  <el-icon><User /></el-icon>
                </div>
                <div class="stat-info">
                  <div class="stat-number">{{ totalUsers }}</div>
                  <div class="stat-label">注册用户</div>
                </div>
              </div>
              
              <div class="stat-card">
                <div class="stat-icon">
                  <el-icon><Star /></el-icon>
                </div>
                <div class="stat-info">
                  <div class="stat-number">{{ totalLikes }}</div>
                  <div class="stat-label">总点赞数</div>
                </div>
              </div>
            </div>
          </div>

          <!-- 资源列表 -->
          <div class="resources">
            <div v-if="filteredResources.length === 0" class="empty-state">
              <div class="empty-icon">📦</div>
              <h3>暂无资源</h3>
              <p>还没有资源被上传，快来分享你的第一个资源吧！</p>
              <el-button type="primary" @click="showUploadDialog = true">
                <el-icon><Upload /></el-icon>
                上传资源
              </el-button>
            </div>
            
            <div v-else class="resources-grid">
              <div 
                v-for="resource in filteredResources.slice((currentPage-1)*pageSize, currentPage*pageSize)" 
                :key="resource.id"
                class="resource-card"
                @click="viewResource(resource.id)"
              >
                <div class="resource-icon">
                  <el-icon size="28" :color="getCategoryColor(resource.category)">
                    <Document />
                  </el-icon>
                </div>
                <div class="resource-content">
                  <div class="resource-header">
                    <h3 class="resource-title">{{ resource.绳包名称 }}</h3>
                    <span class="resource-badge">{{ getCategoryLabel(resource.category) }}</span>
                  </div>
                  <p class="resource-desc">{{ resource.简介 }}</p>
                  <div class="resource-footer">
                    <div class="resource-meta">
                      <span class="meta-item">
                        <el-icon><User /></el-icon>
                        {{ resource.作者 }}
                      </span>
                      <span class="meta-item">
                        <el-icon><Calendar /></el-icon>
                        {{ formatDate(resource.上架时间) }}
                      </span>
                    </div>
                    <div class="resource-actions">
                      <el-button 
                        type="primary" 
                        size="small"
                        @click.stop="downloadResource(resource.id)"
                      >
                        <el-icon><Download /></el-icon>
                        下载
                      </el-button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- 分页 -->
          <div v-if="filteredResources.length > 0" class="pagination">
            <el-pagination
              v-model:current-page="currentPage"
              v-model:page-size="pageSize"
              :total="totalResources"
              :page-sizes="[18, 36, 54]"
              layout="total, sizes, prev, pager, next, jumper"
              @size-change="handleSizeChange"
              @current-change="handleCurrentChange"
              background
            />
          </div>
        </div>
      </div>
    </main>

    <!-- 上传对话框 -->
    <el-dialog
      v-model="showUploadDialog"
      title="上传资源"
      width="600px"
      :before-close="handleUploadClose"
      destroy-on-close
    >
      <el-form 
        :model="uploadForm" 
        :rules="uploadRules" 
        ref="uploadFormRef" 
        label-width="100px"
        size="large"
      >
        <el-form-item label="资源标题" prop="title">
          <el-input 
            v-model="uploadForm.title" 
            placeholder="请输入资源标题"
            clearable
          />
        </el-form-item>
        
        <el-form-item label="资源描述" prop="description">
          <el-input
            v-model="uploadForm.description"
            type="textarea"
            :rows="4"
            placeholder="请输入资源描述"
            show-word-limit
            maxlength="500"
          />
        </el-form-item>
        
        <el-form-item label="资源分类" prop="category">
          <el-select v-model="uploadForm.category" placeholder="选择分类" style="width: 100%">
            <el-option label="教程" value="tutorial" />
            <el-option label="工具" value="tool" />
            <el-option label="模板" value="template" />
            <el-option label="其他" value="other" />
          </el-select>
        </el-form-item>
        
        <el-form-item label="资源标签">
          <el-input
            v-model="uploadForm.tagsInput"
            placeholder="输入标签，用逗号分隔"
            @keyup.enter="addTag"
            clearable
          />
          <div class="tags-container">
            <el-tag
              v-for="tag in uploadForm.tags"
              :key="tag"
              closable
              @close="removeTag(tag)"
              effect="light"
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
            drag
          >
            <el-icon class="el-icon--upload"><Upload /></el-icon>
            <div class="el-upload__text">
              将文件拖到此处，或<em>点击上传</em>
            </div>
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
            drag
          >
            <el-icon class="el-icon--upload"><Picture /></el-icon>
            <div class="el-upload__text">
              将图片拖到此处，或<em>点击上传</em>
            </div>
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
          <el-button @click="showUploadDialog = false" size="large">取消</el-button>
          <el-button 
            type="primary" 
            @click="submitUpload" 
            :loading="uploading"
            size="large"
          >
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
  Upload,
  Document,
  Collection,
  Picture
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

// 统计数据
const totalDownloads = ref(0)
const totalUsers = ref(0)
const totalLikes = ref(0)

// 公告数据
const notices = [
  { id: 1, text: '欢迎来到资源社区！' },
  { id: 2, text: '请遵守社区规范，文明发言。' },
  { id: 3, text: '资源上传请确保无版权争议。' },
  { id: 4, text: '如遇问题请联系管理员。' },
]

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
    
    // 模拟资源数据
    const mockResources = [
      {
        id: 1,
        绳包名称: 'Vue 3 开发模板',
        简介: '基于 Vue 3 + TypeScript + Element Plus 的现代化开发模板，包含完整的项目结构和常用组件。',
        作者: '张三',
        上架时间: '2024-01-15',
        下载次数: 1250,
        likes: 89,
        favorites: 45,
        category: 'template',
        标签: ['Vue3', 'TypeScript', 'Element Plus']
      },
      {
        id: 2,
        绳包名称: 'React 组件库',
        简介: '一套完整的 React 组件库，包含表单、表格、弹窗等常用组件，支持 TypeScript。',
        作者: '李四',
        上架时间: '2024-01-10',
        下载次数: 890,
        likes: 67,
        favorites: 32,
        category: 'tool',
        标签: ['React', '组件库', 'TypeScript']
      },
      {
        id: 3,
        绳包名称: 'Node.js 后端教程',
        简介: '从零开始学习 Node.js 后端开发，包含 Express、MongoDB、JWT 认证等完整教程。',
        作者: '王五',
        上架时间: '2024-01-08',
        下载次数: 2100,
        likes: 156,
        favorites: 78,
        category: 'tutorial',
        标签: ['Node.js', 'Express', 'MongoDB']
      },
      {
        id: 4,
        绳包名称: 'Python 数据分析工具',
        简介: '基于 Python 的数据分析工具包，包含数据处理、可视化、机器学习等功能。',
        作者: '赵六',
        上架时间: '2024-01-05',
        下载次数: 750,
        likes: 43,
        favorites: 21,
        category: 'tool',
        标签: ['Python', '数据分析', '机器学习']
      },
      {
        id: 5,
        绳包名称: 'Flutter 移动应用模板',
        简介: '完整的 Flutter 移动应用开发模板，包含状态管理、路由、网络请求等常用功能。',
        作者: '孙七',
        上架时间: '2024-01-03',
        下载次数: 680,
        likes: 52,
        favorites: 28,
        category: 'template',
        标签: ['Flutter', '移动开发', 'Dart']
      },
      {
        id: 6,
        绳包名称: 'Docker 部署指南',
        简介: '详细的 Docker 容器化部署指南，包含 Dockerfile 编写、Docker Compose 配置等。',
        作者: '周八',
        上架时间: '2024-01-01',
        下载次数: 950,
        likes: 78,
        favorites: 41,
        category: 'tutorial',
        标签: ['Docker', '容器化', '部署']
      }
    ]
    
    // 模拟API响应
    const res = {
      code: 0,
      data: {
        resources: mockResources,
        total: mockResources.length
      }
    }
    
    if (res.code === 0 && res.data) {
      resources.value = res.data.resources || res.data.绳包列表 || []
      totalResources.value = res.data.total || resources.value.length
      
      // 计算统计数据
      totalDownloads.value = resources.value.reduce((sum, resource) => sum + (resource.下载次数 || 0), 0)
      totalLikes.value = resources.value.reduce((sum, resource) => sum + (resource.likes || 0), 0)
      totalUsers.value = Math.floor(Math.random() * 1000) + 500 // 模拟数据
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

const getCategoryColor = (category: string) => {
  const colors = {
    tutorial: '#67C23A',
    tool: '#409EFF',
    template: '#E6A23C',
    other: '#909399'
  }
  return colors[category as keyof typeof colors] || '#909399'
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
      Object.assign(uploadForm, {
        title: '',
        description: '',
        category: '',
        tags: [],
        tagsInput: '',
        file: undefined,
        cover: undefined
      })
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

onMounted(() => {
  loadResources()
})
</script>

<style scoped>
.home-container {
  min-height: 100vh;
  background: #f6f8fa;
}

/* 顶部导航 */
.header {
  background: #fff;
  border-bottom: 1px solid #e5e7eb;
  padding: 16px 0;
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
  display: flex;
  align-items: center;
  gap: 32px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.logo-icon {
  font-size: 32px;
}

.logo-text h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1f2937;
}

.logo-text p {
  margin: 4px 0 0 0;
  font-size: 14px;
  color: #6b7280;
}

.search {
  flex: 1;
  max-width: 400px;
}

.actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
  margin-left: auto;
}

/* 分类导航 */
.nav {
  background: #fff;
  border-bottom: 1px solid #e5e7eb;
}

.nav-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
}

.category-tabs :deep(.el-tabs__header) {
  margin: 0;
}

.category-tabs :deep(.el-tabs__nav-wrap) {
  padding: 8px 0;
}

.category-tabs :deep(.el-tabs__item) {
  font-weight: 500;
  color: #6b7280;
}

.category-tabs :deep(.el-tabs__item.is-active) {
  color: #3b82f6;
  font-weight: 600;
}

/* 筛选栏 */
.filter-bar {
  background: #fff;
  border-bottom: 1px solid #e5e7eb;
  padding: 16px 0;
}

.filter-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
}

.filter-left {
  display: flex;
  gap: 12px;
}

.filter-left .el-select {
  width: 160px;
}

/* 主内容区 */
.main {
  width: 100%;
  min-height: 0;
  height: auto;
  box-sizing: border-box;
  padding: 32px 0;
}

.main-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
  display: flex;
  gap: 32px;
}

.content-left {
  flex: 1;
  min-width: 0;
}

.content-right {
  width: 280px;
  flex-shrink: 0;
}

/* 统计卡片 */
.stats {
  margin-bottom: 32px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.stat-card {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.stat-card:hover {
  box-shadow: 0 4px 12px 0 rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, #3b82f6, #1d4ed8);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 20px;
}

.stat-info {
  flex: 1;
}

.stat-number {
  font-size: 24px;
  font-weight: 700;
  color: #1f2937;
  line-height: 1;
}

.stat-label {
  font-size: 14px;
  color: #6b7280;
  margin-top: 4px;
}

/* 资源列表 */
.resources {
  margin-bottom: 32px;
}

.empty-state {
  text-align: center;
  padding: 80px 20px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.empty-state h3 {
  margin: 0 0 8px 0;
  font-size: 20px;
  font-weight: 600;
  color: #1f2937;
}

.empty-state p {
  margin: 0 0 24px 0;
  color: #6b7280;
}

.resources-grid {
  width: 100%;
  min-width: 0;
  box-sizing: border-box;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.resource-card {
  background: #fff;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
  cursor: pointer;
}

.resource-card:hover {
  box-shadow: 0 8px 25px 0 rgba(0, 0, 0, 0.15);
  transform: translateY(-4px);
}

.resource-icon {
  position: relative;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-secondary);
  border-radius: 8px 8px 0 0;
}

.resource-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  font-size: 12px;
  padding: 2px 6px;
  background: rgba(0, 0, 0, 0.6);
  color: #fff;
  border-radius: 4px;
}

.resource-content {
  padding: 16px;
}

.resource-actions {
  margin-top: 12px;
  display: flex;
  justify-content: center;
}

.resource-title {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.resource-desc {
  font-size: 12px;
  color: #6b7280;
  margin: 0 0 2px 0;
  line-height: 1.4;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 50%;
}

.resource-meta {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: #9ca3af;
}

.resource-stats {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: #6b7280;
  font-weight: 500;
}

.resource-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* 分页 */
.pagination {
  display: flex;
  justify-content: center;
  padding: 20px 0;
}

/* 公告区 */
.notice-card {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 100px;
}

.notice-title {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
}

.notice-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.notice-list li {
  padding: 8px 0;
  font-size: 14px;
  color: #6b7280;
  line-height: 1.5;
  border-bottom: 1px solid #f3f4f6;
}

.notice-list li:last-child {
  border-bottom: none;
}

/* 上传对话框 */
.upload-dialog :deep(.el-dialog) {
  border-radius: 12px;
}

/* 对话框头部背景透明 */
:deep(.el-dialog__header) {
  background-color: transparent !important;
}

.tags-container {
  margin-top: 8px;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .main-content {
    flex-direction: column;
  }
  
  .content-right {
    width: 100%;
    margin-top: 24px;
  }
  
  .notice-card {
    position: static;
  }
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    gap: 16px;
  }
  
  .search {
    max-width: 100%;
  }
  
  .filter-content {
    flex-direction: column;
    gap: 12px;
  }
  
  .filter-left {
    width: 100%;
    justify-content: space-between;
  }
  
  .filter-left .el-select {
    width: 48%;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .resources-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 480px) {
  .header-content,
  .nav-content,
  .filter-content,
  .main-content {
    padding-left: 16px;
    padding-right: 16px;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .stat-card {
    padding: 16px;
  }
  
  .resource-content {
    padding: 16px;
  }
}

/* 深色模式适配 */
.dark .home-container {
  background: #111827;
}

.dark .header,
.dark .nav,
.dark .filter-bar {
  background: #1f2937;
  border-bottom-color: #374151;
}

.dark .stat-card,
.dark .resource-card,
.dark .notice-card,
.dark .empty-state {
  background: #1f2937;
  color: #f9fafb;
}

.dark .logo-text h1,
.dark .resource-title,
.dark .notice-title {
  color: #f9fafb;
}

.dark .logo-text p,
.dark .stat-label,
.dark .resource-desc,
.dark .meta-item,
.dark .notice-list li {
  color: #9ca3af;
}

.dark .stat-number {
  color: #f9fafb;
}

.dark .resource-stats .stat-item {
  color: #9ca3af;
}

/* 让 el-input 输入框背景透明 */
:deep(.el-input__wrapper) input {
  background-color: transparent !important;
}

/* 主题切换按钮图标居中 */
:deep(.theme-btn) {
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
}

:deep(.theme-icon) {
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
}

/* 主题适配 - 统计图标 */
.stat-icon {
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-dark));
  color: #fff;
}

/* 主题适配 - 标签页 */
.category-tabs :deep(.el-tabs__item) {
  color: var(--text-secondary);
  transition: color var(--transition-base);
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
}

.category-tabs :deep(.el-tabs__item.is-active) {
  color: var(--brand-color);
  font-weight: var(--font-weight-semibold);
}

.category-tabs :deep(.el-tabs__item:hover) {
  color: var(--brand-color-light);
}

/* 主题适配 - 公告标题 */
.notice-title {
  color: var(--text-primary);
  font-weight: var(--font-weight-semibold);
}

/* 深色模式特殊处理 */
.dark .stat-icon {
  background: linear-gradient(135deg, #60a5fa, #3b82f6);
}

.dark .category-tabs :deep(.el-tabs__item) {
  color: #9ca3af;
}

.dark .category-tabs :deep(.el-tabs__item.is-active) {
  color: #60a5fa;
}

.dark .notice-title {
  color: #f9fafb;
}
</style> 