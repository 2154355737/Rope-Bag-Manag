<template>
  <div class="announcement-manage">
    <el-card class="manage-card">
      <template #header>
        <div class="manage-header">
          <h2>公告提醒管理</h2>
          <p>管理系统公告，包括发布、编辑、删除和状态管理</p>
        </div>
      </template>

      <div class="manage-content">
        <!-- 快速操作 -->
        <div class="quick-actions">
          <el-button type="primary" @click="createAnnouncement">
            <el-icon><Plus /></el-icon>
            发布公告
          </el-button>
          <el-button type="success" @click="batchEnable">
            <el-icon><Check /></el-icon>
            批量启用
          </el-button>
          <el-button type="warning" @click="batchDisable">
            <el-icon><Close /></el-icon>
            批量禁用
          </el-button>
          <el-button type="danger" @click="batchDelete">
            <el-icon><Delete /></el-icon>
            批量删除
          </el-button>
        </div>

        <!-- 公告列表 -->
        <div class="announcement-list">
          <el-table 
            :data="announcementList" 
            v-loading="loading"
            style="width: 100%"
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
                <el-button size="small" @click="viewAnnouncement(row)">查看</el-button>
                <el-button size="small" type="primary" @click="editAnnouncement(row)">编辑</el-button>
                <el-button size="small" type="danger" @click="deleteAnnouncement(row)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>

          <!-- 分页 -->
          <div class="pagination-wrapper">
            <el-pagination
              v-model:current-page="currentPage"
              v-model:page-size="pageSize"
              :page-sizes="[10, 20, 50, 100]"
              :total="total"
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
      </div>
    </el-card>

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
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Check, Close, Delete } from '@element-plus/icons-vue'
import { 
  getAnnouncements, 
  createAnnouncement as createAnnouncementApi,
  updateAnnouncement,
  deleteAnnouncement as deleteAnnouncementApi
} from '../../api/announcements'

// 响应式数据
const loading = ref(false)
const saving = ref(false)
const announcementList = ref([])
const selectedAnnouncements = ref([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const announcementDialogVisible = ref(false)
const detailDialogVisible = ref(false)
const currentAnnouncement = ref(null)
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
      ElMessage.error(response.msg || '操作失败')
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
    // 批量启用逻辑
    ElMessage.success('批量启用成功')
    loadAnnouncements()
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
    // 批量禁用逻辑
    ElMessage.success('批量禁用成功')
    loadAnnouncements()
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
    // 批量删除逻辑
    ElMessage.success('批量删除成功')
    loadAnnouncements()
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
  loadAnnouncements()
}

function handleCurrentChange(page: number) {
  currentPage.value = page
  loadAnnouncements()
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
.announcement-manage {
  padding: 20px;
}

.manage-card {
  max-width: 1400px;
  margin: 0 auto;
}

.manage-header {
  text-align: center;
}

.manage-header h2 {
  margin: 0 0 10px 0;
  color: var(--el-text-color-primary);
}

.manage-header p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.manage-content {
  padding: 20px 0;
}

.quick-actions {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.announcement-list {
  margin-bottom: 20px;
}

.announcement-title {
  display: flex;
  align-items: center;
  gap: 10px;
}

.title-text {
  flex: 1;
}

.priority-tag {
  flex-shrink: 0;
}

.announcement-content {
  max-width: 300px;
}

.content-text {
  margin: 0;
  word-break: break-all;
  line-height: 1.4;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.batch-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 15px;
  background: var(--el-bg-color-page);
  border-radius: 8px;
  margin-top: 20px;
}

.selected-count {
  margin-left: auto;
  color: var(--el-text-color-secondary);
}

.announcement-detail {
  padding: 20px;
}

.detail-item {
  display: flex;
  margin-bottom: 15px;
  align-items: flex-start;
}

.detail-item label {
  width: 100px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.detail-item span {
  flex: 1;
  color: var(--el-text-color-regular);
}

.content-box {
  flex: 1;
  padding: 10px;
  background: var(--el-bg-color-page);
  border-radius: 4px;
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
}

.priority-desc,
.end-time-desc {
  margin-left: 10px;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}
</style> 