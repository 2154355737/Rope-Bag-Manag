<template>
  <div v-if="showNotice" class="auto-switch-notice">
    <el-alert
      :title="noticeTitle"
      :description="noticeDescription"
      type="info"
      :closable="true"
      show-icon
      @close="closeNotice"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { shouldUseMobileVersion } from '../utils/device'

const route = useRoute()
const showNotice = ref(false)
const noticeTitle = ref('')
const noticeDescription = ref('')

onMounted(() => {
  // 检查是否需要显示自动切换提示
  const currentPath = route.path
  const shouldUseMobile = shouldUseMobileVersion()
  
  if (currentPath === '/users-mobile' && !shouldUseMobile) {
    // 在桌面端显示移动端页面
    showNotice.value = true
    noticeTitle.value = '移动端界面'
    noticeDescription.value = '当前显示的是移动端优化界面，系统会根据您的设备自动选择最适合的界面。'
  } else if (currentPath === '/users' && shouldUseMobile) {
    // 在移动端显示桌面端页面
    showNotice.value = true
    noticeTitle.value = '桌面端界面'
    noticeDescription.value = '当前显示的是桌面端界面，系统会根据您的设备自动选择最适合的界面。'
  }
})

function closeNotice() {
  showNotice.value = false
}
</script>

<style scoped>
.auto-switch-notice {
  position: fixed;
  top: 80px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 2000;
  max-width: 90%;
  width: 400px;
}

@media (max-width: 768px) {
  .auto-switch-notice {
    top: 70px;
    width: 95%;
  }
}

/* 主题切换支持 */
:deep(.dark) .auto-switch-notice .el-alert {
  background-color: #2d2d2d;
  border-color: #404040;
  color: #ffffff;
}

:deep(.light) .auto-switch-notice .el-alert {
  background-color: #ffffff;
  border-color: #e4e7ed;
  color: #303133;
}
</style> 