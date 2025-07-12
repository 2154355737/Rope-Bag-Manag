<template>
  <nav class="mobile-tabbar">
    <div class="tabbar-container">
      <router-link 
        v-for="item in tabItems" 
        :key="item.path"
        :to="item.path"
        class="tab-item"
        :class="{ active: currentPath === item.path }"
        :title="item.title"
      >
        <div class="tab-icon">
          <el-icon :size="20">
            <component :is="iconComponents[item.icon]" />
          </el-icon>
        </div>
        <span class="tab-text">{{ item.title }}</span>
        <div class="tab-badge" v-if="item.badge && item.badge > 0">
          <el-badge :value="item.badge" :hidden="item.badge === 0" />
        </div>
        <div class="tab-indicator" v-if="currentPath === item.path"></div>
      </router-link>
    </div>
  </nav>
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

// 底部导航项配置
const tabItems = [
  {
    path: '/dashboard',
    title: '首页',
    icon: 'House',
    badge: 0
  },
  {
    path: '/users',
    title: '用户',
    icon: 'User',
    badge: 0
  },
  {
    path: '/packages',
    title: '绳包',
    icon: 'Box',
    badge: 0
  },
  {
    path: '/logs',
    title: '日志',
    icon: 'Document',
    badge: 5
  },
  {
    path: '/stats',
    title: '统计',
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
.mobile-tabbar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 60px;
  background-color: var(--bg-card);
  border-top: 1px solid var(--border-color);
  z-index: 1000;
  transition: all 0.3s ease;
  padding-bottom: env(safe-area-inset-bottom);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.tabbar-container {
  display: flex;
  align-items: center;
  justify-content: space-around;
  height: 100%;
  padding: 0 8px;
}

.tab-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  height: 100%;
  color: var(--text-secondary);
  text-decoration: none;
  transition: all 0.3s ease;
  position: relative;
  padding: 8px 4px;
  min-width: 0;
  border-radius: 8px;
  margin: 0 2px;
}

.tab-item:hover {
  color: var(--brand-color);
  background-color: var(--bg-primary);
}

.tab-item.active {
  color: var(--brand-color);
  background-color: var(--bg-primary);
}

.tab-indicator {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 20px;
  height: 3px;
  background-color: var(--brand-color);
  border-radius: 0 0 2px 2px;
  transition: all 0.3s ease;
}

.tab-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  margin-bottom: 4px;
  transition: transform 0.2s ease;
  position: relative;
}

.tab-item:hover .tab-icon,
.tab-item.active .tab-icon {
  transform: scale(1.1);
}

.tab-text {
  font-size: 10px;
  font-weight: 500;
  text-align: center;
  line-height: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.tab-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  transform: scale(0.8);
}

/* 响应式设计 */
@media (min-width: 769px) {
  .mobile-tabbar {
    display: none;
  }
}

@media (max-width: 480px) {
  .mobile-tabbar {
    height: 56px;
  }
  
  .tab-item {
    padding: 6px 2px;
  }
  
  .tab-icon {
    width: 20px;
    height: 20px;
    margin-bottom: 3px;
  }
  
  .tab-text {
    font-size: 9px;
  }
  
  .tab-badge {
    top: 6px;
    right: 6px;
    transform: scale(0.7);
  }
}

@media (max-width: 360px) {
  .mobile-tabbar {
    height: 52px;
  }
  
  .tab-item {
    padding: 4px 1px;
  }
  
  .tab-icon {
    width: 18px;
    height: 18px;
    margin-bottom: 2px;
  }
  
  .tab-text {
    font-size: 8px;
  }
}

/* 深色模式适配 */
.dark .mobile-tabbar {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

.dark .tab-item:hover,
.dark .tab-item.active {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

/* 主题适配 */
.blue .mobile-tabbar::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .mobile-tabbar::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .mobile-tabbar::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .mobile-tabbar::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.blue .tab-item.active,
.green .tab-item.active,
.purple .tab-item.active,
.orange .tab-item.active {
  color: var(--brand-color);
}

.blue .tab-indicator,
.green .tab-indicator,
.purple .tab-indicator,
.orange .tab-indicator {
  background-color: var(--brand-color);
}

/* 触摸优化 */
@media (hover: none) and (pointer: coarse) {
  .tab-item {
    min-height: 44px;
  }
  
  .tab-icon {
    margin-bottom: 6px;
  }
  
  .tab-text {
    font-size: 11px;
  }
}

/* 高对比度模式 */
@media (prefers-contrast: high) {
  .tab-indicator {
    height: 4px;
  }
  
  .mobile-tabbar {
    border-top-width: 2px;
  }
  
  .tab-item.active {
    border: 2px solid var(--brand-color);
  }
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .tab-item,
  .tab-icon,
  .tab-indicator {
    transition: none;
  }
}

/* 安全区域适配 */
@supports (padding: max(0px)) {
  .mobile-tabbar {
    padding-bottom: max(env(safe-area-inset-bottom), 0px);
  }
}
</style> 