<template>
  <div v-if="hasPermission">
    <slot />
  </div>
  <div v-else-if="showFallback" class="permission-denied">
    <slot name="fallback">
      <el-alert
        :title="fallbackTitle"
        :description="fallbackMessage"
        type="warning"
        :closable="false"
        show-icon
      />
    </slot>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { getUserInfo } from '../utils/auth'

interface Props {
  /** 需要的角色列表 */
  roles?: string[]
  /** 是否需要管理员权限 */
  requireAdmin?: boolean
  /** 是否显示无权限时的提示 */
  showFallback?: boolean
  /** 自定义无权限提示标题 */
  fallbackTitle?: string
  /** 自定义无权限提示内容 */
  fallbackMessage?: string
  /** 是否需要资源所有权 */
  requireOwnership?: boolean
  /** 资源所有者ID (当需要所有权时) */
  resourceOwnerId?: number
}

const props = withDefaults(defineProps<Props>(), {
  showFallback: true,
  fallbackTitle: '权限不足',
  fallbackMessage: '您没有权限访问此内容'
})

const hasPermission = computed(() => {
  const userInfo = getUserInfo()
  
  // 未登录用户无权限
  if (!userInfo) {
    return false
  }
  
  // 检查封禁状态（不区分大小写）
  const banStatus = userInfo.ban_status?.toLowerCase()
  if (banStatus && banStatus !== 'normal') {
    return false
  }
  
  // 检查管理员权限
  if (props.requireAdmin && userInfo.role !== 'admin') {
    return false
  }
  
  // 检查角色权限
  if (props.roles && props.roles.length > 0) {
    if (!props.roles.includes(userInfo.role)) {
      return false
    }
  }
  
  // 检查资源所有权
  if (props.requireOwnership && props.resourceOwnerId) {
    if (userInfo.role !== 'admin' && userInfo.id !== props.resourceOwnerId) {
      return false
    }
  }
  
  return true
})

// 暴露权限状态供父组件使用
defineExpose({
  hasPermission
})
</script>

<style scoped>
.permission-denied {
  padding: 16px;
}
</style> 