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
            <h1 class="page-title">内容管理</h1>
            <p class="page-subtitle">管理绳包资源和社区内容</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button type="primary" @click="showAddPackageDialog">
            <el-icon><Plus /></el-icon>
            添加内容
          </el-button>
          <el-button @click="showCategoryDialog = true">
            <el-icon><Folder /></el-icon>
            分类管理
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
            <div class="stat-label">总资源数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Check /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ availablePackages }}</div>
            <div class="stat-label">已发布</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Clock /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ borrowedPackages }}</div>
            <div class="stat-label">待审核</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><User /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ maintenancePackages }}</div>
            <div class="stat-label">活跃用户</div>
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
          <el-option label="已发布" value="Active" />
          <el-option label="待审核" value="Pending" />
          <el-option label="已拒绝" value="Rejected" />
          <el-option label="已下架" value="Inactive" />
          <el-option label="已删除" value="Deleted" />
        </el-select>
        <el-select v-model="typeFilter" placeholder="分类筛选" clearable style="width: 150px">
          <el-option label="全部" value="" />
          <el-option 
            v-for="cat in categories" 
            :key="cat.id" 
            :label="cat.name" 
            :value="cat.name"
            :disabled="!cat.enabled"
          />
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
        <el-table-column prop="type" label="分类" width="100">
          <template #default="{ row }">
            <el-tag :type="getCategoryType(row.type)">
              {{ getCategoryText(row.type) }}
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
            <el-tag :type="getCategoryType(selectedPackage.type)">
              {{ selectedPackage.type }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="下载次数">{{ selectedPackage.downloads }}</el-descriptions-item>
          <el-descriptions-item label="上传时间">{{ formatDate(selectedPackage.uploadTime) }}</el-descriptions-item>
          <el-descriptions-item label="最后更新">{{ formatDate(selectedPackage.lastUpdate) }}</el-descriptions-item>
          <el-descriptions-item label="描述" :span="2">{{ selectedPackage.description }}</el-descriptions-item>
          <el-descriptions-item label="项目链接" :span="2">
            <a :href="selectedPackage.url" target="_blank">{{ selectedPackage.url }}</a>
          </el-descriptions-item>
        </el-descriptions>
        
        <div class="detail-actions">
          <el-button 
            type="primary" 
            @click="downloadPackage(selectedPackage); detailDialogVisible = false"
            :disabled="selectedPackage.status !== '正常' && selectedPackage.status !== 'Active'"
          >
            <el-icon><Download /></el-icon>
            下载绳包
          </el-button>
          <el-button @click="detailDialogVisible = false">
            <el-icon><Close /></el-icon>
            关闭
          </el-button>
        </div>
      </div>
    </el-dialog>

    <!-- 添加内容对话框 -->
    <el-dialog
      v-model="addDialogVisible"
      title="添加内容"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form :model="newPackage" label-width="100px">
        <el-form-item label="内容类型">
                  <el-radio-group v-model="newPackage.type">
          <el-radio value="package">资源</el-radio>
          <el-radio value="post">帖子</el-radio>
        </el-radio-group>
        </el-form-item>
        
        <el-form-item :label="newPackage.type === 'package' ? '资源名称' : '帖子标题'">
          <el-input v-model="newPackage.name" :placeholder="newPackage.type === 'package' ? '请输入资源名称' : '请输入帖子标题'" />
        </el-form-item>
        
        <el-form-item label="作者" v-if="newPackage.type === 'package'">
          <el-input v-model="newPackage.author" placeholder="请输入作者" />
        </el-form-item>
        
        <el-form-item label="版本" v-if="newPackage.type === 'package'">
          <el-input v-model="newPackage.version" placeholder="请输入版本号" />
        </el-form-item>
        
        <el-form-item :label="newPackage.type === 'package' ? '资源分类' : '帖子分类'">
          <el-select v-model="newPackage.category_id" :placeholder="newPackage.type === 'package' ? '选择资源分类' : '选择帖子分类'">
            <el-option 
              v-for="cat in categories" 
              :key="cat.id" 
              :label="cat.name" 
              :value="cat.id"
              :disabled="!cat.enabled"
            />
          </el-select>
        </el-form-item>
        
        <el-form-item label="状态" v-if="newPackage.type === 'package'">
          <el-select v-model="newPackage.status" placeholder="选择状态">
            <el-option label="正常" value="正常" />
            <el-option label="已发布" value="已发布" />
            <el-option label="待审核" value="待审核" />
            <el-option label="已拒绝" value="已拒绝" />
            <el-option label="维护中" value="维护中" />
          </el-select>
        </el-form-item>
        
        <el-form-item :label="newPackage.type === 'package' ? '资源描述' : '帖子内容'">
          <el-input 
            v-model="newPackage.description" 
            type="textarea" 
            :rows="3"
            :placeholder="newPackage.type === 'package' ? '请输入资源描述' : '请输入帖子内容'"
          />
        </el-form-item>
        
        <el-form-item label="标签">
          <el-input
            v-model="newPackage.tagsInput"
            placeholder="输入标签，用逗号分隔"
            @keyup.enter="addTag"
            clearable
          />
          <div class="tags-container">
            <el-tag
              v-for="tag in newPackage.tags"
              :key="tag"
              closable
              @close="removeNewPackageTag(tag)"
              effect="light"
            >
              {{ tag }}
            </el-tag>
          </div>
        </el-form-item>
        
        <el-form-item label="项目链接" v-if="newPackage.type === 'package'">
          <el-input v-model="newPackage.url" placeholder="请输入项目链接" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="addPackage">
          {{ newPackage.type === 'package' ? '添加资源' : '发布帖子' }}
        </el-button>
      </template>
    </el-dialog>

    <!-- 编辑绳包对话框 -->
    <el-dialog
      v-model="editDialogVisible"
      title="编辑资源"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form :model="editingPackage" label-width="100px">
        <el-form-item label="名称">
          <el-input v-model="editingPackage.name" placeholder="请输入资源名称" />
        </el-form-item>
        <el-form-item label="作者">
          <el-input v-model="editingPackage.author" placeholder="请输入作者" />
        </el-form-item>
        <el-form-item label="版本">
          <el-input v-model="editingPackage.version" placeholder="请输入版本号" />
        </el-form-item>
        <el-form-item label="分类">
          <el-select v-model="editingPackage.category_id" placeholder="选择分类">
            <el-option 
              v-for="cat in categories" 
              :key="cat.id" 
              :label="cat.name" 
              :value="cat.id"
              :disabled="!cat.enabled"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="editingPackage.status" placeholder="选择状态">
            <el-option label="正常" value="正常" />
            <el-option label="已发布" value="已发布" />
            <el-option label="待审核" value="待审核" />
            <el-option label="已拒绝" value="已拒绝" />
            <el-option label="维护中" value="维护中" />
          </el-select>
        </el-form-item>
        <el-form-item label="描述">
          <el-input 
            v-model="editingPackage.description" 
            type="textarea" 
            :rows="3"
            placeholder="请输入描述"
          />
        </el-form-item>
        <el-form-item label="项目链接">
          <el-input v-model="editingPackage.url" placeholder="请输入项目链接" />
        </el-form-item>
        <el-form-item label="标签">
          <el-input 
            v-model="editingPackage.tagsInput" 
            placeholder="请输入标签，用逗号分隔"
            @input="handleTagsInput"
            @keyup.enter="addEditingTag"
          />
          <div class="tags-display" v-if="editingPackage.tags && editingPackage.tags.length > 0">
            <el-tag 
              v-for="tag in editingPackage.tags" 
              :key="tag" 
              closable 
              @close="removeTag(tag)"
              style="margin: 4px 4px 0 0;"
            >
              {{ tag }}
            </el-tag>
          </div>
        </el-form-item>
        <el-form-item label="状态设置">
          <el-checkbox v-model="editingPackage.is_pinned">置顶</el-checkbox>
          <el-checkbox v-model="editingPackage.is_featured">精华</el-checkbox>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="updatePackage">保存</el-button>
      </template>
    </el-dialog>

    <!-- 分类管理对话框 -->
    <el-dialog
      v-model="showCategoryDialog"
      title="分类管理"
      width="800px"
      :close-on-click-modal="false"
    >
      <div class="category-management">
        <div class="category-header">
          <el-button type="primary" @click="showAddCategoryDialog = true">
            <el-icon><Plus /></el-icon>
            添加分类
          </el-button>
        </div>
        
        <el-table :data="categories" style="width: 100%" stripe>
          <el-table-column prop="name" label="分类名称" width="150" />
          <el-table-column prop="description" label="描述" min-width="200" />
          <el-table-column prop="count" label="资源数量" width="100" />
          <el-table-column prop="enabled" label="状态" width="100">
            <template #default="{ row }">
              <el-tag :type="row.enabled ? 'success' : 'info'">
                {{ row.enabled ? '启用' : '禁用' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="200">
            <template #default="{ row }">
              <el-button size="small" @click="editCategory(row)">
                <el-icon><Edit /></el-icon>
                编辑
              </el-button>
              <el-button size="small" type="danger" @click="deleteCategoryItem(row)">
                <el-icon><Delete /></el-icon>
                删除
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
      
      <!-- 添加分类对话框 -->
      <el-dialog
        v-model="showAddCategoryDialog"
        title="添加分类"
        width="500px"
        append-to-body
      >
        <el-form :model="newCategory" label-width="100px">
          <el-form-item label="分类名称">
            <el-input v-model="newCategory.name" placeholder="请输入分类名称" />
          </el-form-item>
          <el-form-item label="描述">
            <el-input 
              v-model="newCategory.description" 
              type="textarea" 
              :rows="3"
              placeholder="请输入分类描述"
            />
          </el-form-item>
          <el-form-item label="状态">
            <el-switch v-model="newCategory.enabled" />
          </el-form-item>
        </el-form>
        <template #footer>
          <el-button @click="showAddCategoryDialog = false">取消</el-button>
          <el-button type="primary" @click="addCategoryItem">添加</el-button>
        </template>
      </el-dialog>
      
      <!-- 编辑分类对话框 -->
      <el-dialog
        v-model="showEditCategoryDialog"
        title="编辑分类"
        width="500px"
        append-to-body
      >
        <el-form :model="editingCategory" label-width="100px">
          <el-form-item label="分类名称">
            <el-input v-model="editingCategory.name" placeholder="请输入分类名称" />
          </el-form-item>
          <el-form-item label="描述">
            <el-input 
              v-model="editingCategory.description" 
              type="textarea" 
              :rows="3"
              placeholder="请输入分类描述"
            />
          </el-form-item>
          <el-form-item label="状态">
            <el-switch v-model="editingCategory.enabled" />
          </el-form-item>
        </el-form>
        <template #footer>
          <el-button @click="showEditCategoryDialog = false">取消</el-button>
          <el-button type="primary" @click="saveCategory">保存</el-button>
        </template>
      </el-dialog>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Box, 
  Plus, 
  Edit, 
  Delete, 
  View, 
  Download,
  Search,
  Refresh,
  Folder,
  Clock,
  User,
  Check,
  Close
} from '@element-plus/icons-vue'
import { packageApi, categoryApi } from '../../api'
import { apiCache } from '../../api/cache'
import { resourceRecordApi } from '../../api/resourceRecords'
import { resourceLogger } from '../../utils/loggerService'
import { 
  Package, 
  CreatePackageRequest, 
  UpdatePackageRequest, 
  PackageListResponse 
} from '../../api/packages'
import { Category, CreateCategoryRequest, UpdateCategoryRequest } from '../../api/categories'
import { createPost } from '../../api/posts'
import axios from 'axios' // 导入axios
import userActionService from '../../utils/userActionService'

// 响应式数据
const searchQuery = ref('')
const statusFilter = ref('')
const typeFilter = ref('')
const loading = ref(false)
const currentPage = ref(1)
const pageSize = ref(20)
const detailDialogVisible = ref(false)
const addDialogVisible = ref(false)
const editDialogVisible = ref(false)
const showCategoryDialog = ref(false)
const showAddCategoryDialog = ref(false)
const showEditCategoryDialog = ref(false)
const selectedPackage = ref<any>(null)

// 绳包数据
const packages = ref<any[]>([])
const totalPackages = ref(0)
const availablePackages = ref(0)
const borrowedPackages = ref(0)
const maintenancePackages = ref(0)

// 分类数据
const categories = ref<any[]>([])

// 新内容表单
const newPackage = ref({
  type: 'package' as 'package' | 'post',
  name: '',
  author: '',
  version: '1.0.0',
  category_id: undefined as number | undefined,
  status: '正常',
  description: '',
  url: '',
  tags: [] as string[],
  tagsInput: ''
})

// 新分类表单
const newCategory = ref({
  name: '',
  description: '',
  enabled: true
})

// 编辑绳包表单
const editingPackage = ref({
  id: 0,
  name: '',
  author: '',
  version: '',
  category_id: undefined as number | undefined,
  status: '',
  description: '',
  url: '',
  tags: [] as string[],
  tagsInput: '',
  is_pinned: false,
  is_featured: false
})

// 编辑分类表单
const editingCategory = ref({
  id: 0,
  name: '',
  description: '',
  enabled: true
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
  
  // 状态过滤现在在API级别处理，不需要前端过滤
  
  // 类型过滤 - 使用后端分类名
  if (typeFilter.value) {
    filtered = filtered.filter(pkg => pkg.type === typeFilter.value)
  }
  
  return filtered
})

// 方法
const refreshDownloadCount = async (pkgId: number) => {
  try {
    // 查询单个包的最新信息
    const res = await packageApi.getPackage(pkgId)
    if (res.code === 0 && res.data) {
      // 更新内存中的下载计数
      const index = packages.value.findIndex(p => p.id === pkgId)
      if (index !== -1) {
        packages.value[index].downloads = res.data.download_count
        // 更新统计数据（虽然下载计数不影响统计卡片，但保持一致性）
        updateStatistics()
      }
    }
  } catch (error) {
    console.error('刷新下载计数失败:', error)
  }
}

// 添加观察函数，当包数据变化时更新统计
function setupWatchers() {
  // 监听筛选条件变化
  watch([statusFilter, typeFilter], () => {
    // 当筛选条件变化时，重新加载数据
    loadPackages()
  })
  
  // 监听包数据变化
  watch(() => packages.value, () => {
    updateStatistics()
  }, { deep: true })
}

async function loadPackages() {
  try {
    loading.value = true
    // 强制清除缓存
    apiCache.delete('getPackages')
    
    // 构建查询参数
    const params: any = {}
    if (statusFilter.value) {
      params.status = statusFilter.value
    }
    if (searchQuery.value) {
      params.search = searchQuery.value
    }
    
    const res = await packageApi.getPackages(params)
    if (res.code === 0 && res.data) {
      packages.value = res.data.list?.map((pkg) => {
        // 根据分类ID查找分类名称
        let categoryName = '未分类';
        if (pkg.category_id) {
          const category = categories.value.find(c => c.id === pkg.category_id);
          if (category) {
            categoryName = category.name;
          }
        }
        
        return {
        id: pkg.id,
          name: pkg.name,
          author: pkg.author,
          version: pkg.version || '',
          category_id: pkg.category_id,
          type: categoryName,
          status: pkg.status || 'Active',
          description: pkg.description || '',
          downloads: pkg.download_count,
          uploadTime: pkg.created_at,
          lastUpdate: pkg.updated_at,
          url: pkg.file_url || '' // 确保url字段正确映射
        };
      }) || []
      
      console.log('处理后的绳包数据:', packages.value)
      
      // 更新统计数据
      updateStatistics()
    } else {
      ElMessage.error('获取绳包数据失败')
    }
  } catch (error) {
    console.error('获取绳包数据错误:', error)
    ElMessage.error('获取绳包数据失败')
  } finally {
    loading.value = false
  }
}

// 更新统计数据
function updateStatistics() {
  // 总资源数
  totalPackages.value = packages.value.length
  
  // 已发布资源数（正常或Active状态）
  availablePackages.value = packages.value.filter(pkg => 
    pkg.status === '正常' || pkg.status === 'Active' || pkg.status === '已发布'
  ).length
  
  // 待审核资源数（待审核或Inactive状态）
  borrowedPackages.value = packages.value.filter(pkg => 
    pkg.status === '待审核' || pkg.status === 'Inactive'
  ).length
  
  // 活跃用户数量（统计不同作者数量）
  const uniqueAuthors = new Set(packages.value.map(pkg => pkg.author))
  maintenancePackages.value = uniqueAuthors.size

  console.log('统计数据更新:', {
    总资源数: totalPackages.value,
    已发布: availablePackages.value,
    待审核: borrowedPackages.value,
    活跃用户: maintenancePackages.value
  })
}

async function loadCategories() {
  try {
    // 强制清除缓存
    apiCache.delete('getCategories')
    const res = await categoryApi.getCategories()
    if (res.code === 0 && res.data) {
      // 确保data.list存在且是数组
      categories.value = res.data.list || []
      console.log('获取到的分类数据:', categories.value)
    } else {
      ElMessage.error('获取分类数据失败')
    }
  } catch (error) {
    console.error('获取分类数据错误:', error)
    ElMessage.error('获取分类数据失败')
  }
}

function handleSearch() {
  // 重新加载数据以应用搜索条件
  loadPackages()
}

async function refreshData() {
  // 清除所有相关缓存
  apiCache.delete('getPackages')
  apiCache.delete('getCategories')
  
  // 重新加载数据
  await Promise.all([
    loadPackages(),
    loadCategories()
  ])
  
  ElMessage.success('数据已刷新')
}

function showAddPackageDialog() {
  addDialogVisible.value = true
}

// 查看绳包详情
const viewPackage = (row: any) => {
  selectedPackage.value = row
  detailDialogVisible.value = true
  
  // 记录查看详情的行为
  userActionService.logView('Package', row.id, `管理员查看绳包详情: ${row.name}`)
    .catch(err => console.error('记录查看行为失败:', err))
}

// 下载绳包
const downloadPackage = async (row: any) => {
  try {
    // 检查资源状态是否为正常
    if (row.status !== '正常' && row.status !== 'Active') {
      ElMessage.warning(`资源 ${row.name} 状态不为正常，无法下载`)
      return
    }
    
    // 检查资源是否有项目链接
    if (!row.url || row.url.trim() === '') {
      ElMessage.warning(`资源 ${row.name} 没有项目链接，无法下载`)
      return
    }
    
    // 先增加下载计数
    const res = await packageApi.downloadPackage(row.id)
    if (res.code !== 0) {
      console.warn('增加下载计数失败:', res.message)
      // 继续下载，但记录警告
    }
    
    // 记录资源下载操作
    await resourceLogger.logDownload(row.id)
    
    // 打开下载链接
    window.open(row.url, '_blank')
      ElMessage.success(`开始下载 ${row.name}`)
    
    // 延迟刷新当前包的下载计数
    setTimeout(() => {
      refreshDownloadCount(row.id)
    }, 500)
  } catch (error) {
    console.error('下载错误:', error)
    ElMessage.error('下载失败')
  }
}

function editPackage(pkg: any) {
  // 复制数据到编辑表单
  editingPackage.value = {
    id: pkg.id,
    name: pkg.name,
    author: pkg.author,
    version: pkg.version,
    category_id: pkg.category_id ? Number(pkg.category_id) : undefined,
    status: pkg.status || 'Active',
    description: pkg.description,
    url: pkg.url,
    tags: pkg.tags || [],
    tagsInput: (pkg.tags || []).join(', '),
    is_pinned: pkg.is_pinned || false,
    is_featured: pkg.is_featured || false
  }
  editDialogVisible.value = true
  
  // 记录编辑操作
  userActionService.logAction('EditPackage', `管理员打开编辑绳包: ${pkg.name}`, 'Package', pkg.id)
    .catch(err => console.error('记录编辑行为失败:', err))
}

async function updatePackage() {
  try {
    // 验证表单数据
    if (!editingPackage.value.name.trim()) {
      ElMessage.error('请输入绳包名称')
      return
    }
    if (!editingPackage.value.author.trim()) {
      ElMessage.error('请输入作者')
      return
    }
    if (!editingPackage.value.description.trim()) {
      ElMessage.error('请输入描述')
      return
    }
    if (!editingPackage.value.url.trim()) {
      ElMessage.error('请输入项目链接')
      return
    }
    
    // 将状态字符串转换为后端需要的枚举
    let statusEnum: 'Pending' | 'Active' | 'Rejected' | 'Inactive' | 'Deleted' = 'Active'
    switch (editingPackage.value.status) {
      case '正常':
      case '已发布':
        statusEnum = 'Active'
        break
      case '待审核':
      case '维护中':
        statusEnum = 'Inactive'
        break
      case '已拒绝':
        statusEnum = 'Deleted'
        break
    }
    
    const updateData: UpdatePackageRequest = {
      name: editingPackage.value.name,
      version: editingPackage.value.version,
      description: editingPackage.value.description,
      category_id: editingPackage.value.category_id,
      status: statusEnum,
      file_url: editingPackage.value.url, // 添加file_url字段
      tags: editingPackage.value.tags,
      is_pinned: editingPackage.value.is_pinned,
      is_featured: editingPackage.value.is_featured
    }
    
    console.log('更新绳包数据:', updateData)
    const res = await packageApi.updatePackage(editingPackage.value.id, updateData)
    if (res.code === 0) {
      ElMessage.success('绳包更新成功')
      
      // 注意：资源操作记录已在API层自动处理，不需要在这里重复记录
      
      editDialogVisible.value = false
      // 延迟刷新数据，确保后端数据已更新
      setTimeout(async () => {
        await loadCategories() // 先加载分类数据
        await loadPackages() // 再加载绳包数据
        // 无需单独调用updateStatistics，因为loadPackages中已包含
      }, 500)
    } else {
      ElMessage.error(res.message || '更新失败')
    }
  } catch (error) {
    console.error('更新绳包错误:', error)
    ElMessage.error('更新失败')
  }
}

// 处理标签输入
const handleTagsInput = () => {
  if (editingPackage.value.tagsInput) {
    const tags = editingPackage.value.tagsInput.split(',').map(tag => tag.trim()).filter(tag => tag)
    editingPackage.value.tags = tags
  } else {
    editingPackage.value.tags = []
  }
}

// 移除标签
const removeTag = (tagToRemove: string) => {
  if (editingPackage.value.tags) {
    editingPackage.value.tags = editingPackage.value.tags.filter(tag => tag !== tagToRemove)
    editingPackage.value.tagsInput = editingPackage.value.tags.join(', ')
  }
}

async function deletePackage(pkg: any) {
  try {
    await ElMessageBox.confirm(
      `确定要删除绳包 ${pkg.name} 吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 删除前记录资源的信息，用于日志
    const packageInfo = {
      id: pkg.id,
      name: pkg.name,
      author: pkg.author
    }
    
    const res = await packageApi.deletePackage(pkg.id)
    if (res.code === 0) {
      ElMessage.success('绳包已删除')
      
      // 手动记录删除操作（因为API拦截器可能无法正确获取被删除资源的信息）
      // 注释掉这行，避免重复记录，让API拦截器自动记录
      // await resourceLogger.logDelete(pkg.id)
      
      // 延迟刷新数据，确保后端数据已更新
      setTimeout(async () => {
        await loadPackages() // 重新加载数据
        await loadCategories() // 重新加载分类数据
      }, 500)
    } else {
      ElMessage.error(res.message || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除绳包错误:', error)
      ElMessage.error('删除失败')
    }
  }
}

// 单个包状态更改时更新统计数据
async function updateSinglePackageStatus(pkgId: number, newStatus: string) {
  try {
    // 查询单个包的最新信息
    const res = await packageApi.getPackage(pkgId)
    if (res.code === 0 && res.data) {
      // 更新内存中的状态
      const index = packages.value.findIndex(p => p.id === pkgId)
      if (index !== -1) {
        packages.value[index].status = newStatus
        // 更新统计数据
        updateStatistics()
        
        // 注意：资源状态变更操作已在API层自动处理，不需要在这里重复记录
      }
    }
  } catch (error) {
    console.error('更新包状态失败:', error)
  }
}

// 标签处理函数
function addTag() {
  const tag = newPackage.value.tagsInput?.trim()
  if (tag && tag.length > 0 && !newPackage.value.tags.includes(tag)) {
    newPackage.value.tags.push(tag)
    newPackage.value.tagsInput = ''
  }
}

function addEditingTag() {
  const tag = editingPackage.value.tagsInput?.trim()
  if (tag && tag.length > 0 && !editingPackage.value.tags.includes(tag)) {
    editingPackage.value.tags.push(tag)
    editingPackage.value.tagsInput = ''
  }
}

function removeNewPackageTag(tag: string) {
  const index = newPackage.value.tags.indexOf(tag)
  if (index > -1) {
    newPackage.value.tags.splice(index, 1)
  }
}

async function addPackage() {
  try {
    // 验证表单数据
    if (!newPackage.value.name.trim()) {
      ElMessage.error('请输入标题')
      return
    }
    if (!newPackage.value.description.trim()) {
      ElMessage.error('请输入内容')
      return
    }
    
    if (newPackage.value.type === 'package') {
      // 验证资源特有字段
      if (!newPackage.value.author.trim()) {
        ElMessage.error('请输入作者')
        return
      }
      if (!newPackage.value.url.trim()) {
        ElMessage.error('请输入项目链接')
        return
      }
      
      const packageData: CreatePackageRequest = {
        name: newPackage.value.name,
        author: newPackage.value.author,
        version: newPackage.value.version,
        description: newPackage.value.description,
        category_id: newPackage.value.category_id,
        file_url: newPackage.value.url,
        tags: newPackage.value.tags,
        is_pinned: false,
        is_featured: false
      }
      
      console.log('创建资源数据:', packageData)
      // 管理员使用专用创建接口，可以设置任意作者和状态
      const res = await packageApi.adminCreatePackage(packageData)
      if (res.code === 0) {
        ElMessage.success('资源添加成功')
        
        // 记录用户行为
        userActionService.logUpload('Package', res.data?.id || 0, `管理员创建资源: ${packageData.name}`)
        
        addDialogVisible.value = false
        // 清空表单
        newPackage.value = {
          type: 'package',
          name: '',
          author: '',
          version: '1.0.0',
          category_id: undefined,
          status: '正常',
          description: '',
          url: '',
          tags: [],
          tagsInput: ''
        }
        // 延迟刷新数据，确保后端数据已更新
        setTimeout(async () => {
          await loadCategories() // 先加载分类数据
          await loadPackages() // 再加载绳包数据
        }, 500)
      } else {
        ElMessage.error(res.msg || '添加失败')
      }
    } else {
      // 发布帖子
      const res = await createPost({
        title: newPackage.value.name,
        content: newPackage.value.description,
        category_id: newPackage.value.category_id,
        tags: newPackage.value.tags,
        status: 'Published'
      })
      
      if (res.code === 0) {
        ElMessage.success('帖子发布成功')
        
        addDialogVisible.value = false
        // 清空表单
        newPackage.value = {
          type: 'package',
          name: '',
          author: '',
          version: '1.0.0',
          category_id: undefined,
          status: '正常',
          description: '',
          url: '',
          tags: [],
          tagsInput: ''
        }
        // 延迟刷新数据
        setTimeout(async () => {
          await loadCategories()
          await loadPackages()
        }, 500)
      } else {
        ElMessage.error(res.msg || '发布失败')
      }
    }
  } catch (error) {
    console.error('添加内容错误:', error)
    ElMessage.error(newPackage.value.type === 'package' ? '添加失败' : '发布失败')
  }
}

async function addCategoryItem() {
  try {
    if (!newCategory.value.name.trim()) {
      ElMessage.error('请输入分类名称')
      return
    }
    
    const res = await categoryApi.addCategory({
      name: newCategory.value.name,
      description: newCategory.value.description,
      enabled: newCategory.value.enabled
    })
    
    if (res.code === 0) {
      ElMessage.success('分类添加成功')
      
      // 记录分类创建操作
      if (res.data && res.data.id) {
        await resourceLogger.logCreate(res.data.id, 'Category')
      }
      
      showAddCategoryDialog.value = false
      // 清空表单
      newCategory.value = {
        name: '',
        description: '',
        enabled: true
      }
      // 重新加载分类
      await loadCategories()
    } else {
      ElMessage.error(res.message || '添加分类失败')
    }
  } catch (error) {
    console.error('添加分类出错:', error)
    ElMessage.error('添加分类失败')
  }
}

function editCategory(category: any) {
  // 复制数据到编辑表单
  editingCategory.value = {
    id: category.id,
    name: category.name,
    description: category.description,
    enabled: category.enabled
  }
  showEditCategoryDialog.value = true
}

async function saveCategory() {
  try {
    if (!editingCategory.value.name.trim()) {
      ElMessage.error('请输入分类名称')
      return
    }
    
    const res = await categoryApi.updateCategory(
      editingCategory.value.id,
      {
      name: editingCategory.value.name,
      description: editingCategory.value.description,
        enabled: editingCategory.value.enabled
    }
    )
    
    if (res.code === 0) {
      ElMessage.success('分类更新成功')
      
      // 记录分类更新操作
      await resourceLogger.logUpdate(editingCategory.value.id, 'Category')
      
      showEditCategoryDialog.value = false
      // 重新加载分类
      await loadCategories()
    } else {
      ElMessage.error(res.message || '更新分类失败')
    }
  } catch (error) {
    console.error('更新分类出错:', error)
    ElMessage.error('更新分类失败')
  }
}

async function deleteCategoryItem(category: any) {
  try {
    await ElMessageBox.confirm(
      `确定要删除分类 ${category.name} 吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 先记录删除操作
    await resourceLogger.logDelete(category.id, 'Category')
    
    const res = await categoryApi.deleteCategory(category.id)
    if (res.code === 0) {
      ElMessage.success('分类已删除')
      // 重新加载分类
      await loadCategories()
    } else {
      ElMessage.error(res.message || '删除分类失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除分类出错:', error)
      ElMessage.error('删除分类失败')
    }
  }
}

function getStatusText(status: string) {
  const statusMap: Record<string, string> = {
    // 中文状态名
    '正常': '正常',
    '已发布': '已发布',
    '待审核': '待审核',
    '已拒绝': '已拒绝',
    '维护中': '维护中',
    // 英文枚举值（对应后端PackageStatus）
    'Active': '已发布',
    'Pending': '待审核',
    'Rejected': '已拒绝',
    'Inactive': '已下架',
    'Deleted': '已删除'
  }
  return statusMap[status] || status
}

function getStatusType(status: string) {
  const typeMap: Record<string, string> = {
    // 中文状态
    '正常': 'success',
    '已发布': 'success',
    '待审核': 'warning',
    '已拒绝': 'danger',
    '维护中': 'warning',
    '已下架': 'info',
    '已删除': 'danger',
    // 英文状态（对应后端PackageStatus）
    'Active': 'success',
    'Pending': 'warning',
    'Rejected': 'danger',
    'Inactive': 'info',
    'Deleted': 'danger'
  }
  return typeMap[status] || 'info'
}

function getCategoryText(category: string) {
  // 因为在loadPackages中已经转换为分类名称，这里直接返回
  return category
}

function getCategoryType(category: string) {
  const typeMap: Record<string, string> = {
    '教程': 'primary',
    '工具': 'success', 
    '模板': 'warning',
    '其他': 'info',
    '未分类': 'info'
  }
  return typeMap[category] || 'info'
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
onMounted(async () => {
  await loadCategories() // 先加载分类数据
  await loadPackages() // 再加载绳包数据
  setupWatchers() // 设置数据观察
})
</script>

<style scoped>
.package-manage-desktop {
  padding: 24px;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
  min-height: calc(100vh - 72px); /* 减去导航栏高度 */
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
:global(html.dark) .package-manage-desktop {
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

:global(html.dark) .page-header,
:global(html.dark) .stat-card,
:global(html.dark) .search-section,
:global(html.dark) .table-section,
:global(html.dark) .pagination-section {
  background: linear-gradient(135deg, var(--bg-card) 0%, var(--bg-secondary) 100%);
  border-color: var(--border-color);
}

/* 主题适配 */
:global(html.blue) .stat-card::before,
:global(html.blue) .package-card::before {
  background: linear-gradient(90deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

:global(html.green) .stat-card::before,
:global(html.green) .package-card::before {
  background: linear-gradient(90deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

:global(html.orange) .stat-card::before,
:global(html.orange) .package-card::before {
  background: linear-gradient(90deg, var(--warning-color) 0%, var(--warning-color-light) 100%);
}

:global(html.purple) .stat-card::before,
:global(html.purple) .package-card::before {
  background: linear-gradient(90deg, var(--info-color) 0%, var(--info-color-light) 100%);
}

:global(html.blue) .stat-icon,
:global(html.blue) .header-icon {
  background: linear-gradient(135deg, var(--brand-color) 0%, var(--brand-color-light) 100%);
}

:global(html.green) .stat-icon,
:global(html.green) .header-icon {
  background: linear-gradient(135deg, var(--success-color) 0%, var(--success-color-light) 100%);
}

:global(html.purple) .stat-icon,
:global(html.purple) .header-icon {
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

.tags-container {
  margin-top: 8px;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tags-display {
  margin-top: 8px;
  min-height: 32px;
  padding: 4px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  background-color: var(--el-fill-color-lighter);
}
</style> 