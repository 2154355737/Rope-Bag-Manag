<template>
  <div class="mobile-layout">
    <!-- 顶部导航栏 -->
    <div class="mobile-header" v-if="showHeader">
      <div class="header-left">
        <span v-if="showBack" class="back-btn" @click="goBack">
          <i class="el-icon-arrow-left"></i>
        </span>
        <slot name="header-left"></slot>
      </div>
      <div class="header-title">
        {{ title }}
        <slot name="header-title"></slot>
      </div>
      <div class="header-right">
        <slot name="header-right"></slot>
      </div>
    </div>

    <!-- 内容区域 -->
    <div class="mobile-content" :class="{'has-footer': showFooter, 'has-header': showHeader}">
      <slot></slot>
    </div>

    <!-- 底部导航栏 -->
    <div class="mobile-footer" v-if="showFooter">
      <router-link to="/mobile/home" class="footer-item" :class="{active: activeTab === 'home'}">
        <i class="el-icon-house"></i>
        <span>首页</span>
      </router-link>
      <router-link to="/mobile/resources" class="footer-item" :class="{active: activeTab === 'resources'}">
        <i class="el-icon-folder"></i>
        <span>资源</span>
      </router-link>
      <router-link to="/mobile/posts" class="footer-item" :class="{active: activeTab === 'posts'}">
        <i class="el-icon-discover"></i>
        <span>发现</span>
      </router-link>
      <router-link to="/mobile/messages" class="footer-item" :class="{active: activeTab === 'messages'}">
        <i class="el-icon-bell"></i>
        <span>消息</span>
      </router-link>
      <router-link to="/mobile/user" class="footer-item" :class="{active: activeTab === 'user'}">
        <i class="el-icon-user"></i>
        <span>我的</span>
      </router-link>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const props = defineProps({
  title: {
    type: String,
    default: ''
  },
  showBack: {
    type: Boolean,
    default: true
  },
  showHeader: {
    type: Boolean,
    default: true
  },
  showFooter: {
    type: Boolean,
    default: true
  }
})

const router = useRouter()
const route = useRoute()

const activeTab = computed(() => {
  const path = route.path
  if (path.includes('/mobile/home')) return 'home'
  if (path.includes('/mobile/resources')) return 'resources'
  if (path.includes('/mobile/posts')) return 'posts'
  if (path.includes('/mobile/messages')) return 'messages'
  if (path.includes('/mobile/user')) return 'user'
  return ''
})

const goBack = () => {
  router.back()
}
</script>

<style scoped>
.mobile-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--el-bg-color);
}

.mobile-header {
  display: flex;
  align-items: center;
  height: 50px;
  padding: 0 15px;
  background-color: var(--el-color-primary);
  color: #fff;
  position: sticky;
  top: 0;
  z-index: 100;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.header-left, .header-right {
  flex: 0 0 60px;
  display: flex;
  align-items: center;
}

.header-left {
  justify-content: flex-start;
}

.header-right {
  justify-content: flex-end;
}

.header-title {
  flex: 1;
  text-align: center;
  font-size: 18px;
  font-weight: 500;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.back-btn {
  font-size: 20px;
  cursor: pointer;
  padding: 8px;
  margin-left: -8px;
}

.mobile-content {
  flex: 1;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
  padding: 15px;
}

.mobile-content.has-footer {
  padding-bottom: 65px;
}

.mobile-footer {
  display: flex;
  justify-content: space-around;
  align-items: center;
  height: 50px;
  background-color: #fff;
  border-top: 1px solid #e4e7ed;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 100;
  padding-bottom: env(safe-area-inset-bottom, 0);
}

.footer-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--el-text-color-regular);
  text-decoration: none;
  font-size: 12px;
  padding: 5px 0;
  flex: 1;
}

.footer-item i {
  font-size: 20px;
  margin-bottom: 2px;
}

.footer-item.active {
  color: var(--el-color-primary);
}

@media (prefers-color-scheme: dark) {
  .mobile-footer {
    background-color: var(--el-bg-color);
    border-top-color: var(--el-border-color);
  }
}
</style> 