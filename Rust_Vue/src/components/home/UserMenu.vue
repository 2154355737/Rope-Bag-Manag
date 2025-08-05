<template>
  <el-dropdown trigger="click" placement="bottom-end">
    <div class="user-trigger">
      <el-avatar :size="36" :src="userInfo?.avatar_url">
        <el-icon><User /></el-icon>
      </el-avatar>
      <span class="user-name">{{ userInfo?.nickname || userInfo?.username }}</span>
      <el-icon class="dropdown-arrow"><ArrowDown /></el-icon>
    </div>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item @click="goToUserProfile">
          <el-icon><User /></el-icon>
          个人中心
        </el-dropdown-item>
        <el-dropdown-item @click="goToAdmin" v-if="canAccessAdmin">
          <el-icon><Setting /></el-icon>
          管理后台
        </el-dropdown-item>
        <el-dropdown-item divided @click="handleLogout">
          <el-icon><Switch /></el-icon>
          退出登录
        </el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessageBox, ElMessage } from 'element-plus'
import { User, ArrowDown, Setting, Switch } from '@element-plus/icons-vue'

// Props
interface Props {
  userInfo: any
}

const props = defineProps<Props>()

// Router
const router = useRouter()

// 计算属性
const canAccessAdmin = computed(() => {
  const role = props.userInfo?.role
  return ['admin', 'moderator', 'elder'].includes(role)
})

// 方法
const goToUserProfile = () => {
  const role = props.userInfo?.role
  if (role === 'elder') {
    router.push('/elder/profile')
  } else if (role === 'user') {
    router.push('/user/profile')
  } else if (role === 'admin') {
    router.push('/admin/users')
  } else {
    router.push('/login')
  }
}

const goToAdmin = () => {
  const role = props.userInfo?.role
  if (role === 'admin' || role === 'moderator') {
    router.push('/admin')
  } else if (role === 'elder') {
    router.push('/elder')
  } else {
    router.push('/403')
  }
}

const handleLogout = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要退出登录吗？',
      '确认退出',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 清除用户信息
    localStorage.removeItem('token')
    localStorage.removeItem('userInfo')
    localStorage.removeItem('isLoggedIn')
    localStorage.removeItem('loginTime')
    localStorage.removeItem('remember_me')
    localStorage.removeItem('username')
    
    // 清除Cookie
    document.cookie = 'auth_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
    document.cookie = 'user_info=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
    document.cookie = 'remember_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
    
    ElMessage.success('已退出登录')
    router.push('/login')
  } catch (error) {
    // 用户取消
  }
}
</script>

<style scoped>
.user-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color-light);
}

.user-trigger:hover {
  background: var(--bg-hover);
  border-color: var(--border-color-hover);
}

.user-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dropdown-arrow {
  font-size: 12px;
  color: var(--text-tertiary);
  transition: transform 0.2s ease;
}

.user-trigger:hover .dropdown-arrow {
  transform: rotate(180deg);
}

:deep(.el-dropdown-menu__item) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
}

:deep(.el-dropdown-menu__item .el-icon) {
  font-size: 16px;
}
</style> 