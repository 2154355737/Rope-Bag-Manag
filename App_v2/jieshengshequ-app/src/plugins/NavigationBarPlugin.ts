import { registerPlugin } from '@capacitor/core'
import type { PluginListenerHandle, ListenerCallback } from '@capacitor/core'

export interface NavigationBarInfo {
  hasNavigationBar: boolean
  navigationBarHeight: number
  navigationType: number
  isVisible: boolean
  isFullscreen: boolean
  orientation: number
  deviceInfo: {
    brand: string
    model: string
    sdkVersion: number
    hasNotch: boolean
  }
}

export interface NavigationBarPlugin {
  getNavigationBarInfo(): Promise<NavigationBarInfo>
  addListener(eventName: string, listenerFunc: ListenerCallback): Promise<PluginListenerHandle>
  removeAllListeners(): Promise<void>
}

const NavigationBarPlugin = registerPlugin<NavigationBarPlugin>('NavigationBarPlugin', {
  web: () => import('./NavigationBarPluginWeb').then(m => new m.NavigationBarPluginWeb()),
})

export default NavigationBarPlugin 