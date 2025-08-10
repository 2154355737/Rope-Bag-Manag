<template>
  <div class="user-posts-page">
    <van-nav-bar title="我的帖子" left-arrow @click-left="onBack" fixed />
    <div class="content" :style="{ paddingTop: '46px' }">
      <van-list
        v-model:loading="loading"
        v-model:error="error"
        :finished="finished"
        finished-text="没有更多了"
        error-text="加载失败，点击重试"
        :immediate-check="false"
        @load="loadMore"
      >
        <div v-for="post in posts" :key="post.id" class="post-wrap">
          <post-card :post="post" @click="goDetail(post.id)" />
          <div class="row-actions">
            <van-button size="small" plain type="primary" @click="onEdit(post)">编辑</van-button>
            <van-button size="small" plain type="danger" @click="onDelete(post)">删除</van-button>
          </div>
        </div>
        <van-empty v-if="!loading && posts.length === 0 && finished" description="暂无帖子" />
      </van-list>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import dayjs from 'dayjs';
import { useRouter } from 'vue-router';
import { useUserStore } from '../store/user';
import { postApi } from '../api/post';
import { showConfirmDialog, showToast } from 'vant';
import PostCard from '../components/PostCard.vue';

const router = useRouter();
const userStore = useUserStore();

const posts = ref([]);
const loading = ref(false);
const finished = ref(false);
const error = ref(false);
const page = ref(1);

const onBack = () => router.back();
const formatMeta = (p) => `${dayjs(p.created_at).format('YYYY-MM-DD HH:mm')} · 浏览 ${p.view_count || 0}`;

const resetAndReload = () => { posts.value = []; page.value = 1; finished.value = false; error.value = false; loadMore(); };

const loadMore = async () => {
  if (loading.value || finished.value) return;
  loading.value = true;
  error.value = false;
  try {
    const base = { page: page.value, page_size: 10, author_id: userStore.userId };
    const results = await Promise.allSettled([
      postApi.getPosts({ ...base, status: 'Draft' }),
      postApi.getPosts({ ...base, status: 'Published' })
    ]);
    const draftRes = results[0].status === 'fulfilled' ? results[0].value : null;
    const pubRes = results[1].status === 'fulfilled' ? results[1].value : null;
    if (!draftRes && !pubRes) throw new Error('all requests failed');
    const draftList = draftRes?.data?.list || [];
    const pubList = pubRes?.data?.list || [];
    const merged = [...draftList, ...pubList];
    const uniqueMap = new Map();
    for (const p of merged) { if (p && !uniqueMap.has(p.id)) uniqueMap.set(p.id, p); }
    const list = Array.from(uniqueMap.values()).sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
    posts.value = posts.value.concat(list);
    if (list.length < 10) finished.value = true; else page.value += 1;
  } catch (e) {
    error.value = true;
    showToast('加载失败');
  } finally {
    loading.value = false;
  }
};

const goDetail = (id) => router.push(`/post/${id}`);
const onEdit = (post) => router.push({ path: '/post/create', query: { id: post.id } });
const onDelete = async (post) => {
  try {
    await showConfirmDialog({ title: '确认删除', message: `确定删除《${post.title}》吗？` });
  } catch { return; }
  try {
    await postApi.deletePost(post.id);
    showToast('已删除');
    resetAndReload();
  } catch (e) {
    showToast('删除失败');
  }
};

// 监听用户切换
watch(() => userStore.userId, (n, o) => {
  posts.value = [];
  page.value = 1;
  finished.value = false;
  error.value = false;
  if (!userStore.isLoggedIn) {
    router.replace({ path: '/login', query: { redirect: '/my-posts' } });
    return;
  }
  loadMore();
});

onMounted(async () => {
  if (!userStore.isLoggedIn) {
    router.replace({ path: '/login', query: { redirect: '/my-posts' } });
    return;
  }
  try { await userStore.checkAuth(); } catch {}
  if (!userStore.userId) { showToast('用户信息缺失，请重新登录'); return; }
  // 手动触发首次加载
  loadMore();
});
</script>

<style scoped>
.content { padding: 12px; }
.post-wrap { margin-bottom: 10px; }
.row-actions { display: flex; gap: 8px; padding: 0 8px 8px 8px; }
</style> 