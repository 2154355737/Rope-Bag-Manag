<template>
  <div id="app">
    <router-view v-slot="{ Component, route }">
      <template v-if="route.meta && route.meta.keepAlive">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
      </template>
      <template v-else>
        <component :is="Component" />
      </template>
    </router-view>
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useUserStore } from './store/user';

const router = useRouter();
const userStore = useUserStore();

onMounted(async () => {
  // 尝试自动登录（从本地存储获取token）
  await userStore.checkAuth();
});
</script>

<style>
#app {
  min-height: 100vh;
  background-color: #f7f8fa;
}
</style> 