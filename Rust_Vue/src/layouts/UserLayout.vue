<template>
  <div class="user-layout">
    <NavBar />
    <SideBar />
    <main class="user-main">
      <div class="content-wrapper">
        <router-view />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import NavBar from '@/components/layout/NavBar.vue'
import SideBar from '@/components/layout/UserSideBar.vue'
</script>

<style scoped>
.user-layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  overflow: hidden;
}
.user-main {
  margin-left: 240px;
  margin-top: 64px;
  flex: 1;
  overflow-y: auto;
  background-color: var(--bg-primary);
  transition: all 0.3s ease;
}
.content-wrapper {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
}

.user-profile-card {
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: 0 2px 16px rgba(0,0,0,0.08);
  padding: 32px 40px;
  min-width: 400px;
  max-width: 600px;
  width: 100%;
  color: var(--text-primary);
}

@media (max-width: 768px) {
  .content-wrapper {
    max-width: 100% !important;
    padding: 8px !important;
    margin: 0 !important;
  }
}
</style> 