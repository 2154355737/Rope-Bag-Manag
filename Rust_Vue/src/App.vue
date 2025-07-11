<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Sunny, Moon, Fold, House, User, Box, Document, DataAnalysis, ArrowDown } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getUserInfo, clearLoginStatus } from './utils/auth'

const route = useRoute()
const router = useRouter()
const activeMenu = computed(() => route.path)
const isLoginPage = computed(() => route.path === '/login')
const sidebarOpen = ref(false)

// 用户信息
const userInfo = ref({
  username: '管理员',
  loginTime: ''
})

function toggleSidebar() {
  sidebarOpen.value = !sidebarOpen.value
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
    <!-- 主内容区 -->
    <div v-if="!isLoginPage" class="main-content-wrap">
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
.main-content-wrap {
  margin-top: 12px;
  flex: 1;
  min-height: 0;
  height: calc(100vh - 60px);
  display: flex;
}
.full-height {
  height: 100% !important;
  min-height: 0 !important;
  flex: 1;
}
.el-container {
  height: 100%;
  min-height: 0;
  flex: 1;
}
.side-aside {
  background: var(--bg-sidebar);
  min-height: 0;
  height: 100%;
  box-shadow: 2px 0 8px var(--shadow-color);
  border-radius: 0 18px 18px 0;
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
  .content-scroll {
    padding: 18px 8px 0 8px;
  }
}
@media (max-width: 900px) {
  .side-aside {
    display: none;
  }
  .side-drawer {
    display: block !important;
  }
  .menu-btn {
    display: inline-flex;
  }
  .main-content {
    padding: 0;
    border-radius: 12px;
  }
  .content-scroll {
    padding: 12px 2px 0 2px;
  }
}
@media (max-width: 600px) {
  .top-nav {
    padding: 0 8px;
    border-radius: 0 0 10px 10px;
    font-size: 15px;
  }
  .brand {
    font-size: 16px;
  }
  .content-scroll {
    padding: 6px 0 0 0;
  }
}
</style>
