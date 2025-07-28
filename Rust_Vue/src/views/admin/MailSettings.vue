<template>
  <div class="mail-settings">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>邮件服务配置</span>
          <el-button type="primary" @click="saveSettings" :loading="saving">
            {{ saving ? '保存中...' : '保存配置' }}
          </el-button>
        </div>
      </template>
      
      <el-form :model="mailForm" :rules="rules" ref="formRef" label-width="120px">
        <el-form-item label="启用邮件服务">
          <el-switch v-model="mailForm.enabled" active-text="启用" inactive-text="禁用" />
          <div class="form-tip">开启后才能发送邮件</div>
        </el-form-item>

        <el-form-item label="SMTP服务器" prop="smtp_server">
          <el-input v-model="mailForm.smtp_server" placeholder="如: smtp.qq.com" />
        </el-form-item>
        
        <el-form-item label="SMTP端口" prop="smtp_port">
          <el-input-number v-model="mailForm.smtp_port" :min="1" :max="65535" />
          <div class="form-tip">常用端口：25(非加密)、465(SSL)、587(STARTTLS)</div>
        </el-form-item>

        <el-form-item label="SSL/TLS加密">
          <el-switch v-model="mailForm.use_ssl" active-text="启用" inactive-text="禁用" />
          <div class="form-tip">465端口通常需要SSL，587端口使用STARTTLS</div>
        </el-form-item>

        <el-form-item label="SMTP认证">
          <el-switch v-model="mailForm.auth_required" active-text="需要" inactive-text="不需要" />
          <div class="form-tip">大部分SMTP服务器都需要认证</div>
        </el-form-item>
        
        <el-form-item label="发送邮箱" prop="username">
          <el-input v-model="mailForm.username" placeholder="发送邮件的邮箱地址" />
        </el-form-item>
        
        <el-form-item label="SMTP密码" prop="password">
          <el-input v-model="mailForm.password" type="password" placeholder="SMTP授权码(不是登录密码)" show-password />
          <div class="form-tip">QQ邮箱请使用授权码，不是登录密码</div>
        </el-form-item>
        
        <el-form-item label="发送方名称" prop="from_name">
          <el-input v-model="mailForm.from_name" placeholder="邮件中显示的发送方名称" />
        </el-form-item>
        
        <el-divider>测试邮件</el-divider>
        
        <el-form-item label="测试邮箱">
          <el-input v-model="testEmail" placeholder="输入测试邮箱地址" style="width: 300px;" />
          <el-button type="success" @click="sendTestEmail" :loading="testing" style="margin-left: 10px;">
            {{ testing ? '发送中...' : '发送测试邮件' }}
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>
    
    <el-card style="margin-top: 20px;">
      <template #header>
        <span>邮件功能说明</span>
      </template>
      
      <el-alert 
        title="邮件服务用途" 
        type="info" 
        :closable="false"
        description="邮件服务用于发送注册验证码、密码重置链接、新资源通知等功能。">
      </el-alert>
      
      <div style="margin-top: 15px;">
        <h4>QQ邮箱配置步骤：</h4>
        <ol>
          <li>登录QQ邮箱，进入"设置" -> "账户"</li>
          <li>开启"POP3/IMAP/SMTP/Exchange/CardDAV/CalDAV服务"</li>
          <li>获取授权码（不是QQ密码）</li>
          <li>SMTP服务器填写：smtp.qq.com，端口：465</li>
          <li>用户名填写完整的QQ邮箱地址</li>
          <li>密码填写获取的授权码</li>
        </ol>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElForm } from 'element-plus'
import { adminApi } from '@/api'

const formRef = ref<InstanceType<typeof ElForm> | null>(null)
const saving = ref(false)
const testing = ref(false)
const testEmail = ref('')

const mailForm = reactive({
  smtp_server: '',
  smtp_port: 465,
  username: '',
  password: '',
  from_name: '绳包管理器',
  enabled: false,
  use_ssl: true,
  auth_required: true
})

const rules = {
  smtp_server: [
    { required: true, message: '请输入SMTP服务器地址', trigger: 'blur' }
  ],
  smtp_port: [
    { required: true, message: '请输入SMTP端口', trigger: 'blur' },
    { type: 'number' as const, min: 1, max: 65535, message: '端口范围：1-65535', trigger: 'blur' }
  ],
  username: [
    { required: true, message: '请输入发送邮箱', trigger: 'blur' },
    { type: 'email' as const, message: '请输入有效的邮箱地址', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入SMTP密码', trigger: 'blur' }
  ],
  from_name: [
    { required: true, message: '请输入发送方名称', trigger: 'blur' }
  ]
}

onMounted(() => {
  loadSettings()
})

async function loadSettings() {
  try {
    const response = await adminApi.getMailSettings()
    if (response.code === 0 && response.data) {
      Object.assign(mailForm, response.data)
    }
  } catch (error) {
    console.error('加载邮件设置失败:', error)
  }
}

async function saveSettings() {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    saving.value = true
    
    const response = await adminApi.updateMailSettings(mailForm)
    if (response.code === 0) {
      ElMessage.success('邮件配置保存成功')
    } else {
      ElMessage.error(response.message || '保存失败')
    }
  } catch (error: any) {
    ElMessage.error('保存失败：' + (error.message || '未知错误'))
  } finally {
    saving.value = false
  }
}

async function sendTestEmail() {
  if (!testEmail.value) {
    ElMessage.warning('请输入测试邮箱地址')
    return
  }
  
  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(testEmail.value)) {
    ElMessage.warning('请输入有效的邮箱地址')
    return
  }
  
  try {
    testing.value = true
    // 调用发送测试邮件的API
    const response = await adminApi.sendTestEmail(testEmail.value)
    if (response.code === 0) {
      ElMessage.success('测试邮件发送成功，请检查邮箱')
    } else {
      ElMessage.error(response.message || '发送失败')
    }
  } catch (error: any) {
    ElMessage.error('发送失败：' + (error.message || '未知错误'))
  } finally {
    testing.value = false
  }
}
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.form-tip {
  font-size: 12px;
  color: #909399;
  margin-top: 5px;
}

:deep(.el-form-item__content) {
  flex-direction: column;
  align-items: flex-start;
}

:deep(.el-input-number) {
  width: 200px;
}
</style> 