import { StatusBar, Style } from '@capacitor/status-bar'
import { Capacitor } from '@capacitor/core'

export const initializeStatusBar = async () => {
  if (Capacitor.isNativePlatform()) {
    try {
      // 设置状态栏样式为深色内容（适合浅色背景）
      await StatusBar.setStyle({ style: Style.Dark })
      
      // 设置状态栏背景颜色为白色
      await StatusBar.setBackgroundColor({ color: '#ffffff' })
      
      // 显示状态栏
      await StatusBar.show()
      
      console.log('状态栏配置成功')
    } catch (error) {
      console.error('状态栏配置失败:', error)
    }
  }
}

export const setStatusBarLight = async () => {
  if (Capacitor.isNativePlatform()) {
    try {
      await StatusBar.setStyle({ style: Style.Light })
      await StatusBar.setBackgroundColor({ color: '#000000' })
    } catch (error) {
      console.error('设置浅色状态栏失败:', error)
    }
  }
}

export const setStatusBarDark = async () => {
  if (Capacitor.isNativePlatform()) {
    try {
      await StatusBar.setStyle({ style: Style.Dark })
      await StatusBar.setBackgroundColor({ color: '#ffffff' })
    } catch (error) {
      console.error('设置深色状态栏失败:', error)
    }
  }
} 