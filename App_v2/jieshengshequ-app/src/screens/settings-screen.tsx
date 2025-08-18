import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { ArrowLeft, Monitor, RotateCcw, Save, Info, Shield, Bell, HelpCircle, ChevronRight, Sun, Moon } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Slider } from '@/components/ui/slider'
import { Switch } from '@/components/ui/switch'

import { Alert, AlertDescription } from '@/components/ui/alert'
import { Separator } from '@/components/ui/separator'
import { useTheme } from '@/components/theme-provider'
import { useSafeArea } from '@/components/safe-area-provider'
import { toast } from '@/hooks/use-toast'

const SettingsScreen: React.FC = () => {
  const navigate = useNavigate()
  const { theme, setTheme } = useTheme()
  const { config, updateConfig, resetConfig } = useSafeArea()
  
  // 其他设置状态
  const [notifications, setNotifications] = useState(true)
  const [autoUpdate, setAutoUpdate] = useState(true)
  
  // 预设配置
  const presets = [
    { name: '无边距', topMargin: 0, bottomMargin: 0, autoDetect: false },
    { name: '标准边距', topMargin: 20, bottomMargin: 20, autoDetect: false },
    { name: '大边距', topMargin: 40, bottomMargin: 40, autoDetect: false },
    { name: '仅顶部', topMargin: 30, bottomMargin: 0, autoDetect: false },
    { name: '仅底部', topMargin: 0, bottomMargin: 30, autoDetect: false },
  ]
  
  // 应用预设
  const applyPreset = (preset: typeof presets[0]) => {
    updateConfig({
      topMargin: preset.topMargin,
      bottomMargin: preset.bottomMargin,
      autoDetect: preset.autoDetect
    })
  }
  
  // 保存设置
  const saveSettings = () => {
    toast({
      title: "设置已保存",
      description: "您的界面配置已成功保存",
      duration: 3000,
    })
  }
  
  // 重置设置
  const handleResetSettings = () => {
    resetConfig()
    toast({
      title: "设置已重置",
      description: "所有设置已恢复为默认值",
      duration: 3000,
    })
  }
  
  return (
    <div className="min-h-screen bg-background">
      {/* 头部导航 */}
      <div className="sticky top-0 z-50 bg-background/80 backdrop-blur-sm border-b">
        <div className="flex items-center justify-between p-4">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => navigate(-1)}
          >
            <ArrowLeft className="h-5 w-5" />
          </Button>
          <h1 className="text-lg font-semibold">设置</h1>
          <Button
            variant="ghost"
            size="sm"
            onClick={saveSettings}
            className="text-primary"
          >
            <Save className="h-4 w-4 mr-1" />
            保存
          </Button>
        </div>
      </div>

      <div className="container max-w-2xl mx-auto p-4 space-y-6">
        {/* 界面设置 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Monitor className="h-4 w-4 mr-2" />
              界面设置
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* 主题设置 */}
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                {theme === 'dark' ? <Moon size={18} className="mr-2" /> : <Sun size={18} className="mr-2" />}
                <div>
                  <Label className="text-sm font-medium">深色模式</Label>
                  <p className="text-xs text-muted-foreground">切换应用主题</p>
                </div>
              </div>
              <Switch 
                checked={theme === 'dark'}
                onCheckedChange={(checked) => setTheme(checked ? 'dark' : 'light')}
              />
            </div>
          </CardContent>
        </Card>

        {/* 安全域配置 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center justify-between">
              <div className="flex items-center">
                <Shield className="h-4 w-4 mr-2" />
                安全域配置
              </div>
              <Button
                variant="outline"
                size="sm"
                onClick={() => updateConfig({ previewMode: !config.previewMode })}
              >
                {config.previewMode ? '关闭预览' : '预览模式'}
              </Button>
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* 预览提示 */}
            {config.previewMode && (
              <Alert>
                <Info className="h-4 w-4" />
                <AlertDescription>
                  预览模式已开启，界面边缘会显示安全区域指示线
                </AlertDescription>
              </Alert>
            )}
            
            {/* 自动检测开关 */}
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-sm font-medium">自动检测安全区域</Label>
                <p className="text-xs text-muted-foreground">使用系统检测的安全区域设置</p>
              </div>
              <Switch 
                checked={config.autoDetect}
                onCheckedChange={(checked) => updateConfig({ autoDetect: checked })}
              />
            </div>
            
            {/* 手动配置区域 */}
            {!config.autoDetect && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                exit={{ opacity: 0, height: 0 }}
                className="space-y-6"
              >
                {/* 预设配置 */}
                <div>
                  <Label className="text-sm font-medium mb-3 block">快速预设</Label>
                  <div className="grid grid-cols-2 gap-2">
                    {presets.map((preset, index) => (
                      <Button
                        key={index}
                        variant="outline"
                        size="sm"
                        className="text-xs"
                        onClick={() => applyPreset(preset)}
                      >
                        {preset.name}
                      </Button>
                    ))}
                  </div>
                </div>
                
                <Separator />
                
                {/* 顶部边距 */}
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <Label className="text-sm font-medium">顶部边距</Label>
                    <span className="text-sm text-muted-foreground">{config.topMargin}px</span>
                  </div>
                  <Slider
                    value={[config.topMargin]}
                    onValueChange={(value) => updateConfig({ topMargin: value[0] })}
                    max={100}
                    step={5}
                    className="w-full"
                  />
                  <div className="flex justify-between text-xs text-muted-foreground">
                    <span>0px</span>
                    <span>50px</span>
                    <span>100px</span>
                  </div>
                </div>
                
                {/* 底部边距 */}
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <Label className="text-sm font-medium">底部边距</Label>
                    <span className="text-sm text-muted-foreground">{config.bottomMargin}px</span>
                  </div>
                  <Slider
                    value={[config.bottomMargin]}
                    onValueChange={(value) => updateConfig({ bottomMargin: value[0] })}
                    max={100}
                    step={5}
                    className="w-full"
                  />
                  <div className="flex justify-between text-xs text-muted-foreground">
                    <span>0px</span>
                    <span>50px</span>
                    <span>100px</span>
                  </div>
                </div>
              </motion.div>
            )}
            
            {/* 配置说明 */}
            <Alert>
              <Info className="h-4 w-4" />
              <AlertDescription className="text-xs">
                <strong>安全区域说明：</strong>
                <ul className="mt-1 space-y-0.5">
                  <li>• 自动检测：根据设备类型自动调整界面边距</li>
                  <li>• 手动配置：可以自定义顶部和底部的边距大小</li>
                  <li>• 预览模式：显示安全区域边界，便于调试</li>
                  <li>• 适用于刘海屏、水滴屏等异形屏设备</li>
                </ul>
              </AlertDescription>
            </Alert>
          </CardContent>
        </Card>

        {/* 通知设置 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Bell className="h-4 w-4 mr-2" />
              通知设置
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-sm font-medium">推送通知</Label>
                <p className="text-xs text-muted-foreground">接收应用推送消息</p>
              </div>
              <Switch 
                checked={notifications}
                onCheckedChange={setNotifications}
              />
            </div>
            
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-sm font-medium">自动更新</Label>
                <p className="text-xs text-muted-foreground">自动检查并安装更新</p>
              </div>
              <Switch 
                checked={autoUpdate}
                onCheckedChange={setAutoUpdate}
              />
            </div>
          </CardContent>
        </Card>



        {/* 其他设置 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <HelpCircle className="h-4 w-4 mr-2" />
              其他
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-1">
            <Button 
              variant="ghost" 
              className="w-full justify-between h-12"
              onClick={() => navigate('/help')}
            >
              <span>帮助与支持</span>
              <ChevronRight className="h-4 w-4" />
            </Button>
            <Button 
              variant="ghost" 
              className="w-full justify-between h-12"
              onClick={() => navigate('/about')}
            >
              <span>关于应用</span>
              <ChevronRight className="h-4 w-4" />
            </Button>
            <Button 
              variant="ghost" 
              className="w-full justify-between h-12"
              onClick={() => navigate('/privacy')}
            >
              <span>隐私政策</span>
              <ChevronRight className="h-4 w-4" />
            </Button>
          </CardContent>
        </Card>

        {/* 操作按钮 */}
        <div className="flex gap-3 pb-8">
          <Button
            variant="outline"
            className="flex-1"
            onClick={handleResetSettings}
          >
            <RotateCcw className="h-4 w-4 mr-1" />
            重置设置
          </Button>
          <Button
            className="flex-1"
            onClick={saveSettings}
          >
            <Save className="h-4 w-4 mr-1" />
            保存设置
          </Button>
        </div>
      </div>
    </div>
  )
}

export default SettingsScreen 