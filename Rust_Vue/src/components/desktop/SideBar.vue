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
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { 
  House,
  User,
  Box,
  Document,
  DataAnalysis
} from '@element-plus/icons-vue'

const route = useRoute()

// 当前路径
const currentPath = computed(() => route.path)

// 侧边栏菜单项配置
const menuItems = [
  {
    path: '/dashboard',
    title: '仪表盘',
    icon: 'House',
    badge: 0
  },
  {
    path: '/users',
    title: '用户管理',
    icon: 'User',
    badge: 0
  },
  {
    path: '/packages',
    title: '绳包管理',
    icon: 'Box',
    badge: 0
  },
  {
    path: '/logs',
    title: '日志查看',
    icon: 'Document',
    badge: 5
  },
  {
    path: '/stats',
    title: '统计信息',
    icon: 'DataAnalysis',
    badge: 0
  }
]

// 图标组件映射
const iconComponents: Record<string, any> = {
  House,
  User,
  Box,
  Document,
  DataAnalysis
}
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
  background-color: var(--bg-primary);
}

.footer-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.version-info {
  display: flex;
  justify-content: center;
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
  background-color: #67c23a;
  box-shadow: 0 0 0 2px rgba(103, 194, 58, 0.2);
}

.status-text {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .desktop-sidebar {
    display: none;
  }
}

/* 深色模式适配 */
.dark .desktop-sidebar {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

.dark .sidebar-footer {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

/* 主题适配 */
.blue .desktop-sidebar::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .desktop-sidebar::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .desktop-sidebar::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .desktop-sidebar::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.blue .nav-item.active,
.green .nav-item.active,
.purple .nav-item.active,
.orange .nav-item.active {
  color: var(--brand-color);
}

.blue .nav-item.active::before,
.green .nav-item.active::before,
.purple .nav-item.active::before,
.orange .nav-item.active::before {
  background-color: var(--brand-color);
}

/* 高对比度模式 */
@media (prefers-contrast: high) {
  .nav-item.active::before {
    width: 4px;
  }
  
  .desktop-sidebar {
    border-right-width: 2px;
  }
  
  .sidebar-footer {
    border-top-width: 2px;
  }
}

/* 动画效果 */
@keyframes slide-in {
  0% {
    opacity: 0;
    transform: translateX(-20px);
  }
  100% {
    opacity: 1;
    transform: translateX(0);
  }
}

/* 页面加载动画 */
.desktop-sidebar {
  animation: slide-in 0.6s ease-out;
}

/* 导航项动画 */
.nav-item {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.nav-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.05), transparent);
  transition: left 0.5s ease;
}

.nav-item:hover::before {
  left: 100%;
}

/* 图标动画 */
.nav-icon {
  transition: transform 0.3s ease;
}

.nav-item:hover .nav-icon,
.nav-item.active .nav-icon {
  transform: scale(1.1);
}

/* 徽章动画 */
.nav-badge {
  transition: transform 0.3s ease;
}

.nav-item:hover .nav-badge {
  transform: scale(1.1);
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .desktop-sidebar {
    animation: none;
  }
  
  .nav-item,
  .nav-icon,
  .nav-badge {
    transition: none;
  }
  
  .nav-item::before {
    display: none;
  }
  
  .nav-item:hover .nav-icon,
  .nav-item.active .nav-icon {
    transform: none;
  }
  
  .nav-item:hover .nav-badge {
    transform: none;
  }
}
</style> 