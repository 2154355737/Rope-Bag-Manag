<template>
  <header class="mobile-header">
    <div class="header-content">
      <!-- 左侧：返回按钮和标题 -->
      <div class="header-left">
        <el-button 
          v-if="showBackButton" 
          type="text" 
          class="back-btn"
          @click="handleBack"
        >
          <el-icon><ArrowLeft /></el-icon>
        </el-button>
        
        <div class="header-title">
          <h1 class="title-text">{{ pageTitle }}</h1>
          <div v-if="pageSubtitle" class="subtitle-text">{{ pageSubtitle }}</div>
        </div>
      </div>
      
      <!-- 右侧：操作按钮 -->
      <div class="header-right">
        <!-- 主题切换器 -->
        <el-dropdown 
          trigger="click" 
          placement="bottom-end"
          :visible="isThemeMenuOpen"
          @visible-change="isThemeMenuOpen = $event"
        >
          <el-button 
            type="text" 
            class="theme-btn"
            :title="`当前主题: ${currentThemeLabel}`"
          >
            <span class="theme-icon">{{ currentThemeIcon }}</span>
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
                  <span class="theme-icon">{{ theme.icon }}</span>
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
          <el-button type="text" class="user-btn">
            <el-avatar :size="28" class="user-avatar">
              <el-icon><User /></el-icon>
            </el-avatar>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu class="user-dropdown">
              <el-dropdown-item @click="handleProfile">
                <el-icon><User /></el-icon>
                个人资料
              </el-dropdown-item>
              <el-dropdown-item @click="handleSettings">
                <el-icon><Setting /></el-icon>
                设置
              </el-dropdown-item>
              <el-dropdown-item divided @click="handleLogout">
                <el-icon><Switch /></el-icon>
                退出登录
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessageBox } from 'element-plus'
import { 
  ArrowLeft, 
  User, 
  Setting, 
  Switch, 
  Check 
} from '@element-plus/icons-vue'
import { useThemeSwitcher, themeConfigs, type ThemeType } from '../../utils/theme'
import { logout } from '../../utils/auth'

// Props
interface Props {
  title?: string
  subtitle?: string
  showBack?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  subtitle: '',
  showBack: false
})

const route = useRoute()
const router = useRouter()

// 主题切换器
const {
  currentTheme,
  isThemeMenuOpen,
  switchTheme,
  themeConfigs: configs
} = useThemeSwitcher()

// 计算属性
const pageTitle = computed(() => {
  return props.title || route.meta?.title || '绳包管理'
})

const pageSubtitle = computed(() => props.subtitle)

const showBackButton = computed(() => {
  return props.showBack || route.path !== '/dashboard'
})

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

// 事件处理
const handleBack = () => {
  if (route.path === '/dashboard') {
    router.push('/dashboard')
  } else {
    router.back()
  }
}

const handleProfile = () => {
  router.push('/profile')
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
.mobile-header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 56px;
  background-color: var(--bg-card);
  border-bottom: 1px solid var(--border-color);
  z-index: 1000;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 100%;
  padding: 0 16px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.back-btn {
  color: var(--text-primary);
  padding: 8px;
  border-radius: 8px;
  transition: all 0.3s ease;
  flex-shrink: 0;
}

.back-btn:hover {
  background-color: var(--bg-primary);
  color: var(--brand-color);
}

.header-title {
  flex: 1;
  min-width: 0;
}

.title-text {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.2;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.subtitle-text {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.theme-btn, .user-btn {
  color: var(--text-primary);
  padding: 8px;
  border-radius: 8px;
  transition: all 0.3s ease;
}

.theme-btn:hover, .user-btn:hover {
  background-color: var(--bg-primary);
  color: var(--brand-color);
}

.theme-icon {
  font-size: 16px;
  line-height: 1;
}

.user-avatar {
  border: 2px solid var(--border-color);
  transition: all 0.3s ease;
}

.user-btn:hover .user-avatar {
  border-color: var(--brand-color);
}

/* 主题下拉菜单 */
.theme-dropdown {
  min-width: 240px;
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
  font-size: 13px;
  line-height: 1.2;
  margin-bottom: 2px;
}

.theme-description {
  font-size: 11px;
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
  min-width: 160px;
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
@media (max-width: 480px) {
  .header-content {
    padding: 0 12px;
  }
  
  .header-left {
    gap: 8px;
  }
  
  .title-text {
    font-size: 14px;
  }
  
  .subtitle-text {
    font-size: 11px;
  }
  
  .header-right {
    gap: 4px;
  }
  
  .theme-btn, .user-btn {
    padding: 6px;
  }
  
  .theme-icon {
    font-size: 14px;
  }
}

@media (max-width: 360px) {
  .header-content {
    padding: 0 8px;
  }
  
  .title-text {
    font-size: 13px;
  }
  
  .subtitle-text {
    font-size: 10px;
  }
}

/* 深色模式适配 */
.dark .mobile-header {
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
.blue .mobile-header::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .mobile-header::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .mobile-header::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .mobile-header::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .mobile-header,
  .back-btn,
  .theme-btn,
  .user-btn,
  .user-avatar {
    transition: none;
  }
}
</style> 