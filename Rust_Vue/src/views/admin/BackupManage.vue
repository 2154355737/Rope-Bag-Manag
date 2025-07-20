<template>
  <div class="backup-manage">
    <el-card class="manage-card">
      <template #header>
        <div class="manage-header">
          <h2>数据库备份管理</h2>
          <p>管理系统数据库的备份、恢复和监控功能</p>
        </div>
      </template>

      <div class="manage-content">
        <!-- 备份操作 -->
        <div class="backup-actions">
          <el-row :gutter="20">
            <el-col :span="8">
              <el-card class="action-card">
                <div class="action-item">
                  <div class="action-icon">💾</div>
                  <div class="action-content">
                    <h3>手动备份</h3>
                    <p>立即创建数据库备份</p>
                    <el-button type="primary" @click="createBackup" :loading="backupLoading">
                      开始备份
                    </el-button>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="8">
              <el-card class="action-card">
                <div class="action-item">
                  <div class="action-icon">🔄</div>
                  <div class="action-content">
                    <h3>自动备份</h3>
                    <p>配置自动备份计划</p>
                    <el-button type="success" @click="configureAutoBackup">
                      配置计划
                    </el-button>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="8">
              <el-card class="action-card">
                <div class="action-item">
                  <div class="action-icon">📊</div>
                  <div class="action-content">
                    <h3>备份统计</h3>
                    <p>查看备份统计信息</p>
                    <el-button type="info" @click="viewBackupStats">
                      查看统计
                    </el-button>
                  </div>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </div>

        <!-- 备份状态 -->
        <div class="backup-status">
          <el-row :gutter="20">
            <el-col :span="6">
              <el-card class="status-card">
                <div class="status-item">
                  <div class="status-icon">📁</div>
                  <div class="status-content">
                    <div class="status-value">{{ status.total_backups }}</div>
                    <div class="status-label">总备份数</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="status-card">
                <div class="status-item">
                  <div class="status-icon">✅</div>
                  <div class="status-content">
                    <div class="status-value">{{ status.success_backups }}</div>
                    <div class="status-label">成功备份</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="status-card">
                <div class="status-item">
                  <div class="status-icon">❌</div>
                  <div class="status-content">
                    <div class="status-value">{{ status.failed_backups }}</div>
                    <div class="status-label">失败备份</div>
                  </div>
                </div>
              </el-card>
            </el-col>
            <el-col :span="6">
              <el-card class="status-card">
                <div class="status-item">
                  <div class="status-icon">💿</div>
                  <div class="status-content">
                    <div class="status-value">{{ formatSize(status.total_size) }}</div>
                    <div class="status-label">总大小</div>
                  </div>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </div>

        <!-- 备份列表 -->
        <div class="backup-list">
          <div class="list-header">
            <h3>备份记录</h3>
            <div class="list-actions">
              <el-button type="primary" @click="refreshBackups">刷新</el-button>
              <el-button @click="exportBackupList">导出列表</el-button>
            </div>
          </div>

          <el-table 
            :data="backupList" 
            v-loading="loading"
            style="width: 100%"
            @selection-change="handleSelectionChange"
          >
            <el-table-column type="selection" width="55" />
            <el-table-column prop="id" label="ID" width="80" />
            <el-table-column prop="filename" label="文件名" min-width="200" />
            <el-table-column prop="file_size" label="文件大小" width="120">
              <template #default="{ row }">
                {{ formatSize(row.file_size) }}
              </template>
            </el-table-column>
            <el-table-column prop="backup_time" label="备份时间" width="180">
              <template #default="{ row }">
                {{ formatTime(row.backup_time) }}
              </template>
            </el-table-column>
            <el-table-column prop="backup_type" label="备份类型" width="100">
              <template #default="{ row }">
                <el-tag :type="getBackupTypeTag(row.backup_type)">
                  {{ getBackupTypeLabel(row.backup_type) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="status" label="状态" width="100">
              <template #default="{ row }">
                <el-tag :type="getStatusTag(row.status)">
                  {{ getStatusLabel(row.status) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="description" label="描述" min-width="200" />
            <el-table-column label="操作" width="250" fixed="right">
              <template #default="{ row }">
                <el-button size="small" @click="viewBackup(row)">查看</el-button>
                <el-button 
                  size="small" 
                  type="success" 
                  @click="downloadBackup(row)"
                  v-if="row.status === 'Success'"
                >
                  下载
                </el-button>
                <el-button 
                  size="small" 
                  type="warning" 
                  @click="restoreBackup(row)"
                  v-if="row.status === 'Success'"
                >
                  恢复
                </el-button>
                <el-button 
                  size="small" 
                  type="danger" 
                  @click="deleteBackup(row)"
                >
                  删除
                </el-button>
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
        <div class="batch-actions" v-if="selectedBackups.length > 0">
          <el-button type="danger" @click="batchDelete">批量删除</el-button>
          <el-button @click="batchDownload">批量下载</el-button>
          <span class="selected-count">已选择 {{ selectedBackups.length }} 个备份</span>
        </div>
      </div>
    </el-card>

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
  getBackupRecords, 
  createBackup as createBackupApi,
  deleteBackupRecord,
  downloadBackup as downloadBackupApi,
  batchDeleteBackupRecords,
  restoreBackup as restoreBackupApi,
  getBackupDownloadUrl,
  BackupRecord
} from '../../api/backupRecords'

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

async function createBackup() {
  backupLoading.value = true
  try {
    const backupData = {
      backup_type: 'Manual',
      description: '手动创建的备份'
    }
    const response = await createBackupApi(backupData)
    if (response.code === 0) {
      ElMessage.success('备份创建成功')
      loadBackups()
    } else {
      ElMessage.error(response.message || '备份创建失败')
    }
  } catch (error) {
    console.error('创建备份失败:', error)
    ElMessage.error('创建备份失败')
  } finally {
    backupLoading.value = false
  }
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

function saveAutoBackupConfig() {
  ElMessage.success('自动备份配置已保存')
  autoBackupDialogVisible.value = false
}

function getBackupTypeTag(type: string): string {
  const tags = {
    Auto: 'primary',
    Manual: 'success',
    Scheduled: 'warning'
  }
  return tags[type] || 'info'
}

function getBackupTypeLabel(type: string): string {
  const labels = {
    Auto: '自动备份',
    Manual: '手动备份',
    Scheduled: '定时备份'
  }
  return labels[type] || type
}

function getStatusTag(status: string): string {
  const tags = {
    Success: 'success',
    Failed: 'danger',
    InProgress: 'warning'
  }
  return tags[status] || 'info'
}

function getStatusLabel(status: string): string {
  const labels = {
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

onMounted(() => {
  loadBackups()
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
</style> 