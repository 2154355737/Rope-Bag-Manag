<template>
  <div class="ip-ban-manage">
    <el-card class="manage-card">
      <template #header>
        <div class="card-header">
          <span>IP封禁管理</span>
          <el-button type="primary" @click="showBanDialog = true">
            <el-icon><Plus /></el-icon>
            封禁IP
          </el-button>
        </div>
      </template>

      <!-- 统计概览 -->
      <el-row :gutter="20" class="stats-overview">
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-item">
              <div class="stat-number">{{ stats.total_bans || 0 }}</div>
              <div class="stat-label">总封禁数</div>
              <div class="stat-desc">历史封禁记录总数</div>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-item">
              <div class="stat-number">{{ stats.active_bans || 0 }}</div>
              <div class="stat-label">活跃封禁</div>
              <div class="stat-desc">当前生效的封禁</div>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-number">{{ stats.recent_bans_24h || 0 }}</div>
            <div class="stat-label">24小时内</div>
            <div class="stat-desc">最近24小时新增封禁</div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="stat-card">
            <div class="stat-item">
              <div class="stat-number">{{ whitelist.length }}</div>
              <div class="stat-label">白名单IP</div>
              <div class="stat-desc">豁免封禁的IP数量</div>
            </div>
          </el-card>
        </el-col>
      </el-row>

      <!-- 搜索和过滤 -->
      <div class="search-bar">
        <el-input
          v-model="searchQuery"
          placeholder="搜索IP地址..."
          style="width: 300px; margin-right: 16px;"
          clearable
          @keyup.enter="handleSearch"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        <el-select v-model="filterType" placeholder="封禁类型" style="width: 150px; margin-right: 16px;">
          <el-option label="全部" value="all" />
          <el-option label="临时封禁" value="temporary" />
          <el-option label="永久封禁" value="permanent" />
          <el-option label="下载限制" value="download_only" />
        </el-select>
        <el-select v-model="filterStatus" placeholder="状态" style="width: 120px; margin-right: 16px;">
          <el-option label="全部" value="all" />
          <el-option label="活跃" value="active" />
          <el-option label="已解除" value="inactive" />
        </el-select>
        <el-button type="primary" @click="handleSearch">搜索</el-button>
        <el-button @click="resetSearch">重置</el-button>
      </div>

      <!-- IP封禁列表 -->
      <el-table :data="filteredBans" style="width: 100%" v-loading="loading">
        <el-table-column prop="ip_address" label="IP地址" width="150" />
        <el-table-column prop="reason" label="封禁原因" min-width="200" />
        <el-table-column prop="ban_type" label="封禁类型" width="120">
          <template #default="{ row }">
            <el-tag :type="getBanTypeColor(row.ban_type)">
              {{ getBanTypeName(row.ban_type) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="封禁时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column prop="expires_at" label="过期时间" width="180">
          <template #default="{ row }">
            <span v-if="row.expires_at">{{ formatDate(row.expires_at) }}</span>
            <span v-else class="text-muted">永久</span>
          </template>
        </el-table-column>
        <el-table-column prop="is_active" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.is_active ? 'danger' : 'success'">
              {{ row.is_active ? '活跃' : '已解除' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_by" label="操作者" width="120" />
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button 
              v-if="row.is_active" 
              size="small" 
              type="success" 
              @click="unbanIp(row.ip_address)"
            >
              解除封禁
            </el-button>
            <el-button 
              size="small" 
              type="danger" 
              @click="deleteBan(row.id)"
            >
              删除记录
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-container">
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
    </el-card>

    <!-- 封禁IP对话框 -->
    <el-dialog v-model="showBanDialog" title="封禁IP" width="500px">
      <el-form :model="banForm" :rules="banRules" ref="banFormRef" label-width="100px">
        <el-form-item label="IP地址" prop="ip_address">
          <el-input v-model="banForm.ip_address" placeholder="请输入IP地址" />
        </el-form-item>
        <el-form-item label="封禁原因" prop="reason">
          <el-input v-model="banForm.reason" type="textarea" placeholder="请输入封禁原因" />
        </el-form-item>
        <el-form-item label="封禁类型" prop="ban_type">
          <el-select v-model="banForm.ban_type" placeholder="选择封禁类型">
            <el-option label="临时封禁" value="temporary" />
            <el-option label="永久封禁" value="permanent" />
            <el-option label="下载限制" value="download_only" />
          </el-select>
        </el-form-item>
        <el-form-item label="封禁时长" prop="duration_hours" v-if="banForm.ban_type === 'temporary'">
          <el-input-number v-model="banForm.duration_hours" :min="1" :max="8760" />
          <span style="margin-left: 8px;">小时</span>
        </el-form-item>
        <el-form-item label="备注" prop="notes">
          <el-input v-model="banForm.notes" type="textarea" placeholder="可选备注信息" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showBanDialog = false">取消</el-button>
        <el-button type="primary" @click="submitBan" :loading="submitting">确认封禁</el-button>
      </template>
    </el-dialog>

    <!-- 白名单管理 -->
    <el-card class="whitelist-card">
      <template #header>
        <div class="card-header">
          <span>IP白名单管理</span>
          <el-button type="success" @click="showWhitelistDialog = true">
            <el-icon><Plus /></el-icon>
            添加白名单
          </el-button>
        </div>
      </template>

      <el-table :data="whitelist" style="width: 100%">
        <el-table-column prop="ip_address" label="IP地址" width="200" />
        <el-table-column prop="description" label="描述" min-width="200" />
        <el-table-column prop="created_at" label="添加时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column prop="created_by" label="添加者" width="120" />
        <el-table-column label="操作" width="120">
          <template #default="{ row }">
            <el-button size="small" type="danger" @click="removeFromWhitelist(row.ip_address)">
              移除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 添加白名单对话框 -->
    <el-dialog v-model="showWhitelistDialog" title="添加IP白名单" width="400px">
      <el-form :model="whitelistForm" :rules="whitelistRules" ref="whitelistFormRef" label-width="80px">
        <el-form-item label="IP地址" prop="ip_address">
          <el-input v-model="whitelistForm.ip_address" placeholder="请输入IP地址" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="whitelistForm.description" placeholder="可选描述信息" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showWhitelistDialog = false">取消</el-button>
        <el-button type="primary" @click="submitWhitelist" :loading="submitting">确认添加</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Search } from '@element-plus/icons-vue'
import { adminApi } from '@/api/admin'

// 响应式数据
const loading = ref(false)
const submitting = ref(false)
const searchQuery = ref('')
const filterType = ref('all')
const filterStatus = ref('all')
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)

const bans = ref<any[]>([])
const whitelist = ref<any[]>([])
const stats = ref<any>({})

// 对话框状态
const showBanDialog = ref(false)
const showWhitelistDialog = ref(false)

// 表单数据
const banForm = reactive({
  ip_address: '',
  reason: '',
  ban_type: 'temporary',
  duration_hours: 24,
  notes: ''
})

const whitelistForm = reactive({
  ip_address: '',
  description: ''
})

// 表单验证规则
const banRules = {
  ip_address: [
    { required: true, message: '请输入IP地址', trigger: 'blur' },
    { pattern: /^(\d{1,3}\.){3}\d{1,3}$/, message: '请输入有效的IP地址', trigger: 'blur' }
  ],
  reason: [
    { required: true, message: '请输入封禁原因', trigger: 'blur' }
  ],
  ban_type: [
    { required: true, message: '请选择封禁类型', trigger: 'change' }
  ]
}

const whitelistRules = {
  ip_address: [
    { required: true, message: '请输入IP地址', trigger: 'blur' },
    { pattern: /^(\d{1,3}\.){3}\d{1,3}$/, message: '请输入有效的IP地址', trigger: 'blur' }
  ]
}

// 表单引用
const banFormRef = ref()
const whitelistFormRef = ref()

// 计算属性
const filteredBans = computed(() => {
  let filtered = bans.value

  // 搜索过滤
  if (searchQuery.value) {
    filtered = filtered.filter(ban => 
      ban.ip_address.includes(searchQuery.value)
    )
  }

  // 类型过滤
  if (filterType.value !== 'all') {
    filtered = filtered.filter(ban => ban.ban_type === filterType.value)
  }

  // 状态过滤
  if (filterStatus.value !== 'all') {
    filtered = filtered.filter(ban => 
      filterStatus.value === 'active' ? ban.is_active : !ban.is_active
    )
  }

  return filtered
})

// 获取数据
const fetchData = async () => {
  try {
    loading.value = true
    
    // 获取封禁统计
    const statsResponse = await adminApi.getBanStats()
    if (statsResponse.code === 0) {
      stats.value = statsResponse.data
    }

    // 获取封禁列表
    const bansResponse = await adminApi.getIpBans()
    if (bansResponse.code === 0) {
      bans.value = bansResponse.data.bans || []
    }

    // 获取白名单
    const whitelistResponse = await adminApi.getIpWhitelist()
    if (whitelistResponse.code === 0) {
      whitelist.value = whitelistResponse.data.whitelist || []
    }
  } catch (error: any) {
    console.error('获取数据失败:', error)
    ElMessage.error('获取数据失败: ' + (error.message || '网络错误'))
  } finally {
    loading.value = false
  }
}

// 封禁IP
const submitBan = async () => {
  try {
    await banFormRef.value.validate()
    submitting.value = true

    const response = await adminApi.banIp({
      ip_address: banForm.ip_address,
      reason: banForm.reason,
      ban_type: banForm.ban_type,
      duration_hours: banForm.ban_type === 'temporary' ? banForm.duration_hours : undefined,
      notes: banForm.notes
    })

    if (response.code === 0) {
      ElMessage.success('IP封禁成功')
      showBanDialog.value = false
      resetBanForm()
      fetchData()
    } else {
      ElMessage.error(response.message || '封禁失败')
    }
  } catch (error: any) {
    console.error('封禁失败:', error)
    ElMessage.error('封禁失败: ' + (error.message || '网络错误'))
  } finally {
    submitting.value = false
  }
}

// 解除封禁
const unbanIp = async (ipAddress: string) => {
  try {
    await ElMessageBox.confirm(
      `确定要解除IP ${ipAddress} 的封禁吗？`,
      '确认解除封禁',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    const response = await adminApi.unbanIp({ ip_address: ipAddress })
    if (response.code === 0) {
      ElMessage.success('IP封禁解除成功')
      fetchData()
    } else {
      ElMessage.error(response.message || '解除封禁失败')
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('解除封禁失败:', error)
      ElMessage.error('解除封禁失败: ' + (error.message || '网络错误'))
    }
  }
}

// 删除封禁记录
const deleteBan = async (banId: number) => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这条封禁记录吗？此操作不可恢复。',
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    const response = await adminApi.deleteIpBan(banId)
    if (response.code === 0) {
      ElMessage.success('封禁记录删除成功')
      fetchData()
    } else {
      ElMessage.error(response.message || '删除失败')
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('删除失败:', error)
      ElMessage.error('删除失败: ' + (error.message || '网络错误'))
    }
  }
}

// 添加白名单
const submitWhitelist = async () => {
  try {
    await whitelistFormRef.value.validate()
    submitting.value = true

    const response = await adminApi.addIpToWhitelist({
      ip_address: whitelistForm.ip_address,
      description: whitelistForm.description
    })

    if (response.code === 0) {
      ElMessage.success('IP已添加到白名单')
      showWhitelistDialog.value = false
      resetWhitelistForm()
      fetchData()
    } else {
      ElMessage.error(response.message || '添加失败')
    }
  } catch (error: any) {
    console.error('添加失败:', error)
    ElMessage.error('添加失败: ' + (error.message || '网络错误'))
  } finally {
    submitting.value = false
  }
}

// 移除白名单
const removeFromWhitelist = async (ipAddress: string) => {
  try {
    await ElMessageBox.confirm(
      `确定要从白名单中移除IP ${ipAddress} 吗？`,
      '确认移除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    const response = await adminApi.removeIpFromWhitelist({ ip_address: ipAddress })
    if (response.code === 0) {
      ElMessage.success('IP已从白名单移除')
      fetchData()
    } else {
      ElMessage.error(response.message || '移除失败')
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('移除失败:', error)
      ElMessage.error('移除失败: ' + (error.message || '网络错误'))
    }
  }
}

// 搜索和重置
const handleSearch = () => {
  currentPage.value = 1
  // 这里可以添加实际的搜索逻辑
}

const resetSearch = () => {
  searchQuery.value = ''
  filterType.value = 'all'
  filterStatus.value = 'all'
  currentPage.value = 1
  handleSearch()
}

// 分页处理
const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  handleSearch()
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
  handleSearch()
}

// 重置表单
const resetBanForm = () => {
  banForm.ip_address = ''
  banForm.reason = ''
  banForm.ban_type = 'temporary'
  banForm.duration_hours = 24
  banForm.notes = ''
  banFormRef.value?.resetFields()
}

const resetWhitelistForm = () => {
  whitelistForm.ip_address = ''
  whitelistForm.description = ''
  whitelistFormRef.value?.resetFields()
}

// 工具函数
const formatDate = (dateStr: string) => {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleString('zh-CN')
}

const getBanTypeName = (type: string) => {
  const typeMap: { [key: string]: string } = {
    'temporary': '临时封禁',
    'permanent': '永久封禁',
    'download_only': '下载限制'
  }
  return typeMap[type] || type
}

const getBanTypeColor = (type: string) => {
  const colorMap: { [key: string]: string } = {
    'temporary': 'warning',
    'permanent': 'danger',
    'download_only': 'info'
  }
  return colorMap[type] || 'default'
}

// 组件挂载时获取数据
onMounted(() => {
  fetchData()
})
</script>

<style scoped>
.ip-ban-manage {
  padding: 20px;
}

.manage-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.stats-overview {
  margin-bottom: 20px;
}

.stat-card {
  text-align: center;
}

.stat-item {
  padding: 10px;
}

.stat-number {
  font-size: 24px;
  font-weight: bold;
  color: #409EFF;
  margin-bottom: 5px;
}

.stat-label {
  font-size: 14px;
  color: #666;
  margin-bottom: 4px;
}

.stat-desc {
  font-size: 12px;
  color: #999;
  line-height: 1.2;
}

.search-bar {
  margin-bottom: 20px;
  display: flex;
  align-items: center;
}

.pagination-container {
  margin-top: 20px;
  text-align: right;
}

.whitelist-card {
  margin-top: 20px;
}

.text-muted {
  color: #999;
}
</style> 