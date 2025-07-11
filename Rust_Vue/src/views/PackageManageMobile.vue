<template>
  <div class="package-manage-mobile">
    <!-- 页面标题 -->
    <div class="page-header">
      <h2 class="page-title">绳包管理</h2>
      <el-button type="primary" @click="openAddDialog" class="add-btn">
        <el-icon><Plus /></el-icon>
        新增
      </el-button>
    </div>

    <!-- 搜索栏 -->
    <div class="search-bar">
      <el-input
        v-model="searchKeyword"
        placeholder="搜索绳包..."
        clearable
        @input="handleSearch"
      >
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stat-item">
        <div class="stat-number">{{ totalPackages }}</div>
        <div class="stat-label">总绳包</div>
      </div>
      <div class="stat-item">
        <div class="stat-number">{{ totalDownloads }}</div>
        <div class="stat-label">总下载</div>
      </div>
      <div class="stat-item">
        <div class="stat-number">{{ authorCount }}</div>
        <div class="stat-label">作者数</div>
      </div>
    </div>

    <!-- 筛选栏 -->
    <div class="filter-bar">
      <el-select v-model="selectedAuthor" placeholder="选择作者" clearable @change="handleFilter">
        <el-option
          v-for="author in authorOptions"
          :key="author"
          :label="author"
          :value="author"
        />
      </el-select>
      <el-select v-model="sortBy" placeholder="排序" @change="handleFilter">
        <el-option label="最新" value="id" />
        <el-option label="热门" value="downloads" />
        <el-option label="名称" value="name" />
      </el-select>
    </div>

    <!-- 绳包列表 -->
    <div v-if="isLoading" class="loading-container">
      <el-skeleton :rows="3" animated />
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
    
    <div v-else class="package-list">
      <div
        v-for="pkg in paginatedPackages"
        :key="pkg.id"
        class="package-card"
        @click="viewDetails(pkg)"
      >
        <div class="card-header">
          <div class="package-info">
            <h3 class="package-name">{{ pkg.绳包名称 }}</h3>
            <p class="package-author">{{ pkg.作者 }}</p>
          </div>
          <div class="package-stats">
            <el-tag :type="getDownloadTagType(pkg.下载次数)" size="small">
              {{ pkg.下载次数 }} 下载
            </el-tag>
          </div>
        </div>
        
        <div class="card-content">
          <p class="package-desc">{{ pkg.简介 }}</p>
          <div class="package-meta">
            <span class="version">v{{ pkg.版本 }}</span>
            <span class="upload-time">{{ pkg.上架时间 }}</span>
          </div>
        </div>
        
        <div class="card-actions">
          <el-button size="small" @click.stop="openEditDialog(pkg)">
            <el-icon><Edit /></el-icon>
            编辑
          </el-button>
          <el-button size="small" type="danger" @click.stop="handleDelete(pkg)">
            <el-icon><Delete /></el-icon>
            删除
          </el-button>
        </div>
      </div>
    </div>

    <!-- 分页 -->
    <div v-if="filteredPackages.length > 0" class="pagination-container">
      <el-pagination
        v-model:current-page="currentPage"
        :page-size="pageSize"
        :total="filteredPackages.length"
        layout="prev, pager, next"
        :pager-count="5"
        @current-change="handleCurrentChange"
      />
    </div>

    <!-- 新增/编辑弹窗 -->
    <el-dialog
      v-model="showDialog"
      :title="isEdit ? '编辑绳包' : '新增绳包'"
      width="90%"
      :close-on-click-modal="false"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="formRules"
        label-width="80px"
        class="package-form"
      >
        <el-form-item label="名称" prop="绳包名称">
          <el-input v-model="form.绳包名称" placeholder="请输入绳包名称" />
        </el-form-item>
        
        <el-form-item label="作者" prop="作者">
          <el-input v-model="form.作者" placeholder="请输入作者名称" />
        </el-form-item>
        
        <el-form-item label="版本" prop="版本">
          <el-input v-model="form.版本" placeholder="请输入版本号" />
        </el-form-item>
        
        <el-form-item label="下载次数" prop="下载次数">
          <el-input-number
            v-model="form.下载次数"
            :min="0"
            :max="999999"
            placeholder="下载次数"
            style="width: 100%"
          />
        </el-form-item>
        
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
      width="90%"
    >
      <div v-if="selectedPackage" class="package-detail">
        <div class="detail-section">
          <h3>{{ selectedPackage.绳包名称 }}</h3>
          <p class="author">作者: {{ selectedPackage.作者 }}</p>
          <p class="version">版本: v{{ selectedPackage.版本 }}</p>
        </div>
        
        <div class="detail-section">
          <h4>统计信息</h4>
          <p>下载量: {{ selectedPackage.下载次数 }}</p>
          <p>上架时间: {{ selectedPackage.上架时间 }}</p>
        </div>
        
        <div class="detail-section">
          <h4>简介</h4>
          <p>{{ selectedPackage.简介 }}</p>
        </div>
        
        <div class="detail-section">
          <h4>项目直链</h4>
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
  Edit,
  Delete,
  Box
} from '@element-plus/icons-vue'

const router = useRouter()
const packages = ref<any[]>([])
const isLoading = ref(false)
const submitLoading = ref(false)

// 搜索和筛选
const searchKeyword = ref('')
const selectedAuthor = ref('')
const sortBy = ref('id')

// 分页
const currentPage = ref(1)
const pageSize = ref(10)

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
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    filtered = filtered.filter(pkg => 
      pkg.绳包名称.toLowerCase().includes(keyword) ||
      pkg.作者.toLowerCase().includes(keyword) ||
      pkg.简介.toLowerCase().includes(keyword)
    )
  }

  // 作者筛选
  if (selectedAuthor.value) {
    filtered = filtered.filter(pkg => pkg.作者 === selectedAuthor.value)
  }

  // 排序
  filtered.sort((a, b) => {
    switch (sortBy.value) {
      case 'name':
        return a.绳包名称.localeCompare(b.绳包名称)
      case 'downloads':
        return b.下载次数 - a.下载次数
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

function handleFilter() {
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
.package-manage-mobile {
  padding: 1rem;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.page-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--text-primary);
}

.add-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.search-bar {
  margin-bottom: 1rem;
}

.stats-section {
  display: flex;
  justify-content: space-between;
  margin-bottom: 1rem;
  gap: 0.5rem;
}

.stat-item {
  flex: 1;
  text-align: center;
  padding: 1rem;
  background: var(--bg-card);
  border-radius: 0.75rem;
  border: 1px solid var(--border-color);
}

.stat-number {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--brand-color);
  margin-bottom: 0.25rem;
}

.stat-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.filter-bar {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.filter-bar .el-select {
  flex: 1;
}

.loading-container {
  padding: 2rem 1rem;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 1rem;
  text-align: center;
}

.package-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.package-card {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 0.75rem;
  padding: 1rem;
  transition: all 0.2s ease;
  touch-action: manipulation;
  cursor: pointer;
}

.package-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.package-card:active {
  transform: translateY(0);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.08);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 0.75rem;
}

.package-info {
  flex: 1;
}

.package-name {
  margin: 0 0 0.25rem 0;
  font-size: 1rem;
  font-weight: bold;
  color: var(--text-primary);
  line-height: 1.3;
}

.package-author {
  margin: 0;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.package-stats {
  flex-shrink: 0;
}

.card-content {
  margin-bottom: 1rem;
}

.package-desc {
  margin: 0 0 0.75rem 0;
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.package-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.version {
  font-weight: 500;
}

.upload-time {
  opacity: 0.8;
}

.card-actions {
  display: flex;
  gap: 0.5rem;
}

.card-actions .el-button {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.25rem;
  font-size: 0.875rem;
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

.detail-section {
  margin-bottom: 1.5rem;
}

.detail-section h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.25rem;
  color: var(--text-primary);
}

.detail-section h4 {
  margin: 0 0 0.5rem 0;
  font-size: 1rem;
  color: var(--text-primary);
}

.detail-section p {
  margin: 0.25rem 0;
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.4;
}

.author {
  font-weight: 500;
}

.version {
  font-weight: 500;
}

.package-link {
  color: var(--brand-color);
  text-decoration: none;
  word-break: break-all;
}

.package-link:hover {
  text-decoration: underline;
}

/* 超小屏幕适配 */
@media (max-width: 480px) {
  .package-manage-mobile {
    padding: 0.5rem;
  }
  
  .page-title {
    font-size: 1.25rem;
  }
  
  .stats-section {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .stat-item {
    padding: 0.75rem;
  }
  
  .stat-number {
    font-size: 1.25rem;
  }
  
  .filter-bar {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .package-card {
    padding: 0.75rem;
  }
  
  .package-name {
    font-size: 0.875rem;
  }
  
  .package-desc {
    font-size: 0.8rem;
  }
  
  .package-meta {
    font-size: 0.7rem;
  }
  
  .card-actions .el-button {
    font-size: 0.8rem;
    padding: 0.5rem;
  }
}
</style> 