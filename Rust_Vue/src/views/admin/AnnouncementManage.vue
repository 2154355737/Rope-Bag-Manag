<template>
  <div class="admin-page announcement-manage">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><Bell /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">公告管理</h1>
            <p class="page-subtitle">管理系统公告，包括发布、编辑、删除和状态管理</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button type="primary" @click="createAnnouncement">
            <el-icon><Plus /></el-icon>
            发布公告
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Document /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ announcementList.length }}</div>
            <div class="stat-label">总公告数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Check /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ activeAnnouncements }}</div>
            <div class="stat-label">已启用</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><WarningFilled /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ urgentAnnouncements }}</div>
            <div class="stat-label">紧急公告</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Calendar /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ scheduledAnnouncements }}</div>
            <div class="stat-label">定时公告</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和操作区域 -->
    <div class="search-section">
      <div class="search-left">
        <el-input 
          v-model="searchQuery" 
          placeholder="搜索公告..." 
          clearable
          style="width: 250px"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        
        <el-select 
          v-model="filterType" 
          placeholder="选择类型" 
          clearable
          style="width: 150px"
        >
          <el-option label="全部" value="" />
          <el-option label="系统公告" value="System" />
          <el-option label="活动公告" value="Event" />
          <el-option label="维护公告" value="Maintenance" />
          <el-option label="更新公告" value="Update" />
        </el-select>
        
        <el-select 
          v-model="filterStatus" 
          placeholder="状态筛选" 
          clearable
          style="width: 150px"
        >
          <el-option label="全部" value="" />
          <el-option label="已启用" value="enabled" />
          <el-option label="已禁用" value="disabled" />
        </el-select>
      </div>
      
      <div class="search-right">
        <el-button type="success" @click="batchEnable" :disabled="!selectedAnnouncements.length">
          <el-icon><Check /></el-icon>
          批量启用
        </el-button>
        <el-button type="warning" @click="batchDisable" :disabled="!selectedAnnouncements.length">
          <el-icon><Close /></el-icon>
          批量禁用
        </el-button>
        <el-button type="danger" @click="batchDelete" :disabled="!selectedAnnouncements.length">
          <el-icon><Delete /></el-icon>
          批量删除
        </el-button>
      </div>
    </div>

    <!-- 公告列表 -->
    <div class="table-section">
      <el-table 
        :data="filteredAnnouncements" 
        v-loading="loading"
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
        border
        stripe
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="title" label="标题" min-width="200">
          <template #default="{ row }">
            <div class="announcement-title">
              <span class="title-text">{{ row.title }}</span>
              <el-tag 
                :type="getPriorityTag(row.priority)" 
                size="small"
                class="priority-tag"
              >
                P{{ row.priority }}
              </el-tag>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="content" label="内容" min-width="300">
          <template #default="{ row }">
            <div class="announcement-content">
              <p class="content-text">{{ truncateText(row.content, 100) }}</p>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="type_" label="类型" width="100">
          <template #default="{ row }">
            <el-tag :type="getTypeTag(row.type_)">
              {{ getTypeLabel(row.type_) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="enabled" label="状态" width="100">
          <template #default="{ row }">
            <el-switch 
              v-model="row.enabled" 
              @change="toggleStatus(row)"
              :loading="row.statusLoading"
            />
          </template>
        </el-table-column>
        <el-table-column prop="start_time" label="开始时间" width="180">
          <template #default="{ row }">
            {{ formatTime(row.start_time) }}
          </template>
        </el-table-column>
        <el-table-column prop="end_time" label="结束时间" width="180">
          <template #default="{ row }">
            {{ row.end_time ? formatTime(row.end_time) : '永久' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="viewAnnouncement(row)">
              <el-icon><View /></el-icon>
              查看
            </el-button>
            <el-button size="small" type="primary" @click="editAnnouncement(row)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button size="small" type="danger" @click="deleteAnnouncement(row)">
              <el-icon><Delete /></el-icon>
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-section">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="filteredAnnouncements.length"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </div>

    <!-- 批量操作 -->
    <div class="batch-actions" v-if="selectedAnnouncements.length > 0">
      <span class="selected-count">已选择 {{ selectedAnnouncements.length }} 条公告</span>
    </div>

    <!-- 公告编辑对话框 -->
    <el-dialog 
      v-model="announcementDialogVisible" 
      :title="isEdit ? '编辑公告' : '发布公告'" 
      width="800px"
    >
      <el-form :model="announcementForm" :rules="formRules" ref="formRef" label-width="100px">
        <el-form-item label="公告标题" prop="title">
          <el-input v-model="announcementForm.title" placeholder="请输入公告标题" />
        </el-form-item>
        <el-form-item label="公告内容" prop="content">
          <el-input 
            v-model="announcementForm.content" 
            type="textarea" 
            :rows="6"
            placeholder="请输入公告内容"
          />
        </el-form-item>
        <el-form-item label="公告类型" prop="type_">
          <el-select v-model="announcementForm.type_" placeholder="选择公告类型">
            <el-option label="信息" value="Info" />
            <el-option label="警告" value="Warning" />
            <el-option label="错误" value="Error" />
            <el-option label="成功" value="Success" />
          </el-select>
        </el-form-item>
        <el-form-item label="优先级" prop="priority">
          <el-input-number 
            v-model="announcementForm.priority" 
            :min="1" 
            :max="10" 
            placeholder="1-10"
          />
          <span class="priority-desc">优先级越高，显示越靠前</span>
        </el-form-item>
        <el-form-item label="启用状态">
          <el-switch v-model="announcementForm.enabled" />
        </el-form-item>
        <el-form-item label="开始时间" prop="start_time">
          <el-date-picker
            v-model="announcementForm.start_time"
            type="datetime"
            placeholder="选择开始时间"
            format="YYYY-MM-DD HH:mm:ss"
            value-format="YYYY-MM-DD HH:mm:ss"
          />
        </el-form-item>
        <el-form-item label="结束时间">
          <el-date-picker
            v-model="announcementForm.end_time"
            type="datetime"
            placeholder="选择结束时间（可选）"
            format="YYYY-MM-DD HH:mm:ss"
            value-format="YYYY-MM-DD HH:mm:ss"
          />
          <span class="end-time-desc">留空表示永久有效</span>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="announcementDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveAnnouncement" :loading="saving">
          {{ isEdit ? '更新' : '发布' }}
        </el-button>
      </template>
    </el-dialog>

    <!-- 公告详情对话框 -->
    <el-dialog 
      v-model="detailDialogVisible" 
      title="公告详情" 
      width="600px"
    >
      <div class="announcement-detail" v-if="currentAnnouncement">
        <div class="detail-item">
          <label>公告ID:</label>
          <span>{{ currentAnnouncement.id }}</span>
        </div>
        <div class="detail-item">
          <label>标题:</label>
          <span>{{ currentAnnouncement.title }}</span>
        </div>
        <div class="detail-item">
          <label>内容:</label>
          <div class="content-box">{{ currentAnnouncement.content }}</div>
        </div>
        <div class="detail-item">
          <label>类型:</label>
          <el-tag :type="getTypeTag(currentAnnouncement.type_)">
            {{ getTypeLabel(currentAnnouncement.type_) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>优先级:</label>
          <el-tag :type="getPriorityTag(currentAnnouncement.priority)">
            P{{ currentAnnouncement.priority }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>状态:</label>
          <el-tag :type="currentAnnouncement.enabled ? 'success' : 'info'">
            {{ currentAnnouncement.enabled ? '启用' : '禁用' }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>开始时间:</label>
          <span>{{ formatTime(currentAnnouncement.start_time) }}</span>
        </div>
        <div class="detail-item">
          <label>结束时间:</label>
          <span>{{ currentAnnouncement.end_time ? formatTime(currentAnnouncement.end_time) : '永久' }}</span>
        </div>
      </div>
      <template #footer>
        <el-button @click="detailDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Bell,
  Document,
  Check,
  WarningFilled,
  Calendar,
  Search,
  View,
  Edit,
  Delete,
  Close,
  Plus
} from '@element-plus/icons-vue'
import { 
  getAnnouncements, 
  createAnnouncement as createAnnouncementApi,
  updateAnnouncement,
  deleteAnnouncement as deleteAnnouncementApi,
  batchUpdateAnnouncementStatus,
  batchDeleteAnnouncements,
  Announcement
} from '../../api/announcements'

// 响应式数据
const loading = ref(false)
const saving = ref(false)
const announcementList = ref<Announcement[]>([])
const selectedAnnouncements = ref<Announcement[]>([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const announcementDialogVisible = ref(false)
const detailDialogVisible = ref(false)
const currentAnnouncement = ref<Announcement | null>(null)
const isEdit = ref(false)
const formRef = ref()

const announcementForm = reactive({
  id: null,
  title: '',
  content: '',
  type_: 'Info',
  priority: 5,
  enabled: true,
  start_time: '',
  end_time: ''
})

const formRules = {
  title: [
    { required: true, message: '请输入公告标题', trigger: 'blur' },
    { min: 1, max: 100, message: '标题长度在 1 到 100 个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入公告内容', trigger: 'blur' }
  ],
  type_: [
    { required: true, message: '请选择公告类型', trigger: 'change' }
  ],
  priority: [
    { required: true, message: '请设置优先级', trigger: 'blur' }
  ],
  start_time: [
    { required: true, message: '请选择开始时间', trigger: 'change' }
  ]
}

// 添加搜索和筛选变量
const searchQuery = ref('')
const filterType = ref('')
const filterStatus = ref('')

// 添加计算属性
const activeAnnouncements = computed(() => {
  return announcementList.value.filter(a => a.enabled).length
})

const urgentAnnouncements = computed(() => {
  return announcementList.value.filter(a => a.priority === 1).length
})

const scheduledAnnouncements = computed(() => {
  return announcementList.value.filter(a => a.end_time).length
})

// 筛选后的公告列表
const filteredAnnouncements = computed(() => {
  let result = announcementList.value

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(a => 
      a.title.toLowerCase().includes(query) || 
      a.content.toLowerCase().includes(query)
    )
  }

  if (filterType.value) {
    result = result.filter(a => a.type_ === filterType.value)
  }

  if (filterStatus.value === 'enabled') {
    result = result.filter(a => a.enabled)
  } else if (filterStatus.value === 'disabled') {
    result = result.filter(a => !a.enabled)
  }

  return result
})

// 方法
async function loadAnnouncements() {
  loading.value = true
  try {
    const params = {
      page: currentPage.value,
      size: pageSize.value
    }
    const response = await getAnnouncements(params)
    if (response.code === 0) {
      announcementList.value = response.data.list || []
      total.value = response.data.total || 0
    }
  } catch (error) {
    console.error('加载公告失败:', error)
    ElMessage.error('加载公告失败')
  } finally {
    loading.value = false
  }
}

function createAnnouncement() {
  isEdit.value = false
  resetForm()
  announcementDialogVisible.value = true
}

function editAnnouncement(announcement: any) {
  isEdit.value = true
  Object.assign(announcementForm, announcement)
  announcementDialogVisible.value = true
}

function viewAnnouncement(announcement: any) {
  currentAnnouncement.value = announcement
  detailDialogVisible.value = true
}

async function saveAnnouncement() {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    saving.value = true
    
    const response = isEdit.value 
      ? await updateAnnouncement(announcementForm.id, announcementForm)
      : await createAnnouncementApi(announcementForm)
    
    if (response.code === 0) {
      ElMessage.success(isEdit.value ? '公告更新成功' : '公告发布成功')
      announcementDialogVisible.value = false
      loadAnnouncements()
    } else {
      ElMessage.error(response.message || '操作失败')
    }
  } catch (error) {
    console.error('保存公告失败:', error)
    ElMessage.error('保存失败')
  } finally {
    saving.value = false
  }
}

async function deleteAnnouncement(announcement: any) {
  try {
    await ElMessageBox.confirm('确定要删除这条公告吗？', '确认删除')
    const response = await deleteAnnouncementApi(announcement.id)
    if (response.code === 0) {
      ElMessage.success('公告已删除')
      loadAnnouncements()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

async function toggleStatus(announcement: any) {
  announcement.statusLoading = true
  try {
    const response = await updateAnnouncement(announcement.id, {
      enabled: announcement.enabled
    })
    if (response.code === 0) {
      ElMessage.success(announcement.enabled ? '公告已启用' : '公告已禁用')
    } else {
      // 恢复原状态
      announcement.enabled = !announcement.enabled
      ElMessage.error('状态更新失败')
    }
  } catch (error) {
    announcement.enabled = !announcement.enabled
    ElMessage.error('状态更新失败')
  } finally {
    announcement.statusLoading = false
  }
}

async function batchEnable() {
  if (selectedAnnouncements.value.length === 0) {
    ElMessage.warning('请选择要启用的公告')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要启用选中的 ${selectedAnnouncements.value.length} 条公告吗？`, '确认操作')
    const ids = selectedAnnouncements.value.map(item => item.id)
    const response = await batchUpdateAnnouncementStatus(ids, true)
    
    if (response.code === 0) {
    ElMessage.success('批量启用成功')
    loadAnnouncements()
    } else {
      ElMessage.error(response.message || '操作失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function batchDisable() {
  if (selectedAnnouncements.value.length === 0) {
    ElMessage.warning('请选择要禁用的公告')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要禁用选中的 ${selectedAnnouncements.value.length} 条公告吗？`, '确认操作')
    const ids = selectedAnnouncements.value.map(item => item.id)
    const response = await batchUpdateAnnouncementStatus(ids, false)
    
    if (response.code === 0) {
    ElMessage.success('批量禁用成功')
    loadAnnouncements()
    } else {
      ElMessage.error(response.message || '操作失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

async function batchDelete() {
  if (selectedAnnouncements.value.length === 0) {
    ElMessage.warning('请选择要删除的公告')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedAnnouncements.value.length} 条公告吗？`, '确认删除')
    const ids = selectedAnnouncements.value.map(item => item.id)
    const response = await batchDeleteAnnouncements(ids)
    
    if (response.code === 0) {
    ElMessage.success('批量删除成功')
    loadAnnouncements()
    } else {
      ElMessage.error(response.message || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

function handleSelectionChange(selection: any[]) {
  selectedAnnouncements.value = selection
}

function handleSizeChange(size: number) {
  pageSize.value = size
  currentPage.value = 1
}

function handleCurrentChange(page: number) {
  currentPage.value = page
}

function resetForm() {
  Object.assign(announcementForm, {
    id: null,
    title: '',
    content: '',
    type_: 'Info',
    priority: 5,
    enabled: true,
    start_time: '',
    end_time: ''
  })
}

function getTypeTag(type: string): string {
  const tags = {
    Info: 'info',
    Warning: 'warning',
    Error: 'danger',
    Success: 'success'
  }
  return tags[type] || 'info'
}

function getTypeLabel(type: string): string {
  const labels = {
    Info: '信息',
    Warning: '警告',
    Error: '错误',
    Success: '成功'
  }
  return labels[type] || type
}

function getPriorityTag(priority: number): string {
  if (priority >= 8) return 'danger'
  if (priority >= 6) return 'warning'
  if (priority >= 4) return 'primary'
  return 'info'
}

function truncateText(text: string, maxLength: number): string {
  if (!text) return ''
  return text.length > maxLength ? text.substring(0, maxLength) + '...' : text
}

function formatTime(time: string): string {
  if (!time) return '-'
  return new Date(time).toLocaleString()
}

onMounted(() => {
  loadAnnouncements()
})
</script>

<style scoped>
/* AnnouncementManage页面特定样式 */

.announcement-title {
  display: flex;
  align-items: center;
  gap: 8px;
}

.title-text {
  font-weight: 500;
}

.priority-tag {
  flex-shrink: 0;
}

.announcement-content .content-text {
  margin: 0;
  color: var(--text-secondary);
}

/* 其余特定样式保持不变 */
</style> 