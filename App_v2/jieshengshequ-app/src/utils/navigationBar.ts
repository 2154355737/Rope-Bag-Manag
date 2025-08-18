import { Capacitor } from '@capacitor/core'
import { Device } from '@capacitor/device'
import { StatusBar } from '@capacitor/status-bar'

// 导航栏类型
export enum NavigationType {
  NONE = 'none',           // 无导航栏
  GESTURE = 'gesture',     // 手势导航
  BUTTONS = 'buttons'      // 按键导航
}

// 导航栏信息
export interface NavigationBarInfo {
  type: NavigationType
  height: number
  isVisible: boolean
  hasHomeIndicator: boolean  // iOS的Home Indicator或Android的手势指示器
}

// 检测导航栏类型和高度
export const detectNavigationBar = async (): Promise<NavigationBarInfo> => {
  const platform = Capacitor.getPlatform()
  
  if (platform === 'android') {
    return await detectAndroidNavigationBarWithPlugins()
  } else if (platform === 'ios') {
    return await detectIOSNavigationBarWithPlugins()
  } else {
    // Web平台
    return {
      type: NavigationType.NONE,
      height: 0,
      isVisible: false,
      hasHomeIndicator: false
    }
  }
}

// 使用插件检测Android导航栏
const detectAndroidNavigationBarWithPlugins = async (): Promise<NavigationBarInfo> => {
  try {
    // 获取设备信息
    const deviceInfo = await Device.getInfo()
    console.log('设备信息:', deviceInfo)
    
    // 获取状态栏信息
    let statusBarInfo = null
    try {
      statusBarInfo = await StatusBar.getInfo()
      console.log('状态栏信息:', statusBarInfo)
    } catch (error) {
      console.warn('无法获取状态栏信息:', error)
    }
    
    // 获取屏幕信息
    const screenInfo = {
      screenWidth: window.screen.width,
      screenHeight: window.screen.height,
      availableWidth: window.screen.availWidth,
      availableHeight: window.screen.availHeight,
      windowWidth: window.innerWidth,
      windowHeight: window.innerHeight,
      visualViewportHeight: window.visualViewport?.height || window.innerHeight,
      devicePixelRatio: window.devicePixelRatio || 1
    }
    
    // 获取CSS安全区域
    const safeAreaBottom = getCSSEnvValue('safe-area-inset-bottom')
    const safeAreaTop = getCSSEnvValue('safe-area-inset-top')
    
    console.log('完整检测数据:', {
      deviceInfo,
      statusBarInfo,
      screenInfo,
      safeAreas: { top: safeAreaTop, bottom: safeAreaBottom }
    })
    
    // 使用多种方法检测导航栏
    return analyzeNavigationBar(screenInfo, safeAreaBottom, deviceInfo)
    
  } catch (error) {
    console.error('插件检测Android导航栏失败:', error)
    // 降级到原来的检测方法
    return await detectAndroidNavigationBar()
  }
}

// 分析导航栏信息
const analyzeNavigationBar = (screenInfo: any, safeAreaBottom: number, deviceInfo: any): NavigationBarInfo => {
  let navigationType = NavigationType.NONE
  let navigationHeight = 0
  let isVisible = false
  
  // 方法1: 使用safe-area-inset-bottom（最可靠）
  if (safeAreaBottom > 0) {
    console.log('检测到safe-area-inset-bottom:', safeAreaBottom)
    
    // 根据高度判断导航栏类型
    if (safeAreaBottom < 3) {
      // 极小值，可能是误差，认为无导航栏
      navigationType = NavigationType.NONE
      navigationHeight = 0
      isVisible = false
    } else if (safeAreaBottom <= 15) {
      // 手势导航的指示器
      navigationType = NavigationType.GESTURE
      navigationHeight = safeAreaBottom
      isVisible = true
    } else if (safeAreaBottom <= 25) {
      // 较小的手势导航区域
      navigationType = NavigationType.GESTURE
      navigationHeight = safeAreaBottom
      isVisible = true
    } else {
      // 传统按键导航
      navigationType = NavigationType.BUTTONS
      navigationHeight = safeAreaBottom
      isVisible = true
    }
  } else {
    // 方法2: 使用屏幕高度差值
    const heightDiff = screenInfo.screenHeight - screenInfo.availableHeight
    const windowHeightDiff = screenInfo.screenHeight - screenInfo.windowHeight
    
    console.log('屏幕高度差值:', { heightDiff, windowHeightDiff })
    
    if (heightDiff > 5) {
      navigationHeight = heightDiff
      isVisible = true
      navigationType = heightDiff < 30 ? NavigationType.GESTURE : NavigationType.BUTTONS
    } else if (windowHeightDiff > 100) {
      // 可能有导航栏，但被其他因素影响
      navigationHeight = Math.min(windowHeightDiff - 50, 48) // 减去状态栏等其他因素
      isVisible = navigationHeight > 0
      navigationType = navigationHeight < 30 ? NavigationType.GESTURE : NavigationType.BUTTONS
         } else {
       // 方法3: 使用设备信息和Android版本进行智能判断
       const androidVersion = parseInt(deviceInfo.osVersion?.split('.')[0] || '0')
       const isModernAndroid = androidVersion >= 10 // Android 10+ 支持手势导航
       
       console.log('Android版本分析:', { androidVersion, isModernAndroid })
       
       // 检查是否为全屏模式或沉浸式模式
       const isFullscreen = checkFullScreenMode()
       
       if (isFullscreen) {
         // 全屏模式，无导航栏
         navigationType = NavigationType.NONE
         navigationHeight = 0
         isVisible = false
             } else if (isModernAndroid) {
        // 现代Android设备，需要更智能的检测
        // 如果安全区域为0，可能是：
        // 1. 真的没有导航栏（全屏应用）
        // 2. 有按键导航但CSS检测失败
        // 3. 手势导航但高度为0
        
                 console.log('🔍 现代Android智能检测:', {
           safeAreaBottom: safeAreaBottom,
           screenInfo: screenInfo
         })
         
                  // 多重检测方法
         const screenRatio = screenInfo.screenHeight / screenInfo.screenWidth
         const isLikelyHasNavBar = screenRatio > 1.8 // 长屏设备通常有导航栏
         
         // 方法1: 屏幕比例检测
         const ratioBasedDetection = isLikelyHasNavBar && safeAreaBottom === 0
         
         // 方法2: 检查设备型号（小米设备通常有导航栏选项）
         const isXiaomiDevice = deviceInfo.model?.includes('Xiaomi') || deviceInfo.manufacturer === 'Xiaomi'
         
         // 方法3: Android 15 通常支持多种导航方式
         const supportsMultipleNavTypes = androidVersion >= 14
         
         console.log('🔍 多重检测结果:', {
           ratioBasedDetection,
           isXiaomiDevice,
           supportsMultipleNavTypes,
           deviceModel: deviceInfo.model,
           manufacturer: deviceInfo.manufacturer
         })
         
         if (ratioBasedDetection || (isXiaomiDevice && supportsMultipleNavTypes)) {
           // 很可能有按键导航但CSS检测失败
           console.log('⚠️  推测有按键导航栏但CSS检测失败，使用默认值')
           navigationType = NavigationType.BUTTONS
           navigationHeight = 48 // Android标准导航栏高度
           isVisible = true
         } else {
           // 可能是手势导航或真的没有导航栏
           // 对于现代Android设备，倾向于认为有某种形式的导航
           navigationType = NavigationType.GESTURE
           navigationHeight = 16 // 稍大一点的手势指示器高度
           isVisible = true
         }
      } else {
        // 旧版Android，通常有按键导航
        console.log('📱 旧版Android，默认按键导航')
        navigationType = NavigationType.BUTTONS
        navigationHeight = 48
        isVisible = true
      }
     }
  }
  
      console.log('🎯 导航栏分析结果:', {
      type: navigationType,
      height: navigationHeight + 'px',
      visible: isVisible,
      解读: navigationType === 'none' ? '无导航栏/全屏模式' : 
           navigationType === 'gesture' ? '手势导航' : '按键导航'
    })
  
  return {
    type: navigationType,
    height: navigationHeight,
    isVisible: isVisible,
    hasHomeIndicator: navigationType === NavigationType.GESTURE && navigationHeight > 0
  }
}

// 检测Android导航栏（原方法作为备用）
const detectAndroidNavigationBar = async (): Promise<NavigationBarInfo> => {
  try {
    // 获取屏幕尺寸信息
    const screenHeight = window.screen.height
    const availableHeight = window.screen.availHeight
    const windowHeight = window.innerHeight
    const visualViewportHeight = window.visualViewport?.height || window.innerHeight
    
    // 通过CSS环境变量检测（最可靠的方法）
    const safeAreaBottom = getCSSEnvValue('safe-area-inset-bottom')
    
    console.log('导航栏检测数据:', {
      screenHeight,
      availableHeight,
      windowHeight,
      visualViewportHeight,
      safeAreaBottom,
      screenDiff: screenHeight - availableHeight,
      windowDiff: screenHeight - windowHeight,
      visualDiff: screenHeight - visualViewportHeight
    })
    
    // 检测导航栏类型和状态
    let navigationType = NavigationType.NONE
    let navigationHeight = 0
    let isVisible = false
    
    // 方法1: 使用safe-area-inset-bottom（最准确）
    if (safeAreaBottom > 0) {
      navigationHeight = safeAreaBottom
      isVisible = true
      
      // 判断导航栏类型
      if (safeAreaBottom <= 10) {
        // 很小的值，可能是手势指示器或无导航栏
        if (safeAreaBottom < 5) {
          navigationType = NavigationType.NONE
          navigationHeight = 0
          isVisible = false
        } else {
          navigationType = NavigationType.GESTURE
        }
      } else if (safeAreaBottom < 35) {
        navigationType = NavigationType.GESTURE
      } else {
        navigationType = NavigationType.BUTTONS
      }
    } else {
      // 方法2: 使用屏幕高度差值作为备用检测
      const heightDiff = screenHeight - availableHeight
      
      if (heightDiff > 10) {
        navigationHeight = heightDiff
        isVisible = true
        navigationType = heightDiff < 35 ? NavigationType.GESTURE : NavigationType.BUTTONS
      } else {
        // 方法3: 检查是否为全屏模式
        const isFullScreen = checkFullScreenMode()
        if (isFullScreen) {
          navigationType = NavigationType.NONE
          navigationHeight = 0
          isVisible = false
        } else {
          // 默认假设有导航栏（保守策略）
          navigationType = NavigationType.BUTTONS
          navigationHeight = 0 // 但不预留空间
          isVisible = false
        }
      }
    }
    
    return {
      type: navigationType,
      height: navigationHeight,
      isVisible: isVisible,
      hasHomeIndicator: navigationType === NavigationType.GESTURE && navigationHeight > 0
    }
  } catch (error) {
    console.error('检测Android导航栏失败:', error)
    // 出错时返回无导航栏状态，避免错误预留空间
    return {
      type: NavigationType.NONE,
      height: 0,
      isVisible: false,
      hasHomeIndicator: false
    }
  }
}

// 检查是否为全屏模式
const checkFullScreenMode = (): boolean => {
  try {
    // 检查document.fullscreenElement
    if (document.fullscreenElement) {
      return true
    }
    
    // 检查屏幕高度与视口高度的关系
    const screenHeight = window.screen.height
    const windowHeight = window.innerHeight
    const heightRatio = windowHeight / screenHeight
    
    // 如果视口高度接近屏幕高度（95%以上），可能是全屏模式
    if (heightRatio > 0.95) {
      return true
    }
    
    // 检查CSS媒体查询
    if (window.matchMedia('(display-mode: fullscreen)').matches) {
      return true
    }
    
    return false
  } catch (error) {
    console.error('检查全屏模式失败:', error)
    return false
  }
}

// 使用插件检测iOS导航栏
const detectIOSNavigationBarWithPlugins = async (): Promise<NavigationBarInfo> => {
  try {
    // 获取设备信息
    const deviceInfo = await Device.getInfo()
    console.log('iOS设备信息:', deviceInfo)
    
    // 获取安全区域
    const safeAreaBottom = getCSSEnvValue('safe-area-inset-bottom')
    
    console.log('iOS安全区域检测:', { safeAreaBottom, deviceInfo })
    
    // iOS设备都使用手势导航（从iPhone X开始）
    return {
      type: NavigationType.GESTURE,
      height: safeAreaBottom,
      isVisible: safeAreaBottom > 0,
      hasHomeIndicator: safeAreaBottom > 0
    }
  } catch (error) {
    console.error('插件检测iOS导航栏失败:', error)
    // 降级到原来的检测方法
    return await detectIOSNavigationBar()
  }
}

// 检测iOS导航栏（原方法作为备用）
const detectIOSNavigationBar = async (): Promise<NavigationBarInfo> => {
  const safeAreaBottom = getCSSEnvValue('safe-area-inset-bottom')
  
  return {
    type: NavigationType.GESTURE,
    height: safeAreaBottom,
    isVisible: safeAreaBottom > 0,
    hasHomeIndicator: safeAreaBottom > 0
  }
}

// 获取CSS环境变量值
const getCSSEnvValue = (property: string): number => {
  const testElement = document.createElement('div')
  testElement.style.position = 'fixed'
  testElement.style.top = '0'
  testElement.style.left = '0'
  testElement.style.width = '1px'
  testElement.style.height = '1px'
  testElement.style.paddingBottom = `env(${property})`
  testElement.style.visibility = 'hidden'
  
  document.body.appendChild(testElement)
  const computedStyle = window.getComputedStyle(testElement)
  const paddingBottom = computedStyle.paddingBottom
  document.body.removeChild(testElement)
  
  return parseFloat(paddingBottom) || 0
}

// 设置CSS变量
export const setNavigationBarCSSVariables = (navInfo: NavigationBarInfo) => {
  const root = document.documentElement
  
  // 设置CSS变量
  root.style.setProperty('--navigation-bar-height', `${navInfo.height}px`)
  root.style.setProperty('--navigation-bar-type', navInfo.type)
  root.style.setProperty('--navigation-bar-visible', navInfo.isVisible ? '1' : '0')
  root.style.setProperty('--has-home-indicator', navInfo.hasHomeIndicator ? '1' : '0')
  root.style.setProperty('--safe-bottom-padding', `${navInfo.height}px`)
  
  // 添加对应的CSS类
  document.body.classList.remove('nav-none', 'nav-gesture', 'nav-buttons')
  document.body.classList.add(`nav-${navInfo.type}`)
  
  // 添加可见性类
  if (navInfo.isVisible) {
    document.body.classList.add('nav-visible')
  } else {
    document.body.classList.remove('nav-visible')
  }
  
  if (navInfo.hasHomeIndicator) {
    document.body.classList.add('has-home-indicator')
  } else {
    document.body.classList.remove('has-home-indicator')
  }
  
  // 调试日志
  console.log('✅ 导航栏CSS变量已设置:', {
    height: navInfo.height + 'px',
    type: navInfo.type,
    visible: navInfo.isVisible,
    homeIndicator: navInfo.hasHomeIndicator,
    状态: navInfo.isVisible ? 
      (navInfo.type === 'gesture' ? '✋ 手势导航' : '🔘 按键导航') : 
      '🚫 无导航栏'
  })
}

// 监听导航栏变化（屏幕旋转等）
export const watchNavigationBarChanges = (callback: (navInfo: NavigationBarInfo) => void) => {
  let currentNavInfo: NavigationBarInfo | null = null
  
  const checkChanges = async () => {
    const newNavInfo = await detectNavigationBar()
    
    if (!currentNavInfo || 
        currentNavInfo.type !== newNavInfo.type || 
        Math.abs(currentNavInfo.height - newNavInfo.height) > 5) {
      currentNavInfo = newNavInfo
      callback(newNavInfo)
    }
  }
  
  // 监听屏幕方向变化
  window.addEventListener('orientationchange', () => {
    setTimeout(checkChanges, 100)
  })
  
  // 监听窗口大小变化
  window.addEventListener('resize', () => {
    setTimeout(checkChanges, 100)
  })
  
  // 初始检测
  checkChanges()
  
  return () => {
    window.removeEventListener('orientationchange', checkChanges)
    window.removeEventListener('resize', checkChanges)
  }
} 