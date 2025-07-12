<template>
  <div class="community-nav">
    <el-button
      type="primary"
      text
      @click="openCommunity"
      class="community-nav-btn"
    >
      <el-icon><Share /></el-icon>
      <span class="nav-text">资源社区</span>
    </el-button>
  </div>
</template>

<script setup lang="ts">
import { Share } from '@element-plus/icons-vue'

const openCommunity = () => {
  // 在新窗口打开资源社区
  window.open('/community', '_blank')
}
</script>

<style scoped>
.community-nav {
  display: flex;
  align-items: center;
}

.community-nav-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  color: var(--text-primary);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-base);
}

.community-nav-btn:hover {
  color: var(--brand-color);
  transform: translateX(2px);
}

.nav-text {
  font-size: var(--font-size-sm);
}

/* 在侧边栏中的样式 */
.community-nav.sidebar {
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border-color-light);
}

.community-nav.sidebar .community-nav-btn {
  width: 100%;
  justify-content: flex-start;
  padding: var(--spacing-sm) 0;
}

/* 在顶部导航中的样式 */
.community-nav.header {
  margin-left: var(--spacing-md);
}

.community-nav.header .community-nav-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--border-radius-small);
}

.community-nav.header .community-nav-btn:hover {
  background: var(--bg-secondary);
}
</style> 