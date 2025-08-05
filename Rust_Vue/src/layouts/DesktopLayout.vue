<template>
  <div class="desktop-layout">
    <!-- 顶部导航栏 -->
    <NavBar />
    
    <!-- 侧边栏 -->
    <SideBar />
    
    <!-- 主内容区域 -->
    <main class="desktop-main">
      <div class="content-wrapper" v-loading="globalLoading" element-loading-text="加载中...">
        <router-view />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, provide } from 'vue'
import NavBar from '../components/layout/NavBar.vue'
import SideBar from '../components/layout/SideBar.vue'

// 全局加载状态
const globalLoading = ref(false)

// 提供给子组件的全局状态
provide('globalLoading', globalLoading)
provide('setGlobalLoading', (loading: boolean) => {
  globalLoading.value = loading
})
</script>

<style scoped>
.desktop-layout {
  height: 100vh;
  display: grid;
  grid-template-areas: 
    "navbar navbar"
    "sidebar main";
  grid-template-columns: 280px 1fr;
  grid-template-rows: 72px 1fr;
  background-color: var(--bg-primary);
  overflow: hidden;
  transition: var(--transition-normal);
}

.desktop-main {
  grid-area: main;
  overflow-y: auto;
  background-color: var(--bg-primary);
  display: flex;
  flex-direction: column;
  position: relative;
}

.content-wrapper {
  flex: 1;
  padding: var(--space-3xl);
  max-width: 1440px;
  width: 100%;
  margin: 0 auto;
  box-sizing: border-box;
}

/* 响应式设计 - 大屏幕 */
@media (min-width: 1400px) {
  .desktop-layout {
    grid-template-columns: 320px 1fr;
  }
  
  .content-wrapper {
    max-width: 1600px;
    padding: var(--space-4xl);
  }
}

/* 响应式设计 - 中等屏幕 */
@media (max-width: 1200px) {
  .desktop-layout {
    grid-template-columns: 240px 1fr;
  }
  
  .content-wrapper {
    padding: var(--space-2xl);
    max-width: 1200px;
  }
}

/* 响应式设计 - 小屏幕 */
@media (max-width: 1024px) {
  .desktop-layout {
    grid-template-columns: 200px 1fr;
  }
  
  .content-wrapper {
    padding: var(--space-xl);
    max-width: 100%;
  }
}

/* 响应式设计 - 平板 */
@media (max-width: 768px) {
  .desktop-layout {
    grid-template-areas: 
      "navbar navbar"
      "main main";
    grid-template-columns: 1fr;
    grid-template-rows: 72px 1fr;
  }
  
  .content-wrapper {
    padding: var(--space-lg);
    max-width: 100%;
  }
}

/* 响应式设计 - 手机 */
@media (max-width: 480px) {
  .content-wrapper {
    padding: var(--space-md);
  }
}

/* 滚动条样式优化 */
.desktop-main::-webkit-scrollbar {
  width: 8px;
}

.desktop-main::-webkit-scrollbar-track {
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
}

.desktop-main::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: var(--radius-sm);
  transition: var(--transition-fast);
}

.desktop-main::-webkit-scrollbar-thumb:hover {
  background: var(--brand-color);
}

/* 深色模式适配 */
html.dark .desktop-layout {
  background: var(--bg-primary);
}

html.dark .desktop-main {
  background: var(--bg-primary);
}

html.dark .desktop-main::-webkit-scrollbar-track {
  background: var(--bg-tertiary);
}

html.dark .desktop-main::-webkit-scrollbar-thumb {
  background: var(--border-color);
}

html.dark .desktop-main::-webkit-scrollbar-thumb:hover {
  background: var(--brand-color);
}

/* 加载状态优化 */
.content-wrapper .el-loading-mask {
  background-color: var(--bg-glass-strong) !important;
  backdrop-filter: blur(20px) !important;
}

.content-wrapper .el-loading-spinner {
  color: var(--brand-color) !important;
}

.content-wrapper .el-loading-text {
  color: var(--text-primary) !important;
  font-weight: var(--font-weight-medium) !important;
}

/* 平滑过渡效果 */
.desktop-layout * {
  box-sizing: border-box;
}

@media (prefers-reduced-motion: reduce) {
  .desktop-layout,
  .desktop-main,
  .content-wrapper {
    transition: none !important;
  }
}
</style> 