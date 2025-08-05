<template>
  <div class="category-page">
    <h1 class="page-title">资源分类</h1>
    
    <!-- 分类列表 -->
    <div class="category-container">
      <el-row :gutter="20">
        <el-col :span="6" class="category-sidebar">
          <h3>所有分类</h3>
          <el-menu
            :default-active="selectedCategoryId ? selectedCategoryId.toString() : ''"
            class="category-menu"
            @select="handleCategorySelect"
          >
            <el-menu-item index="all">
              <el-icon><Grid /></el-icon>
              <span>全部资源</span>
            </el-menu-item>
            <el-menu-item 
              v-for="category in categories" 
              :key="category.id"
              :index="category.id.toString()"
            >
              <el-icon><Folder /></el-icon>
              <span>{{ category.name }}</span>
              <span class="category-count">({{ getResourceCountByCategory(category.id) }})</span>
            </el-menu-item>
          </el-menu>
        </el-col>
        
        <el-col :span="18">
          <!-- 搜索和排序工具栏 -->
          <div class="resource-toolbar">
            <div class="search-box">
              <el-input
                v-model="searchQuery"
                placeholder="搜索当前分类资源..."
                clearable
                @keyup.enter="handleSearch"
              >
                <template #prefix>
                  <el-icon><Search /></el-icon>
                </template>
              </el-input>
            </div>
            
            <div class="sort-options">
              <el-select v-model="sortBy" placeholder="排序方式" @change="handleSortChange">
                <el-option label="最新上传" value="latest" />
                <el-option label="下载最多" value="downloads" />
                <el-option label="点赞最多" value="likes" />
              </el-select>
            </div>
          </div>
          
          <!-- 当前分类信息 -->
          <div v-if="currentCategory" class="category-info">
            <h2>{{ currentCategory.name }}</h2>
            <p v-if="currentCategory.description">{{ currentCategory.description }}</p>
          </div>
          <div v-else class="category-info">
            <h2>全部资源</h2>
            <p>显示所有分类的资源</p>
          </div>
          
          <!-- 资源列表 -->
          <div class="resources-container">
            <el-empty v-if="displayedResources.length === 0" description="该分类下暂无资源" />
            
            <div v-else class="resources-list">
              <el-card v-for="resource in displayedResources" :key="resource.id" class="resource-card">
                <div class="resource-header">
                  <h3>{{ resource.name }}</h3>
                  <el-tag size="small">{{ getCategoryName(resource.category_id) }}</el-tag>
                </div>
                
                <p class="resource-description">{{ resource.description || '暂无描述' }}</p>
                
                <div class="resource-meta">
                  <div class="meta-item">
                    <el-icon><User /></el-icon>
                    <span>{{ resource.author }}</span>
                  </div>
                  <div class="meta-item">
                    <el-icon><Download /></el-icon>
                    <span>{{ resource.download_count }} 次下载</span>
                  </div>
                  <div class="meta-item">
                    <el-icon><Calendar /></el-icon>
                    <span>{{ formatDate(resource.created_at) }}</span>
                  </div>
                </div>
                
                <div class="resource-actions">
                  <el-button type="primary" size="small" @click="viewResource(resource.id)">
                    查看详情
                  </el-button>
                  <el-button type="success" size="small" @click="downloadResource(resource.id)">
                    下载
                  </el-button>
                </div>
              </el-card>
            </div>
            
            <!-- 分页 -->
            <div v-if="displayedResources.length > 0" class="pagination">
              <el-pagination
                v-model:current-page="currentPage"
                v-model:page-size="pageSize"
                :total="totalFilteredResources"
                layout="total, sizes, prev, pager, next, jumper"
                :page-sizes="[12, 24, 48]"
                @size-change="handleSizeChange"
                @current-change="handleCurrentChange"
                background
              />
            </div>
          </div>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Search, Grid, Folder, User, Download, Calendar } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { categoryApi, type Category } from '@/api/categories'
import { packageApi, type Package } from '@/api/packages'
import { formatDate } from '@/utils/format'
import { handleDownloadError } from '@/utils/downloadErrorHandler'

const router = useRouter()

// 状态管理
const loading = ref(false)
const categories = ref<Category[]>([])
const resources = ref<Package[]>([])
const selectedCategoryId = ref<number | null>(null)
const searchQuery = ref('')
const sortBy = ref('latest')
const currentPage = ref(1)
const pageSize = ref(12)

// 获取资源数据
const fetchResources = async () => {
  try {
    loading.value = true
    const params: any = {}
    
    // 如果选择了分类，添加过滤条件
    if (selectedCategoryId.value) {
      params.category_id = selectedCategoryId.value
    }
    
    // 添加搜索条件
    if (searchQuery.value) {
      params.search = searchQuery.value
    }
    
    const res = await packageApi.getPackages(params)
    if (res.code === 0 && res.data) {
      resources.value = res.data.list
    } else {
      ElMessage.error('获取资源数据失败')
    }
  } catch (error) {
    console.error('获取资源数据错误:', error)
    ElMessage.error('获取资源数据出错')
  } finally {
    loading.value = false
  }
}

// 获取分类数据
const fetchCategories = async () => {
  try {
    loading.value = true
    const res = await categoryApi.getCategories()
    if (res.code === 0 && res.data) {
      categories.value = res.data.list || []
      console.log('获取到的分类数据:', categories.value)
    } else {
      ElMessage.error('获取分类数据失败')
    }
  } catch (error) {
    console.error('获取分类数据错误:', error)
    ElMessage.error('获取分类数据出错')
  } finally {
    loading.value = false
  }
}

// 当前选择的分类
const currentCategory = computed(() => {
  if (!selectedCategoryId.value) return null
  return categories.value.find(c => c.id === selectedCategoryId.value) || null
})

// 过滤后的资源列表
const filteredResources = computed(() => {
  if (!selectedCategoryId.value) {
    // 如果没有选择分类，显示所有资源
    return resources.value;
  }
  // 根据分类ID过滤资源
  return resources.value.filter(resource => resource.category_id === selectedCategoryId.value);
});

// 分页后的资源
const displayedResources = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return filteredResources.value.slice(start, end);
});

// 总资源数（用于分页）
const totalFilteredResources = computed(() => {
  return filteredResources.value.length
})

// 获取每个分类中的资源数量
const getResourceCountByCategory = (categoryId: number) => {
  return resources.value.filter(r => r.category_id === categoryId).length
}

// 通过分类ID获取分类名称
const getCategoryName = (categoryId: number | null) => {
  if (!categoryId) return '未分类'
  const category = categories.value.find(c => c.id === categoryId)
  return category ? category.name : '未分类'
}

// 事件处理
const handleCategorySelect = (index: string) => {
  if (index === 'all') {
    selectedCategoryId.value = null
  } else {
    selectedCategoryId.value = parseInt(index)
  }
  currentPage.value = 1 // 重置页码
  fetchResources()
}

const handleSortChange = () => {
  currentPage.value = 1 // 重置页码
}

const handleSearch = () => {
  currentPage.value = 1 // 重置页码
}

const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1 // 重置页码
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
}

// 资源操作
const viewResource = (id: number) => {
  router.push(`/resource/${id}`)
}

const downloadResource = async (id: number) => {
  try {
    const res = await packageApi.downloadPackage(id)
    if (res.code === 0 && res.data) {
      // 根据API实际情况处理下载链接
      const downloadUrl = typeof res.data === 'string' ? res.data : res.data.url
      window.open(downloadUrl, '_blank')
      ElMessage.success('开始下载资源')
    } else {
      ElMessage.error('下载资源失败')
    }
  } catch (error: any) {
    handleDownloadError(error, '下载资源失败')
  }
}

// 初始化
onMounted(async () => {
  await fetchCategories();
  // 默认显示所有资源
  await fetchResources();
  
  // 检查URL参数是否指定了分类
  const urlParams = new URLSearchParams(window.location.search);
  const catId = urlParams.get('category');
  if (catId) {
    selectedCategoryId.value = parseInt(catId);
    fetchResources(); // 根据分类重新加载资源
  }
});
</script>

<style scoped>
.category-page {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
}

.page-title {
  font-size: 24px;
  margin-bottom: 24px;
  color: var(--el-text-color-primary);
}

.category-container {
  background-color: var(--el-bg-color);
  border-radius: 8px;
  box-shadow: var(--el-box-shadow-light);
  overflow: hidden;
}

.category-sidebar {
  padding: 16px;
  border-right: 1px solid var(--el-border-color);
  height: 100%;
  background-color: var(--el-bg-color-overlay);
}

.category-sidebar h3 {
  margin-top: 0;
  margin-bottom: 16px;
  font-size: 16px;
  color: var(--el-text-color-primary);
}

.category-menu {
  border-right: none;
}

.category-count {
  margin-left: 5px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.resource-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding: 16px;
}

.search-box {
  width: 300px;
}

.category-info {
  padding: 0 16px;
  margin-bottom: 24px;
}

.category-info h2 {
  margin-top: 0;
  font-size: 20px;
  color: var(--el-text-color-primary);
}

.category-info p {
  color: var(--el-text-color-secondary);
  margin-top: 8px;
}

.resources-container {
  padding: 0 16px 24px 16px;
}

.resources-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.resource-card {
  transition: all 0.3s;
}

.resource-card:hover {
  transform: translateY(-5px);
  box-shadow: var(--el-box-shadow);
}

.resource-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
}

.resource-header h3 {
  margin: 0;
  font-size: 16px;
  flex: 1;
}

.resource-description {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  margin-bottom: 16px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.resource-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 16px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.resource-actions {
  display: flex;
  gap: 8px;
}

.pagination {
  margin-top: 24px;
  display: flex;
  justify-content: center;
}
</style> 