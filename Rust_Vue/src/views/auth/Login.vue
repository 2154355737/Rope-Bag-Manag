<template>
  <div class="login-desktop">
    <!-- 背景装饰 -->
    <div class="background-decoration">
      <div class="decoration-circle circle-1"></div>
      <div class="decoration-circle circle-2"></div>
      <div class="decoration-circle circle-3"></div>
      <div class="decoration-circle circle-4"></div>
      <div class="floating-particles">
        <div class="particle" v-for="i in 12" :key="i" :style="{ animationDelay: `${i * 0.2}s` }"></div>
      </div>
    </div>

    <div class="login-container">
      <!-- 左侧装饰区域 -->
      <div class="login-decoration">
        <div class="decoration-content">
          <div class="logo-section">
            <div class="logo-icon">
              <el-icon :size="64"><Box /></el-icon>
            </div>
            <h1 class="app-title">绳包管理系统</h1>
            <p class="app-subtitle">专业的绳包管理解决方案</p>
          </div>
          <div class="feature-list">
            <div class="feature-item">
              <el-icon><User /></el-icon>
              <span>用户权限管理</span>
            </div>
            <div class="feature-item">
              <el-icon><Box /></el-icon>
              <span>绳包信息管理</span>
            </div>
            <div class="feature-item">
              <el-icon><Document /></el-icon>
              <span>系统日志记录</span>
            </div>
            <div class="feature-item">
              <el-icon><DataAnalysis /></el-icon>
              <span>数据统计分析</span>
            </div>
          </div>
        </div>
        <div class="decoration-bg">
          <div class="bg-circle circle-1"></div>
          <div class="bg-circle circle-2"></div>
          <div class="bg-circle circle-3"></div>
        </div>
      </div>

      <!-- 右侧登录表单 -->
      <div class="login-form-section">
        <div class="form-container">
          <div class="form-header">
            <h2 class="form-title">欢迎登录</h2>
            <p class="form-subtitle">选择您的登录方式</p>
          </div>

          <!-- 登录方式选择 -->
          <el-tabs v-model="loginType" class="login-tabs">
            <el-tab-pane label="密码登录" name="password">
          <el-form 
            ref="loginFormRef" 
            :model="loginForm" 
            :rules="loginRules" 
            class="login-form"
            @submit.prevent="handleLogin"
          >
            <div class="form-group">
                  <label class="form-label">用户名/邮箱</label>
              <el-input
                v-model="loginForm.username"
                    placeholder="请输入用户名或邮箱"
                size="large"
                class="form-input"
                :prefix-icon="User"
                clearable
              />
            </div>

            <div class="form-group">
              <label class="form-label">密码</label>
              <el-input
                v-model="loginForm.password"
                type="password"
                placeholder="请输入密码"
                size="large"
                class="form-input"
                :prefix-icon="Lock"
                show-password
                clearable
              />
            </div>

            <div class="form-options">
              <el-checkbox v-model="rememberMe" class="remember-checkbox">
                记住我
              </el-checkbox>
              <a href="#" class="forgot-link" @click.prevent="goForgot">忘记密码？</a>
            </div>

            <el-button
              type="primary"
              size="large"
              class="login-btn"
              :loading="loading"
              @click="handleLogin"
            >
              <el-icon v-if="!loading"><Right /></el-icon>
              {{ loading ? '登录中...' : '登录' }}
            </el-button>
          </el-form>
            </el-tab-pane>

            <el-tab-pane label="验证码登录" name="code">
              <el-form 
                ref="codeFormRef" 
                :model="codeForm" 
                :rules="codeRules" 
                class="login-form"
                @submit.prevent="handleCodeLogin"
              >
                <div class="form-group">
                  <label class="form-label">邮箱地址</label>
                  <el-input
                    v-model="codeForm.email"
                    placeholder="请输入邮箱地址"
                    size="large"
                    class="form-input"
                    :prefix-icon="User"
                    clearable
                  />
                </div>

                <div class="form-group">
                  <label class="form-label">验证码</label>
                  <div class="code-input-group">
                    <el-input
                      v-model="codeForm.code"
                      placeholder="请输入验证码"
                      size="large"
                      class="form-input code-input"
                      :prefix-icon="Lock"
                      clearable
                    />
                    <el-button 
                      type="primary" 
                      size="large"
                      :loading="sendingCode"
                      :disabled="!isValidEmail(codeForm.email) || countdown > 0"
                      @click="sendLoginCode"
                      class="send-code-btn"
                    >
                      {{ countdown > 0 ? `${countdown}s后重发` : '发送验证码' }}
                    </el-button>
                  </div>
                </div>

                <el-button
                  type="primary"
                  size="large"
                  class="login-btn"
                  :loading="loading"
                  @click="handleCodeLogin"
                >
                  <el-icon v-if="!loading"><Right /></el-icon>
                  {{ loading ? '登录中...' : '验证码登录' }}
                </el-button>
              </el-form>
            </el-tab-pane>
          </el-tabs>

          <!-- 注册账号按钮 -->
          <div class="register-link-container" style="text-align:center;margin-top:16px;">
            <el-button type="success" link @click="goRegister">注册账号</el-button>
          </div>

          <!-- 底部信息 -->
          <div class="login-footer">
            <p class="footer-text">© 2024 绳包管理系统. All rights reserved.</p>
            <p class="footer-version">Version 1.0.0</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock, Right, Box, DataAnalysis } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { authApi, setToken } from '../../api'
import { setLoginStatus, setRememberMe, getRememberCredentials, getUserInfo, getToken, isLoggedIn } from '../../utils/auth'
import userActionService from '../../utils/userActionService'
// import { getUsers, loadUsers } from '../../api/user' // 导入管理员相关API

const router = useRouter()
const loginFormRef = ref<FormInstance>()
const codeFormRef = ref<FormInstance>()
const loading = ref(false)
const sendingCode = ref(false)
const rememberMe = ref(false)
const loginType = ref('password') // 登录类型：password 或 code
const countdown = ref(0) // 验证码倒计时

// 在script部分添加管理员登录功能
const adminLogin = ref(false)
const adminUsername = ref('')
const adminPassword = ref('')

// 登录表单数据
const loginForm = reactive({
  username: '',
  password: ''
})

// 验证码登录表单数据
const codeForm = reactive({
  email: '',
  code: ''
})

// 表单验证规则
const loginRules: FormRules = {
  username: [
    { required: true, message: '请输入用户名或邮箱', trigger: 'blur' },
    { min: 3, max: 50, message: '用户名长度在 3 到 50 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' }
  ]
}

// 验证码表单验证规则
const codeRules: FormRules = {
  email: [
    { required: true, message: '请输入邮箱地址', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  code: [
    { required: true, message: '请输入验证码', trigger: 'blur' },
    { len: 6, message: '验证码为6位数字', trigger: 'blur' }
  ]
}

// 验证邮箱格式
function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

// 开始倒计时
function startCountdown() {
  countdown.value = 60
  const timer = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      clearInterval(timer)
    }
  }, 1000)
}

// 发送登录验证码
async function sendLoginCode() {
  if (!isValidEmail(codeForm.email)) {
    ElMessage.error('请输入正确的邮箱格式')
    return
  }

  try {
    sendingCode.value = true
    const response = await authApi.sendLoginCode({ email: codeForm.email })
    
    if (response.code === 0) {
      ElMessage.success('验证码已发送，请查收邮件')
      startCountdown()
    } else {
      ElMessage.error(response.message || '发送验证码失败')
    }
  } catch (error) {
    console.error('发送验证码失败:', error)
    ElMessage.error('发送验证码失败，请稍后重试')
  } finally {
    sendingCode.value = false
  }
}

// 处理验证码登录
async function handleCodeLogin() {
  if (!codeFormRef.value) return
  
  try {
    await codeFormRef.value.validate()
    loading.value = true
    
    const response = await authApi.loginByEmail({
      email: codeForm.email,
      code: codeForm.code
    })
    
    if (response.code === 0) {
      // 登录成功
      handleLoginSuccess(response, codeForm.email)
    } else {
      ElMessage.error(response.message || '验证码登录失败')
    }
  } catch (error) {
    console.error('验证码登录失败:', error)
    ElMessage.error('验证码登录失败，请稍后重试')
  } finally {
    loading.value = false
  }
}

// 处理登录成功的逻辑
function handleLoginSuccess(response: any, identifier: string) {
      console.log('🎉 开始处理登录成功:', response)
      
      const token = response.data?.token ?? ''
      const user = response.data?.user
      
      console.log('📝 登录数据:', { token: token ? '存在' : '无', user })
      
      // 使用新的Cookie认证机制
      setToken(token)
      setLoginStatus(user?.username || identifier, token, user)
      
      // 验证状态是否正确设置
      setTimeout(() => {
        const savedUserInfo = getUserInfo()
        const savedToken = getToken()
        const isLoggedInStatus = isLoggedIn()
        
        console.log('✅ 登录状态验证:', { 
          savedUserInfo, 
          savedToken: savedToken ? '存在' : '无',
          isLoggedInStatus 
        })
      }, 100)
      
      // 处理记住我功能
      if (rememberMe.value && loginType.value === 'password') {
        setRememberMe(true, {
          username: loginForm.username,
          password: loginForm.password
        })
      } else {
        setRememberMe(false)
      }
      
      ElMessage.success('登录成功')
      
      // 延迟跳转，确保认证状态完全更新且避免路由守卫时序问题
      setTimeout(() => {
        // 记录用户登录行为
        userActionService.logLogin(identifier, true)
          .catch(err => console.error('记录登录行为失败:', err))
        
        // 跳转到对应后台
      if (user?.role === 'admin') {
          console.log('🚀 跳转到管理员后台')
        router.push('/admin')
      } else if (user?.role === 'elder') {
          console.log('🚀 跳转到长老后台')
        router.push('/elder')
      } else {
          console.log('🚀 跳转到用户后台')
        router.push('/user')
      }
      }, 500) // 增加延迟时间，确保认证状态稳定
}

// 处理登录
async function handleLogin() {
  if (!loginFormRef.value) return
  
  try {
    await loginFormRef.value.validate()
    loading.value = true
    
    const response = await authApi.login({
      username: loginForm.username,
      password: loginForm.password
    })
    
    if (response.code === 0) {
      // 登录成功
      handleLoginSuccess(response, loginForm.username)
      return
    } else {
      // 记录登录失败行为
      userActionService.logLogin(loginForm.username, false, response.message)
        .catch(err => console.error('记录登录失败行为失败:', err))
        
      ElMessage.error(response.msg || response.message || '登录失败，请检查用户名和密码')
    }
  } catch (error: any) {
    console.error('登录失败:', error)
    
    // 尝试从后端响应中获取具体错误信息
    if (error.response && error.response.data && error.response.data.message) {
      ElMessage.error(error.response.data.message)
    } else {
      const errorMessage = error instanceof Error ? error.message : String(error)
      
      if (errorMessage.includes('fetch') || 
          errorMessage.includes('network') || 
          errorMessage.includes('Failed to fetch') ||
          errorMessage.includes('ERR_NETWORK') ||
          errorMessage.includes('ERR_CONNECTION_REFUSED') ||
          errorMessage.includes('Service unavailable')) {
        ElMessage.error('服务连接失败，请检查网络连接')
      } else {
        ElMessage.error('登录失败，请稍后重试')
      }
    }
  } finally {
    loading.value = false
  }
}

// 管理员登录方法
async function handleAdminLogin() {
  try {
    loading.value = true
    
    // 保存管理员凭据到localStorage
    const adminInfo = {
      username: adminUsername.value,
      password: adminPassword.value
    }
    localStorage.setItem('adminInfo', JSON.stringify(adminInfo))
    
    // 测试管理员权限
    // const testRes = await getUsers() // This line was removed as per the edit hint
    // if (testRes.code === 0) { // This line was removed as per the edit hint
      ElMessage.success('管理员登录成功') // This line was removed as per the edit hint
      adminLogin.value = false // This line was removed as per the edit hint
      // 重新加载用户数据 // This line was removed as per the edit hint
      // await loadUsers() // This line was removed as per the edit hint
    // } else { // This line was removed as per the edit hint
      // ElMessage.error('管理员凭据无效') // This line was removed as per the edit hint
    // } // This line was removed as per the edit hint
  } catch (error) {
    console.error('管理员登录失败:', error)
    ElMessage.error('管理员登录失败')
  } finally {
    loading.value = false
  }
}

// 清除管理员凭据
function clearAdminCredentials() {
  localStorage.removeItem('adminInfo')
  ElMessage.success('管理员凭据已清除')
}

// 快速登录
function quickLogin(username: string, password: string) {
  loginForm.username = username
  loginForm.password = password
  handleLogin()
}

// 自动填充记住的用户信息 (使用Cookie机制)
const savedCredentials = getRememberCredentials()
if (savedCredentials) {
  loginForm.username = savedCredentials.username || ''
  loginForm.password = savedCredentials.password || ''
    rememberMe.value = true
}

function handleLogout() {
  localStorage.removeItem('loginToken')
  localStorage.removeItem('loginUser')
  router.replace('/login')
}

function goRegister() {
  router.push('/register')
}

function goForgot(){ router.push('/forgot-password') }
</script>

<style scoped>
.login-desktop {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  position: relative;
  overflow: hidden;
}

.login-container {
  width: 100%;
  max-width: 1000px;
  height: 600px;
  background: var(--bg-card);
  border-radius: 24px;
  box-shadow: var(--shadow-base);
  border: 1px solid var(--border-color);
  display: flex;
  overflow: hidden;
  position: relative;
}

/* 左侧装饰区域 */
.login-decoration {
  flex: 1;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.decoration-content {
  position: relative;
  z-index: 10;
  text-align: center;
  color: white;
  padding: 40px;
}

.logo-section {
  margin-bottom: 40px;
}

.logo-icon {
  width: 120px;
  height: 120px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 24px;
  backdrop-filter: blur(10px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-10px);
  }
}

.app-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 12px 0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.app-subtitle {
  font-size: 16px;
  margin: 0;
  opacity: 0.9;
}

.feature-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.feature-item {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
  opacity: 0.9;
}

.feature-item .el-icon {
  font-size: 18px;
}

.decoration-bg {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
}

.bg-circle {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  animation: float 6s ease-in-out infinite;
}

.circle-1 {
  width: 200px;
  height: 200px;
  top: 10%;
  left: 10%;
  animation-delay: 0s;
}

.circle-2 {
  width: 150px;
  height: 150px;
  top: 60%;
  right: 15%;
  animation-delay: 2s;
}

.circle-3 {
  width: 100px;
  height: 100px;
  bottom: 20%;
  left: 20%;
  animation-delay: 4s;
}

/* 右侧登录表单 */
.login-form-section {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px;
}

.form-container {
  width: 100%;
  max-width: 400px;
}

.form-header {
  text-align: center;
  margin-bottom: 32px;
}

.form-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.form-subtitle {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.form-input {
  border-radius: 12px;
}

.form-input :deep(.el-input__wrapper) {
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}

.form-input :deep(.el-input__wrapper:hover) {
  box-shadow: 0 4px 16px rgba(64, 158, 255, 0.2);
}

.form-input :deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 4px 16px rgba(64, 158, 255, 0.3);
}

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.remember-checkbox {
  font-size: 14px;
  color: var(--text-secondary);
}

.forgot-link {
  font-size: 14px;
  color: var(--brand-color);
  text-decoration: none;
  transition: color 0.3s ease;
}

.forgot-link:hover {
  color: var(--brand-color-dark);
}

.login-btn {
  width: 100%;
  height: 48px;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 600;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border: none;
  box-shadow: 0 4px 16px rgba(64, 158, 255, 0.3);
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  position: relative;
  overflow: hidden;
}

.login-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.login-btn:hover::before {
  left: 100%;
}

.login-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 24px rgba(64, 158, 255, 0.4);
}

.login-btn:active {
  transform: translateY(0);
}

/* 登录标签页样式 */
.login-tabs {
  margin-bottom: 20px;
}

.login-tabs .el-tabs__header {
  margin: 0 0 20px 0;
}

.login-tabs .el-tabs__item {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-secondary);
}

.login-tabs .el-tabs__item.is-active {
  color: var(--brand-color);
  font-weight: 600;
}

.login-tabs .el-tabs__active-bar {
  background-color: var(--brand-color);
}

/* 验证码输入组样式 */
.code-input-group {
  display: flex;
  gap: 12px;
  align-items: flex-end;
}

.code-input {
  flex: 1;
}

.send-code-btn {
  height: 48px;
  padding: 0 20px;
  white-space: nowrap;
  border-radius: 12px;
  font-size: 14px;
  min-width: 120px;
}

/* 快速登录 */
.quick-login {
  margin-top: 32px;
}

.quick-login-title {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 20px;
}

.divider-line {
  flex: 1;
  height: 1px;
  background: var(--border-color);
}

.divider-text {
  font-size: 12px;
  color: var(--text-tertiary);
  font-weight: 500;
}

.quick-login-buttons {
  display: flex;
  gap: 12px;
}

.quick-btn {
  flex: 1;
  height: 44px;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  transition: all 0.3s ease;
}

.admin-btn {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
  border: none;
  color: white;
  box-shadow: 0 2px 8px rgba(103, 194, 58, 0.3);
  position: relative;
  overflow: hidden;
}

.admin-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.admin-btn:hover::before {
  left: 100%;
}

.admin-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(103, 194, 58, 0.4);
}

.user-btn {
  background: linear-gradient(135deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
  border: none;
  color: white;
  box-shadow: 0 2px 8px rgba(230, 162, 60, 0.3);
  position: relative;
  overflow: hidden;
}

.user-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.user-btn:hover::before {
  left: 100%;
}

.user-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(230, 162, 60, 0.4);
}

/* 底部信息 */
.login-footer {
  text-align: center;
  margin-top: 32px;
}

.footer-text {
  font-size: 12px;
  color: var(--text-tertiary);
  margin: 0 0 4px 0;
}

.footer-version {
  font-size: 10px;
  color: var(--text-tertiary);
  margin: 0;
}

/* 深色模式适配 */
.dark .login-container {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
}

/* 主题适配 */
.blue .login-btn {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  box-shadow: 0 4px 16px rgba(64, 158, 255, 0.3);
}

.blue .login-btn:hover {
  box-shadow: 0 6px 24px rgba(64, 158, 255, 0.4);
}

.green .login-btn {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
  box-shadow: 0 4px 16px rgba(103, 194, 58, 0.3);
}

.green .login-btn:hover {
  box-shadow: 0 6px 24px rgba(103, 194, 58, 0.4);
}

.orange .login-btn {
  background: linear-gradient(135deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
  box-shadow: 0 4px 16px rgba(230, 162, 60, 0.3);
}

.orange .login-btn:hover {
  box-shadow: 0 6px 24px rgba(230, 162, 60, 0.4);
}

.purple .login-btn {
  background: linear-gradient(135deg, var(--info-color) 0%, var(--info-color-light) 100%);
  box-shadow: 0 4px 16px rgba(144, 147, 153, 0.3);
}

.purple .login-btn:hover {
  box-shadow: 0 6px 24px rgba(144, 147, 153, 0.4);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .login-container {
    flex-direction: column;
    height: auto;
    max-width: 500px;
  }
  
  .login-decoration {
    padding: 40px 20px;
  }
  
  .decoration-content {
    padding: 20px;
  }
  
  .logo-icon {
    width: 80px;
    height: 80px;
  }
  
  .app-title {
    font-size: 24px;
  }
  
  .app-subtitle {
    font-size: 14px;
  }
  
  .feature-list {
    gap: 12px;
  }
  
  .feature-item {
    font-size: 12px;
  }
  
  .login-form-section {
    padding: 32px 24px;
  }
  
  .form-title {
    font-size: 24px;
  }
  
  .form-subtitle {
    font-size: 14px;
  }
  
  .quick-login-buttons {
    flex-direction: column;
    gap: 8px;
  }
  
  .quick-btn {
    height: 40px;
    font-size: 13px;
  }
}

@media (max-width: 480px) {
  .login-desktop {
    padding: 16px;
  }
  
  .login-container {
    border-radius: 16px;
  }
  
  .login-decoration {
    padding: 32px 16px;
  }
  
  .login-form-section {
    padding: 24px 20px;
  }
}

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .logo-icon,
  .bg-circle {
    animation: none;
  }
  
  .login-btn:hover,
  .admin-btn:hover,
  .user-btn:hover {
    transform: none;
  }
}
</style> 