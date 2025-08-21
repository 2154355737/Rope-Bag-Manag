import { ref, computed } from 'vue'
import { isLoggedIn as checkIsLoggedIn, getUserInfo, clearLoginStatus } from '@/utils/auth'

// 认证状态管理
export function useAuthStore() {
  const user = ref(getUserInfo())
  const isAuthenticated = ref(checkIsLoggedIn())

  // 计算属性
  const isLoggedIn = computed(() => isAuthenticated.value)
  const currentUser = computed(() => user.value)

  // 更新用户信息
  const updateUser = (userInfo: any) => {
    user.value = userInfo
    isAuthenticated.value = !!userInfo
  }

  // 清除认证状态
  const logout = () => {
    clearLoginStatus()
    user.value = null
    isAuthenticated.value = false
  }

  // 检查权限
  const hasRole = (role: string) => {
    return user.value?.role === role
  }

  const isAdmin = computed(() => hasRole('admin'))
  const isElder = computed(() => hasRole('elder'))
  const isModerator = computed(() => hasRole('moderator'))

  return {
    user,
    isAuthenticated,
    isLoggedIn,
    currentUser,
    updateUser,
    logout,
    hasRole,
    isAdmin,
    isElder,
    isModerator
  }
} 