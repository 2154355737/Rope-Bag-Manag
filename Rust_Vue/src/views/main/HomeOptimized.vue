<template>
  <div class="home-optimized">
    <!-- 固定头部 -->
    <HomeHeader
      :system-settings="systemSettings"
      :suggestions="searchSuggestions"
      @search="handleSearch"
    />

    <!-- 主容器 -->
    <div class="home-container">
      <!-- 三栏布局 -->
      <div class="three-column-layout">
        <!-- 左侧边栏 -->
        <HomeSidebar
          :categories="categories"
          :announcements="announcements"
          :active-category="activeCategory"
          :total-resources="totalResources"
          :resources="resources"
          @category-change="handleCategoryChange"
        />

        <!-- 主内容区 -->
                 <HomeMainContent
           :carousel-items="carouselItems"
           :items="currentItems"
           :content-type="contentType"
           :sort-by="sortBy"
           :total-count="totalCount"
           :loading="loading"
           :current-page="currentPage"
           :page-size="pageSize"
           @content-type-change="(type: string) => handleContentTypeChange(type)"
           @sort-change="handleSortChange"
           @page-change="handlePageChange"
           @size-change="handleSizeChange"
           @item-click="handleItemClick"
           @item-download="handleItemDownload"
           @reset-filters="handleResetFilters"
         />

        <!-- 右侧边栏 - 懒加载 -->
        <Suspense>
          <template #default>
            <RightSidebar
              v-if="!isMobile"
              :popular-content="popularContent"
              :recent-activities="recentActivities"
              :hot-tags="hotTags"
              :loading-states="rightSidebarLoading"
              @refresh-popular="fetchPopularContent"
              @refresh-activities="fetchRecentActivities"
              @refresh-tags="fetchPopularTags"
              @tag-click="handleTagClick"
            />
          </template>
          <template #fallback>
            <div class="sidebar-loading">
              <el-skeleton :rows="8" animated />
            </div>
          </template>
        </Suspense>
      </div>
    </div>

    <!-- 页脚 -->
    <HomeFooter :system-settings="systemSettings" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, shallowRef, markRaw, nextTick, defineAsyncComponent } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { debounce } from '@/utils/debounce'
import { packageApi } from '@/api/packages'
import { categoryApi } from '@/api/categories'
import { getActiveAnnouncements } from '@/api/announcements'
import { getPosts } from '@/api/posts'
import { communityApi } from '@/api/community'
import { handleDownloadError } from '@/utils/downloadErrorHandler'
import { useSettings } from '@/composables/useSettings'
import type { Package } from '@/api/packages'
import type { Category } from '@/api/categories'
import type { Post } from '@/api/posts'

// 懒加载组件
import HomeHeader from '@/components/home/HomeHeader.vue'
import HomeSidebar from '@/components/home/HomeSidebar.vue'
import HomeMainContent from '@/components/home/HomeMainContent.vue'
import HomeFooter from '@/components/home/HomeFooter.vue'

// 异步组件
const RightSidebar = defineAsyncComponent(() => import('@/components/home/RightSidebar.vue'))

// Router
const router = useRouter()

// Settings composable
const { systemSettings, loadSystemSettings } = useSettings()

// 响应式数据 - 使用shallowRef优化大数组
const loading = ref(false)
const categories = shallowRef<Category[]>([])
const announcements = shallowRef<Array<{ id: number; text: string; created_at: string }>>([])
const resources = shallowRef<Package[]>([])
const posts = shallowRef<Post[]>([])
const carouselItems = shallowRef<Array<{ title: string; description?: string; image?: string; preview?: string; link?: string; tags?: string[] }>>([])

// 搜索和筛选
const searchQuery = ref('')
const activeCategory = ref('all')
const contentType = ref<'resources' | 'posts'>('resources') // 修复类型错误
const sortBy = ref('created_at')

// 分页
const currentPage = ref(1)
const pageSize = ref(12)
const totalResources = ref(0)
const totalPosts = ref(0)

// 右侧边栏数据
const popularContent = shallowRef<Array<{ id: number; type: string; title: string; views: number; downloads?: number }>>([])
const recentActivities = shallowRef<Array<{ id: number; username: string; action: string; target: string; created_at: string }>>([])
const hotTags = shallowRef<Array<{ id: number; name: string; count: number; color: string }>>([])
const rightSidebarLoading = ref({
  popular: false,
  activities: false,
  tags: false
})

// 响应式检测
const isMobile = ref(window.innerWidth <= 768)

// 计算属性
const totalCount = computed(() => {
  return contentType.value === 'resources' ? totalResources.value : totalPosts.value
})

// 实现置顶-精华-普通排序逻辑
const sortedResources = computed(() => {
  return [...resources.value].sort((a, b) => {
    // 置顶优先级最高
    if (a.is_pinned && !b.is_pinned) return -1;
    if (!a.is_pinned && b.is_pinned) return 1;
    
    // 其次是精华
    if (a.is_featured && !b.is_featured) return -1;
    if (!a.is_featured && b.is_featured) return 1;
    
    // 最后按照当前排序方式
    if (sortBy.value === 'created_at') {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    } else if (sortBy.value === 'download_count') {
      return (b.download_count || 0) - (a.download_count || 0);
    }
    return 0;
  });
});

const sortedPosts = computed(() => {
  return [...posts.value].sort((a, b) => {
    // 置顶优先级最高
    if (a.is_pinned && !b.is_pinned) return -1;
    if (!a.is_pinned && b.is_pinned) return 1;
    
    // 其次是精华
    if (a.is_featured && !b.is_featured) return -1;
    if (!a.is_featured && b.is_featured) return 1;
    
    // 最后按照发布时间排序
    return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
  });
});

// 确保应用排序规则
const currentItems = computed(() => {
  // 重要：先排序，然后再应用分页逻辑
  const sorted = contentType.value === 'resources' ? sortedResources.value : sortedPosts.value;
  return sorted;
});

// 搜索建议 - 优化版本
const searchSuggestions = computed(() => {
  if (!searchQuery.value.trim()) return []
  
  const query = searchQuery.value.toLowerCase()
  const suggestions: Array<{ type: string; text: string }> = []
  
  // 只在前20个资源中搜索，减少计算量
  const sampleResources = resources.value.slice(0, 20)
  sampleResources.forEach(resource => {
    if (resource.name.toLowerCase().includes(query)) {
      suggestions.push({
        type: '资源',
        text: resource.name
      })
    }
  })
  
  return suggestions.slice(0, 5)
})

// 防抖搜索
const debouncedSearch = debounce(async (query: string) => {
  searchQuery.value = query
  await loadData()
}, 500)

// 数据加载方法
const loadData = async () => {
  loading.value = true
  try {
    if (contentType.value === 'resources') {
      await loadResources()
    } else {
      await loadPosts()
    }
  } finally {
    loading.value = false
  }
}

// 存储全部资源，用于前端排序和分页
const allResources = shallowRef<Package[]>([])
const allPosts = shallowRef<Post[]>([])

const loadResources = async () => {
  try {
    // 修改为一次性请求更多数据，或者不指定分页参数获取所有数据
    const params = {
      // 不传递分页参数，或传递一个很大的页面大小
      pageSize: 1000, // 尝试获取足够多的数据
      sort: sortBy.value,
      search: searchQuery.value || undefined,
      category_id: activeCategory.value !== 'all' ? parseInt(activeCategory.value) : undefined
    }
    
    const res = await packageApi.getPackages(params)
    if (res.code === 0 && res.data) {
      // 保存所有数据
      allResources.value = markRaw(res.data.list || [])
      totalResources.value = res.data.total || 0
      
      // 更新当前显示的数据
      updateResourcesPage()
    }
  } catch (error) {
    console.error('加载资源失败:', error)
    ElMessage.error('加载资源失败')
  }
}

// 根据当前页码和排序规则更新数据
const updateResourcesPage = () => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  
  // 先应用排序规则
  const sorted = [...allResources.value].sort((a, b) => {
    // 置顶优先级最高
    if (a.is_pinned && !b.is_pinned) return -1
    if (!a.is_pinned && b.is_pinned) return 1
    
    // 其次是精华
    if (a.is_featured && !b.is_featured) return -1
    if (!a.is_featured && b.is_featured) return 1
    
    // 最后按照当前排序方式
    if (sortBy.value === 'created_at') {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    } else if (sortBy.value === 'download_count') {
      return (b.download_count || 0) - (a.download_count || 0)
    }
    return 0
  })
  
  // 应用分页
  resources.value = sorted.slice(start, end)
}

const loadPosts = async () => {
  try {
    const params = {
      // 不使用分页或使用大页面尺寸
      page_size: 1000,
      status: 'Published'
    }
    
    const res = await getPosts(params)
    if (res.code === 0 && res.data) {
      allPosts.value = markRaw(res.data.list || [])
      totalPosts.value = res.data.total || 0
      
      // 更新当前显示的帖子
      updatePostsPage()
    }
  } catch (error) {
    console.error('加载帖子失败:', error)
    ElMessage.error('加载帖子失败')
  }
}

// 根据当前页码和排序规则更新帖子数据
const updatePostsPage = () => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  
  // 先应用排序规则
  const sorted = [...allPosts.value].sort((a, b) => {
    // 置顶优先级最高
    if (a.is_pinned && !b.is_pinned) return -1
    if (!a.is_pinned && b.is_pinned) return 1
    
    // 其次是精华
    if (a.is_featured && !b.is_featured) return -1
    if (!a.is_featured && b.is_featured) return 1
    
    // 最后按照发布时间排序
    return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
  })
  
  // 应用分页
  posts.value = sorted.slice(start, end)
}

const loadBasicData = async () => {
  try {
    // 并行加载基础数据
    const [categoriesRes, announcementsRes] = await Promise.all([
      categoryApi.getCategories().catch(() => ({ code: -1, data: null })),
      getActiveAnnouncements().catch(() => ({ code: -1, data: null }))
    ])
    
    if (categoriesRes.code === 0 && categoriesRes.data) {
      categories.value = markRaw(categoriesRes.data.list || [])
    }
    
    if (announcementsRes.code === 0 && announcementsRes.data) {
      const announcementList = Array.isArray(announcementsRes.data) 
        ? announcementsRes.data 
        : announcementsRes.data.list || []
      
      announcements.value = markRaw(
        announcementList.map((item: any) => ({
          id: item.id,
          text: `${item.title}: ${item.content}`,
          created_at: item.created_at || new Date().toISOString()
        }))
      )
    }
  } catch (error) {
    console.error('加载基础数据失败:', error)
  }
}

// 懒加载右侧边栏数据
const loadRightSidebarData = async () => {
  // 延迟1秒加载右侧边栏，优化首屏性能
  setTimeout(async () => {
    await Promise.all([
      fetchPopularContent(),
      fetchRecentActivities(),
      fetchPopularTags()
    ])
  }, 1000)
}

const fetchPopularContent = async () => {
  if (rightSidebarLoading.value.popular) return
  
  rightSidebarLoading.value.popular = true
  try {
    // 简化的热门内容获取
    const popularItems = resources.value
      .slice()
      .sort((a, b) => (b.download_count || 0) - (a.download_count || 0))
      .slice(0, 8)
      .map(item => ({
        id: item.id,
        type: 'resource',
        title: item.name,
        views: item.download_count || 0,
        downloads: item.download_count || 0
      }))
    
    popularContent.value = markRaw(popularItems)
  } catch (error) {
    console.error('获取热门内容失败:', error)
  } finally {
    rightSidebarLoading.value.popular = false
  }
}

const fetchRecentActivities = async () => {
  if (rightSidebarLoading.value.activities) return
  
  rightSidebarLoading.value.activities = true
  try {
    // 模拟数据，避免复杂API调用
    const activities = []
    for (let i = 0; i < 5; i++) {
      activities.push({
        id: i,
        username: `用户${i + 1}`,
        action: '下载了',
        target: resources.value[i]?.name || '某个资源',
        created_at: new Date(Date.now() - i * 3600000).toISOString()
      })
    }
    recentActivities.value = markRaw(activities)
  } catch (error) {
    console.error('获取社区动态失败:', error)
  } finally {
    rightSidebarLoading.value.activities = false
  }
}

const fetchPopularTags = async () => {
  if (rightSidebarLoading.value.tags) return
  
  rightSidebarLoading.value.tags = true
  try {
    // 简化的标签数据
    const colors = ['#409EFF', '#67C23A', '#E6A23C', '#F56C6C', '#909399']
    const tags = ['Vue', 'React', 'JavaScript', 'TypeScript', 'Node.js']
      .map((name, index) => ({
        id: index + 1,
        name,
        count: Math.floor(Math.random() * 100) + 10,
        color: colors[index % colors.length]
      }))
    
    hotTags.value = markRaw(tags)
  } catch (error) {
    console.error('获取热门标签失败:', error)
  } finally {
    rightSidebarLoading.value.tags = false
  }
}

// 事件处理方法
const handleSearch = (query: string) => {
  debouncedSearch(query)
}

const handleCategoryChange = (category: string) => {
  activeCategory.value = category
  currentPage.value = 1
  loadData()
}

const handleContentTypeChange = (type: string) => {
  contentType.value = type as 'resources' | 'posts'
  currentPage.value = 1
  loadData()
}

const handleSortChange = (sort: string) => {
  sortBy.value = sort
  // 如果是资源类型，使用本地排序
  if (contentType.value === 'resources') {
    updateResourcesPage()
  } else {
    updatePostsPage()
  }
}

const handlePageChange = (page: number) => {
  currentPage.value = page
  // 应用本地分页
  if (contentType.value === 'resources') {
    updateResourcesPage()
  } else {
    updatePostsPage()
  }
}

const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  // 应用本地分页
  if (contentType.value === 'resources') {
    updateResourcesPage()
  } else {
    updatePostsPage()
  }
}

const handleItemClick = (item: any) => {
  if (contentType.value === 'resources') {
    router.push(`/resource/${item.id}`)
  } else {
    router.push(`/posts/${item.id}`)
  }
}

const handleItemDownload = async (item: any) => {
  try {
    const res = await communityApi.downloadResource(item.id)
    if (res.code === 0) {
      ElMessage.success('下载成功')
      // 更新下载数量
      const index = resources.value.findIndex(r => r.id === item.id)
      if (index !== -1) {
        resources.value[index].download_count = (resources.value[index].download_count || 0) + 1
      }
    } else {
      ElMessage.error(res.msg || '下载失败')
    }
  } catch (error) {
    handleDownloadError(error, '下载失败')
  }
}

const handleResetFilters = () => {
  searchQuery.value = ''
  activeCategory.value = 'all'
  sortBy.value = 'created_at'
  currentPage.value = 1
  loadData()
}

const handleTagClick = (tagName: string) => {
  searchQuery.value = tagName
  loadData()
}

// 响应式处理
const handleResize = debounce(() => {
  isMobile.value = window.innerWidth <= 768
}, 150)

// 生命周期
onMounted(async () => {
  try {
    // 优先加载系统设置
    await loadSystemSettings()
    
    // 并行加载基础数据和主要内容
    await Promise.all([
      loadBasicData(),
      loadData()
    ])
    
    // 懒加载右侧边栏数据
    if (!isMobile.value) {
      loadRightSidebarData()
    }
    
    // 添加响应式监听
    window.addEventListener('resize', handleResize)
  } catch (error) {
    console.error('页面初始化失败:', error)
    ElMessage.error('页面加载失败，请刷新重试')
  }
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<style scoped>
.home-optimized {
  min-height: 100vh;
  background: var(--bg-primary);
}

.home-container {
  padding-top: 72px; /* Header height */
  min-height: calc(100vh - 72px);
}

.three-column-layout {
  max-width: 1440px;
  margin: 0 auto;
  padding: 24px;
  display: grid;
  grid-template-columns: 300px 1fr 320px;
  gap: 24px;
  align-items: start;
  min-height: calc(100vh - 120px);
}

.sidebar-loading {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color-light);
  border-radius: 16px;
  padding: 20px;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .three-column-layout {
    grid-template-columns: 280px 1fr 300px;
    gap: 20px;
  }
}

@media (max-width: 1024px) {
  .three-column-layout {
    grid-template-columns: 1fr;
    gap: 20px;
  }
  
  .home-container {
    padding-top: 72px;
  }
}

@media (max-width: 768px) {
  .three-column-layout {
    padding: 16px;
    gap: 16px;
  }
}

@media (max-width: 480px) {
  .three-column-layout {
    padding: 12px;
    gap: 12px;
  }
}

/* 性能优化 */
.home-optimized * {
  box-sizing: border-box;
}

/* 减少重绘和回流 */
.three-column-layout {
  contain: layout style paint;
}

/* 减少动画开销 */
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
</style> 