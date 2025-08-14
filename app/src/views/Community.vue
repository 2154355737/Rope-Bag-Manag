<template>
  <div class="community-page">
    <van-nav-bar title="社区" fixed placeholder />

    <div class="content page-with-fixed-navbar">
      <!-- 公告栏 -->
      <div class="announcement" v-if="latestAnnouncement" @click="viewAnnouncements">
        <van-icon name="volume-o" class="announcement-icon" />
        <div class="announcement-text" :title="latestAnnouncement.title">{{ latestAnnouncement.title }}</div>
        <van-tag type="danger" size="mini" plain v-if="latestAnnouncement.priority === 1">置顶</van-tag>
      </div>
      <div class="announcement" v-else @click="viewAnnouncements">
        <van-icon name="volume-o" class="announcement-icon" />
        <div class="announcement-text">暂无公告，点击查看历史公告</div>
      </div>

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

const latestAnnouncement = ref(null);

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
    // 获取已审核通过的精选帖子
    const res = await postApi.getPosts({ is_featured: true, status: 'approved', page_size: 5 });
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
    // 对于公共显示，应该查询已审核通过的帖子
    const tabParams = activeTab.value === 'popular' ? { status: 'approved' } : activeTab.value === 'featured' ? { is_featured: true, status: 'approved' } : { status: 'approved' };
    const tagParams = currentTag.value ? { tags: [currentTag.value] } : {};
    const res = await postApi.getPosts({ ...base, ...tabParams, ...tagParams });
    const list = res.data?.list || [];
    posts.value = posts.value.concat(list);
    if (list.length < 10) finished.value = true; else page.value += 1;
  } finally {
    loading.value = false;
  }
};

const loadAnnouncements = async () => {
  try {
    const resp = await postApi.getActiveAnnouncements();
    const list = resp?.data?.list || resp?.data || [];
    if (Array.isArray(list) && list.length) {
      latestAnnouncement.value = list.sort((a, b) => (b.priority || 0) - (a.priority || 0) || new Date(b.start_time || 0) - new Date(a.start_time || 0))[0];
    }
  } catch (e) {
    latestAnnouncement.value = null;
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
const viewAnnouncements = () => router.push('/announcements');

onMounted(() => {
  loadHotTags();
  loadFeatured();
  loadMore();
  loadAnnouncements();
});
</script>

<style scoped>
.content { 
  padding: 12px; 
  /* 顶部间距由全局样式统一处理，底部间距由全局 .page-content 统一处理 */
  /* 注释：全局.content样式已自动添加了NavBar高度+16px间距的padding-top */
}
.page-with-fixed-navbar { padding-top: 8px !important; }
.toolbar { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }
.tabs { display: flex; gap: 12px; }
.tab { font-size: 14px; color: var(--text-color-light); cursor: pointer; }
.tab.active { color: var(--primary-color); font-weight: 600; }
.tags { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 8px; }
.tag { margin-bottom: 6px; }
.clear { margin-left: auto; }
.panel { background: #fff; border-radius: 8px; }

.announcement { display: flex; align-items: center; gap: 8px; background: #fff7e6; border: 1px solid #ffd591; color: #ad6800; padding: 8px 10px; border-radius: 8px; margin-bottom: 10px; }
.announcement-icon { color: #fa8c16; }
.announcement-text { flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; font-weight: 600; }
</style> 