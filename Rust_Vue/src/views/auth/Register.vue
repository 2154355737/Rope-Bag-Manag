<template>
  <div class="register-desktop">
    <div class="register-container">
      <div class="register-form-section">
        <div class="form-container">
          <div class="form-header">
            <h2 class="form-title">注册账号</h2>
            <p class="form-subtitle">请填写以下信息完成注册</p>
          </div>
          <el-form ref="registerFormRef" :model="registerForm" :rules="registerRules" class="register-form" @submit.prevent="handleRegister">
            <div class="form-group">
              <label class="form-label">用户名</label>
              <el-input v-model="registerForm.username" placeholder="请输入用户名" size="large" clearable />
            </div>
            <div class="form-group">
              <label class="form-label">密码</label>
              <el-input v-model="registerForm.password" type="password" placeholder="请输入密码" size="large" show-password clearable />
            </div>
            <div class="form-group">
              <label class="form-label">确认密码</label>
              <el-input v-model="registerForm.confirmPassword" type="password" placeholder="请再次输入密码" size="large" show-password clearable />
            </div>
            <div class="form-group">
              <label class="form-label">昵称（可选）</label>
              <el-input v-model="registerForm.nickname" placeholder="请输入昵称（可选）" size="large" clearable />
            </div>
            <div class="form-group">
              <label class="form-label">QQ账号（可选）</label>
              <el-input v-model="registerForm.qq_number" placeholder="请输入QQ号（可选）" size="large" clearable />
            </div>
            <div class="form-group">
              <label class="form-label">邮箱</label>
              <el-input v-model="registerForm.email" placeholder="请输入邮箱" size="large" clearable />
            </div>
            <div class="form-group" style="display:flex;gap:8px;align-items:center;">
              <div style="flex:1;">
                <label class="form-label">验证码</label>
                <el-input v-model="registerForm.code" placeholder="请输入验证码" size="large" clearable />
              </div>
              <el-button type="primary" :disabled="countdown>0 || sending" @click="onSendCode">
                {{ countdown>0 ? countdown+'s' : (sending?'发送中':'获取验证码') }}
              </el-button>
            </div>
            <el-button type="primary" size="large" class="register-btn" :loading="loading" @click="handleRegister">
              {{ loading ? '注册中...' : '注册' }}
            </el-button>
          </el-form>
          <div class="login-link-container" style="text-align:center;margin-top:16px;">
            <el-button type="info" link @click="goLogin">已有账号？去登录</el-button>
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
import { authApi } from '../../api'
import userActionService from '../../utils/userActionService'

const router = useRouter()
const registerFormRef = ref()
const loading = ref(false)

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
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' }
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
    // 昵称为可选字段
  ],
  qq_number: [
    { pattern: /^[1-9][0-9]{4,11}$/, message: '请输入有效的QQ号', trigger: 'blur' }
  ],
  email: [ { required: true, message: '请输入邮箱', trigger: 'blur' }, { type:'email', message:'邮箱格式不正确', trigger:'blur' } ],
  code: [ { required: true, message: '请输入验证码', trigger:'blur' } ]
}

const sending = ref(false)
const countdown = ref(0)
let timer: any = null
function startCountdown(){
  countdown.value = 60
  timer = setInterval(()=>{
    countdown.value--
    if(countdown.value<=0){ clearInterval(timer) }
  },1000)
}
async function onSendCode(){
  if(!registerForm.email){ ElMessage.warning('请先输入邮箱'); return }
  sending.value=true
  try {
    const res = await authApi.sendRegisterCode({ email: registerForm.email })
    if(res.code===0){ 
      ElMessage.success('验证码已发送'); 
      startCountdown() 
    } else { 
      ElMessage.error(res.message||'发送失败') 
    }
  } catch (error) {
    console.error('发送验证码失败:', error)
    ElMessage.error('发送验证码失败，请稍后重试')
  } finally {
  sending.value=false
  }
}

async function handleRegister() {
  if (!registerFormRef.value) return
  
  try {
    await registerFormRef.value.validate()
    loading.value = true
    
    // 直接注册，后端会验证验证码
    const result = await authApi.register({
      username: registerForm.username,
      password: registerForm.password,
      nickname: registerForm.nickname || null,
      qq_number: registerForm.qq_number || null,
      email: registerForm.email,
      verification_code: registerForm.code
    })
    
    if (result.code === 0) {
      ElMessage.success('注册成功！')
      
      // 记录用户行为（暂时禁用，避免401错误）
      // await userActionService.logAction('Register', '用户注册', {
      //   username: registerForm.username,
      //   email: registerForm.email
      // })
      
      // 跳转到登录页面
      router.push('/login')
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
.register-desktop {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--el-bg-color);
}
.register-container {
  width: 420px;
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.08);
  padding: 40px 32px 32px 32px;
}
.form-header {
  text-align: center;
  margin-bottom: 24px;
}
.form-title {
  font-size: 24px;
  font-weight: bold;
  margin-bottom: 8px;
}
.form-subtitle {
  color: var(--el-text-color-secondary);
  font-size: 14px;
}
.form-group {
  margin-bottom: 18px;
}
.form-label {
  display: block;
  margin-bottom: 6px;
  color: var(--el-text-color-regular);
  font-size: 15px;
}
.register-btn {
  width: 100%;
  margin-top: 8px;
}
.login-link-container {
  margin-top: 12px;
}
</style> 