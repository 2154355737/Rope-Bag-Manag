<template>
  <div class="admin-page backup-manage">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><DataAnalysis /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">数据库备份管理</h1>
            <p class="page-subtitle">管理系统数据库的备份、恢复和监控功能</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button type="primary" @click="createBackup" :loading="backupLoading">
            <el-icon><Upload /></el-icon>
            开始备份
          </el-button>
          <el-button @click="refreshData">
            <el-icon><Refresh /></el-icon>
            刷新
          </el-button>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Folder /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ status.total_backups }}</div>
            <div class="stat-label">总备份数</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Check /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ status.success_backups }}</div>
            <div class="stat-label">成功备份</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Close /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ status.failed_backups }}</div>
            <div class="stat-label">失败备份</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon :size="24"><Odometer /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-number">{{ formatSize(Number(status.total_size)) }}</div>
            <div class="stat-label">存储空间</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 备份操作区域 -->
    <div class="content-section backup-operations">
      <h2 class="section-title">备份操作</h2>
      <div class="operations-grid">
        <el-card class="operation-card" shadow="hover">
          <div class="operation-icon">
            <el-icon :size="32"><Timer /></el-icon>
          </div>
          <h3>定时备份</h3>
          <p>配置自动备份计划</p>
          <div class="operation-control">
            <el-switch
              v-model="autoBackupEnabled"
              active-text="已启用"
              inactive-text="已禁用"
              @change="toggleAutoBackup"
            />
          </div>
          <div v-if="autoBackupEnabled" class="schedule-info">
            <p>当前计划: {{ autoBackupSchedule }}</p>
          </div>
          <el-button type="success" @click="configureAutoBackup">
            <el-icon><SetUp /></el-icon>
            配置计划
          </el-button>
        </el-card>

        <el-card class="operation-card" shadow="hover">
          <div class="operation-icon">
            <el-icon :size="32"><Download /></el-icon>
          </div>
          <h3>导出备份</h3>
          <p>将备份导出到本地</p>
          <el-button type="primary" @click="exportBackup">
            <el-icon><Download /></el-icon>
            导出备份
          </el-button>
        </el-card>

        <el-card class="operation-card" shadow="hover">
          <div class="operation-icon">
            <el-icon :size="32"><Upload /></el-icon>
          </div>
          <h3>导入备份</h3>
          <p>从本地导入备份文件</p>
          <el-button type="warning" @click="importBackup">
            <el-icon><Upload /></el-icon>
            导入备份
          </el-button>
        </el-card>

        <el-card class="operation-card" shadow="hover">
          <div class="operation-icon">
            <el-icon :size="32"><Delete /></el-icon>
          </div>
          <h3>清理备份</h3>
          <p>删除过期或不必要的备份</p>
          <el-button type="danger" @click="cleanupBackups">
            <el-icon><Delete /></el-icon>
            清理备份
          </el-button>
        </el-card>
      </div>
    </div>

    <!-- 备份列表 -->
    <div class="table-section">
      <h2 class="section-title">备份记录</h2>
      <el-table 
        :data="backupList" 
        v-loading="loading"
        style="width: 100%"
        :header-cell-style="{ background: 'var(--bg-secondary)', color: 'var(--text-primary)' }"
        :row-style="{ background: 'var(--bg-card)' }"
        border
        stripe
      >
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="filename" label="文件名" min-width="200" />
        <el-table-column prop="type" label="类型" width="120">
          <template #default="{ row }">
            <el-tag :type="getBackupTypeTag(row.type)">
              {{ getBackupTypeLabel(row.type) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="size" label="大小" width="120">
          <template #default="{ row }">
            {{ formatSize(row.size) }}
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="120">
          <template #default="{ row }">
            <el-tag :type="getStatusTag(row.status)">
              {{ getStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="创建时间" width="180">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="250" fixed="right">
          <template #default="{ row }">
            <el-button 
              size="small" 
              type="primary" 
              @click="downloadBackup(row)"
              :disabled="row.status !== 'Success'"
            >
              <el-icon><Download /></el-icon>
              下载
            </el-button>
            <el-button 
              size="small" 
              type="warning" 
              @click="restoreBackup(row)"
              :disabled="row.status !== 'Success'"
            >
              <el-icon><RefreshRight /></el-icon>
              恢复
            </el-button>
            <el-button size="small" type="danger" @click="deleteBackup(row)">
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
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </div>

    <!-- 批量操作 -->
    <div class="batch-actions" v-if="selectedBackups.length > 0">
      <el-button type="danger" @click="batchDelete">批量删除</el-button>
      <el-button @click="batchDownload">批量下载</el-button>
      <span class="selected-count">已选择 {{ selectedBackups.length }} 个备份</span>
    </div>

    <!-- 备份详情对话框 -->
    <el-dialog 
      v-model="backupDialogVisible" 
      title="备份详情" 
      width="600px"
    >
      <div class="backup-detail" v-if="currentBackup">
        <div class="detail-item">
          <label>备份ID:</label>
          <span>{{ currentBackup.id }}</span>
        </div>
        <div class="detail-item">
          <label>文件名:</label>
          <span>{{ currentBackup.filename }}</span>
        </div>
        <div class="detail-item">
          <label>文件大小:</label>
          <span>{{ formatSize(currentBackup.file_size) }}</span>
        </div>
        <div class="detail-item">
          <label>备份时间:</label>
          <span>{{ formatTime(currentBackup.backup_time) }}</span>
        </div>
        <div class="detail-item">
          <label>备份类型:</label>
          <el-tag :type="getBackupTypeTag(currentBackup.backup_type)">
            {{ getBackupTypeLabel(currentBackup.backup_type) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>状态:</label>
          <el-tag :type="getStatusTag(currentBackup.status)">
            {{ getStatusLabel(currentBackup.status) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>文件路径:</label>
          <span>{{ currentBackup.file_path }}</span>
        </div>
        <div class="detail-item">
          <label>描述:</label>
          <div class="description-box">{{ currentBackup.description }}</div>
        </div>
      </div>
      <template #footer>
        <el-button @click="backupDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- 自动备份配置对话框 -->
    <el-dialog 
      v-model="autoBackupDialogVisible" 
      title="自动备份配置" 
      width="500px"
    >
      <el-form :model="autoBackupConfig" label-width="120px">
        <el-form-item label="启用自动备份">
          <el-switch v-model="autoBackupConfig.enable_auto_backup" />
        </el-form-item>
        <el-form-item label="备份间隔">
          <el-input-number 
            v-model="autoBackupConfig.backup_interval_hours" 
            :min="1" 
            :max="168" 
            placeholder="小时"
            :disabled="!autoBackupConfig.enable_auto_backup"
          />
          <span class="config-desc">备份间隔时间（小时）</span>
        </el-form-item>
        <el-form-item label="备份位置">
          <el-input 
            v-model="autoBackupConfig.backup_location" 
            placeholder="备份文件存储路径"
            :disabled="!autoBackupConfig.enable_auto_backup"
          />
        </el-form-item>
        <el-form-item label="最大备份文件">
          <el-input-number 
            v-model="autoBackupConfig.max_backup_files" 
            :min="1" 
            :max="100" 
            placeholder="个"
            :disabled="!autoBackupConfig.enable_auto_backup"
          />
          <span class="config-desc">保留的最大备份文件数量</span>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="autoBackupDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveAutoBackupConfig">保存配置</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  DataAnalysis, 
  Folder, 
  Check, 
  Close, 
  Odometer, 
  Timer, 
  Upload, 
  Download, 
  Delete, 
  Refresh, 
  RefreshRight, 
  SetUp
} from '@element-plus/icons-vue'
import { 
  getBackupRecords, 
  createBackup as createBackupApi,
  deleteBackupRecord,
  downloadBackup as downloadBackupApi,
  batchDeleteBackupRecords,
  restoreBackup as restoreBackupApi,
  getBackupDownloadUrl,
  BackupRecord,
  getBackupStats,
  configureAutoBackup as configAutoBackupApi,
  getAutoBackupConfig
} from '../../api/backupRecords'
import { uploadFile } from '@/utils/apiClient'

// 响应式数据
const loading = ref(false)
const backupLoading = ref(false)
const backupList = ref<BackupRecord[]>([])
const selectedBackups = ref<BackupRecord[]>([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const backupDialogVisible = ref(false)
const autoBackupDialogVisible = ref(false)
const currentBackup = ref<BackupRecord | null>(null)

const status = reactive({
  total_backups: 0,
  success_backups: 0,
  failed_backups: 0,
  total_size: 0
})

const autoBackupConfig = reactive({
  enable_auto_backup: false,
  backup_interval_hours: 24,
  backup_location: './backups',
  max_backup_files: 10
})

const refreshData = async () => {
  await Promise.all([loadBackups(), loadStats()])
}

const loadStats = async () => {
  const res = await getBackupStats()
  if (res.code === 0) {
    Object.assign(status, res.data)
  }
}

const autoBackupEnabled = ref(false);
const toggleAutoBackup = async () => {
  try {
    // 切换自动备份状态
  autoBackupConfig.enable_auto_backup = autoBackupEnabled.value
    
    // 构建API请求参数
    const config = {
      enabled: autoBackupConfig.enable_auto_backup,
      frequency: 'hourly',
      time: '00:00',
      retain_count: autoBackupConfig.max_backup_files,
      auto_clean: true
    }
    
    const result = await configAutoBackupApi(config)
    
    if (result.code === 0) {
      // 更新本地状态
      autoBackupSchedule.value = autoBackupConfig.enable_auto_backup 
        ? `${autoBackupConfig.backup_interval_hours}h` 
        : '已禁用'
      
      ElMessage.success(autoBackupConfig.enable_auto_backup ? '自动备份已启用' : '自动备份已禁用')
    } else {
      // 如果保存失败，回滚状态
      autoBackupEnabled.value = !autoBackupEnabled.value
      autoBackupConfig.enable_auto_backup = autoBackupEnabled.value
      ElMessage.error(result.message || '配置保存失败')
    }
  } catch (error) {
    console.error('切换自动备份状态失败:', error)
    // 如果出错，回滚状态
    autoBackupEnabled.value = !autoBackupEnabled.value
    autoBackupConfig.enable_auto_backup = autoBackupEnabled.value
    ElMessage.error('操作失败，请重试')
  }
}

const autoBackupSchedule = ref('');
const exportBackup = async () => {
  await createBackup()
  ElMessage.info('备份创建后可在列表下载')
}

async function createBackup() {
  backupLoading.value = true
  try {
    const res = await createBackupApi({ backup_type: 'Manual' })
    if (res.code === 0) {
      ElMessage.success('备份创建成功')
      refreshData()
    }
  } catch (e) {
    ElMessage.error('创建备份失败')
  } finally { backupLoading.value=false }
}

const importBackup = () => {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.db'
  input.onchange = async () => {
    if (!input.files?.length) return
    const form = new FormData()
    form.append('file', input.files[0])
    const res = await uploadFile('/api/v1/admin/backup/upload', form)
    if (res.code === 0) {
      ElMessage.success('导入成功')
      refreshData()
    }
  }
  input.click()
}

const cleanupBackups = async () => {
  await ElMessageBox.confirm('确定清理所有失败备份记录吗?', '提示')
  const failIds = backupList.value.filter(b=>b.status!=='Success').map(b=>b.id)
  if (failIds.length===0) {ElMessage.info('无失败备份');return}
  const res = await batchDeleteBackupRecords(failIds)
  if (res.code===0) {
    ElMessage.success('已清理')
    refreshData()
  }
}

// 方法
async function loadBackups() {
  loading.value = true
  try {
    const params = {
      page: currentPage.value,
      size: pageSize.value
    }
    const response = await getBackupRecords(params)
    if (response.code === 0) {
      backupList.value = response.data.list || []
      total.value = response.data.total || 0
      updateStatus()
    }
  } catch (error) {
    console.error('加载备份记录失败:', error)
    ElMessage.error('加载备份记录失败')
  } finally {
    loading.value = false
  }
}

function updateStatus() {
  status.total_backups = total.value
  status.success_backups = backupList.value.filter(b => b.status === 'Success').length
  status.failed_backups = backupList.value.filter(b => b.status === 'Failed').length
  status.total_size = backupList.value.reduce((sum, b) => sum + (b.file_size || 0), 0)
}

function configureAutoBackup() {
  autoBackupDialogVisible.value = true
}

function viewBackupStats() {
  ElMessage.info('备份统计功能开发中...')
}

function refreshBackups() {
  loadBackups()
}

function exportBackupList() {
  ElMessage.info('导出功能开发中...')
}

function handleSelectionChange(selection: BackupRecord[]) {
  selectedBackups.value = selection
}

function handleSizeChange(size: number) {
  pageSize.value = size
  loadBackups()
}

function handleCurrentChange(page: number) {
  currentPage.value = page
  loadBackups()
}

function viewBackup(backup: BackupRecord) {
  currentBackup.value = backup
  backupDialogVisible.value = true
}

async function downloadBackup(backup: BackupRecord) {
  try {
    // 创建一个隐藏的下载链接
    const link = document.createElement('a')
    link.href = `http://127.0.0.1:15201${getBackupDownloadUrl(backup.id)}`
    link.download = backup.filename
    link.style.display = 'none'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    
    ElMessage.success('下载已开始')
  } catch (error) {
    ElMessage.error('下载失败')
  }
}

async function restoreBackup(backup: BackupRecord) {
  try {
    await ElMessageBox.confirm(
      '确定要恢复这个备份吗？这将覆盖当前数据库！',
      '确认恢复',
      { type: 'warning' }
    )
    
    const response = await restoreBackupApi(backup.id)
    if (response.code === 0) {
      ElMessage.success('备份恢复成功')
      loadBackups()
    } else {
      ElMessage.error(response.message || '恢复失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('恢复失败')
    }
  }
}

async function deleteBackup(backup: BackupRecord) {
  try {
    await ElMessageBox.confirm('确定要删除这个备份吗？', '确认删除')
    const response = await deleteBackupRecord(backup.id)
    if (response.code === 0) {
      ElMessage.success('备份已删除')
      loadBackups()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

async function batchDelete() {
  if (selectedBackups.value.length === 0) {
    ElMessage.warning('请选择要删除的备份')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedBackups.value.length} 个备份吗？`, '确认删除')
    
    const ids = selectedBackups.value.map(backup => backup.id)
    const response = await batchDeleteBackupRecords(ids)
    
    if (response.code === 0) {
      ElMessage.success('批量删除成功')
      selectedBackups.value = []
      loadBackups()
    } else {
      ElMessage.error(response.message || '批量删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

function batchDownload() {
  ElMessage.info('批量下载功能开发中...')
}

async function saveAutoBackupConfig() {
  try {
    // 构建API请求参数，将前端的配置结构转换为后端期望的格式
    const config = {
      enabled: autoBackupConfig.enable_auto_backup,
      frequency: 'hourly', // 根据间隔小时数设定频率
      time: '00:00',       // 默认时间
      retain_count: autoBackupConfig.max_backup_files,
      auto_clean: true
    }
    
    const result = await configAutoBackupApi(config)
    
    if (result.code === 0) {
  ElMessage.success('自动备份配置已保存')
      
      // 更新本地状态
      autoBackupEnabled.value = autoBackupConfig.enable_auto_backup
      autoBackupSchedule.value = `${autoBackupConfig.backup_interval_hours}h`
      
  autoBackupDialogVisible.value = false
    } else {
      ElMessage.error(result.message || '保存配置失败')
    }
  } catch (error) {
    console.error('保存自动备份配置失败:', error)
    ElMessage.error('保存配置时发生错误')
  }
}

function getBackupTypeTag(type: string): string {
  const tags: { [key: string]: string } = {
    Auto: 'primary',
    Manual: 'success',
    Scheduled: 'warning'
  }
  return tags[type] || 'info'
}

function getBackupTypeLabel(type: string): string {
  const labels: { [key: string]: string } = {
    Auto: '自动备份',
    Manual: '手动备份',
    Scheduled: '定时备份'
  }
  return labels[type] || type
}

function getStatusTag(status: string): string {
  const tags: { [key: string]: string } = {
    Success: 'success',
    Failed: 'danger',
    InProgress: 'warning'
  }
  return tags[status] || 'info'
}

function getStatusLabel(status: string): string {
  const labels: { [key: string]: string } = {
    Success: '成功',
    Failed: '失败',
    InProgress: '进行中'
  }
  return labels[status] || status
}

function formatSize(bytes: number): string {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function formatTime(time: string): string {
  if (!time) return '-'
  return new Date(time).toLocaleString()
}

// 加载自动备份配置
async function loadAutoBackupConfig() {
  try {
  const cfg = await getAutoBackupConfig()
    if (cfg.code === 0 && cfg.data) {
      // 将后端配置映射到前端结构
      Object.assign(autoBackupConfig, {
        enable_auto_backup: cfg.data.enabled || false,
        backup_interval_hours: 24, // 默认24小时，可根据frequency调整
        backup_location: cfg.data.location || 'backups/',
        max_backup_files: cfg.data.retain_count || 10
      })
      
    autoBackupEnabled.value = autoBackupConfig.enable_auto_backup
    autoBackupSchedule.value = `${autoBackupConfig.backup_interval_hours}h`
  }
  } catch (error) {
    console.error('加载自动备份配置失败:', error)
    // 使用默认配置
    Object.assign(autoBackupConfig, {
      enable_auto_backup: false,
      backup_interval_hours: 24,
      backup_location: 'backups/',
      max_backup_files: 10
    })
  }
}

onMounted(async () => {
  await refreshData()
  await loadAutoBackupConfig()
})
</script>

<style scoped>
.backup-manage {
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

.backup-actions {
  margin-bottom: 30px;
}

.action-card {
  height: 100%;
}

.action-item {
  display: flex;
  align-items: center;
  gap: 15px;
  padding: 20px;
}

.action-icon {
  font-size: 32px;
}

.action-content {
  flex: 1;
}

.action-content h3 {
  margin: 0 0 8px 0;
  color: var(--el-text-color-primary);
}

.action-content p {
  margin: 0 0 15px 0;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.backup-status {
  margin-bottom: 30px;
}

.status-card {
  text-align: center;
}

.status-item {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15px;
  padding: 20px;
}

.status-icon {
  font-size: 24px;
}

.status-content {
  text-align: left;
}

.status-value {
  font-size: 24px;
  font-weight: bold;
  color: var(--el-text-color-primary);
}

.status-label {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.backup-list {
  margin-bottom: 20px;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.list-header h3 {
  margin: 0;
  color: var(--el-text-color-primary);
}

.list-actions {
  display: flex;
  gap: 10px;
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

.backup-detail {
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

.description-box {
  flex: 1;
  padding: 10px;
  background: var(--el-bg-color-page);
  border-radius: 4px;
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
}

.config-desc {
  margin-left: 10px;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

/* 备份管理页特定样式 */
.operations-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 20px;
  margin-bottom: 30px;
}

.operation-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 20px;
}

.operation-icon {
  margin-bottom: 15px;
  color: var(--brand-color);
}

.operation-card h3 {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 8px 0;
  color: var(--text-primary);
}

.operation-card p {
  margin: 0 0 15px 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.operation-control {
  margin: 15px 0;
}

.schedule-info {
  margin-bottom: 15px;
  font-size: 13px;
  color: var(--text-secondary);
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 20px;
  color: var(--text-primary);
}

@media (max-width: 1200px) {
  .operations-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .operations-grid {
    grid-template-columns: 1fr;
  }
}
</style> 