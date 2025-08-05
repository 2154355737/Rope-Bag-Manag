<template>
  <div class="home-main-content">
    <!-- 轮播图区域 -->
    <section v-if="carouselItems.length > 0" class="carousel-section">
      <div class="carousel-container">
        <el-carousel 
          :interval="5000" 
          indicator-position="none"
          arrow="hover"
          height="360px"
          :autoplay="true"
          :loop="true"
        >
          <el-carousel-item 
            v-for="(item, index) in carouselItems" 
            :key="index"
            class="carousel-item"
          >
            <div class="carousel-slide" @click="handleCarouselClick(item)">
              <div class="slide-background">
                <img 
                  :src="item.image || '/api/placeholder/1200/360'" 
                  :alt="item.title || '轮播图片'" 
                  class="background-image"
                  loading="lazy"
                />
                <div class="background-overlay"></div>
              </div>
              
              <div class="slide-content">
                <div class="content-left">
                  <div class="app-icon">📦</div>
                </div>
                <div class="content-center">
                  <h2 class="slide-title">{{ item.title }}</h2>
                  <p class="slide-subtitle">{{ item.description }}</p>
                  <div class="slide-tags" v-if="item.tags?.length">
                    <el-tag 
                      v-for="tag in item.tags.slice(0, 3)" 
                      :key="tag"
                      class="feature-tag"
                      size="small"
                    >
                      {{ tag }}
                    </el-tag>
                  </div>
                </div>
                <div class="content-right" v-if="item.preview">
                  <div class="preview-container">
                    <img :src="item.preview" :alt="item.title + ' 预览'" class="preview-image" />
                  </div>
                </div>
              </div>
            </div>
          </el-carousel-item>
        </el-carousel>
      </div>
    </section>

    <!-- 内容展示区域 -->
    <section class="content-section">
      <div class="content-header">
        <div class="header-left">
          <h2>{{ contentType === 'resources' ? '社区资源' : '社区帖子' }}</h2>
          <span class="content-count">
            共 {{ totalCount }} 个{{ contentType === 'resources' ? '资源' : '帖子' }}
          </span>
        </div>
        <div class="header-right">
          <div class="content-controls">
            <el-select 
              v-model="currentContentType" 
              @change="$emit('content-type-change', currentContentType)"
              size="default"
              popper-class="home-select-popper"
            >
              <el-option label="资源" value="resources" />
              <el-option label="帖子" value="posts" />
            </el-select>
            
            <el-select 
              v-model="currentSortBy" 
              @change="$emit('sort-change', currentSortBy)"
              size="default"
              popper-class="home-select-popper"
            >
              <el-option label="最新发布" value="created_at" />
              <el-option label="下载最多" value="download_count" v-if="contentType === 'resources'" />
              <el-option label="最多查看" value="view_count" v-if="contentType === 'posts'" />
              <el-option label="最多点赞" value="like_count" v-if="contentType === 'posts'" />
              <el-option label="评分最高" value="rating" v-if="contentType === 'resources'" />
            </el-select>
            
            <el-button 
              v-if="contentType === 'posts'" 
              type="primary" 
              @click="goToCreatePost"
            >
              <el-icon><Plus /></el-icon>
              发布帖子
            </el-button>
          </div>
        </div>
      </div>

      <!-- 内容列表 -->
      <div class="content-display">
        <div v-if="loading" class="loading-state">
          <el-skeleton :rows="6" animated />
        </div>
        
        <div v-else-if="!items.length" class="empty-state">
          <el-empty :description="`暂无${contentType === 'resources' ? '资源' : '帖子'}`" :image-size="120">
            <el-button type="primary" @click="resetFilters">
              重置筛选
            </el-button>
          </el-empty>
        </div>
        
        <div v-else class="content-grid">
          <ContentCard
            v-for="item in items"
            :key="item.id"
            :item="item"
            :type="contentType"
            @click="handleItemClick(item)"
            @download="handleItemDownload(item)"
          />
        </div>

        <!-- 分页器 -->
        <div class="pagination-wrapper" v-if="totalCount > 0">
          <el-pagination
            v-model:current-page="currentPage"
            v-model:page-size="pageSize"
            :page-sizes="[12, 24, 48]"
            :total="totalCount"
            layout="total, sizes, prev, pager, next, jumper"
            background
            @size-change="handleSizeChange"
            @current-change="handleCurrentChange"
          />
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { Plus } from '@element-plus/icons-vue'
import { getUserInfo } from '@/utils/auth'
import { ElMessage } from 'element-plus'
import ContentCard from './ContentCard.vue'

// Props
interface CarouselItem {
  title: string
  description?: string
  image?: string
  preview?: string
  link?: string
  tags?: string[]
}

interface Props {
  carouselItems?: CarouselItem[]
  items?: any[]
  contentType?: 'resources' | 'posts'
  sortBy?: string
  totalCount?: number
  loading?: boolean
  currentPage?: number
  pageSize?: number
}

const props = withDefaults(defineProps<Props>(), {
  carouselItems: () => [],
  items: () => [],
  contentType: 'resources',
  sortBy: 'created_at',
  totalCount: 0,
  loading: false,
  currentPage: 1,
  pageSize: 12
})

// Emits
const emit = defineEmits<{
  'content-type-change': [type: string]
  'sort-change': [sort: string]
  'page-change': [page: number]
  'size-change': [size: number]
  'item-click': [item: any]
  'item-download': [item: any]
  'reset-filters': []
}>()

// Router
const router = useRouter()

// 响应式数据
const currentContentType = ref(props.contentType)
const currentSortBy = ref(props.sortBy)

// 计算属性
const userInfo = computed(() => getUserInfo())
const isLoggedIn = computed(() => !!userInfo.value)

// 监听props变化
watch(() => props.contentType, (newType) => {
  currentContentType.value = newType
})

watch(() => props.sortBy, (newSort) => {
  currentSortBy.value = newSort
})

// 方法
const handleCarouselClick = (item: CarouselItem) => {
  if (item.link) {
    if (item.link.startsWith('http')) {
      window.open(item.link, '_blank')
    } else {
      router.push(item.link)
    }
  }
}

const goToCreatePost = () => {
  if (!isLoggedIn.value) {
    ElMessage.warning('请先登录')
    router.push('/login')
    return
  }
  router.push('/posts/create')
}

const handleItemClick = (item: any) => {
  emit('item-click', item)
}

const handleItemDownload = (item: any) => {
  emit('item-download', item)
}

const handleSizeChange = (newSize: number) => {
  emit('size-change', newSize)
}

const handleCurrentChange = (newPage: number) => {
  emit('page-change', newPage)
}

const resetFilters = () => {
  emit('reset-filters')
}
</script>

<style scoped>
.home-main-content {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

/* 轮播图样式 */
.carousel-section {
  margin-bottom: 32px;
}

.carousel-container {
  border-radius: 24px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  border: 1px solid var(--border-color-light);
}

.carousel-slide {
  position: relative;
  width: 100%;
  height: 360px;
  cursor: pointer;
  overflow: hidden;
}

.slide-background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.background-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.6s ease;
}

.carousel-slide:hover .background-image {
  transform: scale(1.05);
}

.background-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    135deg,
    rgba(0, 0, 0, 0.4) 0%,
    rgba(0, 0, 0, 0.2) 50%,
    rgba(0, 0, 0, 0.1) 100%
  );
}

.slide-content {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: center;
  padding: 48px;
  height: 100%;
  gap: 32px;
}

.content-left {
  flex-shrink: 0;
}

.app-icon {
  width: 80px;
  height: 80px;
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(10px);
}

.content-center {
  flex: 1;
  color: white;
}

.slide-title {
  font-size: 36px;
  font-weight: 800;
  margin: 0 0 16px 0;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
  line-height: 1.2;
}

.slide-subtitle {
  font-size: 18px;
  margin: 0 0 24px 0;
  opacity: 0.95;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
  line-height: 1.5;
}

.slide-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.feature-tag {
  background: rgba(255, 255, 255, 0.9) !important;
  color: var(--text-primary) !important;
  border: none !important;
  font-weight: 600;
  backdrop-filter: blur(5px);
}

.content-right {
  flex-shrink: 0;
  max-width: 300px;
}

.preview-container {
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
  border: 2px solid rgba(255, 255, 255, 0.2);
}

.preview-image {
  width: 100%;
  height: 200px;
  object-fit: cover;
  transition: transform 0.4s ease;
}

.carousel-slide:hover .preview-image {
  transform: scale(1.02);
}

/* 内容区域样式 */
.content-section {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color-light);
  border-radius: 20px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
}

.content-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 32px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color-light);
}

.header-left h2 {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.content-count {
  font-size: 14px;
  color: var(--text-secondary);
  font-weight: 500;
}

.content-controls {
  display: flex;
  align-items: center;
  gap: 16px;
}

.content-display {
  padding: 32px;
}

.content-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 24px;
  margin-bottom: 32px;
}

.loading-state,
.empty-state {
  padding: 64px 32px;
  text-align: center;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  padding: 24px 0;
  border-top: 1px solid var(--border-color-light);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .slide-content {
    padding: 32px;
    gap: 24px;
  }
  
  .slide-title {
    font-size: 28px;
  }
  
  .slide-subtitle {
    font-size: 16px;
  }
  
  .content-grid {
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 20px;
  }
}

@media (max-width: 768px) {
  .content-header {
    flex-direction: column;
    gap: 16px;
    padding: 20px 24px;
  }
  
  .header-left {
    text-align: center;
  }
  
  .content-controls {
    justify-content: center;
    flex-wrap: wrap;
    gap: 12px;
  }
  
  .slide-content {
    flex-direction: column;
    text-align: center;
    padding: 24px;
    gap: 20px;
  }
  
  .content-right {
    max-width: 100%;
  }
  
  .preview-container {
    max-width: 250px;
    margin: 0 auto;
  }
  
  .slide-title {
    font-size: 24px;
  }
  
  .slide-subtitle {
    font-size: 14px;
  }
  
  .content-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .content-display {
    padding: 24px;
  }
}

@media (max-width: 480px) {
  .carousel-container {
    border-radius: 16px;
  }
  
  .content-header {
    padding: 16px 20px;
  }
  
  .content-display {
    padding: 20px;
  }
  
  .app-icon {
    width: 60px;
    height: 60px;
    font-size: 24px;
  }
  
  .slide-title {
    font-size: 20px;
  }
}

/* Dropdown style overrides */
:deep(.home-select-popper) {
  background-color: rgba(255, 255, 255, 0.6) !important;
  backdrop-filter: blur(12px) !important;
  border: 1px solid rgba(0, 0, 0, 0.1) !important;
  border-radius: 12px !important;
  box-shadow: var(--shadow-xl) !important;
  z-index: 9999 !important;
}

:deep(.home-select-popper .el-select-dropdown) {
  background: transparent !important;
}

:deep(.home-select-popper .el-select-dropdown__item.hover),
:deep(.home-select-popper .el-select-dropdown__item:hover) {
  background-color: rgba(0, 0, 0, 0.05) !important;
}

:deep(.home-select-popper .el-select-dropdown__item.selected) {
  background-color: rgba(var(--brand-color-rgb), 0.15) !important;
  color: var(--brand-color) !important;
  font-weight: 600;
}

:deep(.home-select-popper .el-popper__arrow::before) {
  display: none !important;
}

html.dark :deep(.home-select-popper) {
  background-color: rgba(30, 41, 59, 0.6) !important;
  border-color: rgba(255, 255, 255, 0.1) !important;
}

html.dark :deep(.home-select-popper .el-select-dropdown__item.hover),
html.dark :deep(.home-select-popper .el-select-dropdown__item:hover) {
  background-color: rgba(255, 255, 255, 0.08) !important;
}
</style> 