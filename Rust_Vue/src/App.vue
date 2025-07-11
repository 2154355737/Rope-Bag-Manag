<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Sunny, Moon, Fold, House, User, Box, Document, DataAnalysis, ArrowDown, ZoomIn, ZoomOut, Refresh, Brush } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getUserInfo, clearLoginStatus } from './utils/auth'
import { onScreenSizeChange, shouldUseMobileVersion } from './utils/device'

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

// 屏幕尺寸变化监听
let screenSizeCleanup: (() => void) | null = null

function toggleSidebar() {
  sidebarOpen.value = !sidebarOpen.value
}

// 自动切换到合适的用户管理页面
function autoSwitchUserManagePage() {
  const currentPath = route.path
  if (currentPath === '/users' || currentPath === '/users-mobile') {
    const shouldUseMobile = shouldUseMobileVersion()
    const targetPath = shouldUseMobile ? '/users-mobile' : '/users'
    
    if (currentPath !== targetPath) {
      router.push(targetPath)
    }
  }
}

// 导航到用户管理页面（自动选择合适版本）
function navigateToUserManage() {
  const shouldUseMobile = shouldUseMobileVersion()
  const targetPath = shouldUseMobile ? '/users-mobile' : '/users'
  router.push(targetPath)
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
  
  // 初始化屏幕尺寸监听
  screenSizeCleanup = onScreenSizeChange((deviceType) => {
    // 当设备类型改变时，自动切换到合适的页面
    autoSwitchUserManagePage()
  })
})

// 清理屏幕尺寸监听
onUnmounted(() => {
  if (screenSizeCleanup) {
    screenSizeCleanup()
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
    document.body.classList.remove('light')
    document.documentElement.classList.remove('light')
  } else {
    document.body.classList.remove('dark')
    document.documentElement.classList.remove('dark')
    document.body.classList.add('light')
    document.documentElement.classList.add('light')
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
        <!-- 缩放控制 - 桌面端显示 -->
        <div class="zoom-controls desktop-only">
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
    
    <!-- 移动端底部导航 -->
    <div v-if="!isLoginPage" class="mobile-nav">
      <div class="mobile-nav-item" :class="{ active: activeMenu === '/dashboard' }" @click="router.push('/dashboard')">
        <el-icon><House /></el-icon>
        <span>仪表盘</span>
      </div>
      <div class="mobile-nav-item" :class="{ active: activeMenu === '/users' || activeMenu === '/users-mobile' }" @click="navigateToUserManage">
        <el-icon><User /></el-icon>
        <span>用户</span>
      </div>
      <div class="mobile-nav-item" :class="{ active: activeMenu === '/packages' }" @click="router.push('/packages')">
        <el-icon><Box /></el-icon>
        <span>绳包</span>
      </div>
      <div class="mobile-nav-item" :class="{ active: activeMenu === '/logs' }" @click="router.push('/logs')">
        <el-icon><Document /></el-icon>
        <span>日志</span>
      </div>
      <div class="mobile-nav-item" :class="{ active: activeMenu === '/stats' }" @click="router.push('/stats')">
        <el-icon><DataAnalysis /></el-icon>
        <span>统计</span>
      </div>
    </div>
    
    <!-- 侧边栏抽屉 -->
    <el-drawer v-if="!isLoginPage" v-model="sidebarOpen" direction="ltr" :with-header="false" size="280px" class="side-drawer" :show-close="true" :modal="true" :append-to-body="true">
      <div class="drawer-header">
        <h3>绳包管理系统</h3>
        <p>管理员控制台</p>
      </div>
      <el-menu :default-active="activeMenu" router background-color="transparent" text-color="var(--text-primary)" active-text-color="var(--brand-color)" class="side-menu">
        <el-menu-item index="/dashboard">
          <el-icon>
            <House />
          </el-icon>
          <span>仪表盘</span>
        </el-menu-item>
        <el-menu-item index="/users" @click="navigateToUserManage">
          <el-icon>
            <User />
          </el-icon>
          <span>用户管理</span>
        </el-menu-item>
        <el-menu-item index="/packages">
          <el-icon>
            <Box />
          </el-icon>
          <span>绳包管理</span>
        </el-menu-item>
        <el-menu-item index="/logs">
          <el-icon>
            <Document />
          </el-icon>
          <span>日志查看</span>
        </el-menu-item>
        <el-menu-item index="/stats">
          <el-icon>
            <DataAnalysis />
          </el-icon>
          <span>统计信息</span>
        </el-menu-item>
        <el-menu-item index="/theme-test">
          <el-icon>
            <Brush />
          </el-icon>
          <span>主题测试</span>
        </el-menu-item>
      </el-menu>
    </el-drawer>
    
    <!-- 主内容区 -->
    <div v-if="!isLoginPage" class="main-layout">
      <el-container class="full-height">
        <!-- 桌面端侧边栏 -->
        <el-aside class="side-aside desktop-only" width="180px">
          <el-menu :default-active="activeMenu" router background-color="var(--bg-sidebar)" text-color="var(--text-primary)" active-text-color="var(--brand-color)" class="side-menu side-menu-static">
            <el-menu-item index="/dashboard">
              <el-icon>
                <House />
              </el-icon>
              仪表盘
            </el-menu-item>
            <el-menu-item index="/users" @click="navigateToUserManage">
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
            <el-menu-item index="/theme-test">
              <el-icon>
                <Brush />
              </el-icon>
              主题测试
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

/* 移动端底部导航 */
.mobile-nav {
  display: none;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background: var(--bg-nav);
  border-top: 1px solid var(--border-color);
  z-index: 1000;
  padding: 0.5rem 0;
}

.mobile-nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 0.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  color: var(--text-secondary);
  font-size: 0.75rem;
}

.mobile-nav-item.active {
  color: var(--brand-color);
}

.mobile-nav-item .el-icon {
  font-size: 1.25rem;
  margin-bottom: 0.25rem;
}

/* 抽屉样式 */
.drawer-header {
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
  text-align: center;
}

.drawer-header h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
  font-size: 1.25rem;
}

.drawer-header p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
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

/* 桌面端样式 */
@media (min-width: 769px) {
  .desktop-only {
    display: block;
  }
  
  .mobile-nav {
    display: none;
  }
  
  .menu-btn {
    display: none;
  }
}

/* 平板端样式 */
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

/* 手机端样式 */
@media (max-width: 768px) {
  .layout-root {
    transform: none;
  }
  
  .desktop-only {
    display: none !important;
  }
  
  .mobile-nav {
    display: flex;
    justify-content: space-around;
  }
  
  .menu-btn {
    display: inline-flex;
  }
  
  .main-layout {
    padding: 0;
    margin: 0;
    border-radius: 0;
    box-shadow: none;
    height: calc(100vh - 120px);
  }
  
  .top-nav {
    margin: 0;
    border-radius: 0;
    padding: 0 1rem;
    height: 60px;
  }
  
  .brand {
    font-size: 1rem;
  }
  
  .content-scroll {
    padding: 1rem;
    height: calc(100vh - 180px);
  }
  
  .side-drawer {
    display: block !important;
  }
  
  .side-menu {
    background: transparent;
  }
  
  .side-menu .el-menu-item {
    height: 3rem;
    line-height: 3rem;
    font-size: 1rem;
  }
  
  .side-menu .el-icon {
    font-size: 1.25rem;
  }
}

@media (max-width: 600px) {
  .top-nav {
    padding: 0 0.5rem;
  }
  
  .content-scroll {
    padding: 0.5rem;
  }
  
  .mobile-nav-item {
    padding: 0.25rem;
    font-size: 0.625rem;
  }
  
  .mobile-nav-item .el-icon {
    font-size: 1rem;
  }
}
</style>
