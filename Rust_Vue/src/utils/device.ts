// 设备检测工具（支持移动端和桌面端）
export function isSmallScreen(): boolean {
  return window.innerWidth <= 768
}

export function isTouchDevice(): boolean {
  return 'ontouchstart' in window || navigator.maxTouchPoints > 0
}

// 检测是否为移动设备
export function isMobileDevice(): boolean {
  const userAgent = navigator.userAgent.toLowerCase()
  const mobileKeywords = [
    'mobile', 'android', 'iphone', 'ipad', 'ipod', 
    'blackberry', 'windows phone', 'opera mini'
  ]
  return mobileKeywords.some(keyword => userAgent.includes(keyword)) || 
         (window.innerWidth <= 768 && isTouchDevice())
}

// 获取设备类型
export function getDeviceType(): 'mobile' | 'tablet' | 'desktop' {
  const width = window.innerWidth
  const userAgent = navigator.userAgent.toLowerCase()
  
  // 检查是否为移动设备
  if (isMobileDevice() && width <= 768) {
    return 'mobile'
  }
  
  // 检查是否为平板
  if ((width > 768 && width <= 1024) || userAgent.includes('ipad')) {
    return 'tablet'
  }
  
  return 'desktop'
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
  return deviceType === 'mobile' ? '/mobile/user' : '/users'
}

// 检查当前是否应该使用移动端版本
export function shouldUseMobileVersion(): boolean {
  return getDeviceType() === 'mobile'
}

// 获取当前设备信息
export function getDeviceInfo() {
  const deviceType = getDeviceType()
  return {
    isMobile: deviceType === 'mobile',
    isTablet: deviceType === 'tablet',
    isDesktop: deviceType === 'desktop',
    isSmallScreen: isSmallScreen(),
    isTouch: isTouchDevice(),
    deviceType,
    userAgent: navigator.userAgent,
    screenWidth: window.innerWidth,
    screenHeight: window.innerHeight
  }
}

// 调试函数：打印设备信息
export function debugDeviceInfo() {
  const info = getDeviceInfo()
  console.log('🔍 设备检测信息:', {
    '是否移动端': info.isMobile,
    '是否平板': info.isTablet,
    '是否桌面端': info.isDesktop,
    '是否小屏幕': info.isSmallScreen,
    '是否触摸设备': info.isTouch,
    '设备类型': info.deviceType,
    '屏幕宽度': info.screenWidth,
    '屏幕高度': info.screenHeight,
    'User Agent': info.userAgent
  })
  return info
} 