<template>
  <div class="register-container">
    <!-- 背景动效 -->
    <div class="animated-background">
      <div class="gradient-circle circle-1"></div>
      <div class="gradient-circle circle-2"></div>
      <div class="gradient-circle circle-3"></div>
      <div class="pattern-grid"></div>
    </div>

    <!-- 注册卡片 -->
    <div class="register-card">
      <!-- 注册表单 -->
      <div class="register-form-section">
        <div class="register-header">
          <h2 class="register-title">创建账号</h2>
          <p class="register-subtitle">加入智圆社区，开启知识分享之旅</p>
        </div>

        <el-form 
          ref="registerFormRef" 
          :model="registerForm" 
          :rules="registerRules" 
          class="register-form"
          @submit.prevent="handleRegister"
        >
          <!-- 基本信息 -->
          <div class="form-section">
            <div class="section-header">
              <el-icon><User /></el-icon>
              <h3>基本信息</h3>
            </div>
            
            <el-form-item prop="username">
              <el-input 
                v-model="registerForm.username" 
                placeholder="用户名" 
                size="large" 
                clearable
                :prefix-icon="User"
              />
            </el-form-item>
            
            <div class="form-row">
              <el-form-item prop="password">
                <el-input 
                  v-model="registerForm.password" 
                  type="password" 
                  placeholder="密码" 
                  size="large" 
                  show-password 
                  clearable
                  :prefix-icon="Lock"
                />
              </el-form-item>
              
              <el-form-item prop="confirmPassword">
                <el-input 
                  v-model="registerForm.confirmPassword" 
                  type="password" 
                  placeholder="确认密码" 
                  size="large" 
                  show-password 
                  clearable
                  :prefix-icon="Lock"
                />
              </el-form-item>
            </div>
          </div>

          <!-- 联系信息 -->
          <div class="form-section">
            <div class="section-header">
              <el-icon><Message /></el-icon>
              <h3>联系信息</h3>
            </div>
            
            <el-form-item prop="email">
              <el-input 
                v-model="registerForm.email" 
                placeholder="邮箱地址" 
                size="large" 
                clearable
                :prefix-icon="Message"
              />
            </el-form-item>
            
            <div class="form-row">
              <el-form-item prop="nickname">
                <el-input 
                  v-model="registerForm.nickname" 
                  placeholder="昵称（可选）" 
                  size="large" 
                  clearable
                  :prefix-icon="UserFilled"
                />
              </el-form-item>
              
              <el-form-item prop="qq_number">
                <el-input 
                  v-model="registerForm.qq_number" 
                  placeholder="QQ号（可选）" 
                  size="large" 
                  clearable
                  :prefix-icon="ChatDotRound"
                />
              </el-form-item>
            </div>
          </div>

          <!-- 验证码 -->
          <div class="form-section">
            <div class="section-header">
              <el-icon><Warning /></el-icon>
              <h3>安全验证</h3>
            </div>
            
            <el-form-item prop="code">
              <div class="code-input-group">
                <el-input 
                  v-model="registerForm.code" 
                  placeholder="验证码" 
                  size="large" 
                  clearable
                  :prefix-icon="Key"
                />
                <el-button 
                  type="primary" 
                  class="code-button"
                  :loading="sending"
                  :disabled="countdown > 0 || !isValidEmail(registerForm.email)"
                  @click="onSendCode"
                >
                  {{ countdown > 0 ? `${countdown}s` : '发送验证码' }}
                </el-button>
              </div>
            </el-form-item>
          </div>

          <!-- 用户协议 -->
          <div class="agreement-section">
            <el-checkbox v-model="agreeToTerms" size="large">
              我已阅读并同意
              <el-button type="primary" text @click.prevent>《用户协议》</el-button>
              和
              <el-button type="primary" text @click.prevent>《隐私政策》</el-button>
            </el-checkbox>
          </div>

          <!-- 注册按钮 -->
          <el-button 
            type="primary" 
            size="large" 
            class="submit-button" 
            :loading="loading"
            :disabled="!agreeToTerms"
            @click="handleRegister"
          >
            {{ loading ? '注册中...' : '立即注册' }}
          </el-button>
        </el-form>
        
        <!-- 登录链接 -->
        <div class="login-link">
          已有账号? <el-button type="primary" text @click="goLogin">立即登录</el-button>
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
import {
  User,
  UserFilled,
  Lock,
  Message,
  Share,
  Trophy,
  Connection,
  Key,
  ChatDotRound,
  Moon,
  Warning,
  Position
} from '@element-plus/icons-vue'
import { authApi } from '../../api'
import userActionService from '../../utils/userActionService'

const router = useRouter()
const registerFormRef = ref()
const loading = ref(false)
const sending = ref(false)
const countdown = ref(0)
const agreeToTerms = ref(false)

const registerForm = reactive({
  username: '',
  password: '',
  confirmPassword: '',
  nickname: '',
  qq_number: '',
  email: '',
  code: ''
})

const registerRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' },
    { pattern: /^[a-zA-Z0-9_\u4e00-\u9fa5]+$/, message: '用户名只能包含字母、数字、下划线和中文', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' },
    { pattern: /^(?=.*[a-zA-Z])(?=.*\d)/, message: '密码必须包含字母和数字', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认密码', trigger: 'blur' },
    { validator: (rule: any, value: string, callback: any) => {
      if (value !== registerForm.password) {
        callback(new Error('两次输入的密码不一致'))
      } else {
        callback()
      }
    }, trigger: 'blur' }
  ],
  nickname: [
    { max: 20, message: '昵称长度不能超过 20 个字符', trigger: 'blur' }
  ],
  qq_number: [
    { pattern: /^[1-9][0-9]{4,11}$/, message: '请输入有效的QQ号', trigger: 'blur' }
  ],
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '邮箱格式不正确', trigger: 'blur' }
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

// 发送验证码
async function onSendCode() {
  if (!registerForm.email) {
    ElMessage.warning('请先输入邮箱')
    return
  }
  
  if (!isValidEmail(registerForm.email)) {
    ElMessage.error('请输入正确的邮箱格式')
    return
  }

  try {
    sending.value = true
    const res = await authApi.sendRegisterCode({ email: registerForm.email })
    if (res.code === 0) {
      ElMessage.success('验证码已发送，请查收邮件')
      startCountdown()
    } else {
      ElMessage.error(res.message || '发送失败')
    }
  } catch (error) {
    console.error('发送验证码失败:', error)
    ElMessage.error('发送验证码失败，请稍后重试')
  } finally {
    sending.value = false
  }
}

// 处理注册
async function handleRegister() {
  if (!registerFormRef.value) return
  
  if (!agreeToTerms.value) {
    ElMessage.warning('请先同意用户协议和隐私政策')
    return
  }
  
  try {
    await registerFormRef.value.validate()
    loading.value = true
    
    const result = await authApi.register({
      username: registerForm.username,
      password: registerForm.password,
      nickname: registerForm.nickname || null,
      qq_number: registerForm.qq_number || null,
      email: registerForm.email,
      verification_code: registerForm.code
    })
    
    if (result.code === 0) {
      ElMessage.success('注册成功！正在跳转到登录页面...')
      
      // 记录用户行为
      await userActionService.logRegister(registerForm.username, true, `用户注册成功: ${registerForm.email}`)
        .catch(err => console.error('记录注册行为失败:', err))
      
      // 延迟跳转，让用户看到成功消息
      setTimeout(() => {
        router.push('/login')
      }, 1500)
    } else {
      ElMessage.error(result.message || '注册失败')
    }
  } catch (error: any) {
    console.error('注册失败:', error)
    
    // 尝试从后端响应中获取具体错误信息
    if (error.response && error.response.data && error.response.data.message) {
      ElMessage.error(error.response.data.message)
    } else if (error.message) {
      ElMessage.error(error.message)
    } else {
      ElMessage.error('注册失败，请重试')
    }
  } finally {
    loading.value = false
  }
}

function goLogin() {
  router.push('/login')
}
</script>

<style scoped>
/* ===== 主容器样式 ===== */
.register-container {
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

:global(html.dark) .register-container {
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

/* ===== 注册卡片样式 ===== */
.register-card {
  width: 100%;
  max-width: 600px;
  background: var(--bg-glass);
  backdrop-filter: var(--glass-backdrop);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-2xl);
  box-shadow: var(--shadow-glass);
  overflow: hidden;
  position: relative;
  z-index: 2;
}

:global(html.dark) .register-card {
  background: var(--bg-glass);
}

/* ===== 表单区域样式 ===== */
.register-form-section {
  padding: var(--space-8);
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.register-header {
  text-align: center;
  margin-bottom: var(--space-8);
}

.register-title {
  font-size: var(--font-size-3xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 var(--space-3) 0;
}

.register-subtitle {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* ===== 表单样式 ===== */
.register-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.form-section {
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-6);
  margin-bottom: var(--space-4);
  border: 1px solid var(--border-color-light);
}

.section-header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
  padding-bottom: var(--space-3);
  border-bottom: 1px solid var(--border-color-light);
  color: var(--color-primary);
}

.section-header h3 {
  font-size: var(--font-size-base);
  margin: 0;
  font-weight: 600;
  color: var(--text-primary);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-4);
}

/* ===== 验证码输入组 ===== */
.code-input-group {
  display: flex;
  gap: var(--space-3);
}

.code-button {
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border: none;
  white-space: nowrap;
  min-width: 120px;
  transition: var(--transition-normal);
  border-radius: var(--radius-lg);
}

.code-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

/* ===== 协议区域 ===== */
.agreement-section {
  margin: var(--space-4) 0;
  text-align: center;
}

/* ===== 提交按钮 ===== */
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
  margin-top: var(--space-4);
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

.submit-button:hover:not(:disabled)::before {
  left: 100%;
}

.submit-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: var(--shadow-xl);
}

/* ===== 登录链接 ===== */
.login-link {
  margin-top: var(--space-6);
  text-align: center;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

/* ===== 页脚 ===== */
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
@media (max-width: 768px) {
  .register-card {
    max-width: 100%;
  }
  
  .register-form-section {
    padding: var(--space-6);
  }
  
  .form-row {
    grid-template-columns: 1fr;
    gap: var(--space-4);
  }
  
  .code-input-group {
    flex-direction: column;
  }
  
  .code-button {
    width: 100%;
  }
}

@media (max-width: 480px) {
  .register-container {
    padding: var(--space-3);
  }
  
  .register-form-section {
    padding: var(--space-4);
  }
  
  .form-section {
    padding: var(--space-4);
  }
  
  .submit-button {
    height: 48px;
  }
}

/* ===== 动画优化 ===== */
@media (prefers-reduced-motion: reduce) {
  * {
    transition: none !important;
    animation: none !important;
  }
}
</style> 