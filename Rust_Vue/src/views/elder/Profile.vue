<template>
  <div class="elder-profile">
    <h2>个人信息编辑</h2>
    <el-form :model="form" :rules="rules" ref="formRef" label-width="90px" style="max-width: 400px;">
      <el-form-item label="昵称" prop="nickname">
        <el-input v-model="form.nickname" placeholder="请输入昵称" />
      </el-form-item>
      <el-form-item label="QQ号" prop="qq_number">
        <el-input v-model="form.qq_number" placeholder="请输入QQ号" />
      </el-form-item>
      <el-form-item label="头像" prop="avatar_url">
        <el-upload
          class="avatar-uploader"
          action=""
          :show-file-list="false"
          :before-upload="handleAvatarUpload"
        >
          <img v-if="form.avatar_url" :src="form.avatar_url" class="avatar" />
          <el-icon v-else><User /></el-icon>
        </el-upload>
        <el-input v-model="form.avatar_url" placeholder="或粘贴头像图片URL" style="margin-top: 8px;" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSave" :loading="saving">保存</el-button>
        <el-button @click="onReset">重置</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElForm } from 'element-plus'
import { userApi, UpdateUserRequest } from '@/api/users'
import { User } from '@/api/types'
import { User as UserIcon } from '@element-plus/icons-vue'

const formRef = ref<InstanceType<typeof ElForm> | null>(null)
const form = reactive<UpdateUserRequest>({
  nickname: '',
  qq_number: '',
  avatar_url: ''
})
const rules = {
  nickname: [ { required: true, message: '请输入昵称', trigger: 'blur' } ],
  qq_number: [ { pattern: /^\d{5,12}$/, message: '请输入有效QQ号', trigger: 'blur' } ]
}
const saving = ref(false)

// 获取当前用户信息
const loadProfile = async () => {
  const res = await userApi.getCurrentUser()
  if (res.code === 0 && res.data) {
    form.nickname = res.data.nickname || ''
    form.qq_number = res.data.qq_number || ''
    form.avatar_url = res.data.avatar_url || ''
  }
}
onMounted(loadProfile)

// 头像上传（本地转base64）
function handleAvatarUpload(file: File) {
  const reader = new FileReader()
  reader.onload = (e) => {
    form.avatar_url = e.target?.result as string
  }
  reader.readAsDataURL(file)
  return false // 阻止自动上传
}

// 保存
const onSave = async () => {
  if (!formRef.value) return
  await formRef.value.validate()
  saving.value = true
  const res = await userApi.updateCurrentUser(form)
  saving.value = false
  if (res.code === 0) {
    ElMessage.success('保存成功')
    loadProfile()
  } else {
    ElMessage.error(res.message || res.msg || '保存失败')
  }
}

// 重置
const onReset = () => {
  loadProfile()
}
</script>

<style scoped>
.elder-profile {
  max-width: 500px;
  margin: 0 auto;
  padding: 32px;
}
.avatar-uploader {
  display: flex;
  align-items: center;
}
.avatar {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  margin-right: 12px;
  object-fit: cover;
  border: 1px solid #eee;
}
</style> 