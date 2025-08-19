import React, { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Separator } from '@/components/ui/separator'
import { Keyboard, Navigation, Eye, EyeOff } from 'lucide-react'

interface KeyboardNavSettings {
  hideNavOnKeyboard: boolean
  animationDuration: number
}

const defaultSettings: KeyboardNavSettings = {
  hideNavOnKeyboard: true,
  animationDuration: 300
}

export const KeyboardNavigationSettings: React.FC = () => {
  const [settings, setSettings] = useState<KeyboardNavSettings>(defaultSettings)

  useEffect(() => {
    // 从localStorage加载设置
    loadSettings()
  }, [])

  const loadSettings = () => {
    try {
      const saved = localStorage.getItem('keyboard-nav-settings')
      if (saved) {
        const parsedSettings = JSON.parse(saved)
        setSettings({ ...defaultSettings, ...parsedSettings })
        console.log('📱 加载键盘导航设置:', parsedSettings)
      }
    } catch (error) {
      console.error('❌ 加载键盘导航设置失败:', error)
    }
  }

  const saveSettings = (newSettings: KeyboardNavSettings) => {
    try {
      localStorage.setItem('keyboard-nav-settings', JSON.stringify(newSettings))
      console.log('💾 保存键盘导航设置:', newSettings)
      applySettings(newSettings)
    } catch (error) {
      console.error('❌ 保存键盘导航设置失败:', error)
    }
  }

  const applySettings = (newSettings: KeyboardNavSettings) => {
    const root = document.documentElement
    
    // 设置CSS变量
    root.style.setProperty('--nav-hide-animation-duration', `${newSettings.animationDuration}ms`)
    
    // 设置CSS类来控制行为
    if (newSettings.hideNavOnKeyboard) {
      document.body.classList.add('hide-nav-on-keyboard')
      document.body.classList.remove('keep-nav-on-keyboard')
    } else {
      document.body.classList.add('keep-nav-on-keyboard')
      document.body.classList.remove('hide-nav-on-keyboard')
    }
    
    console.log('🔧 应用键盘导航设置:', newSettings)
  }

  const handleSettingChange = (key: keyof KeyboardNavSettings, value: any) => {
    const newSettings = { ...settings, [key]: value }
    setSettings(newSettings)
    saveSettings(newSettings)
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base flex items-center">
          <Keyboard className="h-4 w-4 mr-2" />
          键盘导航栏设置
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        
        {/* 当前设置显示 */}
        <div className="bg-muted/50 p-3 rounded-lg">
          <div className="text-xs text-muted-foreground space-y-1">
            <div className="flex justify-between">
              <span>键盘弹出时:</span>
              <span className={settings.hideNavOnKeyboard ? "text-orange-600" : "text-green-600"}>
                {settings.hideNavOnKeyboard ? "隐藏导航栏" : "保持显示"}
              </span>
            </div>
            <div className="flex justify-between">
              <span>动画时长:</span>
              <span>{settings.animationDuration}ms</span>
            </div>
          </div>
        </div>

        {/* 隐藏导航栏开关 */}
        <div className="flex items-center justify-between">
          <div>
            <Label className="text-sm font-medium">键盘弹出时隐藏导航栏</Label>
            <p className="text-xs text-muted-foreground">
              为输入提供更多屏幕空间，提升输入体验
            </p>
          </div>
          <Switch 
            checked={settings.hideNavOnKeyboard}
            onCheckedChange={(checked) => handleSettingChange('hideNavOnKeyboard', checked)}
          />
        </div>

        <Separator />

        {/* 动画时长设置 */}
        <div className="space-y-3">
          <Label className="text-sm font-medium">动画时长</Label>
          <div className="grid grid-cols-3 gap-2">
            {[
              { label: '快速', value: 200 },
              { label: '标准', value: 300 },
              { label: '慢速', value: 500 }
            ].map((option) => (
              <button
                key={option.value}
                onClick={() => handleSettingChange('animationDuration', option.value)}
                className={`px-3 py-2 text-xs rounded border transition-colors ${
                  settings.animationDuration === option.value
                    ? 'bg-primary text-primary-foreground border-primary'
                    : 'bg-background border-border hover:bg-muted'
                }`}
              >
                {option.label}
                <br />
                <span className="text-xs opacity-70">{option.value}ms</span>
              </button>
            ))}
          </div>
        </div>

        {/* 功能说明 */}
        <div className="bg-blue-50 p-3 rounded-lg">
          <p className="text-xs text-blue-800">
            💡 <strong>功能说明:</strong>
            <br />• <strong>隐藏导航栏</strong>: 键盘弹出时完全隐藏底部导航栏，为输入提供更多空间
            <br />• <strong>保持显示</strong>: 键盘弹出时导航栏上移到键盘上方，保持可访问性
            <br />• <strong>动画时长</strong>: 控制导航栏显示/隐藏的动画速度
            <br />• 设置会自动保存并在应用重启后生效
          </p>
        </div>

        {/* 使用提示 */}
        {settings.hideNavOnKeyboard && (
          <div className="bg-orange-50 p-3 rounded-lg">
            <p className="text-xs text-orange-800">
              ⚠️ <strong>注意</strong>: 导航栏隐藏时，您需要先关闭键盘或完成输入才能访问导航功能。
              <br />可以通过点击输入框外的区域或使用设备返回键来关闭键盘。
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  )
}

export default KeyboardNavigationSettings 