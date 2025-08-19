import { WebPlugin } from '@capacitor/core'
import type { NavigationBarPlugin, NavigationBarInfo } from './NavigationBarPlugin'
import type { PluginListenerHandle, ListenerCallback } from '@capacitor/core'

export class NavigationBarPluginWeb extends WebPlugin implements NavigationBarPlugin {
  async getNavigationBarInfo(): Promise<NavigationBarInfo> {
    // Web平台返回空的导航栏信息
    return {
      hasNavigationBar: false,
      navigationBarHeight: 0,
      navigationType: 0,
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

  async addListener(eventName: string, listenerFunc: ListenerCallback): Promise<PluginListenerHandle> {
    // Web平台不需要监听器，返回空的handle
    return Promise.resolve({
      remove: async () => {}
    })
  }

  async removeAllListeners(): Promise<void> {
    // Web平台不需要移除监听器
    return Promise.resolve()
  }
} 