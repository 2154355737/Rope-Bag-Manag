<template>
  <div class="resource-list">
    <!-- 骨架屏加载状态 -->
    <template v-if="loading && resources.length === 0">
      <van-skeleton title :row="3" class="mb-3" v-for="i in 3" :key="i" />
    </template>
    
    <!-- 资源列表 -->
    <template v-else>
      <resource-card 
        v-for="resource in resources" 
        :key="resource.id" 
        :resource="resource"
        :showCover="showCover"
        :showActions="showActions"
        @click="onResourceClick(resource)"
        @more="onMore(resource)"
      />
    </template>
    
    <!-- 加载状态 -->
    <div class="list-loading" v-if="loading && resources.length > 0">
      <van-loading size="24px" vertical>加载中...</van-loading>
    </div>
    
    <!-- 没有更多数据 -->
    <div class="list-finished" v-if="finished && resources.length > 0">
      <van-divider>没有更多了</van-divider>
    </div>
    
    <!-- 空状态 -->
    <div class="list-empty" v-if="!loading && resources.length === 0">
      <van-empty image="search" :description="emptyText" />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import ResourceCard from './ResourceCard.vue';

// 定义属性
const props = defineProps({
  resources: {
    type: Array,
    default: () => []
  },
  loading: {
    type: Boolean,
    default: false
  },
  finished: {
    type: Boolean,
    default: false
  },
  showCover: {
    type: Boolean,
    default: true
  },
  emptyText: {
    type: String,
    default: '暂无资源'
  },
  showActions: {
    type: Boolean,
    default: false
  }
});

// 事件
const emit = defineEmits(['resource-click', 'load-more', 'more']);

// 点击资源
const onResourceClick = (resource) => {
  emit('resource-click', resource);
};

// 卡片更多
const onMore = (resource) => {
  emit('more', resource);
};
</script>

<style scoped>
.resource-list {
  padding: 8px 0;
}

.list-loading,
.list-finished,
.list-empty {
  padding: 16px 0;
  text-align: center;
}

.mb-3 {
  margin-bottom: 12px;
}
</style> 