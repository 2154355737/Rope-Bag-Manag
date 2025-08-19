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
import { initializeStatusBar } from './utils/statusBar'
import { addPlatformClass, initializeKeyboard } from './utils/platform'
import { detectNavigationBar, setNavigationBarCSSVariables, watchNavigationBarChanges } from './utils/navigationBar'
import { detectNavigationBarWithNativePlugin, watchNavigationBarWithNativePlugin } from './utils/navigationBarNative'
import { initializeBackButton } from './utils/backButton'
import BackButtonHandler from './components/BackButtonHandler'
import NavigationDebugPanel from './components/NavigationDebugPanel'

import './styles/safe-area.css'

const App: React.FC = () => {
  const [showSplash, setShowSplash] = useState(true)
  const [firstLaunch, setFirstLaunch] = useState(true)

  useEffect(() => {
    // 初始化平台适配
    const initializePlatform = async () => {
      // 添加平台类名
      addPlatformClass()
      
      // 初始化状态栏
      await initializeStatusBar()
      
      // 初始化键盘监听
      initializeKeyboard()
      
      // 初始化返回键监听器
      initializeBackButton()
      
      // 检测并设置导航栏
                  // 优先使用原生插件检测
            const navInfo = await detectNavigationBarWithNativePlugin()
            setNavigationBarCSSVariables(navInfo)
            console.log('导航栏信息:', navInfo)
      
      // 启用调试样式（方便调试）
      document.body.classList.add('debug-mode')
      
      // 监听导航栏变化
                  // 监听导航栏变化
            watchNavigationBarWithNativePlugin((newNavInfo) => {
              setNavigationBarCSSVariables(newNavInfo)
              console.log('导航栏变化:', newNavInfo)
            })
    }
    
    initializePlatform()

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
      
      {/* 显示调试面板（包括生产环境，方便调试） */}
      <NavigationDebugPanel show={false} />
      
      {/* 键盘调试面板 */}
      
    </>
  )
}

export default App