import React, { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { Monitor, Eye, EyeOff, Palette } from 'lucide-react'
import { isAndroid, isNative } from '@/utils/platform'
import { 
  StatusBarConfig, 
  statusBarPresets, 
  applyStatusBarConfig, 
  applyStatusBarPreset,
  getStatusBarInfo,
  setStatusBarColor
} from '@/utils/statusBar'

export const StatusBarSettings: React.FC = () => {
  const [config, setConfig] = useState<StatusBarConfig>({
    style: 'dark',
    backgroundColor: '#ffffff',
    visible: true,
    overlaysWebView: false
  })

  const [isStatusBarVisible, setIsStatusBarVisible] = useState(true)
  const [currentColor, setCurrentColor] = useState('#ffffff')
  const [isNativePlatform, setIsNativePlatform] = useState(false)
  const [isAndroidPlatform, setIsAndroidPlatform] = useState(false)

  useEffect(() => {
    setIsNativePlatform(isNative())
    setIsAndroidPlatform(isAndroid())
    
    // 获取当前状态栏信息
    if (isNative()) {
      loadCurrentStatusBarInfo()
    }
  }, [])

  const loadCurrentStatusBarInfo = async () => {
    try {
      const info = await getStatusBarInfo()
      if (info) {
        setIsStatusBarVisible(info.visible)
        if (info.color) {
          setCurrentColor(info.color)
        }
        console.log('📱 当前状态栏信息:', info)
      }
    } catch (error) {
      console.error('❌ 获取状态栏信息失败:', error)
    }
  }

  const handleConfigChange = async (newConfig: Partial<StatusBarConfig>) => {
    const updatedConfig = { ...config, ...newConfig }
    setConfig(updatedConfig)
    
    if (isNativePlatform) {
      await applyStatusBarConfig(updatedConfig)
      await loadCurrentStatusBarInfo()
    }
  }

  const handlePresetApply = async (presetName: keyof typeof statusBarPresets) => {
    const success = await applyStatusBarPreset(presetName)
    if (success) {
      const preset = statusBarPresets[presetName]
      setConfig(preset)
      setCurrentColor(preset.backgroundColor)
      await loadCurrentStatusBarInfo()
    }
  }

  const handleColorChange = async (color: string) => {
    setCurrentColor(color)
    // 同时更新配置状态
    const updatedConfig = { ...config, backgroundColor: color }
    setConfig(updatedConfig)
    
    await setStatusBarColor(color, config.style)
    await loadCurrentStatusBarInfo()
  }

  const colorPresets = [
    { name: '白色', color: '#ffffff' },
    { name: '黑色', color: '#000000' },
    { name: '透明', color: '#00000000' },
    { name: '蓝色', color: '#2563eb' },
    { name: '绿色', color: '#16a34a' },
    { name: '红色', color: '#dc2626' }
  ]

  if (!isNativePlatform) {
    return (
      <Card className="opacity-50">
        <CardHeader>
          <CardTitle className="text-base flex items-center">
            <Monitor className="h-4 w-4 mr-2" />
            状态栏设置
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            仅在原生应用中可用
          </p>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base flex items-center">
          <Monitor className="h-4 w-4 mr-2" />
          状态栏设置
          {isAndroidPlatform && (
            <span className="ml-2 text-xs bg-green-100 text-green-800 px-2 py-1 rounded">
              Android
            </span>
          )}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        
        {/* 当前状态显示 */}
        <div className="bg-muted/50 p-3 rounded-lg">
          <div className="text-xs text-muted-foreground space-y-1">
            <div className="flex justify-between">
              <span>可见性:</span>
              <span className={isStatusBarVisible ? "text-green-600" : "text-red-600"}>
                {isStatusBarVisible ? "显示" : "隐藏"}
              </span>
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

        {/* 可见性控制 */}
        <div className="flex items-center justify-between">
          <div>
            <Label className="text-sm font-medium">显示状态栏</Label>
            <p className="text-xs text-muted-foreground">控制状态栏的显示和隐藏</p>
          </div>
          <Switch 
            checked={config.visible}
            onCheckedChange={(visible) => handleConfigChange({ visible })}
          />
        </div>

        <Separator />

        {/* 样式选择 */}
        <div className="space-y-3">
          <Label className="text-sm font-medium">文本/图标样式</Label>
          <div className="grid grid-cols-3 gap-2">
            {(['dark', 'light', 'default'] as const).map((style) => (
              <Button
                key={style}
                variant={config.style === style ? "default" : "outline"}
                size="sm"
                className="text-xs"
                onClick={() => handleConfigChange({ style })}
              >
                {style === 'dark' ? '深色' : style === 'light' ? '浅色' : '默认'}
              </Button>
            ))}
          </div>
        </div>

        {isAndroidPlatform && (
          <>
            <Separator />
            
            {/* Android专用设置 */}
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-sm font-medium">覆盖WebView</Label>
                <p className="text-xs text-muted-foreground">允许内容延伸到状态栏下方</p>
              </div>
              <Switch 
                checked={config.overlaysWebView || false}
                onCheckedChange={(overlaysWebView) => handleConfigChange({ overlaysWebView })}
              />
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
          </>
        )}

        <Separator />

        {/* 预设配置 */}
        <div className="space-y-3">
          <Label className="text-sm font-medium">快速预设</Label>
          <div className="grid grid-cols-2 gap-2">
            {Object.entries(statusBarPresets).map(([key, preset]) => (
              <Button
                key={key}
                variant="outline"
                size="sm"
                className="text-xs"
                onClick={() => handlePresetApply(key as keyof typeof statusBarPresets)}
              >
                {key === 'light' ? '浅色主题' : 
                 key === 'dark' ? '深色主题' : 
                 key === 'transparent' ? '透明' : '沉浸式'}
              </Button>
            ))}
          </div>
        </div>

        {/* 使用说明 */}
        <div className="bg-blue-50 p-3 rounded-lg">
          <p className="text-xs text-blue-800">
            💡 <strong>使用提示:</strong>
            <br />• 深色样式适合浅色背景
            <br />• 浅色样式适合深色背景  
            <br />• 透明背景可创建沉浸式体验
            {isAndroidPlatform && (
              <>
                <br />• Android设备支持自定义背景颜色
                <br />• 覆盖WebView模式可让内容填满屏幕
              </>
            )}
          </p>
        </div>
      </CardContent>
    </Card>
  )
}

export default StatusBarSettings 