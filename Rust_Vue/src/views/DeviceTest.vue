<template>
  <div class="device-test">
    <el-card>
      <div class="header">
        <h2>设备检测测试</h2>
        <p>此页面用于测试设备检测和自动切换功能</p>
      </div>

      <div class="device-info">
        <h3>当前设备信息</h3>
        <el-descriptions :column="2" border>
          <el-descriptions-item label="设备类型">{{ deviceInfo.deviceType }}</el-descriptions-item>
          <el-descriptions-item label="屏幕宽度">{{ deviceInfo.screenWidth }}px</el-descriptions-item>
          <el-descriptions-item label="屏幕高度">{{ deviceInfo.screenHeight }}px</el-descriptions-item>
          <el-descriptions-item label="是否移动设备">{{ deviceInfo.isMobile ? '是' : '否' }}</el-descriptions-item>
          <el-descriptions-item label="是否小屏幕">{{ deviceInfo.isSmallScreen ? '是' : '否' }}</el-descriptions-item>
          <el-descriptions-item label="是否触摸设备">{{ deviceInfo.isTouch ? '是' : '否' }}</el-descriptions-item>
        </el-descriptions>
      </div>

      <div class="recommendation">
        <h3>推荐界面</h3>
        <el-alert
          :title="recommendationTitle"
          :description="recommendationDescription"
          :type="recommendationType"
          show-icon
        />
      </div>

      <div class="test-actions">
        <h3>测试操作</h3>
        <el-row :gutter="20">
          <el-col :span="6">
            <el-button type="primary" @click="goToUserManage" block>
              访问用户管理
            </el-button>
          </el-col>
          <el-col :span="6">
            <el-button type="success" @click="refreshDeviceInfo" block>
              刷新设备信息
            </el-button>
          </el-col>
          <el-col :span="6">
            <el-button type="warning" @click="testScreenResize" block>
              测试屏幕变化
            </el-button>
          </el-col>
          <el-col :span="6">
            <el-button type="info" @click="testThemeSwitch" block>
              测试主题切换
            </el-button>
          </el-col>
        </el-row>
      </div>

      <div class="user-agent">
        <h3>User-Agent 信息</h3>
        <el-input
          v-model="deviceInfo.userAgent"
          type="textarea"
          :rows="3"
          readonly
        />
      </div>

      <div class="theme-status">
        <h3>主题状态</h3>
        <el-descriptions :column="2" border>
          <el-descriptions-item label="当前主题">
            <el-tag :type="currentTheme === 'dark' ? 'danger' : 'success'">
              {{ currentTheme === 'dark' ? '深色模式' : '浅色模式' }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="Body类名">
            {{ bodyClasses }}
          </el-descriptions-item>
          <el-descriptions-item label="HTML类名">
            {{ htmlClasses }}
          </el-descriptions-item>
          <el-descriptions-item label="主题切换">
            <el-button @click="toggleTheme" type="primary" size="small">
              切换主题
            </el-button>
          </el-descriptions-item>
        </el-descriptions>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { getDeviceInfo, shouldUseMobileVersion } from '../utils/device'

const router = useRouter()
const deviceInfo = ref(getDeviceInfo())
const currentTheme = ref('light')
const bodyClasses = ref('')
const htmlClasses = ref('')

// 计算推荐信息
const recommendationTitle = computed(() => {
  const shouldUseMobile = shouldUseMobileVersion()
  return shouldUseMobile ? '推荐使用移动端界面' : '推荐使用桌面端界面'
})

const recommendationDescription = computed(() => {
  const shouldUseMobile = shouldUseMobileVersion()
  if (shouldUseMobile) {
    return '检测到您使用的是移动设备或小屏幕设备，建议使用移动端优化的用户管理界面。'
  } else {
    return '检测到您使用的是桌面设备，建议使用桌面端用户管理界面以获得最佳体验。'
  }
})

const recommendationType = computed(() => {
  const shouldUseMobile = shouldUseMobileVersion()
  return shouldUseMobile ? 'warning' : 'success'
})

function refreshDeviceInfo() {
  deviceInfo.value = getDeviceInfo()
}

function goToUserManage() {
  router.push('/users')
}

function testScreenResize() {
  // 模拟屏幕尺寸变化
  const originalWidth = window.innerWidth
  const newWidth = originalWidth <= 768 ? 1024 : 375
  
  // 临时修改innerWidth进行测试
  Object.defineProperty(window, 'innerWidth', {
    writable: true,
    configurable: true,
    value: newWidth
  })
  
  refreshDeviceInfo()
  
  // 恢复原始宽度
  setTimeout(() => {
    Object.defineProperty(window, 'innerWidth', {
      writable: true,
      configurable: true,
      value: originalWidth
    })
    refreshDeviceInfo()
  }, 2000)
}

function testThemeSwitch() {
  // 测试主题切换功能
  const body = document.body
  const html = document.documentElement
  
  // 检查当前主题
  const isDark = body.classList.contains('dark')
  
  if (isDark) {
    // 切换到浅色模式
    body.classList.remove('dark')
    html.classList.remove('dark')
    body.classList.add('light')
    html.classList.add('light')
    ElMessage.success('已切换到浅色模式')
  } else {
    // 切换到深色模式
    body.classList.remove('light')
    html.classList.remove('light')
    body.classList.add('dark')
    html.classList.add('dark')
    ElMessage.success('已切换到深色模式')
  }
  
  // 3秒后恢复原始主题
  setTimeout(() => {
    if (isDark) {
      body.classList.remove('light')
      html.classList.remove('light')
      body.classList.add('dark')
      html.classList.add('dark')
    } else {
      body.classList.remove('dark')
      html.classList.remove('dark')
      body.classList.add('light')
      html.classList.add('light')
    }
    ElMessage.info('已恢复原始主题')
  }, 3000)
}

function toggleTheme() {
  const body = document.body
  const html = document.documentElement
  
  if (body.classList.contains('dark')) {
    body.classList.remove('dark')
    html.classList.remove('dark')
    body.classList.add('light')
    html.classList.add('light')
    currentTheme.value = 'light'
    ElMessage.success('已切换到浅色模式')
  } else {
    body.classList.remove('light')
    html.classList.remove('light')
    body.classList.add('dark')
    html.classList.add('dark')
    currentTheme.value = 'dark'
    ElMessage.success('已切换到深色模式')
  }
  
  updateThemeStatus()
}

function updateThemeStatus() {
  const body = document.body
  const html = document.documentElement
  
  bodyClasses.value = body.className
  htmlClasses.value = html.className
  
  if (body.classList.contains('dark')) {
    currentTheme.value = 'dark'
  } else if (body.classList.contains('light')) {
    currentTheme.value = 'light'
  } else {
    currentTheme.value = 'none'
  }
}

onMounted(() => {
  // 监听屏幕尺寸变化
  window.addEventListener('resize', refreshDeviceInfo)
  
  // 初始化主题状态
  updateThemeStatus()
  
  // 监听主题变化
  const observer = new MutationObserver(() => {
    updateThemeStatus()
  })
  
  observer.observe(document.body, {
    attributes: true,
    attributeFilter: ['class']
  })
})
</script>

<style scoped>
.device-test {
  padding: 32px;
}

.header {
  margin-bottom: 24px;
}

.header h2 {
  margin: 0 0 8px 0;
  color: #303133;
}

.header p {
  margin: 0;
  color: #606266;
}

.device-info,
.recommendation,
.test-actions,
.user-agent {
  margin-bottom: 24px;
}

.device-info h3,
.recommendation h3,
.test-actions h3,
.user-agent h3 {
  margin: 0 0 16px 0;
  color: #303133;
}

.test-actions .el-row {
  margin-top: 16px;
}

.user-agent .el-input {
  margin-top: 16px;
}

@media (max-width: 768px) {
  .device-test {
    padding: 16px;
  }
  
  .test-actions .el-col {
    margin-bottom: 12px;
  }
}

/* Element Plus 主题切换支持 */
:deep(.dark) .device-test {
  background-color: #1a1a1a;
}

:deep(.dark) .header h2 {
  color: #ffffff;
}

:deep(.dark) .header p {
  color: #b0b0b0;
}

:deep(.dark) .device-info h3,
:deep(.dark) .recommendation h3,
:deep(.dark) .test-actions h3,
:deep(.dark) .user-agent h3 {
  color: #ffffff;
}

:deep(.dark) .el-descriptions {
  background-color: #2d2d2d;
  border-color: #404040;
}

/* 浅色模式优化 */
:deep(.light) .device-test {
  background-color: #f5f5f5;
}

:deep(.light) .header h2 {
  color: #303133;
}

:deep(.light) .header p {
  color: #606266;
}

:deep(.light) .device-info h3,
:deep(.light) .recommendation h3,
:deep(.light) .test-actions h3,
:deep(.light) .user-agent h3 {
  color: #303133;
}

:deep(.light) .el-descriptions {
  background-color: #ffffff;
  border-color: #e4e7ed;
}
</style> 