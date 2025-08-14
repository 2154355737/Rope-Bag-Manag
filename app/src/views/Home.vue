<template>
  <div class="home-page">
    <!-- 顶部搜索栏 -->
    <div class="search-bar">
      <van-search
        v-model="searchValue"
        placeholder="搜索资源"
        readonly
        shape="round"
        @click="goToSearch"
      />
      <!-- 通知图标 -->
      <div class="notification-icon" @click="goToNotifications">
        <div class="notification-wrapper">
          <!-- 通知图标 -->
          <i-mdi-bell-ring-outline class="bell-icon" />
          <span v-if="badgeText" class="notification-badge">{{ badgeText }}</span>
        </div>
      </div>
    </div>
    
    <!-- 轮播图 -->
    <div class="container">
      <banner-swiper :items="banners" @banner-click="onBannerClick" />
    </div>
    
    <!-- 分类导航 -->
    <div class="section">
      <div class="section-header">
        <div class="section-title">分类导航</div>
        <div class="section-more" @click="goToCategory">
          全部 <van-icon name="arrow" />
        </div>
      </div>
      <category-list :categories="categories" :loading="categoryLoading" />
    </div>
    
    <!-- 精选资源 -->
    <div class="section">
      <div class="section-header">
        <div class="section-title">精选资源</div>
      </div>
      <div class="container">
        <resource-list 
          :resources="featuredResources" 
          :loading="featuredLoading"
          :finished="true"
          :showCover="true"
          @resource-click="onResourceClick"
        />
      </div>
    </div>
    
    <!-- 最新资源 -->
    <div class="section">
      <div class="section-header">
        <div class="section-title">最新资源</div>
        <div class="section-more" @click="loadMoreResources">
          更多 <van-icon name="arrow" />
        </div>
      </div>
      <div class="container">
        <resource-list 
          :resources="latestResources" 
          :loading="latestLoading"
          :finished="latestFinished"
          :showCover="true"
          @resource-click="onResourceClick"
        />
      </div>
    </div>
    
    <!-- 热门资源 -->
    <div class="section">
      <div class="section-header">
        <div class="section-title">热门资源</div>
        <div class="section-more" @click="loadMoreHotResources">
          更多 <van-icon name="arrow" />
        </div>
      </div>
      <div class="container">
        <resource-list 
          :resources="hotResources" 
          :loading="hotLoading"
          :finished="hotFinished"
          :showCover="true"
          @resource-click="onResourceClick"
        />
      </div>
    </div>
    
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { resourceApi, categoryApi, bannerApi } from '../api/resource';
import { notificationApi } from '../api/notification';
import { showToast } from 'vant';
import { sortResources } from '../utils/sorting';

import BannerSwiper from '../components/BannerSwiper.vue';
import CategoryList from '../components/CategoryList.vue';
import ResourceList from '../components/ResourceList.vue';

const router = useRouter();
const searchValue = ref('');

// 通知徽章
const badgeText = ref('');
let refreshTimer = null;

// 轮播图数据
const banners = ref([]);
const bannerLoading = ref(false);

// 分类数据
const categories = ref([]);
const categoryLoading = ref(false);

// 精选资源数据
const featuredResources = ref([]);
const featuredLoading = ref(false);

// 最新资源数据
const latestResources = ref([]);
const latestLoading = ref(false);
const latestFinished = ref(false);
const latestPage = ref(1);

// 热门资源数据
const hotResources = ref([]);
const hotLoading = ref(false);
const hotFinished = ref(false);
const hotPage = ref(1);

// 加载轮播图数据
const loadBanners = async () => {
  bannerLoading.value = true;
  try {
    const res = await bannerApi.getBanners();
    if (res.data && Array.isArray(res.data)) {
      // 转换后端数据格式为前端组件所需格式
      const mapped = res.data.map(item => {
        const banner = {
          id: item.id,
          title: item.title,
          image: item.image_url,
          type: item.link_type
        };
        
        // 根据链接类型设置对应的ID
        if (item.link_type === 'resource') {
          banner.resourceId = parseInt(item.link_target);
        } else if (item.link_type === 'category') {
          banner.categoryId = parseInt(item.link_target);
        } else if (item.link_type === 'url') {
          banner.url = item.link_target;
        }
        
        return banner;
      });
      banners.value = mapped;
      if (mapped.length === 0) {
        banners.value = [
          { id: 1, title: '精选资源推荐', image: '', type: 'resource', resourceId: 1 },
          { id: 2, title: '新人指南', image: '', type: 'url', url: 'https://example.com/guide' },
          { id: 3, title: '热门分类', image: '', type: 'category', categoryId: 1 }
        ];
      }
    } else {
      // 如果API未返回数据或格式不正确，使用默认数据
      banners.value = [
        {
          id: 1,
          title: '精选资源推荐',
          image: '/img/banner/banner1.jpg',
          type: 'resource',
          resourceId: 1
        },
        {
          id: 2,
          title: '新人指南',
          image: '/img/banner/banner2.jpg',
          type: 'url',
          url: 'https://example.com/guide'
        },
        {
          id: 3,
          title: '热门分类',
          image: '/img/banner/banner3.jpg',
          type: 'category',
          categoryId: 1
        }
      ];
    }
  } catch (error) {
    console.error('获取轮播图失败', error);
    // 使用默认数据作为回退
    banners.value = [
      {
        id: 1,
        title: '精选资源推荐',
        image: '/img/banner/banner1.jpg',
        type: 'resource',
        resourceId: 1
      },
      {
        id: 2,
        title: '新人指南',
        image: '/img/banner/banner2.jpg',
        type: 'url',
        url: 'https://example.com/guide'
      },
      {
        id: 3,
        title: '热门分类',
        image: '/img/banner/banner3.jpg',
        type: 'category',
        categoryId: 1
      }
    ];
  } finally {
    bannerLoading.value = false;
  }
};

// 加载分类数据
const loadCategories = async () => {
  categoryLoading.value = true;
  try {
    const res = await categoryApi.getAllCategories();
    categories.value = res.data.list || [];
  } catch (error) {
    console.error('获取分类失败', error);
    showToast('获取分类失败');
  } finally {
    categoryLoading.value = false;
  }
};

// 加载精选资源
const loadFeaturedResources = async () => {
  featuredLoading.value = true;
  try {
    const res = await resourceApi.getResources({
      page: 1,
      pageSize: 5,
      is_featured: true
    });
    
    const list = res.data.list || [];
    
    // 对精选资源进行排序：置顶 > 普通精选 > 创建时间
    featuredResources.value = sortResources(list, { sortBy: 'created_at', sortOrder: 'desc' });
  } catch (error) {
    console.error('获取精选资源失败', error);
  } finally {
    featuredLoading.value = false;
  }
};

// 加载最新资源
const loadLatestResources = async () => {
  if (latestLoading.value) return;
  
  latestLoading.value = true;
  try {
    const res = await resourceApi.getResources({
      page: latestPage.value,
      pageSize: 5,
      sort_by: 'created_at',
      sort_order: 'desc'
    });
    
    const list = res.data.list || [];
    
    // 对资源进行排序：置顶 > 精华 > 创建时间
    const sortedList = sortResources(list, { sortBy: 'created_at', sortOrder: 'desc' });
    
    if (latestPage.value === 1) {
      latestResources.value = sortedList;
    } else {
      latestResources.value = [...latestResources.value, ...sortedList];
    }
    
    // 判断是否加载完成
    if (list.length < 5) {
      latestFinished.value = true;
    } else {
      latestPage.value += 1;
    }
  } catch (error) {
    console.error('获取最新资源失败', error);
  } finally {
    latestLoading.value = false;
  }
};

// 加载更多最新资源
const loadMoreResources = () => {
  if (latestFinished.value) {
    showToast('没有更多数据了');
    return;
  }
  
  loadLatestResources();
};

// 加载热门资源
const loadHotResources = async () => {
  if (hotLoading.value) return;
  
  hotLoading.value = true;
  try {
    const res = await resourceApi.getResources({
      page: hotPage.value,
      pageSize: 5,
      sort_by: 'download_count',
      sort_order: 'desc'
    });
    
    const list = res.data.list || [];
    
    // 对热门资源进行排序：置顶 > 精华 > 下载量
    const sortedList = sortResources(list, { sortBy: 'download_count', sortOrder: 'desc' });
    
    if (hotPage.value === 1) {
      hotResources.value = sortedList;
    } else {
      hotResources.value = [...hotResources.value, ...sortedList];
    }
    
    // 判断是否加载完成
    if (list.length < 5) {
      hotFinished.value = true;
    } else {
      hotPage.value += 1;
    }
  } catch (error) {
    console.error('获取热门资源失败', error);
  } finally {
    hotLoading.value = false;
  }
};

// 加载更多热门资源
const loadMoreHotResources = () => {
  if (hotFinished.value) {
    showToast('没有更多数据了');
    return;
  }
  
  loadHotResources();
};

// 跳转搜索页
const goToSearch = () => {
  router.push('/search');
};

// 跳转通知页面
const goToNotifications = () => {
  router.push('/notifications');
};

// 加载通知徽章
const loadUnread = async (silent = false) => {
  const token = localStorage.getItem('token');
  if (!token) { 
    badgeText.value = ''; 
    return; 
  }
  
  try {
    const res = await notificationApi.unreadCount();
    const n = res?.data?.count ?? 0;
    badgeText.value = n > 0 ? (n > 99 ? '99+' : String(n)) : '';
  } catch (e) {
    console.warn('获取未读通知数失败:', e);
    if (!silent) {
      badgeText.value = '';
    }
  }
};

// 开始定时刷新
const startRefreshTimer = () => {
  if (refreshTimer) return;
  refreshTimer = setInterval(() => {
    loadUnread(true); // 静默刷新
  }, 30000); // 每30秒刷新一次
};

// 停止定时刷新
const stopRefreshTimer = () => {
  if (refreshTimer) {
    clearInterval(refreshTimer);
    refreshTimer = null;
  }
};

// 跳转分类页
const goToCategory = () => {
  router.push('/category');
};

// 点击资源
const onResourceClick = (resource) => {
  router.push(`/resource/${resource.id}`);
};

// 点击轮播图
const onBannerClick = (banner) => {
  // 轮播图点击事件处理已在组件内部完成
  console.log('Banner clicked', banner);
};

// 页面加载完成时获取数据
onMounted(() => {
  loadBanners();
  loadCategories();
  loadFeaturedResources();
  loadLatestResources();
  loadHotResources();
  loadUnread();
  startRefreshTimer();
  
  // 页面可见性变化时刷新数据
  document.addEventListener('visibilitychange', () => {
    if (!document.hidden) {
      loadUnread(true);
    }
  });
});

onUnmounted(() => {
  stopRefreshTimer();
});
</script>

<style scoped>
/* 底部间距已由全局 .page-content 统一处理 */
.home-page {
}

.search-bar {
  position: sticky;
  top: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  padding-top: calc(8px + env(safe-area-inset-top));
  padding-top: calc(8px + constant(safe-area-inset-top)); /* iOS 11.0 */
  background-color: var(--van-background-2, #fff);
  border-bottom: 1px solid var(--van-border-color, #ebedf0);
}

.search-bar .van-search {
  flex: 1;
  padding: 0;
  background-color: transparent;
}

.search-bar .van-search :deep(.van-search__content) {
  background-color: var(--van-gray-1, #f7f8fa);
  border-radius: 18px;
  padding-left: 12px;
  padding-right: 12px;
}

.notification-icon {
  position: relative;
  padding: 8px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 44px;
  min-height: 44px;
  background-color: transparent;
}

.notification-icon:hover {
  background-color: rgba(25, 137, 250, 0.08);
  transform: translateY(-1px);
}

.notification-icon:active {
  transform: translateY(0) scale(0.96);
  background-color: rgba(25, 137, 250, 0.12);
}

.notification-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.notification-badge {
  position: absolute;
  top: -6px;
  right: -6px;
  background: linear-gradient(135deg, #ff4757, #ff3742);
  color: white;
  border: 2px solid #fff;
  border-radius: 10px;
  min-width: 18px;
  height: 18px;
  font-size: 10px;
  font-weight: 700;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 3px 12px rgba(255, 71, 87, 0.4), 0 1px 4px rgba(0, 0, 0, 0.1);
  animation: subtle-bounce 0.6s ease-out;
}

/* 铃铛图标样式 */
.bell-icon {
  font-size: 22px;
  color: #666;
  transition: color 0.3s ease;
  display: block;
  line-height: 1;
}

.notification-icon:hover .bell-icon {
  color: var(--van-primary-color, #1989fa);
}

/* 通知图标动画 */
@keyframes subtle-bounce {
  0% {
    transform: scale(0.3);
    opacity: 0;
  }
  50% {
    transform: scale(1.05);
    opacity: 1;
  }
  70% {
    transform: scale(0.95);
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}

/* 悬停效果 */
@media (hover: hover) {
  .notification-icon:hover .notification-badge {
    transform: scale(1.05);
    box-shadow: 0 4px 16px rgba(255, 71, 87, 0.5), 0 2px 8px rgba(0, 0, 0, 0.15);
  }
}

.section {
  margin-bottom: 16px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
}

.section-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-color);
  position: relative;
  padding-left: 10px;
}

.section-title::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 16px;
  background-color: var(--primary-color);
  border-radius: 2px;
}

.section-more {
  font-size: 14px;
  color: var(--text-color-lighter);
  display: flex;
  align-items: center;
}

.section-more .van-icon {
  font-size: 12px;
  margin-left: 2px;
}
</style> 