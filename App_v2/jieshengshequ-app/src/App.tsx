import React, { useState, useEffect } from 'react'
import { Routes, Route } from 'react-router-dom'
import SplashScreen from './screens/splash-screen'
import OnboardingScreen from './screens/onboarding-screen'
import HomeScreen from './screens/home-screen'
import CategoryScreen from './screens/category-screen'
import CommunityScreen from './screens/community-screen'
import MessagesScreen from './screens/messages-screen'
import ProfileScreen from './screens/profile-screen'
import PostDetailScreen from './screens/post-detail-screen'
import ResourceDetailScreen from './screens/resource-detail-screen'
import AnnouncementDetailScreen from './screens/announcement-detail-screen'
import PublishScreen from './screens/publish-screen'
import SettingsScreen from './screens/settings-screen'
import EditProfileScreen from './screens/edit-profile-screen'
import MyContentScreen from './screens/my-content-screen'
import HelpScreen from './screens/help-screen'
import AboutScreen from './screens/about-screen'
import PrivacyScreen from './screens/privacy-screen'
import LoginScreen from './screens/login-screen'
import RegisterScreen from './screens/register-screen'
import ForgotPasswordScreen from './screens/forgot-password-screen'
import TermsScreen from './screens/terms-screen'
import Layout from './components/layout'
import { initializePlatform } from './utils/platform'
import { initializeBackButton } from './utils/backButton'
import BackButtonHandler from './components/BackButtonHandler'
import { initializeSafeArea, setSafeAreaTheme, setSafeAreaDebug } from './utils/safeAreaManager'
import StorageManager from './utils/storage'

import './styles/safe-area-v2.css'

const App: React.FC = () => {
  const [showSplash, setShowSplash] = useState(true)
  const [showOnboarding, setShowOnboarding] = useState(false)
  const [isInitialized, setIsInitialized] = useState(false)

  useEffect(() => {
    // 初始化平台适配
    const initializeApp = async () => {
      try {
        console.log('🚀 开始初始化应用...')
        
        // 初始化平台检测
        initializePlatform()
        
        // 初始化安全区域管理器（纯CSS方案）
        console.log('🔧 正在初始化安全区域管理器（纯CSS方案）...')
        await initializeSafeArea()
        
        // 设置默认主题为浅色
        setSafeAreaTheme('light')
        
        // 调试模式默认关闭
        setSafeAreaDebug(false)
        
        // 初始化返回键监听器
        initializeBackButton()
        
        // 检查是否首次启动
        const isFirstLaunch = StorageManager.isFirstLaunch()
        const launchCount = StorageManager.incrementLaunchCount()
        
        console.log(`📱 应用启动信息:`)
        console.log(`   - 是否首次启动: ${isFirstLaunch}`)
        console.log(`   - 启动次数: ${launchCount}`)
        
        setShowOnboarding(isFirstLaunch)
        setIsInitialized(true)
        
        console.log('✅ 应用初始化完成')
      } catch (error) {
        console.error('❌ 应用初始化失败:', error)
        // 即使初始化失败，也要让应用继续运行
        setIsInitialized(true)
        setShowOnboarding(StorageManager.isFirstLaunch())
      }
    }
    
    initializeApp()

    // 启动页显示时间（模拟加载时间）
    const splashTimer = setTimeout(() => {
      setShowSplash(false)
    }, 3000)

    return () => clearTimeout(splashTimer)
  }, [])

  // 完成引导页
  const handleOnboardingComplete = () => {
    console.log('📚 用户完成引导页')
    StorageManager.markAsLaunched()
    setShowOnboarding(false)
  }

  // 跳过启动页
  const handleSkipSplash = () => {
    console.log('⏭️ 用户跳过启动页')
    setShowSplash(false)
  }

  // 显示启动页
  if (showSplash) {
    return <SplashScreen onSkip={handleSkipSplash} />
  }

  // 显示引导页（仅首次启动时）
  if (showOnboarding && isInitialized) {
    return <OnboardingScreen onComplete={handleOnboardingComplete} />
  }

  // 主应用界面
  return (
    <>
      <BackButtonHandler />
      <Routes>
          <Route path="/" element={<Layout />}>
            <Route index element={<HomeScreen />} />
            <Route path="home" element={<HomeScreen />} />
            <Route path="category" element={<CategoryScreen />} />
            <Route path="community" element={<CommunityScreen />} />
            <Route path="messages" element={<MessagesScreen />} />
            <Route path="profile" element={<ProfileScreen />} />
            <Route path="publish" element={<PublishScreen />} />
            <Route path="post/:id" element={<PostDetailScreen />} />
            <Route path="resource/:id" element={<ResourceDetailScreen />} />
            <Route path="announcement/:id" element={<AnnouncementDetailScreen />} />
            <Route path="settings" element={<SettingsScreen />} />
            <Route path="edit-profile" element={<EditProfileScreen />} />
            <Route path="my-content" element={<MyContentScreen />} />
            <Route path="help" element={<HelpScreen />} />
            <Route path="about" element={<AboutScreen />} />
            <Route path="privacy" element={<PrivacyScreen />} />
          </Route>
          
          {/* 认证相关页面 */}
          <Route path="/login" element={<LoginScreen />} />
          <Route path="/register" element={<RegisterScreen />} />
          <Route path="/forgot-password" element={<ForgotPasswordScreen />} />
          <Route path="/terms" element={<TermsScreen />} />
      </Routes>
      
    </>
  )
}

export default App