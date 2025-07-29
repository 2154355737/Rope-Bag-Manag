<template>
  <nav class="desktop-navbar">
    <div class="navbar-content">
      <!-- 左侧：Logo和标题 -->
      <div class="navbar-left">
        <div class="navbar-logo">
          <el-icon :size="24">
            <Box />
          </el-icon>
          <span class="navbar-title">{{ dynamicTitle }}</span>
        </div>
      </div>
      
      <!-- 右侧：用户信息和主题切换 -->
      <div class="navbar-right">
        <!-- 主题切换器 -->
        <el-dropdown 
          trigger="click" 
          placement="bottom-end"
          :visible="isThemeMenuOpen"
          @visible-change="isThemeMenuOpen = $event"
        >
          <el-button 
            link
            class="theme-btn"
            :title="`当前主题: ${currentThemeLabel}`"
          >
            <span class="theme-icon" v-text="currentThemeIcon"></span>
            <span class="theme-label">{{ currentThemeLabel }}</span>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu class="theme-dropdown">
              <el-dropdown-item 
                v-for="theme in availableThemes" 
                :key="theme.name"
                @click="switchTheme(theme.name as ThemeType)"
                :class="{ active: currentTheme === theme.name }"
              >
                <div class="theme-option">
                  <span class="theme-icon" v-text="theme.icon"></span>
                  <div class="theme-info">
                    <div class="theme-label">{{ theme.label }}</div>
                    <div class="theme-description">{{ theme.description }}</div>
                  </div>
                  <el-icon v-if="currentTheme === theme.name" class="check-icon">
                    <Check />
                  </el-icon>
                </div>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
        
        <!-- 用户菜单 -->
        <el-dropdown trigger="click" placement="bottom-end">
          <div class="user-info">
            <el-avatar :size="32" class="user-avatar" :src="userInfo.avatar_url" v-if="userInfo.avatar_url">
              <img :src="userInfo.avatar_url" />
            </el-avatar>
            <el-avatar :size="32" class="user-avatar" v-else>
              <el-icon><User /></el-icon>
            </el-avatar>
            <span class="user-name">{{ userInfo.username }}</span>
            <el-icon class="dropdown-icon"><Bottom /></el-icon>
          </div>
          <template #dropdown>
            <el-dropdown-menu class="user-dropdown">
              <el-dropdown-item @click="handleProfile">
                <el-icon><User /></el-icon>
                个人资料
              </el-dropdown-item>
              <!-- 系统设置菜单项已移除 -->
              <el-dropdown-item divided @click="handleLogout">
                <el-icon><Switch /></el-icon>
                退出登录
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessageBox } from 'element-plus'
import { 
  Box, 
  User, 
  Setting, 
  Switch, 
  Check
} from '@element-plus/icons-vue'
import { useThemeSwitcher, themeConfigs, type ThemeType } from '../../utils/theme'
import { logout, getUserInfo } from '../../utils/auth'

const router = useRouter()

// 主题切换器
const {
  currentTheme,
  isThemeMenuOpen,
  switchTheme,
  themeConfigs: configs
} = useThemeSwitcher()

// 用户信息
const userInfo = computed(() => {
  return getUserInfo() || { username: '用户' }
})

// 计算属性
const currentThemeIcon = computed(() => {
  const theme = configs[currentTheme.value]
  return theme?.icon || '🎨'
})

const currentThemeLabel = computed(() => {
  const theme = configs[currentTheme.value]
  return theme?.label || '主题'
})

const availableThemes = computed(() => {
  return Object.values(configs)
})

const dynamicTitle = computed(() => {
  const user = getUserInfo()
  if (user?.role === 'admin') return '管理员后台'
  if (user?.role === 'elder') return '元老后台'
  if (user?.role === 'user') return '用户中心'
  return '绳包管理系统'
})

// 事件处理
const handleProfile = () => {
  const user = getUserInfo()
  if (user?.role === 'elder') {
    router.push('/elder/profile')
  } else if (user?.role === 'user') {
    router.push('/user/profile')
  } else if (user?.role === 'admin') {
    router.push('/admin/users')
  } else {
    router.push('/login')
  }
}

const handleSettings = () => {
  router.push('/settings')
}

const handleLogout = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要退出登录吗？',
      '确认退出',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    logout()
    router.push('/login')
  } catch (error) {
    // 用户取消
  }
}
</script>

<style scoped>
.desktop-navbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 64px;
  background-color: var(--bg-card);
  border-bottom: 1px solid var(--border-color);
  z-index: 1000;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.navbar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 100%;
  padding: 0 24px;
}

.navbar-left {
  display: flex;
  align-items: center;
}

.navbar-logo {
  display: flex;
  align-items: center;
  gap: 12px;
}

.navbar-logo .el-icon {
  color: var(--brand-color);
}

.navbar-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.navbar-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.theme-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-primary);
  padding: 8px 12px;
  border-radius: 8px;
  transition: all 0.3s ease;
}

.theme-btn:hover {
  background-color: var(--bg-primary);
  color: var(--brand-color);
}

.theme-icon {
  font-size: 16px;
  line-height: 1;
}

.theme-label {
  font-size: 14px;
  font-weight: 500;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.user-info:hover {
  background-color: var(--bg-primary);
}

.user-avatar {
  border: 2px solid var(--border-color);
  transition: all 0.3s ease;
}

.user-info:hover .user-avatar {
  border-color: var(--brand-color);
}

.user-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.dropdown-icon {
  font-size: 12px;
  color: var(--text-secondary);
  transition: transform 0.3s ease;
}

.user-info:hover .dropdown-icon {
  transform: rotate(180deg);
}

/* 主题下拉菜单 */
.theme-dropdown {
  min-width: 280px;
  padding: 8px 0;
}

.theme-dropdown .el-dropdown-menu__item {
  padding: 12px 16px;
  border-radius: 8px;
  margin: 4px 8px;
  transition: all 0.2s ease;
}

.theme-dropdown .el-dropdown-menu__item:hover {
  background-color: var(--bg-primary);
}

.theme-dropdown .el-dropdown-menu__item.active {
  background-color: var(--brand-color);
  color: #ffffff;
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
}

.theme-info {
  flex: 1;
  min-width: 0;
}

.theme-label {
  font-weight: 500;
  font-size: 14px;
  line-height: 1.2;
  margin-bottom: 2px;
}

.theme-description {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.2;
}

.active .theme-description {
  color: rgba(255, 255, 255, 0.8);
}

.check-icon {
  font-size: 14px;
  color: #ffffff;
}

/* 用户下拉菜单 */
.user-dropdown {
  min-width: 180px;
  padding: 8px 0;
}

.user-dropdown .el-dropdown-menu__item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-radius: 8px;
  margin: 4px 8px;
  transition: all 0.2s ease;
}

.user-dropdown .el-dropdown-menu__item:hover {
  background-color: var(--bg-primary);
}

.user-dropdown .el-dropdown-menu__item.divided {
  border-top: 1px solid var(--border-color);
  margin-top: 8px;
  padding-top: 16px;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .navbar-content {
    padding: 0 20px;
  }
  
  .navbar-right {
    gap: 12px;
  }
}

@media (max-width: 768px) {
  .desktop-navbar {
    display: none;
  }
}

/* 深色模式适配 */
.dark .desktop-navbar {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

.dark .theme-dropdown,
.dark .user-dropdown {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}

/* 主题适配 */
.blue .desktop-navbar::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .desktop-navbar::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .desktop-navbar::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .desktop-navbar::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

/* 动画效果 */
@keyframes slide-down {
  0% {
    opacity: 0;
    transform: translateY(-20px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 页面加载动画 */
.desktop-navbar {
  animation: slide-down 0.6s ease-out;
}

/* 按钮悬停动画 */
.theme-btn,
.user-info {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.theme-btn::before,
.user-info::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transition: left 0.5s ease;
}

.theme-btn:hover::before,
.user-info:hover::before {
  left: 100%;
}

/* 用户头像动画 */
.user-avatar {
  transition: all 0.3s ease;
}

.user-info:hover .user-avatar {
  transform: scale(1.1);
}

/* 下拉图标动画 */
.dropdown-icon {
  transition: transform 0.3s ease;
}

.user-info:hover .dropdown-icon {
  transform: rotate(180deg);
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .desktop-navbar {
    animation: none;
  }
  
  .theme-btn,
  .user-info,
  .dropdown-icon {
    transition: none;
  }
  
  .theme-btn::before,
  .user-info::before {
    display: none;
  }
  
  .user-info:hover .user-avatar {
    transform: none;
  }
  
  .user-info:hover .dropdown-icon {
    transform: none;
  }
}
</style> 