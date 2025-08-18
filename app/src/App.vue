<template>
  <div id="app">
    <!-- 页面预加载层 -->
    <div v-if="isPreloading" class="preload-overlay">
      <div class="preload-content">
        <!-- 隐藏的预加载组件 -->
        <component 
          v-if="preloadComponent" 
          :is="preloadComponent" 
          class="preload-component"
        />
      </div>
    </div>
    
    <!-- 主要内容区域 -->
    <div class="page-content has-fixed-bottom">
      <router-view v-slot="{ Component, route }">
        <transition 
          :name="transitionName" 
          mode="out-in"
          @before-enter="onBeforeEnter"
          @enter="onEnter"
          @leave="onLeave"
        >
          <template v-if="route.meta && route.meta.keepAlive">
            <keep-alive>
              <component :is="Component" :key="route.fullPath" />
            </keep-alive>
          </template>
          <template v-else>
            <component :is="Component" :key="route.fullPath" />
          </template>
        </transition>
      </router-view>
    </div>
    <TabBar v-if="shouldShowTabBar" />
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, watch, nextTick, markRaw, shallowRef } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useUserStore } from './store/user';
import { initKeyboardAdapter, destroyKeyboardAdapter } from './utils/keyboard';
import { initSafeArea, cleanupSafeArea } from './utils/safeAreaHelper';
import preloader from './utils/preloader';
import TabBar from './components/TabBar.vue';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

// 预加载状态
const isPreloading = ref(false);
const preloadComponent = shallowRef(null);
const transitionName = ref('fade');
const shouldShowTabBar = computed(() => route.path !== '/splash');

// 页面过渡事件处理
const onBeforeEnter = (el) => {
  // 避免影响固定定位的元素（如TabBar）
  if (el.classList.contains('van-tabbar')) return;
  
  el.style.opacity = '0';
  el.style.transform = 'translateY(10px)';
};

const onEnter = (el, done) => {
  // 避免影响固定定位的元素（如TabBar）
  if (el.classList.contains('van-tabbar')) {
    done();
    return;
  }
  
  el.offsetHeight; // 强制重排
  el.style.transition = 'all 0.3s ease-out';
  el.style.opacity = '1';
  el.style.transform = 'translateY(0)';
  
  // 在动画完成后清除 transform/transition，避免创建包含块影响 fixed 子元素
  setTimeout(() => {
    el.style.transition = '';
    el.style.transform = '';
    done();
  }, 300);
};

const onLeave = (el, done) => {
  // 避免影响固定定位的元素（如TabBar）
  if (el.classList.contains('van-tabbar')) {
    done();
    return;
  }
  
  el.style.transition = 'all 0.2s ease-in';
  el.style.opacity = '0';
  el.style.transform = 'translateY(-5px)';
  
  setTimeout(() => {
    // 清理离场元素的内联样式，防止残留
    el.style.transition = '';
    el.style.transform = '';
    done();
  }, 200);
};

// 预加载目标页面组件和数据
const preloadTargetPage = async (targetPath) => {
  try {
    isPreloading.value = true;
    console.log('🚀 开始预加载页面:', targetPath);
    
    // 使用新的预加载服务
    const result = await preloader.preloadPage(targetPath, router, userStore);
    
    if (result) {
      console.log('✅ 预加载完成:', result);
    }
    
    // 额外的组件预渲染（可选）
    const route = router.resolve(targetPath);
    if (route.matched.length > 0) {
      const routeRecord = route.matched[route.matched.length - 1];
      if (routeRecord.components?.default) {
        if (typeof routeRecord.components.default === 'function') {
          const component = await routeRecord.components.default();
          // 使用 markRaw 防止组件变成响应式对象
          preloadComponent.value = markRaw(component.default || component);
          await nextTick();
        }
      }
    }
    
  } catch (error) {
    console.warn('预加载失败:', error);
  } finally {
    // 延迟清理，确保过渡顺畅
    setTimeout(() => {
      isPreloading.value = false;
      preloadComponent.value = null;
    }, 200);
  }
};

// 暴露预加载方法给启动页使用
window.preloadTargetPage = preloadTargetPage;

onMounted(async () => {
  // 检查是否需要显示启动页
  const shouldShowSplash = !sessionStorage.getItem('splashShown');
  
  if (shouldShowSplash) {
    // 记录当前路径，启动页完成后跳转回来
    const currentPath = router.currentRoute.value.fullPath;
    if (currentPath !== '/splash') {
      sessionStorage.setItem('redirectPath', currentPath);
      sessionStorage.setItem('splashShown', 'true');
      router.replace('/splash');
      return;
    }
  }
  
  // 尝试自动登录（从本地存储获取token）
  await userStore.checkAuth();
  
  // 初始化键盘适配器
  initKeyboardAdapter();
  
  // 初始化安全区域辅助工具
  initSafeArea();
  
  // 监听安全区域更新事件
  window.addEventListener('safe-area-updated', handleSafeAreaUpdated);
});

// 处理安全区域更新
const handleSafeAreaUpdated = (event) => {
  if (event.detail) {
    console.log('安全区域已更新:', event.detail);
    // 可以在这里执行其他与安全区域相关的逻辑
  }
};

onUnmounted(() => {
  // 清理键盘适配器
  destroyKeyboardAdapter();
  
  // 清理安全区域辅助工具
  cleanupSafeArea();
  
  // 移除安全区域更新事件监听
  window.removeEventListener('safe-area-updated', handleSafeAreaUpdated);
});
</script>

<style>
#app {
  min-height: 100vh;
  background-color: #f7f8fa;
  position: relative;
  /* 确保#app不创建滚动上下文 */
  overflow: visible;
}

/* 确保HTML和body元素不会干扰fixed定位 */
html, body {
  /* 重置可能影响定位的属性 */
  transform: none !important;
  /* 确保不会创建新的堆叠上下文 */
  position: static !important;
  /* 移除任何可能的padding/margin */
  margin: 0 !important;
  padding: 0 !important;
}

/* 预加载层样式 */
.preload-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: -1;
  pointer-events: none;
  opacity: 0;
  background-color: #f7f8fa;
}

.preload-content {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.preload-component {
  width: 100%;
  height: 100%;
  visibility: hidden;
}

/* 页面过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.fade-leave-to {
  opacity: 0;
  transform: translateY(-5px);
}

/* 从启动页过渡的特殊效果 */
.splash-transition-enter-active {
  transition: all 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.splash-transition-enter-from {
  opacity: 0;
  transform: scale(0.95) translateY(20px);
}

/* 确保页面切换时没有白屏 */
.router-view-container {
  min-height: 100vh;
  background-color: #f7f8fa;
}

/* 防止内容被底部导航遮挡：统一底部内边距 */
.page-content {
  padding-bottom: calc(66px + env(safe-area-inset-bottom, 0px));
  padding-bottom: calc(66px + constant(safe-area-inset-bottom, 0px)); /* iOS 11.0 */
}

/* NavBar样式已迁移到main.css中统一管理 */

/* 确保TabBar内容正确对齐 */
.van-tabbar .van-tabbar-item {
  min-height: 56px !important;
  display: flex !important;
  flex-direction: column !important;
  justify-content: center !important;
  align-items: center !important;
}

/* 为包含TabBar的常用页面类添加底部内边距（兼容旧页面样式） — 已由 .page-content 统一处理，这里移除避免重复间距 */

/* 为有固定导航栏的页面内容区域添加顶部内边距 - 注释掉避免与main.css中的设置冲突 */
/* 
.category-content,
.detail-content,
.content,
.login-content,
.register-content {
  padding-top: calc(46px + env(safe-area-inset-top)) !important;
  padding-top: calc(46px + constant(safe-area-inset-top)) !important;
}
*/

/* 社区页面的特殊处理 */
.page-with-fixed-navbar {
  padding-top: 8px !important;
}

/* 针对已确认有导航栏的安卓设备特殊处理 */
.has-android-navbar .page-content {
  padding-bottom: calc(66px + var(--android-navbar-height, 30px)) !important;
}

/* Android系统导航栏底部间距增强处理 */
@supports not (padding-bottom: env(safe-area-inset-bottom)) {
  /* 只有确认有导航栏的设备才添加额外间距 */
  .has-android-navbar .page-content {
    /* 对于不支持safe-area的Android设备，使用更大的底部间距 */
    padding-bottom: 96px !important;
  }
}

/* Android设备的额外底部间距处理 */
@media screen and (min-resolution: 0.001dpcm) {
  @supports (-webkit-appearance: none) {
    /* 只有确认有导航栏的设备才添加额外间距 */
    .has-android-navbar .page-content {
      /* Android Chrome特殊处理，确保内容不被系统导航栏遮挡 */
      padding-bottom: calc(66px + var(--android-navbar-height, 30px)) !important;
    }
  }
}

/* 移除多余的安卓底部导航栏样式，改为更精确的类选择器控制 */
.android-device .page-content {
  /* 默认不添加额外间距 */
  padding-bottom: 66px;
}
</style> 