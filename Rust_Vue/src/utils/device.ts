// 设备检测工具
export function isMobileDevice(): boolean {
  const userAgent = navigator.userAgent.toLowerCase()
  const mobileKeywords = [
    'android', 'iphone', 'ipad', 'ipod', 'blackberry', 
    'windows phone', 'mobile', 'tablet'
  ]
  
  return mobileKeywords.some(keyword => userAgent.includes(keyword))
}

export function isSmallScreen(): boolean {
  return window.innerWidth <= 768
}

export function isTouchDevice(): boolean {
  return 'ontouchstart' in window || navigator.maxTouchPoints > 0
}

// 获取设备类型
export function getDeviceType(): 'mobile' | 'tablet' | 'desktop' {
  const width = window.innerWidth
  
  if (width <= 768) {
    return 'mobile'
  } else if (width <= 1024) {
    return 'tablet'
  } else {
    return 'desktop'
  }
}

// 监听屏幕尺寸变化
export function onScreenSizeChange(callback: (deviceType: 'mobile' | 'tablet' | 'desktop') => void) {
  const handleResize = () => {
    callback(getDeviceType())
  }
  
  window.addEventListener('resize', handleResize)
  
  // 返回清理函数
  return () => {
    window.removeEventListener('resize', handleResize)
  }
}

// 获取推荐的用户管理页面路径
export function getRecommendedUserManagePath(): string {
  const deviceType = getDeviceType()
  
  if (deviceType === 'mobile') {
    return '/users-mobile'
  } else {
    return '/users'
  }
}

// 检查当前是否应该使用移动端版本
export function shouldUseMobileVersion(): boolean {
  // 优先检查屏幕尺寸，然后检查设备类型
  return isSmallScreen() || isMobileDevice()
}

// 获取当前设备信息
export function getDeviceInfo() {
  return {
    isMobile: isMobileDevice(),
    isSmallScreen: isSmallScreen(),
    isTouch: isTouchDevice(),
    deviceType: getDeviceType(),
    userAgent: navigator.userAgent,
    screenWidth: window.innerWidth,
    screenHeight: window.innerHeight
  }
}

// 调试函数：打印设备信息
export function debugDeviceInfo() {
  const info = getDeviceInfo()
  console.log('🔍 设备检测信息:', {
    '是否移动设备': info.isMobile,
    '是否小屏幕': info.isSmallScreen,
    '是否触摸设备': info.isTouch,
    '设备类型': info.deviceType,
    '屏幕宽度': info.screenWidth,
    '屏幕高度': info.screenHeight,
    '应该使用移动端版本': shouldUseMobileVersion(),
    'User Agent': info.userAgent
  })
  return info
} 