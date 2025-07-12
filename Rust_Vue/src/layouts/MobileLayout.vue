<template>
  <div class="mobile-layout">
    <!-- 移动端顶部导航 -->
    <MobileHeader />
    
    <!-- 主内容区域 -->
    <main class="mobile-main">
      <div class="content-wrapper">
        <slot />
      </div>
    </main>
    
    <!-- 底部导航栏 -->
    <MobileTabBar />
    
    <!-- 全局加载状态 -->
    <div v-if="globalLoading" class="global-loading">
      <el-loading-spinner />
      <span class="loading-text">加载中...</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, provide } from 'vue'
import MobileHeader from '../components/mobile/MobileHeader.vue'
import MobileTabBar from '../components/mobile/MobileTabBar.vue'

// 全局加载状态
const globalLoading = ref(false)

// 提供给子组件的全局状态
provide('globalLoading', globalLoading)
provide('setGlobalLoading', (loading: boolean) => {
  globalLoading.value = loading
})
</script>

<style scoped>
.mobile-layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  overflow: hidden;
}

.mobile-main {
  flex: 1;
  overflow-y: auto;
  margin-top: 56px;
  margin-bottom: 60px;
  background-color: var(--bg-primary);
  transition: all 0.3s ease;
}

.content-wrapper {
  padding: 16px;
  max-width: 100%;
  margin: 0 auto;
}

/* 全局加载状态 */
.global-loading {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  backdrop-filter: blur(4px);
}

.loading-text {
  margin-top: 12px;
  color: #ffffff;
  font-size: 14px;
  font-weight: 500;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .content-wrapper {
    padding: 12px;
  }
}

@media (max-width: 360px) {
  .content-wrapper {
    padding: 8px;
  }
}

/* 滚动条样式 */
.mobile-main::-webkit-scrollbar {
  width: 4px;
}

.mobile-main::-webkit-scrollbar-track {
  background: transparent;
}

.mobile-main::-webkit-scrollbar-thumb {
  background-color: var(--border-color);
  border-radius: 2px;
}

.mobile-main::-webkit-scrollbar-thumb:hover {
  background-color: var(--text-secondary);
}

/* 深色模式适配 */
.dark .mobile-layout {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .mobile-main {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

/* 主题适配 */
.blue .mobile-layout::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .mobile-layout::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .mobile-layout::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .mobile-layout::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .mobile-main {
    transition: none;
  }
}
</style> 