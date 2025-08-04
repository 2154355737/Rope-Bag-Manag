<template>
  <div class="tag-manage">
    <el-card>
      <template #header>
        <div class="header">
          <el-input
            v-model="search"
            placeholder="搜索标签"
            clearable
            style="max-width: 220px"
            @change="loadTags"
          />
          <div class="header-buttons">
            <el-button @click="refreshTags">刷新</el-button>
            <el-button type="primary" @click="showAddDialog">新增标签</el-button>
          </div>
        </div>
      </template>

      <el-table :data="tags" style="width: 100%" :loading="loading">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="名称" />
        <el-table-column prop="description" label="描述" />
        <el-table-column prop="color" label="颜色" width="120">
          <template #default="{ row }">
            <div v-if="row.color" :style="{ background: row.color, width: '24px', height: '24px', borderRadius: '4px' }" />
          </template>
        </el-table-column>
        <el-table-column prop="use_count" label="使用次数" width="120" />
        <el-table-column prop="created_at" label="创建时间" />
        <el-table-column label="操作" width="180">
          <template #default="{ row }">
            <el-button size="small" @click="editTag(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="deleteTag(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 新增标签对话框 -->
    <el-dialog v-model="addDialogVisible" title="新增标签">
      <el-form ref="addFormRef" :model="newTag" :rules="rules" label-width="80px">
        <el-form-item label="名称" prop="name">
          <el-input v-model="newTag.name" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="newTag.description" />
        </el-form-item>
        <el-form-item label="颜色" prop="color">
          <el-input v-model="newTag.color" placeholder="#409EFF" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="cancelAdd">取消</el-button>
        <el-button type="primary" @click="handleAdd">确定</el-button>
      </template>
    </el-dialog>

    <!-- 编辑标签对话框 -->
    <el-dialog v-model="editDialogVisible" title="编辑标签">
      <el-form ref="editFormRef" :model="editTagData" :rules="rules" label-width="80px">
        <el-form-item label="名称" prop="name">
          <el-input v-model="editTagData.name" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="editTagData.description" />
        </el-form-item>
        <el-form-item label="颜色" prop="color">
          <el-input v-model="editTagData.color" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="cancelEdit">取消</el-button>
        <el-button type="primary" @click="handleUpdate">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import * as tagApi from '@/api/tags'
import type { Tag, CreateTagRequest, UpdateTagRequest } from '@/api/tags'

const tags = ref<Tag[]>([])
const search = ref('')
const loading = ref(false)

// 加载标签
const loadTags = async () => {
  loading.value = true
  try {
    console.log('🔍 [TagManage] 开始加载标签, search:', search.value)
    
    const res = search.value
      ? await tagApi.getTags({ search: search.value })
      : await tagApi.getAllTags()

    console.log('🔍 [TagManage] API响应:', res)

    if (res.code === 0 && res.data) {
      const tagsList = Array.isArray(res.data) ? res.data : res.data.list || []
      console.log('🔍 [TagManage] 解析后的标签列表:', tagsList)
      tags.value = tagsList
      
      if (tagsList.length === 0) {
        console.warn('⚠️ [TagManage] 标签列表为空')
      }
    } else {
      console.error('❌ [TagManage] API返回错误:', res)
      ElMessage.error(res.msg || res.message || '获取标签失败')
    }
  } catch (error) {
    console.error('❌ [TagManage] 请求异常:', error)
    ElMessage.error('获取标签失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadTags()
})

// 手动刷新功能
const refreshTags = () => {
  console.log('🔄 [TagManage] 手动刷新标签')
  loadTags()
}

// 新增标签逻辑
const addDialogVisible = ref(false)
const newTag = ref<CreateTagRequest>({ name: '', description: '', color: '' })
const addFormRef = ref()

const rules = {
  name: [
    { required: true, message: '请输入名称', trigger: 'blur' },
    { min: 2, max: 20, message: '标签名称长度在 2 到 20 个字符', trigger: 'blur' }
  ],
  color: [
    { 
      validator: (rule: any, value: string, callback: Function) => {
        if (!value || value.trim() === '') {
          // 空值允许
          callback()
        } else if (!/^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/.test(value)) {
          callback(new Error('请输入有效的颜色值，如 #409EFF'))
        } else {
          callback()
        }
      },
      trigger: 'blur' 
    }
  ]
}

const showAddDialog = () => {
  newTag.value = { name: '', description: '', color: '' }
  addDialogVisible.value = true
  // 重置表单验证状态
  if (addFormRef.value) {
    addFormRef.value.clearValidate()
  }
}

const cancelAdd = () => {
  addDialogVisible.value = false
  // 重置表单数据和验证状态
  newTag.value = { name: '', description: '', color: '' }
  if (addFormRef.value) {
    addFormRef.value.resetFields()
    addFormRef.value.clearValidate()
  }
}

const handleAdd = async () => {
  if (!addFormRef.value) return
  
  try {
    // 表单验证
    await addFormRef.value.validate()
  } catch (error) {
    // 验证失败，直接返回
    return
  }
  
  try {
    // 处理空颜色值
    const tagData = { ...newTag.value }
    if (!tagData.color || tagData.color.trim() === '') {
      tagData.color = undefined
    }
    
    const res = await tagApi.createTag(tagData)
    console.log('🚀 [TagManage] 创建标签API响应:', res)
    
    if (res.code === 0) {
      ElMessage.success('创建成功')
      addDialogVisible.value = false
      await loadTags() // 确保等待加载完成
      // 重置表单
      newTag.value = { name: '', description: '', color: '' }
      addFormRef.value.resetFields()
    } else {
      ElMessage.error(res.msg || res.message || '创建失败')
    }
  } catch (error: any) {
    console.error('创建标签失败', error)
    
    // 检查是否是400错误（重复名称等业务错误）
    if (error.response?.status === 400 && error.response?.data?.msg) {
      ElMessage.error(error.response.data.msg)
    } else if (error.response?.data?.msg) {
      ElMessage.error(error.response.data.msg)  
    } else {
      ElMessage.error('创建失败')
    }
  }
}

// 编辑标签逻辑
const editDialogVisible = ref(false)
const editTagData = ref<(Tag & UpdateTagRequest) | any>({})
const editFormRef = ref()

const editTag = (row: Tag) => {
  editTagData.value = { ...row }
  editDialogVisible.value = true
  // 重置表单验证状态
  if (editFormRef.value) {
    editFormRef.value.clearValidate()
  }
}

const cancelEdit = () => {
  editDialogVisible.value = false
  // 重置表单验证状态
  if (editFormRef.value) {
    editFormRef.value.clearValidate()
  }
}

const handleUpdate = async () => {
  if (!editFormRef.value) return
  
  try {
    // 表单验证
    await editFormRef.value.validate()
  } catch (error) {
    // 验证失败，直接返回
    return
  }
  
  try {
    const { id, ...data } = editTagData.value
    // 处理空颜色值
    if (!data.color || data.color.trim() === '') {
      data.color = undefined
    }
    
    const res = await tagApi.updateTag(id, data)
    console.log('🚀 [TagManage] 更新标签API响应:', res)
    
    if (res.code === 0) {
      ElMessage.success('更新成功')
      editDialogVisible.value = false
      await loadTags() // 确保等待加载完成
    } else {
      ElMessage.error(res.msg || res.message || '更新失败')
    }
  } catch (error: any) {
    console.error('更新标签失败', error)
    
    // 检查是否是400错误（重复名称等业务错误）
    if (error.response?.status === 400 && error.response?.data?.msg) {
      ElMessage.error(error.response.data.msg)
    } else if (error.response?.data?.msg) {
      ElMessage.error(error.response.data.msg)  
    } else {
      ElMessage.error('更新失败')
    }
  }
}

// 删除标签
const deleteTag = (row: Tag) => {
  ElMessageBox.confirm(`确定删除标签 "${row.name}" ?`, '提示', { type: 'warning' })
    .then(async () => {
      try {
        const res = await tagApi.deleteTag(row.id)
        console.log('🚀 [TagManage] 删除标签API响应:', res)
        
        if (res.code === 0) {
          ElMessage.success('删除成功')
          await loadTags() // 确保等待加载完成
        } else {
          ElMessage.error(res.msg || res.message || '删除失败')
        }
      } catch (error: any) {
        console.error('删除标签失败', error)
        
        // 检查是否是400错误（业务错误）
        if (error.response?.status === 400 && error.response?.data?.msg) {
          ElMessage.error(error.response.data.msg)
        } else if (error.response?.data?.msg) {
          ElMessage.error(error.response.data.msg)  
        } else {
          ElMessage.error('删除失败')
        }
      }
    })
    .catch(() => {}) // 用户取消删除
}
</script>

<style scoped>
.tag-manage {
  padding: 24px;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.header-buttons {
  display: flex;
  gap: 10px; /* Adjust as needed for spacing */
}
</style> 