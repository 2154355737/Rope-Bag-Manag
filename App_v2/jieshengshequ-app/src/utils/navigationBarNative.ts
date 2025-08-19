import NavigationBarPlugin, { NavigationBarInfo } from '../plugins/NavigationBarPlugin'
import { NavigationType, NavigationBarInfo as AppNavigationBarInfo } from './navigationBar'
import { Capacitor } from '@capacitor/core'

/**
 * 监听导航栏变化
 */
export const addNavigationBarListener = async (
  callback: (info: NavigationBarInfo) => void
) => {
  if (Capacitor.isNativePlatform()) {
    try {
      await NavigationBarPlugin.addListener('navigationBarChanged', callback)
      console.log('✅ 导航栏监听器已添加')
    } catch (error) {
      console.error('❌ 添加导航栏监听器失败:', error)
    }
  }
}

/**
 * 获取导航栏信息
 */
export const getNavigationBarInfo = async (): Promise<NavigationBarInfo | null> => {
  if (Capacitor.isNativePlatform()) {
    try {
      const info = await NavigationBarPlugin.getNavigationBarInfo()
      console.log('📱 导航栏信息:', info)
      return info
    } catch (error) {
      console.error('❌ 获取导航栏信息失败:', error)
      return null
    }
  }
  return null
}

/**
 * 移除所有导航栏监听器
 */
export const removeNavigationBarListeners = async () => {
  if (Capacitor.isNativePlatform()) {
    try {
      await NavigationBarPlugin.removeAllListeners()
      console.log('🗑️ 导航栏监听器已移除')
    } catch (error) {
      console.error('❌ 移除导航栏监听器失败:', error)
    }
  }
} 

// Android底部导航栏配置接口
export interface AndroidNavigationBarConfig {
  backgroundColor: string
  buttonColor: 'light' | 'dark' | 'default'
  hidden: boolean
  overlaysContent: boolean // 是否覆盖内容
}

// 默认Android导航栏配置
export const defaultAndroidNavBarConfig: AndroidNavigationBarConfig = {
  backgroundColor: '#ffffff',
  buttonColor: 'dark',
  hidden: false,
  overlaysContent: false
}

// 导航栏预设
export const androidNavBarPresets = {
  // 浅色主题
  light: {
    backgroundColor: '#ffffff',
    buttonColor: 'dark' as const,
    hidden: false,
    overlaysContent: false
  },
  // 深色主题
  dark: {
    backgroundColor: '#000000',
    buttonColor: 'light' as const,
    hidden: false,
    overlaysContent: false
  },
  // 透明导航栏
  transparent: {
    backgroundColor: '#00000000',
    buttonColor: 'dark' as const,
    hidden: false,
    overlaysContent: true
  },
  // 沉浸式 (隐藏导航栏)
  immersive: {
    backgroundColor: '#000000',
    buttonColor: 'light' as const,
    hidden: true,
    overlaysContent: true
  }
}

// 应用Android导航栏配置（使用CSS和原生接口的混合方案）
export const applyAndroidNavigationBarConfig = async (config: AndroidNavigationBarConfig) => {
  if (Capacitor.getPlatform() !== 'android') {
    console.log('⚠️ 非Android平台，跳过导航栏配置')
    return false
  }

  try {
    console.log('🔧 应用Android导航栏配置:', config)

    // 首先尝试获取导航栏信息
    const navInfo = await getNavigationBarInfo()
    
    // 使用CSS变量和类来控制导航栏样式
    const root = document.documentElement
    
    // 设置CSS变量（用于 CSS/遮罩联动）
    root.style.setProperty('--android-nav-bar-color', config.backgroundColor)
    root.style.setProperty('--android-nav-bar-button-style', config.buttonColor)
    root.style.setProperty('--android-nav-bar-hidden', config.hidden ? '1' : '0')
    
    console.log('🎨 设置Android导航栏CSS变量:', {
      color: config.backgroundColor,
      buttonStyle: config.buttonColor,
      hidden: config.hidden
    })
    
    // 设置CSS类来控制样式
    if (config.hidden) {
      document.body.classList.add('hide-navigation-bar')
      document.body.classList.remove('show-navigation-bar')
    } else {
      document.body.classList.add('show-navigation-bar')
      document.body.classList.remove('hide-navigation-bar')
    }

    // 同步到平台类，用于 CSS 画底部背景层
    if (typeof document !== 'undefined') {
      const hasAndroidClass = document.body.classList.contains('platform-android')
      if (hasAndroidClass) {
        if (config.hidden) {
          document.body.classList.remove('show-navigation-bar')
        } else {
          document.body.classList.add('show-navigation-bar')
        }
      }
    }
    
    if (config.overlaysContent) {
      document.body.classList.add('navigation-overlays-content')
    } else {
      document.body.classList.remove('navigation-overlays-content')
    }
    
    // 设置主题相关的类
    document.body.classList.remove('nav-light', 'nav-dark', 'nav-default')
    document.body.classList.add(`nav-${config.buttonColor}`)

    // 如果有原生导航栏信息，记录到控制台
    if (navInfo) {
      console.log('📱 原生导航栏信息:', {
        hasNavigationBar: navInfo.hasNavigationBar,
        height: navInfo.navigationBarHeight,
        type: navInfo.navigationType,
        visible: navInfo.isVisible
      })
    }

    console.log('✅ Android导航栏配置应用成功（CSS方案）')
    return true
  } catch (error) {
    console.error('❌ Android导航栏配置失败:', error)
    return false
  }
}

// 快速应用预设
export const applyAndroidNavBarPreset = async (presetName: keyof typeof androidNavBarPresets) => {
  const preset = androidNavBarPresets[presetName]
  if (!preset) {
    console.error('❌ 未知的Android导航栏预设:', presetName)
    return false
  }
  
  console.log(`🎨 应用Android导航栏预设: ${presetName}`)
  return await applyAndroidNavigationBarConfig(preset)
}

// 动态设置导航栏颜色
export const setAndroidNavigationBarColor = async (
  color: string, 
  buttonStyle?: 'light' | 'dark' | 'default'
) => {
  const config: AndroidNavigationBarConfig = {
    backgroundColor: color,
    buttonColor: buttonStyle || 'dark',
    hidden: false,
    overlaysContent: false
  }
  return await applyAndroidNavigationBarConfig(config)
}

// 切换导航栏可见性
export const toggleAndroidNavigationBar = async () => {
  if (Capacitor.getPlatform() !== 'android') return false
  
  try {
    const isHidden = document.body.classList.contains('hide-navigation-bar')
    
    const config: AndroidNavigationBarConfig = {
      ...defaultAndroidNavBarConfig,
      hidden: !isHidden
    }
    
    return await applyAndroidNavigationBarConfig(config)
  } catch (error) {
    console.error('❌ 切换Android导航栏可见性失败:', error)
    return false
  }
}

// 获取导航栏高度
export const getAndroidNavigationBarHeight = async (): Promise<number> => {
  if (Capacitor.getPlatform() !== 'android') return 0
  
  try {
    // 尝试从原生插件获取实际高度
    const navInfo = await getNavigationBarInfo()
    if (navInfo && navInfo.navigationBarHeight > 0) {
      console.log('📏 从原生获取导航栏高度:', navInfo.navigationBarHeight)
      return navInfo.navigationBarHeight
    }
  } catch (error) {
    console.log('⚠️ 无法从原生获取导航栏高度，使用估算值')
  }
  
  // 降级：使用估算值
  const density = window.devicePixelRatio || 1
  const estimatedHeightDp = 48 // dp
  const estimatedHeight = estimatedHeightDp * density
  
  console.log('📏 估算导航栏高度:', estimatedHeight)
  return estimatedHeight
}

// 检测设备是否有硬件导航按键
export const hasHardwareNavigationButtons = async (): Promise<boolean> => {
  if (Capacitor.getPlatform() !== 'android') return false
  
  try {
    // 尝试从原生插件获取准确信息
    const navInfo = await getNavigationBarInfo()
    if (navInfo) {
      // navigationType: 0=无, 1=传统按键, 2=手势导航
      const hasHardware = navInfo.navigationType === 1
      console.log('🔍 从原生检测导航类型:', {
        type: navInfo.navigationType,
        hasHardware
      })
      return hasHardware
    }
  } catch (error) {
    console.log('⚠️ 无法从原生检测导航类型，使用估算方法')
  }
  
  // 降级：使用估算方法
  const screenHeight = window.screen.height
  const availableHeight = window.innerHeight
  const heightDifference = screenHeight - availableHeight
  
  // 如果差异很小，可能是硬件按键
  const hasHardware = heightDifference < 100
  
  console.log('🔍 估算导航类型:', {
    screenHeight,
    availableHeight,
    heightDifference,
    hasHardware
  })
  
  return hasHardware
}

// 初始化导航栏（在应用启动时调用）
export const initializeAndroidNavigationBar = async () => {
  if (Capacitor.getPlatform() !== 'android') {
    console.log('⚠️ 非Android平台，跳过导航栏初始化')
    return false
  }
  
  console.log('🔧 初始化Android导航栏...')
  
  try {
    // 获取设备导航栏信息
    const navInfo = await getNavigationBarInfo()
    const hasHardware = await hasHardwareNavigationButtons()
    const navBarHeight = await getAndroidNavigationBarHeight()
    
    console.log('📱 Android导航栏设备信息:', {
      hasNavigationBar: navInfo?.hasNavigationBar,
      navigationType: navInfo?.navigationType,
      height: navBarHeight,
      hasHardwareButtons: hasHardware,
      isVisible: navInfo?.isVisible
    })
    
    // 应用默认配置
    const success = await applyAndroidNavigationBarConfig(defaultAndroidNavBarConfig)
    
    // 添加监听器
    if (navInfo) {
      await addNavigationBarListener((info) => {
        console.log('📱 导航栏状态变化:', info)
        
        // 更新CSS变量以反映实际状态
        const root = document.documentElement
        root.style.setProperty('--actual-nav-bar-height', `${info.navigationBarHeight}px`)
        root.style.setProperty('--actual-nav-bar-visible', info.isVisible ? '1' : '0')
      })
    }
    
    return success
  } catch (error) {
    console.error('❌ 初始化Android导航栏失败:', error)
    return false
  }
} 