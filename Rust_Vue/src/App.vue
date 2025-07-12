<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from 'vue'
import { initTheme } from './utils/theme'
import { getDeviceType, shouldUseMobileVersion, debugDeviceInfo } from './utils/device'
import DesktopLayout from './layouts/DesktopLayout.vue'
import MobileLayout from './layouts/MobileLayout.vue'

// 初始化主题
onMounted(() => {
  initTheme()
})

// 设备类型检测
const isMobile = ref(false)
const windowWidth = ref(0)

// 检测设备类型
const detectDeviceType = () => {
  windowWidth.value = window.innerWidth
  isMobile.value = shouldUseMobileVersion()
  
  // 调试信息
  debugDeviceInfo()
  console.log('📱 当前布局:', isMobile.value ? '移动端' : '桌面端')
}

// 监听窗口大小变化
const handleResize = () => {
  detectDeviceType()
}

// 计算当前布局
const currentLayout = computed(() => {
  return isMobile.value ? 'mobile' : 'desktop'
})

onMounted(() => {
  detectDeviceType()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<template>
  <div id="app" class="theme-transition">
    <!-- 桌面端布局 -->
    <DesktopLayout v-if="currentLayout === 'desktop'">
      <router-view />
    </DesktopLayout>
    
    <!-- 移动端布局 -->
    <MobileLayout v-else>
      <router-view />
    </MobileLayout>
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
</style>
