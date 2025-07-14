<template>
  <aside class="user-sidebar">
    <nav class="user-menu">
      <router-link to="/user/resources" class="user-menu-item" active-class="active">我的资源</router-link>
      <router-link to="/user/comments" class="user-menu-item" active-class="active">我的评论</router-link>
      <router-link to="/user/profile" class="user-menu-item" active-class="active">个人信息</router-link>
    </nav>
  </aside>
</template>

<script setup lang="ts">
</script>

<style scoped>
.user-sidebar {
  width: 180px;
  background: var(--bg-card);
  border-right: 1px solid var(--border-color);
  padding-top: 24px;
  height: 100vh;
}
.user-menu {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.user-menu-item {
  padding: 12px 24px;
  color: var(--text-secondary);
  text-decoration: none;
  border-radius: 6px;
  transition: background 0.2s;
}
.user-menu-item.active, .user-menu-item:hover {
  background: var(--bg-primary);
  color: var(--brand-color);
}
</style> 