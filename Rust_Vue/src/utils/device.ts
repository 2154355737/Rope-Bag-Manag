// 设备检测工具（简化版，仅支持桌面端）
export function isSmallScreen(): boolean {
  return window.innerWidth <= 768
}

export function isTouchDevice(): boolean {
  return 'ontouchstart' in window || navigator.maxTouchPoints > 0
}

// 获取设备类型
export function getDeviceType(): 'desktop' {
  return 'desktop'
}

// 监听屏幕尺寸变化
export function onScreenSizeChange(callback: (deviceType: 'desktop') => void) {
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
  return '/users'
}

// 检查当前是否应该使用移动端版本（始终返回false）
export function shouldUseMobileVersion(): boolean {
  return false
}

// 获取当前设备信息
export function getDeviceInfo() {
  return {
    isMobile: false,
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
    '是否小屏幕': info.isSmallScreen,
    '是否触摸设备': info.isTouch,
    '设备类型': info.deviceType,
    '屏幕宽度': info.screenWidth,
    '屏幕高度': info.screenHeight,
    'User Agent': info.userAgent
  })
  return info
} 