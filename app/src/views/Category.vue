<template>
  <div class="category-page">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      title="分类浏览"
      left-arrow
      @click-left="onBack"
      fixed
    />
    
    <div class="category-content">
      <!-- 分类列表 -->
      <van-skeleton title :row="8" :loading="loading" v-if="loading && !categories.length" />
      
      <template v-else>
        <div class="category-list">
          <van-sidebar v-model="activeIndex" @change="onSidebarChange">
            <van-sidebar-item 
              v-for="category in categories" 
              :key="category.id" 
              :title="category.name" 
            />
          </van-sidebar>
          
          <div class="category-resources">
            <!-- 当前分类标题 -->
            <div class="current-category" v-if="currentCategory">
              <div class="current-title">{{ currentCategory.name }}</div>
              <div class="current-desc" v-if="currentCategory.description">
                {{ currentCategory.description }}
              </div>
            </div>
            
            <!-- 资源列表 -->
            <resource-list
              :resources="resources"
              :loading="resourceLoading"
              :finished="finished"
              :emptyText="'该分类暂无资源'"
              @resource-click="onResourceClick"
            />
            
            <!-- 加载更多 -->
            <van-loading size="24px" v-if="resourceLoading && resources.length > 0" class="loading-more">
              加载中...
            </van-loading>
          </div>
        </div>
      </template>
    </div>
    
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { showToast } from 'vant';
import { categoryApi, resourceApi } from '../api/resource';
import ResourceList from '../components/ResourceList.vue';
import { sortResources } from '../utils/sorting';

const router = useRouter();
const route = useRoute();

const categories = ref([]);
const loading = ref(false);
const activeIndex = ref(0);

const resources = ref([]);
const resourceLoading = ref(false);
const finished = ref(false);
const page = ref(1);
const pageSize = 10;

// 当前选中的分类
const currentCategory = computed(() => {
  if (!categories.value.length) return null;
  return categories.value[activeIndex.value];
});

// 返回上一页
const onBack = () => {
  router.back();
};

// 侧边栏切换
const onSidebarChange = (index) => {
  activeIndex.value = index;
  // 重置资源列表
  resources.value = [];
  page.value = 1;
  finished.value = false;
  
  // 加载该分类下的资源
  loadCategoryResources();
};

// 点击资源
const onResourceClick = (resource) => {
  router.push(`/resource/${resource.id}`);
};

// 加载分类列表
const loadCategories = async () => {
  loading.value = true;
  try {
    const res = await categoryApi.getAllCategories();
    categories.value = res.data.list || [];
    
    // 如果路由参数中有分类ID，则切换到对应分类
    if (route.params.id) {
      const categoryId = Number(route.params.id);
      const index = categories.value.findIndex(item => item.id === categoryId);
      if (index !== -1) {
        activeIndex.value = index;
      }
    }
    
    // 加载资源
    if (categories.value.length > 0) {
      loadCategoryResources();
    }
  } catch (error) {
    console.error('获取分类失败', error);
    showToast('获取分类失败');
  } finally {
    loading.value = false;
  }
};

// 加载分类下的资源
const loadCategoryResources = async () => {
  if (!currentCategory.value || resourceLoading.value) return;
  
  resourceLoading.value = true;
  try {
    const res = await resourceApi.getResourcesByCategory(
      currentCategory.value.id,
      page.value,
      pageSize
    );
    
    const newResources = res.data.list || [];
    
    // 对资源进行排序：置顶 > 精华 > 创建时间
    const sortedResources = sortResources(newResources, { sortBy: 'created_at', sortOrder: 'desc' });
    
    if (page.value === 1) {
      resources.value = sortedResources;
    } else {
      resources.value = [...resources.value, ...sortedResources];
    }
    
    // 判断是否加载完毕
    if (newResources.length < pageSize) {
      finished.value = true;
    } else {
      page.value += 1;
    }
  } catch (error) {
    console.error('获取分类资源失败', error);
    showToast('获取资源失败');
  } finally {
    resourceLoading.value = false;
  }
};

// 加载更多资源
const loadMoreResources = () => {
  if (finished.value || resourceLoading.value) return;
  loadCategoryResources();
};

// 监听当前分类变化
watch(() => currentCategory.value, () => {
  if (currentCategory.value) {
    resources.value = [];
    page.value = 1;
    finished.value = false;
    loadCategoryResources();
  }
});

// 页面加载时获取分类
onMounted(() => {
  loadCategories();
});
</script>

<style scoped>
/* 底部间距已由全局 .page-content 统一处理 */
.category-page {
  min-height: 100vh;
  background-color: var(--background-color);
}

.category-content {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 46px); /* 仅扣除顶部导航高度，底部间距由全局处理 */
}

.category-list {
  display: flex;
  flex: 1;
  height: 100%;
}

.van-sidebar {
  width: 85px;
  height: 100%;
  overflow-y: auto;
  background-color: #f7f8fa;
  /* 添加右边框增强分离感 */
  border-right: 1px solid var(--border-color);
  /* 优化滚动条样式 */
  scrollbar-width: thin;
  scrollbar-color: rgba(79, 192, 141, 0.3) transparent;
}

.van-sidebar::-webkit-scrollbar {
  width: 3px;
}

.van-sidebar::-webkit-scrollbar-track {
  background: transparent;
}

.van-sidebar::-webkit-scrollbar-thumb {
  background: rgba(79, 192, 141, 0.3);
  border-radius: 2px;
}

.van-sidebar::-webkit-scrollbar-thumb:hover {
  background: rgba(79, 192, 141, 0.5);
}

.category-resources {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.current-category {
  padding: 0 0 12px;
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 12px;
}

.current-title {
  font-size: 18px;
  font-weight: 500;
  color: var(--text-color);
  margin-bottom: 8px;
}

.current-desc {
  font-size: 14px;
  color: var(--text-color-light);
  line-height: 1.5;
}

.loading-more {
  display: flex;
  justify-content: center;
  padding: 16px 0;
}

.van-sidebar-item--select {
  color: var(--primary-color) !important;
  font-weight: 500 !important;
  background-color: rgba(79, 192, 141, 0.08) !important;
}

.van-sidebar-item--select::before {
  background-color: var(--primary-color) !important;
  width: 3px !important;
}

/* 增强选中状态的视觉效果 */
.van-sidebar-item {
  border-radius: 0 8px 8px 0 !important;
  margin: 2px 0 !important;
  transition: all 0.3s ease !important;
  position: relative !important;
}

.van-sidebar-item:hover {
  background-color: rgba(79, 192, 141, 0.04) !important;
  transform: translateX(1px) !important;
}

.van-sidebar-item--select {
  transform: translateX(2px) !important;
  box-shadow: 2px 0 8px rgba(79, 192, 141, 0.15) !important;
}

.van-sidebar-item--select:hover {
  background-color: rgba(79, 192, 141, 0.12) !important;
  transform: translateX(3px) !important;
}
</style> 