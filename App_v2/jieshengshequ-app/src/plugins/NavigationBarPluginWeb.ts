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

  // 新增：Web 端空实现，同时同步 CSS 变量用于预览
  async setScrimColors(options: { statusColor?: string; navColor?: string }): Promise<void> {
    try {
      const root = document.documentElement
      if (options?.statusColor) {
        root.style.setProperty('--status-bar-scrim-color', options.statusColor)
      }
      if (options?.navColor) {
        root.style.setProperty('--android-nav-bar-color', options.navColor)
      }
    } catch (_) {
      // ignore in web
    }
    return Promise.resolve()
  }
} 