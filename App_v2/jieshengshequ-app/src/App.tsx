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

import './styles/safe-area-v2.css'

const App: React.FC = () => {
  const [showSplash, setShowSplash] = useState(true)
  const [firstLaunch, setFirstLaunch] = useState(true)

  useEffect(() => {
    // 初始化平台适配
    const initializeApp = async () => {
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
      
      console.log('✅ 应用初始化完成')
    }
    
    initializeApp()

    // 模拟启动页显示3秒
    const timer = setTimeout(() => {
      setShowSplash(false)
    }, 3000)

    // 检查是否首次启动（实际应用中应该使用AsyncStorage或类似存储）
    // 这里仅作演示
    const checkFirstLaunch = async () => {
      // 模拟检查首次启动
      // 实际应用中应该从存储中读取
      setFirstLaunch(true)
    }
    
    checkFirstLaunch()
    
    return () => clearTimeout(timer)
  }, [])

  if (showSplash) {
    return <SplashScreen onSkip={() => setShowSplash(false)} />
  }

  if (firstLaunch && !showSplash) {
    return <OnboardingScreen onComplete={() => setFirstLaunch(false)} />
  }

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