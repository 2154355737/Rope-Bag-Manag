<template>
  <div class="log-view">
    <el-card>
      <h2>日志查看</h2>
      <div class="log-controls">
        <el-button @click="refresh">刷新</el-button>
        <el-pagination
          background
          layout="prev, pager, next"
          :total="total"
          :page-size="pageSize"
          :current-page.sync="page"
          @current-change="loadLogs"
          style="margin-left: 24px; display: inline-block;"
        />
      </div>
      <el-table :data="logs" style="width: 100%; margin-top: 12px;">
        <el-table-column prop="timestamp" label="时间" width="180" />
        <el-table-column prop="level" label="级别" width="80">
          <template #default="scope">
            <el-tag :type="scope.row.level === 'ERROR' ? 'danger' : (scope.row.level === 'WARN' ? 'warning' : 'success')">
              {{ scope.row.level }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="message" label="内容" />
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { getLogEntries } from '../api'
import { ElMessage } from 'element-plus'
import { useRouter } from 'vue-router'

const logs = ref<any[]>([])
const page = ref(1)
const pageSize = 20
const total = ref(1000) // 这里假设最大1000条，后端如有总数可替换
const router = useRouter()

async function loadLogs(val = 1) {
  page.value = val
  try {
    const res = await getLogEntries(page.value, pageSize)
    if (res.code === 0 && res.data) {
      logs.value = res.data.entries || res.data
      // 如后端返回总数可赋值 total.value = res.data.total
    }
  } catch (error) {
    console.error('加载日志失败:', error)
    
    // 检查是否是网络错误或服务不可用
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED') ||
        errorMessage.includes('Service unavailable')) {
      
      // 显示服务异常提示
      ElMessage.error('服务异常已安全退出！')
      
      // 延迟跳转到登录页面
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      
      return
    }
    
    // 其他错误显示通用提示
    ElMessage.error('日志加载失败，请稍后重试')
  }
}

function refresh() {
  loadLogs(page.value)
}

watch(page, (val) => loadLogs(val))
onMounted(() => loadLogs(1))
</script>

<style scoped>
.log-view {
  padding: 32px;
}
.log-controls {
  margin-bottom: 16px;
  display: flex;
  align-items: center;
}
</style> 