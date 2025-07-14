<template>
  <div class="user-layout">
    <UserNavBar />
    <div class="user-main">
      <UserSideBar />
      <main class="user-content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import UserNavBar from '@/components/layout/UserNavBar.vue'
import UserSideBar from '@/components/layout/UserSideBar.vue'
</script>

<style scoped>
.user-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
}
.user-main {
  display: flex;
  flex: 1;
}
.user-content {
  flex: 1;
  padding: 32px;
  background: var(--bg-primary);
  overflow-y: auto;
}
</style> 