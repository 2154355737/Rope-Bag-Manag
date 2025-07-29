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
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  backdrop-filter: blur(12px);
  border: none;
  color: #fff;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 15px 0 rgba(102, 126, 234, 0.4);
  position: relative;
  overflow: hidden;
  width: 40px;
  height: 40px;
}

.theme-btn:hover {
  background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
  transform: translateY(-3px) scale(1.05);
  box-shadow: 0 8px 25px 0 rgba(102, 126, 234, 0.6);
}

.theme-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s;
}

.theme-btn:hover::before {
  left: 100%;
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
  font-size: 1.2rem;
  width: 20px;
  text-align: center;
  position: relative;
  z-index: 1;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
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
  background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
  box-shadow: 0 4px 15px 0 rgba(79, 70, 229, 0.4);
}

.dark .theme-btn:hover {
  background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%);
  box-shadow: 0 8px 25px 0 rgba(79, 70, 229, 0.6);
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