<template>
  <div class="reset-password-container">
    <div class="reset-form-card">
      <h2>重置密码</h2>
      <el-form :model="form" :rules="rules" ref="formRef" label-width="100px">
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="form.email" placeholder="请输入注册邮箱" :disabled="true" />
        </el-form-item>
        <el-form-item label="重置令牌" prop="token">
          <el-input v-model="form.token" placeholder="从邮件中获取的重置令牌" :disabled="true" />
        </el-form-item>
        <el-form-item label="新密码" prop="newPassword">
          <el-input v-model="form.newPassword" type="password" placeholder="请输入新密码" show-password />
        </el-form-item>
        <el-form-item label="确认密码" prop="confirmPassword">
          <el-input v-model="form.confirmPassword" type="password" placeholder="请确认新密码" show-password />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="loading" @click="onSubmit">
            {{ loading ? '重置中...' : '重置密码' }}
          </el-button>
          <el-button @click="goLogin">返回登录</el-button>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElForm, ElMessage } from 'element-plus'
import { authApi } from '@/api'

const router = useRouter()
const route = useRoute()
const formRef = ref<InstanceType<typeof ElForm> | null>(null)
const loading = ref(false)

const form = reactive({
  email: '',
  token: '',
  newPassword: '',
  confirmPassword: ''
})

const rules = {
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { 
      pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/, 
      message: '邮箱格式不正确', 
      trigger: 'blur' 
    }
  ],
  token: [
    { required: true, message: '重置令牌不能为空', trigger: 'blur' }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认密码', trigger: 'blur' },
    {
      validator: (rule: any, value: string, callback: any) => {
        if (value !== form.newPassword) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

onMounted(() => {
  // 从URL参数中获取token和email
  form.token = (route.query.token as string) || ''
  // 对邮箱地址进行URL解码，因为后端发送时进行了编码
  const emailParam = (route.query.email as string) || ''
  form.email = emailParam ? decodeURIComponent(emailParam) : ''
  
  if (!form.token || !form.email) {
    ElMessage.error('重置链接无效或已过期')
    router.push('/forgot-password')
  }
})

async function onSubmit() {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    loading.value = true
    
    const response = await authApi.resetPassword({
      email: form.email,
      token: form.token,
      new_password: form.newPassword
    })
    
    if (response.code === 0) {
      ElMessage.success('密码重置成功，请使用新密码登录')
      router.push('/login')
    } else {
      ElMessage.error(response.message || '密码重置失败')
    }
  } catch (error: any) {
    console.error('密码重置失败:', error)
    ElMessage.error(error.message || '密码重置失败')
  } finally {
    loading.value = false
  }
}

function goLogin() {
  router.push('/login')
}
</script>

<style scoped>
.reset-password-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.reset-form-card {
  background: white;
  padding: 40px;
  border-radius: 12px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
  width: 100%;
  max-width: 400px;
}

.reset-form-card h2 {
  text-align: center;
  margin-bottom: 30px;
  color: #333;
  font-weight: 600;
}

:deep(.el-form-item__label) {
  color: #606266;
  font-weight: 500;
}

:deep(.el-input__inner) {
  border-radius: 6px;
}

:deep(.el-button) {
  border-radius: 6px;
  padding: 12px 20px;
}

:deep(.el-button--primary) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
}

:deep(.el-button--primary:hover) {
  background: linear-gradient(135deg, #5a6fd8 0%, #6a4190 100%);
}
</style> 