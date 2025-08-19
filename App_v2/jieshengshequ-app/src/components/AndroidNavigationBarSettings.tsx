import React, { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { Smartphone, Palette } from 'lucide-react'
import { isAndroid, isNative } from '@/utils/platform'
import { 
  AndroidNavigationBarConfig,
  androidNavBarPresets,
  applyAndroidNavigationBarConfig,
  applyAndroidNavBarPreset,
  setAndroidNavigationBarColor,
  getAndroidNavigationBarHeight,
  hasHardwareNavigationButtons
} from '@/utils/navigationBarNative'

export const AndroidNavigationBarSettings: React.FC = () => {
  const [config, setConfig] = useState<AndroidNavigationBarConfig>({
    backgroundColor: '#ffffff',
    buttonColor: 'dark',
    hidden: false,
    overlaysContent: false
  })

  const [currentColor, setCurrentColor] = useState('#ffffff')
  const [isNativePlatform, setIsNativePlatform] = useState(false)
  const [isAndroidPlatform, setIsAndroidPlatform] = useState(false)
  const [hasHardwareButtons, setHasHardwareButtons] = useState(false)
  const [navBarHeight, setNavBarHeight] = useState(0)

  const loadDeviceInfo = async () => {
    try {
      const hasHardware = await hasHardwareNavigationButtons()
      const height = await getAndroidNavigationBarHeight()
      
      setHasHardwareButtons(hasHardware)
      setNavBarHeight(height)
      
      console.log('📱 Android导航栏设备信息:', {
        hasHardwareButtons: hasHardware,
        estimatedHeight: height
      })
    } catch (error) {
      console.error('❌ 获取设备信息失败:', error)
    }
  }

  useEffect(() => {
    const nativePlatform = isNative()
    const androidPlatform = isAndroid()
    
    setIsNativePlatform(nativePlatform)
    setIsAndroidPlatform(androidPlatform)
    
    if (androidPlatform) {
      loadDeviceInfo()
    }
  }, [])

  const handleConfigChange = async (newConfig: Partial<AndroidNavigationBarConfig>) => {
    const updatedConfig = { ...config, ...newConfig }
    setConfig(updatedConfig)
    
    if (isAndroidPlatform) {
      await applyAndroidNavigationBarConfig(updatedConfig)
    }
  }

  const handlePresetApply = async (presetName: keyof typeof androidNavBarPresets) => {
    const success = await applyAndroidNavBarPreset(presetName)
    if (success) {
      const preset = androidNavBarPresets[presetName]
      setConfig(preset)
      setCurrentColor(preset.backgroundColor)
    }
  }

  const handleColorChange = async (color: string) => {
    setCurrentColor(color)
    await handleConfigChange({ backgroundColor: color })
  }

  const colorPresets = [
    { name: '白色', color: '#ffffff' },
    { name: '黑色', color: '#000000' },
    { name: '透明', color: '#00000000' },
    { name: '蓝色', color: '#2563eb' },
    { name: '绿色', color: '#16a34a' },
    { name: '红色', color: '#dc2626' }
  ]

  if (!isAndroidPlatform) {
    return (
      <Card className="opacity-50">
        <CardHeader>
          <CardTitle className="text-base flex items-center">
            <Smartphone className="h-4 w-4 mr-2" />
            Android导航栏设置
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            仅在Android设备上可用
          </p>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base flex items-center">
          <Smartphone className="h-4 w-4 mr-2" />
          Android导航栏设置
          <span className="ml-2 text-xs bg-green-100 text-green-800 px-2 py-1 rounded">
            Android
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* 设备信息显示 */}
        <div className="bg-muted/50 p-3 rounded-lg">
          <div className="text-xs text-muted-foreground space-y-1">
            <div className="flex justify-between">
              <span>导航类型:</span>
              <span className="text-blue-600">
                {hasHardwareButtons ? '硬件按键' : '软件导航栏'}
              </span>
            </div>
            <div className="flex justify-between">
              <span>估算高度:</span>
              <span>{navBarHeight}px</span>
            </div>
            <div className="flex justify-between">
              <span>当前颜色:</span>
              <div className="flex items-center space-x-2">
                <div 
                  className="w-4 h-4 rounded border border-border" 
                  style={{ backgroundColor: currentColor }}
                />
                <span className="font-mono text-xs">{currentColor}</span>
              </div>
            </div>
          </div>
        </div>

        <Separator />

        {/* 按钮样式选择 */}
        <div className="space-y-3">
          <Label className="text-sm font-medium">按钮样式</Label>
          <div className="grid grid-cols-3 gap-2">
            {(['dark', 'light', 'default'] as const).map((style) => (
              <Button
                key={style}
                variant={config.buttonColor === style ? 'default' : 'outline'}
                size="sm"
                className="text-xs"
                onClick={() => handleConfigChange({ buttonColor: style })}
              >
                {style === 'dark' ? '深色' : style === 'light' ? '浅色' : '默认'}
              </Button>
            ))}
          </div>
        </div>

        <Separator />

        {/* 背景颜色设置 */}
        <div className="space-y-3">
          <Label className="text-sm font-medium flex items-center">
            <Palette className="h-4 w-4 mr-1" />
            背景颜色
          </Label>
          
          {/* 颜色输入 */}
          <div className="flex items-center space-x-2">
            <input
              type="color"
              value={currentColor.length === 7 ? currentColor : '#ffffff'}
              onChange={(e) => handleColorChange(e.target.value)}
              className="w-8 h-8 rounded border border-border cursor-pointer"
            />
            <input
              type="text"
              value={currentColor}
              onChange={(e) => handleColorChange(e.target.value)}
              placeholder="#ffffff"
              className="flex-1 px-2 py-1 text-xs border border-border rounded font-mono"
            />
          </div>

          {/* 颜色预设 */}
          <div className="grid grid-cols-3 gap-2">
            {colorPresets.map((preset) => (
              <Button
                key={preset.color}
                variant="outline"
                size="sm"
                className="text-xs h-8 flex items-center space-x-1"
                onClick={() => handleColorChange(preset.color)}
              >
                <div 
                  className="w-3 h-3 rounded border border-border" 
                  style={{ backgroundColor: preset.color }}
                />
                <span>{preset.name}</span>
              </Button>
            ))}
          </div>
        </div>

        {/* 预设配置 */}
        <div className="space-y-3">
          <Label className="text-sm font-medium">快速预设</Label>
          <div className="grid grid-cols-2 gap-2">
            {Object.entries(androidNavBarPresets).map(([key, preset]) => (
              <Button
                key={key}
                variant="outline"
                size="sm"
                className="text-xs"
                onClick={() => handlePresetApply(key as keyof typeof androidNavBarPresets)}
              >
                {key === 'light' ? '浅色主题' : 
                 key === 'dark' ? '深色主题' : 
                 key === 'transparent' ? '透明' : '沉浸式'}
              </Button>
            ))}
          </div>
        </div>

        {/* 说明 */}
        <div className="bg-orange-50 p-3 rounded-lg">
          <p className="text-xs text-orange-800">
            ⚠️ <strong>说明:</strong>
            <br />• 仅适用于软件导航栏（虚拟按键）
            <br />• 硬件按键设备无法自定义导航栏样式
            <br />• 沉浸式模式可隐藏导航栏创建全屏体验
          </p>
        </div>

        {hasHardwareButtons && (
          <div className="bg-yellow-50 p-3 rounded-lg">
            <p className="text-xs text-yellow-800">
              📱 <strong>硬件按键设备:</strong>
              <br />检测到此设备使用硬件导航按键，部分设置可能不会生效。
              硬件按键设备通常不支持自定义导航栏样式。
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  )
}

export default AndroidNavigationBarSettings 