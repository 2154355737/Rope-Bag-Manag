<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Sunny, Moon, Fold, House, User, Box, Document, DataAnalysis, ArrowDown, ZoomIn, ZoomOut, Refresh } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getUserInfo, clearLoginStatus } from './utils/auth'

const route = useRoute()
const router = useRouter()
const activeMenu = computed(() => route.path)
const isLoginPage = computed(() => route.path === '/login')
const sidebarOpen = ref(false)

// 缩放控制
const scaleFactor = ref(1)
const minScale = 0.8
const maxScale = 1.4

// 用户信息
const userInfo = ref({
  username: '管理员',
  loginTime: ''
})

function toggleSidebar() {
  sidebarOpen.value = !sidebarOpen.value
}

// 缩放控制函数
function zoomIn() {
  if (scaleFactor.value < maxScale) {
    scaleFactor.value = Math.min(scaleFactor.value + 0.1, maxScale)
    updateScale()
  }
}

function zoomOut() {
  if (scaleFactor.value > minScale) {
    scaleFactor.value = Math.max(scaleFactor.value - 0.1, minScale)
    updateScale()
  }
}

function resetZoom() {
  scaleFactor.value = 1
  updateScale()
}

function updateScale() {
  document.documentElement.style.setProperty('--scale-factor', scaleFactor.value.toString())
  localStorage.setItem('scale-factor', scaleFactor.value.toString())
}

// 退出登录
const handleLogout = async () => {
  try {
    await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    // 清除登录状态
    clearLoginStatus()
    
    ElMessage.success('已退出登录')
    router.push('/login')
  } catch {
    // 用户取消
  }
}

// 让body、html、#app全高
onMounted(() => {
  document.documentElement.style.height = '100%'
  document.body.style.height = '100%'
  const app = document.getElementById('app')
  if (app) app.style.height = '100%'
  
  // 初始化缩放
  const savedScale = localStorage.getItem('scale-factor')
  if (savedScale) {
    scaleFactor.value = parseFloat(savedScale)
    updateScale()
  }
  
  // 初始化暗色模式
  const dark = localStorage.getItem('theme-dark') === '1'
  isDark.value = dark
  setDarkClass(dark)
  
  // 获取用户信息
  const savedUserInfo = getUserInfo()
  if (savedUserInfo) {
    userInfo.value = savedUserInfo
  }
})

const isDark = ref(false)
function toggleDark(val: boolean) {
  setDarkClass(val)
  localStorage.setItem('theme-dark', val ? '1' : '0')
}
function setDarkClass(val: boolean) {
  if (val) {
    document.body.classList.add('dark')
    document.documentElement.classList.add('dark')
  } else {
    document.body.classList.remove('dark')
    document.documentElement.classList.remove('dark')
  }
}
</script>

<template>
  <div class="layout-root">
    <!-- 顶部导航栏 - 登录页不显示 -->
    <el-header v-if="!isLoginPage" class="top-nav">
      <div class="nav-left">
        <el-button class="menu-btn" circle @click="toggleSidebar">
          <el-icon>
            <Fold />
          </el-icon>
        </el-button>
        <span class="brand">绳包管理系统</span>
      </div>
      <div class="nav-right">
        <!-- 缩放控制 -->
        <div class="zoom-controls">
          <el-button size="small" circle @click="zoomOut" :disabled="scaleFactor <= minScale">
            <el-icon><ZoomOut /></el-icon>
          </el-button>
          <span class="zoom-text">{{ Math.round(scaleFactor * 100) }}%</span>
          <el-button size="small" circle @click="zoomIn" :disabled="scaleFactor >= maxScale">
            <el-icon><ZoomIn /></el-icon>
          </el-button>
          <el-button size="small" circle @click="resetZoom" :disabled="scaleFactor === 1">
            <el-icon><Refresh /></el-icon>
          </el-button>
        </div>
        
        <el-switch
          v-model="isDark"
          active-color="#222"
          inactive-color="#f6f8fa"
          :active-icon="Moon"
          :inactive-icon="Sunny"
          style="margin-right: 18px;"
          @change="toggleDark"
        />
        <el-avatar :size="36" src="https://api.dicebear.com/7.x/miniavs/svg?seed=admin" style="margin-right: 8px;" />
        <el-dropdown>
          <span class="el-dropdown-link">
            {{ userInfo.username }} 
            <el-icon>
              <ArrowDown />
            </el-icon>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item>个人中心</el-dropdown-item>
              <el-dropdown-item @click="handleLogout">退出登录</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </el-header>
    <!-- 侧边栏 --->
    <el-drawer v-if="!isLoginPage" v-model="sidebarOpen" direction="ltr" :with-header="false" size="180px" class="side-drawer" :show-close="true" :modal="true" :append-to-body="true">
              <el-menu :default-active="activeMenu" router background-color="#fff" text-color="#333" active-text-color="#42b983" class="side-menu">
          <el-menu-item index="/dashboard">
            <el-icon>
              <House />
            </el-icon>
            仪表盘
          </el-menu-item>
          <el-menu-item index="/users">
            <el-icon>
              <User />
            </el-icon>
            用户管理
          </el-menu-item>
          <el-menu-item index="/packages">
            <el-icon>
              <Box />
            </el-icon>
            绳包管理
          </el-menu-item>
          <el-menu-item index="/logs">
            <el-icon>
              <Document />
            </el-icon>
            日志查看
          </el-menu-item>
          <el-menu-item index="/stats">
            <el-icon>
              <DataAnalysis />
            </el-icon>
            统计信息
          </el-menu-item>
        </el-menu>
    </el-drawer>
    <!-- 统一背景和圆角的主布局容器 -->
    <div v-if="!isLoginPage" class="main-layout">
      <el-container class="full-height">
        <el-aside class="side-aside" width="180px">
          <el-menu :default-active="activeMenu" router background-color="#fff" text-color="#333" active-text-color="#42b983" class="side-menu side-menu-static">
            <el-menu-item index="/dashboard">
              <el-icon>
                <House />
              </el-icon>
              仪表盘
            </el-menu-item>
            <el-menu-item index="/users">
              <el-icon>
                <User />
              </el-icon>
              用户管理
            </el-menu-item>
            <el-menu-item index="/packages">
              <el-icon>
                <Box />
              </el-icon>
              绳包管理
            </el-menu-item>
            <el-menu-item index="/logs">
              <el-icon>
                <Document />
              </el-icon>
              日志查看
            </el-menu-item>
            <el-menu-item index="/stats">
              <el-icon>
                <DataAnalysis />
              </el-icon>
              统计信息
            </el-menu-item>
          </el-menu>
        </el-aside>
        <el-main class="main-content">
          <div class="content-scroll">
            <router-view />
          </div>
        </el-main>
      </el-container>
    </div>
    <!-- 登录页独立显示 -->
    <router-view v-if="isLoginPage" />
  </div>
</template>

<style scoped>
.layout-root {
  min-height: 100vh;
  height: 100vh;
  width: 100vw;
  min-width: 100vw;
  background: var(--bg-primary);
  display: flex;
  flex-direction: column;
  transform: scale(var(--scale-factor, 1));
  transform-origin: top left;
}

.zoom-controls {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-right: 18px;
}

.zoom-text {
  font-size: 12px;
  color: var(--text-secondary);
  min-width: 40px;
  text-align: center;
}

.main-layout {
  background: var(--bg-primary);
  border-radius: 18px;
  box-shadow: 0 2px 16px var(--shadow-color);
  padding: 24px;
  margin: 10px;
  display: flex;
  min-height: 0;
  height: calc(100vh - 84px);
}

.top-nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 60px;
  background: var(--bg-nav);
  box-shadow: 0 2px 8px var(--shadow-color);
  border-radius: 18px;
  padding: 0 32px;
  position: sticky;
  top: 0;
  z-index: 100;
  transition: box-shadow 0.3s;
  margin: 10px;
}

.nav-left {
  display: flex;
  align-items: center;
}

.menu-btn {
  margin-right: 18px;
  display: none;
}

.brand {
  font-size: 20px;
  font-weight: bold;
  color: var(--brand-color);
}

.nav-right {
  display: flex;
  align-items: center;
}

.side-drawer {
  display: none;
}

.side-aside {
  background: var(--bg-sidebar);
  min-height: 0;
  height: 100%;
  box-shadow: 2px 0 8px var(--shadow-color);
  border-radius: 18px;
  transition: box-shadow 0.3s;
  display: flex;
  flex-direction: column;
  padding-bottom: 12px;
}

.side-menu {
  border-right: none;
  min-height: 0;
  height: 100%;
  font-size: 16px;
  flex: 1;
}

.side-menu-static {
  display: block;
}

.main-content {
  padding: 0;
  min-height: 0;
  height: 100%;
  background: var(--bg-primary);
  border-radius: 18px;
  transition: background 0.3s;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.content-scroll {
  padding: 32px 32px 0 32px;
  min-height: 0;
  height: 100%;
  overflow-x: auto;
  overflow-y: auto;
  box-sizing: border-box;
}

@media (max-width: 1200px) {
  .main-layout {
    padding: 8px;
    margin: 4px;
  }
  .top-nav {
    margin: 4px;
  }
  .content-scroll {
    padding: 18px 8px 0 8px;
  }
  .zoom-controls {
    margin-right: 12px;
  }
}

@media (max-width: 900px) {
  .main-layout {
    padding: 2px;
    margin: 2px;
  }
  .top-nav {
    margin: 2px;
  }
  .content-scroll {
    padding: 12px 2px 0 2px;
  }
  .zoom-controls {
    margin-right: 8px;
  }
  .zoom-text {
    display: none;
  }
}

@media (max-width: 600px) {
  .main-layout {
    padding: 0;
    margin: 0;
    border-radius: 0;
    box-shadow: none;
  }
  .top-nav {
    margin: 0;
  }
  .content-scroll {
    padding: 6px 0 0 0;
  }
  .zoom-controls {
    display: none;
  }
}
</style>
