<template>
  <MobileLayout title="资源社区" :showBack="false">
    <template #header-right>
      <el-icon class="search-icon" @click="goToSearch"><Search /></el-icon>
    </template>

    <!-- 下拉刷新 -->
    <el-pull-refresh v-model="isRefreshing" @refresh="onRefresh">
      <!-- 轮播图 -->
      <div class="banner-section">
        <el-carousel height="150px" indicator-position="inside" :interval="4000">
          <el-carousel-item v-for="(item, index) in featuredResources" :key="index">
            <div class="banner-item" @click="viewResource(item.id)">
              <img :src="item.image_url || defaultImage" :alt="item.name" class="banner-img">
              <div class="banner-title">{{ item.name }}</div>
            </div>
          </el-carousel-item>
        </el-carousel>
      </div>

      <!-- 公告栏 -->
      <div class="announcement-section" v-if="latestAnnouncement" @click="viewAnnouncements">
        <el-icon><Bell /></el-icon>
        <div class="announcement-text">{{ latestAnnouncement.title }}</div>
        <el-icon><ArrowRight /></el-icon>
      </div>

      <!-- 分类导航 -->
      <div class="category-section">
        <div class="category-list">
          <div 
            v-for="category in categories.slice(0, 8)" 
            :key="category.id" 
            class="category-item"
            @click="viewCategory(category.id)"
          >
            <div class="category-icon">
              <el-icon><Folder /></el-icon>
            </div>
            <div class="category-name">{{ category.name }}</div>
          </div>
          <div class="category-item" @click="viewAllCategories">
            <div class="category-icon">
              <el-icon><More /></el-icon>
            </div>
            <div class="category-name">更多</div>
          </div>
        </div>
      </div>

      <!-- 热门资源 -->
      <div class="section-header">
        <h2>热门资源</h2>
        <span class="view-more" @click="viewMore('popular')">查看更多 ></span>
      </div>
      <div class="resources-grid">
        <div 
          v-for="item in popularResources" 
          :key="item.id"
          class="resource-card"
          @click="viewResource(item.id)"
        >
          <div class="resource-image">
            <img :src="item.image_url || defaultImage" :alt="item.name">
          </div>
          <div class="resource-info">
            <div class="resource-name">{{ item.name }}</div>
            <div class="resource-rating">
              <el-rate
                v-model="item.rating"
                disabled
                text-color="#ff9900"
                score-template="{value}"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- 最新更新 -->
      <div class="section-header">
        <h2>最新更新</h2>
        <span class="view-more" @click="viewMore('latest')">查看更多 ></span>
      </div>
      <div class="resources-list">
        <div 
          v-for="item in latestResources" 
          :key="item.id"
          class="resource-list-item"
          @click="viewResource(item.id)"
        >
          <div class="resource-list-content">
            <div class="resource-list-name">{{ item.name }}</div>
            <div class="resource-list-desc">{{ item.description || '暂无描述' }}</div>
          </div>
          <div class="resource-list-rating">
            <el-rate
              v-model="item.rating"
              disabled
              text-color="#ff9900"
              score-template="{value}"
            />
          </div>
        </div>
      </div>

      <!-- 加载更多 -->
      <div class="load-more" v-if="hasMoreLatest" @click="loadMore">
        加载更多...
      </div>
    </el-pull-refresh>
  </MobileLayout>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import MobileLayout from '@/layouts/mobile/MobileLayout.vue'
import { Search, Bell, Folder, More, ArrowRight } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

// 路由
const router = useRouter()

// 默认图片
const defaultImage = '/assets/default-resource.jpg'

// 下拉刷新状态
const isRefreshing = ref(false)

// 数据状态
const featuredResources = ref([])
const popularResources = ref([])
const latestResources = ref([])
const categories = ref([])
const latestAnnouncement = ref(null)
const hasMoreLatest = ref(true)
const currentPage = ref(1)

// 初始加载数据
onMounted(async () => {
  await Promise.all([
    fetchFeaturedResources(),
    fetchPopularResources(),
    fetchLatestResources(),
    fetchCategories(),
    fetchLatestAnnouncement()
  ])
})

// 获取推荐资源
const fetchFeaturedResources = async () => {
  try {
    // 这里替换为实际的API调用
    // const response = await api.getFeaturedResources()
    // featuredResources.value = response.data.list
    
    // 模拟数据
    featuredResources.value = [
      { id: 1, name: '精选资源1', image_url: '', rating: 4.8 },
      { id: 2, name: '精选资源2', image_url: '', rating: 4.7 },
      { id: 3, name: '精选资源3', image_url: '', rating: 4.9 }
    ]
  } catch (error) {
    console.error('获取推荐资源失败:', error)
    ElMessage.error('获取推荐资源失败')
  }
}

// 获取热门资源
const fetchPopularResources = async () => {
  try {
    // 这里替换为实际的API调用
    // const response = await api.getPopularResources()
    // popularResources.value = response.data.list
    
    // 模拟数据
    popularResources.value = [
      { id: 4, name: '热门资源1', image_url: '', rating: 4.5 },
      { id: 5, name: '热门资源2', image_url: '', rating: 4.6 },
      { id: 6, name: '热门资源3', image_url: '', rating: 4.7 },
      { id: 7, name: '热门资源4', image_url: '', rating: 4.8 }
    ]
  } catch (error) {
    console.error('获取热门资源失败:', error)
    ElMessage.error('获取热门资源失败')
  }
}

// 获取最新资源
const fetchLatestResources = async (page = 1) => {
  try {
    // 这里替换为实际的API调用
    // const response = await api.getLatestResources({ page, page_size: 10 })
    // if (page === 1) {
    //   latestResources.value = response.data.list
    // } else {
    //   latestResources.value = [...latestResources.value, ...response.data.list]
    // }
    // hasMoreLatest.value = response.data.list.length === 10
    
    // 模拟数据
    const mockData = [
      { id: 8, name: '最新资源1', description: '这是最新资源1的简介', rating: 4.0 },
      { id: 9, name: '最新资源2', description: '这是最新资源2的简介', rating: 4.2 },
      { id: 10, name: '最新资源3', description: '这是最新资源3的简介', rating: 3.9 }
    ]
    
    if (page === 1) {
      latestResources.value = mockData
    } else {
      latestResources.value = [...latestResources.value, ...mockData]
    }
    
    hasMoreLatest.value = page < 3 // 模拟3页数据
  } catch (error) {
    console.error('获取最新资源失败:', error)
    ElMessage.error('获取最新资源失败')
  }
}

// 获取分类列表
const fetchCategories = async () => {
  try {
    // 这里替换为实际的API调用
    // const response = await api.getCategories()
    // categories.value = response.data.list
    
    // 模拟数据
    categories.value = [
      { id: 1, name: '分类1' },
      { id: 2, name: '分类2' },
      { id: 3, name: '分类3' },
      { id: 4, name: '分类4' },
      { id: 5, name: '分类5' },
      { id: 6, name: '分类6' },
      { id: 7, name: '分类7' },
      { id: 8, name: '分类8' },
      { id: 9, name: '分类9' }
    ]
  } catch (error) {
    console.error('获取分类失败:', error)
    ElMessage.error('获取分类失败')
  }
}

// 获取最新公告
const fetchLatestAnnouncement = async () => {
  try {
    // 这里替换为实际的API调用
    // const response = await api.getLatestAnnouncement()
    // latestAnnouncement.value = response.data
    
    // 模拟数据
    latestAnnouncement.value = {
      id: 1,
      title: '系统公告：资源社区App全新上线，欢迎体验！'
    }
  } catch (error) {
    console.error('获取公告失败:', error)
    // 公告获取失败不显示错误提示
  }
}

// 下拉刷新
const onRefresh = async () => {
  try {
    await Promise.all([
      fetchFeaturedResources(),
      fetchPopularResources(),
      fetchLatestResources(1),
      fetchCategories(),
      fetchLatestAnnouncement()
    ])
    ElMessage.success('刷新成功')
  } catch (error) {
    console.error('刷新失败:', error)
    ElMessage.error('刷新失败')
  } finally {
    isRefreshing.value = false
  }
}

// 加载更多
const loadMore = async () => {
  currentPage.value++
  await fetchLatestResources(currentPage.value)
}

// 跳转到搜索页
const goToSearch = () => {
  router.push('/mobile/search')
}

// 查看资源详情
const viewResource = (id) => {
  router.push(`/mobile/resource/${id}`)
}

// 查看分类
const viewCategory = (id) => {
  router.push(`/mobile/category/${id}`)
}

// 查看所有分类
const viewAllCategories = () => {
  router.push('/mobile/categories')
}

// 查看公告
const viewAnnouncements = () => {
  router.push('/mobile/announcements')
}

// 查看更多资源
const viewMore = (type) => {
  if (type === 'popular') {
    router.push('/mobile/resources?sort=popular')
  } else {
    router.push('/mobile/resources?sort=latest')
  }
}
</script>

<style scoped>
.search-icon {
  font-size: 20px;
  cursor: pointer;
}

.banner-section {
  margin: -15px -15px 15px -15px;
  border-radius: 0;
}

.banner-item {
  position: relative;
  width: 100%;
  height: 100%;
}

.banner-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.banner-title {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 10px;
  background: linear-gradient(to top, rgba(0,0,0,0.7), rgba(0,0,0,0));
  color: white;
  font-weight: 500;
}

.announcement-section {
  display: flex;
  align-items: center;
  background-color: #f8f9fa;
  padding: 10px 15px;
  border-radius: 8px;
  margin-bottom: 15px;
  cursor: pointer;
}

.announcement-section .el-icon {
  color: var(--el-color-danger);
  margin-right: 8px;
}

.announcement-text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 14px;
}

.category-section {
  margin-bottom: 20px;
}

.category-list {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 10px;
}

.category-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
}

.category-icon {
  width: 45px;
  height: 45px;
  background-color: var(--el-color-primary-light-8);
  color: var(--el-color-primary);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 6px;
}

.category-icon .el-icon {
  font-size: 24px;
}

.category-name {
  font-size: 12px;
  text-align: center;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.section-header h2 {
  font-size: 18px;
  margin: 0;
}

.view-more {
  font-size: 14px;
  color: var(--el-color-primary);
  cursor: pointer;
}

.resources-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 15px;
  margin-bottom: 20px;
}

.resource-card {
  border-radius: 8px;
  overflow: hidden;
  background-color: #fff;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  cursor: pointer;
}

.resource-image {
  width: 100%;
  aspect-ratio: 1 / 1;
  overflow: hidden;
}

.resource-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.resource-info {
  padding: 10px;
}

.resource-name {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 5px;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.resources-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
  margin-bottom: 20px;
}

.resource-list-item {
  display: flex;
  background-color: #fff;
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  cursor: pointer;
}

.resource-list-content {
  flex: 1;
}

.resource-list-name {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 5px;
}

.resource-list-desc {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.load-more {
  text-align: center;
  color: var(--el-color-primary);
  padding: 15px 0;
  cursor: pointer;
}

@media (prefers-color-scheme: dark) {
  .announcement-section {
    background-color: var(--el-bg-color-page);
  }
  
  .resource-card,
  .resource-list-item {
    background-color: var(--el-bg-color);
    border: 1px solid var(--el-border-color);
  }
}
</style> 