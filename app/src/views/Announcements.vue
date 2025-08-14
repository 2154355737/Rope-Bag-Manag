<template>
  <div class="announcements-page">
    <van-nav-bar title="公告" left-arrow @click-left="onBack" fixed placeholder />

    <div class="content announcements-content">
      <van-skeleton title :row="6" :loading="loading" v-if="loading" />

      <template v-else>
        <div v-if="announcements.length > 0">
          <div class="announcement-card" v-for="a in announcements" :key="a.id">
            <div class="card-header">
              <van-icon name="volume-o" class="icon" />
              <div class="title ellipsis" :title="a.title">{{ a.title }}</div>
              <van-tag v-if="a.priority === 1" type="danger" size="mini" plain>置顶</van-tag>
            </div>
            <div class="meta">
              <span>开始：{{ formatDate(a.start_time) }}</span>
              <span v-if="a.end_time">结束：{{ formatDate(a.end_time) }}</span>
            </div>
            <div class="content-text">{{ a.content }}</div>
          </div>
        </div>
        <div v-else class="empty">
          <van-empty image="search" description="暂无公告" />
        </div>
      </template>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { postApi } from '../api/post';
import dayjs from 'dayjs';
import { useRouter } from 'vue-router';

const router = useRouter();
const onBack = () => router.back();

const announcements = ref([]);
const loading = ref(false);

const formatDate = (d) => {
  if (!d) return '';
  return dayjs(d).format('YYYY-MM-DD HH:mm');
};

const load = async () => {
  loading.value = true;
  try {
    const resp = await postApi.getActiveAnnouncements();
    const list = resp?.data?.list || resp?.data || [];
    announcements.value = Array.isArray(list)
      ? list.sort((a, b) => (b.priority || 0) - (a.priority || 0) || new Date(b.start_time || 0) - new Date(a.start_time || 0))
      : [];
  } finally {
    loading.value = false;
  }
};

onMounted(load);
</script>

<style scoped>
.content { padding: 12px; }
.announcements-content { padding-top: 80px !important; }
.announcement-card { background: #fff; border-radius: 12px; padding: 12px; margin-bottom: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.06); }
.card-header { display: flex; align-items: center; gap: 8px; margin-bottom: 6px; }
.icon { color: #fa8c16; }
.title { font-size: 16px; font-weight: 600; color: var(--text-color); flex: 1; }
.meta { display: flex; gap: 12px; color: var(--text-color-lighter); font-size: 12px; margin-bottom: 8px; }
.content-text { color: var(--text-color); line-height: 1.7; white-space: pre-wrap; }
.empty { margin-top: 20vh; }
</style> 