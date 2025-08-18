# 结绳社区移动应用开发指南 🚀

本指南将帮助您快速上手结绳社区移动应用的开发工作，包含详细的环境搭建、开发流程、代码规范和最佳实践。

## 📋 目录

- [环境搭建](#环境搭建)
- [项目结构详解](#项目结构详解)
- [开发流程](#开发流程)
- [代码规范](#代码规范)
- [组件开发](#组件开发)
- [状态管理](#状态管理)
- [路由配置](#路由配置)
- [样式开发](#样式开发)
- [移动端适配](#移动端适配)
- [调试技巧](#调试技巧)
- [性能优化](#性能优化)
- [部署发布](#部署发布)
- [常见问题](#常见问题)

## 🛠️ 环境搭建

### 基础环境要求

#### 必需软件
- **Node.js** >= 18.0.0 ([下载地址](https://nodejs.org/))
- **npm** >= 8.0.0 (随 Node.js 安装)
- **Git** 版本控制工具
- **VS Code** 推荐代码编辑器

#### Android 开发环境 (可选)
- **Java JDK 21** ([下载地址](https://adoptium.net/))
- **Android Studio** ([下载地址](https://developer.android.com/studio))
- **Android SDK** (通过 Android Studio 安装)

### 环境变量配置

#### Windows 系统
```powershell
# 设置 JAVA_HOME
$env:JAVA_HOME = "C:\Program Files\Eclipse Adoptium\jdk-21.0.1.12-hotspot"

# 设置 ANDROID_HOME
$env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk"
$env:ANDROID_SDK_ROOT = "$env:ANDROID_HOME"

# 添加到 PATH
$env:PATH = "$env:JAVA_HOME\bin;$env:ANDROID_HOME\tools;$env:ANDROID_HOME\tools\bin;$env:ANDROID_HOME\platform-tools;$env:PATH"
```

#### macOS/Linux 系统
```bash
# 在 ~/.bashrc 或 ~/.zshrc 中添加
export JAVA_HOME="/Library/Java/JavaVirtualMachines/temurin-21.jdk/Contents/Home"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export PATH="$JAVA_HOME/bin:$ANDROID_HOME/tools:$ANDROID_HOME/tools/bin:$ANDROID_HOME/platform-tools:$PATH"
```

### VS Code 推荐插件

```json
{
  "recommendations": [
    "ms-vscode.vscode-typescript-next",
    "bradlc.vscode-tailwindcss",
    "dsznajder.es7-react-js-snippets",
    "formulahendry.auto-rename-tag",
    "esbenp.prettier-vscode",
    "ms-vscode.vscode-json",
    "christian-kohler.path-intellisense",
    "bradlc.vscode-tailwindcss",
    "formulahendry.auto-close-tag"
  ]
}
```

### 项目初始化

```bash
# 1. 克隆项目
git clone <repository-url>
cd App_v2/jieshengshequ-app

# 2. 安装依赖
npm install

# 3. 启动开发服务器
npm run dev

# 4. (可选) 同步 Capacitor 配置
npm run sync
```

## 📁 项目结构详解

```
jieshengshequ-app/
├── android/                          # Android 原生代码
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── java/                 # Java 源代码
│   │   │   ├── res/                  # 资源文件
│   │   │   └── AndroidManifest.xml   # 应用清单
│   │   └── build.gradle              # 应用构建配置
│   └── build.gradle                  # 项目构建配置
├── src/
│   ├── components/                   # 可复用组件
│   │   ├── ui/                       # 基础 UI 组件
│   │   │   ├── button.tsx            # 按钮组件
│   │   │   ├── input.tsx             # 输入框组件
│   │   │   ├── card.tsx              # 卡片组件
│   │   │   └── ...                   # 其他 UI 组件
│   │   ├── layout.tsx                # 布局组件
│   │   ├── theme-provider.tsx        # 主题提供者
│   │   └── NavigationDebugPanel.tsx  # 调试面板
│   ├── screens/                      # 页面组件
│   │   ├── splash-screen.tsx         # 启动页
│   │   ├── onboarding-screen.tsx     # 引导页
│   │   ├── home-screen.tsx           # 首页
│   │   ├── category-screen.tsx       # 分类页
│   │   ├── community-screen.tsx      # 社区页
│   │   ├── messages-screen.tsx       # 消息页
│   │   ├── profile-screen.tsx        # 个人页
│   │   └── post-detail-screen.tsx    # 帖子详情页
│   ├── hooks/                        # 自定义 Hooks
│   │   ├── use-mobile.tsx            # 移动端检测
│   │   └── use-toast.ts              # 提示消息
│   ├── lib/                          # 工具库
│   │   └── utils.ts                  # 通用工具函数
│   ├── plugins/                      # Capacitor 插件
│   │   ├── NavigationBarPlugin.ts    # 导航栏插件
│   │   └── NavigationBarPluginWeb.ts # Web 端导航栏插件
│   ├── styles/                       # 样式文件
│   │   ├── safe-area.css             # 安全区域样式
│   │   └── theme.css                 # 主题样式
│   ├── utils/                        # 工具函数
│   │   ├── navigationBar.ts          # 导航栏工具
│   │   ├── navigationBarNative.ts    # 原生导航栏工具
│   │   ├── platform.ts               # 平台检测
│   │   └── statusBar.ts              # 状态栏工具
│   ├── App.tsx                       # 应用入口
│   ├── main.tsx                      # 主入口文件
│   └── vite-env.d.ts                # Vite 类型定义
├── capacitor.config.ts               # Capacitor 配置
├── components.json                   # shadcn/ui 配置
├── package.json                      # 项目依赖
├── tailwind.config.ts               # Tailwind 配置
├── tsconfig.json                    # TypeScript 配置
├── vite.config.ts                   # Vite 配置
└── postcss.config.js                # PostCSS 配置
```

## 🔄 开发流程

### 1. 功能开发流程

```bash
# 1. 创建功能分支
git checkout -b feature/new-feature

# 2. 开发功能
# 编写代码...

# 3. 测试功能
npm run dev      # Web 端测试
npm run android:dev  # Android 端测试 (可选)

# 4. 提交代码
git add .
git commit -m "feat: add new feature"

# 5. 推送分支
git push origin feature/new-feature

# 6. 创建 Pull Request
```

### 2. 页面开发流程

#### 创建新页面
```tsx
// src/screens/new-screen.tsx
import React from 'react'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

const NewScreen: React.FC = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      <Card>
        <h1 className="text-2xl font-bold">新页面</h1>
        <Button>点击按钮</Button>
      </Card>
    </div>
  )
}

export default NewScreen
```

#### 添加路由
```tsx
// src/App.tsx
import NewScreen from './screens/new-screen'

// 在 Routes 中添加
<Route path="new" element={<NewScreen />} />
```

### 3. 组件开发流程

#### 创建 UI 组件
```tsx
// src/components/ui/new-component.tsx
import React from 'react'
import { cn } from '@/lib/utils'

interface NewComponentProps {
  className?: string
  children: React.ReactNode
}

const NewComponent = React.forwardRef<
  HTMLDivElement,
  NewComponentProps
>(({ className, children, ...props }, ref) => (
  <div
    ref={ref}
    className={cn(
      "rounded-lg border bg-card text-card-foreground",
      className
    )}
    {...props}
  >
    {children}
  </div>
))

NewComponent.displayName = "NewComponent"

export { NewComponent }
```

## 📝 代码规范

### TypeScript 规范

#### 类型定义
```tsx
// 使用 interface 定义对象类型
interface User {
  id: string
  name: string
  email: string
  avatar?: string
}

// 使用 type 定义联合类型
type Status = 'loading' | 'success' | 'error'

// 组件 Props 类型定义
interface ButtonProps {
  variant?: 'default' | 'destructive' | 'outline'
  size?: 'default' | 'sm' | 'lg'
  disabled?: boolean
  onClick?: () => void
  children: React.ReactNode
}
```

#### 函数组件规范
```tsx
// 使用 React.FC 类型
const MyComponent: React.FC<MyComponentProps> = ({ 
  prop1, 
  prop2,
  ...props 
}) => {
  // 组件逻辑
  return <div {...props}>内容</div>
}

// 使用 forwardRef
const MyInput = React.forwardRef<
  HTMLInputElement,
  MyInputProps
>(({ className, ...props }, ref) => (
  <input
    ref={ref}
    className={cn("base-styles", className)}
    {...props}
  />
))
```

### 命名规范

#### 文件命名
- 组件文件：`PascalCase.tsx` (如 `UserProfile.tsx`)
- 工具文件：`camelCase.ts` (如 `formatDate.ts`)
- 页面文件：`kebab-case.tsx` (如 `user-profile.tsx`)
- 样式文件：`kebab-case.css` (如 `safe-area.css`)

#### 变量命名
```tsx
// 组件名：PascalCase
const UserProfile: React.FC = () => {}

// 变量名：camelCase
const userName = 'John Doe'
const isLoading = false

// 常量名：UPPER_CASE
const API_BASE_URL = 'https://api.example.com'

// 类型名：PascalCase
interface UserData {}
type ApiResponse = {}
```

### 代码组织

#### 导入顺序
```tsx
// 1. React 相关
import React, { useState, useEffect } from 'react'

// 2. 第三方库
import { useNavigate } from 'react-router-dom'
import { toast } from 'sonner'

// 3. 内部组件
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'

// 4. 工具函数
import { cn } from '@/lib/utils'
import { formatDate } from '@/utils/date'

// 5. 类型定义
import type { User } from '@/types/user'
```

## 🎨 样式开发

### Tailwind CSS 使用

#### 基础样式
```tsx
// 布局
<div className="flex flex-col items-center justify-center min-h-screen">
  
// 间距
<div className="p-4 m-2 px-6 py-3">

// 颜色
<div className="bg-primary text-primary-foreground">

// 响应式
<div className="w-full md:w-1/2 lg:w-1/3">
```

#### 自定义样式
```tsx
// 使用 cn 函数合并类名
import { cn } from '@/lib/utils'

const Button: React.FC<ButtonProps> = ({ 
  className,
  variant = 'default',
  ...props 
}) => {
  return (
    <button
      className={cn(
        // 基础样式
        "inline-flex items-center justify-center rounded-md font-medium transition-colors",
        // 变体样式
        {
          "bg-primary text-primary-foreground hover:bg-primary/90": variant === 'default',
          "border border-input hover:bg-accent": variant === 'outline',
        },
        // 自定义样式
        className
      )}
      {...props}
    />
  )
}
```

### 主题系统

#### CSS 变量定义
```css
/* styles/theme.css */
:root {
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  --primary: 222.2 47.4% 11.2%;
  --primary-foreground: 210 40% 98%;
}

.dark {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;
  --primary: 210 40% 98%;
  --primary-foreground: 222.2 47.4% 11.2%;
}
```

#### 主题切换
```tsx
// 使用 next-themes
import { useTheme } from 'next-themes'

const ThemeToggle: React.FC = () => {
  const { theme, setTheme } = useTheme()
  
  return (
    <button
      onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
    >
      切换主题
    </button>
  )
}
```

## 📱 移动端适配

### 平台检测
```tsx
// utils/platform.ts
export const isPlatform = (platform: string): boolean => {
  return document.body.classList.contains(`platform-${platform}`)
}

export const isAndroid = (): boolean => isPlatform('android')
export const isWeb = (): boolean => isPlatform('web')
```

### 安全区域适配
```css
/* styles/safe-area.css */
.safe-area-top {
  padding-top: env(safe-area-inset-top);
}

.safe-area-bottom {
  padding-bottom: env(safe-area-inset-bottom);
}

.safe-area-left {
  padding-left: env(safe-area-inset-left);
}

.safe-area-right {
  padding-right: env(safe-area-inset-right);
}
```

### 导航栏适配
```tsx
// utils/navigationBar.ts
export interface NavigationBarInfo {
  height: number
  visible: boolean
}

export const detectNavigationBar = (): NavigationBarInfo => {
  // 检测逻辑
  return {
    height: window.innerHeight - document.documentElement.clientHeight,
    visible: true
  }
}

export const setNavigationBarCSSVariables = (info: NavigationBarInfo) => {
  document.documentElement.style.setProperty(
    '--navigation-bar-height',
    `${info.height}px`
  )
}
```

## 🐛 调试技巧

### 开发工具

#### 浏览器调试
```tsx
// 使用 console 调试
console.log('调试信息:', data)
console.error('错误信息:', error)
console.table(arrayData)

// 使用 debugger
const handleClick = () => {
  debugger // 断点
  // 处理逻辑
}
```

#### React DevTools
- 安装 React Developer Tools 浏览器插件
- 查看组件树和 Props
- 分析性能问题

### 移动端调试

#### Android 调试
```bash
# 启用开发模式
npm run android:dev

# 查看日志
adb logcat | grep "Capacitor"

# Chrome 远程调试
# 在 Chrome 中访问 chrome://inspect
```

#### 调试面板
```tsx
// components/NavigationDebugPanel.tsx
const NavigationDebugPanel: React.FC<{ show: boolean }> = ({ show }) => {
  if (!show) return null
  
  return (
    <div className="fixed bottom-0 left-0 right-0 bg-black/80 text-white p-4">
      <div className="text-xs">
        <div>导航栏高度: {navigationBarHeight}px</div>
        <div>安全区域: {safeAreaInsets}</div>
        <div>平台: {platform}</div>
      </div>
    </div>
  )
}
```

## ⚡ 性能优化

### 代码分割
```tsx
// 使用 React.lazy 进行代码分割
const LazyComponent = React.lazy(() => import('./LazyComponent'))

// 在路由中使用
<Route 
  path="/lazy" 
  element={
    <Suspense fallback={<div>加载中...</div>}>
      <LazyComponent />
    </Suspense>
  } 
/>
```

### 组件优化
```tsx
// 使用 React.memo 避免不必要的重渲染
const OptimizedComponent = React.memo<Props>(({ data }) => {
  return <div>{data}</div>
})

// 使用 useMemo 缓存计算结果
const ExpensiveComponent: React.FC<Props> = ({ items }) => {
  const expensiveValue = useMemo(() => {
    return items.reduce((acc, item) => acc + item.value, 0)
  }, [items])
  
  return <div>{expensiveValue}</div>
}

// 使用 useCallback 缓存函数
const ParentComponent: React.FC = () => {
  const [count, setCount] = useState(0)
  
  const handleClick = useCallback(() => {
    setCount(c => c + 1)
  }, [])
  
  return <ChildComponent onClick={handleClick} />
}
```

### 图片优化
```tsx
// 使用合适的图片格式和尺寸
<img 
  src="/images/hero.webp"
  alt="Hero Image"
  loading="lazy"
  className="w-full h-auto"
/>

// 使用 Aspect Ratio 组件
import { AspectRatio } from '@/components/ui/aspect-ratio'

<AspectRatio ratio={16 / 9}>
  <img src="/image.jpg" alt="Image" className="object-cover" />
</AspectRatio>
```

## 🚀 部署发布

### Web 部署
```bash
# 构建生产版本
npm run build

# 预览构建结果
npm run preview

# 部署到静态托管服务
# 将 dist/ 目录上传到服务器
```

### Android 发布
```bash
# 1. 构建 Web 应用
npm run build

# 2. 同步到 Android
npm run sync

# 3. 打开 Android Studio
npm run build:android

# 4. 在 Android Studio 中构建 APK
# Build -> Build Bundle(s) / APK(s) -> Build APK(s)
```

### 版本管理
```json
// package.json
{
  "version": "1.0.0",
  "scripts": {
    "version:patch": "npm version patch",
    "version:minor": "npm version minor",
    "version:major": "npm version major"
  }
}
```

## ❓ 常见问题

### 环境问题

#### Q: npm install 失败
```bash
# 清理缓存
npm cache clean --force

# 删除 node_modules 重新安装
rm -rf node_modules package-lock.json
npm install
```

#### Q: Capacitor 同步失败
```bash
# 清理 Capacitor
npx cap clean
npx cap sync
```

### 开发问题

#### Q: TypeScript 类型错误
```tsx
// 使用类型断言
const element = document.getElementById('id') as HTMLElement

// 使用可选链
const value = data?.user?.name

// 使用类型守卫
const isString = (value: unknown): value is string => {
  return typeof value === 'string'
}
```

#### Q: 样式不生效
```tsx
// 检查 Tailwind 类名是否正确
// 使用开发者工具检查 CSS

// 确保导入了样式文件
import '@/globals.css'
```

### 构建问题

#### Q: Android 构建失败
```bash
# 检查 Java 版本
java -version

# 检查环境变量
echo $ANDROID_HOME
echo $JAVA_HOME

# 清理并重新构建
cd android
./gradlew clean
./gradlew build
```

## 📚 学习资源

### 官方文档
- [React 官方文档](https://react.dev/)
- [TypeScript 官方文档](https://www.typescriptlang.org/)
- [Tailwind CSS 文档](https://tailwindcss.com/)
- [Capacitor 文档](https://capacitorjs.com/)
- [Vite 文档](https://vitejs.dev/)

### 社区资源
- [Radix UI 组件库](https://www.radix-ui.com/)
- [shadcn/ui 组件](https://ui.shadcn.com/)
- [React Router 文档](https://reactrouter.com/)
- [Framer Motion 文档](https://www.framer.com/motion/)

---

**提示**：遇到问题时，建议先查看控制台错误信息，然后参考相关文档或社区讨论。保持代码整洁和文档更新是良好的开发习惯。 