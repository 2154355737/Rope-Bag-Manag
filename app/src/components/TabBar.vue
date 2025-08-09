<template>
  <van-tabbar v-model="active" route>
    <van-tabbar-item to="/" icon="home-o">首页</van-tabbar-item>
    <van-tabbar-item to="/category" icon="apps-o">分类</van-tabbar-item>
    <van-tabbar-item to="/community" icon="friends-o">社区</van-tabbar-item>
    <van-tabbar-item to="/search" icon="search">搜索</van-tabbar-item>
    <van-tabbar-item to="/profile" icon="user-o" :badge="badgeText">我的</van-tabbar-item>
  </van-tabbar>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { notificationApi } from '@/api/notification';

const route = useRoute();
const active = ref(0);
const badgeText = ref('');

// 根据当前路由路径设置激活的标签页
watch(
  () => route.path,
  (path) => {
    if (path === '/' || path.startsWith('/home')) {
      active.value = 0;
    } else if (path === '/category' || path.startsWith('/category/')) {
      active.value = 1;
    } else if (path === '/community' || path.startsWith('/community')) {
      active.value = 2;
    } else if (path === '/search' || path.startsWith('/search')) {
      active.value = 3;
    } else if (path === '/profile' || path.startsWith('/profile') || 
              path === '/login' || path === '/register' ||
              path === '/my-resources' || path === '/my-comments' || 
              path === '/my-posts' || path === '/favorites' || path === '/notifications' || path.startsWith('/post/')) {
      active.value = 4;
    }
  },
  { immediate: true }
);

const loadUnread = async () => {
  const token = localStorage.getItem('token');
  if (!token) { badgeText.value = ''; return; }
  try {
    const res = await notificationApi.unreadCount();
    const n = res?.data?.count ?? 0;
    badgeText.value = n > 0 ? (n > 99 ? '99+' : String(n)) : '';
  } catch (e) {
    badgeText.value = '';
  }
};

onMounted(() => {
  loadUnread();
});
</script>

<style scoped>
.van-tabbar {
  border-top: 1px solid var(--border-color);
}

.van-tabbar-item--active {
  color: var(--primary-color);
}
</style> 