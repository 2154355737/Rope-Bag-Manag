import React, { useState, useEffect } from 'react'
import { detectNavigationBar, setNavigationBarCSSVariables, NavigationType, NavigationBarInfo } from '@/utils/navigationBar'
import { getPlatform, isNative } from '@/utils/platform'
import { Device } from '@capacitor/device'
import { StatusBar } from '@capacitor/status-bar'
import { Card, CardContent } from '@/components/ui/card'

interface NavigationDebugPanelProps {
  show?: boolean
}

const NavigationDebugPanel: React.FC<NavigationDebugPanelProps> = ({ show = false }) => {
  const [navInfo, setNavInfo] = useState<NavigationBarInfo | null>(null)
  const [deviceInfo, setDeviceInfo] = useState<any>(null)
  const [statusBarInfo, setStatusBarInfo] = useState<any>(null)
  const [screenInfo, setScreenInfo] = useState({
    screenHeight: 0,
    availableHeight: 0,
    windowHeight: 0,
    innerHeight: 0,
    safeAreaBottom: 0,
    safeAreaTop: 0
  })

  // 添加手动覆盖状态
  const [manualOverride, setManualOverride] = useState<{
    enabled: boolean
    type: NavigationType
    height: number
  }>({
    enabled: false,
    type: NavigationType.NONE,
    height: 0
  })

  useEffect(() => {
    const updateInfo = async () => {
      const info = await detectNavigationBar()
      setNavInfo(info)
      
      // 获取设备信息
      try {
        const device = await Device.getInfo()
        setDeviceInfo(device)
      } catch (error) {
        console.warn('无法获取设备信息:', error)
      }
      
      // 获取状态栏信息
      try {
        const statusBar = await StatusBar.getInfo()
        setStatusBarInfo(statusBar)
      } catch (error) {
        console.warn('无法获取状态栏信息:', error)
      }
      
      // 获取屏幕信息
      const safeAreaBottom = parseFloat(
        getComputedStyle(document.documentElement)
          .getPropertyValue('--safe-area-inset-bottom')
          .replace('px', '')
      ) || 0
      
      const safeAreaTop = parseFloat(
        getComputedStyle(document.documentElement)
          .getPropertyValue('--safe-area-inset-top')
          .replace('px', '')
      ) || 0
      
      setScreenInfo({
        screenHeight: window.screen.height,
        availableHeight: window.screen.availHeight,
        windowHeight: window.outerHeight,
        innerHeight: window.innerHeight,
        safeAreaBottom,
        safeAreaTop
      })
    }

    if (show && isNative()) {
      updateInfo()
      
      // 监听窗口变化
      const handleResize = () => updateInfo()
      const handleOrientationChange = () => setTimeout(updateInfo, 100)
      
      window.addEventListener('resize', handleResize)
      window.addEventListener('orientationchange', handleOrientationChange)
      
      return () => {
        window.removeEventListener('resize', handleResize)
        window.removeEventListener('orientationchange', handleOrientationChange)
      }
    }
  }, [show])

  // 手动设置导航栏
  const handleManualOverride = (type: NavigationType, height: number) => {
    const navInfo = {
      type,
      height,
      isVisible: type !== NavigationType.NONE,
      hasHomeIndicator: type === NavigationType.GESTURE && height > 0
    }
    
    setNavigationBarCSSVariables(navInfo)
    setManualOverride({ enabled: true, type, height })
    
    console.log('🔧 手动覆盖导航栏设置:', navInfo)
  }

  // 恢复自动检测
  const handleResetToAuto = async () => {
    setManualOverride({ enabled: false, type: NavigationType.NONE, height: 0 })
    const navInfo = await detectNavigationBar()
    setNavigationBarCSSVariables(navInfo)
    console.log('🔄 恢复自动检测:', navInfo)
  }

  if (!show || !isNative() || !navInfo) {
    return null
  }

  return (
    <div className={`fixed top-2 right-2 bg-black/80 text-white p-3 rounded-lg text-xs max-w-xs z-50 ${show ? 'block' : 'hidden'}`}>
      {/* 现有的调试信息显示 */}
      <div className="mb-2">
        <div className="font-bold text-green-400">🛠️ 导航栏调试面板</div>
        <Card className="bg-black/80 text-white text-xs">
          <CardContent className="p-3">
            <h3 className="font-bold mb-2">导航栏调试信息</h3>
            
            <div className="space-y-1">
              <div>平台: {getPlatform()}</div>
              <div>导航类型: {navInfo.type}</div>
              <div>导航高度: {navInfo.height}px</div>
              <div>是否可见: {navInfo.isVisible ? '是' : '否'}</div>
              <div>Home指示器: {navInfo.hasHomeIndicator ? '是' : '否'}</div>
            </div>
            
            <hr className="my-2 border-gray-600" />
            
            <div className="space-y-1">
              <div>屏幕高度: {screenInfo.screenHeight}px</div>
              <div>可用高度: {screenInfo.availableHeight}px</div>
              <div>窗口高度: {screenInfo.windowHeight}px</div>
              <div>内部高度: {screenInfo.innerHeight}px</div>
              <div>顶部安全区: {screenInfo.safeAreaTop}px</div>
              <div>底部安全区: {screenInfo.safeAreaBottom}px</div>
            </div>
            
            {deviceInfo && (
              <>
                <hr className="my-2 border-gray-600" />
                <div className="space-y-1">
                  <div className="font-semibold">设备信息</div>
                  <div>型号: {deviceInfo.model}</div>
                  <div>系统: {deviceInfo.operatingSystem} {deviceInfo.osVersion}</div>
                  <div>制造商: {deviceInfo.manufacturer}</div>
                </div>
              </>
            )}
            
            {statusBarInfo && (
              <>
                <hr className="my-2 border-gray-600" />
                <div className="space-y-1">
                  <div className="font-semibold">状态栏信息</div>
                  <div>可见: {statusBarInfo.visible ? '是' : '否'}</div>
                  <div>样式: {statusBarInfo.style}</div>
                </div>
              </>
            )}
            
            <hr className="my-2 border-gray-600" />
            
            <div className="space-y-1">
              <div className="font-semibold">计算差值</div>
              <div>屏幕差值: {screenInfo.screenHeight - screenInfo.availableHeight}px</div>
              <div>窗口差值: {screenInfo.screenHeight - screenInfo.innerHeight}px</div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* 手动覆盖控制 */}
      <div className="border-t border-gray-600 pt-2 mt-2">
        <div className="font-bold text-yellow-400 mb-2">🔧 手动覆盖</div>
        {manualOverride.enabled && (
          <div className="text-orange-400 mb-2">
            ⚠️ 当前使用手动设置
          </div>
        )}
        <div className="space-y-1">
          <button 
            className="w-full bg-blue-600 hover:bg-blue-700 px-2 py-1 rounded text-xs"
            onClick={() => handleManualOverride(NavigationType.BUTTONS, 48)}
          >
            🔘 按键导航 (48px)
          </button>
          <button 
            className="w-full bg-green-600 hover:bg-green-700 px-2 py-1 rounded text-xs"
            onClick={() => handleManualOverride(NavigationType.GESTURE, 12)}
          >
            ✋ 手势导航 (12px)
          </button>
          <button 
            className="w-full bg-gray-600 hover:bg-gray-700 px-2 py-1 rounded text-xs"
            onClick={() => handleManualOverride(NavigationType.NONE, 0)}
          >
            🚫 无导航栏 (0px)
          </button>
          <button 
            className="w-full bg-red-600 hover:bg-red-700 px-2 py-1 rounded text-xs"
            onClick={handleResetToAuto}
          >
            🔄 恢复自动检测
          </button>
        </div>
      </div>
    </div>
  )
}

export default NavigationDebugPanel 