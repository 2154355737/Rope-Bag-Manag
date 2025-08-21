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

// 主题切换功能
const { currentTheme, switchTheme, isThemeMenuOpen } = useThemeSwitcher()

// 用户信息
const userInfo = getUserInfo()

// 动态标题
const dynamicTitle = computed(() => {
  return '绳圈社区管理'
})

// 当前主题信息
const currentThemeIcon = computed(() => {
  return themeConfigs[currentTheme.value]?.icon || '🎨'
})

const currentThemeLabel = computed(() => {
  return themeConfigs[currentTheme.value]?.label || '未知主题'
})

// 可用主题列表
const availableThemes = [
  themeConfigs.light,
  themeConfigs.dark,
  themeConfigs.blue,
  themeConfigs.green,
  themeConfigs.purple,
  themeConfigs.orange,
  themeConfigs.red,
  themeConfigs.auto
]

// 处理个人资料
const handleProfile = () => {
  router.push('/admin/profile')
}

// 处理退出登录
const handleLogout = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要退出登录吗？',
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    
    logout()
    router.push('/auth/login')
  } catch {
    // 用户取消
  }
}
</script>

<style scoped>
.desktop-navbar {
  grid-area: navbar;
  width: 100%;
  height: 72px;
  background: linear-gradient(135deg, 
    var(--bg-elevated) 0%, 
    var(--bg-secondary) 100%);
  border-bottom: 1px solid var(--border-color-light);
  display: flex;
  align-items: center;
  position: relative;
  z-index: 1000;
  transition: var(--transition-normal);
  backdrop-filter: blur(20px);
  box-shadow: var(--shadow-md);
}

.desktop-navbar::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 100%);
  opacity: 0;
  transition: var(--transition-normal);
  pointer-events: none;
}

.desktop-navbar:hover::before {
  opacity: 1;
}

.desktop-navbar::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 5%;
  right: 5%;
  height: 2px;
  background: linear-gradient(90deg, 
    transparent 0%, 
    var(--brand-color) 50%, 
    transparent 100%);
  opacity: 0.6;
}

.navbar-content {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--space-3xl);
  max-width: 100%;
  box-sizing: border-box;
}

.navbar-left {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.navbar-logo {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  cursor: pointer;
  transition: var(--transition-fast);
  background: linear-gradient(135deg, 
    var(--bg-tertiary) 0%, 
    var(--bg-secondary) 100%);
  padding: var(--space-md) var(--space-lg);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-color-light);
  position: relative;
  overflow: hidden;
}

.navbar-logo::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 100%);
  opacity: 0;
  transition: var(--transition-fast);
  pointer-events: none;
}

.navbar-logo:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
  border-color: var(--brand-color);
}

.navbar-logo:hover::before {
  opacity: 1;
}

.navbar-logo .el-icon {
  color: var(--brand-color);
  font-size: 28px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
  transition: var(--transition-fast);
  position: relative;
  z-index: 1;
}

.navbar-logo:hover .el-icon {
  transform: scale(1.1) rotate(5deg);
  filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.2));
}

.navbar-title {
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-bold);
  background: linear-gradient(135deg, var(--text-primary), var(--brand-color));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  white-space: nowrap;
  line-height: var(--line-height-tight);
  position: relative;
  z-index: 1;
  transition: var(--transition-fast);
}

.navbar-logo:hover .navbar-title {
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.navbar-right {
  display: flex;
  align-items: center;
  gap: var(--space-xl);
  flex-shrink: 0;
}

/* 主题切换按钮重新设计 */
.theme-btn {
  display: flex !important;
  align-items: center !important;
  gap: var(--space-sm) !important;
  padding: var(--space-md) var(--space-lg) !important;
  border-radius: var(--radius-xl) !important;
  transition: var(--transition-fast) !important;
  background: linear-gradient(135deg, 
    var(--bg-secondary) 0%, 
    var(--bg-tertiary) 100%) !important;
  border: 1px solid var(--border-color-light) !important;
  color: var(--text-primary) !important;
  font-size: var(--font-size-sm) !important;
  font-weight: var(--font-weight-medium) !important;
  min-height: 44px !important;
  box-sizing: border-box !important;
  position: relative !important;
  overflow: hidden !important;
  backdrop-filter: blur(10px) !important;
  box-shadow: var(--shadow-sm) !important;
}

.theme-btn::before {
  content: '' !important;
  position: absolute !important;
  top: 0 !important;
  left: 0 !important;
  right: 0 !important;
  bottom: 0 !important;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 100%) !important;
  opacity: 0 !important;
  transition: var(--transition-fast) !important;
  pointer-events: none !important;
}

.theme-btn:hover,
.theme-btn:focus {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%) !important;
  border-color: var(--brand-color) !important;
  transform: translateY(-2px) !important;
  box-shadow: var(--shadow-lg) !important;
}

.theme-btn:hover::before,
.theme-btn:focus::before {
  opacity: 1 !important;
}

.theme-icon {
  font-size: var(--font-size-lg) !important;
  line-height: 1 !important;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) !important;
}

.theme-label {
  white-space: nowrap !important;
  font-size: var(--font-size-sm) !important;
  color: inherit !important;
}

/* 主题下拉菜单 */
.theme-dropdown {
  min-width: 280px !important;
  padding: var(--space-lg) !important;
  background: var(--bg-glass-strong) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid var(--border-color) !important;
  border-radius: var(--radius-xl) !important;
  box-shadow: var(--shadow-xl) !important;
}

.theme-option {
  display: flex !important;
  align-items: center !important;
  gap: var(--space-lg) !important;
  padding: var(--space-lg) !important;
  border-radius: var(--radius-md) !important;
  transition: var(--transition-fast) !important;
  cursor: pointer !important;
}

.theme-option:hover {
  background: var(--bg-hover) !important;
}

.theme-option.active {
  background: var(--bg-selected) !important;
  color: var(--text-brand) !important;
}

.theme-option .theme-icon {
  font-size: 1.2rem !important;
  width: 24px !important;
  text-align: center !important;
}

.theme-info {
  flex: 1 !important;
  display: flex !important;
  flex-direction: column !important;
  gap: var(--space-xs) !important;
}

.theme-info .theme-label {
  color: var(--text-primary) !important;
  font-weight: var(--font-weight-medium) !important;
  font-size: var(--font-size-sm) !important;
}

.theme-info .theme-description {
  color: var(--text-tertiary) !important;
  font-size: var(--font-size-xs) !important;
  line-height: var(--line-height-tight) !important;
}

.check-icon {
  color: var(--brand-color) !important;
  font-weight: var(--font-weight-bold) !important;
}

/* 用户信息区域重新设计 */
.user-info {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-md) var(--space-lg);
  border-radius: var(--radius-xl);
  cursor: pointer;
  transition: var(--transition-fast);
  background: linear-gradient(135deg, 
    var(--bg-secondary) 0%, 
    var(--bg-tertiary) 100%);
  border: 1px solid var(--border-color-light);
  min-height: 44px;
  box-sizing: border-box;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(10px);
  box-shadow: var(--shadow-sm);
}

.user-info::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 100%);
  opacity: 0;
  transition: var(--transition-fast);
  pointer-events: none;
}

.user-info:hover {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%);
  border-color: var(--brand-color);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.user-info:hover::before {
  opacity: 1;
}

.user-avatar {
  flex-shrink: 0;
  border: 2px solid var(--border-color-light);
  transition: var(--transition-fast);
  position: relative;
  z-index: 1;
  box-shadow: var(--shadow-sm);
}

.user-info:hover .user-avatar {
  border-color: var(--brand-color);
  transform: scale(1.05);
  box-shadow: var(--shadow-md);
}

.user-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  line-height: var(--line-height-tight);
  position: relative;
  z-index: 1;
  transition: var(--transition-fast);
}

.user-info:hover .user-name {
  color: var(--text-brand);
  transform: translateX(2px);
}

.dropdown-icon {
  color: var(--text-tertiary);
  font-size: var(--font-size-sm);
  transition: var(--transition-fast);
}

.user-info:hover .dropdown-icon {
  color: var(--text-secondary);
}

/* 用户下拉菜单重新设计 */
.user-dropdown {
  min-width: 220px !important;
  padding: var(--space-lg) !important;
  background: linear-gradient(135deg, 
    var(--bg-glass-strong) 0%, 
    var(--bg-elevated) 100%) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid var(--border-color-light) !important;
  border-radius: var(--radius-xl) !important;
  box-shadow: var(--shadow-2xl) !important;
  position: relative !important;
  overflow: hidden !important;
}

.user-dropdown::before {
  content: '' !important;
  position: absolute !important;
  top: 0 !important;
  left: 0 !important;
  right: 0 !important;
  bottom: 0 !important;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 100%) !important;
  opacity: 0.5 !important;
  pointer-events: none !important;
}

.user-dropdown .el-dropdown-menu__item {
  display: flex !important;
  align-items: center !important;
  gap: var(--space-md) !important;
  padding: var(--space-lg) var(--space-xl) !important;
  border-radius: var(--radius-lg) !important;
  color: var(--text-primary) !important;
  font-size: var(--font-size-sm) !important;
  font-weight: var(--font-weight-medium) !important;
  transition: var(--transition-fast) !important;
  margin: var(--space-xs) 0 !important;
  position: relative !important;
  z-index: 1 !important;
  background: transparent !important;
  border: 1px solid transparent !important;
}

.user-dropdown .el-dropdown-menu__item:hover {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-selected) 100%) !important;
  color: var(--text-brand) !important;
  border-color: var(--brand-color) !important;
  transform: translateY(-1px) !important;
  box-shadow: var(--shadow-sm) !important;
}

.user-dropdown .el-dropdown-menu__item .el-icon {
  font-size: var(--font-size-base) !important;
  color: inherit !important;
}

/* 深色主题适配 */
html.dark .desktop-navbar {
  background: linear-gradient(135deg, 
    var(--bg-elevated) 0%, 
    var(--bg-secondary) 100%);
  border-bottom-color: var(--border-color);
}

html.dark .desktop-navbar::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%);
}

html.dark .navbar-logo {
  background: linear-gradient(135deg, 
    var(--bg-tertiary) 0%, 
    var(--bg-secondary) 100%);
}

html.dark .navbar-logo::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%);
}

html.dark .theme-btn {
  background: linear-gradient(135deg, 
    var(--bg-secondary) 0%, 
    var(--bg-tertiary) 100%) !important;
  border-color: var(--border-color) !important;
}

html.dark .theme-btn::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%) !important;
}

html.dark .theme-btn:hover {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%) !important;
  border-color: var(--brand-color) !important;
}

html.dark .user-info {
  background: linear-gradient(135deg, 
    var(--bg-secondary) 0%, 
    var(--bg-tertiary) 100%);
  border-color: var(--border-color);
}

html.dark .user-info::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%);
}

html.dark .user-info:hover {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%);
  border-color: var(--brand-color);
}

html.dark .user-dropdown {
  background: linear-gradient(135deg, 
    var(--bg-glass-strong) 0%, 
    var(--bg-elevated) 100%) !important;
}

html.dark .user-dropdown::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%) !important;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .navbar-content {
    padding: 0 var(--space-2xl);
  }
  
  .navbar-right {
    gap: var(--space-lg);
  }
}

@media (max-width: 1024px) {
  .navbar-content {
    padding: 0 var(--space-xl);
  }
  
  .navbar-title {
    font-size: var(--font-size-lg);
  }
  
  .theme-label {
    display: none !important;
  }
  
  .user-name {
    display: none;
  }
}

@media (max-width: 768px) {
  .desktop-navbar {
    height: 64px;
  }
  
  .navbar-content {
    padding: 0 var(--space-lg);
  }
  
  .navbar-right {
    gap: var(--space-md);
  }
  
  .theme-btn,
  .user-info {
    padding: var(--space-sm) !important;
    min-height: 36px !important;
  }
  
  .navbar-logo .el-icon {
    font-size: 20px;
  }
  
  .navbar-title {
    font-size: var(--font-size-base);
  }
}

@media (max-width: 480px) {
  .navbar-content {
    padding: 0 var(--space-md);
  }
  
  .navbar-logo {
    gap: var(--space-sm);
  }
  
  .navbar-title {
    font-size: var(--font-size-sm);
  }
}

/* 触摸设备优化 */
@media (hover: none) and (pointer: coarse) {
  .theme-btn:hover,
  .user-info:hover,
  .navbar-logo:hover {
    transform: none !important;
  }
  
  .theme-btn:active,
  .user-info:active {
    transform: scale(0.98) !important;
  }
}

/* 减少动画模式支持 */
@media (prefers-reduced-motion: reduce) {
  .desktop-navbar,
  .theme-btn,
  .user-info,
  .navbar-logo,
  .theme-option {
    transition: none !important;
  }
  
  .theme-btn:hover,
  .user-info:hover,
  .navbar-logo:hover {
    transform: none !important;
  }
}

/* 移除所有黑色边框和outline */
* {
  outline: none !important;
}

button,
.el-button,
input,
.el-input,
select,
.el-select,
a,
[tabindex] {
  outline: none !important;
}

button:focus,
.el-button:focus,
.el-button:active,
input:focus,
.el-input:focus,
select:focus,
.el-select:focus,
a:focus,
[tabindex]:focus {
  outline: none !important;
  box-shadow: none !important;
}

/* Element Plus 组件黑边框移除 */
:deep(.el-button) {
  outline: none !important;
}

:deep(.el-button:focus),
:deep(.el-button:active) {
  outline: none !important;
  box-shadow: none !important;
}

:deep(.el-dropdown-menu__item:focus) {
  outline: none !important;
  background: var(--bg-hover) !important;
  transform: translateY(-1px) !important;
  color: var(--text-brand) !important;
}

:deep(.el-avatar:focus) {
  outline: none !important;
}

.theme-btn:focus,
.user-info:focus {
  outline: none !important;
  box-shadow: var(--shadow-lg) !important;
  border-color: var(--brand-color) !important;
  transform: translateY(-2px) !important;
}

.navbar-logo:focus {
  outline: none !important;
  transform: translateY(-2px) !important;
  box-shadow: var(--shadow-lg) !important;
  border-color: var(--brand-color) !important;
}

/* 主题选项focus状态 */
.theme-option:focus {
  outline: none !important;
  background: var(--bg-hover) !important;
  transform: translateX(4px) !important;
}

/* 键盘导航可访问性 */
*:focus-visible {
  outline: 2px solid var(--brand-color) !important;
  outline-offset: 2px !important;
  border-radius: var(--radius-sm) !important;
}

/* 但对于我们自定义的交互元素，移除这个outline，使用自定义样式 */
.navbar-logo:focus-visible,
.theme-btn:focus-visible,
.user-info:focus-visible,
.theme-option:focus-visible,
:deep(.el-button:focus-visible),
:deep(.el-dropdown-menu__item:focus-visible),
:deep(.el-avatar:focus-visible) {
  outline: none !important;
}

/* 高对比度模式支持 */
@media (prefers-contrast: high) {
  .desktop-navbar {
    border-bottom-width: 2px;
  }
  
  .theme-btn,
  .user-info {
    border-width: 2px !important;
  }
  
  .theme-btn:hover,
  .theme-btn:focus,
  .user-info:hover {
    border-color: var(--brand-color) !important;
  }
}
</style> 