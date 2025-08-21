<template>
  <div class="forgot-container">
    <h2>忘记密码</h2>
    <el-form :model="form" :rules="rules" ref="formRef" label-width="80px" style="max-width:360px;">
      <el-form-item label="邮箱" prop="email">
        <el-input v-model="form.email" placeholder="请输入注册邮箱" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" :loading="loading" @click="onSubmit">{{ loading?'发送中...':'发送重置邮件' }}</el-button>
        <el-button @click="goLogin">返回登录</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElForm, ElMessage } from 'element-plus'
import { authApi } from '@/api'

const router = useRouter()
const formRef = ref<InstanceType<typeof ElForm>|null>(null)
const loading = ref(false)
const form = reactive({ email:'' })
const rules = { 
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' }, 
    { type: 'email' as const, message: '格式不正确', trigger: 'blur' }
  ] 
}

async function onSubmit(){
  if(!formRef.value) return
  await formRef.value.validate()
  loading.value=true
  const res = await authApi.resetRequest(form.email)
  loading.value=false
  if(res.code===0){ ElMessage.success('已发送重置邮件，请检查邮箱') } else { ElMessage.error(res.message||'发送失败') }
}
function goLogin(){ router.push('/login') }
</script>

<style scoped>
.forgot-container{ display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;gap:16px; }
</style> 