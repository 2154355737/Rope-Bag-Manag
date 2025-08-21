<template>
  <div class="loading-skeleton">
    <div class="skeleton-header">
      <div class="skeleton-logo"></div>
      <div class="skeleton-search"></div>
      <div class="skeleton-actions">
        <div class="skeleton-btn"></div>
        <div class="skeleton-btn"></div>
      </div>
    </div>
    
    <div class="skeleton-tabs">
      <div class="skeleton-tab" v-for="i in 7" :key="i"></div>
    </div>
    
    <div class="skeleton-toolbar">
      <div class="skeleton-select"></div>
      <div class="skeleton-select"></div>
      <div class="skeleton-btn"></div>
    </div>
    
    <div class="skeleton-grid">
      <div class="skeleton-card" v-for="i in 6" :key="i">
        <div class="skeleton-image"></div>
        <div class="skeleton-content">
          <div class="skeleton-title"></div>
          <div class="skeleton-text"></div>
          <div class="skeleton-text short"></div>
          <div class="skeleton-meta">
            <div class="skeleton-meta-item"></div>
            <div class="skeleton-meta-item"></div>
          </div>
          <div class="skeleton-stats">
            <div class="skeleton-stat"></div>
            <div class="skeleton-stat"></div>
            <div class="skeleton-stat"></div>
          </div>
          <div class="skeleton-tags">
            <div class="skeleton-tag"></div>
            <div class="skeleton-tag"></div>
            <div class="skeleton-tag"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 骨架屏组件，用于加载状态
</script>

<style scoped>
.loading-skeleton {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-dark) 100%);
  padding: var(--spacing-lg);
}

.skeleton-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-radius: var(--border-radius-large);
  padding: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
}

.skeleton-logo {
  width: 120px;
  height: 40px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
}

.skeleton-search {
  flex: 1;
  height: 40px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-large);
}

.skeleton-actions {
  display: flex;
  gap: var(--spacing-md);
}

.skeleton-btn {
  width: 80px;
  height: 40px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
}

.skeleton-tabs {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(20px);
  border-radius: var(--border-radius-large);
  padding: var(--spacing-md);
  margin-bottom: var(--spacing-md);
  display: flex;
  gap: var(--spacing-md);
}

.skeleton-tab {
  width: 80px;
  height: 32px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-large);
}

.skeleton-toolbar {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px);
  border-radius: var(--border-radius-large);
  padding: var(--spacing-md);
  margin-bottom: var(--spacing-lg);
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
}

.skeleton-select {
  width: 120px;
  height: 32px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
}

.skeleton-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: var(--spacing-lg);
}

.skeleton-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: var(--border-radius-extra-large);
  overflow: hidden;
  box-shadow: var(--shadow-light);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
}

.skeleton-image {
  height: 200px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
}

.skeleton-content {
  padding: var(--spacing-lg);
}

.skeleton-title {
  height: 24px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
  margin-bottom: var(--spacing-md);
}

.skeleton-text {
  height: 16px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
  margin-bottom: var(--spacing-sm);
}

.skeleton-text.short {
  width: 60%;
}

.skeleton-meta {
  display: flex;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.skeleton-meta-item {
  width: 60px;
  height: 12px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
}

.skeleton-stats {
  display: flex;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.skeleton-stat {
  width: 40px;
  height: 12px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
}

.skeleton-tags {
  display: flex;
  gap: var(--spacing-xs);
}

.skeleton-tag {
  width: 50px;
  height: 20px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--border-radius-base);
}

@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

/* 深色模式适配 */
.dark .skeleton-logo,
.dark .skeleton-search,
.dark .skeleton-btn,
.dark .skeleton-tab,
.dark .skeleton-select,
.dark .skeleton-image,
.dark .skeleton-title,
.dark .skeleton-text,
.dark .skeleton-meta-item,
.dark .skeleton-stat,
.dark .skeleton-tag {
  background: linear-gradient(90deg, #404040 25%, #505050 50%, #404040 75%);
  background-size: 200% 100%;
}

.dark .skeleton-header,
.dark .skeleton-tabs,
.dark .skeleton-toolbar {
  background: rgba(44, 44, 44, 0.95);
}

.dark .skeleton-card {
  background: rgba(44, 44, 44, 0.95);
  border-color: rgba(255, 255, 255, 0.1);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .skeleton-header {
    flex-direction: column;
    gap: var(--spacing-md);
  }
  
  .skeleton-search {
    width: 100%;
  }
  
  .skeleton-actions {
    width: 100%;
    justify-content: center;
  }
  
  .skeleton-grid {
    grid-template-columns: 1fr;
  }
  
  .skeleton-tabs {
    overflow-x: auto;
  }
  
  .skeleton-tab {
    flex-shrink: 0;
  }
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .skeleton-logo,
  .skeleton-search,
  .skeleton-btn,
  .skeleton-tab,
  .skeleton-select,
  .skeleton-image,
  .skeleton-title,
  .skeleton-text,
  .skeleton-meta-item,
  .skeleton-stat,
  .skeleton-tag {
    animation: none;
  }
}
</style> 