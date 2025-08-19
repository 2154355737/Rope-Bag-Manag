# Android 返回键处理指南

## 📱 功能概述

本项目已集成了完整的Android硬件返回键处理功能，解决了Capacitor应用中按返回键直接退出应用的问题。

## 🔧 技术实现

### 核心组件

1. **BackButtonManager** (`src/utils/backButton.ts`)
   - 管理返回键事件监听器
   - 支持多个处理器的优先级管理
   - 提供统一的返回键处理接口

2. **useBackButton Hook** (`src/hooks/use-back-button.tsx`)
   - React Hook形式的返回键处理
   - 智能路由导航逻辑
   - 双击退出功能

## 🚀 功能特性

### 智能路由导航
- **首页** (`/`, `/home`): 双击退出应用，显示提示信息
- **详情页** (`/post/*`, `/resource/*`, `/announcement/*`): 返回首页
- **认证页** (`/login`, `/register`, `/forgot-password`, `/terms`): 返回首页
- **设置页** (`/settings`, `/help`, `/about`, `/privacy`): 返回个人页面
- **其他页面**: 返回首页

### 双击退出功能
- 在首页按返回键显示"再按一次返回键退出应用"提示
- 2秒内再次按返回键退出应用
- 可配置时间间隔和开关

### 优先级管理
- 支持多个返回键处理器
- 按优先级执行（数字越大优先级越高）
- 灵活的处理器注册和移除

## 📋 使用方法

### 1. 基础使用

在任何React组件中使用：

```tsx
import { useBackButton } from '@/hooks/use-back-button'

const MyComponent = () => {
  // 基础使用，采用默认配置
  useBackButton()
  
  return <div>我的组件</div>
}
```

### 2. 自定义配置

```tsx
import { useBackButton } from '@/hooks/use-back-button'

const MyComponent = () => {
  useBackButton({
    // 自定义返回处理函数
    onBack: async () => {
      // 执行自定义逻辑
      console.log('自定义返回处理')
      // 返回 true 表示已处理，false 继续默认行为
      return false
    },
    
    // 处理器优先级（数字越大优先级越高）
    priority: 20,
    
    // 是否启用双击退出（仅在首页生效）
    enableDoubleBackExit: true,
    
    // 双击退出时间间隔（毫秒）
    doubleBackExitInterval: 3000
  })
  
  return <div>我的组件</div>
}
```

### 3. 手动处理返回键

```tsx
import { useBackButton } from '@/hooks/use-back-button'

const MyComponent = () => {
  const { triggerBack, currentPath } = useBackButton()
  
  const handleCustomBack = async () => {
    // 手动触发返回逻辑
    await triggerBack()
  }
  
  return (
    <div>
      <p>当前路径: {currentPath}</p>
      <button onClick={handleCustomBack}>手动返回</button>
    </div>
  )
}
```

### 4. 低级API使用

如果需要更精细的控制，可以直接使用底层API：

```tsx
import { addBackButtonHandler, initializeBackButton } from '@/utils/backButton'
import { useEffect } from 'react'

const MyComponent = () => {
  useEffect(() => {
    // 注册自定义处理器
    const removeHandler = addBackButtonHandler(async () => {
      console.log('自定义处理器被调用')
      // 执行自定义逻辑
      return true // 返回 true 表示已处理
    }, 15) // 优先级 15
    
    // 清理函数
    return () => {
      removeHandler()
    }
  }, [])
  
  return <div>我的组件</div>
}
```

## 🔧 配置说明

### 返回键处理器优先级

- **系统默认**: 0 (最低优先级)
- **useBackButton Hook**: 10 (默认)
- **自定义处理器**: 可自定义 (1-100)

### 路由导航规则

| 当前页面 | 返回行为 |
|---------|---------|
| `/`, `/home` | 双击退出应用 |
| `/post/*`, `/resource/*`, `/announcement/*` | 返回首页 |
| `/login`, `/register`, `/forgot-password`, `/terms` | 返回首页 |
| `/settings`, `/help`, `/about`, `/privacy` | 返回个人页面 |
| 其他页面 | 返回首页 |

## 🐛 调试功能

### 控制台日志

返回键处理过程中会输出详细的调试信息：

```
硬件返回键被按下 {canGoBack: false}
当前路径: /post/123
返回键事件已被处理
```

### 开发者工具

可以在浏览器控制台中访问全局管理器：

```javascript
// 查看当前处理器数量
console.log(window.backButtonManager?.getHandlerCount())

// 清除所有处理器
window.backButtonManager?.clearHandlers()
```

## 📱 测试方法

### 1. 开发环境测试

```bash
# 启动开发服务器
npm run dev

# 在Android设备上运行
npm run android:dev
```

### 2. 生产环境测试

```bash
# 构建应用
npm run build

# 构建Android版本
npm run build:android

# 在Android Studio中测试
```

### 3. 测试场景

1. **首页双击退出**
   - 在首页按返回键，应显示提示信息
   - 2秒内再次按返回键，应退出应用

2. **页面导航**
   - 在详情页按返回键，应返回首页
   - 在设置页按返回键，应返回个人页面

3. **优先级测试**
   - 注册多个处理器，验证优先级顺序

## ⚠️ 注意事项

1. **仅在Android原生环境生效**
   - Web环境不会触发硬件返回键事件
   - 使用 `Capacitor.isNativePlatform()` 进行平台检测

2. **处理器执行顺序**
   - 按优先级从高到低执行
   - 一旦某个处理器返回 `true`，后续处理器不会执行

3. **内存泄漏防护**
   - 组件卸载时自动移除处理器
   - 使用 `useEffect` 清理函数确保正确清理

4. **错误处理**
   - 处理器执行错误不会影响其他处理器
   - 错误信息会输出到控制台

## 🔄 更新日志

- **v1.0.0**: 初始版本，支持基础返回键处理
- **v1.1.0**: 添加智能路由导航和双击退出功能
- **v1.2.0**: 添加优先级管理和自定义处理器支持

## 📞 技术支持

如果在使用过程中遇到问题，请：

1. 检查控制台输出的调试信息
2. 确认 `@capacitor/app` 插件已正确安装
3. 验证 Capacitor 配置是否同步 (`npx cap sync`)
4. 查看本文档的常见问题部分 