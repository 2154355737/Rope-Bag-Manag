<template>
  <div class="package-manage">
    <!-- 页面标题和操作栏 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">绳包管理</h2>
        <p class="page-subtitle">管理系统中的所有绳包资源</p>
      </div>
      <div class="header-right">
        <el-button type="primary" @click="openAddDialog" class="add-btn">
          <el-icon><Plus /></el-icon>
          <span class="desktop-only">新增绳包</span>
          <span class="mobile-only">新增</span>
        </el-button>
      </div>
    </div>

    <!-- 搜索和筛选区域 -->
    <el-card class="search-card">
      <div class="search-form">
        <div class="search-inputs">
          <el-input
            v-model="searchForm.keyword"
            placeholder="搜索绳包名称、作者或简介..."
            clearable
            @input="handleSearch"
            class="search-input"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          
          <el-select
            v-model="searchForm.author"
            placeholder="选择作者"
            clearable
            @change="handleSearch"
            class="filter-select"
          >
            <el-option
              v-for="author in authorOptions"
              :key="author"
              :label="author"
              :value="author"
            />
          </el-select>
          
          <el-select
            v-model="searchForm.sortBy"
            placeholder="排序方式"
            @change="handleSearch"
            class="filter-select"
          >
            <el-option label="按ID排序" value="id" />
            <el-option label="按名称排序" value="name" />
            <el-option label="按下载量排序" value="downloads" />
            <el-option label="按上架时间排序" value="upload_time" />
          </el-select>
        </div>
        
        <div class="search-actions">
          <el-button @click="resetSearch" class="reset-btn">
            <el-icon><Refresh /></el-icon>
            <span class="desktop-only">重置</span>
          </el-button>
        </div>
      </div>
    </el-card>

    <!-- 统计信息 -->
    <div class="stats-cards">
      <el-card class="stat-card">
        <div class="stat-content">
          <div class="stat-icon total-icon">
            <el-icon :size="24"><Box /></el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-label">总绳包数</div>
            <div class="stat-value">{{ totalPackages }}</div>
          </div>
        </div>
      </el-card>
      
      <el-card class="stat-card">
        <div class="stat-content">
          <div class="stat-icon downloads-icon">
            <el-icon :size="24"><Download /></el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-label">总下载量</div>
            <div class="stat-value">{{ totalDownloads }}</div>
          </div>
        </div>
      </el-card>
      
      <el-card class="stat-card">
        <div class="stat-content">
          <div class="stat-icon authors-icon">
            <el-icon :size="24"><User /></el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-label">作者数量</div>
            <div class="stat-value">{{ authorCount }}</div>
          </div>
        </div>
      </el-card>
    </div>

    <!-- 绳包列表 -->
    <el-card class="list-card">
      <template #header>
        <div class="card-header">
          <span>绳包列表</span>
          <div class="header-actions">
            <el-button size="small" @click="refreshData" :loading="isLoading">
              <el-icon><Refresh /></el-icon>
              <span class="desktop-only">刷新</span>
            </el-button>
          </div>
        </div>
      </template>
      
      <div v-if="isLoading" class="loading-container">
        <el-skeleton :rows="5" animated />
      </div>
      
      <div v-else-if="filteredPackages.length === 0" class="empty-state">
        <el-icon :size="64" style="color: #c0c4cc; margin-bottom: 16px;">
          <Box />
        </el-icon>
        <p style="color: #909399; margin: 0;">暂无绳包数据</p>
        <el-button type="primary" @click="openAddDialog" style="margin-top: 16px;">
          添加第一个绳包
        </el-button>
      </div>
      
      <el-table
        v-else
        :data="paginatedPackages"
        style="width: 100%"
        :row-class-name="getRowClassName"
        @row-click="handleRowClick"
      >
        <el-table-column prop="id" label="ID" width="80" sortable />
        <el-table-column prop="绳包名称" label="名称" min-width="160" show-overflow-tooltip />
        <el-table-column prop="作者" label="作者" width="120" />
        <el-table-column prop="版本" label="版本" width="100" />
        <el-table-column prop="下载次数" label="下载量" width="100" sortable>
          <template #default="scope">
            <el-tag :type="getDownloadTagType(scope.row.下载次数)" size="small">
              {{ scope.row.下载次数 }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="上架时间" label="上架时间" width="120" />
        <el-table-column prop="简介" label="简介" min-width="200" show-overflow-tooltip />
        <el-table-column label="操作" width="260" fixed="right">
          <template #default="scope">
            <div class="table-action-buttons">
              <el-button size="small" @click.stop="openEditDialog(scope.row)">
                <el-icon><Edit /></el-icon>
                <span class="desktop-only">编辑</span>
              </el-button>
              <el-button size="small" type="danger" @click.stop="handleDelete(scope.row)">
                <el-icon><Delete /></el-icon>
                <span class="desktop-only">删除</span>
              </el-button>
              <el-button size="small" type="info" @click.stop="viewDetails(scope.row)">
                <el-icon><View /></el-icon>
                <span class="desktop-only">详情</span>
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
      
      <!-- 分页 -->
      <div v-if="filteredPackages.length > 0" class="pagination-container">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="filteredPackages.length"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog
      v-model="showDialog"
      :title="isEdit ? '编辑绳包' : '新增绳包'"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="formRules"
        label-width="100px"
        class="package-form"
      >
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="绳包名称" prop="绳包名称">
              <el-input v-model="form.绳包名称" placeholder="请输入绳包名称" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="作者" prop="作者">
              <el-input v-model="form.作者" placeholder="请输入作者名称" />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="版本" prop="版本">
              <el-input v-model="form.版本" placeholder="请输入版本号" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="下载次数" prop="下载次数">
              <el-input-number
                v-model="form.下载次数"
                :min="0"
                :max="999999"
                placeholder="下载次数"
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-form-item label="项目直链" prop="项目直链">
          <el-input v-model="form.项目直链" placeholder="请输入项目直链地址" />
        </el-form-item>
        
        <el-form-item label="简介" prop="简介">
          <el-input
            v-model="form.简介"
            type="textarea"
            :rows="4"
            placeholder="请输入绳包简介"
            maxlength="500"
            show-word-limit
          />
        </el-form-item>
      </el-form>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="closeDialog">取消</el-button>
          <el-button type="primary" @click="submitForm" :loading="submitLoading">
            {{ isEdit ? '更新' : '创建' }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 详情弹窗 -->
    <el-dialog
      v-model="showDetailDialog"
      title="绳包详情"
      width="500px"
    >
      <div v-if="selectedPackage" class="package-detail">
        <div class="detail-item">
          <label>ID:</label>
          <span>{{ selectedPackage.id }}</span>
        </div>
        <div class="detail-item">
          <label>名称:</label>
          <span>{{ selectedPackage.绳包名称 }}</span>
        </div>
        <div class="detail-item">
          <label>作者:</label>
          <span>{{ selectedPackage.作者 }}</span>
        </div>
        <div class="detail-item">
          <label>版本:</label>
          <span>{{ selectedPackage.版本 }}</span>
        </div>
        <div class="detail-item">
          <label>下载量:</label>
          <span>{{ selectedPackage.下载次数 }}</span>
        </div>
        <div class="detail-item">
          <label>上架时间:</label>
          <span>{{ selectedPackage.上架时间 }}</span>
        </div>
        <div class="detail-item">
          <label>简介:</label>
          <span>{{ selectedPackage.简介 }}</span>
        </div>
        <div class="detail-item">
          <label>项目直链:</label>
          <a :href="selectedPackage.项目直链" target="_blank" class="package-link">
            {{ selectedPackage.项目直链 }}
          </a>
        </div>
      </div>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showDetailDialog = false">关闭</el-button>
          <el-button type="primary" @click="editFromDetail">编辑</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import { ElMessage, ElMessageBox, ElForm } from 'element-plus'
import { getPackages, addPackage, updatePackage, deletePackage } from '../api'
import { useRouter } from 'vue-router'
import {
  Plus,
  Search,
  Refresh,
  Edit,
  Delete,
  View,
  Box,
  Download,
  User
} from '@element-plus/icons-vue'

const router = useRouter()
const packages = ref<any[]>([])
const isLoading = ref(false)
const submitLoading = ref(false)

// 搜索和筛选
const searchForm = ref({
  keyword: '',
  author: '',
  sortBy: 'id'
})

// 分页
const currentPage = ref(1)
const pageSize = ref(20)

// 弹窗状态
const showDialog = ref(false)
const showDetailDialog = ref(false)
const isEdit = ref(false)
const form = ref<any>({})
const editId = ref<number|null>(null)
const selectedPackage = ref<any>(null)
const formRef = ref<InstanceType<typeof ElForm>>()

// 表单验证规则
const formRules = {
  绳包名称: [
    { required: true, message: '请输入绳包名称', trigger: 'blur' },
    { min: 1, max: 50, message: '名称长度在 1 到 50 个字符', trigger: 'blur' }
  ],
  作者: [
    { required: true, message: '请输入作者名称', trigger: 'blur' },
    { min: 1, max: 30, message: '作者名称长度在 1 到 30 个字符', trigger: 'blur' }
  ],
  版本: [
    { required: true, message: '请输入版本号', trigger: 'blur' },
    { pattern: /^[\d.]+$/, message: '版本号格式不正确', trigger: 'blur' }
  ],
  简介: [
    { required: true, message: '请输入绳包简介', trigger: 'blur' },
    { min: 1, max: 500, message: '简介长度在 1 到 500 个字符', trigger: 'blur' }
  ],
  项目直链: [
    { required: true, message: '请输入项目直链', trigger: 'blur' },
    { pattern: /^https?:\/\/.+/, message: '请输入正确的URL地址', trigger: 'blur' }
  ]
}

// 计算属性
const authorOptions = computed(() => {
  const authors = new Set(packages.value.map(pkg => pkg.作者))
  return Array.from(authors).sort()
})

const filteredPackages = computed(() => {
  let filtered = packages.value

  // 关键词搜索
  if (searchForm.value.keyword) {
    const keyword = searchForm.value.keyword.toLowerCase()
    filtered = filtered.filter(pkg => 
      pkg.绳包名称.toLowerCase().includes(keyword) ||
      pkg.作者.toLowerCase().includes(keyword) ||
      pkg.简介.toLowerCase().includes(keyword)
    )
  }

  // 作者筛选
  if (searchForm.value.author) {
    filtered = filtered.filter(pkg => pkg.作者 === searchForm.value.author)
  }

  // 排序
  filtered.sort((a, b) => {
    switch (searchForm.value.sortBy) {
      case 'name':
        return a.绳包名称.localeCompare(b.绳包名称)
      case 'downloads':
        return b.下载次数 - a.下载次数
      case 'upload_time':
        return new Date(b.上架时间).getTime() - new Date(a.上架时间).getTime()
      default:
        return a.id - b.id
    }
  })

  return filtered
})

const paginatedPackages = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredPackages.value.slice(start, end)
})

const totalPackages = computed(() => packages.value.length)
const totalDownloads = computed(() => packages.value.reduce((sum, pkg) => sum + (pkg.下载次数 || 0), 0))
const authorCount = computed(() => authorOptions.value.length)

// 方法
onMounted(async () => {
  await loadPackages()
})

async function loadPackages() {
  try {
    isLoading.value = true
    const res = await getPackages()
    if (res.code === 0 && res.data) {
      packages.value = res.data.绳包列表 || []
    }
  } catch (error) {
    console.error('加载绳包数据失败:', error)
    
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED') ||
        errorMessage.includes('Service unavailable')) {
      
      ElMessage.error('服务异常已安全退出！')
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      return
    }
    
    ElMessage.error('绳包数据加载失败，请稍后重试')
  } finally {
    isLoading.value = false
  }
}

function handleSearch() {
  currentPage.value = 1
}

function resetSearch() {
  searchForm.value = {
    keyword: '',
    author: '',
    sortBy: 'id'
  }
  currentPage.value = 1
}

function refreshData() {
  loadPackages()
}

function handleSizeChange(size: number) {
  pageSize.value = size
  currentPage.value = 1
}

function handleCurrentChange(page: number) {
  currentPage.value = page
}

function getDownloadTagType(downloads: number) {
  if (downloads >= 1000) return 'danger'
  if (downloads >= 500) return 'warning'
  if (downloads >= 100) return 'success'
  return 'info'
}

function getRowClassName({ row }: { row: any }) {
  return row.下载次数 >= 1000 ? 'high-download-row' : ''
}

function handleRowClick(row: any) {
  viewDetails(row)
}

function openAddDialog() {
  isEdit.value = false
  form.value = {
    绳包名称: '',
    作者: '',
    版本: '',
    简介: '',
    项目直链: '',
    下载次数: 0
  }
  showDialog.value = true
  editId.value = null
}

function openEditDialog(pkg: any) {
  isEdit.value = true
  form.value = { ...pkg }
  showDialog.value = true
  editId.value = pkg.id
}

function closeDialog() {
  showDialog.value = false
  nextTick(() => {
    formRef.value?.clearValidate()
  })
}

function viewDetails(pkg: any) {
  selectedPackage.value = pkg
  showDetailDialog.value = true
}

function editFromDetail() {
  showDetailDialog.value = false
  openEditDialog(selectedPackage.value)
}

async function submitForm() {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    submitLoading.value = true
    
    if (isEdit.value) {
      const res = await updatePackage({ id: editId.value, ...form.value })
      if (res.code === 0) {
        ElMessage.success('编辑成功')
        await loadPackages()
        showDialog.value = false
      } else {
        ElMessage.error(res.msg || '操作失败')
      }
    } else {
      const res = await addPackage(form.value)
      if (res.code === 0) {
        ElMessage.success('新增成功')
        await loadPackages()
        showDialog.value = false
      } else {
        ElMessage.error(res.msg || '操作失败')
      }
    }
  } catch (error) {
    console.error('提交表单失败:', error)
    
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED') ||
        errorMessage.includes('Service unavailable')) {
      
      ElMessage.error('服务异常已安全退出！')
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      return
    }
    
    ElMessage.error('操作失败，请稍后重试')
  } finally {
    submitLoading.value = false
  }
}

async function handleDelete(pkg: any) {
  try {
    await ElMessageBox.confirm(
      `确定要删除绳包 "${pkg.绳包名称}" 吗？此操作不可恢复。`,
      '确认删除',
      {
        confirmButtonText: '确定删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    const res = await deletePackage(pkg.id)
    if (res.code === 0) {
      ElMessage.success('删除成功')
      await loadPackages()
    } else {
      ElMessage.error(res.msg || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除绳包失败:', error)
      
      const errorMessage = error instanceof Error ? error.message : String(error)
      if (errorMessage.includes('fetch') || 
          errorMessage.includes('network') || 
          errorMessage.includes('Failed to fetch') ||
          errorMessage.includes('ERR_NETWORK') ||
          errorMessage.includes('ERR_CONNECTION_REFUSED') ||
          errorMessage.includes('Service unavailable')) {
        
        ElMessage.error('服务异常已安全退出！')
        setTimeout(() => {
          router.push('/login')
        }, 2000)
        return
      }
      
      ElMessage.error('删除失败，请稍后重试')
    }
  }
}
</script>

<style scoped>
.package-manage {
  padding: 0;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.page-title {
  margin: 0 0 0.5rem 0;
  font-size: 1.75rem;
  font-weight: bold;
  color: var(--text-primary);
}

.page-subtitle {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.add-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.search-card {
  margin-bottom: 1.5rem;
}

.search-form {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.search-inputs {
  display: flex;
  gap: 1rem;
  flex: 1;
}

.search-input {
  flex: 1;
}

.filter-select {
  width: 150px;
}

.search-actions {
  display: flex;
  gap: 0.5rem;
}

.reset-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  border-radius: 0.75rem;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.stat-icon {
  width: 3rem;
  height: 3rem;
  border-radius: 0.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
}

.total-icon { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
.downloads-icon { background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); }
.authors-icon { background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); }

.stat-info {
  flex: 1;
}

.stat-label {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin-bottom: 0.25rem;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--text-primary);
}

.list-card {
  border-radius: 0.75rem;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: bold;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.loading-container {
  padding: 2rem;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.pagination-container {
  display: flex;
  justify-content: center;
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.package-form {
  padding: 1rem 0;
}

.dialog-footer {
  display: flex;
  justify-content: center;
  gap: 1rem;
}

.package-detail {
  padding: 1rem 0;
}

.detail-item {
  display: flex;
  margin-bottom: 1rem;
  padding: 0.75rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
}

.detail-item label {
  font-weight: bold;
  color: var(--text-primary);
  min-width: 100px;
  margin-right: 1rem;
}

.detail-item span {
  color: var(--text-secondary);
  flex: 1;
}

.package-link {
  color: var(--brand-color);
  text-decoration: none;
  word-break: break-all;
}

.package-link:hover {
  text-decoration: underline;
}

.high-download-row {
  background-color: rgba(103, 194, 58, 0.1) !important;
}

.table-action-buttons {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .package-manage {
    padding: 0.5rem;
  }
  
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 1rem;
  }
  
  .page-title {
    font-size: 1.5rem;
  }
  
  .search-form {
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .search-inputs {
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .search-input,
  .filter-select {
    width: 100%;
  }
  
  .stats-cards {
    grid-template-columns: 1fr;
    gap: 0.75rem;
  }
  
  .stat-content {
    gap: 0.75rem;
  }
  
  .stat-icon {
    width: 2.5rem;
    height: 2.5rem;
  }
  
  .stat-value {
    font-size: 1.25rem;
  }
  
  .card-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }
  
  .header-actions {
    width: 100%;
    justify-content: flex-end;
  }
  
  .empty-state {
    padding: 2rem 1rem;
  }
  
  .pagination-container {
    margin-top: 1rem;
  }

  .table-action-buttons {
    gap: 0.25rem;
  }
}

/* 桌面端和移动端显示控制 */
.desktop-only {
  display: inline;
}

.mobile-only {
  display: none;
}

@media (max-width: 768px) {
  .desktop-only {
    display: none !important;
  }
  
  .mobile-only {
    display: inline !important;
  }
}
</style> 