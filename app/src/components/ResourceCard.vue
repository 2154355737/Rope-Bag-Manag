<template>
  <div class="resource-card" @click="handleClick">
    <!-- 封面已移除，保持紧凑卡片 -->
    <div class="resource-card__content">
      <div class="resource-card__header">
        <i-mdi-file-document-outline class="resource-card__icon" />
        <h3 class="resource-card__title ellipsis">{{ resource.name }}</h3>
        <van-tag
          v-if="statusInfo"
          :type="statusInfo.type"
          size="mini"
          plain
          class="status-tag"
        >
          {{ statusInfo.text }}
        </van-tag>
      </div>
      <div
        v-if="showActions"
        class="resource-card__more"
        @click.stop="handleMore"
        title="更多操作"
      >
        <van-icon name="ellipsis" />
      </div>
      <p class="resource-card__desc ellipsis-2" v-if="resource.description">{{ resource.description }}</p>
      <div class="resource-card__info">
        <span>{{ resource.author }}</span>
        <span>{{ formatDate(resource.created_at) }}</span>
      </div>
      <div class="resource-card__tags" v-if="resource.tags && resource.tags.length">
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
        <van-tag v-if="resource.tags.length > maxTags" plain round size="small" type="primary">+{{ resource.tags.length - maxTags }}</van-tag>
      </div>
      <div class="resource-card__footer">
        <div class="resource-card__stats">
          <van-icon name="down" /> {{ resource.download_count || 0 }}
          <van-icon name="like-o" class="ml-2" /> {{ resource.like_count || 0 }}
          <van-icon name="comment-o" class="ml-2" /> {{ resource.comment_count || 0 }}
        </div>
        <div class="resource-card__category" v-if="resource.category">
          {{ resource.category }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import dayjs from 'dayjs';
import defaultCover from '@/assets/resource-placeholder.svg';

const emit = defineEmits(['click', 'more']);

// 定义属性
const props = defineProps({
  resource: {
    type: Object,
    required: true
  },
  showCover: {
    type: Boolean,
    default: true
  },
  maxTags: {
    type: Number,
    default: 2
  },
  showActions: {
    type: Boolean,
    default: false
  }
});

const handleClick = () => {
  emit('click', props.resource);
};

const handleMore = () => {
  emit('more', props.resource);
};

// 计算显示的标签数量
const visibleTags = computed(() => {
  if (!props.resource.tags) return [];
  return props.resource.tags.slice(0, props.maxTags);
});

// 状态标签信息
const statusInfo = computed(() => {
  const s = (props.resource.status || '').toString().toLowerCase();
  if (!s) return null;
  switch (s) {
    case 'pending':
      return { text: '待审核', type: 'warning' };
    case 'active':
      return { text: '已审核', type: 'success' };
    case 'rejected':
      return { text: '已拒绝', type: 'danger' };
    case 'inactive':
      return { text: '已下架', type: 'default' };
    case 'deleted':
      return { text: '已删除', type: 'danger' };
    default:
      return { text: props.resource.status, type: 'primary' };
  }
});

// 格式化日期
const formatDate = (date) => {
  if (!date) return '';
  return dayjs(date).format('YYYY-MM-DD');
};
</script>

<style scoped>
.resource-card {
  background-color: #fff;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  margin-bottom: 12px;
  position: relative;
}

.resource-card__cover {
  height: 140px;
  position: relative;
  overflow: hidden;
  background-color: #f5f7fa;
}

.resource-card__cover img, 
.resource-card__cover .van-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.resource-card__tag {
  position: absolute;
  top: 8px;
  right: 8px;
}

.resource-card__content {
  padding: 12px;
}

/* 去掉多重白底重叠：让卡片背景露出一层 */
.resource-card__desc,
.resource-card__info,
.resource-card__tags,
.resource-card__footer {
  background: transparent;
}

.resource-card__header { display: flex; align-items: center; gap: 8px; }
.resource-card__icon { width: 28px; height: 28px; color: var(--primary-color); }

.resource-card__title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 6px;
  color: var(--text-color);
}

.status-tag { margin-left: 6px; }

/* 右上角更多操作按钮 */
.resource-card__more {
  position: absolute;
  top: 8px;
  right: 8px;
  color: var(--text-color-lighter);
}

.resource-card__desc {
  font-size: 14px;
  color: var(--text-color-light);
  margin-bottom: 8px;
  line-height: 1.4;
  max-height: 40px;
}

.resource-card__info {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-color-lighter);
  margin-bottom: 8px;
}

.resource-card__tags {
  margin-bottom: 8px;
}

.resource-card__footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 8px;
  font-size: 12px;
  color: var(--text-color-lighter);
  border-top: 1px solid var(--border-color);
}

.resource-card__stats {
  display: flex;
  align-items: center;
}

.resource-card__category {
  background-color: var(--background-color);
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
}

.mr-1 {
  margin-right: 4px;
}

.ml-2 {
  margin-left: 8px;
}
</style> 