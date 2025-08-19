import { applyStatusBarConfig } from './statusBar'
import { applyAndroidNavigationBarConfig } from './navigationBarNative'
import { isAndroid, isNative } from './platform'

// 测试状态栏和导航栏功能
export const testStatusBarAndNavBarFunctions = async () => {
  console.log('🧪 开始测试状态栏和导航栏功能...')
  
  if (!isNative()) {
    console.log('⚠️ 非原生平台，跳过测试')
    return
  }

  try {
    // 测试状态栏
    console.log('📱 测试状态栏功能...')
    
    // 设置白色背景，深色文本
    await applyStatusBarConfig({
      style: 'dark',
      backgroundColor: '#ffffff',
      visible: true,
      overlaysWebView: false
    })
    console.log('✅ 状态栏白色背景设置完成')
    
    // 等待2秒
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    // 设置蓝色背景，浅色文本
    await applyStatusBarConfig({
      style: 'light',
      backgroundColor: '#2563eb',
      visible: true,
      overlaysWebView: false
    })
    console.log('✅ 状态栏蓝色背景设置完成')
    
    // 测试Android导航栏
    if (isAndroid()) {
      console.log('🤖 测试Android导航栏功能...')
      
      // 等待2秒
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      // 设置白色导航栏
      await applyAndroidNavigationBarConfig({
        backgroundColor: '#ffffff',
        buttonColor: 'dark',
        hidden: false,
        overlaysContent: false
      })
      console.log('✅ Android导航栏白色背景设置完成')
      
      // 等待2秒
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      // 设置绿色导航栏
      await applyAndroidNavigationBarConfig({
        backgroundColor: '#16a34a',
        buttonColor: 'light',
        hidden: false,
        overlaysContent: false
      })
      console.log('✅ Android导航栏绿色背景设置完成')
    }
    
    console.log('🎉 所有测试完成！')
    
  } catch (error) {
    console.error('❌ 测试过程中出现错误:', error)
  }
}

// 恢复默认设置
export const resetToDefaults = async () => {
  console.log('🔄 恢复默认设置...')
  
  try {
    // 恢复状态栏默认设置
    await applyStatusBarConfig({
      style: 'dark',
      backgroundColor: '#ffffff',
      visible: true,
      overlaysWebView: false
    })
    
    // 恢复Android导航栏默认设置
    if (isAndroid()) {
      await applyAndroidNavigationBarConfig({
        backgroundColor: '#ffffff',
        buttonColor: 'dark',
        hidden: false,
        overlaysContent: false
      })
    }
    
    console.log('✅ 默认设置恢复完成')
  } catch (error) {
    console.error('❌ 恢复默认设置失败:', error)
  }
}

// 暴露到全局对象供调试使用
if (typeof window !== 'undefined' && import.meta.env.DEV) {
  const globalWindow = window as any
  globalWindow.testStatusBarAndNavBar = testStatusBarAndNavBarFunctions
  globalWindow.resetStatusBarAndNavBar = resetToDefaults
  console.log('🔧 测试函数已暴露到全局：testStatusBarAndNavBar(), resetStatusBarAndNavBar()')
} 