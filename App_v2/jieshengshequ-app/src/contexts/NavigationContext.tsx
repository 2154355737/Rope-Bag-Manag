import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import StorageManager from '@/utils/storage'

interface NavigationState {
  homeActiveTab: string
  categoryActiveTab: string
  communityActiveTab: string
}

interface NavigationContextType {
  navigationState: NavigationState
  setActiveTab: (page: string, tab: string) => void
  getActiveTab: (page: string, defaultTab?: string) => string
}

const NavigationContext = createContext<NavigationContextType | undefined>(undefined)

export const useNavigation = () => {
  const context = useContext(NavigationContext)
  if (!context) {
    throw new Error('useNavigation must be used within a NavigationProvider')
  }
  return context
}

interface NavigationProviderProps {
  children: ReactNode
}

export const NavigationProvider: React.FC<NavigationProviderProps> = ({ children }) => {
  const [navigationState, setNavigationState] = useState<NavigationState>({
    homeActiveTab: 'home',
    categoryActiveTab: 'list',
    communityActiveTab: 'all'
  })

  // 初始化时从存储中恢复状态
  useEffect(() => {
    const savedState = {
      homeActiveTab: StorageManager.getActiveTab('home', 'home'),
      categoryActiveTab: StorageManager.getActiveTab('category', 'list'),
      communityActiveTab: StorageManager.getActiveTab('community', 'all')
    }
    setNavigationState(savedState)
  }, [])

  const setActiveTab = (page: string, tab: string) => {
    console.log(`🔄 导航状态更新: ${page} -> ${tab}`)
    
    // 更新本地状态
    setNavigationState(prev => ({
      ...prev,
      [`${page}ActiveTab`]: tab
    }) as NavigationState)
    
    // 持久化保存
    StorageManager.saveActiveTab(page, tab)
  }

  const getActiveTab = (page: string, defaultTab: string = 'home'): string => {
    const stateKey = `${page}ActiveTab` as keyof NavigationState
    return navigationState[stateKey] || defaultTab
  }

  const value: NavigationContextType = {
    navigationState,
    setActiveTab,
    getActiveTab
  }

  return (
    <NavigationContext.Provider value={value}>
      {children}
    </NavigationContext.Provider>
  )
} 