<template>
  <div class="community-page">
    <van-nav-bar title="社区" fixed />

    <div class="content">
      <div class="toolbar">
        <div class="tabs">
          <span :class="['tab', activeTab === 'latest' && 'active']" @click="switchTab('latest')">最新</span>
          <span :class="['tab', activeTab === 'featured' && 'active']" @click="switchTab('featured')">精选</span>
          <span :class="['tab', activeTab === 'popular' && 'active']" @click="switchTab('popular')">热门</span>
        </div>
        <van-button size="small" type="primary" round @click="goCreate">写帖子</van-button>
      </div>

      <div class="tags" v-if="hotTags.length">
        <van-tag
          v-for="tag in hotTags"
          :key="tag"
          plain
          round
          type="primary"
          class="tag"
          @click="filterByTag(tag)"
        >
          # {{ tag }}
        </van-tag>
        <van-button size="small" type="default" plain class="clear" @click="clearTag" v-if="currentTag">清除标签：{{ currentTag }}</van-button>
      </div>

      <van-skeleton :row="3" :loading="featuredLoading" v-if="activeTab==='featured'"/>
      <div v-if="activeTab==='featured' && !featuredLoading" class="panel">
        <post-card v-for="post in featured" :key="post.id" :post="post" @click="goDetail(post.id)" />
      </div>

      <van-list v-model:loading="loading" :finished="finished" finished-text="没有更多了" @load="loadMore">
        <post-card v-for="post in posts" :key="post.id" :post="post" @click="goDetail(post.id)" />
      </van-list>
    </div>

  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import dayjs from 'dayjs';
import { postApi } from '../api/post';
import { useRouter } from 'vue-router';
import { tagApi } from '../api/resource';
import PostCard from '../components/PostCard.vue';

const router = useRouter();

const posts = ref([]);
const page = ref(1);
const loading = ref(false);
const finished = ref(false);

const featured = ref([]);
const featuredLoading = ref(false);

const hotTags = ref([]);
const currentTag = ref('');
const activeTab = ref('latest');

const formatMeta = (post) => {
  const date = dayjs(post.created_at).format('YYYY-MM-DD HH:mm');
  return `作者：${post.author_name || post.author_id} · ${date} · 浏览 ${post.view_count || 0}`;
};

const loadHotTags = async () => {
  const res = await tagApi.getHotTags(12);
  const items = res.data || [];
  hotTags.value = items.map(t => (typeof t === 'string' ? t : t.name)).filter(Boolean);
};

const loadFeatured = async () => {
  featuredLoading.value = true;
  try {
    const res = await postApi.getFeaturedPosts(5);
    featured.value = res.data?.list || [];
  } finally {
    featuredLoading.value = false;
  }
};

const loadMore = async () => {
  if (loading.value) return;
  loading.value = true;
  try {
    const base = { page: page.value, page_size: 10 };
    const tabParams = activeTab.value === 'popular' ? {} : activeTab.value === 'featured' ? { is_featured: true } : { status: 'Published' };
    const tagParams = currentTag.value ? { tags: [currentTag.value] } : {};
    const res = await postApi.getPosts({ ...base, ...tabParams, ...tagParams });
    const list = res.data?.list || [];
    posts.value = posts.value.concat(list);
    if (list.length < 10) finished.value = true; else page.value += 1;
  } finally {
    loading.value = false;
  }
};

const switchTab = (tab) => {
  if (activeTab.value === tab) return;
  activeTab.value = tab;
  posts.value = [];
  page.value = 1;
  finished.value = false;
  loadMore();
};

const filterByTag = (tag) => {
  currentTag.value = tag;
  posts.value = [];
  page.value = 1;
  finished.value = false;
  loadMore();
};

const clearTag = () => {
  currentTag.value = '';
  posts.value = [];
  page.value = 1;
  finished.value = false;
  loadMore();
};

const goDetail = (id) => router.push(`/post/${id}`);
const goCreate = () => router.push('/post/create');

onMounted(() => {
  loadHotTags();
  loadFeatured();
  loadMore();
});
</script>

<style scoped>
.content { 
  padding: 12px; 
  /* 顶部间距由全局样式统一处理，底部间距由全局 .page-content 统一处理 */
  /* 注释：全局.content样式已自动添加了NavBar高度+16px间距的padding-top */
}
.toolbar { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }
.tabs { display: flex; gap: 12px; }
.tab { font-size: 14px; color: var(--text-color-light); cursor: pointer; }
.tab.active { color: var(--primary-color); font-weight: 600; }
.tags { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 8px; }
.tag { margin-bottom: 6px; }
.clear { margin-left: auto; }
.panel { background: #fff; border-radius: 8px; }
</style> 