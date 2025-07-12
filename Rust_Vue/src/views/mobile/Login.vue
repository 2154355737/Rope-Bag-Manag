<template>
  <div class="login-mobile">
    <!-- 背景装饰 -->
    <div class="background-decoration">
      <div class="decoration-circle circle-1"></div>
      <div class="decoration-circle circle-2"></div>
      <div class="decoration-circle circle-3"></div>
      <div class="decoration-circle circle-4"></div>
      <div class="floating-particles">
        <div class="particle" v-for="i in 6" :key="i" :style="{ animationDelay: `${i * 0.5}s` }"></div>
      </div>
    </div>

    <div class="login-container">
      <!-- 头部Logo区域 -->
      <div class="login-header">
        <div class="logo-container">
          <div class="logo-icon">
            <el-icon :size="48"><Box /></el-icon>
          </div>
          <h1 class="app-title">绳包管理系统</h1>
          <p class="app-subtitle">专业的绳包管理解决方案</p>
        </div>
      </div>

      <!-- 登录表单 -->
      <div class="login-form">
        <div class="form-header">
          <h2 class="form-title">欢迎登录</h2>
          <p class="form-subtitle">请输入您的账号信息</p>
        </div>

        <el-form 
          ref="loginFormRef" 
          :model="loginForm" 
          :rules="loginRules" 
          class="login-form-content"
          @submit.prevent="handleLogin"
        >
          <div class="form-group">
            <label class="form-label">用户名</label>
            <el-input
              v-model="loginForm.username"
              placeholder="请输入用户名"
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
            <a href="#" class="forgot-link">忘记密码？</a>
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

        <!-- 快速登录选项 -->
        <div class="quick-login">
          <div class="quick-login-title">
            <span class="divider-line"></span>
            <span class="divider-text">快速登录</span>
            <span class="divider-line"></span>
          </div>
          
          <div class="quick-login-buttons">
            <el-button 
              class="quick-btn admin-btn" 
              @click="quickLogin('admin', 'admin123')"
            >
              <el-icon><User /></el-icon>
              管理员
            </el-button>
            
            <el-button 
              class="quick-btn user-btn" 
              @click="quickLogin('user', 'user123')"
            >
              <el-icon><User /></el-icon>
              普通用户
            </el-button>
          </div>
        </div>
      </div>

      <!-- 底部信息 -->
      <div class="login-footer">
        <p class="footer-text">© 2024 绳包管理系统. All rights reserved.</p>
        <p class="footer-version">Version 1.0.0</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { User, Lock, Right, Box } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { login } from '../../api'
import { setLoginStatus } from '../../utils/auth'

const router = useRouter()
const loginFormRef = ref<FormInstance>()
const loading = ref(false)
const rememberMe = ref(false)

// 登录表单数据
const loginForm = reactive({
  username: '',
  password: ''
})

// 表单验证规则
const loginRules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' }
  ]
}

// 处理登录
async function handleLogin() {
  if (!loginFormRef.value) return
  
  try {
    await loginFormRef.value.validate()
    loading.value = true
    
    const response = await login(loginForm.username, loginForm.password)
    
         if (response.code === 0) {
       // 登录成功
       setLoginStatus(loginForm.username)
       
       ElMessage.success('登录成功！')
       
       // 延迟跳转，让用户看到成功消息
       setTimeout(() => {
         router.push('/dashboard')
       }, 1000)
     } else {
      ElMessage.error(response.msg || '登录失败，请检查用户名和密码')
    }
  } catch (error) {
    console.error('登录失败:', error)
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

// 自动填充记住的用户信息
const savedUserInfo = localStorage.getItem('savedUserInfo')
if (savedUserInfo) {
  try {
    const userInfo = JSON.parse(savedUserInfo)
    loginForm.username = userInfo.username || ''
    loginForm.password = userInfo.password || ''
    rememberMe.value = true
  } catch {
    // 忽略解析错误
  }
}
</script>

<style scoped>
.login-mobile {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  position: relative;
  overflow: hidden;
  animation: gradient-shift 10s ease-in-out infinite;
}

@keyframes gradient-shift {
  0%, 100% {
    background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  }
  50% {
    background: linear-gradient(135deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
  }
}

.login-container {
  width: 100%;
  max-width: 400px;
  background: var(--bg-card);
  border-radius: 20px;
  padding: 32px 24px;
  box-shadow: var(--shadow-base);
  border: 1px solid var(--border-color);
  position: relative;
  z-index: 10;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  animation: slide-up 0.8s ease-out;
}

@keyframes slide-up {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 头部Logo区域 */
.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.logo-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.logo-icon {
  width: 80px;
  height: 80px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 8px 32px rgba(64, 158, 255, 0.3);
  animation: float 3s ease-in-out infinite;
  position: relative;
  overflow: hidden;
}

.logo-icon::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(45deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  animation: shine 3s ease-in-out infinite;
}

@keyframes shine {
  0% {
    transform: translateX(-100%) translateY(-100%) rotate(45deg);
  }
  50% {
    transform: translateX(100%) translateY(100%) rotate(45deg);
  }
  100% {
    transform: translateX(-100%) translateY(-100%) rotate(45deg);
  }
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
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.app-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

/* 登录表单 */
.login-form {
  margin-bottom: 24px;
}

.form-header {
  text-align: center;
  margin-bottom: 24px;
}

.form-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.form-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.login-form-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
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

/* 快速登录 */
.quick-login {
  margin-bottom: 24px;
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
  margin-top: 24px;
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

/* 背景装饰 */
.background-decoration {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 1;
}

.decoration-circle {
  position: absolute;
  border-radius: 50%;
  background: linear-gradient(135deg, rgba(64, 158, 255, 0.1) 0%, rgba(103, 194, 58, 0.1) 100%);
  animation: float 6s ease-in-out infinite;
}

.circle-1 {
  width: 120px;
  height: 120px;
  top: 10%;
  left: 10%;
  animation-delay: 0s;
}

.circle-2 {
  width: 80px;
  height: 80px;
  top: 60%;
  right: 15%;
  animation-delay: 2s;
}

.circle-3 {
  width: 60px;
  height: 60px;
  bottom: 20%;
  left: 20%;
  animation-delay: 4s;
}

.circle-4 {
  width: 100px;
  height: 100px;
  top: 30%;
  left: 70%;
  animation-delay: 1s;
}

.floating-particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 2;
}

.particle {
  position: absolute;
  width: 8px;
  height: 8px;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 50%;
  animation: float-particle 6s infinite ease-in-out;
}

@keyframes float-particle {
  0% {
    transform: translateY(0) translateX(0) scale(0.5);
    opacity: 0;
  }
  25% {
    opacity: 0.7;
  }
  50% {
    transform: translateY(-20px) translateX(20px) scale(1);
    opacity: 0.5;
  }
  75% {
    opacity: 0.7;
  }
  100% {
    transform: translateY(0) translateX(0) scale(0.5);
    opacity: 0;
  }
}

/* 深色模式适配 */
.dark .login-container {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
}

.dark .decoration-circle {
  background: linear-gradient(135deg, rgba(64, 158, 255, 0.05) 0%, rgba(103, 194, 58, 0.05) 100%);
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
@media (max-width: 480px) {
  .login-mobile {
    padding: 16px;
  }
  
  .login-container {
    padding: 24px 20px;
  }
  
  .logo-icon {
    width: 64px;
    height: 64px;
  }
  
  .app-title {
    font-size: 20px;
  }
  
  .form-title {
    font-size: 18px;
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

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .logo-icon,
  .decoration-circle {
    animation: none;
  }
  
  .login-btn:hover,
  .admin-btn:hover,
  .user-btn:hover {
    transform: none;
  }
}
</style> 