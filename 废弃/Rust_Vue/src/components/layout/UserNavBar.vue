<template>
  <header class="user-navbar">
    <div class="navbar-title">用户中心</div>
    <div class="navbar-actions">
      <!-- 可放退出、设置等按钮 -->
    </div>
  </header>
</template>

<script setup lang="ts">
</script>

<style scoped>
.user-navbar {
  height: 56px;
  background: var(--bg-card);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 32px;
  border-bottom: 1px solid var(--border-color);
}
.navbar-title {
  font-size: 20px;
  font-weight: bold;
}
</style> 