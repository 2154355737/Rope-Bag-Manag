<script setup lang="ts">
import { onMounted, ref, computed, nextTick } from 'vue'
import { useRoute } from 'vue-router'
import { initTheme } from './utils/theme'
import { debugDeviceInfo } from './utils/device'
import DesktopLayout from './layouts/DesktopLayout.vue'

// 初始化主题
onMounted(() => {
  initTheme()
})

const route = useRoute()
const isLayoutReady = ref(false)

// 调试信息
const debugLayout = () => {
  debugDeviceInfo()
  console.log('🖥️ 当前布局: 桌面端')
}

// 判断当前路由的布局类型
const routeLayout = computed(() => {
  return route.meta?.layout || 'desktop'
})

// 判断是否需要独立布局
const needsIndependentLayout = computed(() => {
  return routeLayout.value === 'independent'
})

// 初始化布局
const initLayout = async () => {
  debugLayout()
  
  // 等待下一个tick确保路由信息已更新
  await nextTick()
  
  // 设置布局就绪状态
  isLayoutReady.value = true
  
  console.log('🎨 布局初始化完成:', {
    route: route.path,
    layout: routeLayout.value,
    device: 'desktop',
    isIndependent: needsIndependentLayout.value
  })
}

onMounted(async () => {
  await initLayout()
})
</script>

<template>
  <div id="app" class="theme-transition">
    <!-- 加载状态 -->
    <div v-if="!isLayoutReady" class="loading-container">
      <div class="loading-spinner">
        <div class="spinner"></div>
        <p>正在加载...</p>
      </div>
    </div>
    
    <!-- 布局内容 -->
    <template v-else>
      <!-- 独立布局：资源社区和登录页面 -->
      <template v-if="needsIndependentLayout">
        <router-view />
      </template>
      
      <!-- 后台管理布局 -->
      <template v-else>
        <!-- 桌面端布局 -->
        <DesktopLayout>
          <router-view />
        </DesktopLayout>
      </template>
    </template>
  </div>
</template>

<style scoped>
/* 主题切换动画 */
.theme-transition {
  transition: all 0.3s ease;
}

.theme-transition * {
  transition: color 0.3s ease, background-color 0.3s ease, border-color 0.3s ease;
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .theme-transition,
  .theme-transition * {
    transition: none !important;
  }
}

/* 全局样式重置 */
#app {
  height: 100vh;
  overflow: hidden;
}

/* 确保布局组件占满整个视口 */
#app > div {
  height: 100vh;
  width: 100vw;
}

/* 加载状态样式 */
.loading-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: var(--el-bg-color);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.loading-spinner {
  text-align: center;
  color: var(--el-text-color-primary);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--el-border-color-light);
  border-top: 3px solid var(--el-color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.loading-spinner p {
  margin: 0;
  font-size: 14px;
  color: var(--el-text-color-regular);
}
</style>
