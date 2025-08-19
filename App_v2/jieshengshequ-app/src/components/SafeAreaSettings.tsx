import React, { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from './ui/card'
import { Button } from './ui/button'
import { Badge } from './ui/badge'
import { Switch } from './ui/switch'
import { safeAreaManager } from '../utils/safeAreaManager'

export const SafeAreaSettings: React.FC = () => {
  const [config, setConfig] = useState(safeAreaManager.getConfig())
  const [debugMode, setDebugMode] = useState(false)

  useEffect(() => {
    const updateConfig = () => {
      setConfig(safeAreaManager.getConfig())
    }

    // 监听配置变化
    const interval = setInterval(updateConfig, 1000)
    
    return () => clearInterval(interval)
  }, [])

  const handleThemeChange = (theme: string) => {
    safeAreaManager.setTheme(theme as any)
    setConfig(safeAreaManager.getConfig())
  }

  const handleDebugToggle = (enabled: boolean) => {
    setDebugMode(enabled)
    safeAreaManager.setDebugMode(enabled)
  }

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">安全区域状态</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <div className="text-sm text-gray-600">键盘状态</div>
              <Badge variant={config.keyboardVisible ? "default" : "secondary"}>
                {config.keyboardVisible ? "显示" : "隐藏"}
              </Badge>
            </div>
          </div>
          
          <div className="pt-4 border-t">
            <div className="text-sm text-gray-600 mb-2">说明</div>
            <div className="text-xs text-gray-500 space-y-1">
              <p>• 使用CSS env()函数自动适配系统安全区域</p>
              <p>• 不再依赖原生高度计算，避免白条问题</p>
              <p>• 键盘状态用于控制导航栏显示/隐藏</p>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-lg">主题设置</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => handleThemeChange('light')}
            >
              浅色主题
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => handleThemeChange('dark')}
            >
              深色主题
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => handleThemeChange('primary')}
            >
              主题色
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => handleThemeChange('transparent')}
            >
              透明
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-lg">调试设置</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="space-y-1">
              <div className="text-sm font-medium">调试模式</div>
              <div className="text-xs text-gray-500">显示安全区域边界</div>
            </div>
            <Switch
              checked={debugMode}
              onCheckedChange={handleDebugToggle}
            />
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 