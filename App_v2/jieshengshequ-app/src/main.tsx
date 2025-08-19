import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App.tsx'
import './globals.css'
import './styles/safe-area.css'
import './styles/theme.css'
import './styles/modern-keyboard.css'
import './styles/statusbar-fix.css'
import { ThemeProvider } from './components/theme-provider.tsx'
import { Toaster } from './components/ui/toaster.tsx'
import { SafeAreaProvider } from './components/safe-area-provider.tsx'

// 初始化平台相关功能
import { addPlatformClass } from './utils/platform.ts'
import { initializeBackButton } from './utils/backButton.ts'
// 导入现代化键盘管理器（自动初始化）
import './utils/simpleKeyboard'

// 添加平台类名
addPlatformClass()

// 现代化键盘管理器会自动初始化，无需手动调用

// 初始化返回键处理
initializeBackButton()

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter
      future={{
        v7_startTransition: true,
        v7_relativeSplatPath: true
      }}
    >
      <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
        <SafeAreaProvider>
          <App />
          <Toaster />
        </SafeAreaProvider>
      </ThemeProvider>
    </BrowserRouter>
  </React.StrictMode>,
)