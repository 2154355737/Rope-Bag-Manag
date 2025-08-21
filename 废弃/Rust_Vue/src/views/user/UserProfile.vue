<template>
  <div class="user-profile">
    <!-- 个人信息编辑 -->
    <el-card shadow="hover" class="profile-card">
      <template #header>
        <div class="card-header">
          <h3>个人信息</h3>
          <el-button type="primary" size="small" @click="showEditDialog = true">编辑信息</el-button>
        </div>
      </template>
      
      <div class="profile-display" v-loading="loading">
        <div class="profile-info">
          <div class="avatar-section">
            <el-avatar :size="80" :src="userProfile.avatar_url" v-if="userProfile.avatar_url">
              <img :src="userProfile.avatar_url" @error="handleAvatarError" />
            </el-avatar>
            <el-avatar :size="80" v-else>
              <el-icon><User /></el-icon>
            </el-avatar>
          </div>
          <div class="info-section">
            <div class="info-item">
              <span class="label">用户名：</span>
              <span class="value">{{ userProfile.username || '未设置' }}</span>
            </div>
            <div class="info-item">
              <span class="label">昵称：</span>
              <span class="value">{{ userProfile.nickname || '未设置' }}</span>
            </div>
            <div class="info-item">
              <span class="label">QQ号：</span>
              <span class="value">{{ userProfile.qq_number || '未设置' }}</span>
            </div>
            <div class="info-item">
              <span class="label">积分：</span>
              <span class="value highlight">{{ userProfile.star || 0 }}</span>
            </div>
            <div class="info-item">
              <span class="label">注册时间：</span>
              <span class="value">{{ formatDate(userProfile.created_at) }}</span>
            </div>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 资源订阅设置 -->
    <el-card shadow="hover" class="subscription-card">
      <template #header>
        <div class="card-header">
          <h3>资源订阅</h3>
          <el-button link size="small" @click="loadCategories">刷新</el-button>
        </div>
      </template>
      
      <div class="subscription-content">
        <p class="subscription-desc">订阅您感兴趣的资源分类，我们会在有新资源时通知您。</p>
        <el-table :data="categoryList" style="width: 100%;" v-loading="categoryLoading">
          <el-table-column prop="name" label="分类名称" />
          <el-table-column prop="description" label="分类描述" show-overflow-tooltip />
          <el-table-column label="资源数量" width="100">
            <template #default="{ row }">
              <el-tag size="small">{{ row.count || 0 }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column label="订阅状态" width="100">
            <template #default="{ row }">
              <el-switch 
                v-model="subscribed[row.id]" 
                @change="(val: boolean) => onToggle(row.id, val)"
                :loading="toggleLoading[row.id]"
              />
            </template>
          </el-table-column>
        </el-table>
      </div>
    </el-card>

    <!-- 编辑信息对话框 -->
    <el-dialog v-model="showEditDialog" title="编辑个人信息" width="500px">
      <el-form :model="editForm" :rules="rules" ref="formRef" label-width="90px">
        <el-form-item label="昵称" prop="nickname">
          <el-input v-model="editForm.nickname" placeholder="请输入昵称" />
        </el-form-item>
        <el-form-item label="QQ号" prop="qq_number">
          <el-input v-model="editForm.qq_number" placeholder="请输入QQ号" />
        </el-form-item>
        <el-form-item label="头像" prop="avatar_url">
          <div class="avatar-upload-section">
            <el-upload
              class="avatar-uploader"
              action=""
              :show-file-list="false"
              :before-upload="handleAvatarUpload"
              accept="image/*"
            >
              <img v-if="editForm.avatar_url" :src="editForm.avatar_url" class="avatar-preview" @error="handleAvatarError" />
              <div v-else class="avatar-uploader-icon">
                <el-icon><Plus /></el-icon>
                <span>上传头像</span>
              </div>
            </el-upload>
            <div class="avatar-input">
              <el-input 
                v-model="editForm.avatar_url" 
                placeholder="或粘贴头像图片URL" 
                clearable
              />
            </div>
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showEditDialog = false">取消</el-button>
          <el-button type="primary" @click="onSave" :loading="saving">保存</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElForm } from 'element-plus'
import { User, Plus } from '@element-plus/icons-vue'
import { categoryApi, subscriptionApi } from '@/api'
import type { UserSubscriptions } from '@/api/subscriptions'
import { userApi, type UpdateUserRequest } from '@/api/users'
import { getUserInfo, refreshUserInfo } from '@/utils/auth'

interface CategoryWithCount { 
  id: number
  name: string
  description: string | null
  enabled: boolean
  subscription_locked: boolean
  created_at: string
  count?: number
}

const formRef = ref<InstanceType<typeof ElForm> | null>(null)
const loading = ref(false)
const categoryLoading = ref(false)
const saving = ref(false)
const showEditDialog = ref(false)

// 用户信息
const userProfile = ref<any>({})

// 分类列表
const categoryList = ref<CategoryWithCount[]>([])
const subscribed = reactive<Record<number, boolean>>({})
const toggleLoading = reactive<Record<number, boolean>>({})

// 编辑表单
const editForm = reactive<UpdateUserRequest>({
  nickname: '',
  qq_number: '',
  avatar_url: ''
})

const rules = {
  nickname: [
    { required: true, message: '请输入昵称', trigger: 'blur' },
    { min: 2, max: 20, message: '昵称长度在 2 到 20 个字符之间', trigger: 'blur' }
  ],
  qq_number: [
    { pattern: /^\d{5,12}$/, message: '请输入5-12位数字的QQ号', trigger: 'blur' }
  ]
}

// 加载用户信息
const loadUserProfile = async () => {
  loading.value = true
  try {
    const res = await userApi.getCurrentUser()
    if (res.code === 0 && res.data) {
      userProfile.value = res.data
      // 同时更新编辑表单
      editForm.nickname = res.data.nickname || ''
      editForm.qq_number = res.data.qq_number || ''
      editForm.avatar_url = res.data.avatar_url || ''
    }
  } catch (error) {
    console.error('加载用户信息失败:', error)
    ElMessage.error('加载用户信息失败')
  } finally {
    loading.value = false
  }
}

// 加载分类列表
const loadCategories = async () => {
  categoryLoading.value = true
  try {
    const res = await categoryApi.getCategories()
    if (res.code === 0) {
      categoryList.value = res.data?.list || []
      // 加载用户订阅状态
      await loadUserSubscriptions()
    }
  } catch (error) {
    console.error('加载分类列表失败:', error)
    ElMessage.error('加载分类列表失败')
  } finally {
    categoryLoading.value = false
  }
}

// 加载用户订阅状态
const loadUserSubscriptions = async () => {
  try {
    const res = await subscriptionApi.getUserSubscriptions()
    if (res.code === 0 && res.data) {
      // 更新订阅状态
      Object.assign(subscribed, res.data)
      // 确保所有分类都有订阅状态（默认为false）
      categoryList.value.forEach(c => {
        if (subscribed[c.id] === undefined) {
          subscribed[c.id] = false
        }
      })
    }
  } catch (error) {
    console.error('加载用户订阅状态失败:', error)
    // 初始化默认状态
    categoryList.value.forEach(c => { 
      if (subscribed[c.id] === undefined) {
        subscribed[c.id] = false 
      }
    })
  }
}

// 切换订阅状态
const onToggle = async (catId: number, enabled: boolean) => {
  toggleLoading[catId] = true
  try {
    const res = await subscriptionApi.setSubscription({ category_id: catId, enabled })
    if (res.code === 0) { 
      ElMessage.success(enabled ? '已订阅' : '已取消订阅')
    } else { 
      ElMessage.error(res.message || '操作失败')
      subscribed[catId] = !enabled // 恢复原状态
    }
  } catch (error) {
    console.error('切换订阅状态失败:', error)
    ElMessage.error('操作失败')
    subscribed[catId] = !enabled // 恢复原状态
  } finally {
    toggleLoading[catId] = false
  }
}

// 头像上传
const handleAvatarUpload = (file: File) => {
  const isImage = file.type.startsWith('image/')
  const isLt2M = file.size / 1024 / 1024 < 2

  if (!isImage) {
    ElMessage.error('只能上传图片文件!')
    return false
  }
  if (!isLt2M) {
    ElMessage.error('图片大小不能超过 2MB!')
    return false
  }

  const reader = new FileReader()
  reader.onload = (e) => {
    editForm.avatar_url = e.target?.result as string
  }
  reader.readAsDataURL(file)
  return false // 阻止自动上传
}

// 头像加载失败处理
const handleAvatarError = (event: Event) => {
  console.warn('头像加载失败:', event)
  // 如果是QQ头像请求失败，我们可以选择隐藏错误的图片
  const target = event.target as HTMLImageElement
  if (target && target.src.includes('q.qlogo.cn')) {
    // 对于QQ头像加载失败，设置为null让el-avatar显示默认图标
    target.style.display = 'none'
  }
}

// 保存个人信息
const onSave = async () => {
  if (!formRef.value) return
  try {
    await formRef.value.validate()
    saving.value = true
    const res = await userApi.updateCurrentUser(editForm)
    if (res.code === 0) {
      ElMessage.success('保存成功')
      showEditDialog.value = false
      await loadUserProfile()
      // 刷新全局用户信息
      await refreshUserInfo()
    } else {
      ElMessage.error(res.message || res.msg || '保存失败')
    }
  } catch (error) {
    console.error('保存个人信息失败:', error)
    ElMessage.error('保存失败')
  } finally {
    saving.value = false
  }
}

// 格式化日期
const formatDate = (dateStr: string) => {
  if (!dateStr) return '未知'
  return new Date(dateStr).toLocaleDateString('zh-CN')
}

onMounted(() => {
  loadUserProfile()
  loadCategories()
})
</script>

<style scoped>
.user-profile {
  padding: 24px;
  background-color: #f5f7fa;
  min-height: calc(100vh - 60px);
}

.profile-card,
.subscription-card {
  margin-bottom: 24px;
  border-radius: 8px;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.card-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.profile-display {
  padding: 16px 0;
}

.profile-info {
  display: flex;
  gap: 24px;
  align-items: flex-start;
}

.avatar-section {
  display: flex;
  justify-content: center;
  flex-shrink: 0;
}

.info-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.label {
  font-weight: 500;
  color: #606266;
  min-width: 80px;
}

.value {
  color: #303133;
  font-size: 14px;
}

.value.highlight {
  color: #409EFF;
  font-weight: 600;
}

.subscription-content {
  padding: 8px 0;
}

.subscription-desc {
  margin: 0 0 16px 0;
  color: #606266;
  font-size: 14px;
}

.avatar-upload-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.avatar-uploader {
  display: inline-block;
}

.avatar-uploader :deep(.el-upload) {
  border: 1px dashed #d9d9d9;
  border-radius: 6px;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  transition: 0.2s;
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.avatar-uploader :deep(.el-upload:hover) {
  border-color: #409EFF;
}

.avatar-preview {
  width: 80px;
  height: 80px;
  border-radius: 6px;
  object-fit: cover;
}

.avatar-uploader-icon {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  color: #8c939d;
  font-size: 12px;
}

.avatar-uploader-icon .el-icon {
  font-size: 24px;
}

.avatar-input {
  flex: 1;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

@media screen and (max-width: 768px) {
  .profile-info {
    flex-direction: column;
    align-items: center;
    text-align: center;
  }
  
  .info-section {
    width: 100%;
  }
}
</style> 