import { registerPlugin } from '@capacitor/core'

export interface NavigationBarInfo {
  // 导航栏是否存在
  hasNavigationBar: boolean
  // 导航栏高度（像素）
  navigationBarHeight: number
  // 导航栏类型：0=无, 1=传统按键, 2=手势导航
  navigationType: number
  // 导航栏是否可见
  isVisible: boolean
  // 是否为全屏模式
  isFullscreen: boolean
  // 屏幕方向
  orientation: number
  // 设备信息
  deviceInfo: {
    brand: string
    model: string
    sdkVersion: number
    hasNotch: boolean
  }
  // 调试信息
  debugInfo?: {
    screenSize: string
    realSize: string
    heightDifference: number
    widthDifference: number
    resourceNavBarHeight: number
  }
}

export interface NavigationBarPlugin {
  /**
   * 获取导航栏详细信息
   */
  getNavigationBarInfo(): Promise<NavigationBarInfo>
  
  /**
   * 监听导航栏变化
   */
  addListener(
    eventName: 'navigationBarChanged',
    listenerFunc: (info: NavigationBarInfo) => void,
  ): Promise<any>
  
  /**
   * 移除监听器
   */
  removeAllListeners(): Promise<void>
}

const NavigationBarPlugin = registerPlugin<NavigationBarPlugin>('NavigationBarPlugin', {
  web: () => import('./NavigationBarPluginWeb').then(m => new m.NavigationBarPluginWeb()),
})

export default NavigationBarPlugin 