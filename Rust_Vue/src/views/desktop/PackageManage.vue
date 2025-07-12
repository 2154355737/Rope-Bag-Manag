<template>
  <div class="package-manage-desktop">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><Box /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">绳包管理</h1>
            <p class="page-subtitle">管理绳包信息和状态</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button type="primary" @click="showAddPackageDialog">
            <el-icon><Plus /></el-icon>
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
            <el-icon :size="24"><Box /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ totalPackages }}</div>
            <div class="stat-label">总绳包数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Check /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ availablePackages }}</div>
            <div class="stat-label">可用绳包</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Upload /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ borrowedPackages }}</div>
            <div class="stat-label">借出绳包</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Setting /></el-icon>
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
      <div class="search-left">
        <el-input
          v-model="searchQuery"
          placeholder="搜索绳包..."
          prefix-icon="Search"
          clearable
          style="width: 300px"
          @input="handleSearch"
        />
        <el-select v-model="statusFilter" placeholder="状态筛选" clearable style="width: 150px">
          <el-option label="全部" value="" />
          <el-option label="可用" value="available" />
          <el-option label="借出" value="borrowed" />
          <el-option label="维护" value="maintenance" />
        </el-select>
        <el-select v-model="typeFilter" placeholder="类型筛选" clearable style="width: 150px">
          <el-option label="全部" value="" />
          <el-option label="绳索" value="rope" />
          <el-option label="装备" value="equipment" />
        </el-select>
      </div>
      <div class="search-right">
        <el-button @click="refreshData">
          <el-icon><Refresh /></el-icon>
          刷新
        </el-button>
      </div>
    </div>

    <!-- 绳包表格 -->
    <div class="table-section">
      <el-table
        :data="filteredPackages"
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
        v-loading="loading"
        border
        stripe
      >
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="名称" width="200" />
        <el-table-column prop="author" label="作者" width="120" />
        <el-table-column prop="version" label="版本" width="100" />
        <el-table-column prop="type" label="类型" width="100">
          <template #default="{ row }">
            <el-tag :type="row.type === 'rope' ? 'primary' : 'success'">
              {{ row.type === 'rope' ? '绳索' : '装备' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="downloads" label="下载次数" width="120" />
        <el-table-column prop="uploadTime" label="上传时间" width="150">
          <template #default="{ row }">
            {{ formatDate(row.uploadTime) }}
          </template>
        </el-table-column>
        <el-table-column prop="lastUpdate" label="最后更新" width="150">
          <template #default="{ row }">
            {{ formatDate(row.lastUpdate) }}
          </template>
        </el-table-column>
        <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />
        <el-table-column label="操作" width="250" fixed="right">
          <template #default="{ row }">
            <el-button size="small" type="primary" @click="viewPackage(row)">
              <el-icon><View /></el-icon>
              查看
            </el-button>
            <el-button size="small" type="success" @click="downloadPackage(row)">
              <el-icon><Download /></el-icon>
              下载
            </el-button>
            <el-button size="small" type="warning" @click="editPackage(row)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button size="small" type="danger" @click="deletePackage(row)">
              <el-icon><Delete /></el-icon>
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 分页 -->
    <div class="pagination-section">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50, 100]"
        :total="totalPackages"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </div>

    <!-- 绳包详情对话框 -->
    <el-dialog
      v-model="detailDialogVisible"
      title="绳包详情"
      width="800px"
      :close-on-click-modal="false"
    >
      <div v-if="selectedPackage" class="package-detail">
        <div class="detail-header">
          <h3>{{ selectedPackage.name }}</h3>
          <el-tag :type="getStatusType(selectedPackage.status)">
            {{ getStatusText(selectedPackage.status) }}
          </el-tag>
        </div>
        
        <el-descriptions :column="2" border>
          <el-descriptions-item label="作者">{{ selectedPackage.author }}</el-descriptions-item>
          <el-descriptions-item label="版本">{{ selectedPackage.version }}</el-descriptions-item>
          <el-descriptions-item label="类型">
            <el-tag :type="selectedPackage.type === 'rope' ? 'primary' : 'success'">
              {{ selectedPackage.type === 'rope' ? '绳索' : '装备' }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="下载次数">{{ selectedPackage.downloads }}</el-descriptions-item>
          <el-descriptions-item label="上传时间">{{ formatDate(selectedPackage.uploadTime) }}</el-descriptions-item>
          <el-descriptions-item label="最后更新">{{ formatDate(selectedPackage.lastUpdate) }}</el-descriptions-item>
          <el-descriptions-item label="描述" :span="2">{{ selectedPackage.description }}</el-descriptions-item>
        </el-descriptions>
        
        <div class="detail-actions">
          <el-button type="primary" @click="downloadPackage(selectedPackage)">
            <el-icon><Download /></el-icon>
            下载绳包
          </el-button>
        </div>
      </div>
      <template #footer>
        <el-button @click="detailDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- 添加绳包对话框 -->
    <el-dialog
      v-model="addDialogVisible"
      title="添加绳包"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form :model="newPackage" label-width="100px">
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
        <el-form-item label="状态">
          <el-select v-model="newPackage.status" placeholder="选择状态">
            <el-option label="可用" value="available" />
            <el-option label="维护中" value="maintenance" />
            <el-option label="已借出" value="borrowed" />
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
  Plus, 
  Edit, 
  Delete, 
  View,
  Download,
  Upload,
  Setting,
  Check,
  Search,
  Refresh
} from '@element-plus/icons-vue'

// 响应式数据
const searchQuery = ref('')
const statusFilter = ref('')
const typeFilter = ref('')
const loading = ref(false)
const currentPage = ref(1)
const pageSize = ref(20)
const detailDialogVisible = ref(false)
const addDialogVisible = ref(false)
const selectedPackage = ref<any>(null)

// 绳包数据
const packages = ref<any[]>([])
const totalPackages = ref(0)
const availablePackages = ref(0)
const borrowedPackages = ref(0)
const maintenancePackages = ref(0)

// 新绳包表单
const newPackage = ref({
  name: '',
  author: '',
  version: '',
  type: 'rope',
  status: 'available',
  description: ''
})

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
  if (statusFilter.value) {
    filtered = filtered.filter(pkg => pkg.status === statusFilter.value)
  }
  
  // 类型过滤
  if (typeFilter.value) {
    filtered = filtered.filter(pkg => pkg.type === typeFilter.value)
  }
  
  return filtered
})

// 方法
function handleSearch() {
  // 搜索逻辑
}

function refreshData() {
  loading.value = true
  setTimeout(() => {
    loading.value = false
    ElMessage.success('数据已刷新')
  }, 1000)
}

function showAddPackageDialog() {
  addDialogVisible.value = true
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

function addPackage() {
  // 添加绳包逻辑
  ElMessage.success('绳包添加成功')
  addDialogVisible.value = false
}

function getStatusText(status: string) {
  const statusMap: Record<string, string> = {
    available: '可用',
    maintenance: '维护中',
    borrowed: '已借出'
  }
  return statusMap[status] || status
}

function getStatusType(status: string) {
  const typeMap: Record<string, string> = {
    available: 'success',
    maintenance: 'warning',
    borrowed: 'danger'
  }
  return typeMap[status] || 'info'
}

function handleSizeChange(size: number) {
  pageSize.value = size
  currentPage.value = 1
}

function handleCurrentChange(page: number) {
  currentPage.value = page
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
      lastUpdate: '2024-01-15'
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
      lastUpdate: '2024-01-20'
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
      lastUpdate: '2024-01-25'
    },
    {
      id: 4,
      name: '多功能工具包',
      author: '赵六',
      version: '1.2.1',
      type: 'equipment',
      status: 'available',
      description: '包含各种攀岩工具的完整套装，适合各种攀岩场景',
      downloads: 567,
      uploadTime: '2024-02-01',
      lastUpdate: '2024-02-10'
    }
  ]
  
  totalPackages.value = packages.value.length
  availablePackages.value = packages.value.filter(p => p.status === 'available').length
  borrowedPackages.value = packages.value.filter(p => p.status === 'borrowed').length
  maintenancePackages.value = packages.value.filter(p => p.status === 'maintenance').length
})
</script>

<style scoped>
.package-manage-desktop {
  padding: 24px;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  min-height: 100vh;
}

/* 页面头部 */
.page-header {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 32px;
  margin-bottom: 32px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  position: relative;
  overflow: hidden;
}

.page-header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(64, 158, 255, 0.1) 0%, rgba(103, 194, 58, 0.1) 100%);
  z-index: 0;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: relative;
  z-index: 1;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon {
  width: 64px;
  height: 64px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 8px 32px rgba(64, 158, 255, 0.3);
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.page-subtitle {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

/* 统计卡片 */
.stats-section {
  margin-bottom: 32px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 20px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-medium);
}

.stat-icon {
  width: 56px;
  height: 56px;
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
  border-radius: 14px;
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
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 14px;
  color: var(--text-secondary);
  margin-top: 8px;
}

/* 搜索和筛选 */
.search-section {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.search-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.search-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

/* 表格区域 */
.table-section {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
  overflow: hidden;
}

/* 分页区域 */
.pagination-section {
  display: flex;
  justify-content: center;
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-color);
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
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.detail-actions {
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .package-manage-desktop {
    padding: 16px;
  }
  
  .page-header {
    padding: 24px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .header-left {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .header-actions {
    margin-top: 16px;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .search-section {
    flex-direction: column;
    gap: 16px;
  }
  
  .search-left {
    flex-wrap: wrap;
    gap: 12px;
  }
  
  .search-left .el-input,
  .search-left .el-select {
    width: 100% !important;
  }
}

/* 深色模式适配 */
.dark .package-manage-desktop {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.dark .page-header,
.dark .stat-card,
.dark .search-section,
.dark .table-section,
.dark .pagination-section {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
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
.blue .header-icon {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

.green .stat-icon,
.green .header-icon {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

.purple .stat-icon,
.purple .header-icon {
  background: linear-gradient(135deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

.orange .stat-icon,
.orange .header-icon {
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

.search-section {
  animation: slide-up 0.6s ease-out 0.4s both;
}

.table-section {
  animation: slide-up 0.6s ease-out 0.6s both;
}

.pagination-section {
  animation: slide-up 0.6s ease-out 0.8s both;
}

/* 卡片悬停动画 */
.stat-card {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.stat-card::before {
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

.stat-card::after {
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

.stat-card:hover::before {
  transform: scaleX(1);
}

.stat-card:hover::after {
  transform: translateX(100%) translateY(100%) rotate(45deg);
}

/* 图标动画 */
.stat-icon {
  transition: transform 0.3s ease;
}

.stat-card:hover .stat-icon {
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

/* 表格行悬停动画 */
.el-table__row {
  transition: all 0.3s ease;
}

.el-table__row:hover {
  background-color: var(--bg-secondary) !important;
  transform: translateX(4px);
}

/* 动画优化 */
@media (prefers-reduced-motion: reduce) {
  .page-header,
  .stats-grid,
  .search-section,
  .table-section,
  .pagination-section {
    animation: none;
  }
  
  .stat-card {
    transition: none;
  }
  
  .stat-card:hover {
    transform: none;
  }
  
  .stat-icon {
    transition: none;
  }
  
  .el-button::before {
    display: none;
  }
  
  .el-table__row {
    transition: none;
  }
  
  .el-table__row:hover {
    transform: none;
  }
}
</style> 