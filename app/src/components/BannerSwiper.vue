<template>
  <div class="banner-swiper">
    <van-swipe :autoplay="3000" lazy-render indicator-color="white" :show-indicators="items.length > 1">
      <van-swipe-item v-for="(item, index) in items" :key="index" @click="onBannerClick(item)">
        <div class="banner-item">
          <van-image v-if="item.image" :src="item.image" fit="cover" />
          <div v-else class="banner-fallback">
            <i-mdi-image size="48" />
          </div>
          <div class="banner-title" v-if="item.title">{{ item.title }}</div>
        </div>
      </van-swipe-item>
    </van-swipe>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

// 定义属性
const props = defineProps({
  items: {
    type: Array,
    default: () => []
  }
});

// 事件
const emit = defineEmits(['banner-click']);

const router = useRouter();

// 点击轮播图
const onBannerClick = (item) => {
  emit('banner-click', item);
  
  // 根据banner类型执行不同操作
  if (item.type === 'resource' && item.resourceId) {
    router.push(`/resource/${item.resourceId}`);
  } else if (item.type === 'url' && item.url) {
    window.open(item.url, '_blank');
  } else if (item.type === 'category' && item.categoryId) {
    router.push(`/category/${item.categoryId}`);
  }
};
</script>

<style scoped>
.banner-swiper {
  margin: 12px 0;
  border-radius: 12px;
  overflow: hidden;
}

.banner-item {
  width: 100%;
  height: 160px;
  position: relative;
}

.banner-item .van-image {
  width: 100%;
  height: 100%;
}

.banner-fallback { display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; background: #f5f7fa; color: var(--primary-color); }

.banner-title {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(to top, rgba(0, 0, 0, 0.7), transparent);
  color: white;
  padding: 12px;
  font-size: 14px;
}
</style> 