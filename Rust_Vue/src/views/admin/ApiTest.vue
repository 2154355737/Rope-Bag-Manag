<template>
  <div class="api-test-page">
    <el-card class="test-card">
      <template #header>
        <div class="card-header">
          <h2>API 测试面板</h2>
          <el-button type="primary" @click="testConnection">测试连接</el-button>
        </div>
      </template>

      <!-- 连接状态 -->
      <div class="status-section">
        <el-alert
          :title="connectionStatus.title"
          :type="connectionStatus.type"
          :description="connectionStatus.description"
          show-icon
        />
      </div>

      <!-- API测试区域 -->
      <div class="test-section">
        <h3>API 功能测试</h3>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-card class="test-item">
              <template #header>
                <span>认证测试</span>
              </template>
              <div class="test-content">
                <el-button @click="testAuth" :loading="authLoading">
                  测试登录
                </el-button>
                <div class="test-result" v-if="authResult">
                  <pre>{{ JSON.stringify(authResult, null, 2) }}</pre>
                </div>
              </div>
            </el-card>
          </el-col>

          <el-col :span="8">
            <el-card class="test-item">
              <template #header>
                <span>用户管理</span>
              </template>
              <div class="test-content">
                <el-button @click="testUsers" :loading="usersLoading">
                  获取用户列表
                </el-button>
                <div class="test-result" v-if="usersResult">
                  <pre>{{ JSON.stringify(usersResult, null, 2) }}</pre>
                </div>
              </div>
            </el-card>
          </el-col>

          <el-col :span="8">
            <el-card class="test-item">
              <template #header>
                <span>统计信息</span>
              </template>
              <div class="test-content">
                <el-button @click="testStats" :loading="statsLoading">
                  获取统计数据
                </el-button>
                <div class="test-result" v-if="statsResult">
                  <pre>{{ JSON.stringify(statsResult, null, 2) }}</pre>
                </div>
              </div>
            </el-card>
          </el-col>
        </el-row>

        <el-row :gutter="20" style="margin-top: 20px;">
          <el-col :span="8">
            <el-card class="test-item">
              <template #header>
                <span>绳包管理</span>
              </template>
              <div class="test-content">
                <el-button @click="testPackages" :loading="packagesLoading">
                  获取绳包列表
                </el-button>
                <div class="test-result" v-if="packagesResult">
                  <pre>{{ JSON.stringify(packagesResult, null, 2) }}</pre>
                </div>
              </div>
            </el-card>
          </el-col>

          <el-col :span="8">
            <el-card class="test-item">
              <template #header>
                <span>系统日志</span>
              </template>
              <div class="test-content">
                <el-button @click="testLogs" :loading="logsLoading">
                  获取系统日志
                </el-button>
                <div class="test-result" v-if="logsResult">
                  <pre>{{ JSON.stringify(logsResult, null, 2) }}</pre>
                </div>
              </div>
            </el-card>
          </el-col>

          <el-col :span="8">
            <el-card class="test-item">
              <template #header>
                <span>用户行为</span>
              </template>
              <div class="test-content">
                <el-button @click="testUserActions" :loading="actionsLoading">
                  获取用户行为
                </el-button>
                <div class="test-result" v-if="actionsResult">
                  <pre>{{ JSON.stringify(actionsResult, null, 2) }}</pre>
                </div>
              </div>
            </el-card>
          </el-col>
        </el-row>
      </div>

      <!-- 批量测试 -->
      <div class="batch-test">
        <h3>批量测试</h3>
        <el-button type="success" @click="runAllTests" :loading="batchLoading">
          运行所有测试
        </el-button>
        <div class="batch-result" v-if="batchResults.length > 0">
          <h4>测试结果:</h4>
          <div v-for="(result, index) in batchResults" :key="index" class="batch-item">
            <el-tag :type="result.success ? 'success' : 'danger'">
              {{ result.name }}
            </el-tag>
            <span class="batch-message">{{ result.message }}</span>
          </div>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { healthCheck, authApi, userApi, adminApi, packageApi } from '../../api'

// 状态管理
const connectionStatus = reactive({
  title: '未连接',
  type: 'warning' as const,
  description: '点击"测试连接"按钮检查后端服务状态'
})

// 加载状态
const authLoading = ref(false)
const usersLoading = ref(false)
const statsLoading = ref(false)
const packagesLoading = ref(false)
const logsLoading = ref(false)
const actionsLoading = ref(false)
const batchLoading = ref(false)

// 测试结果
const authResult = ref(null)
const usersResult = ref(null)
const statsResult = ref(null)
const packagesResult = ref(null)
const logsResult = ref(null)
const actionsResult = ref(null)
const batchResults = ref<Array<{name: string, success: boolean, message: string}>>([])

// 测试连接
async function testConnection() {
  try {
    const response = await healthCheck()
    if (response.code === 0) {
      connectionStatus.title = '连接成功'
      connectionStatus.type = 'success'
      connectionStatus.description = `后端服务运行正常，响应时间: ${Date.now()}ms`
      ElMessage.success('后端连接成功')
    } else {
      connectionStatus.title = '连接失败'
      connectionStatus.type = 'error'
      connectionStatus.description = response.message || '后端服务响应异常'
      ElMessage.error('后端连接失败')
    }
  } catch (error) {
    connectionStatus.title = '连接失败'
    connectionStatus.type = 'error'
    connectionStatus.description = '无法连接到后端服务，请检查服务是否启动'
    ElMessage.error('无法连接到后端服务')
  }
}

// 测试认证
async function testAuth() {
  authLoading.value = true
  try {
    const response = await authApi.login({
      username: 'admin',
      password: 'admin123'
    })
    authResult.value = response
    ElMessage.success('认证测试成功')
  } catch (error) {
    authResult.value = { error: error.message }
    ElMessage.error('认证测试失败')
  } finally {
    authLoading.value = false
  }
}

// 测试用户管理
async function testUsers() {
  usersLoading.value = true
  try {
    const response = await userApi.getUsers({ page: 1, pageSize: 10 })
    usersResult.value = response
    ElMessage.success('用户管理测试成功')
  } catch (error) {
    usersResult.value = { error: error.message }
    ElMessage.error('用户管理测试失败')
  } finally {
    usersLoading.value = false
  }
}

// 测试统计信息
async function testStats() {
  statsLoading.value = true
  try {
    const response = await adminApi.getStats()
    statsResult.value = response
    ElMessage.success('统计信息测试成功')
  } catch (error) {
    statsResult.value = { error: error.message }
    ElMessage.error('统计信息测试失败')
  } finally {
    statsLoading.value = false
  }
}

// 测试绳包管理
async function testPackages() {
  packagesLoading.value = true
  try {
    const response = await packageApi.getPackages({ page: 1, pageSize: 10 })
    packagesResult.value = response
    ElMessage.success('绳包管理测试成功')
  } catch (error) {
    packagesResult.value = { error: error.message }
    ElMessage.error('绳包管理测试失败')
  } finally {
    packagesLoading.value = false
  }
}

// 测试系统日志
async function testLogs() {
  logsLoading.value = true
  try {
    const response = await adminApi.getLogs({ page: 1, pageSize: 10 })
    logsResult.value = response
    ElMessage.success('系统日志测试成功')
  } catch (error) {
    logsResult.value = { error: error.message }
    ElMessage.error('系统日志测试失败')
  } finally {
    logsLoading.value = false
  }
}

// 测试用户行为
async function testUserActions() {
  actionsLoading.value = true
  try {
    const response = await adminApi.getUserActions({ page: 1, pageSize: 10 })
    actionsResult.value = response
    ElMessage.success('用户行为测试成功')
  } catch (error) {
    actionsResult.value = { error: error.message }
    ElMessage.error('用户行为测试失败')
  } finally {
    actionsLoading.value = false
  }
}

// 运行所有测试
async function runAllTests() {
  batchLoading.value = true
  batchResults.value = []
  
  const tests = [
    { name: '健康检查', fn: testConnection },
    { name: '认证测试', fn: testAuth },
    { name: '用户管理', fn: testUsers },
    { name: '统计信息', fn: testStats },
    { name: '绳包管理', fn: testPackages },
    { name: '系统日志', fn: testLogs },
    { name: '用户行为', fn: testUserActions }
  ]

  for (const test of tests) {
    try {
      await test.fn()
      batchResults.value.push({
        name: test.name,
        success: true,
        message: '测试通过'
      })
    } catch (error) {
      batchResults.value.push({
        name: test.name,
        success: false,
        message: error.message || '测试失败'
      })
    }
  }

  batchLoading.value = false
  ElMessage.success('批量测试完成')
}
</script>

<style scoped>
.api-test-page {
  padding: 20px;
}

.test-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.status-section {
  margin-bottom: 20px;
}

.test-section {
  margin-bottom: 30px;
}

.test-item {
  margin-bottom: 20px;
}

.test-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.test-result {
  margin-top: 10px;
  padding: 10px;
  background-color: #f5f5f5;
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
}

.test-result pre {
  margin: 0;
  font-size: 12px;
  white-space: pre-wrap;
  word-break: break-all;
}

.batch-test {
  margin-top: 30px;
  padding-top: 20px;
  border-top: 1px solid #eee;
}

.batch-result {
  margin-top: 15px;
}

.batch-item {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
}

.batch-message {
  color: #666;
  font-size: 14px;
}
</style> 