<template>
  <div class="community-link">
    <el-button
      type="primary"
      size="large"
      @click="openCommunity"
      class="community-btn"
    >
      <el-icon><Share /></el-icon>
      资源社区
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
.community-link {
  display: flex;
  justify-content: center;
  padding: var(--spacing-lg);
}

.community-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
  padding: var(--spacing-md) var(--spacing-lg);
  border-radius: var(--border-radius-base);
  box-shadow: var(--shadow-light);
  transition: all var(--transition-base);
}

.community-btn:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
}
</style> 