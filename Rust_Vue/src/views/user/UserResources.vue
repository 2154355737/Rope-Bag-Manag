<template>
  <div class="user-resource-manage">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <h2>我的资源</h2>
        <el-button type="primary" icon="Plus" @click="showUploadDialog = true">
          上传资源
        </el-button>
      </div>
    </div>

    <!-- 统计信息 -->
    <el-row :gutter="20" class="stats-row">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#409EFF"><Document /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ totalResources }}</div>
              <div class="stat-label">总资源数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#67C23A"><Select /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ publishedResources }}</div>
              <div class="stat-label">已发布</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#E6A23C"><Clock /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ pendingResources }}</div>
              <div class="stat-label">审核中</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon">
              <el-icon color="#F56C6C"><Download /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-number">{{ totalDownloads }}</div>
              <div class="stat-label">总下载量</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 筛选和搜索 -->
    <el-card shadow="hover" class="filter-card">
      <el-form :model="searchForm" :inline="true" class="search-form">
        <el-form-item label="状态筛选:">
          <el-select v-model="searchForm.status" placeholder="全部状态" clearable style="width: 120px">
            <el-option label="全部" value="" />
            <el-option label="已发布" value="Active" />
            <el-option label="审核中" value="Pending" />
            <el-option label="已拒绝" value="Rejected" />
            <el-option label="已下架" value="Inactive" />
          </el-select>
        </el-form-item>
        <el-form-item label="搜索:">
          <el-input 
            v-model="searchForm.search" 
            placeholder="输入资源名称" 
            clearable 
            style="width: 200px"
            @keyup.enter="loadResources"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" icon="Search" @click="loadResources">搜索</el-button>
          <el-button icon="Refresh" @click="resetSearch">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 资源列表 -->
    <el-card shadow="hover" class="resource-list-card">
      <div v-loading="loading">
        <div v-if="resourceList.length === 0 && !loading" class="empty-state">
          <el-empty description="还没有上传任何资源">
            <el-button type="primary" @click="showUploadDialog = true">
              立即上传
            </el-button>
          </el-empty>
        </div>
        <div v-else class="resource-grid">
          <div 
            v-for="resource in resourceList" 
            :key="resource.id"
            class="resource-card"
          >
            <div class="resource-header">
              <div class="resource-title" @click="$router.push(`/resource/${resource.id}`)">
                {{ resource.name }}
              </div>
              <el-dropdown @command="(command) => handleResourceAction(command, resource)">
                <el-button type="text" size="small">
                  <el-icon><MoreFilled /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item command="edit">编辑</el-dropdown-item>
                    <el-dropdown-item command="view">查看</el-dropdown-item>
                    <el-dropdown-item command="delete" divided>删除</el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>
            
            <div class="resource-info">
              <div class="info-item">
                <span class="label">作者:</span>
                <span class="value">{{ resource.author }}</span>
              </div>
              <div class="info-item">
                <span class="label">版本:</span>
                <span class="value">{{ resource.version || '未设置' }}</span>
              </div>
              <div class="info-item">
                <span class="label">状态:</span>
                <el-tag :type="getStatusType(resource.status)" size="small">
                  {{ getStatusText(resource.status) }}
                </el-tag>
              </div>
            </div>

            <div class="resource-description">
              {{ resource.description || '暂无描述' }}
            </div>

            <div class="resource-stats">
              <div class="stat-item">
                <el-icon><Download /></el-icon>
                <span>{{ resource.download_count }}</span>
              </div>
              <div class="stat-item">
                <el-icon><Star /></el-icon>
                <span>{{ resource.like_count || 0 }}</span>
              </div>
              <div class="stat-item">
                <el-icon><Clock /></el-icon>
                <span>{{ formatDate(resource.created_at) }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 分页 -->
        <div class="pagination-wrapper" v-if="totalResources > 0">
          <el-pagination
            v-model:current-page="pagination.page"
            v-model:page-size="pagination.pageSize"
            :page-sizes="[10, 20, 50]"
            :total="totalResources"
            layout="total, sizes, prev, pager, next, jumper"
            @size-change="loadResources"
            @current-change="loadResources"
          />
        </div>
      </div>
    </el-card>

    <!-- 上传资源对话框 -->
    <el-dialog v-model="showUploadDialog" title="上传新资源" width="600px" :close-on-click-modal="false">
      <el-form :model="uploadForm" :rules="uploadRules" ref="uploadFormRef" label-width="80px">
        <el-form-item label="资源名称" prop="name">
          <el-input v-model="uploadForm.name" placeholder="请输入资源名称" />
        </el-form-item>
                  <el-form-item label="作者" prop="author">
            <el-input v-model="uploadForm.author" placeholder="当前用户" disabled />
        </el-form-item>
        <el-form-item label="版本" prop="version">
          <el-input v-model="uploadForm.version" placeholder="请输入版本号（可选）" />
        </el-form-item>
        <el-form-item label="分类" prop="category_id">
          <el-select v-model="uploadForm.category_id" placeholder="请选择分类" style="width: 100%">
            <el-option 
              v-for="category in categories" 
              :key="category.id" 
              :label="category.name" 
              :value="category.id" 
            />
          </el-select>
        </el-form-item>
        <el-form-item label="资源描述" prop="description">
          <el-input 
            v-model="uploadForm.description" 
            type="textarea" 
            :rows="4" 
            placeholder="请输入资源描述"
          />
        </el-form-item>
        <el-form-item label="文件链接" prop="file_url">
          <el-input v-model="uploadForm.file_url" placeholder="请输入文件下载链接" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showUploadDialog = false">取消</el-button>
          <el-button type="primary" @click="handleUpload" :loading="uploading">
            提交上传
          </el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 编辑资源对话框 -->
    <el-dialog v-model="showEditDialog" title="编辑资源" width="600px" :close-on-click-modal="false">
      <el-form :model="editForm" :rules="uploadRules" ref="editFormRef" label-width="80px">
        <el-form-item label="资源名称" prop="name">
          <el-input v-model="editForm.name" placeholder="请输入资源名称" />
        </el-form-item>
        <el-form-item label="版本" prop="version">
          <el-input v-model="editForm.version" placeholder="请输入版本号（可选）" />
        </el-form-item>
        <el-form-item label="分类" prop="category_id">
          <el-select v-model="editForm.category_id" placeholder="请选择分类" style="width: 100%">
            <el-option 
              v-for="category in categories" 
              :key="category.id" 
              :label="category.name" 
              :value="category.id" 
            />
          </el-select>
        </el-form-item>
        <el-form-item label="资源描述" prop="description">
          <el-input 
            v-model="editForm.description" 
            type="textarea" 
            :rows="4" 
            placeholder="请输入资源描述"
          />
        </el-form-item>
        <el-form-item label="文件链接" prop="file_url">
          <el-input v-model="editForm.file_url" placeholder="请输入文件下载链接" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showEditDialog = false">取消</el-button>
          <el-button type="primary" @click="handleEdit" :loading="editing">
            保存修改
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox, ElForm } from 'element-plus'
import { 
  Document, Select, Clock, Download, Star, Plus, Search, Refresh, 
  MoreFilled 
} from '@element-plus/icons-vue'
import { packageApi, type Package, type CreatePackageRequest, type UpdatePackageRequest } from '@/api/packages'
import { categoryApi, userApi } from '@/api'
import { getUserInfo } from '@/utils/auth'

const uploadFormRef = ref<InstanceType<typeof ElForm> | null>(null)
const editFormRef = ref<InstanceType<typeof ElForm> | null>(null)

// 基础数据
const loading = ref(false)
const uploading = ref(false)
const editing = ref(false)
const showUploadDialog = ref(false)
const showEditDialog = ref(false)
const userInfo = getUserInfo()
const resourceList = ref<Package[]>([])
const categories = ref<any[]>([])
const currentEditResource = ref<Package | null>(null)

// 分页数据
const pagination = reactive({
  page: 1,
  pageSize: 12
})

// 搜索表单
const searchForm = reactive({
  status: '',
  search: ''
})

// 上传表单
const uploadForm = reactive<CreatePackageRequest>({
  name: '',
  author: userInfo?.username || '',
  version: '',
  description: '',
  category_id: undefined,
  file_url: ''
})

// 编辑表单
const editForm = reactive<UpdatePackageRequest>({
  name: '',
  version: '',
  description: '',
  category_id: undefined,
  file_url: ''
})

// 表单验证规则
const uploadRules = {
  name: [
    { required: true, message: '请输入资源名称', trigger: 'blur' },
    { min: 2, max: 50, message: '名称长度在 2 到 50 个字符之间', trigger: 'blur' }
  ],
  author: [
    { required: true, message: '请输入作者名称', trigger: 'blur' }
  ],
  description: [
    { max: 500, message: '描述不能超过 500 个字符', trigger: 'blur' }
  ],
  file_url: [
    { required: true, message: '请输入文件链接', trigger: 'blur' },
    { type: 'url', message: '请输入有效的URL地址', trigger: 'blur' }
  ]
}

// 计算属性
const totalResources = computed(() => resourceList.value.length)
const publishedResources = computed(() => resourceList.value.filter(r => r.status === 'Active').length)
const pendingResources = computed(() => resourceList.value.filter(r => r.status === 'Pending').length)
const totalDownloads = computed(() => resourceList.value.reduce((sum, r) => sum + r.download_count, 0))

// 加载资源列表
const loadResources = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.page,
      pageSize: pagination.pageSize,
      status: searchForm.status || undefined
    }
    console.log('📤 加载用户资源参数:', params)
    const res = await userApi.getMyResources(params)
    if (res.code === 0) {
      console.log('✅ 用户资源加载成功:', res.data)
      resourceList.value = res.data?.list || []
      pagination.total = res.data?.total || 0
    } else {
      console.warn('❌ 用户资源加载失败:', res)
      ElMessage.error(res.message || '加载资源失败')
    }
  } catch (error) {
    console.error('🚫 加载资源列表失败:', error)
    ElMessage.error('加载资源列表失败')
  } finally {
    loading.value = false
  }
}

// 删除重复的loadCategories函数定义

// 重置搜索
const resetSearch = () => {
  searchForm.status = ''
  searchForm.search = ''
  pagination.page = 1
  loadResources()
}

// 处理资源操作
const handleResourceAction = (command: string, resource: Package) => {
  switch (command) {
    case 'edit':
      openEditDialog(resource)
      break
    case 'view':
      window.open(`/resource/${resource.id}`, '_blank')
      break
    case 'delete':
      handleDelete(resource)
      break
  }
}

// 打开编辑对话框
const openEditDialog = (resource: Package) => {
  currentEditResource.value = resource
  editForm.name = resource.name
  editForm.version = resource.version || ''
  editForm.description = resource.description || ''
  editForm.category_id = resource.category_id
  editForm.file_url = resource.file_url
  showEditDialog.value = true
}

// 处理上传
const handleUpload = async () => {
  if (!uploadFormRef.value) return
  try {
    await uploadFormRef.value.validate()
    uploading.value = true
    // 普通用户使用专用的提交接口，自动设置作者为当前用户
const categoryName = uploadForm.category_id ? 
  categories.value.find(c => c.id === uploadForm.category_id)?.name : 
  undefined

const submitData = {
  title: uploadForm.name,
  description: uploadForm.description,
  category: categoryName,
  file_url: uploadForm.file_url || ''
}
console.log('📤 普通用户提交资源数据:', submitData)
const res = await packageApi.userSubmitResource(submitData)
    if (res.code === 0) {
      ElMessage.success('资源上传成功，等待审核')
      showUploadDialog.value = false
      resetUploadForm()
      loadResources()
    } else {
      ElMessage.error(res.message || '上传失败')
    }
  } catch (error) {
    console.error('上传资源失败:', error)
    ElMessage.error('上传失败')
  } finally {
    uploading.value = false
  }
}

// 处理编辑
const handleEdit = async () => {
  if (!editFormRef.value || !currentEditResource.value) return
  try {
    await editFormRef.value.validate()
    editing.value = true
    const res = await packageApi.updatePackage(currentEditResource.value.id, editForm)
    if (res.code === 0) {
      ElMessage.success('资源更新成功')
      showEditDialog.value = false
      loadResources()
    } else {
      ElMessage.error(res.message || '更新失败')
    }
  } catch (error) {
    console.error('更新资源失败:', error)
    ElMessage.error('更新失败')
  } finally {
    editing.value = false
  }
}

// 处理删除
const handleDelete = async (resource: Package) => {
  try {
    await ElMessageBox.confirm(
      `确定要删除资源 "${resource.name}" 吗？此操作不可恢复。`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    const res = await packageApi.deletePackage(resource.id)
    if (res.code === 0) {
      ElMessage.success('删除成功')
      loadResources()
    } else {
      ElMessage.error(res.message || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除资源失败:', error)
      ElMessage.error('删除失败')
    }
  }
}

// 重置上传表单
const resetUploadForm = () => {
  Object.assign(uploadForm, {
    name: '',
    author: userInfo?.username || '',
    version: '',
    description: '',
    category_id: undefined,
    file_url: ''
  })
}

// 获取状态类型
const getStatusType = (status: string) => {
  const statusMap: Record<string, string> = {
    'Active': 'success',
    'Pending': 'warning',
    'Rejected': 'danger',
    'Inactive': 'info'
  }
  return statusMap[status] || 'info'
}

// 获取状态文本
const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'Active': '已发布',
    'Pending': '审核中',
    'Rejected': '已拒绝',
    'Inactive': '已下架'
  }
  return statusMap[status] || status
}

// 格式化日期
const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('zh-CN')
}

// 加载分类数据
const loadCategories = async () => {
  try {
    console.log('正在加载分类数据...')
    const res = await categoryApi.getCategories()
    console.log('分类API响应:', res)
    if (res.code === 0 && res.data) {
      categories.value = Array.isArray(res.data) ? res.data : res.data.list || []
      console.log('加载到的分类:', categories.value)
    } else {
      console.error('分类API返回错误:', res)
    }
  } catch (error) {
    console.error('加载分类失败:', error)
  }
}

onMounted(() => {
  loadCategories()
  loadResources()
})
</script>

<style scoped>
.user-resource-manage {
  padding: 24px;
  background-color: #f5f7fa;
  min-height: calc(100vh - 60px);
}

.page-header {
  margin-bottom: 24px;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-content h2 {
  margin: 0;
  color: #303133;
  font-size: 28px;
  font-weight: 600;
}

.stats-row {
  margin-bottom: 24px;
}

.stat-card {
  border-radius: 8px;
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 8px;
  background-color: #f0f9ff;
}

.stat-icon .el-icon {
  font-size: 24px;
}

.stat-info {
  flex: 1;
}

.stat-number {
  font-size: 24px;
  font-weight: 600;
  color: #303133;
  line-height: 1;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: #909399;
}

.filter-card,
.resource-list-card {
  margin-bottom: 24px;
  border-radius: 8px;
}

.search-form {
  margin: 0;
}

.empty-state {
  padding: 60px 0;
  text-align: center;
}

.resource-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  margin-bottom: 24px;
}

.resource-card {
  padding: 20px;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  background: white;
  transition: all 0.3s;
}

.resource-card:hover {
  border-color: #409EFF;
  box-shadow: 0 2px 12px rgba(64, 158, 255, 0.1);
}

.resource-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.resource-title {
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  cursor: pointer;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.resource-title:hover {
  color: #409EFF;
}

.resource-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.label {
  color: #909399;
  min-width: 40px;
}

.value {
  color: #303133;
  flex: 1;
}

.resource-description {
  color: #606266;
  font-size: 14px;
  margin-bottom: 16px;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.resource-stats {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 12px;
  color: #909399;
  border-top: 1px solid #f0f0f0;
  padding-top: 12px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  padding: 24px 0;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

@media screen and (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }
  
  .resource-grid {
    grid-template-columns: 1fr;
  }
  
  .stats-row .el-col {
    margin-bottom: 12px;
  }
}
</style> 