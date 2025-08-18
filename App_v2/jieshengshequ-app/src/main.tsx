import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { ThemeProvider } from './components/theme-provider'
import { SafeAreaProvider } from './components/safe-area-provider'
import './globals.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ThemeProvider defaultTheme="light" storageKey="jieshengshequ-theme">
      <SafeAreaProvider>
        <App />
      </SafeAreaProvider>
    </ThemeProvider>
  </React.StrictMode>,
)