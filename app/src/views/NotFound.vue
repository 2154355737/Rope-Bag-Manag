<template>
  <div class="not-found-page">
    <van-empty
      image="error"
      description="页面不存在"
    >
      <template #bottom>
        <van-button round type="primary" @click="goHome">
          返回首页
        </van-button>
      </template>
    </van-empty>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router';

const router = useRouter();

// 返回首页
const goHome = () => {
  router.replace('/');
};
</script>

<style scoped>
.not-found-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding: 20px;
}
</style> 