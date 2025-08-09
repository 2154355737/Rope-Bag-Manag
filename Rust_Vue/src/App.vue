<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import DesktopLayout from './layouts/DesktopLayout.vue'
import UserLayout from './layouts/UserLayout.vue'
import ElderLayout from './layouts/ElderLayout.vue'
import { getDeviceType } from './utils/device'

// Stagewise 工具栏集成
import { initToolbar } from '@stagewise/toolbar'

const route = useRoute()
const router = useRouter()

const layoutComponent = computed(() => {
  // 只在明确有 layout 字段时才渲染布局
  const layout = route.meta.layout
  if (!layout) return null
  if (layout === 'independent') return null
  if (layout === 'user') return UserLayout
  if (layout === 'elder') return ElderLayout
  if (layout === 'desktop') return DesktopLayout
  return null
})

// 仅桌面端布局，不进行移动端跳转

// 初始化 Stagewise 工具栏
const isDev = import.meta.env.DEV
if (isDev) {
  initToolbar({
    plugins: []
  })
}
</script>

<template>
  <component v-if="layoutComponent" :is="layoutComponent" :key="route.fullPath" />
  <router-view v-else />
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
