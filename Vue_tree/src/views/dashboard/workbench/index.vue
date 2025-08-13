<template>
  <PageWrapper>
    <!-- 使用现有的WorkbenchHeader组件 -->
    <template #headerContent>
      <WorkbenchHeader />
    </template>
    
    <div class="lg:flex">
      <div class="lg:w-7/10 w-full !mr-4 enter-y">
        <!-- 使用现有的ProjectCard组件 -->
        <ProjectCard :loading="loading" class="enter-y" />
        
        <!-- 系统状态展示 - 使用Card组件 -->
        <Card title="系统状态" :loading="loading" class="!my-4 enter-y">
          <template #extra>
            <Tag :color="getStatusColor(stats?.system_status)">
              {{ getStatusText(stats?.system_status) }}
            </Tag>
          </template>
          
          <div class="md:flex">
            <div class="md:w-1/4 w-full p-4 text-center">
              <div class="text-2xl font-bold text-primary">{{ stats?.total_users || 0 }}</div>
              <div class="text-sm text-secondary">总用户数</div>
              <div class="text-xs text-success" v-if="stats?.new_users_today">
                +{{ stats.new_users_today }} 今日新增
              </div>
            </div>
            
            <div class="md:w-1/4 w-full p-4 text-center">
              <div class="text-2xl font-bold text-success">{{ stats?.total_packages || 0 }}</div>
              <div class="text-sm text-secondary">资源包总数</div>
              <div class="text-xs text-success" v-if="stats?.new_packages_today">
                +{{ stats.new_packages_today }} 今日新增
              </div>
            </div>
            
            <div class="md:w-1/4 w-full p-4 text-center">
              <div class="text-2xl font-bold text-warning">{{ stats?.total_comments || 0 }}</div>
              <div class="text-sm text-secondary">评论总数</div>
            </div>
            
            <div class="md:w-1/4 w-full p-4 text-center">
              <div class="text-2xl font-bold text-error">{{ stats?.active_users || 0 }}</div>
              <div class="text-sm text-secondary">活跃用户</div>
            </div>
          </div>
          
          <Divider />
          
          <div class="flex justify-between items-center">
            <div>
              <span class="text-secondary">系统运行时间：</span>
              <span class="font-medium">{{ formatUptime(stats?.uptime) }}</span>
            </div>
            <Button type="primary" @click="refreshStats" :loading="refreshing">
              刷新数据
            </Button>
          </div>
        </Card>
        
        <!-- 使用现有的DynamicInfo组件 -->
        <DynamicInfo :loading="loading" class="!my-4 enter-y" />
      </div>
      
      <div class="lg:w-3/10 w-full enter-y">
        <!-- 使用现有的QuickNav组件 -->
        <QuickNav :loading="loading" class="enter-y" />

        <!-- 快捷操作 - 使用Card组件 -->
        <Card title="快捷操作" class="!my-4 enter-y">
          <div class="grid grid-cols-2 gap-2">
            <Button type="primary" size="small" block @click="handleCreateBackup" :loading="backupLoading">
              <Icon icon="ion:cloud-download-outline" class="mr-1" />
              创建备份
            </Button>
            <Button size="small" block @click="router.push('/system/user')">
              <Icon icon="ion:person-outline" class="mr-1" />
              用户管理
            </Button>
            <Button size="small" block @click="router.push('/system/package')">
              <Icon icon="ion:archive-outline" class="mr-1" />
              资源管理
            </Button>
            <Button size="small" block @click="router.push('/system/category')">
              <Icon icon="ion:folder-outline" class="mr-1" />
              分类管理
            </Button>
            <Button size="small" block @click="router.push('/system/logs')">
              <Icon icon="ion:document-text-outline" class="mr-1" />
              系统日志
            </Button>
            <Button size="small" block @click="router.push('/system/settings')">
              <Icon icon="ion:cog-outline" class="mr-1" />
              系统设置
            </Button>
          </div>
        </Card>

        <!-- 使用现有的插图和SaleRadar组件 -->
        <Card class="!my-4 enter-y" :loading="loading">
          <img class="xl:h-50 h-30 mx-auto" src="../../../assets/svg/illustration.svg" />
        </Card>

        <SaleRadar :loading="loading" class="enter-y" />
      </div>
    </div>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue'
  import { Card, Tag, Button, Divider } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { useRouter } from 'vue-router'
  import { useMessage } from '/@/hooks/web/useMessage'
  import { Icon } from '/@/components/Icon'
  import { getStats, createBackup, type Stats } from '/@/api/dashboard'
  
  // 导入现有组件
  import WorkbenchHeader from './components/WorkbenchHeader.vue'
  import ProjectCard from './components/ProjectCard.vue'
  import QuickNav from './components/QuickNav.vue'
  import DynamicInfo from './components/DynamicInfo.vue'
  import SaleRadar from './components/SaleRadar.vue'

  const router = useRouter()
  const { createMessage } = useMessage()
  const loading = ref(true)
  const refreshing = ref(false)
  const backupLoading = ref(false)
  const stats = ref<Stats>()

  // 模拟加载状态，与原始模板一致
  setTimeout(() => {
    loading.value = false
  }, 1500)

  const getStatusColor = (status?: string) => {
    switch (status) {
      case 'healthy': return 'green'
      case 'warning': return 'orange'
      case 'error': return 'red'
      default: return 'gray'
    }
  }

  const getStatusText = (status?: string) => {
    switch (status) {
      case 'healthy': return '运行正常'
      case 'warning': return '警告'
      case 'error': return '错误'
      default: return '未知'
    }
  }

  const formatUptime = (seconds?: number) => {
    if (!seconds) return '未知'
    
    const days = Math.floor(seconds / 86400)
    const hours = Math.floor((seconds % 86400) / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    
    if (days > 0) {
      return `${days}天 ${hours}小时 ${minutes}分钟`
    } else if (hours > 0) {
      return `${hours}小时 ${minutes}分钟`
    } else {
      return `${minutes}分钟`
    }
  }

  const fetchStats = async () => {
    try {
      stats.value = await getStats()
    } catch (error) {
      console.error('获取统计数据失败:', error)
    }
  }

  const refreshStats = async () => {
    refreshing.value = true
    try {
      stats.value = await getStats()
      createMessage.success('数据刷新成功')
    } catch (error) {
      console.error('刷新统计数据失败:', error)
      createMessage.error('数据刷新失败')
    } finally {
      refreshing.value = false
    }
  }

  const handleCreateBackup = async () => {
    backupLoading.value = true
    try {
      await createBackup({
        backup_type: 'Manual',
        description: '手动创建的备份'
      })
      createMessage.success('备份创建成功')
    } catch (error) {
      console.error('创建备份失败:', error)
      createMessage.error('备份创建失败')
    } finally {
      backupLoading.value = false
    }
  }

  onMounted(() => {
    fetchStats()
  })
</script>

<style scoped>
.enter-y {
  animation: slideInUp 0.3s ease-out forwards;
}

@keyframes slideInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
