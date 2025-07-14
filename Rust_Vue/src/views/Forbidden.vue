<template>
  <div class="forbidden-page">
    <div class="forbidden-icon">
      <el-icon :size="80"><Lock /></el-icon>
    </div>
    <h1 class="forbidden-title">403 无权限访问</h1>
    <p class="forbidden-desc">您没有权限访问此页面，请联系管理员或切换账号。</p>
    <el-button type="primary" @click="goHome">返回首页</el-button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { Lock } from '@element-plus/icons-vue'
const router = useRouter()
function goHome() {
  router.push('/')
}
</script>

<style scoped>
.forbidden-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  padding: 40px 0;
}
.forbidden-icon {
  margin-bottom: 24px;
  color: #f56c6c;
}
.forbidden-title {
  font-size: 32px;
  font-weight: bold;
  margin-bottom: 12px;
  color: #333;
}
.forbidden-desc {
  font-size: 16px;
  color: #888;
  margin-bottom: 24px;
}
</style> 