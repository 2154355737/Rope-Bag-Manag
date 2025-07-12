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
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  color: var(--text-primary);
  transition: all var(--transition-base);
  box-shadow: var(--shadow-light);
}

.theme-btn:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
}

.theme-menu {
  min-width: 280px;
  padding: var(--spacing-sm);
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: var(--border-radius-large);
  box-shadow: var(--shadow-dark);
}

.theme-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-base);
  transition: all var(--transition-base);
  cursor: pointer;
}

.theme-item:hover {
  background: var(--bg-secondary);
}

.theme-icon {
  font-size: 1.5rem;
  width: 24px;
  text-align: center;
}

.theme-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.theme-label {
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.theme-desc {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  line-height: 1.3;
}

.check-icon {
  color: var(--brand-color);
  font-weight: var(--font-weight-bold);
}

/* 深色模式适配 */
.dark .theme-btn {
  background: rgba(44, 44, 44, 0.9);
  border-color: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
}

.dark .theme-btn:hover {
  background: rgba(44, 44, 44, 1);
}

.dark .theme-menu {
  background: rgba(44, 44, 44, 0.95);
  border-color: rgba(255, 255, 255, 0.1);
}

.dark .theme-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .theme-menu {
    min-width: 240px;
  }
  
  .theme-item {
    padding: var(--spacing-xs);
  }
  
  .theme-icon {
    font-size: 1.2rem;
    width: 20px;
  }
  
  .theme-label {
    font-size: var(--font-size-xs);
  }
  
  .theme-desc {
    font-size: 10px;
  }
}

/* 触摸设备优化 */
@media (hover: none) and (pointer: coarse) {
  .theme-btn:hover {
    transform: none;
  }
  
  .theme-btn:active {
    transform: scale(0.95);
  }
  
  .theme-item:active {
    background: var(--brand-color-light);
    color: white;
  }
}
</style> 