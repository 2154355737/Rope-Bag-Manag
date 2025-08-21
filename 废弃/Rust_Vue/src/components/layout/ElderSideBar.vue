<template>
  <aside class="desktop-sidebar">
    <div class="sidebar-content">
      <nav class="sidebar-nav">
        <router-link to="/elder/profile" class="nav-item" active-class="active" title="个人信息">
          <div class="nav-icon"><el-icon :size="20"><User /></el-icon></div>
          <span class="nav-text">个人信息</span>
        </router-link>
        <router-link to="/elder/comments" class="nav-item" active-class="active" title="我的评论">
          <div class="nav-icon"><el-icon :size="20"><ChatDotRound /></el-icon></div>
          <span class="nav-text">我的评论</span>
        </router-link>
        <router-link to="/elder/resources" class="nav-item" active-class="active" title="资源管理">
          <div class="nav-icon"><el-icon :size="20"><Box /></el-icon></div>
          <span class="nav-text">资源管理</span>
        </router-link>
        <router-link to="/elder/my-resources" class="nav-item" active-class="active" title="我的资源">
          <div class="nav-icon"><el-icon :size="20"><Files /></el-icon></div>
          <span class="nav-text">我的资源</span>
        </router-link>
      </nav>
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
import { User, ChatDotRound, Box, Files } from '@element-plus/icons-vue'
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
</style> 