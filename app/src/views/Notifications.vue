<template>
  <div class="notifications-page">
    <van-nav-bar title="我的通知" left-arrow @click-left="onBack" fixed placeholder>
		  <template #right>
			<van-button size="small" type="primary" plain :loading="markingAll" @click="markAll">全部已读</van-button>
		  </template>
		</van-nav-bar>
    <div class="content page-with-fixed-navbar">
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
import { showToast } from 'vant';

const router = useRouter();
const list = ref([]);
const page = ref(1);
const loading = ref(false);
const finished = ref(false);
const loadError = ref(false);
const markingAll = ref(false);

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

const markAll = async () => {
  if (markingAll.value) return;
  markingAll.value = true;
  try {
    const res = await notificationApi.markAllRead();
    if (res && res.code === 0) {
      // 本地标记
      list.value = list.value.map(n => ({ ...n, is_read: true }));
      showToast('已全部标记为已读');
    } else {
      showToast(res?.message || '操作失败');
    }
  } catch (e) {
    showToast('操作失败');
  } finally {
    markingAll.value = false;
  }
};

const open = async (n) => {
  if (!n.is_read) {
    try { await notificationApi.markRead(n.id); n.is_read = true; } catch (e) {}
  }
  if (n.link) {
    let targetLink = n.link;

    // 如果是完整 URL，尝试识别内部路由并改用前端路由跳转
    try {
      if (/^https?:\/\//i.test(targetLink)) {
        const u = new URL(targetLink);
        const internalPaths = [
          /^\/resource\//, /^\/post\//, /^\/category\//, /^\/search$/, /^\/community$/,
          /^\/profile(\/.*)?$/, /^\/notifications$/, /^\/my-resources$/, /^\/my-posts$/, /^\/my-comments$/, /^\/favorites$/
        ];
        const isInternal = internalPaths.some(rx => rx.test(u.pathname));
        if (isInternal) {
          router.push(u.pathname + u.search + u.hash);
          return;
        }
      }
    } catch (e) {
      // ignore URL parse error
    }

    // 非完整URL或外部链接保留默认处理
    if (targetLink.startsWith('http://') || targetLink.startsWith('https://')) {
      window.location.href = targetLink;
    } else {
      if (!targetLink.startsWith('/')) targetLink = '/' + targetLink;
      router.push(targetLink);
    }
  }
};

onMounted(() => {
  // 主动触发首轮加载，避免某些环境下 van-list 未主动触发
  loadMore();
});
</script>

<style scoped>
.notifications-page { 
  min-height: 100vh; 
  background: var(--background-color); 
  /* 确保页面占满整个视口 */
  display: flex;
  flex-direction: column;
}

.content { 
  /* 基础内边距 */
  padding: 12px; 
  /* 顶部：NavBar高度 + 间距 + 内容边距，与全局样式保持一致 */
  padding-top: calc(46px + env(safe-area-inset-top, 0px) + 16px + 12px);
  /* 底部：底部导航高度 + 内容边距 */
  padding-bottom: calc(66px + env(safe-area-inset-bottom, 0px) + 12px);
  /* 布局 */
  flex: 1;
  overflow-y: auto;
}
.page-with-fixed-navbar { padding-top: 8px !important; }

.notif-item { 
  background: #fff; 
  border-radius: 12px; 
  padding: 12px; 
  margin-bottom: 12px;
  /* 添加轻微的阴影效果 */
  box-shadow: var(--shadow-normal);
  /* 添加过渡动画 */
  transition: transform var(--animation-fast) var(--ease-out-cubic);
}

.notif-item:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-large);
}

.notif-item:active {
  transform: translateY(0);
}

.row { 
  display: flex; 
  align-items: center; 
  justify-content: space-between; 
}

.title { 
  font-weight: 600; 
  color: var(--text-color);
  flex: 1;
  /* 防止标题过长时破坏布局 */
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-right: 8px;
}

.time { 
  color: var(--text-color-light); 
  font-size: 12px;
  /* 确保时间不被压缩 */
  flex-shrink: 0;
}

.content-text { 
  margin-top: 6px; 
  color: var(--text-color);
  /* 内容文本多行显示 */
  line-height: 1.4;
  word-break: break-word;
}

.mr6 { 
  margin-right: 6px; 
}

/* 加载状态优化 */
.van-list {
  min-height: 200px;
}

/* 空状态优化 */
.van-empty {
  padding: 40px 20px;
}

/* 响应式优化 */
@media (max-width: 375px) {
  .content {
    padding-left: 8px;
    padding-right: 8px;
  }
  
  .notif-item {
    padding: 10px;
    margin-bottom: 8px;
  }
}

/* 横屏适配 - 与主CSS文件中的NavBar横屏高度保持一致 */
@media (orientation: landscape) and (max-height: 500px) {
  .content {
    padding-top: calc(40px + env(safe-area-inset-top, 0px) + 16px + 12px);
  }
}
</style> 