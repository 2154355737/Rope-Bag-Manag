<template>
  <aside class="home-sidebar">
    <!-- 内容分类 -->
    <div class="sidebar-panel">
      <div class="panel-header" @click="toggleCategory">
        <h3>
          <el-icon><Menu /></el-icon>
          内容分类
        </h3>
        <el-icon class="collapse-icon" :class="{ collapsed: !categoryExpanded }">
          <ArrowDown />
        </el-icon>
      </div>
      <el-collapse-transition>
        <div v-show="categoryExpanded" class="panel-content">
          <div class="category-list">
            <div 
              class="category-item"
              :class="{ active: activeCategory === 'all' }"
              @click="$emit('category-change', 'all')"
            >
              <el-icon><Grid /></el-icon>
              <span>全部资源</span>
              <div class="category-count">{{ totalResources }}</div>
            </div>
            <div 
              v-for="category in categories" 
              :key="category.id"
              class="category-item"
              :class="{ active: activeCategory === category.id.toString() }"
              @click="$emit('category-change', category.id.toString())"
            >
              <el-icon><Document /></el-icon>
              <span>{{ category.name }}</span>
              <div class="category-count">{{ getCategoryCount(category.id) }}</div>
            </div>
          </div>
        </div>
      </el-collapse-transition>
    </div>
    
    <!-- 社区公告 -->
    <div class="sidebar-panel">
      <div class="panel-header" @click="toggleAnnouncement">
        <h3>
          <el-icon><Bell /></el-icon>
          社区公告
        </h3>
        <el-icon class="collapse-icon" :class="{ collapsed: !announcementExpanded }">
          <ArrowDown />
        </el-icon>
      </div>
      <el-collapse-transition>
        <div v-show="announcementExpanded" class="panel-content">
          <div v-if="!announcements.length" class="empty-state">
            <el-empty description="暂无公告" :image-size="60" />
          </div>
          <div v-else class="announcement-list">
            <div 
              v-for="announcement in announcements.slice(0, 5)" 
              :key="announcement.id" 
              class="announcement-item"
            >
              <div class="announcement-dot"></div>
              <div class="announcement-content">
                <div class="announcement-text">{{ announcement.text }}</div>
                <div class="announcement-time">{{ formatTime(announcement.created_at) }}</div>
              </div>
            </div>
          </div>
        </div>
      </el-collapse-transition>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Menu, ArrowDown, Grid, Document, Bell } from '@element-plus/icons-vue'

// Props
interface Category {
  id: number
  name: string
}

interface Announcement {
  id: number
  text: string
  created_at: string
}

interface Props {
  categories?: Category[]
  announcements?: Announcement[]
  activeCategory?: string
  totalResources?: number
  resources?: any[]
}

const props = withDefaults(defineProps<Props>(), {
  categories: () => [],
  announcements: () => [],
  activeCategory: 'all',
  totalResources: 0,
  resources: () => []
})

// Emits
const emit = defineEmits<{
  'category-change': [category: string]
}>()

// 响应式数据
const categoryExpanded = ref(true)
const announcementExpanded = ref(true)

// 计算属性
const getCategoryCount = (categoryId: number) => {
  return props.resources.filter(resource => resource.category_id === categoryId).length
}

// 方法
const toggleCategory = () => {
  categoryExpanded.value = !categoryExpanded.value
}

const toggleAnnouncement = () => {
  announcementExpanded.value = !announcementExpanded.value
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
.home-sidebar {
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
  cursor: pointer;
  transition: all 0.2s ease;
}

.panel-header:hover {
  background: var(--bg-hover);
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

.collapse-icon {
  color: var(--text-tertiary);
  transition: transform 0.3s ease;
  font-size: 16px;
}

.collapse-icon.collapsed {
  transform: rotate(-90deg);
}

.panel-content {
  padding: 20px;
  max-height: 320px;
  overflow-y: auto;
}

.category-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.category-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-radius: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--bg-tertiary);
  border: 1px solid transparent;
}

.category-item:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
  transform: translateX(4px);
  border-color: var(--border-color-hover);
}

.category-item.active {
  color: var(--brand-color);
  background: var(--bg-brand-subtle);
  border-color: var(--brand-color);
  font-weight: 600;
  transform: translateX(6px);
}

.category-item .el-icon {
  font-size: 16px;
  flex-shrink: 0;
}

.category-item span {
  flex: 1;
  font-size: 14px;
}

.category-count {
  background: var(--bg-secondary);
  color: var(--text-tertiary);
  font-size: 12px;
  font-weight: 600;
  padding: 4px 8px;
  border-radius: 8px;
  min-width: 20px;
  text-align: center;
  border: 1px solid var(--border-color-light);
}

.category-item.active .category-count {
  background: var(--brand-color);
  color: white;
  border-color: var(--brand-color);
}

.announcement-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.announcement-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px;
  border-radius: 12px;
  background: var(--bg-tertiary);
  transition: all 0.2s ease;
}

.announcement-item:hover {
  background: var(--bg-hover);
  transform: translateX(4px);
}

.announcement-dot {
  width: 8px;
  height: 8px;
  background: var(--brand-color);
  border-radius: 50%;
  flex-shrink: 0;
  margin-top: 6px;
}

.announcement-content {
  flex: 1;
  min-width: 0;
}

.announcement-text {
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.5;
  margin-bottom: 4px;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.announcement-time {
  font-size: 12px;
  color: var(--text-tertiary);
}

.empty-state {
  text-align: center;
  padding: 32px 16px;
}

/* 滚动条样式 */
.home-sidebar::-webkit-scrollbar,
.panel-content::-webkit-scrollbar {
  width: 6px;
}

.home-sidebar::-webkit-scrollbar-track,
.panel-content::-webkit-scrollbar-track {
  background: var(--bg-secondary);
  border-radius: 3px;
}

.home-sidebar::-webkit-scrollbar-thumb,
.panel-content::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 3px;
  transition: background 0.2s ease;
}

.home-sidebar::-webkit-scrollbar-thumb:hover,
.panel-content::-webkit-scrollbar-thumb:hover {
  background: var(--brand-color);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .home-sidebar {
    position: static;
    max-height: none;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }
}

@media (max-width: 768px) {
  .home-sidebar {
    grid-template-columns: 1fr;
    gap: 16px;
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