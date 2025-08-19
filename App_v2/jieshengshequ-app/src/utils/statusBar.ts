import { StatusBar, Style } from '@capacitor/status-bar'
import { Capacitor } from '@capacitor/core'
import NavigationBarPlugin from '@/plugins/NavigationBarPlugin'

// 状态栏配置接口
export interface StatusBarConfig {
  style: 'dark' | 'light' | 'default'
  backgroundColor: string
  visible: boolean
  overlaysWebView?: boolean // Android专用
}

// 默认状态栏配置
export const defaultStatusBarConfig: StatusBarConfig = {
  style: 'dark',
  backgroundColor: '#ffffff',
  visible: true,
  overlaysWebView: false // 确保不使用覆盖模式
}

// 获取当前状态栏信息
export const getStatusBarInfo = async () => {
  if (Capacitor.isNativePlatform()) {
    try {
      const info = await StatusBar.getInfo()
      console.log('📱 当前状态栏信息:', info)
      return info
    } catch (error) {
      console.error('❌ 获取状态栏信息失败:', error)
      return null
    }
  }
  return null
}

// 应用状态栏配置
export const applyStatusBarConfig = async (config: StatusBarConfig) => {
  if (!Capacitor.isNativePlatform()) {
    console.log('⚠️ 非原生平台，跳过状态栏配置')
    return false
  }

  try {
    console.log('🔧 应用状态栏配置:', config)

    // 设置样式
    let capacitorStyle: Style
    switch (config.style) {
      case 'light':
        capacitorStyle = Style.Light
        break
      case 'dark':
        capacitorStyle = Style.Dark
        break
      case 'default':
      default:
        capacitorStyle = Style.Default
        break
    }
    await StatusBar.setStyle({ style: capacitorStyle })

    // 设置背景颜色 (仅Android)
    if (Capacitor.getPlatform() === 'android') {
      // 先设置overlaysWebView，这个设置会影响背景颜色的显示效果
      console.log('📱 设置状态栏覆盖WebView:', config.overlaysWebView)
      await StatusBar.setOverlaysWebView({ overlay: config.overlaysWebView || false })
      
      // 然后设置背景颜色
      console.log('🎨 设置状态栏背景颜色:', config.backgroundColor)
      await StatusBar.setBackgroundColor({ color: config.backgroundColor })
      // 同步顶部遮罩颜色 & CSS 变量
      await setStatusBarScrimColor(config.backgroundColor)
    }

    // 设置可见性
    if (config.visible) {
      await StatusBar.show()
    } else {
      await StatusBar.hide()
    }

    console.log('✅ 状态栏配置应用成功')
    return true
  } catch (error) {
    console.error('❌ 状态栏配置失败:', error)
    return false
  }
}

// 预设配置
export const statusBarPresets = {
  // 浅色主题
  light: {
    style: 'dark' as const,
    backgroundColor: '#ffffff',
    visible: true,
    overlaysWebView: false
  },
  // 深色主题
  dark: {
    style: 'light' as const,
    backgroundColor: '#000000',
    visible: true,
    overlaysWebView: false
  },
  // 透明状态栏
  transparent: {
    style: 'dark' as const,
    backgroundColor: '#00000000', // 透明
    visible: true,
    overlaysWebView: true
  },
  // 沉浸式 (隐藏状态栏)
  immersive: {
    style: 'default' as const,
    backgroundColor: '#000000',
    visible: false,
    overlaysWebView: true
  }
}

// 快速应用预设
export const applyStatusBarPreset = async (presetName: keyof typeof statusBarPresets) => {
  const preset = statusBarPresets[presetName]
  if (!preset) {
    console.error('❌ 未知的状态栏预设:', presetName)
    return false
  }
  
  console.log(`🎨 应用状态栏预设: ${presetName}`)
  return await applyStatusBarConfig(preset)
}

// 兼容旧版本的函数
export const initializeStatusBar = async () => {
  const success = await applyStatusBarConfig(defaultStatusBarConfig)
  if (success) {
    console.log('✅ 状态栏初始化成功')
  } else {
    console.log('⚠️ 状态栏初始化失败')
  }
  return success
}

// 强制重新应用状态栏设置（解决透明问题）
export const forceStatusBarRefresh = async () => {
  if (!Capacitor.isNativePlatform()) return false
  
  try {
    console.log('🔄 强制刷新状态栏设置')
    
    // 强制设置为非透明背景
    const config: StatusBarConfig = {
      style: 'dark',
      backgroundColor: '#ffffff',
      visible: true,
      overlaysWebView: false
    }
    
    // 先隐藏再显示，确保设置生效
    await StatusBar.hide()
    await new Promise(resolve => setTimeout(resolve, 50))
    await StatusBar.show()
    
    // 应用配置
    await applyStatusBarConfig(config)
    
    console.log('✅ 状态栏强制刷新完成')
    return true
  } catch (error) {
    console.error('❌ 强制刷新状态栏失败:', error)
    return false
  }
}

export const setStatusBarLight = async () => {
  return await applyStatusBarPreset('light')
}

export const setStatusBarDark = async () => {
  return await applyStatusBarPreset('dark')
}

// 动态设置状态栏颜色
export const setStatusBarColor = async (color: string, style?: 'dark' | 'light' | 'default') => {
  const config: StatusBarConfig = {
    style: style || 'dark',
    backgroundColor: color,
    visible: true,
    overlaysWebView: false
  }
  const ok = await applyStatusBarConfig(config)
  if (ok && Capacitor.getPlatform() === 'android') {
    await setStatusBarScrimColor(color)
  }
  return ok
}

// 新增：设置顶部遮罩颜色 + CSS 变量
export const setStatusBarScrimColor = async (color: string) => {
  try {
    if (Capacitor.isNativePlatform() && Capacitor.getPlatform() === 'android') {
      await NavigationBarPlugin.setScrimColors({ statusColor: color })
    }
  } catch (e) {
    console.warn('无法设置原生遮罩颜色，降级为 CSS 变量', e)
  }
  const root = document.documentElement
  root.style.setProperty('--status-bar-scrim-color', color)
}

// 切换状态栏可见性
export const toggleStatusBar = async () => {
  if (!Capacitor.isNativePlatform()) return false
  
  try {
    const info = await StatusBar.getInfo()
    if (info.visible) {
      await StatusBar.hide()
      console.log('📱 状态栏已隐藏')
    } else {
      await StatusBar.show()
      console.log('📱 状态栏已显示')
    }
    return true
  } catch (error) {
    console.error('❌ 切换状态栏可见性失败:', error)
    return false
  }
} 