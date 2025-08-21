import { ref, watch, onMounted, readonly } from 'vue'

// 主题类型定义
export type ThemeType = 'light' | 'dark' | 'blue' | 'green' | 'purple' | 'orange' | 'red' | 'auto'

// 主题配置
export interface ThemeConfig {
  name: string
  label: string
  icon: string
  description: string
}

// 主题配置映射
export const themeConfigs: Record<ThemeType, ThemeConfig> = {
  light: {
    name: 'light',
    label: '浅色模式',
    icon: '☀️',
    description: '明亮清晰的界面'
  },
  dark: {
    name: 'dark',
    label: '深色模式',
    icon: '🌙',
    description: '护眼舒适的深色界面'
  },
  blue: {
    name: 'blue',
    label: '蓝色主题',
    icon: '🔵',
    description: '清新蓝色系界面'
  },
  green: {
    name: 'green',
    label: '绿色主题',
    icon: '🟢',
    description: '自然绿色系界面'
  },
  purple: {
    name: 'purple',
    label: '紫色主题',
    icon: '🟣',
    description: '优雅紫色系界面'
  },
  orange: {
    name: 'orange',
    label: '橙色主题',
    icon: '🟠',
    description: '活力橙色系界面'
  },
  red: {
    name: 'red',
    label: '红色主题',
    icon: '🔴',
    description: '热情红色系界面'
  },
  auto: {
    name: 'auto',
    label: '跟随系统',
    icon: '🔄',
    description: '自动跟随系统主题'
  }
}

// 主题状态管理
const currentTheme = ref<ThemeType>('light')
const isAutoTheme = ref(false)
const systemTheme = ref<'light' | 'dark'>('light')

// 获取系统主题
function getSystemTheme(): 'light' | 'dark' {
  if (typeof window !== 'undefined' && window.matchMedia) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }
  return 'light'
}

// 检测系统主题变化
function setupSystemThemeListener() {
  if (typeof window !== 'undefined' && window.matchMedia) {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    
    const handleChange = (e: MediaQueryListEvent) => {
      systemTheme.value = e.matches ? 'dark' : 'light'
      if (isAutoTheme.value) {
        applyTheme(systemTheme.value)
      }
    }
    
    mediaQuery.addEventListener('change', handleChange)
    
    // 返回清理函数
    return () => mediaQuery.removeEventListener('change', handleChange)
  }
  return () => {}
}

// 应用主题
export function applyTheme(theme: ThemeType) {
  const html = document.documentElement
  const body = document.body
  
  console.log('开始应用主题:', theme)
  
  // 移除所有主题类
  const allThemes = ['light', 'dark', 'blue', 'green', 'purple', 'orange', 'red']
  allThemes.forEach(themeClass => {
    html.classList.remove(themeClass)
    body.classList.remove(themeClass)
  })
  
  // 处理自动主题
  if (theme === 'auto') {
    isAutoTheme.value = true
    const actualTheme = systemTheme.value
    console.log('自动主题，实际应用:', actualTheme)
    // 统一只给 HTML 元素添加主题类
    html.classList.add(actualTheme)
    currentTheme.value = actualTheme
  } else {
    isAutoTheme.value = false
    console.log('应用固定主题:', theme)
    // 统一只给 HTML 元素添加主题类
    html.classList.add(theme)
    currentTheme.value = theme
  }
  
  // 保存到本地存储
  localStorage.setItem('theme', theme)
  
  // 触发主题变化事件
  window.dispatchEvent(new CustomEvent('themechange', { detail: { theme } }))
  
  // 验证主题是否应用成功
  setTimeout(() => {
    const appliedTheme = html.classList.contains(currentTheme.value)
    console.log('主题应用验证:', {
      requested: theme,
      current: currentTheme.value,
      applied: appliedTheme,
      htmlClasses: html.className,
      bodyClasses: body.className
    })
  }, 100)
  
  console.log('主题已切换到:', theme)
}

// 获取当前主题
export function getCurrentTheme(): ThemeType {
  return currentTheme.value
}

// 初始化主题
export function initTheme() {
  // 从本地存储加载主题设置
  const savedTheme = localStorage.getItem('theme') as ThemeType
  
  // 设置系统主题监听
  const cleanup = setupSystemThemeListener()
  
  // 应用保存的主题或默认主题
  const themeToApply = savedTheme && themeConfigs[savedTheme] ? savedTheme : 'light'
  applyTheme(themeToApply)
  
  // 返回清理函数
  return cleanup
}

// 主题切换器组件
export function useThemeSwitcher() {
  const isThemeMenuOpen = ref(false)
  const showThemeNotification = ref(false)
  const notificationMessage = ref('')
  
  // 切换主题
  const switchTheme = (theme: ThemeType) => {
    applyTheme(theme)
    isThemeMenuOpen.value = false
    
    // 显示通知
    notificationMessage.value = `已切换到${themeConfigs[theme].label}`
    showThemeNotification.value = true
    
    setTimeout(() => {
      showThemeNotification.value = false
    }, 2000)
  }
  
  // 快速切换（在浅色和深色之间）
  const quickToggle = () => {
    const current = getCurrentTheme()
    const newTheme = current === 'light' ? 'dark' : 'light'
    switchTheme(newTheme)
  }
  
  // 键盘快捷键处理
  const handleKeydown = (event: KeyboardEvent) => {
    // Ctrl/Cmd + T: 快速切换
    if ((event.ctrlKey || event.metaKey) && event.key === 't') {
      event.preventDefault()
      quickToggle()
    }
    
    // Ctrl/Cmd + Shift + T: 打开主题菜单
    if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'T') {
      event.preventDefault()
      isThemeMenuOpen.value = !isThemeMenuOpen.value
    }
  }
  
  // 设置键盘监听
  onMounted(() => {
    document.addEventListener('keydown', handleKeydown)
  })
  
  return {
    currentTheme: readonly(currentTheme),
    isThemeMenuOpen,
    showThemeNotification,
    notificationMessage,
    switchTheme,
    quickToggle,
    themeConfigs
  }
}

// 导出主题相关的响应式变量
export {
  currentTheme,
  isAutoTheme,
  systemTheme
} 