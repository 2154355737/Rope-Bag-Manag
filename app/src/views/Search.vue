<template>
  <div class="search-page">
    <!-- 搜索框 -->
    <div class="search-header">
      <van-search
        v-model="searchValue"
        show-action
        placeholder="搜索资源"
        @search="onSearch"
        @cancel="onCancel"
        @clear="onClear"
      />
    </div>
    
    <div class="search-content">
      <!-- 搜索历史和热门搜索 -->
      <template v-if="!hasSearched">
        <!-- 搜索历史 -->
        <div class="search-section" v-if="searchHistory.length > 0">
          <div class="section-header">
            <div class="section-title">搜索历史</div>
            <div class="section-action" @click="clearHistory">
              <van-icon name="delete" />
            </div>
          </div>
          <div class="search-tags">
            <van-tag
              v-for="(item, index) in searchHistory"
              :key="`history-${index}`"
              class="search-tag"
              plain
              round
              @click="searchWithKeyword(item)"
            >
              {{ item }}
            </van-tag>
          </div>
        </div>
        
        <!-- 热门搜索 -->
        <div class="search-section">
          <div class="section-header">
            <div class="section-title">热门搜索</div>
          </div>
          <div class="search-tags" v-if="hotSearches.length">
            <van-tag
              v-for="(item, index) in hotSearches"
              :key="`hot-${index}`"
              class="search-tag"
              round
              :type="index < 3 ? 'danger' : ''"
              @click="searchWithKeyword(item)"
            >
              {{ item }}
            </van-tag>
          </div>
          <van-skeleton title :row="3" v-else />
        </div>
        
        <!-- 热门标签 -->
        <div class="search-section">
          <div class="section-header">
            <div class="section-title">热门标签</div>
          </div>
          <div class="search-tags" v-if="hotTags.length">
            <van-tag
              v-for="tag in hotTags"
              :key="`tag-${tag}`"
              class="search-tag"
              plain
              round
              type="primary"
              @click="searchWithKeyword(tag)"
            >
              {{ tag }}
            </van-tag>
          </div>
          <van-skeleton title :row="3" v-else />
        </div>
      </template>
      
      <!-- 搜索结果 -->
      <template v-else>
        <div class="search-result-header">
          <div class="result-count">
            找到 <span class="highlight">{{ totalResults }}</span> 个资源
          </div>
          <div class="result-sort">
            <span 
              class="sort-item" 
              :class="{ active: sortBy === 'relevance' }" 
              @click="sortByRelevance"
            >
              相关性
            </span>
            <span 
              class="sort-item" 
              :class="{ active: sortBy === 'latest' }" 
              @click="sortByLatest"
            >
              最新
            </span>
            <span 
              class="sort-item" 
              :class="{ active: sortBy === 'download' }" 
              @click="sortByDownload"
            >
              下载量
            </span>
          </div>
        </div>
        
        <resource-list 
          :resources="searchResults" 
          :loading="loading"
          :finished="finished"
          :empty-text="'未找到相关资源'"
          @resource-click="onResourceClick"
          @load-more="loadMore"
        />
        
        <!-- 没有更多结果 -->
        <div class="load-more" v-if="searchResults.length > 0 && finished">
          <van-divider>没有更多了</van-divider>
        </div>
      </template>
    </div>
    
    <!-- 底部Tab栏 -->
    <tab-bar />
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import { showToast } from 'vant';
import { resourceApi, tagApi } from '../api/resource';
import ResourceList from '../components/ResourceList.vue';
import TabBar from '../components/TabBar.vue';

const router = useRouter();
const searchValue = ref('');
const hasSearched = ref(false);
const loading = ref(false);
const finished = ref(false);
const searchResults = ref([]);
const totalResults = ref(0);
const page = ref(1);
const pageSize = 10;

// 排序方式
const sortBy = ref('relevance');
const sortOptions = reactive({
  relevance: { sort_by: 'relevance', sort_order: 'desc' },
  latest: { sort_by: 'created_at', sort_order: 'desc' },
  download: { sort_by: 'download_count', sort_order: 'desc' }
});

// 搜索历史
const searchHistory = ref([]);

// 热门搜索
const hotSearches = ref([
  '绳包工具', '初学者教程', '高级技巧', '安全绳', 
  '绳艺教程', '基础套路', '装备推荐', '绳结教程',
  '实用技巧', '进阶教程'
]);

// 热门标签
const hotTags = ref([]);

// 从本地存储加载搜索历史
const loadSearchHistory = () => {
  try {
    const history = localStorage.getItem('searchHistory');
    if (history) {
      searchHistory.value = JSON.parse(history);
    }
  } catch (error) {
    console.error('加载搜索历史失败', error);
  }
};

// 保存搜索历史到本地存储
const saveSearchHistory = (keyword) => {
  try {
    if (!keyword) return;
    
    // 添加到历史记录前面
    const index = searchHistory.value.indexOf(keyword);
    if (index !== -1) {
      // 如果已存在，移除旧的
      searchHistory.value.splice(index, 1);
    }
    
    // 添加到开头
    searchHistory.value.unshift(keyword);
    
    // 限制历史记录数量
    if (searchHistory.value.length > 10) {
      searchHistory.value = searchHistory.value.slice(0, 10);
    }
    
    // 保存到本地存储
    localStorage.setItem('searchHistory', JSON.stringify(searchHistory.value));
  } catch (error) {
    console.error('保存搜索历史失败', error);
  }
};

// 清除搜索历史
const clearHistory = () => {
  searchHistory.value = [];
  localStorage.removeItem('searchHistory');
  showToast('搜索历史已清空');
};

// 加载热门标签
const loadHotTags = async () => {
  try {
    const res = await tagApi.getHotTags(10);
    if (res.data && Array.isArray(res.data)) {
      hotTags.value = res.data.map(tag => tag.name);
    }
  } catch (error) {
    console.error('获取热门标签失败', error);
  }
};

// 使用关键词搜索
const searchWithKeyword = (keyword) => {
  searchValue.value = keyword;
  onSearch();
};

// 执行搜索
const onSearch = () => {
  if (!searchValue.value.trim()) {
    showToast('请输入搜索关键词');
    return;
  }
  
  // 重置搜索状态
  searchResults.value = [];
  page.value = 1;
  finished.value = false;
  hasSearched.value = true;
  
  // 保存搜索历史
  saveSearchHistory(searchValue.value);
  
  // 执行搜索
  doSearch();
};

// 取消搜索
const onCancel = () => {
  router.back();
};

// 清空搜索
const onClear = () => {
  searchResults.value = [];
  hasSearched.value = false;
};

// 执行搜索请求
const doSearch = async () => {
  if (loading.value) return;
  
  loading.value = true;
  try {
    const sortOption = sortOptions[sortBy.value];
    
    const res = await resourceApi.searchResources(
      searchValue.value,
      page.value,
      pageSize,
      sortOption
    );
    
    const list = res.data.list || [];
    totalResults.value = res.data.total || 0;
    
    if (page.value === 1) {
      searchResults.value = list;
    } else {
      searchResults.value = [...searchResults.value, ...list];
    }
    
    // 判断是否加载完毕
    if (list.length < pageSize) {
      finished.value = true;
    } else {
      page.value += 1;
    }
  } catch (error) {
    console.error('搜索失败', error);
    showToast('搜索失败，请重试');
  } finally {
    loading.value = false;
  }
};

// 加载更多结果
const loadMore = () => {
  if (finished.value || loading.value) return;
  doSearch();
};

// 点击资源
const onResourceClick = (resource) => {
  router.push(`/resource/${resource.id}`);
};

// 排序方法
const sortByRelevance = () => {
  if (sortBy.value === 'relevance') return;
  sortBy.value = 'relevance';
  resetSearch();
};

const sortByLatest = () => {
  if (sortBy.value === 'latest') return;
  sortBy.value = 'latest';
  resetSearch();
};

const sortByDownload = () => {
  if (sortBy.value === 'download') return;
  sortBy.value = 'download';
  resetSearch();
};

// 重置搜索
const resetSearch = () => {
  searchResults.value = [];
  page.value = 1;
  finished.value = false;
  doSearch();
};

// 监听排序变化
watch(() => sortBy.value, () => {
  if (hasSearched.value) {
    resetSearch();
  }
});

onMounted(() => {
  loadSearchHistory();
  loadHotTags();
});
</script>

<style scoped>
.search-page {
  min-height: 100vh;
  background-color: var(--background-color);
  padding-bottom: 50px;
}

.search-header {
  position: sticky;
  top: 0;
  z-index: 100;
}

.search-content {
  padding: 16px;
}

.search-section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-color);
}

.section-action {
  font-size: 14px;
  color: var(--text-color-light);
}

.search-tags {
  display: flex;
  flex-wrap: wrap;
}

.search-tag {
  margin-right: 8px;
  margin-bottom: 8px;
  font-size: 13px;
  padding: 6px 12px;
}

.search-result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color);
}

.result-count {
  font-size: 14px;
  color: var(--text-color-light);
}

.highlight {
  color: var(--primary-color);
  font-weight: 500;
}

.result-sort {
  display: flex;
  font-size: 14px;
}

.sort-item {
  margin-left: 16px;
  color: var(--text-color-light);
  cursor: pointer;
}

.sort-item.active {
  color: var(--primary-color);
  font-weight: 500;
}

.load-more {
  margin-top: 16px;
}
</style> 