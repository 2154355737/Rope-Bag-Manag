import { Capacitor } from '@capacitor/core'
import { Keyboard } from '@capacitor/keyboard'

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
}

// 键盘事件处理
export const initializeKeyboard = () => {
  if (isNative()) {
    Keyboard.addListener('keyboardWillShow', () => {
      document.body.classList.add('keyboard-open')
    })
    
    Keyboard.addListener('keyboardWillHide', () => {
      document.body.classList.remove('keyboard-open')
    })
  }
}

// 获取设备信息
export const getDeviceInfo = async () => {
  if (isNative()) {
    try {
      const { Device } = await import('@capacitor/device')
      return await Device.getInfo()
    } catch (error) {
      console.error('获取设备信息失败:', error)
      return null
    }
  }
  return null
} 