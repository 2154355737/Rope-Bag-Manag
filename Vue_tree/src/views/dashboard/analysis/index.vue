<template>
  <PageWrapper title="数据分析" content="绳包管理器系统数据分析报告">
    <div class="p-4">
      <!-- 数据分析快捷导航 - 使用Card.Grid布局 -->
      <Card title="数据分析导航" class="!mb-4 enter-y">
        <CardGrid class="!w-1/6 text-center cursor-pointer hover:shadow-md transition-shadow" @click="navigateToUserStats">
          <div class="p-2">
            <Icon icon="ion:person-outline" size="24" class="text-primary mb-2" />
            <div class="text-sm">用户统计</div>
          </div>
        </CardGrid>
        <CardGrid class="!w-1/6 text-center cursor-pointer hover:shadow-md transition-shadow" @click="navigateToPackageStats">
          <div class="p-2">
            <Icon icon="ion:archive-outline" size="24" class="text-success mb-2" />
            <div class="text-sm">资源统计</div>
          </div>
        </CardGrid>
        <CardGrid class="!w-1/6 text-center cursor-pointer hover:shadow-md transition-shadow" @click="navigateToBackupManage">
          <div class="p-2">
            <Icon icon="ion:cloud-download-outline" size="24" class="text-warning mb-2" />
            <div class="text-sm">备份管理</div>
          </div>
        </CardGrid>
        <CardGrid class="!w-1/6 text-center cursor-pointer hover:shadow-md transition-shadow" @click="navigateToSystemLogs">
          <div class="p-2">
            <Icon icon="ion:document-text-outline" size="24" class="text-error mb-2" />
            <div class="text-sm">系统日志</div>
          </div>
        </CardGrid>
        <CardGrid class="!w-1/6 text-center cursor-pointer hover:shadow-md transition-shadow" @click="navigateToUserActions">
          <div class="p-2">
            <Icon icon="ion:analytics-outline" size="24" class="text-purple-500 mb-2" />
            <div class="text-sm">用户行为</div>
          </div>
        </CardGrid>
        <CardGrid class="!w-1/6 text-center cursor-pointer hover:shadow-md transition-shadow" @click="navigateToSystemSettings">
          <div class="p-2">
            <Icon icon="ion:cog-outline" size="24" class="text-gray-500 mb-2" />
            <div class="text-sm">系统设置</div>
          </div>
        </CardGrid>
      </Card>

      <!-- 使用现有的GrowCard组件显示统计数据 -->
      <GrowCard :loading="loading" class="enter-y" />
      
      <!-- 使用现有的SiteAnalysis组件 -->
      <SiteAnalysis class="!my-4 enter-y" :loading="loading" />
      
      <!-- 使用现有组件的布局模式 -->
      <div class="md:flex enter-y">
        <!-- 系统概览 - 使用Card组件 -->
        <Card title="系统概览" :loading="loading" class="md:w-1/3 w-full">
          <template #extra>
            <Tag :color="getStatusColor(stats?.system_status)">
              {{ getStatusText(stats?.system_status) }}
            </Tag>
          </template>
          
          <div class="space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-secondary">总用户数</span>
              <span class="text-lg font-bold">{{ stats?.total_users || 0 }}</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-secondary">资源包数</span>
              <span class="text-lg font-bold">{{ stats?.total_packages || 0 }}</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-secondary">评论总数</span>
              <span class="text-lg font-bold">{{ stats?.total_comments || 0 }}</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-secondary">活跃用户</span>
              <span class="text-lg font-bold">{{ stats?.active_users || 0 }}</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-secondary">运行时间</span>
              <span class="font-medium">{{ formatUptime(stats?.uptime) }}</span>
            </div>
          </div>
        </Card>

        <!-- 分类统计 - 使用现有的VisitSource组件样式 -->
        <Card title="分类统计" :loading="loading" class="md:w-1/3 !md:mx-4 !md:my-0 !my-4 w-full">
          <div class="space-y-3">
            <div v-for="category in categories.slice(0, 5)" :key="category.id" 
                 class="flex justify-between items-center">
              <div class="flex items-center">
                <Icon icon="ant-design:folder-outlined" class="mr-2" />
                <span>{{ category.name }}</span>
              </div>
              <div class="flex items-center space-x-2">
                <Tag v-if="category.enabled" color="success" size="small">启用</Tag>
                <Tag v-else color="error" size="small">禁用</Tag>
              </div>
            </div>
          </div>
        </Card>

        <!-- 今日数据 - 使用Card组件 -->
        <Card title="今日数据" :loading="loading" class="md:w-1/3 w-full">
          <div class="space-y-4">
            <div class="text-center">
              <div class="text-2xl font-bold text-primary">{{ stats?.new_users_today || 0 }}</div>
              <div class="text-sm text-secondary">新增用户</div>
            </div>
            <div class="text-center">
              <div class="text-2xl font-bold text-success">{{ stats?.new_packages_today || 0 }}</div>
              <div class="text-sm text-secondary">新增资源</div>
            </div>
            <div class="text-center">
              <div class="text-xs text-secondary">
                系统版本: v1.0.0
              </div>
            </div>
          </div>
        </Card>
      </div>

      <!-- 用户统计表格 - 使用标准Card + Table -->
      <Card title="用户活跃度统计" :loading="loading" class="!my-4 enter-y">
        <Table
          :columns="userStatsColumns"
          :data-source="userStatsData"
          :pagination="{ pageSize: 10 }"
          size="middle"
        />
      </Card>
    </div>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, h } from 'vue'
  import { Card, Tag, Table } from 'ant-design-vue'
  import { useRouter } from 'vue-router'
  import { PageWrapper } from '/@/components/Page'
  import { Icon } from '/@/components/Icon'
  import { getStats, getUserStats, getCategories, type Stats, type UserStats } from '/@/api/dashboard'
  
  // 导入现有组件
  import GrowCard from './components/GrowCard.vue'
  import SiteAnalysis from './components/SiteAnalysis.vue'

  const CardGrid = Card.Grid
  const router = useRouter()
  const loading = ref(true)
  const stats = ref<Stats>()
  const userStatsData = ref<UserStats[]>([])
  const categories = ref<any[]>([])

  // 用户统计表格配置
  const userStatsColumns = [
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: '角色',
      dataIndex: 'role',
      key: 'role',
      customRender: ({ text }: any) => {
        const colorMap: any = {
          'admin': 'red',
          'user': 'blue',
          'moderator': 'orange'
        }
        return h(Tag, { color: colorMap[text] || 'default' }, () => text || '用户')
      }
    },
    {
      title: '资源包数量',
      dataIndex: 'package_count',
      key: 'package_count',
      sorter: (a: any, b: any) => a.package_count - b.package_count,
    },
    {
      title: '评论数量',
      dataIndex: 'comment_count',
      key: 'comment_count',
      sorter: (a: any, b: any) => a.comment_count - b.comment_count,
    },
    {
      title: '最后活跃',
      dataIndex: 'last_active',
      key: 'last_active',
      customRender: ({ text }: any) => {
        return text ? new Date(text).toLocaleDateString() : '未知'
      }
    },
  ]

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
      return `${days}天 ${hours}小时`
    } else if (hours > 0) {
      return `${hours}小时 ${minutes}分钟`
    } else {
      return `${minutes}分钟`
    }
  }

  // 获取数据
  const fetchData = async () => {
    try {
      // 并行获取所有数据
      const [statsRes, userStatsRes, categoriesRes] = await Promise.all([
        getStats().catch(() => null),
        getUserStats({ page: 1, size: 10 }).catch(() => ({ list: [] })),
        getCategories().catch(() => [])
      ])

      if (statsRes) stats.value = statsRes
      if (userStatsRes) userStatsData.value = userStatsRes.list || []
      if (categoriesRes) categories.value = categoriesRes

    } catch (error) {
      console.error('获取分析数据失败:', error)
    } finally {
      loading.value = false
    }
  }

  // 导航函数
  const navigateToUserStats = () => {
    router.push('/system/user')
  }

  const navigateToPackageStats = () => {
    router.push('/system/package')
  }

  const navigateToBackupManage = () => {
    router.push('/system/backup')
  }

  const navigateToSystemLogs = () => {
    router.push('/system/logs')
  }

  const navigateToUserActions = () => {
    // 如果有用户行为分析页面的话
    router.push('/system/user-actions')
  }

  const navigateToSystemSettings = () => {
    router.push('/system/settings')
  }

  onMounted(() => {
    fetchData()
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
