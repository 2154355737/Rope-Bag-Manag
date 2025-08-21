<template>
  <aside class="desktop-sidebar">
    <div class="sidebar-content">
      <!-- 导航菜单 -->
      <nav class="sidebar-nav">
        <router-link 
          v-for="item in menuItems" 
          :key="item.path"
          :to="item.path"
          class="nav-item"
          :class="{ active: currentPath === item.path }"
          :title="item.title"
        >
          <div class="nav-icon">
            <el-icon :size="20">
              <component :is="iconComponents[item.icon as keyof typeof iconComponents]" />
            </el-icon>
          </div>
          <span class="nav-text">{{ item.title }}</span>
          <div class="nav-badge" v-if="item.badge && item.badge > 0">
            <el-badge :value="item.badge" :hidden="item.badge === 0" />
          </div>
        </router-link>
      </nav>
      
      <!-- 底部信息 -->
      <div class="sidebar-footer">
        <div class="footer-info">
          <div class="version-info">
            <span class="version-text">v1.0.0</span>
          </div>
          <div class="status-info">
            <div class="status-item">
              <span class="status-dot online"></span>
              <span class="status-text">系统正常</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { getUserInfo } from '../../utils/auth'
import {
  House,
  User,
  Box,
  Document,
  DataAnalysis,
  Setting,
  ChatDotRound,
  Monitor,
  Files,
  Download,
  Bell,
  UserFilled,
  DocumentChecked,
  Message, // 订阅管理和邮件设置图标
  Tickets, // 标签管理图标
  Lock, // IP封禁和下载安全图标
  EditPen, // 帖子管理图标
  DocumentCopy // 帖子审核图标
} from '@element-plus/icons-vue'
import { databaseExport } from '../../utils/sqliteExport'

const route = useRoute()

// 组件挂载时根据用户角色过滤菜单
onMounted(() => {
  const user = getUserInfo()
  const role = user?.role || 'user'
  menuItems.value = allMenuItems.filter(item => !item.roles || item.roles.includes(role))
})

// 当前路径
const currentPath = computed(() => route.path)

// 侧边栏菜单项配置 - 从API获取
interface MenuItem {
  path: string
  title: string
  icon: string
  badge: number
  roles?: string[] // 可见角色
}
const allMenuItems: MenuItem[] = [
  { path: '/admin', title: '仪表盘', icon: 'House', badge: 0 },
  { path: '/admin/users', title: '用户管理', icon: 'User', badge: 0, roles: ['admin'] },
  { path: '/admin/packages', title: '资源管理', icon: 'Box', badge: 0, roles: ['admin', 'moderator'] },
  { path: '/admin/resource-review', title: '资源审核', icon: 'DocumentChecked', badge: 0, roles: ['admin', 'elder'] },
  { path: '/admin/post-review', title: '帖子审核', icon: 'DocumentCopy', badge: 0, roles: ['admin', 'elder'] },
  { path: '/admin/posts', title: '帖子管理', icon: 'EditPen', badge: 0, roles: ['admin', 'elder'] },
  { path: '/admin/comments', title: '评论管理', icon: 'ChatDotRound', badge: 0, roles: ['admin', 'moderator'] },
  { path: '/admin/tags', title: '标签管理', icon: 'Tickets', badge: 0, roles: ['admin', 'elder'] },
  { path: '/admin/user-actions', title: '行为记录', icon: 'Monitor', badge: 0, roles: ['admin'] },
  { path: '/admin/resource-records', title: '资源记录', icon: 'Files', badge: 0, roles: ['admin'] },
  { path: '/admin/backup', title: '数据备份', icon: 'Download', badge: 0, roles: ['admin'] },
  { path: '/admin/announcements', title: '公告管理', icon: 'Bell', badge: 0, roles: ['admin', 'moderator'] },
  { path: '/admin/subscriptions', title: '订阅管理', icon: 'Message', badge: 0, roles: ['admin'] },
  { path: '/admin/mail-settings', title: '邮件设置', icon: 'Message', badge: 0, roles: ['admin'] },
  { path: '/admin/download-security', title: '下载安全', icon: 'Lock', badge: 0, roles: ['admin'] },
  { path: '/admin/ip-ban', title: 'IP封禁', icon: 'Lock', badge: 0, roles: ['admin'] },
  { path: '/admin/logs', title: '日志查看', icon: 'Document', badge: 0, roles: ['admin'] },
  { path: '/admin/stats', title: '统计信息', icon: 'DataAnalysis', badge: 0, roles: ['admin', 'moderator'] },
  { path: '/admin/homepage-settings', title: '主页设置', icon: 'Setting', badge: 0, roles: ['admin'] },
  { path: '/admin/theme-settings', title: '主题设置', icon: 'Setting', badge: 0, roles: ['admin'] },
  { path: '/admin/community-settings', title: '社区设置', icon: 'Setting', badge: 0, roles: ['admin'] }
]

// 响应式菜单项
const menuItems = ref<MenuItem[]>([])

// Element Plus 图标组件映射
const iconComponents = {
  House,
  User,
  Box,
  Document,
  DataAnalysis,
  Setting,
  ChatDotRound,
  Monitor,
  Files,
  Download,
  Bell,
  UserFilled,
  DocumentChecked,
  Message,
  Tickets,
  Lock,
  EditPen,
  DocumentCopy
}

// 数据库导出功能
const handleExportDatabase = async () => {
  try {
    await databaseExport.exportToFile()
  } catch (error) {
    console.error('数据库导出失败:', error)
  }
}
</script>

<style scoped>
.desktop-sidebar {
  grid-area: sidebar;
  width: 100%;
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color-light);
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 100;
  transition: var(--transition-normal);
}

.sidebar-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.sidebar-nav {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: var(--space-lg) var(--space-md);
  gap: var(--space-xs);
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  padding: var(--space-lg) var(--space-xl);
  border-radius: var(--radius-lg);
  color: var(--text-secondary);
  text-decoration: none;
  transition: var(--transition-fast);
  position: relative;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  min-height: 48px;
  box-sizing: border-box;
}

.nav-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  transform: translateX(4px);
}

.nav-item.active {
  background: var(--bg-selected);
  color: var(--text-brand);
  font-weight: var(--font-weight-semibold);
  border-left: 4px solid var(--brand-color);
  padding-left: calc(var(--space-xl) - 4px);
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 24px;
  background: var(--brand-color);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}

.nav-icon {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  color: inherit;
}

.nav-text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: inherit;
  line-height: var(--line-height-tight);
}

.nav-badge {
  flex-shrink: 0;
}

.nav-badge .el-badge {
  line-height: 1;
}

.sidebar-footer {
  padding: var(--space-xl);
  border-top: 1px solid var(--border-color-light);
  background: var(--bg-tertiary);
}

.footer-info {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

.version-info {
  text-align: center;
}

.version-text {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  font-weight: var(--font-weight-medium);
  background: var(--bg-secondary);
  padding: var(--space-xs) var(--space-sm);
  border-radius: var(--radius-full);
  border: 1px solid var(--border-color-light);
}

.status-info {
  display: flex;
  justify-content: center;
}

.status-item {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-dot.online {
  background: var(--success-color);
  box-shadow: 0 0 6px rgba(82, 196, 26, 0.4);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% { box-shadow: 0 0 6px rgba(82, 196, 26, 0.4); }
  50% { box-shadow: 0 0 12px rgba(82, 196, 26, 0.6); }
  100% { box-shadow: 0 0 6px rgba(82, 196, 26, 0.4); }
}

.status-text {
  font-weight: var(--font-weight-medium);
}

/* 滚动条优化 */
.sidebar-nav::-webkit-scrollbar {
  width: 4px;
}

.sidebar-nav::-webkit-scrollbar-track {
  background: transparent;
}

.sidebar-nav::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: var(--radius-full);
}

.sidebar-nav::-webkit-scrollbar-thumb:hover {
  background: var(--brand-color);
}

/* 深色模式适配 */
html.dark .desktop-sidebar {
  background: var(--bg-secondary);
  border-right-color: var(--border-color);
}

html.dark .sidebar-footer {
  background: var(--bg-tertiary);
  border-top-color: var(--border-color);
}

html.dark .version-text {
  background: var(--bg-elevated);
  border-color: var(--border-color);
}

html.dark .nav-item:hover {
  background: var(--bg-hover);
}

html.dark .nav-item.active {
  background: var(--bg-selected);
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .sidebar-nav {
    padding: var(--space-md) var(--space-sm);
  }
  
  .nav-item {
    padding: var(--space-md) var(--space-lg);
    gap: var(--space-md);
  }
  
  .nav-item.active {
    padding-left: calc(var(--space-lg) - 4px);
  }
}

@media (max-width: 1024px) {
  .nav-item {
    padding: var(--space-sm) var(--space-md);
    min-height: 40px;
  }
  
  .nav-item.active {
    padding-left: calc(var(--space-md) - 4px);
  }
  
  .sidebar-footer {
    padding: var(--space-lg);
  }
}

@media (max-width: 768px) {
  .desktop-sidebar {
    display: none;
  }
}

/* 触摸设备优化 */
@media (hover: none) and (pointer: coarse) {
  .nav-item:hover {
    transform: none;
  }
  
  .nav-item:active {
    background: var(--bg-selected);
    transform: scale(0.98);
  }
}

/* 减少动画模式支持 */
@media (prefers-reduced-motion: reduce) {
  .nav-item,
  .status-dot.online {
    transition: none !important;
    animation: none !important;
  }
  
  .nav-item:hover {
    transform: none !important;
  }
}

/* 高对比度模式支持 */
@media (prefers-contrast: high) {
  .nav-item {
    border: 1px solid transparent;
  }
  
  .nav-item:hover,
  .nav-item.active {
    border-color: var(--brand-color);
  }
  
  .sidebar-footer {
    border-top-width: 2px;
  }
}
</style> 