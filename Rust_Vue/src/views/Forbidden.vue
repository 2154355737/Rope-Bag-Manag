<template>
  <div class="forbidden-container">
    <div class="forbidden-content">
      <!-- 403 图标 -->
      <div class="forbidden-icon">
        <el-icon :size="120" color="#f56565">
          <Lock />
        </el-icon>
      </div>
      
      <!-- 主要信息 -->
      <div class="forbidden-info">
        <h1 class="forbidden-title">403 - 访问被拒绝</h1>
        <p class="forbidden-message">{{ message }}</p>
        
        <!-- 用户信息 -->
        <div v-if="userInfo" class="user-info">
          <el-card shadow="hover">
            <template #header>
              <span>当前用户信息</span>
            </template>
            <el-descriptions :column="2" border>
              <el-descriptions-item label="用户名">{{ userInfo.username }}</el-descriptions-item>
              <el-descriptions-item label="角色">{{ getUserRoleName(userInfo.role) }}</el-descriptions-item>
              <el-descriptions-item label="状态">
                <el-tag :type="getBanStatusType(userInfo.ban_status)">
                  {{ getBanStatusName(userInfo.ban_status) }}
                </el-tag>
              </el-descriptions-item>
              <el-descriptions-item label="访问时间">{{ currentTime }}</el-descriptions-item>
            </el-descriptions>
          </el-card>
        </div>
        
        <!-- 权限说明 -->
        <div class="permission-guide">
          <el-alert
            :title="alertTitle"
            :description="alertDescription"
            type="warning"
            :closable="false"
            show-icon
          />
        </div>
        
        <!-- 操作按钮 -->
        <div class="forbidden-actions">
          <el-button-group>
            <el-button type="primary" @click="goBack">
              <el-icon><ArrowLeft /></el-icon>
              返回上一页
            </el-button>
            <el-button @click="goHome">
              <el-icon><House /></el-icon>
              返回首页
            </el-button>
            <el-button v-if="userInfo" @click="logout">
              <el-icon><SwitchButton /></el-icon>
              切换账户
            </el-button>
            <el-button v-else @click="goLogin">
              <el-icon><User /></el-icon>
              登录
            </el-button>
          </el-button-group>
        </div>
        
        <!-- 联系管理员 -->
        <div class="contact-admin">
          <el-text type="info" size="small">
            如果您认为这是一个错误，请联系系统管理员
          </el-text>
        </div>
      </div>
    </div>
    
    <!-- 安全提示 -->
    <div class="security-notice">
      <el-card shadow="never">
        <template #header>
          <el-icon><InfoFilled /></el-icon>
          安全提示
        </template>
        <ul class="security-tips">
          <li>🔒 系统已记录您的访问尝试</li>
          <li>⚠️ 重复越权访问可能导致账户被限制</li>
          <li>📞 如需申请权限，请联系系统管理员</li>
          <li>🛡️ 请遵守系统使用规范和安全政策</li>
        </ul>
      </el-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { 
  Lock, ArrowLeft, House, SwitchButton, User, InfoFilled 
} from '@element-plus/icons-vue'
import { getUserInfo, logout as authLogout } from '../utils/auth'
import { ElMessage } from 'element-plus'

const router = useRouter()
const userInfo = ref(getUserInfo())
const currentTime = ref(new Date().toLocaleString())

// 根据URL参数或用户状态确定具体的错误信息
const message = computed(() => {
  const query = router.currentRoute.value.query
  
  if (query.error === 'banned') {
    return query.message as string || '您的账户已被封禁'
  }
  
  if (!userInfo.value) {
    return '您需要登录后才能访问此页面'
  }
  
  const banStatus = userInfo.value.ban_status?.toLowerCase()
  if (banStatus && banStatus !== 'normal') {
    return '您的账户状态异常，无法访问此页面'
  }
  
  return '您没有权限访问此页面，请联系管理员获取相应权限'
})

const alertTitle = computed(() => {
  if (!userInfo.value) return '需要登录'
  const banStatus = userInfo.value.ban_status?.toLowerCase()
  if (banStatus && banStatus !== 'normal') return '账户状态异常'
  return '权限不足'
})

const alertDescription = computed(() => {
  if (!userInfo.value) {
    return '此页面需要用户登录后才能访问，请先登录您的账户。'
  }
  
  const banStatus = userInfo.value.ban_status?.toLowerCase()
  if (banStatus && banStatus !== 'normal') {
    return '您的账户当前状态异常，无法正常访问系统功能。如有疑问请联系管理员。'
  }
  
  const targetPath = router.currentRoute.value.query.target as string
  if (targetPath?.includes('/admin')) {
    return '此页面仅限系统管理员访问。如需管理员权限，请联系超级管理员申请。'
  }
  
  if (targetPath?.includes('/elder')) {
    return '此页面仅限元老用户访问。元老权限需要通过社区贡献或管理员指定获得。'
  }
  
  return '您当前的用户角色无法访问此页面。不同页面需要不同的用户权限，请确认您的访问权限。'
})

// 获取用户角色显示名称
function getUserRoleName(role: string): string {
  const roleMap: Record<string, string> = {
    'admin': '系统管理员',
    'elder': '元老用户',
    'moderator': '版主',
    'user': '普通用户',
    'guest': '游客'
  }
  return roleMap[role] || role
}

// 获取封禁状态类型
function getBanStatusType(status: string): string {
  const typeMap: Record<string, string> = {
    'normal': 'success',
    'Normal': 'success',
    'suspended': 'warning',
    'Suspended': 'warning',
    'banned': 'danger',
    'Banned': 'danger'
  }
  return typeMap[status] || 'info'
}

// 获取封禁状态显示名称
function getBanStatusName(status: string): string {
  const statusMap: Record<string, string> = {
    'normal': '正常',
    'Normal': '正常',
    'suspended': '暂停',
    'Suspended': '暂停',
    'banned': '封禁',
    'Banned': '封禁'
  }
  return statusMap[status] || status
}

// 返回上一页
function goBack() {
  router.back()
}

// 返回首页
function goHome() {
  router.push('/')
}

// 前往登录页
function goLogin() {
  router.push('/login')
}

// 退出登录
async function logout() {
  try {
    await authLogout()
    ElMessage.success('已退出登录')
    router.push('/login')
  } catch (error) {
    console.error('退出登录失败:', error)
    ElMessage.error('退出登录失败')
  }
}

// 记录访问事件
onMounted(() => {
  // 可以在这里记录403访问事件到用户行为日志
  console.warn('403访问被拒绝:', {
    user: userInfo.value?.username,
    path: router.currentRoute.value.fullPath,
    time: currentTime.value
  })
})
</script>

<style scoped>
.forbidden-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.forbidden-content {
  max-width: 600px;
  width: 100%;
  text-align: center;
  background: white;
  border-radius: 12px;
  padding: 40px 30px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
  margin-bottom: 20px;
}

.forbidden-icon {
  margin-bottom: 30px;
  animation: shake 0.5s ease-in-out;
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-5px); }
  75% { transform: translateX(5px); }
}

.forbidden-title {
  font-size: 2.5rem;
  font-weight: bold;
  color: #2d3748;
  margin-bottom: 15px;
}

.forbidden-message {
  font-size: 1.1rem;
  color: #718096;
  margin-bottom: 30px;
  line-height: 1.6;
}

.user-info {
  margin: 30px 0;
  text-align: left;
}

.permission-guide {
  margin: 30px 0;
}

.forbidden-actions {
  margin: 30px 0;
}

.contact-admin {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid #e2e8f0;
}

.security-notice {
  max-width: 600px;
  width: 100%;
}

.security-tips {
  list-style: none;
  padding: 0;
  margin: 0;
}

.security-tips li {
  padding: 8px 0;
  color: #4a5568;
  font-size: 0.9rem;
}

@media (max-width: 768px) {
  .forbidden-content {
    padding: 30px 20px;
  }
  
  .forbidden-title {
    font-size: 2rem;
  }
  
  .el-button-group .el-button {
    margin-bottom: 10px;
  }
}
</style> 