<template>
  <div class="download-security">
    <el-card class="security-card">
      <template #header>
        <div class="card-header">
          <span>下载安全监控</span>
          <el-button type="primary" @click="refreshData" :loading="loading">
            {{ loading ? '加载中...' : '刷新数据' }}
          </el-button>
        </div>
      </template>

      <!-- 统计概览 -->
      <el-row :gutter="20" class="stats-overview">
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-item">
              <div class="stat-number">{{ stats.total_anomalies || 0 }}</div>
              <div class="stat-label">总异常数</div>
              <div class="stat-desc">24小时内检测到的异常</div>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-item">
              <div class="stat-number">{{ stats.by_severity?.critical || 0 }}</div>
              <div class="stat-label">严重异常</div>
              <div class="stat-desc">需要立即处理</div>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-item">
              <div class="stat-number">{{ stats.by_severity?.high || 0 }}</div>
              <div class="stat-label">高危异常</div>
              <div class="stat-desc">需要关注处理</div>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-item">
              <div class="stat-number">{{ stats.by_severity?.medium || 0 }}</div>
              <div class="stat-label">中等异常</div>
              <div class="stat-desc">建议监控</div>
            </div>
          </el-card>
        </el-col>
      </el-row>

      <!-- 数据状态提示 -->
      <el-alert
        v-if="loading"
        title="正在加载数据"
        description="正在从服务器获取防刷量系统的最新数据..."
        type="info"
        :closable="false"
        show-icon
        style="margin-bottom: 20px;"
      />
      <el-alert
        v-else-if="!stats.total_anomalies && stats.total_anomalies !== 0"
        title="暂无数据"
        description="防刷量系统正在运行，但尚未检测到异常。这是正常现象，说明系统运行良好。"
        type="info"
        :closable="false"
        show-icon
        style="margin-bottom: 20px;"
      />
      <el-alert
        v-else-if="stats.total_anomalies === 0"
        title="系统运行正常"
        description="24小时内未检测到任何异常，防刷量系统运行良好。"
        type="success"
        :closable="false"
        show-icon
        style="margin-bottom: 20px;"
      />
      <el-alert
        v-else-if="stats.total_anomalies > 0"
        :title="`检测到 ${stats.total_anomalies} 个异常`"
        description="系统已检测到异常行为，建议及时查看和处理。"
        type="warning"
        :closable="false"
        show-icon
        style="margin-bottom: 20px;"
      />

      <!-- 异常类型统计 -->
      <el-card class="anomaly-types">
        <template #header>
          <span>异常类型分布</span>
        </template>
        <div v-if="!stats.by_type || Object.keys(stats.by_type).length === 0" class="no-data">
          <el-empty description="暂无异常数据" />
        </div>
        <div v-else class="anomaly-chart">
          <div v-for="(count, type) in stats.by_type" :key="type" class="anomaly-type-item">
            <div class="type-name">{{ getAnomalyTypeName(type) }}</div>
            <div class="type-count">{{ count }}</div>
            <el-progress 
              :percentage="getPercentage(count)" 
              :color="getTypeColor(type)"
              :stroke-width="8"
            />
          </div>
        </div>
      </el-card>

      <!-- 安全配置 -->
      <el-card class="security-config">
        <template #header>
          <span>安全配置</span>
        </template>
        <el-form :model="config" label-width="200px">
          <el-form-item label="启用频率限制">
            <el-switch v-model="config.enable_rate_limiting" />
          </el-form-item>
          <el-form-item label="启用异常检测">
            <el-switch v-model="config.enable_anomaly_detection" />
          </el-form-item>
          <el-form-item label="启用统计分析">
            <el-switch v-model="config.enable_statistical_analysis" />
          </el-form-item>
          <el-form-item label="用户每小时最大下载次数">
            <el-input-number v-model="config.max_downloads_per_user_per_hour" :min="1" :max="100" />
          </el-form-item>
          <el-form-item label="IP每小时最大下载次数">
            <el-input-number v-model="config.max_downloads_per_ip_per_hour" :min="1" :max="200" />
          </el-form-item>
          <el-form-item label="资源每天最大下载次数">
            <el-input-number v-model="config.max_downloads_per_resource_per_day" :min="1" :max="1000" />
          </el-form-item>
          <el-form-item label="可疑模式阈值">
            <el-slider 
              v-model="config.suspicious_pattern_threshold" 
              :min="0.1" 
              :max="1.0" 
              :step="0.1"
              :format-tooltip="(val: number) => `${(val * 100).toFixed(0)}%`"
            />
          </el-form-item>
          <el-form-item label="统计异常阈值">
            <el-slider 
              v-model="config.statistical_anomaly_threshold" 
              :min="1.0" 
              :max="5.0" 
              :step="0.1"
            />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="saveConfig">保存配置</el-button>
            <el-button @click="resetConfig">重置配置</el-button>
          </el-form-item>
        </el-form>
      </el-card>

      <!-- 实时监控 -->
      <el-card class="real-time-monitor">
        <template #header>
          <span>实时监控</span>
        </template>
        <div class="monitor-content">
          <el-alert
            v-if="recentAnomalies.length > 0"
            :title="`最近发现 ${recentAnomalies.length} 个异常`"
            type="warning"
            :closable="false"
            show-icon
          />
          <div v-else class="no-anomalies">
            <el-icon><Check /></el-icon>
            <span>暂无异常检测</span>
            <div class="monitor-desc">
              系统正在实时监控下载行为，包括频率限制、可疑模式检测和统计异常分析
            </div>
          </div>
        </div>
      </el-card>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Check } from '@element-plus/icons-vue'
import { adminApi } from '@/api/admin'

// 响应式数据
const stats = ref<any>({})
const config = ref({
  enable_rate_limiting: true,
  enable_anomaly_detection: true,
  enable_statistical_analysis: true,
  max_downloads_per_user_per_hour: 10,
  max_downloads_per_ip_per_hour: 20,
  max_downloads_per_resource_per_day: 50,
  suspicious_pattern_threshold: 0.8,
  statistical_anomaly_threshold: 2.0
})
const recentAnomalies = ref<any[]>([])
const loading = ref(false)

// 获取统计数据
const fetchStats = async () => {
  try {
    loading.value = true
    const response = await adminApi.getDownloadSecurityStats()
    if (response.code === 0) {
      stats.value = response.data
    } else {
      ElMessage.warning('获取安全统计失败: ' + response.message)
    }
  } catch (error: any) {
    console.error('获取安全统计失败:', error)
    if (error.response?.status === 403) {
      ElMessage.warning('权限不足，请确保您有管理员权限')
    } else if (error.response?.status === 404) {
      ElMessage.warning('API接口不可用，请检查后端服务是否正常运行')
    } else {
      ElMessage.error('获取安全统计失败: ' + (error.message || '网络错误'))
    }
  } finally {
    loading.value = false
  }
}

// 获取配置
const fetchConfig = async () => {
  try {
    const response = await adminApi.getDownloadSecurityConfig()
    if (response.code === 0) {
      config.value = response.data
    } else {
      ElMessage.warning('获取安全配置失败: ' + response.message)
    }
  } catch (error: any) {
    console.error('获取安全配置失败:', error)
    if (error.response?.status === 403) {
      ElMessage.warning('权限不足，请确保您有管理员权限')
    } else if (error.response?.status === 404) {
      ElMessage.warning('API接口不可用，请检查后端服务是否正常运行')
    } else {
      ElMessage.error('获取安全配置失败: ' + (error.message || '网络错误'))
    }
  }
}

// 保存配置
const saveConfig = async () => {
  try {
    const response = await adminApi.updateDownloadSecurityConfig(config.value)
    if (response.code === 0) {
      ElMessage.success('配置保存成功')
    }
  } catch (error) {
    console.error('保存配置失败:', error)
    ElMessage.error('保存配置失败')
  }
}

// 重置配置
const resetConfig = () => {
  config.value = {
    enable_rate_limiting: true,
    enable_anomaly_detection: true,
    enable_statistical_analysis: true,
    max_downloads_per_user_per_hour: 10,
    max_downloads_per_ip_per_hour: 20,
    max_downloads_per_resource_per_day: 50,
    suspicious_pattern_threshold: 0.8,
    statistical_anomaly_threshold: 2.0
  }
  ElMessage.info('配置已重置')
}

// 刷新数据
const refreshData = () => {
  fetchStats()
  fetchConfig()
}

// 获取异常类型名称
const getAnomalyTypeName = (type: string | number) => {
  const typeStr = String(type)
  const typeMap: { [key: string]: string } = {
    'rate_limit_exceeded': '频率限制超限',
    'suspicious_pattern': '可疑模式',
    'statistical_anomaly': '统计异常'
  }
  return typeMap[typeStr] || typeStr
}

// 获取百分比
const getPercentage = (count: number) => {
  const total = stats.value.total_anomalies || 1
  return Math.round((count / total) * 100)
}

// 获取类型颜色
const getTypeColor = (type: string | number) => {
  const typeStr = String(type)
  const colorMap: { [key: string]: string } = {
    'rate_limit_exceeded': '#E6A23C',
    'suspicious_pattern': '#F56C6C',
    'statistical_anomaly': '#909399'
  }
  return colorMap[typeStr] || '#409EFF'
}

// 组件挂载时获取数据
onMounted(() => {
  fetchStats()
  fetchConfig()
})
</script>

<style scoped>
.download-security {
  padding: 20px;
}

.security-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.stats-overview {
  margin-bottom: 20px;
}

.stat-card {
  text-align: center;
}

.stat-item {
  padding: 10px;
}

.stat-number {
  font-size: 24px;
  font-weight: bold;
  color: #409EFF;
  margin-bottom: 5px;
}

.stat-label {
  font-size: 14px;
  color: #666;
  margin-bottom: 4px;
}

.stat-desc {
  font-size: 12px;
  color: #999;
  line-height: 1.2;
}

.anomaly-types {
  margin-bottom: 20px;
}

.anomaly-chart {
  padding: 20px 0;
}

.no-data {
  padding: 40px 0;
  text-align: center;
}

.anomaly-type-item {
  display: flex;
  align-items: center;
  margin-bottom: 15px;
  padding: 10px;
  background: #f8f9fa;
  border-radius: 8px;
}

.type-name {
  flex: 1;
  font-weight: 500;
  color: #333;
}

.type-count {
  flex: 0 0 60px;
  text-align: center;
  font-weight: bold;
  color: #409EFF;
}

.security-config {
  margin-bottom: 20px;
}

.real-time-monitor {
  margin-bottom: 20px;
}

.monitor-content {
  padding: 20px 0;
}

.no-anomalies {
  display: flex;
  align-items: center;
  justify-content: center;
  color: #67C23A;
  font-size: 16px;
  padding: 40px 0;
}

.no-anomalies .el-icon {
  margin-right: 8px;
  font-size: 20px;
}

.monitor-desc {
  margin-top: 8px;
  font-size: 12px;
  color: #999;
  line-height: 1.4;
}
</style> 