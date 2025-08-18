import { WebPlugin } from '@capacitor/core'
import type { NavigationBarPlugin, NavigationBarInfo } from './NavigationBarPlugin'

export class NavigationBarPluginWeb extends WebPlugin implements NavigationBarPlugin {
  
  async getNavigationBarInfo(): Promise<NavigationBarInfo> {
    // Web端返回模拟数据
    return {
      hasNavigationBar: false,
      navigationBarHeight: 0,
      navigationType: 0, // 无导航栏
      isVisible: false,
      isFullscreen: false,
      orientation: 0,
      deviceInfo: {
        brand: 'Web',
        model: 'Browser',
        sdkVersion: 0,
        hasNotch: false
      }
    }
  }

  async addListener(
    eventName: 'navigationBarChanged',
    listenerFunc: (info: NavigationBarInfo) => void,
  ): Promise<any> {
    // Web端不支持监听
    return Promise.resolve()
  }

  async removeAllListeners(): Promise<void> {
    return Promise.resolve()
  }
} 