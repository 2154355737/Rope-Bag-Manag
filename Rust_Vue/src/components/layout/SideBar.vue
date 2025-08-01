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
              <component :is="iconComponents[item.icon]" />
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
  Message, // 新增订阅管理图标
  Tickets // 新增标签图标
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
  { path: '/admin/comments', title: '评论管理', icon: 'ChatDotRound', badge: 0, roles: ['admin', 'moderator'] },
  { path: '/admin/user-actions', title: '行为记录', icon: 'Monitor', badge: 0, roles: ['admin'] },
  { path: '/admin/resource-records', title: '资源记录', icon: 'Files', badge: 0, roles: ['admin'] },
  { path: '/admin/backup', title: '数据备份', icon: 'Download', badge: 0, roles: ['admin'] },
  { path: '/admin/announcements', title: '公告管理', icon: 'Bell', badge: 0, roles: ['admin', 'moderator'] },
  { path: '/admin/logs', title: '日志查看', icon: 'Document', badge: 0, roles: ['admin'] },
  { path: '/admin/stats', title: '统计信息', icon: 'DataAnalysis', badge: 0, roles: ['admin', 'moderator'] },
  { path: '/admin/theme-settings', title: '系统设置', icon: 'Setting', badge: 0, roles: ['admin'] },
  { path: '/admin/community-settings', title: '社区设置', icon: 'House', badge: 0, roles: ['admin'] },
  { path: '/admin/posts', title: '帖子管理', icon: 'ChatDotRound', badge: 0, roles: ['admin', 'elder'] },
  { path: '/admin/tags', title: '标签管理', icon: 'Tickets', badge: 0, roles: ['admin', 'elder'] },
  { path: '/admin/mail-settings', title: '邮件设置', icon: 'Setting', badge: 0, roles: ['admin'] },
  { path: '/admin/subscriptions', title: '订阅管理', icon: 'Message', badge: 0, roles: ['admin'] }
]
const menuItems = ref<MenuItem[]>([])

// 图标组件映射
const iconComponents: Record<string, any> = {
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
  Tickets
}

// 例如在按钮点击事件中调用
// databaseExport.exportToFile('database.sqlite')
</script>

<style scoped>
.desktop-sidebar {
  position: fixed;
  top: 0;
  left: 0;
  width: 240px;
  height: 100vh;
  background-color: var(--bg-card);
  border-right: 1px solid var(--border-color);
  z-index: 999;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.sidebar-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding-top: 64px;
}

.sidebar-nav {
  flex: 1;
  padding: 16px 0;
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  color: var(--text-secondary);
  text-decoration: none;
  transition: all 0.3s ease;
  position: relative;
  margin: 4px 8px;
  border-radius: 8px;
}

.nav-item:hover {
  color: var(--brand-color);
  background-color: var(--bg-primary);
}

.nav-item.active {
  color: var(--brand-color);
  background-color: var(--bg-primary);
  font-weight: 500;
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  background-color: var(--brand-color);
  border-radius: 0 2px 2px 0;
}

.nav-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  flex-shrink: 0;
  transition: transform 0.2s ease;
}

.nav-item:hover .nav-icon,
.nav-item.active .nav-icon {
  transform: scale(1.1);
}

.nav-text {
  flex: 1;
  font-size: 14px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-badge {
  flex-shrink: 0;
  transform: scale(0.8);
}

.sidebar-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
}

.footer-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.version-info {
  text-align: center;
}

.version-text {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.status-info {
  display: flex;
  justify-content: center;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-dot.online {
  background-color: var(--success-color);
  box-shadow: 0 0 6px var(--success-color);
}

.status-text {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 滚动条样式 */
.sidebar-nav::-webkit-scrollbar {
  width: 4px;
}

.sidebar-nav::-webkit-scrollbar-track {
  background: transparent;
}

.sidebar-nav::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 2px;
}

.sidebar-nav::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style> 