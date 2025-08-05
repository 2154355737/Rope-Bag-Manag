<template>
  <div 
    class="content-card"
    :class="[type]"
    @click="$emit('click', item)"
  >
    <!-- 资源卡片 -->
    <template v-if="type === 'resources'">
      <div class="card-header">
        <div class="card-icon">
          <el-icon size="20" :color="getCategoryColor(item.category_id)">
            <Document />
          </el-icon>
        </div>
        <div class="card-title">
          <div class="title-row">
            <h4>{{ item.name }}</h4>
            <div class="status-badges" v-if="item.is_featured || item.is_pinned">
              <el-tag v-if="item.is_pinned" type="danger" size="small" effect="dark">
                <el-icon><Star /></el-icon>
                置顶
              </el-tag>
              <el-tag v-if="item.is_featured" type="warning" size="small" effect="dark">
                <el-icon><Trophy /></el-icon>
                精华
              </el-tag>
            </div>
          </div>
          <span class="card-category">{{ getCategoryLabel(item.category_id) }}</span>
        </div>
        <div class="card-actions">
          <el-dropdown trigger="click" @click.stop>
            <el-button link size="small">
              <el-icon><MoreFilled /></el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item @click.stop="$emit('click', item)">
                  <el-icon><View /></el-icon>
                  查看详情
                </el-dropdown-item>
                <el-dropdown-item @click.stop="$emit('download', item)">
                  <el-icon><Download /></el-icon>
                  立即下载
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </div>
      
      <div class="card-body">
        <p class="card-description">{{ item.description || '暂无描述' }}</p>
        <div class="card-tags" v-if="item.tags && item.tags.length > 0">
          <el-tag 
            v-for="tag in item.tags.slice(0, 3)" 
            :key="tag"
            size="small"
            effect="plain"
          >
            {{ tag }}
          </el-tag>
        </div>
      </div>
      
      <div class="card-footer">
        <div class="footer-meta">
          <span class="meta-item">
            <el-icon><User /></el-icon>
            {{ item.author }}
          </span>
          <span class="meta-item">
            <el-icon><Download /></el-icon>
            {{ item.download_count || 0 }}
          </span>
          <span class="meta-item">
            <el-icon><Clock /></el-icon>
            {{ formatTime(item.created_at) }}
          </span>
        </div>
        <div class="footer-actions">
          <el-button 
            type="primary" 
            size="small"
            @click.stop="$emit('download', item)"
          >
            下载
          </el-button>
        </div>
      </div>
    </template>

    <!-- 帖子卡片 -->
    <template v-else-if="type === 'posts'">
      <div class="post-header">
        <div class="post-avatar">
          <el-avatar :size="36" :src="item.author_avatar">
            <el-icon><User /></el-icon>
          </el-avatar>
        </div>
        <div class="post-author">
          <div class="author-name">{{ item.author_name || item.author }}</div>
          <div class="post-time">{{ formatTime(item.created_at) }}</div>
        </div>
        <div class="post-category">
          <el-tag size="small">{{ item.category || '未分类' }}</el-tag>
        </div>
      </div>
      
      <div class="post-content">
        <div class="post-title-row">
          <h3 class="post-title">{{ item.title }}</h3>
          <div class="status-badges" v-if="item.is_pinned || item.is_featured">
            <el-tag v-if="item.is_pinned" type="danger" size="small" effect="dark">
              <el-icon><Star /></el-icon>
              置顶
            </el-tag>
            <el-tag v-if="item.is_featured" type="warning" size="small" effect="dark">
              <el-icon><Trophy /></el-icon>
              精华
            </el-tag>
          </div>
        </div>
        <p class="post-excerpt">{{ getExcerpt(item.content) }}</p>
      </div>
      
      <div class="post-footer">
        <div class="post-stats">
          <span class="stat-item">
            <el-icon><View /></el-icon>
            {{ item.view_count || 0 }}
          </span>
          <span class="stat-item">
            <el-icon><ChatLineRound /></el-icon>
            {{ item.comment_count || 0 }}
          </span>
          <span class="stat-item">
            <el-icon><Star /></el-icon>
            {{ item.like_count || 0 }}
          </span>
        </div>
        <div class="post-actions">
          <el-button link size="small">
            阅读更多
          </el-button>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { 
  Document, 
  Star, 
  Trophy, 
  MoreFilled, 
  View, 
  Download, 
  User, 
  Clock, 
  ChatLineRound 
} from '@element-plus/icons-vue'

// Props
interface Props {
  item: any
  type: 'resources' | 'posts'
  categories?: Array<{ id: number; name: string }>
}

const props = withDefaults(defineProps<Props>(), {
  categories: () => []
})

// Emits
const emit = defineEmits<{
  click: [item: any]
  download: [item: any]
}>()

// 计算属性
const getCategoryLabel = (categoryId: number | null) => {
  if (!categoryId) return '未分类'
  const category = props.categories.find(c => c.id === categoryId)
  return category ? category.name : '未分类'
}

const getCategoryColor = (categoryId: any) => {
  const colorMap: { [key: string]: string } = {
    1: '#409EFF', // 蓝色
    2: '#67C23A', // 绿色
    3: '#E6A23C', // 黄色
    4: '#F56C6C', // 红色
    5: '#909399'  // 灰色
  }
  return colorMap[categoryId] || '#409EFF'
}

// 方法
const formatTime = (dateString: string) => {
  if (!dateString) return '刚刚'
  
  const date = new Date(dateString)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  const minutes = Math.floor(diff / (1000 * 60))
  const hours = Math.floor(diff / (1000 * 60 * 60))
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 30) return `${days}天前`
  
  return date.toLocaleDateString()
}

const getExcerpt = (content: string) => {
  if (!content) return '暂无内容'
  return content.length > 120 ? content.substring(0, 120) + '...' : content
}
</script>

<style scoped>
.content-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color-light);
  border-radius: 16px;
  overflow: hidden;
  transition: all 0.3s ease;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  height: fit-content;
  display: flex;
  flex-direction: column;
}

.content-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  border-color: var(--brand-color);
}

/* 资源卡片样式 */
.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 16px 20px;
  border-bottom: 1px solid var(--border-color-light);
  background: var(--bg-secondary);
}

.card-icon {
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color-light);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.card-title {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 4px;
}

.title-row h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.status-badges {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.status-badges .el-tag {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 6px;
}

.status-badges .el-icon {
  font-size: 10px;
  margin-right: 2px;
}

.card-category {
  font-size: 12px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 2px 8px;
  border-radius: 6px;
  border: 1px solid var(--border-color-light);
  font-weight: 500;
  width: fit-content;
}

.card-actions {
  flex-shrink: 0;
}

.card-body {
  padding: 16px 20px;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.card-description {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  flex: 1;
}

.card-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: auto;
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background: var(--bg-tertiary);
  border-top: 1px solid var(--border-color-light);
  margin-top: auto;
}

.footer-meta {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 12px;
  color: var(--text-tertiary);
  font-weight: 500;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--border-color-light);
}

.footer-actions {
  flex-shrink: 0;
}

/* 帖子卡片样式 */
.post-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 16px 20px;
  border-bottom: 1px solid var(--border-color-light);
  background: var(--bg-secondary);
}

.post-avatar {
  flex-shrink: 0;
}

.post-author {
  flex: 1;
}

.author-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.post-time {
  font-size: 12px;
  color: var(--text-muted);
}

.post-category {
  flex-shrink: 0;
}

.post-content {
  padding: 20px;
  flex: 1;
}

.post-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.post-title-row .status-badges {
  display: flex;
  gap: 8px;
}

.post-content .status-badges .el-tag {
  font-size: 12px;
  padding: 2px 6px;
  border-radius: 4px;
}

.post-content .status-badges .el-icon {
  font-size: 12px;
  margin-right: 4px;
}

.post-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-excerpt {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background: var(--bg-tertiary);
  border-top: 1px solid var(--border-color-light);
  margin-top: auto;
}

.post-stats {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--text-muted);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--border-color-light);
}

.post-actions {
  flex-shrink: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .card-header,
  .post-header {
    padding: 16px;
  }
  
  .card-body,
  .post-content {
    padding: 16px;
  }
  
  .card-footer,
  .post-footer {
    padding: 12px 16px;
    flex-direction: column;
    gap: 12px;
    align-items: stretch;
  }
  
  .footer-meta,
  .post-stats {
    justify-content: center;
    flex-wrap: wrap;
    gap: 8px;
  }
  
  .footer-actions,
  .post-actions {
    display: flex;
    justify-content: center;
  }
}

@media (max-width: 480px) {
  .title-row {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .status-badges {
    align-self: flex-end;
  }
  
  .footer-meta,
  .post-stats {
    gap: 4px;
  }
  
  .meta-item,
  .stat-item {
    font-size: 10px;
    padding: 2px 6px;
  }
}
</style> 