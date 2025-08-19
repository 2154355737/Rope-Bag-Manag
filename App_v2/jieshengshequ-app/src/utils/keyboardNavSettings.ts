interface KeyboardNavSettings {
  hideNavOnKeyboard: boolean
  animationDuration: number
}

const defaultSettings: KeyboardNavSettings = {
  hideNavOnKeyboard: true,
  animationDuration: 300
}

// 初始化键盘导航设置
export const initializeKeyboardNavSettings = () => {
  try {
    const saved = localStorage.getItem('keyboard-nav-settings')
    let settings = defaultSettings
    
    if (saved) {
      const parsedSettings = JSON.parse(saved)
      settings = { ...defaultSettings, ...parsedSettings }
      console.log('📱 加载键盘导航设置:', settings)
    } else {
      console.log('📱 使用默认键盘导航设置:', settings)
    }
    
    applyKeyboardNavSettings(settings)
  } catch (error) {
    console.error('❌ 初始化键盘导航设置失败:', error)
    applyKeyboardNavSettings(defaultSettings)
  }
}

// 应用键盘导航设置
export const applyKeyboardNavSettings = (settings: KeyboardNavSettings) => {
  const root = document.documentElement
  
  // 设置CSS变量
  root.style.setProperty('--nav-hide-animation-duration', `${settings.animationDuration}ms`)
  
  // 设置CSS类来控制行为
  if (settings.hideNavOnKeyboard) {
    document.body.classList.add('hide-nav-on-keyboard')
    document.body.classList.remove('keep-nav-on-keyboard')
  } else {
    document.body.classList.add('keep-nav-on-keyboard')
    document.body.classList.remove('hide-nav-on-keyboard')
  }
  
  console.log('🔧 应用键盘导航设置:', settings)
}

// 获取当前设置
export const getKeyboardNavSettings = (): KeyboardNavSettings => {
  try {
    const saved = localStorage.getItem('keyboard-nav-settings')
    if (saved) {
      const parsedSettings = JSON.parse(saved)
      return { ...defaultSettings, ...parsedSettings }
    }
  } catch (error) {
    console.error('❌ 获取键盘导航设置失败:', error)
  }
  return defaultSettings
}

// 保存设置
export const saveKeyboardNavSettings = (settings: KeyboardNavSettings) => {
  try {
    localStorage.setItem('keyboard-nav-settings', JSON.stringify(settings))
    console.log('💾 保存键盘导航设置:', settings)
    applyKeyboardNavSettings(settings)
    return true
  } catch (error) {
    console.error('❌ 保存键盘导航设置失败:', error)
    return false
  }
} 