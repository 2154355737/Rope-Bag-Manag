<template>
  <van-tabbar route :safe-area-inset-bottom="true" :z-index="1000" v-model:active="active">
    <van-tabbar-item to="/" icon="home-o">首页</van-tabbar-item>
    <van-tabbar-item to="/category" icon="apps-o">分类</van-tabbar-item>
    <van-tabbar-item to="/community" icon="friends-o">社区</van-tabbar-item>
    <van-tabbar-item to="/search" icon="search">搜索</van-tabbar-item>
    <van-tabbar-item to="/profile" icon="user-o">我的</van-tabbar-item>
  </van-tabbar>
</template>

<script setup>
import { ref, watch } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const active = ref(0);

const syncActiveByPath = (path) => {
  if (path === '/' || path.startsWith('/home')) return 0;
  if (path === '/category' || path.startsWith('/category/')) return 1;
  if (path === '/community' || path.startsWith('/community')) return 2;
  if (path === '/search' || path.startsWith('/search')) return 3;
  if (
    path === '/profile' || path.startsWith('/profile') ||
    path === '/login' || path === '/register' ||
    path === '/my-resources' || path === '/my-comments' ||
    path === '/my-posts' || path === '/favorites' ||
    path === '/notifications' || path.startsWith('/post/')
  ) return 4;
  return active.value;
};

watch(
  () => route.path,
  (path) => {
    if (path === '/splash') return;
    active.value = syncActiveByPath(path);
  },
  { immediate: true }
);


</script>

<style>
/* 固定于底部，采用简洁稳定的布局，避免内部元素偏移 */
.van-tabbar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  width: 100%;
  z-index: 1000;
  background: var(--navbar-bg);
  border-top: 1px solid var(--navbar-border);
  box-shadow: var(--navbar-shadow);
  -webkit-backdrop-filter: blur(10px);
  backdrop-filter: blur(10px);
  padding-bottom: env(safe-area-inset-bottom);
  min-height: 56px;
}

.van-tabbar-item {
  flex: 1;
  min-height: 56px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.van-tabbar-item .van-icon {
  font-size: 22px;
  line-height: 1;
}

.van-tabbar-item__text {
  font-size: 11px;
  margin-top: 2px;
  line-height: 1.2;
  opacity: 1;
  visibility: visible;
  transform: none;
}

.van-tabbar-item--active {
  color: var(--primary-color);
}

/* 优化的徽标样式 */
.van-badge__wrapper .van-badge {
  background: linear-gradient(135deg, #ff4757, #ff3742);
  border: 2px solid #fff;
  box-shadow: 0 3px 12px rgba(255, 71, 87, 0.4), 0 1px 4px rgba(0, 0, 0, 0.1);
  font-size: 10px;
  font-weight: 700;
  min-width: 18px;
  height: 18px;
  border-radius: 10px;
  transform: scale(1);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  animation: subtle-bounce 0.6s ease-out;
}

/* 徽章脉冲动画 */
@keyframes badge-pulse {
  0% {
    transform: scale(1);
    box-shadow: 0 3px 12px rgba(255, 71, 87, 0.4), 0 1px 4px rgba(0, 0, 0, 0.1);
  }
  50% {
    transform: scale(1.1);
    box-shadow: 0 4px 16px rgba(255, 71, 87, 0.6), 0 2px 8px rgba(0, 0, 0, 0.15);
  }
  100% {
    transform: scale(1);
    box-shadow: 0 3px 12px rgba(255, 71, 87, 0.4), 0 1px 4px rgba(0, 0, 0, 0.1);
  }
}

/* 徽章出现动画 */
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

/* TabBar项目脉冲动画 */
.van-tabbar-item.badge-pulse {
  animation: badge-pulse 0.6s ease-out;
}

/* 徽章悬停效果（仅在支持hover的设备上） */
@media (hover: hover) {
  .van-badge__wrapper .van-badge:hover {
    transform: scale(1.05);
    box-shadow: 0 4px 16px rgba(255, 71, 87, 0.5), 0 2px 8px rgba(0, 0, 0, 0.15);
  }
}

/* 响应式微调 */
@media (min-width: 768px) {
  .van-tabbar { min-height: 60px; }
  .van-tabbar-item { min-height: 60px; }
  .van-tabbar-item .van-icon { font-size: 24px; }
  .van-tabbar-item__text { font-size: 12px; }
}

@media (orientation: landscape) and (max-height: 500px) {
  .van-tabbar { min-height: 46px; }
  .van-tabbar-item { min-height: 46px; }
  .van-tabbar-item .van-icon { font-size: 18px; }
  .van-tabbar-item__text { font-size: 10px; }
}

@media (prefers-color-scheme: dark) {
  .van-tabbar {
    background: rgba(30, 30, 30, 0.95);
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }
  .van-tabbar-item--active { color: var(--primary-color); }
}
</style> 