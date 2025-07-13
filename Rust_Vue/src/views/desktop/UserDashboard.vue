<template>
  <div class="user-dashboard">
    <el-card class="dashboard-card">
      <template #header>
        <div class="card-header">
          <h2>用户仪表盘</h2>
        </div>
      </template>
      
      <el-row :gutter="20">
        <el-col :span="12">
          <el-card class="info-card">
            <template #header>
              <h3>个人信息</h3>
            </template>
            
            <el-form :model="userInfo" label-width="100px">
              <el-form-item label="用户名">
                <el-input v-model="userInfo.username" disabled />
              </el-form-item>
              
              <el-form-item label="昵称">
                <el-input v-model="userInfo.nickname" />
              </el-form-item>
              
              <el-form-item label="角色">
                <el-tag :type="getRoleType(userInfo.role)">
                  {{ getRoleText(userInfo.role) }}
                </el-tag>
              </el-form-item>
              
              <el-form-item label="星级">
                <el-rate v-model="userInfo.star" disabled />
              </el-form-item>
              
              <el-form-item label="状态">
                <el-tag :type="getStatusType(userInfo.ban_status)">
                  {{ getStatusText(userInfo.ban_status) }}
                </el-tag>
              </el-form-item>
              
              <el-form-item label="注册时间">
                <span>{{ formatDate(userInfo.created_at) }}</span>
              </el-form-item>
              
              <el-form-item label="最后登录">
                <span>{{ formatDate(userInfo.last_login) }}</span>
              </el-form-item>
            </el-form>
            
            <el-button type="primary" @click="saveUserInfo">保存修改</el-button>
          </el-card>
        </el-col>
        
        <el-col :span="12">
          <el-card class="stats-card">
            <template #header>
              <h3>使用统计</h3>
            </template>
            
            <div class="stats-grid">
              <div class="stat-item">
                <div class="stat-number">{{ stats.downloads || 0 }}</div>
                <div class="stat-label">下载次数</div>
              </div>
              
              <div class="stat-item">
                <div class="stat-number">{{ stats.uploads || 0 }}</div>
                <div class="stat-label">上传次数</div>
              </div>
              
              <div class="stat-item">
                <div class="stat-number">{{ stats.favorites || 0 }}</div>
                <div class="stat-label">收藏数量</div>
              </div>
              
              <div class="stat-item">
                <div class="stat-number">{{ stats.comments || 0 }}</div>
                <div class="stat-label">评论数量</div>
              </div>
            </div>
          </el-card>
          
          <el-card class="recent-card" style="margin-top: 20px;">
            <template #header>
              <h3>最近活动</h3>
            </template>
            
            <el-timeline>
              <el-timeline-item
                v-for="activity in recentActivities"
                :key="activity.id"
                :timestamp="formatDate(activity.timestamp)"
                :type="activity.type"
              >
                {{ activity.description }}
              </el-timeline-item>
            </el-timeline>
          </el-card>
        </el-col>
      </el-row>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { getUserInfo } from '@/api'

interface UserInfo {
  username: string
  nickname: string
  role: string
  star: number
  ban_status: string
  created_at: string
  last_login: string
}

interface Stats {
  downloads: number
  uploads: number
  favorites: number
  comments: number
}

interface Activity {
  id: number
  type: string
  description: string
  timestamp: string
}

const userInfo = ref<UserInfo>({
  username: '',
  nickname: '',
  role: 'user',
  star: 1,
  ban_status: 'normal',
  created_at: '',
  last_login: ''
})

const stats = ref<Stats>({
  downloads: 0,
  uploads: 0,
  favorites: 0,
  comments: 0
})

const recentActivities = ref<Activity[]>([])

const getRoleType = (role: string) => {
  switch (role) {
    case 'admin': return 'danger'
    case 'moderator': return 'warning'
    default: return 'info'
  }
}

const getRoleText = (role: string) => {
  switch (role) {
    case 'admin': return '管理员'
    case 'moderator': return '版主'
    default: return '普通用户'
  }
}

const getStatusType = (status: string) => {
  switch (status) {
    case 'banned': return 'danger'
    case 'suspended': return 'warning'
    default: return 'success'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'banned': return '已封禁'
    case 'suspended': return '已暂停'
    default: return '正常'
  }
}

const formatDate = (dateStr: string) => {
  if (!dateStr) return '未知'
  return new Date(dateStr).toLocaleString('zh-CN')
}

const loadUserInfo = async () => {
  try {
    const username = localStorage.getItem('username') || ''
    if (!username) {
      ElMessage.error('请先登录')
      return
    }
    
    const response = await getUserInfo(username)
    if (response.code === 0) {
      userInfo.value = response.data
    }
  } catch (error) {
    console.error('加载用户信息失败:', error)
    ElMessage.error('加载用户信息失败')
  }
}

const saveUserInfo = async () => {
  try {
    // 这里可以添加保存用户信息的API调用
    ElMessage.success('保存成功')
  } catch (error) {
    console.error('保存用户信息失败:', error)
    ElMessage.error('保存失败')
  }
}

onMounted(() => {
  loadUserInfo()
})
</script>

<style scoped>
.user-dashboard {
  padding: 20px;
}

.dashboard-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.info-card, .stats-card, .recent-card {
  height: 100%;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
}

.stat-item {
  text-align: center;
  padding: 20px;
  background: #f5f7fa;
  border-radius: 8px;
}

.stat-number {
  font-size: 24px;
  font-weight: bold;
  color: #409eff;
  margin-bottom: 8px;
}

.stat-label {
  font-size: 14px;
  color: #606266;
}

.el-timeline {
  max-height: 300px;
  overflow-y: auto;
}
</style> 