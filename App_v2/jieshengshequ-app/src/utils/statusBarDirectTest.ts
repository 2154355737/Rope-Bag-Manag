import { StatusBar, Style } from '@capacitor/status-bar'
import { Capacitor } from '@capacitor/core'

/**
 * 直接测试状态栏API，确保正确的调用顺序和参数
 * 基于官方文档: https://ionicframework.cn/docs/native/status-bar
 */

// 强制设置白色状态栏（非透明）
export const forceWhiteStatusBar = async () => {
  if (!Capacitor.isNativePlatform()) {
    console.log('⚠️ 非原生平台，跳过状态栏设置')
    return false
  }

  try {
    console.log('🔧 强制设置白色状态栏...')
    
    // 1. 首先确保状态栏不覆盖WebView (Android only)
    if (Capacitor.getPlatform() === 'android') {
      console.log('📱 设置overlaysWebView为false')
      await StatusBar.setOverlaysWebView({ overlay: false })
    }
    
    // 2. 设置状态栏样式为深色内容（适合白色背景）
    console.log('🎨 设置状态栏样式为深色')
    await StatusBar.setStyle({ style: Style.Dark })
    
    // 3. 设置白色背景 (Android only)
    if (Capacitor.getPlatform() === 'android') {
      console.log('🎨 设置状态栏背景为白色')
      await StatusBar.setBackgroundColor({ color: '#ffffff' })
    }
    
    // 4. 确保状态栏可见
    console.log('👁️ 显示状态栏')
    await StatusBar.show()
    
    // 5. 获取当前状态进行验证
    const info = await StatusBar.getInfo()
    console.log('✅ 状态栏设置完成，当前状态:', info)
    
    return true
  } catch (error) {
    console.error('❌ 强制设置白色状态栏失败:', error)
    return false
  }
}

// 强制设置黑色状态栏
export const forceBlackStatusBar = async () => {
  if (!Capacitor.isNativePlatform()) {
    console.log('⚠️ 非原生平台，跳过状态栏设置')
    return false
  }

  try {
    console.log('🔧 强制设置黑色状态栏...')
    
    // 1. 确保状态栏不覆盖WebView
    if (Capacitor.getPlatform() === 'android') {
      await StatusBar.setOverlaysWebView({ overlay: false })
    }
    
    // 2. 设置状态栏样式为浅色内容（适合黑色背景）
    await StatusBar.setStyle({ style: Style.Light })
    
    // 3. 设置黑色背景
    if (Capacitor.getPlatform() === 'android') {
      await StatusBar.setBackgroundColor({ color: '#000000' })
    }
    
    // 4. 确保状态栏可见
    await StatusBar.show()
    
    const info = await StatusBar.getInfo()
    console.log('✅ 黑色状态栏设置完成，当前状态:', info)
    
    return true
  } catch (error) {
    console.error('❌ 强制设置黑色状态栏失败:', error)
    return false
  }
}

// 测试透明状态栏（仅用于调试）
export const testTransparentStatusBar = async () => {
  if (!Capacitor.isNativePlatform()) {
    console.log('⚠️ 非原生平台，跳过状态栏设置')
    return false
  }

  try {
    console.log('🔧 测试透明状态栏...')
    
    // 1. 启用overlaysWebView让内容显示在状态栏下方
    if (Capacitor.getPlatform() === 'android') {
      console.log('📱 启用overlaysWebView')
      await StatusBar.setOverlaysWebView({ overlay: true })
      
      // 2. 设置透明背景
      console.log('🎨 设置透明背景')
      await StatusBar.setBackgroundColor({ color: '#00000000' })
    }
    
    // 3. 设置样式
    await StatusBar.setStyle({ style: Style.Dark })
    
    const info = await StatusBar.getInfo()
    console.log('✅ 透明状态栏设置完成，当前状态:', info)
    
    return true
  } catch (error) {
    console.error('❌ 透明状态栏设置失败:', error)
    return false
  }
}

// 获取当前状态栏信息
export const getStatusBarDebugInfo = async () => {
  if (!Capacitor.isNativePlatform()) {
    return { platform: 'web', message: '非原生平台' }
  }

  try {
    const info = await StatusBar.getInfo()
    const platform = Capacitor.getPlatform()
    
    return {
      platform,
      visible: info.visible,
      style: info.style,
      color: info.color,
      overlays: info.overlays,
      timestamp: new Date().toISOString()
    }
  } catch (error) {
    return { 
      platform: Capacitor.getPlatform(), 
      error: error.message 
    }
  }
}

// 重置状态栏到安全的默认状态
export const resetStatusBarToDefault = async () => {
  if (!Capacitor.isNativePlatform()) {
    return false
  }

  try {
    console.log('🔄 重置状态栏到默认状态...')
    
    // 1. 禁用overlaysWebView
    if (Capacitor.getPlatform() === 'android') {
      await StatusBar.setOverlaysWebView({ overlay: false })
    }
    
    // 2. 设置默认样式
    await StatusBar.setStyle({ style: Style.Default })
    
    // 3. 设置白色背景（如果是Android）
    if (Capacitor.getPlatform() === 'android') {
      await StatusBar.setBackgroundColor({ color: '#ffffff' })
    }
    
    // 4. 确保可见
    await StatusBar.show()
    
    console.log('✅ 状态栏已重置到默认状态')
    return true
  } catch (error) {
    console.error('❌ 重置状态栏失败:', error)
    return false
  }
} 