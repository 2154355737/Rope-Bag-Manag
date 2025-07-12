<template>
  <div class="package-manage-mobile">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="24"><Box /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">绳包管理</h1>
            <p class="page-subtitle">管理绳包信息和状态</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button 
            type="primary" 
            size="small"
            @click="showAddDialog"
            :icon="Plus"
          >
            添加绳包
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="20"><Box /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalPackages }}</div>
            <div class="stat-label">总绳包</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="20"><Check /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ availablePackages }}</div>
            <div class="stat-label">可用绳包</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="20"><Setting /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ maintenancePackages }}</div>
            <div class="stat-label">维护中</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和筛选 -->
    <div class="search-section">
      <div class="search-box">
        <el-input
          v-model="searchQuery"
          placeholder="搜索绳包..."
          prefix-icon="Search"
          clearable
          @input="handleSearch"
        />
      </div>
      <div class="filter-tabs">
        <div 
          v-for="tab in filterTabs" 
          :key="tab.key"
          class="filter-tab"
          :class="{ active: activeFilter === tab.key }"
          @click="setFilter(tab.key)"
        >
          <span class="tab-icon">{{ tab.icon }}</span>
          <span class="tab-label">{{ tab.label }}</span>
          <span class="tab-count">{{ tab.count }}</span>
        </div>
      </div>
    </div>

    <!-- 绳包列表 -->
    <div class="packages-list">
      <div v-for="pkg in filteredPackages" :key="pkg.id" class="package-card">
        <div class="package-header">
          <div class="package-icon">
            <el-icon :size="24"><Box /></el-icon>
          </div>
          <div class="package-status" :class="pkg.status">
            <span class="status-text">{{ getStatusText(pkg.status) }}</span>
          </div>
        </div>
        
        <div class="package-info">
          <div class="package-title">
            <h3 class="package-name">{{ pkg.name }}</h3>
            <div class="package-badge" :class="pkg.type">
              {{ pkg.type === 'rope' ? '绳索' : '装备' }}
            </div>
          </div>
          
          <div class="package-details">
            <div class="detail-item">
              <el-icon><User /></el-icon>
              <span>作者: {{ pkg.author }}</span>
            </div>
            <div class="detail-item">
              <el-icon><Star /></el-icon>
              <span>版本: {{ pkg.version }}</span>
            </div>
            <div class="detail-item">
              <el-icon><Download /></el-icon>
              <span>下载: {{ pkg.downloads }}次</span>
            </div>
          </div>
          
          <div class="package-desc">
            {{ pkg.description }}
          </div>
          
          <div class="package-meta">
            <div class="meta-item">
              <span class="meta-label">上传时间</span>
              <span class="meta-value">{{ formatDate(pkg.uploadTime) }}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">最后更新</span>
              <span class="meta-value">{{ formatDate(pkg.lastUpdate) }}</span>
            </div>
          </div>
        </div>
        
        <div class="package-actions">
          <el-button 
            size="small" 
            type="primary" 
            @click="viewPackage(pkg)"
            :icon="View"
          >
            查看
          </el-button>
          <el-button 
            size="small" 
            type="success" 
            @click="downloadPackage(pkg)"
            :icon="Download"
          >
            下载
          </el-button>
          <el-button 
            size="small" 
            type="warning" 
            @click="editPackage(pkg)"
            :icon="Edit"
          >
            编辑
          </el-button>
          <el-button 
            size="small" 
            type="danger" 
            @click="deletePackage(pkg)"
            :icon="Delete"
          >
            删除
          </el-button>
        </div>
      </div>
    </div>

    <!-- 加载更多 -->
    <div v-if="hasMore" class="load-more">
      <el-button 
        type="primary" 
        plain 
        @click="loadMore"
        :loading="loading"
      >
        加载更多
      </el-button>
    </div>

    <!-- 空状态 -->
    <div v-if="filteredPackages.length === 0" class="empty-state">
      <div class="empty-icon">
        <el-icon :size="48"><Box /></el-icon>
      </div>
      <h3 class="empty-title">暂无绳包</h3>
      <p class="empty-desc">当前筛选条件下没有找到绳包</p>
    </div>

    <!-- 绳包详情对话框 -->
    <el-dialog
      v-model="detailDialogVisible"
      title="绳包详情"
      width="90%"
      :close-on-click-modal="false"
    >
      <div v-if="selectedPackage" class="package-detail">
        <div class="detail-header">
          <h3>{{ selectedPackage.name }}</h3>
          <div class="detail-status" :class="selectedPackage.status">
            {{ getStatusText(selectedPackage.status) }}
          </div>
        </div>
        
        <div class="detail-content">
          <div class="detail-section">
            <h4>基本信息</h4>
            <div class="detail-grid">
              <div class="detail-item">
                <span class="label">作者:</span>
                <span class="value">{{ selectedPackage.author }}</span>
              </div>
              <div class="detail-item">
                <span class="label">版本:</span>
                <span class="value">{{ selectedPackage.version }}</span>
              </div>
              <div class="detail-item">
                <span class="label">类型:</span>
                <span class="value">{{ selectedPackage.type === 'rope' ? '绳索' : '装备' }}</span>
              </div>
              <div class="detail-item">
                <span class="label">下载次数:</span>
                <span class="value">{{ selectedPackage.downloads }}</span>
              </div>
            </div>
          </div>
          
          <div class="detail-section">
            <h4>描述</h4>
            <p>{{ selectedPackage.description }}</p>
          </div>
          
          <div class="detail-section">
            <h4>下载链接</h4>
            <div class="download-links">
              <div class="download-item">
                <span class="link-label">主文件:</span>
                <a :href="selectedPackage.downloadUrl" target="_blank" class="download-link">
                  {{ selectedPackage.name }}.zip
                </a>
              </div>
              <div class="download-item">
                <span class="link-label">文档:</span>
                <a :href="selectedPackage.docUrl" target="_blank" class="download-link">
                  使用说明.pdf
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </el-dialog>

    <!-- 添加绳包对话框 -->
    <el-dialog
      v-model="addDialogVisible"
      title="添加绳包"
      width="90%"
      :close-on-click-modal="false"
    >
      <el-form :model="newPackage" label-width="80px">
        <el-form-item label="名称">
          <el-input v-model="newPackage.name" placeholder="请输入绳包名称" />
        </el-form-item>
        <el-form-item label="作者">
          <el-input v-model="newPackage.author" placeholder="请输入作者" />
        </el-form-item>
        <el-form-item label="版本">
          <el-input v-model="newPackage.version" placeholder="请输入版本号" />
        </el-form-item>
        <el-form-item label="类型">
          <el-select v-model="newPackage.type" placeholder="选择类型">
            <el-option label="绳索" value="rope" />
            <el-option label="装备" value="equipment" />
          </el-select>
        </el-form-item>
        <el-form-item label="描述">
          <el-input 
            v-model="newPackage.description" 
            type="textarea" 
            :rows="3"
            placeholder="请输入描述"
          />
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="newPackage.status" placeholder="选择状态">
            <el-option label="可用" value="available" />
            <el-option label="维护中" value="maintenance" />
            <el-option label="已借出" value="borrowed" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="addPackage">添加</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Box, 
  User, 
  Star, 
  Download, 
  View, 
  Edit, 
  Delete, 
  Plus,
  Check,
  Setting,
  Search
} from '@element-plus/icons-vue'

// 响应式数据
const searchQuery = ref('')
const activeFilter = ref('all')
const loading = ref(false)
const detailDialogVisible = ref(false)
const addDialogVisible = ref(false)
const selectedPackage = ref<any>(null)

// 绳包数据
const packages = ref<any[]>([])
const totalPackages = ref(0)
const availablePackages = ref(0)
const maintenancePackages = ref(0)

// 新绳包表单
const newPackage = ref({
  name: '',
  author: '',
  version: '',
  type: 'rope',
  description: '',
  status: 'available'
})

// 筛选标签
const filterTabs = ref([
  { key: 'all', label: '全部', icon: '📦', count: 0 },
  { key: 'available', label: '可用', icon: '✅', count: 0 },
  { key: 'maintenance', label: '维护', icon: '🔧', count: 0 },
  { key: 'borrowed', label: '借出', icon: '📤', count: 0 }
])

// 计算属性
const filteredPackages = computed(() => {
  let filtered = packages.value
  
  // 搜索过滤
  if (searchQuery.value) {
    filtered = filtered.filter(pkg =>
      pkg.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      pkg.author.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  }
  
  // 状态过滤
  if (activeFilter.value !== 'all') {
    filtered = filtered.filter(pkg => pkg.status === activeFilter.value)
  }
  
  return filtered
})

const hasMore = computed(() => {
  return filteredPackages.value.length < packages.value.length
})

// 方法
function handleSearch() {
  // 搜索逻辑
}

function setFilter(filter: string) {
  activeFilter.value = filter
}

function getStatusText(status: string) {
  const statusMap: Record<string, string> = {
    available: '可用',
    maintenance: '维护中',
    borrowed: '已借出'
  }
  return statusMap[status] || status
}

function viewPackage(pkg: any) {
  selectedPackage.value = pkg
  detailDialogVisible.value = true
}

function downloadPackage(pkg: any) {
  ElMessage.success(`开始下载 ${pkg.name}`)
}

function editPackage(pkg: any) {
  ElMessage.info('编辑功能开发中')
}

function deletePackage(pkg: any) {
  ElMessageBox.confirm(
    `确定要删除绳包 ${pkg.name} 吗？`,
    '确认删除',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    }
  ).then(() => {
    ElMessage.success('绳包已删除')
  }).catch(() => {
    // 用户取消
  })
}

function showAddDialog() {
  addDialogVisible.value = true
}

function addPackage() {
  // 添加绳包逻辑
  ElMessage.success('绳包添加成功')
  addDialogVisible.value = false
}

function loadMore() {
  // 加载更多绳包
  loading.value = true
  setTimeout(() => {
    loading.value = false
    ElMessage.success('已加载更多绳包')
  }, 1000)
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('zh-CN')
}

// 初始化数据
onMounted(() => {
  // 模拟数据
  packages.value = [
    {
      id: 1,
      name: '基础绳索套装',
      author: '张三',
      version: '1.0.0',
      type: 'rope',
      status: 'available',
      description: '包含多种规格的绳索，适用于基础训练和日常使用',
      downloads: 156,
      uploadTime: '2024-01-01',
      lastUpdate: '2024-01-15',
      downloadUrl: 'https://example.com/rope1.zip',
      docUrl: 'https://example.com/rope1.pdf'
    },
    {
      id: 2,
      name: '专业攀岩装备',
      author: '李四',
      version: '2.1.0',
      type: 'equipment',
      status: 'maintenance',
      description: '专业级攀岩装备，包含安全带、头盔等安全设备',
      downloads: 89,
      uploadTime: '2024-01-10',
      lastUpdate: '2024-01-20',
      downloadUrl: 'https://example.com/equipment1.zip',
      docUrl: 'https://example.com/equipment1.pdf'
    },
    {
      id: 3,
      name: '救援绳索包',
      author: '王五',
      version: '1.5.2',
      type: 'rope',
      status: 'borrowed',
      description: '专门用于救援工作的绳索套装，具有高强度和高安全性',
      downloads: 234,
      uploadTime: '2024-01-05',
      lastUpdate: '2024-01-25',
      downloadUrl: 'https://example.com/rescue1.zip',
      docUrl: 'https://example.com/rescue1.pdf'
    }
  ]
  
  totalPackages.value = packages.value.length
  availablePackages.value = packages.value.filter(p => p.status === 'available').length
  maintenancePackages.value = packages.value.filter(p => p.status === 'maintenance').length
  
  // 更新筛选标签数量
  filterTabs.value[0].count = totalPackages.value
  filterTabs.value[1].count = availablePackages.value
  filterTabs.value[2].count = maintenancePackages.value
  filterTabs.value[3].count = packages.value.filter(p => p.status === 'borrowed').length
})
</script>

<style scoped>
.package-manage-mobile {
  padding: 16px;
  min-height: 100vh;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

/* 页面头部 */
.page-header {
  margin-bottom: 24px;
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-icon {
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 6px 24px rgba(64, 158, 255, 0.3);
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 统计卡片 */
.stats-section {
  margin-bottom: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  gap: 12px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-medium);
}

.stat-icon {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.stat-content {
  flex: 1;
  min-width: 0;
}

.stat-number {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}

/* 搜索和筛选 */
.search-section {
  margin-bottom: 24px;
}

.search-box {
  margin-bottom: 16px;
}

.search-box .el-input {
  border-radius: 12px;
}

.filter-tabs {
  display: flex;
  gap: 8px;
  overflow-x: auto;
  padding-bottom: 4px;
}

.filter-tab {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
  flex-shrink: 0;
}

.filter-tab:hover {
  background: var(--bg-primary);
  border-color: var(--brand-color);
}

.filter-tab.active {
  background: var(--brand-color);
  border-color: var(--brand-color);
  color: white;
}

.tab-icon {
  font-size: 14px;
}

.tab-label {
  font-size: 12px;
  font-weight: 500;
}

.tab-count {
  font-size: 10px;
  background: rgba(255, 255, 255, 0.2);
  padding: 2px 6px;
  border-radius: 10px;
  min-width: 16px;
  text-align: center;
}

/* 绳包列表 */
.packages-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 24px;
}

.package-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
}

.package-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-medium);
}

.package-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.package-icon {
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.package-status {
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
}

.package-status.available {
  background: linear-gradient(135deg, #67c23a 0%, #95d475 100%);
  color: white;
}

.package-status.maintenance {
  background: linear-gradient(135deg, #e6a23c 0%, #f0c78a 100%);
  color: white;
}

.package-status.borrowed {
  background: linear-gradient(135deg, #f56c6c 0%, #f78989 100%);
  color: white;
}

.package-info {
  margin-bottom: 16px;
}

.package-title {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.package-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.package-badge {
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 10px;
  font-weight: 500;
}

.package-badge.rope {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  color: white;
}

.package-badge.equipment {
  background: linear-gradient(135deg, #909399 0%, #c8c9cc 100%);
  color: white;
}

.package-details {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.detail-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.detail-item .el-icon {
  font-size: 12px;
  color: var(--brand-color);
}

.package-desc {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
  margin-bottom: 12px;
}

.package-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.meta-item {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
}

.meta-label {
  color: var(--text-secondary);
}

.meta-value {
  color: var(--text-primary);
  font-weight: 500;
}

.package-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

/* 加载更多 */
.load-more {
  display: flex;
  justify-content: center;
  margin-bottom: 24px;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.empty-icon {
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.empty-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

/* 详情对话框 */
.package-detail {
  padding: 20px 0;
}

.detail-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.detail-header h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.detail-status {
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.detail-section h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
}

.detail-item .label {
  color: var(--text-secondary);
}

.detail-item .value {
  color: var(--text-primary);
  font-weight: 500;
}

.download-links {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.download-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.link-label {
  font-size: 14px;
  color: var(--text-secondary);
  min-width: 60px;
}

.download-link {
  color: var(--brand-color);
  text-decoration: none;
  font-size: 14px;
}

.download-link:hover {
  text-decoration: underline;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .package-actions {
    justify-content: center;
  }
  
  .detail-grid {
    grid-template-columns: 1fr;
  }
}

/* 深色模式适配 */
.dark .package-manage-mobile {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .page-header,
.dark .stat-card,
.dark .package-card,
.dark .filter-tab {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

.dark .filter-tab.active {
  background: var(--brand-color);
  border-color: var(--brand-color);
}

/* 主题适配 */
.blue .stat-card::before,
.blue .package-card::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-card::before,
.green .package-card::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.orange .stat-card::before,
.orange .package-card::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

.purple .stat-card::before,
.purple .package-card::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.blue .stat-icon,
.blue .header-icon,
.blue .package-icon {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-icon,
.green .header-icon,
.green .package-icon {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.purple .stat-icon,
.purple .header-icon,
.purple .package-icon {
  background: linear-gradient(135deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.orange .stat-icon,
.orange .header-icon,
.orange .package-icon {
  background: linear-gradient(135deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
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

.stats-grid {
  animation: slide-up 0.6s ease-out 0.2s both;
}

.filter-tabs {
  animation: slide-up 0.6s ease-out 0.4s both;
}

.package-list {
  animation: slide-up 0.6s ease-out 0.6s both;
}

/* 卡片悬停动画 */
.stat-card,
.package-card {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.stat-card::before,
.package-card::before {
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

.stat-card::after,
.package-card::after {
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

.stat-card:hover,
.package-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-medium);
}

.stat-card:hover::before,
.package-card:hover::before {
  transform: scaleX(1);
}

.stat-card:hover::after,
.package-card:hover::after {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

/* 图标动画 */
.stat-icon,
.package-icon {
  transition: transform 0.3s ease;
}

.stat-card:hover .stat-icon,
.package-card:hover .package-icon {
  transform: scale(1.1);
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

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .page-header,
  .stats-grid,
  .filter-tabs,
  .package-list {
    animation: none;
  }
  
  .stat-card,
  .package-card {
    transition: none;
  }
  
  .stat-card:hover,
  .package-card:hover {
    transform: none;
  }
  
  .stat-icon,
  .package-icon {
    transition: none;
  }
  
  .el-button::before {
    display: none;
  }
}
</style> 