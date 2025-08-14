<template>
  <div class="post-card" @click="handleClick">
    <div class="post-card__content">
      <div class="post-card__header">
        <i-mdi-forum-outline class="post-card__icon" />
        <h3 class="post-card__title ellipsis">{{ post.title }}</h3>
        <div class="tags">
          <van-tag v-if="post.is_pinned" type="danger" size="mini" plain>置顶</van-tag>
          <van-tag v-if="post.is_featured" type="warning" size="mini" plain>精华</van-tag>
          <van-tag v-if="statusInfo" :type="statusInfo.type" size="mini" plain>{{ statusInfo.text }}</van-tag>
        </div>
      </div>

      <p class="post-card__desc ellipsis-2" v-if="post.content">{{ post.content }}</p>

      <div class="post-card__info">
        <span>{{ post.author_name || ('用户#' + post.author_id) }}</span>
        <span>{{ formatDate(post.created_at) }}</span>
      </div>

      <div class="post-card__tags" v-if="visibleTags.length">
        <van-tag 
          v-for="tag in visibleTags" 
          :key="tag" 
          plain 
          round
          size="small" 
          type="primary"
          class="mr-1"
        >
          {{ tag }}
        </van-tag>
        <van-tag v-if="post.tags && post.tags.length > maxTags" plain round size="small" type="primary">+{{ (post.tags?.length || 0) - maxTags }}</van-tag>
      </div>

      <div class="post-card__footer">
        <div class="post-card__stats">
          <van-icon name="eye-o" /> {{ post.view_count || 0 }}
          <van-icon name="like-o" class="ml-2" /> {{ post.like_count || 0 }}
          <van-icon name="comment-o" class="ml-2" /> {{ post.comment_count || 0 }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import dayjs from 'dayjs';

const emit = defineEmits(['click']);

const props = defineProps({ 
  post: { type: Object, required: true },
  maxTags: { type: Number, default: 2 },
});

const handleClick = () => {
  emit('click', props.post);
};

// 优先使用业务状态 status，其次参考审核状态 review_status
const statusInfo = computed(() => {
  const st = String(props.post.status || '').toLowerCase();
  const rs = String(props.post.review_status || '').toLowerCase();
  if (st === 'published') return { text: '已发布', type: 'success' };
  if (st === 'draft') return { text: '草稿', type: 'warning' };
  if (rs === 'approved') return { text: '已发布', type: 'success' };
  if (rs === 'rejected') return { text: '已拒绝', type: 'danger' };
  if (rs === 'pending') return { text: '待审核', type: 'warning' };
  return null;
});

// 可见标签（列表接口可能没有 tags）
const visibleTags = computed(() => {
  if (!props.post.tags) return [];
  return props.post.tags.slice(0, props.maxTags);
});

const formatDate = (d) => (d ? dayjs(d).format('YYYY-MM-DD') : '');
</script>

<style scoped>
.post-card {
  background-color: #fff;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  margin-bottom: 12px;
  position: relative;
  cursor: pointer;
}

.post-card__content { padding: 12px; }

.post-card__header { 
  display: flex; 
  align-items: flex-start; 
  gap: 8px; 
  flex-wrap: wrap;
}
.post-card__icon { width: 28px; height: 28px; color: var(--primary-color); }

/* 标签横向排列样式 */
.tags {
  display: flex;
  gap: 4px;
  align-items: center;
  flex-shrink: 0;
  flex-wrap: nowrap;
  margin-left: auto;
}

.post-card__title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 6px;
  color: var(--text-color);
  flex: 1;
  max-width: 160px;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.post-card__desc {
  font-size: 14px;
  color: var(--text-color-light);
  margin-bottom: 8px;
  line-height: 1.4;
  max-height: 40px;
}

.post-card__info {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-color-lighter);
  margin-bottom: 8px;
}

.post-card__tags { margin-bottom: 8px; }

.post-card__footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 8px;
  font-size: 12px;
  color: var(--text-color-lighter);
  border-top: 1px solid var(--border-color);
}

.post-card__stats { display: flex; align-items: center; }

.mr-1 { margin-right: 4px; }
.ml-2 { margin-left: 8px; }
</style> 