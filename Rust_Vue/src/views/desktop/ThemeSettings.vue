<template>
  <div class="theme-settings">
    <el-card class="settings-card">
      <template #header>
        <div class="settings-header">
          <h2>主题设置</h2>
          <p>自定义您的界面主题和外观</p>
        </div>
      </template>

      <div class="settings-content">
        <!-- 主题选择 -->
        <div class="setting-section">
          <h3>主题选择</h3>
          <div class="theme-grid">
            <div 
              v-for="theme in availableThemes" 
              :key="theme.key"
              class="theme-option"
              :class="{ active: currentTheme === theme.key }"
              @click="switchTheme(theme.key)"
            >
              <div class="theme-preview" :style="{ backgroundColor: getThemeColor(theme.key) }"></div>
              <div class="theme-info">
                <span class="theme-icon" v-text="theme.icon"></span>
                <span class="theme-name">{{ theme.name }}</span>
                <span class="theme-desc">{{ theme.description }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 当前主题预览 -->
        <div class="setting-section">
          <h3>当前主题预览</h3>
          <div class="theme-preview-card">
            <div class="preview-header">
              <h4>{{ currentThemeConfig.label }}</h4>
              <span class="preview-icon" v-text="currentThemeConfig.icon"></span>
            </div>
            <div class="preview-content">
              <p>{{ currentThemeConfig.description }}</p>
              <div class="preview-colors">
                <div class="color-item">
                  <div class="color-preview" style="background-color: var(--bg-primary)"></div>
                  <span>主背景</span>
                </div>
                <div class="color-item">
                  <div class="color-preview" style="background-color: var(--text-primary)"></div>
                  <span>主文字</span>
                </div>
                <div class="color-item">
                  <div class="color-preview" style="background-color: var(--brand-color)"></div>
                  <span>品牌色</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 快捷键帮助 -->
        <div class="setting-section">
          <h3>快捷键帮助</h3>
          <div class="shortcut-list">
            <div class="shortcut-item">
              <kbd>Ctrl + T</kbd>
              <span>快速切换浅色/深色模式</span>
            </div>
            <div class="shortcut-item">
              <kbd>Ctrl + Shift + T</kbd>
              <span>打开主题菜单</span>
            </div>
          </div>
        </div>

        <!-- 主题测试区域 -->
        <div class="setting-section">
          <h3>主题测试区域</h3>
          <div class="theme-test-area">
            <!-- 快速测试按钮 -->
            <div class="quick-test">
              <h4>快速测试</h4>
              <div class="test-buttons">
                <el-button @click="testTheme('light')" type="primary">测试浅色</el-button>
                <el-button @click="testTheme('dark')" type="primary">测试深色</el-button>
                <el-button @click="testTheme('blue')" type="primary">测试蓝色</el-button>
                <el-button @click="testTheme('green')" type="primary">测试绿色</el-button>
                <el-button @click="testTheme('purple')" type="primary">测试紫色</el-button>
                <el-button @click="testTheme('orange')" type="primary">测试橙色</el-button>
                <el-button @click="testTheme('red')" type="primary">测试红色</el-button>
              </div>
              <p class="test-info">当前主题: {{ currentTheme }} | HTML类: {{ htmlClasses }} | Body类: {{ bodyClasses }}</p>
            </div>
            
            <div class="test-card">
              <h4>测试卡片</h4>
              <p>这是一个测试卡片，用于验证主题切换效果。</p>
              <div class="test-buttons">
                <el-button type="primary">主要按钮</el-button>
                <el-button type="success">成功按钮</el-button>
                <el-button type="warning">警告按钮</el-button>
                <el-button type="danger">危险按钮</el-button>
              </div>
            </div>
            
            <div class="test-form">
              <h4>测试表单</h4>
              <el-form label-width="80px">
                <el-form-item label="用户名">
                  <el-input placeholder="请输入用户名" />
                </el-form-item>
                <el-form-item label="密码">
                  <el-input type="password" placeholder="请输入密码" />
                </el-form-item>
                <el-form-item label="选择">
                  <el-select placeholder="请选择">
                    <el-option label="选项1" value="1" />
                    <el-option label="选项2" value="2" />
                    <el-option label="选项3" value="3" />
                  </el-select>
                </el-form-item>
              </el-form>
            </div>
            
            <div class="test-table">
              <h4>测试表格</h4>
              <el-table :data="testTableData" style="width: 100%">
                <el-table-column prop="name" label="姓名" />
                <el-table-column prop="age" label="年龄" />
                <el-table-column prop="city" label="城市" />
              </el-table>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="setting-actions">
          <el-button type="primary" @click="saveSettings">保存设置</el-button>
          <el-button @click="resetSettings">重置设置</el-button>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { 
  getCurrentTheme, 
  applyTheme, 
  themeConfigs, 
  type ThemeType 
} from '../../utils/theme'

// 响应式数据
const currentTheme = ref(getCurrentTheme())

const availableThemes = computed(() => {
  return Object.entries(themeConfigs).map(([key, config]) => ({
    key: key as ThemeType,
    name: config.label,
    icon: config.icon,
    description: config.description
  }))
})

const currentThemeConfig = computed(() => {
  return themeConfigs[currentTheme.value] || themeConfigs.light
})

// 测试数据
const testTableData = ref([
  { name: '张三', age: 25, city: '北京' },
  { name: '李四', age: 30, city: '上海' },
  { name: '王五', age: 28, city: '广州' }
])

// 计算当前HTML和Body的类名
const htmlClasses = computed(() => {
  return document.documentElement.className
})

const bodyClasses = computed(() => {
  return document.body.className
})

// 方法
function switchTheme(themeKey: string) {
  applyTheme(themeKey as ThemeType)
  currentTheme.value = getCurrentTheme()
  ElMessage.success(`已切换到${themeConfigs[themeKey as ThemeType].label}`)
}

// 测试主题切换
function testTheme(theme: ThemeType) {
  console.log('测试主题切换:', theme)
  applyTheme(theme)
  currentTheme.value = getCurrentTheme()
  
  // 显示测试结果
  setTimeout(() => {
    const htmlClass = document.documentElement.className
    const bodyClass = document.body.className
    console.log('测试结果:', {
      theme,
      htmlClass,
      bodyClass,
      hasThemeClass: htmlClass.includes(theme) || bodyClass.includes(theme)
    })
    
    ElMessage.success(`测试主题: ${themeConfigs[theme].label}`)
  }, 100)
}

function getThemeColor(themeKey: string): string {
  const themeColors = {
    light: '#409EFF',
    dark: '#409EFF',
    blue: '#1890FF',
    green: '#52C41A',
    purple: '#722ED1',
    orange: '#FA8C16',
    red: '#F5222D',
    auto: '#409EFF'
  }
  return themeColors[themeKey as ThemeType] || '#409EFF'
}

function saveSettings() {
  localStorage.setItem('theme-settings', JSON.stringify({
    currentTheme: currentTheme.value
  }))
  ElMessage.success('设置已保存')
}

function resetSettings() {
  applyTheme('light')
  currentTheme.value = getCurrentTheme()
  localStorage.removeItem('theme-settings')
  ElMessage.success('设置已重置')
}

onMounted(() => {
  // 加载保存的设置
  const savedSettings = localStorage.getItem('theme-settings')
  if (savedSettings) {
    try {
      const settings = JSON.parse(savedSettings)
      if (settings.currentTheme) {
        applyTheme(settings.currentTheme)
        currentTheme.value = getCurrentTheme()
      }
    } catch (e) {
      console.warn('Failed to load theme settings:', e)
    }
  }
})
</script>

<style scoped>
.theme-settings {
  padding: 20px;
}

.settings-card {
  max-width: 800px;
  margin: 0 auto;
}

.settings-header h2 {
  margin: 0 0 8px 0;
  color: var(--text-primary);
}

.settings-header p {
  margin: 0;
  color: var(--text-secondary);
}

.setting-section {
  margin-bottom: 32px;
}

.setting-section h3 {
  margin: 0 0 16px 0;
  color: var(--text-primary);
  font-size: 18px;
  font-weight: 600;
}

.theme-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.theme-option {
  display: flex;
  align-items: center;
  padding: 16px;
  border: 2px solid var(--border-color);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  background-color: var(--bg-card);
}

.theme-option:hover {
  border-color: var(--brand-color);
  transform: translateY(-2px);
  box-shadow: var(--shadow-base);
}

.theme-option.active {
  border-color: var(--brand-color);
  background-color: var(--bg-secondary);
}

.theme-preview {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  margin-right: 12px;
  border: 2px solid var(--border-color);
}

.theme-info {
  flex: 1;
}

.theme-icon {
  font-size: 20px;
  margin-right: 8px;
}

.theme-name {
  display: block;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.theme-desc {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
}

.theme-preview-card {
  padding: 20px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-card);
}

.preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.preview-header h4 {
  margin: 0;
  color: var(--text-primary);
}

.preview-icon {
  font-size: 24px;
}

.preview-content p {
  margin: 0 0 16px 0;
  color: var(--text-secondary);
}

.preview-colors {
  display: flex;
  gap: 16px;
}

.color-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.color-preview {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  border: 1px solid var(--border-color);
}

.color-item span {
  font-size: 12px;
  color: var(--text-secondary);
}

.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.shortcut-item kbd {
  padding: 4px 8px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
  color: var(--text-primary);
}

.shortcut-item span {
  color: var(--text-secondary);
}

.theme-test-area {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 20px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-card);
}

.quick-test {
  padding: 15px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  margin-bottom: 20px;
}

.quick-test h4 {
  margin: 0 0 15px 0;
  color: var(--text-primary);
}

.test-buttons {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  margin-bottom: 15px;
}

.test-info {
  font-size: 14px;
  color: var(--text-secondary);
  text-align: center;
}

.test-card {
  padding: 15px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
}

.test-card h4 {
  margin: 0 0 10px 0;
  color: var(--text-primary);
}

.test-card p {
  margin: 0 0 15px 0;
  color: var(--text-secondary);
}

.test-form {
  padding: 15px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
}

.test-form .el-form-item {
  margin-bottom: 15px;
}

.test-form .el-form-item label {
  color: var(--text-primary);
}

.test-form .el-input,
.test-form .el-select {
  width: 100%;
}

.test-table {
  padding: 15px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
}

.test-table .el-table {
  border-radius: 6px;
}

.test-table .el-table th {
  background-color: var(--bg-secondary);
  color: var(--text-primary);
}

.test-table .el-table td {
  color: var(--text-secondary);
}

.setting-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  margin-top: 32px;
}

/* 深色模式适配 */
.dark .theme-settings-desktop {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .page-header,
.dark .theme-card,
.dark .theme-preview-card,
.dark .theme-test-area {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

/* 主题适配 */
.blue .theme-card::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .theme-card::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .theme-card::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .theme-card::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

/* 动画效果 */
@keyframes slide-up {
  0% {
    opacity: 0;
    transform: translateY(20px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.05);
    opacity: 0.9;
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) scale(1);
    opacity: 0.6;
  }
  50% {
    transform: translateY(-8px) scale(1.1);
    opacity: 1;
  }
}

/* 页面加载动画 */
.page-header {
  animation: slide-up 0.6s ease-out;
}

.theme-grid {
  animation: slide-up 0.6s ease-out 0.2s both;
}

.theme-test-area {
  animation: slide-up 0.6s ease-out 0.4s both;
}

.setting-actions {
  animation: slide-up 0.6s ease-out 0.6s both;
}

/* 主题卡片悬停动画 */
.theme-card {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.theme-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  transform: scaleX(0);
  transition: transform 0.3s ease;
}

.theme-card::after {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(45deg, transparent, rgba(255, 255, 255, 0.05), transparent);
  transform: translateX(-100%) translateY(-100%) rotate(45deg);
  transition: transform 0.6s ease;
}

.theme-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-medium);
}

.theme-card:hover::before {
  transform: scaleX(1);
}

.theme-card:hover::after {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

/* 按钮光泽动画 */
.el-button {
  position: relative;
  overflow: hidden;
}

.el-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.el-button:hover::before {
  left: 100%;
}

/* 测试区域动画 */
.quick-test,
.test-card,
.test-form,
.test-table {
  transition: all 0.3s ease;
}

.quick-test:hover,
.test-card:hover,
.test-form:hover,
.test-table:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-light);
}

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .page-header,
  .theme-grid,
  .theme-test-area,
  .setting-actions {
    animation: none;
  }
  
  .theme-card {
    transition: none;
  }
  
  .theme-card:hover {
    transform: none;
  }
  
  .el-button::before {
    display: none;
  }
  
  .quick-test,
  .test-card,
  .test-form,
  .test-table {
    transition: none;
  }
  
  .quick-test:hover,
  .test-card:hover,
  .test-form:hover,
  .test-table:hover {
    transform: none;
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .theme-grid {
    grid-template-columns: 1fr;
  }
  
  .preview-colors {
    flex-direction: column;
    align-items: center;
  }
  
  .setting-actions {
    flex-direction: column;
  }
}
</style> 