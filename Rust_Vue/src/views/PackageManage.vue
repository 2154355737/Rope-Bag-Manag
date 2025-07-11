<template>
  <div class="package-manage">
    <el-card>
      <h2>绳包管理</h2>
      <el-button type="primary" class="add-btn" @click="openAddDialog">新增绳包</el-button>
      <el-table :data="packages" style="width: 100%; margin-top: 18px;">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="绳包名称" label="名称" width="160" />
        <el-table-column prop="作者" label="作者" width="120" />
        <el-table-column prop="版本" label="版本" width="100" />
        <el-table-column prop="简介" label="简介" />
        <el-table-column label="操作" width="180">
          <template #default="scope">
            <el-button size="small" @click="openEditDialog(scope.row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
    <!-- 新增/编辑弹窗 -->
    <el-dialog v-model="showDialog" :title="isEdit ? '编辑绳包' : '新增绳包'" width="400px">
      <el-form label-width="70px">
        <el-form-item label="名称">
          <el-input v-model="form.绳包名称" />
        </el-form-item>
        <el-form-item label="作者">
          <el-input v-model="form.作者" />
        </el-form-item>
        <el-form-item label="版本">
          <el-input v-model="form.版本" />
        </el-form-item>
        <el-form-item label="简介">
          <el-input v-model="form.简介" />
        </el-form-item>
        <el-form-item label="直链">
          <el-input v-model="form.项目直链" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="closeDialog">取消</el-button>
        <el-button type="primary" @click="submitForm">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getPackages, addPackage, updatePackage, deletePackage } from '../api'
import { useRouter } from 'vue-router'

const packages = ref<any[]>([])
const showDialog = ref(false)
const isEdit = ref(false)
const form = ref<any>({})
const editId = ref<number|null>(null)
const router = useRouter()

onMounted(async () => {
  await loadPackages()
})

async function loadPackages() {
  try {
    const res = await getPackages()
    if (res.code === 0 && res.data) {
      packages.value = res.data.绳包列表
    }
  } catch (error) {
    console.error('加载绳包数据失败:', error)
    
    // 检查是否是网络错误或服务不可用
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED') ||
        errorMessage.includes('Service unavailable')) {
      
      // 显示服务异常提示
      ElMessage.error('服务异常已安全退出！')
      
      // 延迟跳转到登录页面
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      
      return
    }
    
    // 其他错误显示通用提示
    ElMessage.error('绳包数据加载失败，请稍后重试')
  }
}

function openAddDialog() {
  isEdit.value = false
  form.value = { 绳包名称: '', 作者: '', 版本: '', 简介: '', 项目直链: '' }
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
}

async function submitForm() {
  try {
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
    
    // 检查是否是网络错误或服务不可用
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED') ||
        errorMessage.includes('Service unavailable')) {
      
      // 显示服务异常提示
      ElMessage.error('服务异常已安全退出！')
      
      // 延迟跳转到登录页面
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      
      return
    }
    
    // 其他错误显示通用提示
    ElMessage.error('操作失败，请稍后重试')
  }
}

async function handleDelete(pkg: any) {
  if (!(await ElMessageBox.confirm('确定要删除该绳包吗？', '提示', { type: 'warning' }).catch(() => false))) return
  
  try {
    const res = await deletePackage(pkg.id)
    if (res.code === 0) {
      ElMessage.success('删除成功')
      await loadPackages()
    } else {
      ElMessage.error(res.msg || '操作失败')
    }
  } catch (error) {
    console.error('删除绳包失败:', error)
    
    // 检查是否是网络错误或服务不可用
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage.includes('fetch') || 
        errorMessage.includes('network') || 
        errorMessage.includes('Failed to fetch') ||
        errorMessage.includes('ERR_NETWORK') ||
        errorMessage.includes('ERR_CONNECTION_REFUSED') ||
        errorMessage.includes('Service unavailable')) {
      
      // 显示服务异常提示
      ElMessage.error('服务异常已安全退出！')
      
      // 延迟跳转到登录页面
      setTimeout(() => {
        router.push('/login')
      }, 2000)
      
      return
    }
    
    // 其他错误显示通用提示
    ElMessage.error('删除失败，请稍后重试')
  }
}
</script>

<style scoped>
.package-manage {
  padding: 32px;
}
.add-btn {
  margin-bottom: 16px;
}
</style> 