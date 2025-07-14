<template>
  <div class="desktop-layout">
    <!-- 顶部导航栏 -->
    <NavBar />
    
    <!-- 侧边栏 -->
    <SideBar />
    
    <!-- 主内容区域 -->
    <main class="desktop-main">
      <div class="content-wrapper">
        <router-view />
      </div>
    </main>
    
    <!-- 全局加载状态 -->
    <el-loading v-if="globalLoading" :fullscreen="true" text="加载中..." />
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
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  overflow: hidden;
}

.desktop-main {
  margin-left: 240px;
  margin-top: 64px;
  flex: 1;
  overflow-y: auto;
  background-color: var(--bg-primary);
  transition: all 0.3s ease;
}

.content-wrapper {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
}



/* 响应式设计 */
@media (max-width: 1200px) {
  .content-wrapper {
    padding: 20px;
  }
}

@media (max-width: 768px) {
  .desktop-layout {
    display: none;
  }
}

/* 滚动条样式 */
.desktop-main::-webkit-scrollbar {
  width: 8px;
}

.desktop-main::-webkit-scrollbar-track {
  background: transparent;
}

.desktop-main::-webkit-scrollbar-thumb {
  background-color: var(--border-color);
  border-radius: 4px;
}

.desktop-main::-webkit-scrollbar-thumb:hover {
  background-color: var(--text-secondary);
}

/* 深色模式适配 */
.dark .desktop-layout {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .desktop-main {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

/* 主题适配 */
.blue .desktop-layout::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .desktop-layout::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .desktop-layout::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .desktop-layout::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .desktop-main {
    transition: none;
  }
}
</style> 