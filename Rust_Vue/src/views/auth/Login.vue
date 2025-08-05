<template>
  <div class="login-container">
    <!-- 背景动效 -->
    <div class="animated-background">
      <div class="gradient-circle circle-1"></div>
      <div class="gradient-circle circle-2"></div>
      <div class="gradient-circle circle-3"></div>
      <div class="pattern-grid"></div>
    </div>

    <!-- 登录卡片 -->
    <div class="login-card">
      <!-- 登录表单区 -->
      <div class="login-form-section">
        <div class="login-header">
          <h2 class="login-title">欢迎回来</h2>
          <p class="login-subtitle">登录您的账号继续访问</p>
        </div>

        <div class="login-options">
          <div 
            :class="['login-option', { active: loginType === 'password' }]"
            @click="loginType = 'password'"
          >
            <el-icon><Lock /></el-icon>
            <span>密码登录</span>
          </div>
          <div 
            :class="['login-option', { active: loginType === 'code' }]"
            @click="loginType = 'code'"
          >
            <el-icon><Message /></el-icon>
            <span>验证码登录</span>
          </div>
        </div>

        <!-- 密码登录表单 -->
        <el-form 
          v-if="loginType === 'password'"
          ref="loginFormRef" 
          :model="loginForm" 
          :rules="loginRules"
          class="form"
          @submit.prevent="handleLogin"
        >
          <el-form-item prop="username">
            <el-input
              v-model="loginForm.username"
              placeholder="用户名或邮箱"
              :prefix-icon="User"
              size="large"
              clearable
            />
          </el-form-item>
          
          <el-form-item prop="password">
            <el-input
              v-model="loginForm.password"
              type="password"
              placeholder="密码"
              :prefix-icon="Lock"
              size="large"
              show-password
              clearable
            />
          </el-form-item>
          
          <div class="form-options">
            <el-checkbox v-model="rememberMe">记住我</el-checkbox>
            <el-button type="primary" text @click="goForgot">忘记密码?</el-button>
          </div>
          
          <el-button
            type="primary"
            size="large"
            class="submit-button"
            :loading="loading"
            @click="handleLogin"
          >
            登录
          </el-button>
        </el-form>
        
        <!-- 验证码登录表单 -->
        <el-form 
          v-if="loginType === 'code'"
          ref="codeFormRef" 
          :model="codeForm" 
          :rules="codeRules"
          class="form"
          @submit.prevent="handleCodeLogin"
        >
          <el-form-item prop="email">
            <el-input
              v-model="codeForm.email"
              placeholder="邮箱"
              :prefix-icon="Message"
              size="large"
              clearable
            />
          </el-form-item>
          
          <el-form-item prop="code">
            <div class="code-input-group">
              <el-input
                v-model="codeForm.code"
                placeholder="验证码"
                :prefix-icon="Key"
                size="large"
                clearable
              />
              <el-button 
                type="primary"
                :loading="sendingCode"
                :disabled="!isValidEmail(codeForm.email) || countdown > 0"
                @click="sendLoginCode"
                class="send-code-btn"
              >
                {{ countdown > 0 ? `${countdown}s` : '获取验证码' }}
              </el-button>
            </div>
          </el-form-item>
          
          <el-button
            type="primary"
            size="large"
            class="submit-button"
            :loading="loading"
            @click="handleCodeLogin"
          >
            登录
          </el-button>
        </el-form>
        
        <!-- 删除整个快速登录区域 -->
        
        <div class="register-link">
          还没有账号? <el-button type="primary" text @click="goRegister">立即注册</el-button>
        </div>
      </div>
    </div>
    
    <!-- 页脚 -->
    <div class="footer">
      <div class="theme-toggle">
        <el-button type="primary" text>
          <el-icon><Moon /></el-icon>
        </el-button>
      </div>
      <div class="copyright">© 2024 智圆社区 版本1.2.0</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock, Right, DataAnalysis, Document, Message, Setting, UserFilled, Connection, Key, Moon } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { authApi, setToken } from '../../api'
import { setLoginStatus, setRememberMe, getRememberCredentials, getUserInfo, getToken, isLoggedIn } from '../../utils/auth'
import userActionService from '../../utils/userActionService'

const router = useRouter()
const loginFormRef = ref<FormInstance>()
const codeFormRef = ref<FormInstance>()
const loading = ref(false)
const sendingCode = ref(false)
const rememberMe = ref(false)
const loginType = ref('password') // 登录类型：password 或 code
const countdown = ref(0) // 验证码倒计时

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
  
  ElMessage.success('登录成功，欢迎回来！')
  
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
      userActionService.logLogin(loginForm.username, false, response.msg || response.message)
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

function goRegister() {
  router.push('/register')
}

function goForgot() { 
  router.push('/forgot-password') 
}
</script>

<style scoped>
/* ===== 使用统一主题系统 ===== */
/* 所有主题变量现在由 theme-variables.css 统一管理 */

/* ===== 主容器样式 ===== */
.login-container {
  min-height: 100vh;
  position: relative;
  overflow: hidden;
  background: linear-gradient(135deg, 
    hsl(220, 20%, 97%) 0%, 
    hsl(220, 20%, 95%) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
}

:global(html.dark) .login-container {
  background: linear-gradient(135deg, 
    hsl(220, 20%, 8%) 0%, 
    hsl(220, 20%, 12%) 100%);
}

/* ===== 背景动效 ===== */
.animated-background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: -1;
  overflow: hidden;
}

.gradient-circle {
  position: absolute;
  border-radius: 50%;
  opacity: 0.1;
  animation: float 15s infinite ease-in-out;
}

.circle-1 {
  width: 300px;
  height: 300px;
  background: rgba(99, 102, 241, 0.3);
  top: -50px;
  left: -100px;
  animation-delay: -2s;
}

.circle-2 {
  width: 400px;
  height: 400px;
  background: rgba(168, 85, 247, 0.2);
  bottom: 100px;
  right: -150px;
  animation-delay: -5s;
}

.circle-3 {
  width: 250px;
  height: 250px;
  background: rgba(236, 72, 153, 0.2);
  top: 300px;
  left: 50%;
  transform: translateX(-50%);
  animation-delay: -8s;
}

.pattern-grid {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: 
    radial-gradient(circle at 20% 20%, rgba(99, 102, 241, 0.05) 0%, transparent 50%),
    radial-gradient(circle at 80% 80%, rgba(168, 85, 247, 0.05) 0%, transparent 50%),
    radial-gradient(circle at 60% 40%, rgba(236, 72, 153, 0.05) 0%, transparent 50%);
  opacity: 0.1;
  animation: gridFloat 10s ease-in-out infinite;
}

@keyframes float {
  0% { transform: translateY(0) translateX(0) scale(1); }
  50% { transform: translateY(-20px) translateX(20px) scale(1.1); }
  100% { transform: translateY(0) translateX(0) scale(1); }
}

@keyframes gridFloat {
  0%, 100% { transform: translate(0, 0) rotate(0deg); }
  33% { transform: translate(-10px, -10px) rotate(1deg); }
  66% { transform: translate(10px, -5px) rotate(-1deg); }
}

/* ===== 登录卡片样式 ===== */
.login-card {
  width: 100%;
  max-width: 500px;
  background: var(--bg-glass);
  backdrop-filter: var(--glass-backdrop);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-2xl);
  box-shadow: var(--shadow-glass);
  overflow: hidden;
  position: relative;
  z-index: 2;
}

:global(html.dark) .login-card {
  background: var(--bg-glass);
}

/* ===== 右侧表单区域样式 ===== */
.login-form-section {
  padding: var(--space-8);
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.login-header {
  text-align: center;
  margin-bottom: var(--space-8);
}

.login-title {
  font-size: var(--font-size-3xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 var(--space-3) 0;
}

.login-subtitle {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* ===== 登录选项切换 ===== */
.login-options {
  display: flex;
  gap: var(--space-4);
  margin-bottom: var(--space-8);
  border-bottom: 1px solid var(--border-light);
  padding-bottom: var(--space-4);
}

.login-option {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-6);
  border-radius: var(--radius-lg);
  font-size: var(--font-size-base);
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: var(--transition-fast);
  border: 1px solid transparent;
}

.login-option.active {
  color: var(--color-primary);
  font-weight: 600;
  background: var(--bg-elevated);
  border-color: var(--color-primary);
  box-shadow: var(--shadow-sm);
}

.login-option:hover {
  color: var(--color-primary);
  text-decoration: underline;
}

/* ===== 表单样式 ===== */
.form {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.form-label {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.form-input :deep(.el-input__wrapper) {
  border-radius: var(--radius-xl);
  border: 2px solid var(--border-light);
  box-shadow: var(--shadow-sm);
  transition: var(--transition-normal);
  padding: var(--space-4);
}

.form-input :deep(.el-input__wrapper):hover {
  border-color: var(--color-primary);
  box-shadow: var(--shadow-md);
}

.form-input :deep(.el-input__wrapper.is-focus) {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

/* ===== 注册链接 ===== */
.register-link {
  margin-top: var(--space-6);
  text-align: center;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

/* ===== 表单选项 ===== */
.form-options {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

/* ===== 快速登录分隔线 ===== */
.divider {
  display: flex;
  align-items: center;
  text-align: center;
  margin: var(--space-6) 0;
  color: var(--text-muted);
  font-size: var(--font-size-sm);
}

.divider::before,
.divider::after {
  content: '';
  flex: 1;
  border-bottom: 1px solid var(--border-light);
}

.divider::before {
  margin-right: var(--space-4);
}

.divider::after {
  margin-left: var(--space-4);
}

/* ===== 快速登录按钮 ===== */
.quick-options {
  display: flex;
  justify-content: center;
  gap: var(--space-4);
}

.quick-options .el-button {
  flex: 1;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  transition: var(--transition-normal);
  font-size: var(--font-size-sm);
  border: 1px solid var(--border-color);
}

.quick-options .el-button:hover {
  border-color: var(--color-primary);
  color: var(--color-primary);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.remember-checkbox {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.forgot-link {
  font-size: var(--font-size-sm);
  color: var(--color-primary);
  text-decoration: none;
  font-weight: 500;
  transition: var(--transition-fast);
}

.forgot-link:hover {
  color: var(--color-primary-dark);
  text-decoration: underline;
}

.submit-button {
  width: 100%;
  height: 56px;
  border-radius: var(--radius-xl);
  font-size: var(--font-size-lg);
  font-weight: 600;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border: none;
  box-shadow: var(--shadow-lg);
  transition: var(--transition-normal);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  position: relative;
  overflow: hidden;
}

.submit-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.submit-button:hover::before {
  left: 100%;
}

.submit-button:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-xl);
}

.submit-button:active {
  transform: translateY(0);
}

/* ===== 验证码输入组样式 ===== */
.code-input-group {
  display: flex;
  gap: var(--space-3);
  align-items: flex-end;
}

.code-input {
  flex: 1;
}

.send-code-btn {
  height: 56px;
  padding: 0 var(--space-6);
  white-space: nowrap;
  border-radius: var(--radius-xl);
  font-size: var(--font-size-sm);
  font-weight: 600;
  min-width: 140px;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border: none;
  box-shadow: var(--shadow-md);
  transition: var(--transition-normal);
}

.send-code-btn:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

/* ===== 快速登录区域样式 ===== */
.quick-login {
  margin: var(--space-8) 0;
  text-align: center;
}

.divider {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.divider-line {
  flex: 1;
  height: 1px;
  background: var(--border-color);
}

.divider-text {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  font-weight: 500;
  padding: 0 var(--space-2);
}

.quick-options {
  display: flex;
  gap: var(--space-4);
}

.quick-btn {
  flex: 1;
  height: 48px;
  border-radius: var(--radius-xl);
  font-size: var(--font-size-sm);
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  transition: var(--transition-normal);
  position: relative;
  overflow: hidden;
}

.admin-btn {
  background: linear-gradient(135deg, var(--color-success), #10b981);
  border: none;
  color: white;
  box-shadow: var(--shadow-md);
}

.user-btn {
  background: linear-gradient(135deg, var(--color-warning), #f59e0b);
  border: none;
  color: white;
  box-shadow: var(--shadow-md);
}

.quick-btn:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

/* ===== 注册区域样式 ===== */
.register-section {
  margin: var(--space-6) 0;
  padding: var(--space-4) 0;
  border-top: 1px solid var(--border-light);
}

.register-prompt {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
}

.prompt-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.register-link {
  font-size: var(--font-size-sm);
  font-weight: 600;
}

/* ===== 页脚样式 ===== */
.footer {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  padding: var(--space-4);
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--bg-elevated);
  border-top: 1px solid var(--border-light);
  z-index: 10;
}

.theme-toggle {
  padding: var(--space-2);
}

.copyright {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

/* ===== 响应式设计 ===== */
@media (max-width: 1024px) {
  .login-card {
    grid-template-columns: 1fr;
    gap: var(--space-8);
  }
  
  .brand-section {
    padding: var(--space-8);
  }
  
  .login-form-section {
    padding: var(--space-8);
  }
  
  .features-list {
    flex-direction: row;
    gap: var(--space-4);
  }
  
  .feature-item {
    flex-direction: column;
    text-align: center;
    padding: var(--space-3);
  }
  
  .feature-text h3 {
    font-size: var(--font-size-sm);
  }
  
  .feature-text p {
    font-size: var(--font-size-xs);
  }
}

@media (max-width: 768px) {
  .login-container {
    padding: var(--space-4);
  }
  
  .brand-section {
    padding: var(--space-6);
  }
  
  .login-form-section {
    padding: var(--space-6);
  }
  
  .brand-logo-container {
    width: 100px;
    height: 100px;
    font-size: var(--font-size-2xl);
  }
  
  .brand-name {
    font-size: var(--font-size-2xl);
  }
  
  .brand-slogan {
    font-size: var(--font-size-base);
  }
  
  .login-title {
    font-size: var(--font-size-2xl);
  }
  
  .features-list {
    flex-direction: column;
    gap: var(--space-3);
  }
  
  .feature-item {
    flex-direction: row;
    text-align: left;
    padding: var(--space-3);
  }
  
  .feature-icon {
    width: 40px;
    height: 40px;
    font-size: var(--font-size-base);
  }
  
  .quick-options {
    flex-direction: column;
    gap: var(--space-3);
  }
  
  .code-input-group {
    flex-direction: column;
    gap: var(--space-3);
  }
  
  .send-code-btn {
    width: 100%;
  }
}

@media (max-width: 480px) {
  .login-container {
    padding: var(--space-3);
  }
  
  .brand-section,
  .login-form-section {
    padding: var(--space-4);
  }
  
  .login-options {
    flex-direction: column;
    gap: var(--space-3);
  }
  
  .login-option {
    padding: var(--space-2) var(--space-4);
    font-size: var(--font-size-sm);
  }
  
  .form-input :deep(.el-input__wrapper) {
    padding: var(--space-3);
  }
  
  .submit-button {
    height: 48px;
    font-size: var(--font-size-base);
  }
  
  .send-code-btn {
    height: 48px;
    min-width: 120px;
  }
}

/* ===== 动画优化 ===== */
@media (prefers-reduced-motion: reduce) {
  .brand-logo-container {
    animation: none;
  }
  
  .submit-button:hover,
  .quick-btn:hover,
  .feature-item:hover {
    transform: none;
  }
  
  * {
    transition: none !important;
    animation: none !important;
  }
}
</style> 