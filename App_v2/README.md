# 结绳社区移动应用 📱

一个基于 React + TypeScript + Capacitor 构建的跨平台移动社区应用，专注于提供现代化的用户体验和原生性能。

## 📖 项目概述

结绳社区是一个功能丰富的移动端社区应用，支持 Android 和 Web 平台。应用采用现代化的技术栈，提供流畅的用户体验和美观的界面设计。

### ✨ 核心特性

- 🎯 **跨平台支持** - 一套代码，支持 Android 和 Web 平台
- 🎨 **现代化 UI** - 基于 Radix UI 和 Tailwind CSS 的精美界面
- 📱 **原生体验** - 通过 Capacitor 实现原生功能集成
- 🌙 **主题系统** - 支持明暗主题切换
- 🔧 **TypeScript** - 完整的类型安全支持
- ⚡ **高性能** - Vite 构建工具提供快速的开发体验

### 🏗️ 技术架构

```
结绳社区移动应用
├── Frontend (React + TypeScript)
├── UI Framework (Radix UI + Tailwind CSS)
├── 路由系统 (React Router)
├── 跨平台框架 (Capacitor)
├── 构建工具 (Vite)
└── 移动端适配 (Android)
```

## 🛠️ 技术栈

### 核心框架
- **React 18.2.0** - 用户界面构建
- **TypeScript 5.2.2** - 类型安全的 JavaScript
- **Capacitor 7.4.2** - 跨平台移动应用框架
- **Vite 5.0.8** - 快速构建工具

### UI 组件库
- **Radix UI** - 无障碍的 UI 组件基础
- **Tailwind CSS 3.4.1** - 实用优先的 CSS 框架
- **Lucide React** - 精美的图标库
- **Framer Motion** - 流畅的动画效果

### 路由与状态
- **React Router DOM 6.22.0** - 单页应用路由
- **React Hook Form 7.62.0** - 表单状态管理

### 开发工具
- **ESLint + Prettier** - 代码规范和格式化
- **PostCSS** - CSS 处理工具

## 📱 功能模块

### 主要页面
- **启动页** (`splash-screen`) - 应用启动欢迎页面
- **引导页** (`onboarding-screen`) - 首次使用引导
- **首页** (`home-screen`) - 主要内容展示
- **分类页** (`category-screen`) - 内容分类浏览
- **社区页** (`community-screen`) - 社区互动功能
- **消息页** (`messages-screen`) - 消息通知中心
- **个人页** (`profile-screen`) - 用户个人信息
- **帖子详情** (`post-detail-screen`) - 内容详情展示

### 核心组件
- **Layout** - 应用整体布局组件
- **Navigation** - 导航栏组件
- **Theme Provider** - 主题管理组件
- **UI Components** - 30+ 可复用 UI 组件

### 平台适配
- **状态栏管理** - 原生状态栏样式控制
- **导航栏检测** - 自动适配设备导航栏
- **键盘监听** - 软键盘弹出适配
- **安全区域** - 刘海屏等异形屏适配

## 🚀 快速开始

### 环境要求

- **Node.js** >= 18.0.0
- **npm** >= 8.0.0
- **Java JDK** 21 (用于 Android 开发)
- **Android Studio** (用于 Android 开发)

### 安装依赖

```bash
# 克隆项目
git clone <repository-url>
cd App_v2/jieshengshequ-app

# 安装依赖
npm install
```

### 开发模式

```bash
# 启动开发服务器 (Web)
npm run dev

# 在 Android 设备上运行 (需要先配置 Android 环境)
npm run android:dev
```

### 构建应用

```bash
# 构建 Web 版本
npm run build

# 构建 Android 版本
npm run build:android
```

## 📁 项目结构

```
jieshengshequ-app/
├── android/                    # Android 平台代码
├── src/
│   ├── components/            # 可复用组件
│   │   ├── ui/               # UI 基础组件
│   │   ├── layout.tsx        # 布局组件
│   │   └── theme-provider.tsx # 主题提供者
│   ├── screens/              # 页面组件
│   │   ├── splash-screen.tsx
│   │   ├── home-screen.tsx
│   │   └── ...
│   ├── hooks/                # 自定义 Hooks
│   ├── lib/                  # 工具库
│   ├── plugins/              # Capacitor 插件
│   ├── styles/               # 样式文件
│   ├── utils/                # 工具函数
│   └── App.tsx              # 应用入口
├── capacitor.config.ts       # Capacitor 配置
├── package.json             # 项目依赖
├── tailwind.config.ts       # Tailwind 配置
├── vite.config.ts          # Vite 配置
└── tsconfig.json           # TypeScript 配置
```

## 🎨 UI 设计系统

### 主题配置
- 支持明暗主题切换
- 基于 CSS 变量的动态主题系统
- 符合现代设计规范的配色方案

### 组件库
项目包含 30+ 精心设计的 UI 组件：
- 表单组件 (Input, Select, Checkbox, Radio 等)
- 导航组件 (Navigation Menu, Breadcrumb 等)
- 反馈组件 (Alert, Toast, Dialog 等)
- 数据展示组件 (Table, Card, Avatar 等)

### 动画系统
- 基于 Framer Motion 的流畅动画
- 自定义 Tailwind 动画类
- 符合移动端交互习惯的过渡效果

## 📱 移动端适配

### 平台检测
- 自动检测运行平台 (Android/Web)
- 平台特定的样式和功能适配

### 原生功能集成
- 状态栏样式控制
- 导航栏高度检测
- 键盘弹出处理
- 设备信息获取

### 安全区域适配
- 自动适配刘海屏、水滴屏等异形屏
- 动态计算安全区域边距
- 响应式布局适配

## 🔧 开发工具配置

### VS Code 推荐插件
- TypeScript and JavaScript Language Features
- Tailwind CSS IntelliSense
- ES7+ React/Redux/React-Native snippets
- Auto Rename Tag
- Prettier - Code formatter

### 开发调试
- 浏览器开发者工具
- React Developer Tools
- Capacitor 原生调试功能
- 导航栏调试面板

## 📚 更多文档

- [Android 构建指南](./ANDROID_BUILD_GUIDE.md) - 详细的 Android 打包说明
- [开发指南](./DEVELOPMENT_GUIDE.md) - 完整的开发文档
- [组件文档](./docs/components.md) - UI 组件使用说明
- [API 文档](./docs/api.md) - 接口文档

## 🤝 贡献指南

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 📞 联系我们

- 项目维护者：[您的姓名]
- 邮箱：[您的邮箱]
- 项目地址：[项目仓库地址]

---

**注意**：这是一个正在开发中的项目，某些功能可能还不完善。欢迎提出建议和反馈！ 