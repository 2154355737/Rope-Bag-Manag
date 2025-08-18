import NavigationBarPlugin, { NavigationBarInfo } from '../plugins/NavigationBarPlugin'
import { NavigationType, NavigationBarInfo as AppNavigationBarInfo } from './navigationBar'

/**
 * 使用原生插件获取导航栏信息
 */
export const detectNavigationBarWithNativePlugin = async (): Promise<AppNavigationBarInfo> => {
  try {
    console.log('🔧 使用原生插件检测导航栏...')
    
    const nativeInfo: NavigationBarInfo = await NavigationBarPlugin.getNavigationBarInfo()
    
    console.log('📱 原生插件返回数据:', nativeInfo)
    
    // 转换原生数据到应用格式
    let navigationType: NavigationType
    switch (nativeInfo.navigationType) {
      case 0:
        navigationType = NavigationType.NONE
        break
      case 1:
        navigationType = NavigationType.BUTTONS
        break
      case 2:
        navigationType = NavigationType.GESTURE
        break
      default:
        navigationType = NavigationType.BUTTONS
    }
    
    const result: AppNavigationBarInfo = {
      type: navigationType,
      height: nativeInfo.navigationBarHeight,
      isVisible: nativeInfo.isVisible,
      hasHomeIndicator: navigationType === NavigationType.GESTURE && nativeInfo.navigationBarHeight > 0
    }
    
    console.log('✅ 转换后的导航栏信息:', result)
    console.log('🔍 详细调试信息:', {
      原生数据: nativeInfo,
      设备信息: nativeInfo.deviceInfo,
      调试信息: nativeInfo.debugInfo
    })
    
    return result
    
  } catch (error) {
    console.error('❌ 原生插件检测失败:', error)
    
    // 降级到原来的检测方法
    const { detectNavigationBar } = await import('./navigationBar')
    return await detectNavigationBar()
  }
}

/**
 * 监听导航栏变化
 */
export const watchNavigationBarWithNativePlugin = (
  callback: (info: AppNavigationBarInfo) => void
) => {
  try {
    NavigationBarPlugin.addListener('navigationBarChanged', async (nativeInfo) => {
      console.log('📱 导航栏发生变化:', nativeInfo)
      
      // 转换并回调
      let navigationType: NavigationType
      switch (nativeInfo.navigationType) {
        case 0:
          navigationType = NavigationType.NONE
          break
        case 1:
          navigationType = NavigationType.BUTTONS
          break
        case 2:
          navigationType = NavigationType.GESTURE
          break
        default:
          navigationType = NavigationType.BUTTONS
      }
      
      const result: AppNavigationBarInfo = {
        type: navigationType,
        height: nativeInfo.navigationBarHeight,
        isVisible: nativeInfo.isVisible,
        hasHomeIndicator: navigationType === NavigationType.GESTURE && nativeInfo.navigationBarHeight > 0
      }
      
      callback(result)
    })
  } catch (error) {
    console.error('❌ 监听导航栏变化失败:', error)
  }
} 