<template>
  <div class="post-card" @click="$emit('click', post)">
    <div class="header">
      <div class="title ellipsis-2">{{ post.title }}</div>
      <van-tag v-if="statusInfo" :type="statusInfo.type" size="mini" plain>{{ statusInfo.text }}</van-tag>
      <van-tag v-if="post.is_featured" type="primary" size="mini" plain class="ml8">精选</van-tag>
      <van-tag v-if="post.is_pinned" type="warning" size="mini" plain class="ml8">置顶</van-tag>
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
.post-card { background: #fff; border-radius: 8px; padding: 12px; margin-bottom: 10px; }
.header { display: flex; align-items: center; gap: 6px; margin-bottom: 6px; }
.title { font-size: 16px; font-weight: 600; color: var(--text-color); flex: 1; }
.meta { font-size: 12px; color: var(--text-color-lighter); display: flex; gap: 10px; margin-bottom: 6px; }
.excerpt { font-size: 14px; color: var(--text-color-light); }
.ml8 { margin-left: 8px; }
</style> 