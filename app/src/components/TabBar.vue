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
import { ref, watch, onMounted, onUnmounted } from 'vue';
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

// Android系统导航栏检测和动态调整
const detectAndroidNavBar = () => {
  const isAndroid = /Android/i.test(navigator.userAgent);
  const isMobile = /Mobile|Android/i.test(navigator.userAgent);
  
  if (isAndroid && isMobile) {
    // 检测是否支持safe-area-inset
    const supportsSafeArea = CSS.supports('padding-bottom', 'env(safe-area-inset-bottom)');
    
    // 检测当前safe-area-inset-bottom的值
    const testEl = document.createElement('div');
    testEl.style.cssText = 'position:fixed;bottom:0;padding-bottom:env(safe-area-inset-bottom,0px);visibility:hidden;';
    document.body.appendChild(testEl);
    
    const computedStyle = window.getComputedStyle(testEl);
    const safeAreaBottom = parseInt(computedStyle.paddingBottom) || 0;
    
    document.body.removeChild(testEl);
    
    // 如果safe-area-inset-bottom为0或很小，说明可能有系统导航栏但未被检测到
    if (safeAreaBottom < 20) {
      // 添加Android系统导航栏适配类
      document.documentElement.classList.add('android-nav-bar');
      
      // 动态添加样式来处理系统导航栏
      const styleId = 'android-nav-fix';
      if (!document.getElementById(styleId)) {
        const style = document.createElement('style');
        style.id = styleId;
        style.textContent = `
          .android-nav-bar .van-tabbar {
            padding-bottom: 20px !important;
            min-height: 76px !important;
          }
          .android-nav-bar .page-content {
            padding-bottom: 86px !important;
          }
        `;
        document.head.appendChild(style);
      }
    }
  }
};

// 窗口尺寸变化检测（用户可能切换系统导航栏模式）
const handleResize = () => {
  // 延迟执行，等待UI调整完成
  setTimeout(detectAndroidNavBar, 100);
};

onMounted(() => {
  detectAndroidNavBar();
  window.addEventListener('resize', handleResize);
  window.addEventListener('orientationchange', handleResize);
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  window.removeEventListener('orientationchange', handleResize);
});

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
  /* 增强的底部安全区域处理，同时兼容Android和iOS */
  padding-bottom: env(safe-area-inset-bottom, 0px);
  padding-bottom: constant(safe-area-inset-bottom, 0px); /* iOS 11.0 */
  /* 对于Android系统导航栏，如果safe-area-inset-bottom为0，则使用默认高度 */
  min-height: calc(56px + env(safe-area-inset-bottom, 0px));
  min-height: calc(56px + constant(safe-area-inset-bottom, 0px)); /* iOS 11.0 */
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

/* Android系统导航栏特殊处理 - 当safe-area-inset-bottom为0时的fallback */
@supports not (padding-bottom: env(safe-area-inset-bottom)) {
  .van-tabbar {
    /* 对于不支持safe-area的Android设备，添加额外的底部间距 */
    padding-bottom: 16px;
    min-height: 72px;
  }
}

/* 检测Android设备的媒体查询 */
@media screen and (min-resolution: 0.001dpcm) {
  @supports (-webkit-appearance: none) {
    .van-tabbar {
      /* Android Chrome特殊处理 */
      padding-bottom: max(env(safe-area-inset-bottom, 0px), 8px);
      min-height: calc(56px + max(env(safe-area-inset-bottom, 0px), 8px));
    }
  }
}

/* 响应式微调 */
@media (min-width: 768px) {
  .van-tabbar { 
    min-height: calc(60px + env(safe-area-inset-bottom, 0px));
    min-height: calc(60px + constant(safe-area-inset-bottom, 0px)); /* iOS 11.0 */
  }
  .van-tabbar-item { min-height: 60px; }
  .van-tabbar-item .van-icon { font-size: 24px; }
  .van-tabbar-item__text { font-size: 12px; }
}

@media (orientation: landscape) and (max-height: 500px) {
  .van-tabbar { 
    min-height: calc(46px + env(safe-area-inset-bottom, 0px));
    min-height: calc(46px + constant(safe-area-inset-bottom, 0px)); /* iOS 11.0 */
  }
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