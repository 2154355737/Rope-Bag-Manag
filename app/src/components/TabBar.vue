<template>
  <van-tabbar route :safe-area-inset-bottom="true" :z-index="1000" class="custom-tabbar" v-model:active="active">
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
    // 添加Android设备标记类
    document.documentElement.classList.add('android-device');
    
    // 尝试获取实际的导航栏高度
    let navBarHeight = 0;
    
    // 检测是否支持safe-area-inset
    const supportsSafeArea = CSS.supports('padding-bottom', 'env(safe-area-inset-bottom)');
    
    if (supportsSafeArea) {
      // 测量safe-area-inset-bottom
      const testEl = document.createElement('div');
      testEl.style.cssText = 'position:fixed;bottom:0;padding-bottom:env(safe-area-inset-bottom,0px);visibility:hidden;';
      document.body.appendChild(testEl);
      
      const computedStyle = window.getComputedStyle(testEl);
      const safeAreaBottom = parseInt(computedStyle.paddingBottom) || 0;
      
      document.body.removeChild(testEl);
      
      if (safeAreaBottom > 0) {
        // 有安全区域值，使用该值
        navBarHeight = safeAreaBottom;
      } else {
        // 安全区域为0，使用默认值
        navBarHeight = 30; // 默认安卓导航栏高度
      }
    } else {
      // 不支持safe-area，使用默认值
      navBarHeight = 30;
    }
    
    // 设置CSS变量
    document.documentElement.style.setProperty('--android-navbar-height', `${navBarHeight}px`);
    
    // 添加安卓导航栏适配类
    document.documentElement.classList.add('has-android-navbar');
    
    // 动态添加样式
    const styleId = 'android-nav-fix';
    if (!document.getElementById(styleId)) {
      const style = document.createElement('style');
      style.id = styleId;
      style.textContent = `
        /* 安卓导航栏适配样式 */
        .has-android-navbar .van-tabbar,
        .has-android-navbar .custom-tabbar {
          padding-bottom: calc(var(--android-navbar-height, 30px)) !important;
          min-height: calc(56px + var(--android-navbar-height, 30px)) !important;
        }
        
        .has-android-navbar .page-content {
          padding-bottom: calc(66px + var(--android-navbar-height, 30px)) !important;
        }
        
        .has-android-navbar .fixed-bottom {
          padding-bottom: calc(var(--android-navbar-height, 30px)) !important;
        }
      `;
      document.head.appendChild(style);
    }
  }
};

// 窗口尺寸变化检测（用户可能切换系统导航栏模式）
const handleResize = () => {
  // 延迟执行，等待UI调整完成
  setTimeout(detectAndroidNavBar, 300);
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
.van-tabbar,
.custom-tabbar {
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
  /* 使用更大的默认高度确保不被遮挡 */
  min-height: calc(56px + env(safe-area-inset-bottom, 0px));
  min-height: calc(56px + constant(safe-area-inset-bottom, 0px)); /* iOS 11.0 */
}

/* 针对已确认有导航栏的安卓设备特殊处理 */
.has-android-navbar .van-tabbar,
.has-android-navbar .custom-tabbar {
  padding-bottom: var(--android-navbar-height, 30px) !important;
  min-height: calc(56px + var(--android-navbar-height, 30px)) !important;
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
  /* 只有确认有导航栏的设备才添加额外间距 */
  .has-android-navbar .van-tabbar,
  .has-android-navbar .custom-tabbar {
    padding-bottom: 30px;
    min-height: 86px;
  }
}

/* 检测Android设备的媒体查询 */
@media screen and (min-resolution: 0.001dpcm) {
  @supports (-webkit-appearance: none) {
    /* 只有确认有导航栏的设备才添加额外间距 */
    .has-android-navbar .van-tabbar,
    .has-android-navbar .custom-tabbar {
      padding-bottom: 30px !important;
      min-height: calc(56px + 30px) !important;
    }
  }
}

/* 响应式微调 */
@media (min-width: 768px) {
  .van-tabbar, 
  .custom-tabbar { 
    min-height: calc(60px + env(safe-area-inset-bottom, 30px));
    min-height: calc(60px + constant(safe-area-inset-bottom, 30px)); /* iOS 11.0 */
  }
  .van-tabbar-item { min-height: 60px; }
  .van-tabbar-item .van-icon { font-size: 24px; }
  .van-tabbar-item__text { font-size: 12px; }
}

@media (orientation: landscape) and (max-height: 500px) {
  .van-tabbar,
  .custom-tabbar { 
    min-height: calc(46px + env(safe-area-inset-bottom, 30px));
    min-height: calc(46px + constant(safe-area-inset-bottom, 30px)); /* iOS 11.0 */
  }
  .van-tabbar-item { min-height: 46px; }
  .van-tabbar-item .van-icon { font-size: 18px; }
  .van-tabbar-item__text { font-size: 10px; }
}

@media (prefers-color-scheme: dark) {
  .van-tabbar,
  .custom-tabbar {
    background: rgba(30, 30, 30, 0.95);
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }
  .van-tabbar-item--active { color: var(--primary-color); }
}
</style> 