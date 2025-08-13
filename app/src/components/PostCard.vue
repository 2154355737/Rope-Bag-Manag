<template>
  <div class="post-card" @click="$emit('click', post)">
    <div class="header">
      <div class="title ellipsis-2">{{ post.title }}</div>
      <div class="tags">
        <van-tag v-if="post.is_pinned" type="warning" size="mini" plain>置顶</van-tag>
        <van-tag v-if="post.is_featured" type="primary" size="mini" plain>精选</van-tag>
        <van-tag v-if="statusInfo" :type="statusInfo.type" size="mini" plain>{{ statusInfo.text }}</van-tag>
      </div>
    </div>
    <div class="meta">
      <span>{{ post.author_name || '用户#'+post.author_id }}</span>
      <span>{{ formatDate(post.created_at) }}</span>
      <span>浏览 {{ post.view_count || 0 }}</span>
      <span>点赞 {{ post.like_count || 0 }}</span>
      <span>评论 {{ post.comment_count || 0 }}</span>
    </div>
    <div class="excerpt ellipsis-2">{{ post.content }}</div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import dayjs from 'dayjs';
const props = defineProps({ post: { type: Object, required: true } });
const statusInfo = computed(() => {
  const rs = (props.post.review_status || props.post.status || '').toLowerCase();
  if (!rs) return null;
  if (rs === 'pending' || rs === 'draft') return { text: '待审核', type: 'warning' };
  if (rs === 'approved' || rs === 'published') return { text: '已发布', type: 'success' };
  if (rs === 'rejected') return { text: '已拒绝', type: 'danger' };
  return null;
});
const formatDate = (d) => (d ? dayjs(d).format('YYYY-MM-DD HH:mm') : '');
</script>

<style scoped>
.post-card { 
  background: #fff; 
  border-radius: 8px; 
  padding: 12px; 
  margin-bottom: 10px; 
  cursor: pointer; 
}

.header { 
  display: flex; 
  align-items: center; 
  justify-content: space-between; 
  margin-bottom: 6px; 
}

.title { 
  font-size: 16px; 
  font-weight: 600; 
  color: var(--text-color); 
  flex: 1; 
  margin-right: 8px; 
}

.tags { 
  display: flex; 
  gap: 4px; 
  align-items: center; 
  flex-shrink: 0; 
  flex-wrap: nowrap; 
}

.meta { 
  font-size: 12px; 
  color: var(--text-color-lighter); 
  display: flex; 
  gap: 10px; 
  margin-bottom: 6px; 
}

.excerpt { 
  font-size: 14px; 
  color: var(--text-color-light); 
}
</style> 