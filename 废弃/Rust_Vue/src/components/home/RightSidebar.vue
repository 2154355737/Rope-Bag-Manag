<template>
  <aside class="right-sidebar">
    <!-- 热门内容 -->
    <div class="sidebar-panel">
      <div class="panel-header">
        <h3>
          <el-icon><Star /></el-icon>
          热门内容
        </h3>
        <el-button 
          link 
          size="small" 
          :loading="loadingStates.popular"
          @click="$emit('refresh-popular')"
          class="refresh-btn"
        >
          <el-icon><Refresh /></el-icon>
        </el-button>
      </div>
      <div class="panel-content">
        <div v-if="loadingStates.popular" class="loading-state">
          <el-skeleton :rows="4" animated />
        </div>
        <div v-else-if="!popularContent.length" class="empty-state">
          <p>暂无热门内容</p>
        </div>
        <div v-else class="content-list">
          <div
            v-for="(item, index) in popularContent"
            :key="item.id"
            class="content-item"
            @click="handleContentClick(item)"
          >
            <div class="item-rank">
              <span class="rank-number" :class="getRankClass(index)">{{ index + 1 }}</span>
            </div>
            <div class="item-content">
              <div class="item-title">{{ item.title }}</div>
              <div class="item-stats">
                <span class="stat-item">
                  <el-icon><View /></el-icon>
                  {{ formatNumber(item.views) }}
                </span>
                <span v-if="item.type === 'resource'" class="stat-item">
                  <el-icon><Download /></el-icon>
                  {{ formatNumber(item.downloads || 0) }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 社区动态 -->
    <div class="sidebar-panel">
      <div class="panel-header">
        <h3>
          <el-icon><ChatDotRound /></el-icon>
          社区动态
        </h3>
        <el-button 
          link 
          size="small" 
          :loading="loadingStates.activities"
          @click="$emit('refresh-activities')"
          class="refresh-btn"
        >
          <el-icon><Refresh /></el-icon>
        </el-button>
      </div>
      <div class="panel-content">
        <div v-if="loadingStates.activities" class="loading-state">
          <el-skeleton :rows="3" animated />
        </div>
        <div v-else-if="!recentActivities.length" class="empty-state">
          <el-empty description="暂无动态" :image-size="60" />
        </div>
        <div v-else class="activity-list">
          <div 
            v-for="activity in recentActivities.slice(0, 8)" 
            :key="activity.id"
            class="activity-item"
          >
            <div class="activity-content">
              <div class="activity-text">
                <strong>{{ activity.username }}</strong>
                {{ activity.action }}
                <span class="activity-target">{{ activity.target }}</span>
              </div>
              <div class="activity-time">{{ formatTime(activity.created_at) }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 热门标签 -->
    <div class="sidebar-panel">
      <div class="panel-header">
        <h3>
          <el-icon><PriceTag /></el-icon>
          热门标签
        </h3>
        <el-button 
          link 
          size="small" 
          :loading="loadingStates.tags"
          @click="$emit('refresh-tags')"
          class="refresh-btn"
        >
          <el-icon><Refresh /></el-icon>
        </el-button>
      </div>
      <div class="panel-content">
        <div v-if="loadingStates.tags" class="loading-state">
          <el-skeleton :rows="3" animated />
        </div>
        <div v-else-if="!hotTags.length" class="empty-state">
          <p>暂无热门标签</p>
        </div>
        <div v-else class="hot-tags">
          <el-tag 
            v-for="tag in hotTags.slice(0, 12)"
            :key="tag.id"
            size="small"
            class="tag-item"
            :style="{ 
              backgroundColor: tag.color, 
              borderColor: 'transparent',
              color: '#FFFFFF'
            }"
            @click="$emit('tag-click', tag.name)"
          >
            {{ tag.name }}
            <span class="tag-count">({{ tag.count }})</span>
          </el-tag>
        </div>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { 
  Star, 
  Refresh, 
  View, 
  Download, 
  ChatDotRound, 
  PriceTag 
} from '@element-plus/icons-vue'

// Props
interface Props {
  popularContent?: Array<{ id: number; type: string; title: string; views: number; downloads?: number }>
  recentActivities?: Array<{ id: number; username: string; action: string; target: string; created_at: string }>
  hotTags?: Array<{ id: number; name: string; count: number; color: string }>
  loadingStates?: {
    popular: boolean
    activities: boolean
    tags: boolean
  }
}

const props = withDefaults(defineProps<Props>(), {
  popularContent: () => [],
  recentActivities: () => [],
  hotTags: () => [],
  loadingStates: () => ({
    popular: false,
    activities: false,
    tags: false
  })
})

// Emits
const emit = defineEmits<{
  'refresh-popular': []
  'refresh-activities': []
  'refresh-tags': []
  'tag-click': [tagName: string]
}>()

// 方法
const handleContentClick = (item: any) => {
  // 可以添加跳转逻辑
  console.log('点击内容:', item)
}

const getRankClass = (index: number) => {
  if (index === 0) return 'rank-first'
  if (index === 1) return 'rank-second'  
  if (index === 2) return 'rank-third'
  return 'rank-normal'
}

const formatNumber = (num: number) => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

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
</script>

<style scoped>
.right-sidebar {
  display: flex;
  flex-direction: column;
  gap: 24px;
  position: sticky;
  top: 96px;
  max-height: calc(100vh - 120px);
  overflow-y: auto;
  scrollbar-width: thin;
}

.sidebar-panel {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color-light);
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.sidebar-panel:hover {
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
  transform: translateY(-2px);
}

.panel-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color-light);
  background: var(--bg-secondary);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.panel-header h3 {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.panel-header h3 .el-icon {
  color: var(--brand-color);
  font-size: 18px;
}

.refresh-btn {
  padding: 4px 8px;
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.refresh-btn:hover {
  opacity: 1;
}

.panel-content {
  padding: 20px;
  max-height: 320px;
  overflow-y: auto;
}

.loading-state,
.empty-state {
  padding: 32px 16px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 14px;
}

.content-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.content-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 12px;
  background: var(--bg-tertiary);
  transition: all 0.2s ease;
  cursor: pointer;
}

.content-item:hover {
  background: var(--bg-hover);
  transform: translateX(4px);
}

.item-rank {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.rank-number {
  font-size: 12px;
  font-weight: 700;
  color: var(--text-secondary);
}

.rank-first { color: #f56565; }
.rank-second { color: #ed8936; }
.rank-third { color: #38b2ac; }

.item-content {
  flex: 1;
  min-width: 0;
}

.item-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-stats {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: var(--text-tertiary);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.activity-item {
  padding: 12px;
  border-radius: 12px;
  background: var(--bg-tertiary);
  transition: all 0.2s ease;
}

.activity-item:hover {
  background: var(--bg-hover);
  transform: translateX(4px);
}

.activity-text {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.4;
  margin-bottom: 4px;
}

.activity-text strong {
  font-weight: 600;
  color: var(--text-primary);
}

.activity-target {
  font-weight: 500;
  color: var(--brand-color);
}

.activity-time {
  font-size: 12px;
  color: var(--text-tertiary);
}

.hot-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.tag-item {
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
  border: none !important;
  border-radius: 16px !important;
  padding: 4px 12px !important;
  font-size: 12px !important;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.tag-item:hover {
  transform: translateY(-2px) scale(1.05);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.tag-count {
  margin-left: 4px;
  opacity: 0.8;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.2);
  padding: 1px 6px;
  border-radius: 8px;
  font-size: 10px;
  display: inline-block;
  min-width: 16px;
  text-align: center;
  line-height: 1.2;
}

/* 滚动条样式 */
.right-sidebar::-webkit-scrollbar,
.panel-content::-webkit-scrollbar {
  width: 6px;
}

.right-sidebar::-webkit-scrollbar-track,
.panel-content::-webkit-scrollbar-track {
  background: var(--bg-secondary);
  border-radius: 3px;
}

.right-sidebar::-webkit-scrollbar-thumb,
.panel-content::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 3px;
  transition: background 0.2s ease;
}

.right-sidebar::-webkit-scrollbar-thumb:hover,
.panel-content::-webkit-scrollbar-thumb:hover {
  background: var(--brand-color);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .right-sidebar {
    position: static;
    max-height: none;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;
  }
}

@media (max-width: 768px) {
  .sidebar-panel {
    border-radius: 12px;
  }
  
  .panel-header {
    padding: 16px 20px;
  }
  
  .panel-content {
    padding: 16px;
    max-height: 240px;
  }
}
</style> 