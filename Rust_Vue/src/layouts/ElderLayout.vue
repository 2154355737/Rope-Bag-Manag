<template>
  <div class="elder-layout">
    <NavBar />
    <SideBar />
    <main class="elder-main">
      <div class="content-wrapper">
        <router-view />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import NavBar from '@/components/layout/NavBar.vue'
import SideBar from '@/components/layout/ElderSideBar.vue'
</script>

<style scoped>
.elder-layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  overflow: hidden;
}
.elder-main {
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

.elder-profile-card {
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: 0 2px 16px rgba(0,0,0,0.08);
  padding: 32px 40px;
  min-width: 400px;
  max-width: 600px;
  width: 100%;
  color: var(--text-primary);
}
</style> 