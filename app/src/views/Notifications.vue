<template>
  <div class="notifications-page">
    <van-nav-bar title="我的通知" left-arrow @click-left="onBack" fixed />
    <div class="content" :style="{ paddingTop: '46px' }">
      <van-list
        v-model:loading="loading"
        v-model:error="loadError"
        error-text="加载失败，点击重试"
        :finished="finished"
        finished-text="没有更多了"
        @load="loadMore"
      >
        <div v-for="n in list" :key="n.id" class="notif-item" @click="open(n)">
          <div class="row">
            <div class="title">
              <van-tag size="small" type="primary" v-if="!n.is_read" class="mr6">未读</van-tag>
              {{ n.title }}
            </div>
            <div class="time">{{ formatDate(n.created_at) }}</div>
          </div>
          <div class="content-text">{{ n.content }}</div>
        </div>
        <van-empty v-if="!loading && list.length === 0" description="暂无通知" />
      </van-list>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import dayjs from 'dayjs';
import { notificationApi } from '@/api/notification';

const router = useRouter();
const list = ref([]);
const page = ref(1);
const loading = ref(false);
const finished = ref(false);
const loadError = ref(false);

const formatDate = (s) => s ? dayjs(s).format('YYYY-MM-DD HH:mm') : '';
const onBack = () => router.back();

const loadMore = async () => {
  if (loading.value) return;
  loading.value = true;
  loadError.value = false;
  try {
    const res = await notificationApi.list(page.value, 10);
    const arr = res?.data?.list || [];
    list.value = list.value.concat(arr);
    if (arr.length < 10) finished.value = true; else page.value += 1;
  } catch (e) {
    loadError.value = true;
  } finally {
    loading.value = false;
  }
};

const open = async (n) => {
  if (!n.is_read) {
    try { await notificationApi.markRead(n.id); n.is_read = true; } catch (e) {}
  }
  if (n.link) {
    // 站内链接以 http 开头直接跳转，否则作为路由处理
    if (n.link.startsWith('http')) {
      window.location.href = n.link;
    } else {
      router.push(n.link);
    }
  }
};

onMounted(() => {
  // 主动触发首轮加载，避免某些环境下 van-list 未主动触发
  loadMore();
});
</script>

<style scoped>
.notifications-page { min-height: 100vh; background: var(--background-color); }
.content { padding: 12px; }
.notif-item { background: #fff; border-radius: 12px; padding: 12px; margin-bottom: 12px; }
.row { display: flex; align-items: center; justify-content: space-between; }
.title { font-weight: 600; color: var(--text-color); }
.time { color: var(--text-color-light); font-size: 12px; }
.content-text { margin-top: 6px; color: var(--text-color); }
.mr6 { margin-right: 6px; }
</style> 