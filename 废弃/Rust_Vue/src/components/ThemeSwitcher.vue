<template>
  <div class="theme-switcher">
    <el-dropdown @command="handleThemeChange" trigger="click" placement="bottom-end">
      <el-button class="theme-btn" circle>
        <span class="theme-icon" v-text="currentThemeIcon"></span>
      </el-button>
      
      <template #dropdown>
        <el-dropdown-menu class="theme-menu">
          <el-dropdown-item 
            v-for="theme in availableThemes" 
            :key="theme.name"
            :command="theme.name"
            :class="{ active: currentTheme === theme.name }"
          >
            <div class="theme-item">
              <span class="theme-icon" v-text="theme.icon"></span>
              <div class="theme-info">
                <span class="theme-label">{{ theme.label }}</span>
                <span class="theme-desc">{{ theme.description }}</span>
              </div>
              <el-icon v-if="currentTheme === theme.name" class="check-icon">
                <Check />
              </el-icon>
            </div>
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Setting, Check } from '@element-plus/icons-vue'
import { useThemeSwitcher, themeConfigs } from '@/utils/theme'

const { currentTheme, switchTheme } = useThemeSwitcher()

// 可用的主题列表
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

// 当前主题图标
const currentThemeIcon = computed(() => {
  return themeConfigs[currentTheme.value]?.icon || '🎨'
})

// 处理主题切换
const handleThemeChange = (theme: string) => {
  switchTheme(theme as any)
}
</script>

<style scoped>
.theme-switcher {
  position: relative;
}

.theme-btn {
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-dark)) !important;
  backdrop-filter: blur(12px) !important;
  border: none !important;
  color: var(--text-inverse) !important;
  transition: var(--transition-normal) !important;
  box-shadow: var(--shadow-md) !important;
  position: relative !important;
  overflow: hidden !important;
  width: 40px !important;
  height: 40px !important;
}

.theme-btn:hover,
.theme-btn:focus {
  background: linear-gradient(135deg, var(--brand-color-light), var(--brand-color)) !important;
  transform: translateY(-2px) scale(1.05) !important;
  box-shadow: var(--shadow-lg) !important;
}

.theme-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.theme-btn:hover::before {
  left: 100%;
}

.theme-icon {
  font-size: 1.2rem !important;
  position: relative !important;
  z-index: 1 !important;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) !important;
}

.theme-menu {
  min-width: 280px !important;
  padding: var(--space-lg) !important;
  background: var(--bg-glass-strong) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid var(--border-color) !important;
  border-radius: var(--radius-lg) !important;
  box-shadow: var(--shadow-xl) !important;
}

.theme-item {
  display: flex !important;
  align-items: center !important;
  gap: var(--space-lg) !important;
  padding: var(--space-lg) !important;
  border-radius: var(--radius-md) !important;
  transition: var(--transition-fast) !important;
  cursor: pointer !important;
}

.theme-item:hover {
  background: var(--bg-hover) !important;
  transform: translateX(4px) !important;
}

.theme-item.active {
  background: var(--bg-selected) !important;
  border-left: 3px solid var(--brand-color) !important;
  padding-left: calc(var(--space-lg) - 3px) !important;
}

.theme-item .theme-icon {
  font-size: 1.2rem !important;
  width: 24px !important;
  text-align: center !important;
  position: relative !important;
  z-index: 1 !important;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) !important;
}

.theme-info {
  flex: 1 !important;
  display: flex !important;
  flex-direction: column !important;
  gap: var(--space-xs) !important;
}

.theme-label {
  font-weight: var(--font-weight-medium) !important;
  color: var(--text-primary) !important;
  font-size: var(--font-size-sm) !important;
  line-height: var(--line-height-tight) !important;
}

.theme-desc {
  font-size: var(--font-size-xs) !important;
  color: var(--text-tertiary) !important;
  line-height: var(--line-height-tight) !important;
}

.check-icon {
  color: var(--brand-color) !important;
  font-weight: var(--font-weight-bold) !important;
  font-size: 1.2rem !important;
  animation: checkPulse 0.3s ease-out !important;
}

@keyframes checkPulse {
  0% {
    transform: scale(0.8);
    opacity: 0;
  }
  50% {
    transform: scale(1.2);
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}

/* 深色模式适配 */
html.dark .theme-btn {
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-dark)) !important;
  box-shadow: var(--shadow-md) !important;
}

html.dark .theme-btn:hover,
html.dark .theme-btn:focus {
  background: linear-gradient(135deg, var(--brand-color-light), var(--brand-color)) !important;
  box-shadow: var(--shadow-lg) !important;
}

html.dark .theme-menu {
  background: var(--bg-glass-strong) !important;
  border-color: var(--border-color) !important;
}

html.dark .theme-item:hover {
  background: var(--bg-hover) !important;
}

html.dark .theme-item.active {
  background: var(--bg-selected) !important;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .theme-menu {
    min-width: 240px !important;
    padding: var(--space-md) !important;
  }
  
  .theme-item {
    padding: var(--space-md) !important;
  }
  
  .theme-item .theme-icon {
    font-size: 1.2rem !important;
    width: 20px !important;
  }
  
  .theme-label {
    font-size: var(--font-size-xs) !important;
  }
  
  .theme-desc {
    font-size: 10px !important;
  }
}

/* 触摸设备优化 */
@media (hover: none) and (pointer: coarse) {
  .theme-btn:hover {
    transform: none !important;
  }
  
  .theme-btn:active {
    transform: scale(0.95) !important;
  }
  
  .theme-item:hover {
    transform: none !important;
  }
  
  .theme-item:active {
    background: var(--bg-selected) !important;
    color: var(--text-brand) !important;
  }
}

/* 减少动画模式支持 */
@media (prefers-reduced-motion: reduce) {
  .theme-btn,
  .theme-item,
  .check-icon {
    transition: none !important;
    animation: none !important;
  }
  
  .theme-btn:hover,
  .theme-item:hover {
    transform: none !important;
  }
  
  .theme-btn::before {
    display: none !important;
  }
}
</style> 