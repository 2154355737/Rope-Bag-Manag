import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App.tsx'
import './globals.css'
import './styles/safe-area-v2.css'
import './styles/theme.css'
import { ThemeProvider } from './components/theme-provider.tsx'
import { Toaster } from './components/ui/toaster.tsx'
import { SafeAreaProvider } from './components/safe-area-provider.tsx'

const AppTree = (
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
)

ReactDOM.createRoot(document.getElementById('root')!).render(
  import.meta.env.PROD ? (
    <React.StrictMode>{AppTree}</React.StrictMode>
  ) : (
    AppTree
  ),
)