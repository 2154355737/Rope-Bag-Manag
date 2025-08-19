import { Capacitor } from '@capacitor/core'

export const getPlatform = () => {
  return Capacitor.getPlatform()
}

export const isNative = () => {
  return Capacitor.isNativePlatform()
}

export const isAndroid = () => {
  return Capacitor.getPlatform() === 'android'
}

export const isIOS = () => {
  return Capacitor.getPlatform() === 'ios'
}

export const isWeb = () => {
  return Capacitor.getPlatform() === 'web'
}

// 添加平台类名到body
export const addPlatformClass = () => {
  const platform = getPlatform()
  document.body.classList.add(`platform-${platform}`)
  
  if (isNative()) {
    document.body.classList.add('platform-native')
  } else {
    document.body.classList.add('platform-web')
  }
  
  console.log('✅ 平台类名已添加:', platform)
}

// 初始化平台相关功能
export const initializePlatform = () => {
  addPlatformClass()
  console.log('✅ 平台初始化完成:', {
    platform: getPlatform(),
    isNative: isNative(),
    userAgent: navigator.userAgent
  })
} 